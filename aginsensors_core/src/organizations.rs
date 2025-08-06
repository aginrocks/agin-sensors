use std::{collections::HashMap, path::PathBuf, sync::Arc};

use color_eyre::eyre::Result;
use tokio::{fs::read_to_string, sync::OnceCell};

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

define_filter!("macs", MacFilter { macs: Vec<String> });
define_filter!("tokens", TokenFilter { tokens: Vec<String> });

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
#[serde(untagged)]
pub enum Filter {
    MacFilter(MacFilter),
    TokenFilter(TokenFilter),
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub bucket: String,
    pub filters: Vec<Filter>,
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
pub struct OrganizationsState {
    pub organizations: HashMap<String, Organization>,
}

impl OrganizationsState {
    pub async fn try_load(config_folder_path: PathBuf) -> Result<Arc<Self>> {
        let organizations_config_path = config_folder_path.join("organizations.yaml");

        if !organizations_config_path.exists() {
            return Err(color_eyre::eyre::eyre!(
                "Organizations config file not found at: {}",
                organizations_config_path.display()
            ));
        }

        let config = read_to_string(&organizations_config_path).await?;

        let parsed_config: Self = serde_yaml::from_str(&config)
            .map_err(|e| color_eyre::eyre::eyre!("Failed to parse global config: {}", e))?;

        Ok(Arc::new(parsed_config))
    }
}

static ORG_STATE: OnceCell<Arc<OrganizationsState>> = OnceCell::const_new();

pub async fn get_app_state(config_folder_path: PathBuf) -> &'static Arc<OrganizationsState> {
    ORG_STATE
        .get_or_try_init(|| OrganizationsState::try_load(config_folder_path))
        .await
        .expect("Failed to initialize AppState")
}
