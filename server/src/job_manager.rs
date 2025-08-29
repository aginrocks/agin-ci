pub mod dispatcher;

use aginci_core::workflow::{self, Job};
use color_eyre::eyre::Result;
use mongodb::Database;
use pulsar::{Pulsar, TokioExecutor};
use uuid::Uuid;

/// JobManager manages job dispatching and status tracking.
/// It also updates job statuses in the database.
pub struct JobManager {
    pub pulsar: Pulsar<TokioExecutor>,
    pub database: Database,
}

impl JobManager {
    pub fn new(pulsar: Pulsar<TokioExecutor>, database: Database) -> Self {
        Self { pulsar, database }
    }

    pub fn dispatch_job(&self, job: Job) -> Result<()> {
        // TODO: Write a scheduling algorithm to select the best worker.
        // For now, we just select the first worker.

        Ok(())
    }

    fn dispatch_job_to_worker(&self, job: Job, worker_id: Uuid) -> Result<()> {
        let job_id = Uuid::new_v4();
        let run = workflow::JobRun { id: job_id, job };

        Ok(())
    }
}
