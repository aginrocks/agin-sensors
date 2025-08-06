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

                tx: std::sync::Arc<tokio::sync::mpsc::Sender<aginsensors_core::connector::ConnectorEvent>>,
                rx: std::sync::Arc<std::sync::Mutex<Option<tokio::sync::mpsc::Receiver<aginsensors_core::connector::ConnectorEvent>>>>,

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
