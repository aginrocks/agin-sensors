use std::{collections::HashMap, sync::Arc};

use aginsensors_core::{
    connector::{ConnectorEvent, ConnectorRunner, EventMetadata, Measurement, ReadRequest},
    define_connector,
};
use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike, Utc};
use color_eyre::eyre::{Context as _, ContextCompat, Result, bail};
use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::{
    sync::{
        Mutex,
        mpsc::{Receiver, Sender, channel},
        oneshot,
    },
    task::JoinHandle,
    time::{Duration, interval},
};
use tokio_modbus::{
    Address, Quantity,
    client::{Context, Reader, Writer},
    prelude::*,
};
use tracing::debug;

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct ModbusDevice {
    name: String,
    ip: String,
    port: u16,
    timefix: Option<bool>,
}

define_connector!("modbus", Modbus, config = {
    devices: Vec<ModbusDevice>
}, state = {});

const STEP_REGISTER: u16 = 0x0200;
const TIME_REGISTER: u16 = 0x0201;
const NUMBER_REGISTER: u16 = 0x0204;
const DATA_REGISTERS_START: u16 = 0x0206;
const MAX_REGISTERS_PER_READ: u16 = 125;
const TIME_START_REGISTER: u16 = 264;
const TIME_STOP_REGISTER: u16 = 267;
const RECORD_SIZE_REGISTER: u16 = 277;
const HEADER_COUNT_REGISTER: u16 = 278;
const DATA_COUNT_REGISTER: u16 = 258;

impl ModbusConnector for Modbus {
    fn new(config: &ConfigModbus) -> Self {
        let (tx, rx) = channel::<ConnectorEvent>(1000);

        Modbus {
            config: config.clone(),
            tx: Arc::new(tx),
            rx: Arc::new(std::sync::Mutex::new(Some(rx))),
        }
    }
}

impl ConnectorRunner for Modbus {
    fn run(&self) -> Receiver<ConnectorEvent> {
        let mut tasks = Vec::new();

        for device in self.config.devices.clone() {
            let this = self.clone();
            let handle: JoinHandle<Result<()>> = tokio::spawn(async move {
                let mut ticker = interval(Duration::from_secs(30 * 60));

                loop {
                    ticker.tick().await;

                    let client = ModbusReader::try_connect(
                        format!("{}:{}", device.ip, device.port),
                        this.tx.clone(),
                        device.timefix.unwrap_or_default(),
                    )
                    .await
                    .wrap_err_with(|| format!("Failed to connect to {}", device.ip))?;
                }
            });

            tasks.push(handle);
        }

        self.rx
            .lock()
            .unwrap()
            .take()
            .expect("Receiver already taken")
    }
}

enum ReadType {
    Header,
    Data,
}

struct ModbusReader {
    address: String,
    client: Context,
    sender: Arc<Sender<ConnectorEvent>>,
    timefix: bool,
}

#[derive(Debug)]
struct ModbusMetadata {
    serial_number: String,
    column_headers: Vec<String>,
    event_metadata: EventMetadata,
}

#[derive(Debug, Clone)]
struct ReadResult {
    date: DateTime<Utc>,
    measurements: HashMap<String, f64>,
    next_counter: u32,
    counter: u32,
}

impl ModbusReader {
    pub async fn try_connect(
        address: String,
        sender: Arc<Sender<ConnectorEvent>>,
        timefix: bool,
    ) -> Result<Self> {
        let ctx = tcp::connect(address.parse()?).await?;

        Ok(Self {
            address,
            client: ctx,
            sender,
            timefix,
        })
    }

    /// Read registers (divide read to blocks)
    async fn read_registers_in_blocks(
        &mut self,
        start_address: Address,
        total_registers: Quantity,
    ) -> Result<Vec<u16>> {
        let mut result = Vec::new();
        let mut remaining = total_registers;
        let mut current_address = start_address;

        while remaining > 0 {
            let count = remaining.min(MAX_REGISTERS_PER_READ);

            let data = self
                .client
                .read_holding_registers(current_address, count)
                .await??;

            result.extend(data);

            remaining -= count;
            current_address += count;
        }

        Ok(result)
    }

