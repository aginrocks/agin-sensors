use aginsensors_core::{
    connector::{ConnectorRunner, Measurement},
    define_connector,
};
use color_eyre::eyre::Result;
use tokio::sync::mpsc::Receiver;

define_connector!(
    "socketio",
    SocketIo,
    config = {
        pub port: u16,
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
