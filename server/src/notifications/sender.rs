use color_eyre::eyre::Result;
use mongodb::Database;
use tokio::task::JoinHandle;
use tracing::info;

use crate::database::PartialNotification;

use super::Simple;

#[derive(Clone)]
pub struct NotificationSender {
    pub database: Database,
}

impl NotificationSender {
    pub fn send(&self, notification: PartialNotification<Simple>) -> JoinHandle<Result<()>> {
        info!("Sending notification");
        let database = self.database.clone();

        let handle: JoinHandle<Result<()>> = tokio::spawn(async move {
            database
                .collection::<PartialNotification<Simple>>("notifications")
                .insert_one(notification)
                .await?;
            info!("Sent");

            Ok(())
        });
        handle
    }
}
