use serde_json::{Value, json};
use socketioxide::{
    SocketIo,
    extract::{AckSender, Extension, SocketRef},
};
use tracing::info;

use crate::socket::UserData;

pub async fn handler(
    socket: SocketRef,
    io: SocketIo,
    ack: AckSender,
    Extension(data): Extension<UserData>,
) {
    let name = data.0.job.clone().name.unwrap_or("Unknown".to_string());

    info!("job: {name}");

    ack.send(&data.0.job).ok();
}
