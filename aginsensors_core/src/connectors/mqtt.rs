use color_eyre::eyre::Result;
use tokio::sync::mpsc::Receiver;

use crate::{
    connector::{ConnectorRunner, Measurement},
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

impl MqttConnector for Mqtt {
    fn new(config: &ConfigMqtt) -> Self {
        Mqtt {
            config: config.clone(),
        }
    }
}

impl ConnectorRunner for Mqtt {
    fn run(&self) -> Result<Receiver<Measurement>> {
        todo!()
    }
}
