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

            impl $crate::connector::ConnectorRunner for ConnectorType {
                fn run(&self) -> color_eyre::eyre::Result<tokio::sync::mpsc::Receiver<$crate::connector::Measurement>> {
                    match self {
                        $(ConnectorType::$name(connector) => connector.run()),*
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_connector {
    (
        $tag_value:literal,
        $struct_name:ident,
        config = { $($config:tt)* },
        state = { $($state:tt)* }
    ) => {
        paste::paste! {
            #[derive(Clone, Debug)]
            pub struct $struct_name {
                pub config: [<Config$struct_name>],

                $($state)*
            }

            #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
            pub struct [<Config$struct_name>] {
                 pub r#type: [<ConnectorType$struct_name>],

                 $($config)*
            }

            #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
            pub enum [<ConnectorType$struct_name>] {
                #[serde(rename = $tag_value)]
                Value,
            }

            pub trait [<$struct_name Connector>] {
                fn new(config: &[<Config$struct_name>]) -> Self;
            }
        }
    };
}
