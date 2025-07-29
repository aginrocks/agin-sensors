use async_trait::async_trait;
use color_eyre::eyre::Result;

use crate::{connector::Measurement, databases::GlobalDB};

#[async_trait]
pub trait Database {
    /// Returns the last measurement timestamp from the database.
    async fn get_last_measurement(&self) -> Result<u64>;

    /// Wrties a batch of measurements to the database.
    async fn write_measurements(&self, measurement: Vec<Measurement>) -> Result<()>;
}

pub trait IntoGlobalDB {
    fn into_global_db(self) -> GlobalDB;
}
