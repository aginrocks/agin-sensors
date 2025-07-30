use crate::define_connectors;

pub mod mqtt;
pub mod socketio;

define_connectors!(mqtt::Mqtt, socketio::SocketIo);
