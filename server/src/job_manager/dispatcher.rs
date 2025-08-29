use std::sync::Arc;

use aginci_core::{
    pulsar::ToWorkerMessage,
    workflow::{CancelJob, JobRun},
};
use color_eyre::eyre::Result;
use pulsar::{Producer, Pulsar, TokioExecutor};
use uuid::Uuid;

pub struct WorkerDispatcher {
    producer: Producer<TokioExecutor>,
}

impl WorkerDispatcher {
    pub fn new(producer: Producer<TokioExecutor>) -> Self {
        Self { producer }
    }

    pub async fn new_connect(pulsar: Arc<Pulsar<TokioExecutor>>, worker_id: Uuid) -> Result<Self> {
        let topic = format!("persistent://aginci/{}/jobs", worker_id);
        let producer = pulsar
            .producer()
            .with_topic(topic)
            .with_name("dispatcher")
            .build()
            .await?;

        Ok(Self { producer })
    }

    pub async fn dispatch_job(&mut self, job: JobRun) -> Result<()> {
        let message = ToWorkerMessage::JobRun(job);

        self.producer.send_non_blocking(message).await?;

        Ok(())
    }

    pub async fn cancel_job(&mut self, job_id: Uuid) -> Result<()> {
        let cancel = CancelJob { id: job_id };
        let message = ToWorkerMessage::CancelJob(cancel);

        self.producer.send_non_blocking(message).await?;

        Ok(())
    }
}
