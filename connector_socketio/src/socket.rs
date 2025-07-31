mod handle_batch;
mod handle_single;

use color_eyre::eyre::Result;
use socketioxide::{SocketIo, extract::SocketRef};
use tracing::debug;

pub async fn init_io(io: &SocketIo) -> Result<()> {
    io.ns("/", on_connection);

    Ok(())
}

pub async fn on_connection(s: SocketRef) {
    debug!("new connection");

    s.on("measurement", handle_single::handler);
    s.on("measurements", handle_batch::handler);
}
