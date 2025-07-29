mod influx;

use crate::database::{Database, DatabaseDispatch};
use enum_dispatch::enum_dispatch;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
#[enum_dispatch]
pub enum DatabaseType {
    Influx(influx::Influx),
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(untagged)]
#[enum_dispatch]
pub enum GlobalDatabaseConfig {
    Influx(influx::GlobalConfigInflux),
}

#[enum_dispatch]
pub enum GlobalDatabase {
    Influx(influx::GlobalInflux),
}

#[enum_dispatch(GlobalDatabaseConfig)]
pub trait IntoClient {
    fn into_client(self) -> GlobalDatabase;
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
            pub struct $struct_name {
                 pub r#type: [<DatabaseType$struct_name>],

                 pub name: String,

                 #[serde(skip)]
                 pub global_state: Option<[<Global$struct_name>]>,

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

            #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
            pub enum [<DatabaseType$struct_name>] {
                #[serde(rename = $tag_value)]
                Value,
            }

            impl $crate::database::DatabaseDispatch for $struct_name {
                fn as_database(&self) -> &dyn Database {
                    self
                }
            }
        }
    };
}
