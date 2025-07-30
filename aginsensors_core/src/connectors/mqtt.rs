use color_eyre::eyre::Result;
use tokio::sync::mpsc::Receiver;

use crate::{
    connector::{Connector, Measurement},
    connectors::ConnectorConfig,
    define_connector,
};

define_connector!(
    "mqtt",
    Mqtt,
    config = {
        pub url: String,
    },
    state = {}
);

impl Connector for Mqtt {
    fn new(ConnectorConfig::Mqtt(config): &ConnectorConfig) -> Self {
        Mqtt {
            config: config.clone(),
        }
    }

    fn run(&self) -> Result<Receiver<Measurement>> {
        todo!()
    }
}
