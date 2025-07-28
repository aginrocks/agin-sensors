use color_eyre::eyre::Result;
use std::collections::HashMap;
use tokio::fs::read_to_string;

use crate::{databases::GlobalDatabase, global_config::GlobalConfig};

pub struct AppState {
    pub databases: HashMap<String, GlobalDatabase>,
}

impl AppState {
    pub async fn try_load() -> Result<Self> {
        let config = read_to_string("global.yaml").await?;

        let parsed_config: GlobalConfig = serde_yaml::from_str(&config)
            .map_err(|e| color_eyre::eyre::eyre!("Failed to parse global config: {}", e))?;

        let mut databases: HashMap<String, GlobalDatabase> = HashMap::new();

        // for (db_name, db_config) in parsed_config.databases.iter() {
        //     let client = GlobalDatabase::from(db_config);
        // }

        todo!()
    }
}
