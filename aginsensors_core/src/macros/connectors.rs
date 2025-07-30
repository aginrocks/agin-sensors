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

                 pub name: String,

                 $($config)*
            }

            #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
            pub enum [<ConnectorType$struct_name>] {
                #[serde(rename = $tag_value)]
                Value,
            }
        }
    };
}
