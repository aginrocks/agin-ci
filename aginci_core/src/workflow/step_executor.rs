use color_eyre::eyre::Result;
use enum_dispatch::enum_dispatch;
use std::pin::Pin;

use crate::runner_messages::report_progress::ProgressReport;

pub type ReportCallback =
    Box<dyn Fn(ProgressReport) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

#[enum_dispatch(Step)]
#[cfg(feature = "step_executor")]
pub trait StepExecutor {
    // TODO: Add execution context and results
    fn execute(
        &self,
        report_callback: ReportCallback,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;
}
