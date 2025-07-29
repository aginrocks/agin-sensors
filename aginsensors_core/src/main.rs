use color_eyre::eyre::{Context, Result};
use tracing::level_filters::LevelFilter;
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{schema::write_schema, state::get_app_state, database::Database};

mod connector;
pub mod database;
pub mod databases;
pub mod global_config;
pub mod macros;
mod project_config;
mod schema;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    init_tracing().wrap_err("failed to set global tracing subscriber")?;

    let state = get_app_state().await;

    write_schema().await?;

    println!("Hello, world!");

    let base = state.databases.get("influxdb").unwrap();

    base

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
