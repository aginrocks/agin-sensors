mod influx;
mod prometheus;

use crate::database::{Database, DatabaseDispatch};
use enum_dispatch::enum_dispatch;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
#[enum_dispatch]
pub enum DatabaseType {
    Influx(influx::Influx),
    Prometheus(prometheus::Prometheus),
}

#[macro_export]
macro_rules! define_database {
    ($tag_value:literal, $struct_name:ident { $($field:tt)* }) => {
        paste::paste! {
            #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
            pub struct $struct_name {
                 pub r#type: [<DatabaseType$struct_name>],

                 $($field)*
            }

            #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
            pub enum [<DatabaseType$struct_name>] {
                #[serde(rename = $tag_value)]
                Value,
            }

            impl crate::database::DatabaseDispatch for $struct_name {
                fn as_database(&self) -> &dyn Database {
                    self
                }
            }
        }
    };
}
