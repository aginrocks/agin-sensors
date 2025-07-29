#[macro_export]
macro_rules! define_databases {
    ( $( $path:ident::$name:ident ),* ) => {
        use $crate::database::{IntoGlobalDB};

        paste::paste! {
            #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
            #[serde(untagged)]
            #[enum_dispatch::enum_dispatch]
            pub enum LocalDBConfig {
                $($name($path::[<LocalConfig$name>])),*
            }

            #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
            #[serde(untagged)]
            #[enum_dispatch::enum_dispatch]
            pub enum GlobalDBConfig {
                $($name($path::[<GlobalConfig$name>])),*
            }

            #[derive(Clone, Debug)]
            pub enum GlobalDB {
                $($name($path::[<Global$name>])),*
            }

            #[derive(Clone, Debug)]
            pub enum LocalDB {
                $($name($path::[<Local$name>])),*
            }
        }
    };
}

#[macro_export]
macro_rules! define_database {
    (
        $tag_value:literal,
        $struct_name:ident,
        global_config = { $($global_config:tt)* },
        global_state = { $($global_state:tt)* },
        local_config = { $($local_config:tt)* }
    ) => {
        paste::paste! {
            #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
            pub struct [<LocalConfig$struct_name>] {
                 pub r#type: [<DatabaseType$struct_name>],

                 pub name: String,

                 $($local_config)*
            }

            #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
            pub struct [<GlobalConfig$struct_name>] {
                 pub r#type: [<DatabaseType$struct_name>],

                 $($global_config)*
            }

            #[derive(Clone, Debug)]
            pub struct [<Global$struct_name>] {
                 $($global_state)*
            }

            #[derive(Clone, Debug)]
            pub struct [<Local$struct_name>] {
                pub config: [<LocalConfig$struct_name>],

                pub global: [<Global$struct_name>],
            }

            #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
            pub enum [<DatabaseType$struct_name>] {
                #[serde(rename = $tag_value)]
                Value,
            }
        }
    };
}
