use socketioxide::extract::{AckSender, Extension};
use tracing::info;

use crate::socket::UserData;

pub async fn handler(ack: AckSender, Extension(data): Extension<UserData>) {
    let name = data.job.clone().job.name.unwrap_or("Unknown".to_string());

    info!("job: {name}");

    ack.send(&data.job.job).ok();
}
