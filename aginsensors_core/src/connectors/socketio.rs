use color_eyre::eyre::Result;
use tokio::sync::mpsc::Receiver;

use crate::{
    connector::{ConnectorRunner, Measurement},
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

impl SocketIoConnector for SocketIo {
    fn new(config: &ConfigSocketIo) -> Self {
        SocketIo {
            config: config.clone(),
        }
    }
}

impl ConnectorRunner for SocketIo {
    fn run(&self) -> Result<Receiver<Measurement>> {
        todo!()
    }
}
