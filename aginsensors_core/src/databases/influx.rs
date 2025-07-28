use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use influxdb::Client;

use crate::{
    database::Database,
    databases::{GlobalDatabase, IntoClient},
    define_database,
};

define_database!(
    "influxdb",
    Influx,
    global_config = {
        pub url: String,
        pub token: String,
    },
    global_state = {
        pub client: Client,
    },
    local_config = {
        pub bucket: String,
    }
);

impl IntoClient for GlobalConfigInflux {
    fn into_client(self) -> GlobalDatabase {
        GlobalDatabase::Influx(GlobalInflux {
            client: Client::new(self.url, self.token),
        })
    }
}

#[async_trait]
impl Database for Influx {
    async fn connect(&self) -> color_eyre::eyre::Result<()> {
        Ok(())
    }

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
