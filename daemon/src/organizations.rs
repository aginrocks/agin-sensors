use std::{collections::HashMap, sync::Arc};

use aginsensors_core::connector::Measurement;
use color_eyre::eyre::{ContextCompat, Result};
use modules::{
    databases::{GlobalDB, LocalDB, LocalDBConfig},
    modifiers::ModifierType,
};
use tokio::{
    fs::read_to_string,
    sync::{OnceCell, RwLock},
};

use crate::state::AppState;

macro_rules! define_filter {
    ($tag_value:literal, $struct_name:ident { $($field:tt)* }) => {
        paste::paste! {

            #[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone,)]
            pub struct $struct_name {
                r#type: [<Uses$struct_name>],
                $($field)*
            }

            #[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone,)]
            pub enum [<Uses$struct_name>] {
                #[serde(rename = $tag_value)]
                Value,
            }
        }
    };
}

define_filter!("macs", MacFilter { pub(crate) macs: Vec<String> });
define_filter!("tokens", TokenFilter { pub(crate) tokens: Vec<String> });

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
#[serde(untagged)]
pub enum Filter {
    MacFilter(MacFilter),
    TokenFilter(TokenFilter),
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
pub struct OrganizationDatabaseConfig {
    pub key: String,
    #[serde(flatten)]
    pub config: LocalDBConfig,
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
pub struct OrganizationYaml {
    pub name: String,
    pub filters: Vec<Filter>,
    pub buffer: Option<bool>,
    pub modifiers: Option<Vec<ModifierType>>,
    pub databases: Vec<OrganizationDatabaseConfig>,
}

#[derive(Clone, Debug)]
pub struct Organization {
    pub name: String,
    pub filters: Vec<Filter>,
    pub buffer: Option<Arc<RwLock<Vec<Measurement>>>>,
    pub modifiers: Option<Vec<ModifierType>>,
    pub databases: Vec<LocalDB>,
}

#[derive(Clone, Debug)]
pub struct OrganizationsState {
    pub organizations: HashMap<String, Organization>,
}

impl OrganizationsState {
    pub async fn try_load(appstate: Arc<AppState>) -> Result<Arc<Self>> {
        let organizations_config_path = appstate.config_folder_path.join("organizations.yaml");

        if !organizations_config_path.exists() {
            return Err(color_eyre::eyre::eyre!(
                "Organizations config file not found at: {}",
                organizations_config_path.display()
            ));
        }

        let config = read_to_string(&organizations_config_path).await?;

        let parsed_config: HashMap<String, OrganizationYaml> = serde_yaml::from_str(&config)
            .map_err(|e| color_eyre::eyre::eyre!("Failed to parse global config: {}", e))?;

        Ok(Arc::new(Self::from_parsed_yaml(
            parsed_config,
            appstate.databases.clone(),
        )))
    }

    fn from_parsed_yaml(
        config: HashMap<String, OrganizationYaml>,
        databases: HashMap<String, GlobalDB>,
    ) -> Self {
        let organizations = config
            .into_iter()
            .map(|(id, org_yaml)| {
                let buffer = if org_yaml.buffer.unwrap_or(false) {
                    Some(Arc::new(RwLock::new(Vec::new())))
                } else {
                    None
                };

                // Create local databases for the organization
                let databases = org_yaml
                    .databases
                    .iter()
                    .map(|db_config| {
                        databases
                            .get(&db_config.key)
                            .wrap_err(format!(
                                "Database '{}' not found in global databases",
                                db_config.key
                            ))
                            .unwrap()
                            .new_local_client(&db_config.config)
                    })
                    .collect::<Vec<_>>();

                (
                    id.clone(),
                    Organization {
                        name: org_yaml.name,
                        filters: org_yaml.filters,
                        modifiers: org_yaml.modifiers,
                        buffer,
                        databases,
                    },
                )
            })
            .collect();
        OrganizationsState { organizations }
    }
}

static ORG_STATE: OnceCell<Arc<OrganizationsState>> = OnceCell::const_new();

pub async fn get_app_state(appstate: Arc<AppState>) -> &'static Arc<OrganizationsState> {
    ORG_STATE
        .get_or_try_init(|| OrganizationsState::try_load(appstate))
        .await
        .expect("Failed to initialize AppState")
}
