use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{connectors::ConnectorConfig, databases::GlobalDBConfig};

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct GlobalConfig {
    /// Defines global databases that can be written to by projects.
    pub databases: HashMap<String, GlobalDBConfig>,

    /// Defines connectors that can receive measurements.
    pub connectors: HashMap<String, ConnectorConfig>,
}
