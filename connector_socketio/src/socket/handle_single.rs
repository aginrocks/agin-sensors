use std::collections::HashMap;

use aginsensors_core::connector::{IntoMeasurements, Measurement};
use serde::Deserialize;
use socketioxide::extract::{Data, State};

use crate::SocketIo;

#[derive(Deserialize, Debug, Clone)]
pub struct SingleMeasurement {
    pub bucket: String,

    #[serde(rename = "ts")]
    pub timestamp: i64,

    pub values: HashMap<String, f64>,

    pub measurement: String,
}

impl IntoMeasurements for SingleMeasurement {
    fn into_measurements(self) -> Vec<Measurement> {
        vec![Measurement {
            timestamp: self.timestamp,
            measurement: self.measurement,
            bucket: Some(self.bucket),
            values: self.values,
        }]
    }
}

// idk why we need to ack only on a batch and not on a single measurement but it is what it is
pub async fn handler(Data(measurement): Data<SingleMeasurement>, State(state): State<SocketIo>) {
    let measurement = measurement.into_measurements();

    state.tx.clone().send(measurement).await.ok();
}
