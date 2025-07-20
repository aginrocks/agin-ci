use aginci_core::runner_messages::report_progress::ProgressReport;
use socketioxide::extract::{Data, Extension};
use tracing::debug;

use crate::socket::UserData;

pub async fn handler(Data(progress): Data<ProgressReport>, Extension(data): Extension<UserData>) {
    debug!("Received progress report: {:?}", progress);
    let _ = data.progress_tx.send(progress);
}
