use crate::connectors::ConnectorConfig;

pub trait ConnectorBuilder {
    fn new(config: &ConnectorConfig) -> Self;
}
