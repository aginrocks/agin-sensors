use color_eyre::eyre::Result;
use std::collections::HashMap;
use tokio::sync::mpsc::Receiver;

#[derive(Debug, Clone)]
pub struct Measurement {
    pub timestamp: i64,
    pub measurement: String,
    pub bucket: Option<String>,
    pub values: HashMap<String, f64>,
}

pub trait ConnectorRunner {
    /// Runs the connector (conencts to a broker, starts a HTTP server, etc.).
    /// Returns a Tokio mpsc channel with Measurement events.
    fn run(&self) -> Result<Receiver<Measurement>>;
}
