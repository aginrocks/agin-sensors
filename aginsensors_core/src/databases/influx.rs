use async_trait::async_trait;
use chrono::{DateTime, FixedOffset};
use color_eyre::{Result, eyre::Context};
use futures::prelude::*;
use influxdb2::{
    Client, FromDataPoint,
    models::{DataPoint, Query},
};

use crate::{
    database::{Database, IntoGlobalDB},
    databases::GlobalDB,
    define_database,
};

#[derive(Debug, FromDataPoint, Default)]
pub struct InfluxMeasurement {
    pub _time: DateTime<FixedOffset>,
}

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

impl IntoGlobalInflux for GlobalConfigInflux {
    fn into_global_db(self) -> GlobalInflux {
        let client = Client::new(self.url, self.organization, self.token);

        GlobalInflux { client }
    }
}

#[async_trait]
impl Database for LocalInflux {
    async fn get_last_measurement(&self) -> color_eyre::eyre::Result<i64> {
        let last_measurement: Vec<InfluxMeasurement> = self
            .global
            .client
            .query(Some(Query::new(format!(
                "from(bucket: \"{}\") |> range(start: 1970-01-01T00:00:00Z) |> last()",
                self.config.bucket
            ))))
            .await?;

        dbg!(&last_measurement);

        Ok(last_measurement.first().map_or(0, |m| m._time.timestamp()))
    }

    async fn write_measurements(
        &self,
        measurement: Vec<crate::connector::Measurement>,
    ) -> color_eyre::eyre::Result<()> {
        let measurement = measurement
            .into_iter()
            .map(|m| -> Result<DataPoint> {
                let mut datapoint = DataPoint::builder(m.measurement).timestamp(m.timestamp);
                for (k, v) in m.values.iter() {
                    datapoint = datapoint.field(k, *v);
                }
                datapoint.build().wrap_err("Failed to build DataPoint")
            })
            .collect::<Result<Vec<_>>>()?;

        self.global
            .client
            .write_with_precision(
                &self.config.bucket,
                stream::iter(measurement),
                influxdb2::api::write::TimestampPrecision::Milliseconds,
            )
            .await?;

        Ok(())
    }
}
