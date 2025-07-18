use aginci_core::runner_messages::report_progress::ProgressReport;
use socketioxide::extract::{Data, Extension};
use tracing::info;

use crate::socket::UserData;

pub async fn handler(Data(progress): Data<ProgressReport>, Extension(data): Extension<UserData>) {
    info!("Received progress report: {:?}", progress);
}
