use color_eyre::eyre::Result;
use socketioxide::{SocketIo, extract::SocketRef};
use tracing::info;

pub async fn init_io(io: &SocketIo) -> Result<()> {
    io.ns("/", |s: SocketRef| {
        info!("new connection");

        // s.on("join", room_join_handler);

        // s.on("leave", room_disconnect);

        // s.on("message", message_handler);

        // s.on("song_select", song_select);

        // s.on("start", start_game);

        // s.on_disconnect(room_disconnect);
    });

    Ok(())
}
