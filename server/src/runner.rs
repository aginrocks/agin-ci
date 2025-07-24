use mongodb::Database;
use pulsar::{Pulsar, TokioExecutor};
use std::sync::Arc;

pub struct Runner {
    pub database: Database,
    pub pulsar: Arc<Pulsar<TokioExecutor>>,
}

impl Runner {
    pub fn new(database: Database, pulsar: Arc<Pulsar<TokioExecutor>>) -> Self {
        Self { database, pulsar }
    }
}
