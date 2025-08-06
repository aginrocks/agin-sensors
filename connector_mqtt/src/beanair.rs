use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

use aginsensors_core::connector::{EventMetadata, Measurement};
use binrw::BinRead;
use color_eyre::eyre::Result;
use rumqttc::Publish;

#[derive(BinRead, Debug)]
#[br(big)]
enum BeanAirHeader {
    #[br(magic(3u8))]
    HiInc(HiIncHeader),

    #[br(magic(5u8))]
    XInc(XIncHeader),
}

#[derive(BinRead, Debug)]
#[br(little)]
struct HiIncHeader {}

#[derive(BinRead, Debug)]
#[br(little)]
struct XIncHeader {
    acquisition: AcquisitionType,
}

#[derive(BinRead, Debug)]
#[br(little)]
#[allow(dead_code)]
struct StreamingAcquisition {
    reference_time: u32,

    reference_milliseconds: u16,

    frequency: u16,

    channel_mask: u32,

    #[br(map = |b: [u8; 3]| u32::from_le_bytes([b[0], b[1], b[2], 0]))]
    sequence_id: u32,

    data_aq_per_channel: u16,

    #[br(map = |b: [u8; 3]| u32::from_le_bytes([b[0], b[1], b[2], 0]))]
    data_aq_cycle: u32,

    #[br(map = |b: [u8; 3]| u32::from_le_bytes([b[0], b[1], b[2], 0]))]
    data_aq_duration: u32,

    prev_aq_per_channel: u16,

    flags: u8,

    network_quality: u8,
}

#[derive(BinRead, Debug)]
#[br(little)]
enum AcquisitionType {
    #[br(magic(3u8))]
    Streaming(StreamingAcquisition),
}

const CHANNEL_MAP: [&str; 5] = ["Z", "X", "Y", "X_INC", "Y_INC"];

pub fn parse(publish: &Publish) -> Result<(Vec<Measurement>, EventMetadata)> {
    let mut cursor = Cursor::new(publish.payload.clone());

    let header = BeanAirHeader::read(&mut cursor)?;

    dbg!(&header);

    match header {
        BeanAirHeader::HiInc(_header) => {
            todo!();
        }
        BeanAirHeader::XInc(header) => match header.acquisition {
            AcquisitionType::Streaming(data) => {
                let mac = publish
                    .topic
                    .split('/')
                    .nth(1)
                    .ok_or_else(|| color_eyre::eyre::eyre!("MAC not found in topic"))?
                    .to_string();

                let metadata = EventMetadata::builder()
                    .topic(publish.topic.clone())
                    .mac(mac);

                let time_ms = (data.reference_time as u64) * 1000
                    + (data.reference_milliseconds as u64)
                    - 3600000;

                let midx = data.sequence_id as u64 * data.prev_aq_per_channel as u64;

                let mut measurements = Vec::new();

                for i in 0..data.data_aq_per_channel {
                    let timestamp = time_ms
                        + (1000f64 * (midx as f64 + i as f64) / data.frequency as f64).round()
                            as u64;

                    let mut measurement = Measurement {
                        timestamp: timestamp as i64,
                        measurement: "".to_string(),
                        values: HashMap::new(),
                    };

                    for (j, channel) in CHANNEL_MAP.iter().enumerate() {
                        if data.channel_mask & (1 << j) == 0 {
                            continue;
                        }

                        let mut bytes: [u8; 3] = [0; 3];
                        cursor.read_exact(&mut bytes)?;

                        let data_sample = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], 0]);

                        let sign_bit = (data_sample >> 23) & 0x01;
                        let remaining_bits = data_sample & 0x7FFFFF;
                        let decimal_value = if sign_bit == 0 {
                            remaining_bits as f64 / 1000f64
                        } else {
                            -(remaining_bits as f64 / 1000f64)
                        };

                        measurement
                            .values
                            .insert(channel.to_string(), decimal_value);
                    }

                    measurements.push(measurement);
                }

                dbg!(&measurements);

                Ok((measurements, metadata))
            }
        },
    }
}
