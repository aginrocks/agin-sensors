#[macro_export]
macro_rules! define_connectors {
    ( $( $path:ident::$name:ident ),* ) => {
        paste::paste! {
            #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
            #[serde(untagged)]
            pub enum ConnectorConfig {
                $($name($path::[<Config$name>])),*
            }

            #[derive(Clone, Debug)]
            pub enum ConnectorType {
                $($name($path::$name)),*
            }

            impl $crate::connector::ConnectorBuilder for ConnectorType {
                fn new(config: &ConnectorConfig) -> Self {
                    match config {
                        $(ConnectorConfig::$name(config) => ConnectorType::$name(<$path::$name as $path::[<$name Connector>]>::new(config))),*
                    }
                }
            }

            impl aginsensors_core::connector::ConnectorRunner for ConnectorType {
                fn run(&self) -> tokio::sync::mpsc::Receiver<aginsensors_core::connector::ConnectorEvent> {
                    match self {
                        $(ConnectorType::$name(connector) => connector.run()),*
                    }
                }
            }
        }
    };
}
