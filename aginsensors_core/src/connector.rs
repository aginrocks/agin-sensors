use color_eyre::eyre::Result;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::oneshot;

use crate::organizations::{Filter, Organization, OrganizationsState};

#[derive(Debug, Clone)]
pub struct Measurement {
    pub timestamp: i64,
    pub measurement: String,
    pub values: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub enum ReadRequest {
    LastMeasurement { sender: Arc<oneshot::Sender<i64>> },
}

#[derive(Debug, Clone)]
pub struct EventMetadata {
    /// Bucket that the data needs to be written to
    pub bucket: Option<String>,

    /// Topic from which the data is received (e.g. MQTT topic)
    pub topic: Option<String>,

    /// Sensor's MAC address or other unique identifier
    pub mac: Option<String>,

    /// Token used by the sensor
    pub auth_token: Option<String>,
}

impl EventMetadata {
    pub fn builder() -> Self {
        Self {
            bucket: None,
            topic: None,
            mac: None,
            auth_token: None,
        }
    }

    pub fn bucket(mut self, bucket: String) -> Self {
        self.bucket = Some(bucket);
        self
    }

    pub fn topic(mut self, topic: String) -> Self {
        self.topic = Some(topic);
        self
    }

    pub fn auth_token(mut self, auth_token: String) -> Self {
        self.auth_token = Some(auth_token);
        self
    }

    pub fn mac(mut self, mac: String) -> Self {
        self.mac = Some(mac);
        self
    }
}

#[derive(Debug, Clone)]
pub struct ConnectorEvent {
    pub body: ConnectorEventBody,
    pub metadata: EventMetadata,
}

#[derive(Debug, Clone)]
pub struct FilteredConnectorEvent {
    pub body: ConnectorEventBody,
    pub organizations: Vec<Organization>,
}

impl ConnectorEvent {
    pub fn new_measurements(measurements: Vec<Measurement>, metadata: EventMetadata) -> Self {
        ConnectorEvent {
            body: ConnectorEventBody::Measurements(measurements),
            metadata,
        }
    }

    pub fn new_read_request(read_request: ReadRequest, metadata: EventMetadata) -> Self {
        ConnectorEvent {
            body: ConnectorEventBody::ReadRequest(read_request),
            metadata,
        }
    }

    pub fn filter(&self, organizations: &OrganizationsState) -> Result<FilteredConnectorEvent> {
        let matching_orgs: Vec<Organization> = organizations
            .organizations
            .values()
            .filter(|org| {
                if let Some(bucket) = &self.metadata.bucket {
                    if &org.bucket == bucket {
                        return true;
                    }
                } else if let Some(mac) = &self.metadata.mac {
                    if org.filters.iter().any(|f| match f {
                        Filter::MacFilter(mac_filter) => mac_filter.macs.contains(mac),
                        _ => false,
                    }) {
                        return true;
                    }
                } else if let Some(auth_token) = &self.metadata.auth_token {
                    if org.filters.iter().any(|f| match f {
                        Filter::TokenFilter(token_filter) => {
                            token_filter.tokens.contains(auth_token)
                        }
                        _ => false,
                    }) {
                        return true;
                    }
                }
                false
            })
            .cloned()
            .collect();

        if matching_orgs.is_empty() {
            Err(color_eyre::eyre::eyre!(
                "Couldn't find organization for event",
            ))
        } else {
            Ok(FilteredConnectorEvent {
                body: self.body.clone(),
                organizations: matching_orgs,
            })
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConnectorEventBody {
    Measurements(Vec<Measurement>),
    ReadRequest(ReadRequest),
}

pub trait IntoEvents {
    fn into_events(self) -> Vec<ConnectorEvent>;
}

pub trait ConnectorRunner {
    /// Runs the connector (connects to a broker, starts a HTTP server, etc.).
    /// Returns a Tokio mpsc receiver for ConnectorEvent batches.
    fn run(&self) -> tokio::sync::mpsc::Receiver<ConnectorEvent>;
}
