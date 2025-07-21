use aginci_core::runner_messages::report_progress::OrderedReport;
use socketioxide::extract::{Data, Extension};
use tracing::debug;

use crate::socket::UserData;

pub async fn handler(
    Data(OrderedReport { ord, body }): Data<OrderedReport>,
    Extension(data): Extension<UserData>,
) {
    debug!("Received progress report {ord}: {:?}", body);
    let _ = data.progress_tx.send(body);
}
