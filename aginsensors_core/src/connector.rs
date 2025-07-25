use std::{collections::HashMap, sync::Arc};

use color_eyre::eyre::Result;
use tokio::sync::broadcast::Receiver;

use crate::global_config::GlobalConfig;

pub struct Measurement {
    pub timestamp: u64,
    pub measurement: String,
    pub bucket: Option<String>,
    pub values: HashMap<String, f64>,
}

pub trait Connector {
    fn new(config: &Arc<GlobalConfig>) -> Self;

    /// Runs the connector (conencts to a broker, starts a HTTP server, etc.).
    /// Returns a Tokio broadcast channel with Measurement events.
    fn run(&self) -> Result<Receiver<Measurement>>;
}
