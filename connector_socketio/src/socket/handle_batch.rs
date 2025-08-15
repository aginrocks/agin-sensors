use std::collections::HashMap;

use aginsensors_core::connector::{ConnectorEvent, EventMetadata, IntoEvents, Measurement};
use color_eyre::eyre::ContextCompat;
use serde::Deserialize;
use socketioxide::extract::{AckSender, Data, SocketRef, State};
use tracing::error;

use crate::{SocketIo, middleware::extract_token};

#[derive(Deserialize, Debug, Clone)]
pub struct Batch {
    #[serde(rename = "b")]
    pub bucket: String,

    #[serde(rename = "v")]
    pub groups: Vec<Group>,
}

impl Batch {
    fn into_events(self, token: String) -> Vec<ConnectorEvent> {
        self.groups
            .into_iter()
            .map(|group| {
                // let token = self.token.clone();
                let metadata = EventMetadata::builder().auth_token(token.clone());

                ConnectorEvent::new_measurements(
                    group
                        .values
                        .into_iter()
                        .map(|value| Measurement {
                            timestamp: group.timestamp,
                            measurement: value.measurement,
                            values: value.values,
                        })
                        .collect(),
                    metadata,
                )

                // group.values.into_iter().map(move |value| {
                //     ConnectorEvent::new_measurements(
                //         Measurement {
                //             timestamp: group.timestamp,
                //             measurement: value.measurement,
                //             values: value.values,
                //         },
                //         metadata.clone(),
                //     )
                // })
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

pub async fn handler(
    socket: SocketRef,
    ack: AckSender,
    Data(batch): Data<Batch>,
    State(state): State<SocketIo>,
) {
    let measurements = batch.into_events(extract_token(&socket));

    for measurement in measurements {
        let _ = state.tx.send(measurement).await;
    }
    ack.send("OK").ok();
}
