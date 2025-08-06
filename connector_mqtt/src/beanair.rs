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

pub fn parse() -> Result<(Vec<Measurement>, EventMetadata)> {
    // let metadata = EventMetadata::builder().topic(publish.topic.clone());
    let bytes = hex::decode(
        "0503973293680f00c8001f0000009439004200000000000000420001a1eb0300070080100000bc02800a0280ea03000700800e0000c40280260280ea03000600800c0000c50280230280e60300090080070000c10280f40180eb03000600800c0000c70280cd0180eb03000800800c0000cb0280da0180e903000800800b0000b40280e90180ed03000500800e0000c00280e90180e40300080080110000c60280110280ea03000500800d0000c502802f0280eb03000400800d0000bc0280230280e10300080080080000c40280eb0180ea03000a00800b0000bc0280cc0180ee03000500800e0000c40280d60180e803000700800c0000be0280f10180f70300040080100000bb02800e0280e603000900800c0000be0280080280ea03000600800d0000c502800a0280ec03000600800c0000c10280090280ed03000300800c0000be0280df0180e803000a00800a0000ac0280d10180ec03000800800c0000c10280ec0180e703000800800d0000c10280ff0180e903000700800f0000ca02801d0280ee03000400800d0000c50280110280eb03000400800a0000c20280eb0180eb03000700800c0000c80280da0180eb03000600800c0000c10280d30180e803000700800c0000bc0280d80180ed03000800800c0000c40280e10180eb03000300800f0000c60280090280ee0300050080120000c80280370280e803000900800a0000c80280120280ee03000500800a0000c10280f90180e90300050080080000c00280d80180e903000700800e0000c00280c90180eb0300090080090000bc0280da0180ed03000500800f0000bf0280e60180ef03000600800e0000bd0280190280e503000600800e0000b302801c0280eb03000500800a0000c10280130280ea03000400800b0000c70280ed0180ea03000600800c0000c50280cd0180e403000900800b0000c70280d90180e703000800800b0000bc0280ee0180e403000600800c0000c40280f80180ea03000800800a0000b20280200280eb03000900800c0000c302800a0280e303000600800b0000c90280f30180e903000700800c0000c20280d60180ea03000600800c0000c50280c00180e903000700800c0000c10280e20180eb03000600800f0000c202800b0280eb03000700800d0000c30280210280ea03000600800a0000bd0280110280ea03000500800b0000c80280fe0180ed03000500800b0000bd0280e10180ec03000700800c0000c30280c50180e903000600800e0000bc0280ca0180e803000800800b0000bc0280f30180e903000700800e0000c10280140280e903000700800c0000b70280320280ea03000700800a0000bf0280170280ec0300060080090000ce0280e10180ec03000700800d0000c70280cb0180ea03000700800c0000c30280dc0180",
    )?;

    let mut cursor = Cursor::new(bytes);

    let header = BeanAirHeader::read(&mut cursor)?;

    dbg!(&header);

    match header {
        BeanAirHeader::HiInc(header) => {
            todo!();
        }
        BeanAirHeader::XInc(header) => match header.acquisition {
            AcquisitionType::Streaming(data) => {
                let metadata = EventMetadata::builder().topic("".to_string());

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

                dbg!(measurements);

                // (measurements)
                todo!()
            }
        },
    }
}
