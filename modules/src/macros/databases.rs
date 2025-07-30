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
            impl aginsensors_core::database::Database for LocalDB {
                async fn get_last_measurement(&self) -> color_eyre::eyre::Result<i64> {
                    match self {
                        $(LocalDB::$name(db) => db.get_last_measurement().await),*
                    }
                }

                async fn write_measurements(&self, measurement: Vec<aginsensors_core::connector::Measurement>) -> color_eyre::eyre::Result<()> {
                    match self {
                        $(LocalDB::$name(db) => db.write_measurements(measurement).await),*
                    }
                }
            }
        }
    };
}
