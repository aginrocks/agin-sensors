use aginsensors_core::{
    connector::{ConnectorEvent, ConnectorRunner},
    define_connector,
};
use tokio::sync::mpsc::Receiver;

define_connector!(
    "mqtt",
    Mqtt,
    config = {
        pub url: String,
        pub username: String,
        pub password: String,
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
    fn run(&self) -> Receiver<Vec<ConnectorEvent>> {
        todo!()
    }
}
