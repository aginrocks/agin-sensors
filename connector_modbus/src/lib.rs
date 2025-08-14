use std::sync::Arc;

use color_eyre::eyre::{Context as _, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::{
    sync::{
        Mutex,
        mpsc::{Receiver, channel},
    },
    task::JoinHandle,
    time::{Duration, interval},
};
use tokio_modbus::{client::Context, prelude::*};

use aginsensors_core::{
    connector::{ConnectorEvent, ConnectorRunner},
    define_connector,
};

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

struct ModbusReader {
    address: String,
    ctx: Mutex<Context>,
    timefix: bool,
}

impl ModbusReader {
    pub async fn try_connect(address: String, timefix: bool) -> Result<Self> {
        let ctx = tcp::connect(address.parse()?).await?;

        Ok(Self {
            address,
            ctx: Mutex::new(ctx),
            timefix,
        })
    }
}
