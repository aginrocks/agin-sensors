use std::collections::HashMap;

use aginsensors_core::connector::{ConnectorEvent, EventMetadata, IntoEvents, Measurement};
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

impl IntoEvents for SingleMeasurement {
    fn into_events(self) -> Vec<ConnectorEvent> {
        let metadata = EventMetadata::builder().bucket(self.bucket);

        vec![ConnectorEvent::new_measurement(
            Measurement {
                timestamp: self.timestamp,
                measurement: self.measurement,
                values: self.values,
            },
            metadata,
        )]
    }
}

// idk why we need to ack only on a batch and not on a single measurement but it is what it is
pub async fn handler(Data(measurement): Data<SingleMeasurement>, State(state): State<SocketIo>) {
    let measurement = measurement.into_events();

    state.tx.clone().send(measurement).await.ok();
}
