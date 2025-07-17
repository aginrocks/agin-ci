mod get_job;

use color_eyre::eyre::Result;
use socketioxide::{SocketIo, extract::SocketRef};
use tracing::info;

pub async fn init_io(io: &SocketIo) -> Result<()> {
    io.ns("/", |s: SocketRef| {
        info!("new connection");

        s.on("get_job", get_job::handler)
    });

    Ok(())
}
