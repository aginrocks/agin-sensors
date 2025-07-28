use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::databases::GlobalDatabaseConfig;

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct GlobalConfig {
    pub databases: HashMap<String, GlobalDatabaseConfig>,
}
