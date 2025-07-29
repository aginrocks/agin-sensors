use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use influxdb::Client;

use crate::{
    database::{Database, IntoGlobalDB},
    databases::GlobalDB,
    define_database,
};

define_database!(
    "influxdb",
    Influx,
    global_config = {
        pub url: String,
        pub token: String,
        pub organization: String,
    },
    global_state = {
        pub client: Client,
    },
    local_config = {
        pub bucket: String,
    }
);

impl IntoGlobalDB for GlobalConfigInflux {
    fn into_global_db(self) -> GlobalDB {
        let client = Client::new(self.url, self.organization).with_token(self.token);

        GlobalDB::Influx(GlobalInflux { client })
    }
}

#[async_trait]
impl Database for LocalInflux {
    async fn get_last_measurement(&self) -> color_eyre::eyre::Result<u64> {
        Ok(0)
    }

    async fn write_measurements(
        &self,
        measurement: Vec<crate::connector::Measurement>,
    ) -> color_eyre::eyre::Result<()> {
        Ok(())
    }
}
