use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ProjectConfig {
    pub name: String,
    pub filters: ProjectFilters,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ProjectFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    mac: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<Vec<String>>,
}
