use std::collections::HashMap;

use aginsensors_core::connector::{IntoMeasurements, Measurement};
use serde::Deserialize;
use socketioxide::extract::{AckSender, Data, State};

use crate::SocketIo;

#[derive(Deserialize, Debug, Clone)]
pub struct Batch {
    #[serde(rename = "b")]
    pub bucket: String,

    #[serde(rename = "v")]
    pub groups: Vec<Group>,
}

impl IntoMeasurements for Batch {
    fn into_measurements(self) -> Vec<Measurement> {
        self.groups
            .into_iter()
            .flat_map(|group| {
                let bucket = self.bucket.clone();
                group.values.into_iter().map(move |value| Measurement {
                    timestamp: group.timestamp,
                    measurement: value.measurement,
                    bucket: Some(bucket.clone()),
                    values: value.values,
                })
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Group {
    #[serde(rename = "ts")]
    pub timestamp: i64,

    #[serde(rename = "v")]
    pub values: Vec<GroupedMeasurement>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GroupedMeasurement {
    #[serde(rename = "m")]
    pub measurement: String,

    #[serde(rename = "d")]
    pub values: HashMap<String, f64>,
}

pub async fn handler(ack: AckSender, Data(batch): Data<Batch>, State(state): State<SocketIo>) {
    let measurements = batch.into_measurements();

    if state.tx.clone().send(measurements).await.is_ok() {
        ack.send("OK").ok();
    }
}
