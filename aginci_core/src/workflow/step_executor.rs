use enum_dispatch::enum_dispatch;
#[cfg(feature = "step_executor")]
use tokio::sync::broadcast::Receiver;

use crate::runner_messages::report_progress::ProgressReport;

#[enum_dispatch(Step)]
#[cfg(feature = "step_executor")]
pub trait StepExecutor {
    fn execute(&self) -> Receiver<ProgressReport>;
}
