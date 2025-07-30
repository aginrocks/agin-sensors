use crate::define_connectors;

define_connectors!(connector_mqtt::Mqtt, connector_socketio::SocketIo);
