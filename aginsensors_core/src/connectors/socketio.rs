use color_eyre::eyre::Result;
use tokio::sync::mpsc::Receiver;

use crate::{
    connector::{Connector, Measurement},
    connectors::ConnectorConfig,
    define_connector,
};

define_connector!(
    "socketio",
    SocketIo,
    config = {
        pub url: String,
    },
    state = {}
);

impl Connector for SocketIo {
    fn new(config: &ConnectorConfig) -> Self {
        if let ConnectorConfig::SocketIo(config) = config {
            SocketIo {
                config: config.clone(),
            }
        } else {
            panic!("Invalid connector configuration for Socket.IO");
        }
    }

    fn run(&self) -> Result<Receiver<Measurement>> {
        todo!()
    }
}
