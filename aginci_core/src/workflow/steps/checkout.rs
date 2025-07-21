#[cfg(feature = "step_executor")]
use {
    crate::{
        runner_messages::report_progress::ProgressReport,
        workflow::step_executor::StepExecutorInner,
    },
    color_eyre::eyre::Result,
    std::{pin::Pin, sync::Arc},
    tokio::sync::broadcast::Sender,
};

use crate::define_step;

define_step!("aginci/checkout", CheckoutStep {});

#[cfg(feature = "step_executor")]
impl StepExecutorInner for CheckoutStep {
    fn execute(
        self: Arc<Self>,
        progress_tx: Sender<ProgressReport>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'static>> {
        Box::pin(async move { Ok(()) })
    }
}
