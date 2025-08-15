mod filter;
pub mod global_config;
mod organizations;
mod project_config;
mod schema;
mod state;

use std::sync::Arc;

use aginsensors_core::connector::{ConnectorEventBody, ConnectorRunner, Measurement, ReadRequest};
use aginsensors_core::database::Database;
use aginsensors_core::modifier::Modifier;
use color_eyre::eyre::{Context, Result};
use modules::databases::LocalDB;
use tokio::sync::RwLock;
use tracing::level_filters::LevelFilter;
use tracing::{error, info};
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

use crate::filter::{FilteredConnectorEvent, filter};
use crate::organizations::Organization;
use crate::{schema::write_schema, state::get_app_state};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    color_eyre::install()?;

    init_tracing().wrap_err("failed to set global tracing subscriber")?;

    write_schema().await?;

    let state = get_app_state().await;

    let organizations_state = organizations::get_app_state(state.clone()).await;

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

    // dbg!(&state.connectors);

    // Initialize all connectors and collect their receivers
    let mut connector_tasks = Vec::new();

    for (name, connector) in &state.connectors {
        info!("Initializing connector: {}", name);
        let mut receiver = connector.run();
        let connector_name = name.clone();

        // Spawn a task to handle events from this connector
        let task = tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                info!("Received event from connector '{}'", connector_name);

                let event = filter(event, organizations_state);

                let Ok(event) = event else {
                    error!("Error filtering event: {:?}", event.unwrap_err());
                    continue;
                };

                handle_filtered_event(&event)
                    .await
                    .wrap_err_with(|| {
                        format!("Failed to handle event from connector '{connector_name}'")
                    })
                    .unwrap_or_else(|e| {
                        error!("Error handling event: {:?}", e);
                    });
            }
        });

        connector_tasks.push(task);
    }

    info!("All connectors initialized, listening for events...");

    // Wait for all connector tasks to complete (they run indefinitely)
    for task in connector_tasks {
        if let Err(e) = task.await {
            error!("Connector task failed: {:?}", e);
        }
    }

    Ok(())
}

async fn handle_filtered_event(event: &FilteredConnectorEvent) -> Result<()> {
    match &event.body {
        ConnectorEventBody::Measurements(measurements) => {
            handle_measurements(event, measurements).await?;
        }
        ConnectorEventBody::ReadRequest(read_request) => {
            handle_read_request(event, read_request).await?;
        }
    }
    Ok(())
}

async fn handle_measurements(
    event: &FilteredConnectorEvent,
    measurements: &Vec<Measurement>,
) -> Result<()> {
    for measurement in measurements {
        let organizations = &event.organizations;

        for organization in organizations {
            let processed_measurements = if let Some(buffer) = &organization.buffer {
                process_buffer(organization, measurement, buffer.clone()).await?
            } else {
                vec![measurement.clone()]
            };

            for database in &organization.databases {
                database
                    .write_measurements(processed_measurements.clone())
                    .await?;
            }

            info!(
                "writing measurements for organization '{}' to databases {:?}: {:?}",
                organization.name, organization.databases, processed_measurements
            );
        }
    }

    Ok(())
}

async fn handle_read_request(
    event: &FilteredConnectorEvent,
    read_request: &ReadRequest,
) -> Result<()> {
    let organization = event
        .organizations
        .first()
        .ok_or_else(|| color_eyre::eyre::eyre!("No organizations found in the event"))?;

    let database = organization
        .databases
        .iter()
        .find(|db| matches!(db, LocalDB::Influx(_)))
        .ok_or_else(|| {
            color_eyre::eyre::eyre!(
                "No Influx database configured for organization '{}'",
                organization.name
            )
        })?;

    match read_request {
        ReadRequest::LastMeasurement { sender } => {
            // let last_measurement = database.get_last_measurement().await?;

            // let sender_arc = sender.lock().await.send(last_measurement);
            // if sender_arc.is_err() {
            //     error!("Failed to send last measurement timestamp");
            // }
        }
    }

    Ok(())
}

async fn process_buffer(
    organization: &Organization,
    measurement: &Measurement,
    buffer: Arc<RwLock<Vec<Measurement>>>,
) -> Result<Vec<Measurement>> {
    let mut buffer = buffer.write().await;
    // dbg!(&buffer);
    buffer.push(measurement.clone());

    let modifiers = organization.modifiers.clone().unwrap_or_default();

    let mut results: Vec<Measurement> = vec![measurement.clone()];
    for modifier in modifiers {
        let mod_result = modifier
            .calc(buffer.clone())
            .wrap_err_with(|| format!("Error applying modifier {modifier:?}"))?;
        results.extend(mod_result);
    }

    Ok(results)
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
