#[macro_export]
macro_rules! define_databases {
    ( $( $path:ident::$name:ident ),* ) => {
        paste::paste! {
            #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
            #[serde(untagged)]
            pub enum LocalDBConfig {
                $($name($path::[<LocalConfig$name>])),*
            }

            #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
            #[serde(untagged)]
            pub enum GlobalDBConfig {
                $($name($path::[<GlobalConfig$name>])),*
            }

            #[derive(Clone, Debug)]
            pub enum GlobalDB {
                $($name($path::[<Global$name>])),*
            }

            impl GlobalDB {
                pub fn new_local_client(&self, config: &LocalDBConfig) -> LocalDB {
                    match self {
                        $(GlobalDB::$name(global) => LocalDB::$name($path::[<Local$name>] {
                            config: match config {
                                LocalDBConfig::$name(local_config) => local_config.clone(),
                            },
                            global: global.clone(),
                        })),*
                    }
                }
            }

            #[derive(Clone, Debug)]
            pub enum LocalDB {
                $($name($path::[<Local$name>])),*
            }

            impl $crate::database::IntoGlobalDB for GlobalDBConfig {
                fn into_global_db(self) -> GlobalDB {
                    $(use $path::[<IntoGlobal$name>] ;)*
                    match self {
                        $(GlobalDBConfig::$name(config) => GlobalDB::$name(config.into_global_db())),*
                    }
                }
            }

            #[async_trait::async_trait]
            impl $crate::database::Database for LocalDB {
                async fn get_last_measurement(&self) -> color_eyre::eyre::Result<i64> {
                    match self {
                        $(LocalDB::$name(db) => db.get_last_measurement().await),*
                    }
                }

                async fn write_measurements(&self, measurement: Vec<$crate::connector::Measurement>) -> color_eyre::eyre::Result<()> {
                    match self {
                        $(LocalDB::$name(db) => db.write_measurements(measurement).await),*
                    }
                }
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

            pub trait [<IntoGlobal$struct_name>] {
                fn into_global_db(self) -> [<Global$struct_name>];
            }
        }
    };
}
