#[cfg(feature = "step_executor")]
use {
    crate::runner_messages::report_progress::ProgressReport, color_eyre::eyre::Result,
    tokio::sync::broadcast::Sender,
};

use crate::define_step;

#[cfg(feature = "step_executor")]
async fn execute(step: BuildStep, progress_tx: Sender<ProgressReport>) -> Result<()> {
    Ok(())
}

define_step!("aginci/build", BuildStep {}, execute);
