use aginci_core::runner_messages::report_progress::ProgressReport;
use socketioxide::{
    SocketIo,
    extract::{Data, Extension, SocketRef},
};
use tracing::info;

use crate::socket::UserData;

pub async fn handler(
    socket: SocketRef,
    io: SocketIo,
    Data(progress): Data<ProgressReport>,
    Extension(data): Extension<UserData>,
) {
    info!("Received progress report: {:?}", progress);
}
