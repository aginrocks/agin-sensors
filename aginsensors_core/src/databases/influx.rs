use async_trait::async_trait;
use influxdb::{Client, ReadQuery};

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
        let last_measurement = self
            .global
            .client
            .query(ReadQuery::new(format!(
                "from(bucket: {}) |> range(start: 1970-01-01T00:00:00Z) |> last()",
                self.config.bucket
            )))
            .await?;

        dbg!(last_measurement);

        Ok(0)
    }

    async fn write_measurements(
        &self,
        _measurement: Vec<crate::connector::Measurement>,
    ) -> color_eyre::eyre::Result<()> {
        Ok(())
    }
}