    /// Convert modbus registers to ASCII text
    fn convert_registers_to_ascii(registers: Vec<u16>) -> String {
        let mut result = "".to_string();
        for register in registers {
            let high_byte = ((register >> 8) & 0xFF) as u8;
            let low_byte = (register & 0xFF) as u8;

            result.push(high_byte as char);
            result.push(low_byte as char);
        }

        result.trim().to_string()
    }

    /// Get number of lines for header data
    async fn get_header_count(&mut self) -> Result<u16> {
        let result = self
            .client
            .read_holding_registers(HEADER_COUNT_REGISTER, 1)
            .await??;

        let count = result.first().unwrap_or(&0);

        Ok(*count)
    }

    /// Get number of lines for measurements data
    async fn get_record_count(&mut self) -> Result<u32> {
        let result = self
            .client
            .read_holding_registers(DATA_COUNT_REGISTER, 2)
            .await??;

        let count = ((*(result.get(1).unwrap_or(&0)) as u32) << 16)
            + (*(result.first().unwrap_or(&0)) as u32);

        Ok(count)
    }

    /// Set up read type and current line number
    async fn setup_read(&mut self, read_type: ReadType, offset: u32) -> Result<()> {
        self.client
            .write_multiple_registers(
                NUMBER_REGISTER,
                &[
                    (offset & 0xffff) as u16,
                    match read_type {
                        ReadType::Header => 0x8000,
                        ReadType::Data => (offset >> 16) as u16,
                    },
                ],
            )
            .await??;

        Ok(())
    }

    /// Get current line size in number of Modbus registers to read
    async fn get_record_size(&mut self) -> Result<u16> {
        let result = self
            .client
            .read_holding_registers(RECORD_SIZE_REGISTER, 1)
            .await??;
        let raw_size = result.first().unwrap_or(&0);
        let size = (raw_size + 1) >> 1;
        Ok(size)
    }

    /// Read raw headers
    async fn read_headers_raw(&mut self) -> Result<Vec<String>> {
        let mut headers = Vec::new();
        let mut count = self.get_header_count().await?;

        self.setup_read(ReadType::Header, 0).await?;

        while count > 0 {
            count -= 1;

            let len = self.get_record_size().await?;
            let header_data = self
                .read_registers_in_blocks(DATA_REGISTERS_START, len)
                .await?;
            let header = ModbusReader::convert_registers_to_ascii(header_data);
            let header = header.trim().to_string();

            if !header.is_empty() {
                headers.push(header);
            }
        }

        Ok(headers)
    }

    /// Write last measurement date to the sensor
    async fn write_date(&mut self, reg: Address, date: DateTime<Utc>) -> Result<()> {
        let year = date.year() - 2000;
        let month = date.month() as i32;
        let day = date.day();
        let hour = date.hour();
        let minute = date.minute();
        let second = date.second();

        let reg_1 = ((year << 8) | month) as u16;
        let reg_2 = ((day << 8) | hour) as u16;
        let reg_3 = ((minute << 8) | second) as u16;

        self.client
            .write_multiple_registers(reg, &[reg_1, reg_2, reg_3])
            .await??;

        Ok(())
    }

    /// Reads and parses headers
    async fn read_headers(&mut self) -> Result<ModbusMetadata> {
        self.client
            .write_single_register(STEP_REGISTER, 1)
            .await??;
        let header = self.read_headers_raw().await?;

        let mut serial_number = None;
        let mut column_headers = Vec::new();

        let serial_regex = Regex::new(r"Nr seryjny:;\s*(\d+)")?;

        for (i, line) in header.iter().enumerate() {
            if line.starts_with("MPI-C") {
                if let Some(sn) = serial_regex.captures(line)
                    && let Some(sn) = sn.get(0)
                {
                    serial_number = Some(sn.as_str().to_string());
                };
            } else if line.starts_with("Symbol") {
                let data_lines = &header[i..];

                for line in data_lines {
                    if line.starts_with("Rekordy") {
                        break;
                    }
                    let mut parts = line.split(';');
                    if let Some(name) = parts.nth(1)
                        && !name.trim().starts_with("Wynik")
                    {
                        column_headers.push(name.trim().to_string());
                    }
                }
            }
        }

        let serial_number = serial_number.wrap_err("No serial number found")?;

        let event_metadata = EventMetadata::builder()
            .mac(serial_number.clone())
            .ip(self.address.clone());

        Ok(ModbusMetadata {
            serial_number,
            column_headers,
            event_metadata,
        })
    }

