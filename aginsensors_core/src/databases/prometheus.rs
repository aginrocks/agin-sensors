use async_trait::async_trait;

use crate::{database::Database, define_database};

define_database!("prometheus", Prometheus {});

#[async_trait]
impl Database for Prometheus {
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
