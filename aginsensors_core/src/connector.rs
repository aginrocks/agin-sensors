use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc::Receiver, oneshot};

#[derive(Debug, Clone)]
pub struct Measurement {
    pub timestamp: i64,
    pub measurement: String,
    pub values: HashMap<String, f64>,
}

#[derive(Debug)]
pub enum ReadRequest {
    LastMeasurement { sender: oneshot::Sender<i64> },
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

#[derive(Debug)]
pub struct ConnectorEvent {
    pub body: ConnectorEventBody,
    pub metadata: EventMetadata,
}

impl ConnectorEvent {
    pub fn new_measurement(measurement: Measurement, metadata: EventMetadata) -> Self {
        ConnectorEvent {
            body: ConnectorEventBody::Measurement(measurement),
            metadata,
        }
    }

    pub fn new_read_request(read_request: ReadRequest, metadata: EventMetadata) -> Self {
        ConnectorEvent {
            body: ConnectorEventBody::ReadRequest(read_request),
            metadata,
        }
    }
}

#[derive(Debug)]
pub enum ConnectorEventBody {
    Measurement(Measurement),
    ReadRequest(ReadRequest),
}

pub trait IntoEvents {
    fn into_events(self) -> Vec<ConnectorEvent>;
}

pub trait ConnectorRunner {
    /// Runs the connector (conencts to a broker, starts a HTTP server, etc.).
    /// Returns a Tokio mpsc channel with Measurement events.
    fn run(&self) -> Arc<Receiver<Vec<ConnectorEvent>>>;
}
