use async_trait::async_trait;
use color_eyre::eyre::Result;
use enum_dispatch::enum_dispatch;

use crate::connector::Measurement;

#[enum_dispatch(DatabaseType)]
pub trait DatabaseDispatch {
    fn as_database(&self) -> &dyn Database;
}

#[async_trait]
pub trait Database {
    async fn connect(&self) -> Result<()>;

    /// Returns the last measurement timestamp from the database.
    async fn get_last_measurement(&self) -> Result<u64>;

    /// Wrties a batch of measurements to the database.
    async fn write_measurements(&self, measurement: Vec<Measurement>) -> Result<()>;
}
