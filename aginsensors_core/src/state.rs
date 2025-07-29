use color_eyre::eyre::Result;
use std::{collections::HashMap, sync::Arc};
use tokio::{fs::read_to_string, sync::OnceCell};

use crate::{database::IntoGlobalDB, databases::GlobalDB, global_config::GlobalConfig};

pub struct AppState {
    pub databases: HashMap<String, GlobalDB>,
}

impl AppState {
    pub async fn try_load() -> Result<Arc<Self>> {
        let config = read_to_string("global.yaml").await?;

        let parsed_config: GlobalConfig = serde_yaml::from_str(&config)
            .map_err(|e| color_eyre::eyre::eyre!("Failed to parse global config: {}", e))?;

        let databases = parsed_config
            .databases
            .into_iter()
            .map(|(db_name, db_config)| (db_name, db_config.into_global_db()))
            .collect::<HashMap<_, _>>();

        Ok(Arc::new(AppState { databases }))
    }
}

static APP_STATE: OnceCell<Arc<AppState>> = OnceCell::const_new();

pub async fn get_app_state() -> &'static Arc<AppState> {
    APP_STATE
        .get_or_try_init(AppState::try_load)
        .await
        .expect("Failed to initialize AppState")
}
