pub mod dispatcher;

use std::sync::Arc;

use aginci_core::{
    pulsar::ToWorkerMessage,
    workflow::{self, Job},
};
use color_eyre::eyre::{Result, bail};
use mongo_utils::doc;
use mongodb::Database;
use pulsar::{Pulsar, TokioExecutor};
use uuid::Uuid;

use crate::database::Runner;

/// JobManager manages job dispatching and status tracking.
/// It also updates job statuses in the database.
pub struct JobManager {
    pub pulsar: Arc<Pulsar<TokioExecutor>>,
    pub database: Database,
}

impl JobManager {
    pub fn new(pulsar: Arc<Pulsar<TokioExecutor>>, database: Database) -> Self {
        Self { pulsar, database }
    }

    pub async fn dispatch_job(&self, job: Job) -> Result<()> {
        // TODO: Write a scheduling algorithm to select the best worker.
        // For now, we just select the first worker.

        let worker = self
            .database
            .collection::<Runner>("runners")
            .find_one(doc! {})
            .await?;

        if worker.is_none() {
            bail!("No workers available");
        }
        let worker = worker.unwrap();

        self.dispatch_job_to_worker(job, worker.uuid).await?;

        Ok(())
    }

    async fn dispatch_job_to_worker(&self, job: Job, runner_id: Uuid) -> Result<()> {
        let job_id = Uuid::new_v4();
        let run = workflow::JobRun { id: job_id, job };

        let topic = format!("persistent://aginci/{}/jobs", runner_id);

        let mut producer = self
            .pulsar
            .producer()
            .with_topic(topic)
            .with_name("dispatcher")
            .build()
            .await?;

        let message = ToWorkerMessage::JobRun(run);

        producer.send_non_blocking(message).await?;

        Ok(())
    }
}