    /// Reads a single record
    async fn read_record(
        &mut self,
        column_headers: Vec<String>,
        counter: u32,
        setup: bool,
    ) -> Result<ReadResult> {
        if setup {
            self.setup_read(ReadType::Data, counter).await?;
        }

        let len = self.get_record_size().await?;
        let record_data = self
            .read_registers_in_blocks(DATA_REGISTERS_START, len)
            .await?;
        let line = ModbusReader::convert_registers_to_ascii(record_data);
        let columns = line.split(';').collect::<Vec<_>>();
        if columns.len() < 6 {
            bail!("Too few columns");
        }

        let date_str = format!("20{}", columns[0]);
        let time_str = columns[1];

        let date = NaiveDate::parse_from_str(&date_str, "%y-%m-%d")?;
        let time = NaiveTime::parse_from_str(time_str, "%H:%M:%S")?;

        let naive_dt = NaiveDateTime::new(date, time);
        let record_date = Utc.from_utc_datetime(&naive_dt);

        let measurements = column_headers
            .into_iter()
            .enumerate()
            .map(|(i, header)| -> Result<(String, f64)> {
                let value = columns.get(4 + i).wrap_err("Column not found")?;
                let value = value.replace(",", ".").parse::<f64>()?;
                Ok((header, value))
            })
            .filter_map(|header| header.ok())
            .collect::<HashMap<_, _>>();

        let next_counter = columns
            .get(24)
            .wrap_err("No counter found")?
            .parse::<u32>()?;

        let result = ReadResult {
            date: record_date,
            measurements,
            next_counter,
            counter,
        };

        Ok(result)
    }

    /// Reads data points
    async fn read_records(&mut self, metadata: &ModbusMetadata) -> Result<()> {
        // let mut result = Vec::new();
        let record_count = self.get_record_count().await?;
        debug!("Number of data lines: {record_count}");

        let mut last_counter = record_count - 1;

        if self.timefix {
            todo!();
        } else {
            // Incremental read

            let date = self.get_last_date(metadata.event_metadata.clone()).await?;
            self.write_date(DATA_REGISTERS_START, date).await?;

            self.read_registers_in_blocks(NUMBER_REGISTER, 1).await?;

            let mut last = None;

            if let Ok(check) = self
                .read_record(metadata.column_headers.clone(), 0, false)
                .await
            {
                last_counter = check.next_counter - 1;
                last = Some(check);
            }

            while last_counter != 0
                && let Some(last) = last.clone()
                && last.date > date
                && last_counter + 1 == last.next_counter
            {}

            debug!(?date, last_counter, "Read in forward direction");
        }

        Ok(())
    }

    /// Gets last date from the database
    async fn get_last_date(&self, metadata: EventMetadata) -> Result<DateTime<Utc>> {
        let (tx, rx) = oneshot::channel::<i64>();

        self.sender
            .send(ConnectorEvent::new_read_request(
                ReadRequest::LastMeasurement {
                    sender: Arc::new(Mutex::new(Some(tx))),
                },
                metadata,
            ))
            .await?;

        let date = rx.await?;
        debug!("Last date: {date}");

        let date = DateTime::from_timestamp_millis(date).wrap_err("Invalid date")?;

        Ok(date)
    }

    async fn read(&mut self) -> Result<Vec<Measurement>> {
        let metadata = self.read_headers().await?;

        self.read_records(&metadata).await?;

        todo!()
    }
}
