mod connector;
pub mod connectors;
pub mod database;
pub mod databases;
pub mod global_config;
pub mod macros;
mod project_config;
mod schema;
mod state;

use std::{collections::HashMap, thread, time::Duration};

use chrono::Local;
use color_eyre::eyre::{Context, Result};
use tracing::level_filters::LevelFilter;
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    database::Database, databases::influx::LocalConfigInflux, schema::write_schema,
    state::get_app_state,
};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    init_tracing().wrap_err("failed to set global tracing subscriber")?;

    let state = get_app_state().await;

    write_schema().await?;

    println!("Hello, world!");

    let base = state.databases.get("influx").unwrap();

    let db = base.new_local_client(&databases::LocalDBConfig::Influx(LocalConfigInflux {
        r#type: crate::databases::influx::DatabaseTypeInflux::Value,
        name: "Influx".to_string(),
        bucket: "test-bucket".to_string(),
    }));

    for i in 0..10 {
        db.write_measurements(vec![connector::Measurement {
            timestamp: Local::now().timestamp_millis(),
            measurement: "something".to_string(),
            bucket: Some("test-bucket".to_string()),
            values: HashMap::from([("value".to_string(), (i as f64))]),
        }])
        .await?;
        thread::sleep(Duration::from_millis(1));
    }

    let mesurement = db.get_last_measurement().await?;

    dbg!(mesurement);

    Ok(())
}

fn init_tracing() -> Result<()> {
    tracing_subscriber::Registry::default()
        .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .with(ErrorLayer::default())
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .with_env_var("RUST_LOG")
                .from_env()?,
        )
        .try_init()?;

    Ok(())
}
