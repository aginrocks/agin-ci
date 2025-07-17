use socketioxide::{
    SocketIo,
    extract::{Extension, SocketRef},
};
use tracing::info;

use crate::require_auth::UserData;

pub fn handler(socket: SocketRef, io: SocketIo, Extension(data): Extension<UserData>) {
    let name = data.0.job.name.unwrap_or("Unknown".to_string());

    info!("job: {name}");
}
