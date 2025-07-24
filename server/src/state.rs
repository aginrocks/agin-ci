use std::sync::Arc;

use mongodb::Database;
use pulsar::{Pulsar, TokioExecutor};

use crate::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub settings: Arc<Settings>,
    pub pulsar: Arc<Pulsar<TokioExecutor>>,
}
