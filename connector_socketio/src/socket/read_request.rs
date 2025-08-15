use std::sync::Arc;

use aginsensors_core::connector::{ConnectorEvent, EventMetadata, ReadRequest};
use socketioxide::extract::{Data, SocketRef, State};
use tokio::sync::{Mutex, oneshot::channel};
use tracing::error;

use crate::{SocketIo, middleware::extract_token, socket::handle_single::SingleMeasurement};

pub async fn handler(socket: SocketRef, State(state): State<SocketIo>) {
    let Ok(token) = extract_token(&socket) else {
        error!("No token found in socket extensions");
        return;
    };

    let metadata = EventMetadata::builder().auth_token(token.clone());

    let (tx, rx) = channel::<i64>();

    let handle = tokio::spawn(async move {
        let _ = state
            .tx
            .send(ConnectorEvent::new_read_request(
                ReadRequest::LastMeasurement {
                    sender: Arc::new(tx),
                },
                metadata,
            ))
            .await;
    });

    if let Ok(data) = rx.await {
        let emit = socket.emit("last", &data);

        if emit.is_err() {
            error!("Failed to emit last measurement: {:?}", emit.err());
        }
    } else {
        error!("Failed to receive last measurement response");
    }

    if let Err(e) = handle.await {
        error!("Error handling last measurement request: {}", e);
    }
}
