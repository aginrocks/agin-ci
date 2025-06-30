use color_eyre::eyre::Result;
use mongodb::{Client, Database};
use tower_sessions::{Expiry, SessionManagerLayer, cookie::time::Duration};
use tower_sessions_mongodb_store::{MongoDBStore, mongodb::Client as SessionClient};

use crate::settings::Settings;

pub async fn init_database(settings: &Settings) -> Result<Database> {
    let client = Client::with_uri_str(&settings.db.connection_string).await?;
    let database = client.database(&settings.db.database_name);

    Ok(database)
}

pub async fn init_session_store(settings: &Settings) -> Result<SessionManagerLayer<MongoDBStore>> {
    let client = SessionClient::with_uri_str(&settings.db.connection_string).await?;
    let session_store = MongoDBStore::new(client, settings.db.database_name.clone());

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(10)));

    Ok(session_layer)
}
