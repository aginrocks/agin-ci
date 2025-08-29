use std::sync::Arc;

use aginci_core::pulsar::ToWorkerMessage;
use color_eyre::eyre::Result;
use librunner::WorkflowRunner;
use pulsar::{Consumer, TokioExecutor, consumer::Message};
use tracing::{error, info};

pub async fn handle_message(
    consumer: &mut Consumer<ToWorkerMessage, TokioExecutor>,
    runner: &Arc<WorkflowRunner>,
    msg: Message<ToWorkerMessage>,
) -> Result<()> {
    consumer.ack(&msg).await?;
    let data = msg.deserialize()?;
    dbg!(&data);
    match data {
        ToWorkerMessage::JobRun(data) => {
            info!("Received JobRun: {:?}", data);

            let runner = runner.clone();
            tokio::spawn(async move {
                let workflow_id = data.id;
                runner.run_workflow(data).await.map_err(|e| {
                    error!(error = %e, ?workflow_id, "Error running workflow");
                })
            });
        }
        ToWorkerMessage::CancelJob(data) => {
            // TODO: Cancel the job
            info!("Received CancelJob: {:?}", data);
        }
    }

    Ok(())
}
