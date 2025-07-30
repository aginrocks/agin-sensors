use color_eyre::eyre::Result;
use schemars::schema_for;

use crate::{global_config::GlobalConfig, project_config::ProjectConfig};

pub async fn write_schema() -> Result<()> {
    let global_schema = schema_for!(GlobalConfig);
    let global_schema = serde_json::to_string(&global_schema)?;

    tokio::fs::write("global_schema.json", global_schema).await?;

    let project_schema = schema_for!(ProjectConfig);
    let project_schema = serde_json::to_string(&project_schema)?;

    tokio::fs::write("project_schema.json", project_schema).await?;

    Ok(())
}
