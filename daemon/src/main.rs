pub mod global_config;
mod organizations;
mod project_config;
mod schema;
mod state;

use aginsensors_core::connector::ConnectorRunner;
use color_eyre::eyre::{Context, Result};
use database_influx::{DatabaseTypeInflux, LocalConfigInflux};
use modules::databases;
use tracing::level_filters::LevelFilter;
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{schema::write_schema, state::get_app_state};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    color_eyre::install()?;

    init_tracing().wrap_err("failed to set global tracing subscriber")?;

    write_schema().await?;

    let state = get_app_state().await;

    // println!("Hello, world!");

    // let base = state.databases.get("influx").unwrap();

    // for i in 0..10 {
    //     db.write_measurements(vec![Measurement {
    //         timestamp: Local::now().timestamp_millis(),
    //         measurement: "something".to_string(),
    //         // bucket: Some("test-bucket".to_string()),
    //         values: HashMap::from([("value".to_string(), (i as f64))]),
    //     }])
    //     .await?;
    //     thread::sleep(Duration::from_millis(1));
    // }

    // let mesurement = db.get_last_measurement().await?;

    dbg!(&state.connectors);

    // Initialize all connectors and collect their receivers
    let mut connector_tasks = Vec::new();

    for (name, connector) in &state.connectors {
        tracing::info!("Initializing connector: {}", name);
        let mut receiver = connector.run();
        let connector_name = name.clone();

        // Spawn a task to handle events from this connector
        let task = tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                tracing::info!("Received event from connector '{}'", connector_name);

                match &event.body {
                    aginsensors_core::connector::ConnectorEventBody::Measurements(measurements) => {
                        for measurement in measurements {
                            tracing::info!(
                                "Measurements from '{}': {} = {:?} at {}",
                                connector_name,
                                measurement.measurement,
                                measurement.values,
                                measurement.timestamp
                            );
                        }
                    }
                    aginsensors_core::connector::ConnectorEventBody::ReadRequest(read_request) => {
                        tracing::info!(
                            "Read request from '{}': {:?}",
                            connector_name,
                            read_request
                        );
                    }
                }
            }
        });

        connector_tasks.push(task);
    }

    tracing::info!("All connectors initialized, listening for events...");

    // Wait for all connector tasks to complete (they run indefinitely)
    for task in connector_tasks {
        if let Err(e) = task.await {
            tracing::error!("Connector task failed: {:?}", e);
        }
    }

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
