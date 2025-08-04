mod socket;

use aginsensors_core::{
    connector::{ConnectorEvent, ConnectorRunner},
    define_connector,
};
use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};
use color_eyre::eyre::{Context, Result};
use socketioxide::{SocketIoBuilder, layer::SocketIoLayer};
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tracing::error;

use crate::socket::init_io;

define_connector!(
    "socketio",
    SocketIo,
    config = {
        pub port: u16,
    },
    state = {
        tx: Arc<Sender<Vec<ConnectorEvent>>>,
        rx: Arc<Receiver<Vec<ConnectorEvent>>>,
    }
);

impl SocketIoConnector for SocketIo {
    fn new(config: &ConfigSocketIo) -> Self {
        let (tx, rx) = channel::<Vec<ConnectorEvent>>(1000);

        SocketIo {
            config: config.clone(),
            tx: Arc::new(tx),
            rx: Arc::new(rx),
        }
    }
}

impl ConnectorRunner for SocketIo {
    fn run(&self) -> Arc<Receiver<Vec<ConnectorEvent>>> {
        let mut this = self.clone();
        tokio::spawn(async move { this.serve().await });

        self.rx.clone()
    }
}

impl SocketIo {
    pub async fn serve(&mut self) -> Result<()> {
        let (layer, io) = SocketIoBuilder::new()
            .with_state(self.clone())
            .build_layer();

        init_io(&io).await?;

        self.init_axum(layer)
            .await
            .expect("Failed to initialize axum server");

        Ok(())
    }

    async fn init_axum(&self, io_layer: SocketIoLayer) -> Result<()> {
        let app_state = self.clone();

        let app = Router::new()
            .route("/", get(root_handler))
            .fallback(|| async { (StatusCode::NOT_FOUND, "Not found").into_response() })
            .layer(io_layer)
            // .layer(from_fn_with_state(app_state.clone(), require_auth))
            .with_state(app_state); // Provide shared state here

        let listener = tokio::net::TcpListener::bind("0.0.0.0:37581")
            .await
            .wrap_err("Failed to bind")?;

        tokio::spawn(async move {
            let app = app.into_make_service();

            if let Err(err) = axum::serve(listener, app).await {
                error!("Server crashed: {:?}", err);
            }
        });

        Ok(())
    }
}

async fn root_handler() -> String {
    format!("Agin Sensors {}", env!("CARGO_PKG_VERSION"))
}
