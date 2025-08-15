use std::collections::HashMap;

use aginsensors_core::connector::{ConnectorEvent, EventMetadata, IntoEvents, Measurement};
use serde::Deserialize;
use socketioxide::extract::{Data, SocketRef, State};

use crate::{SocketIo, middleware::extract_token};

#[derive(Deserialize, Debug, Clone)]
pub struct SingleMeasurement {
    pub bucket: String,

    #[serde(rename = "ts")]
    pub timestamp: i64,

    pub values: HashMap<String, f64>,

    pub measurement: String,
}

impl SingleMeasurement {
    fn into_events(self, token: String) -> Vec<ConnectorEvent> {
        let metadata = EventMetadata::builder().auth_token(token.clone());

        vec![ConnectorEvent::new_measurements(
            vec![Measurement {
                timestamp: self.timestamp,
                measurement: self.measurement,
                values: self.values,
            }],
            metadata,
        )]
    }
}

// idk why we need to ack only on a batch and not on a single measurement but it is what it is
pub async fn handler(
    socket: SocketRef,
    Data(measurement): Data<SingleMeasurement>,
    State(state): State<SocketIo>,
) {
    let measurement = measurement.into_events(extract_token(&socket));

    let _ = state.tx.send(measurement.first().unwrap().to_owned()).await;
}
