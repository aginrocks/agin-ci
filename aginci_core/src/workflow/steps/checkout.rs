#[cfg(feature = "step_executor")]
use {
    crate::workflow::step_executor::{ReportCallback, StepExecutor},
    color_eyre::eyre::Result,
    std::pin::Pin,
};

use crate::define_step;

define_step!("aginci/checkout", CheckoutStep {});

#[cfg(feature = "step_executor")]
impl StepExecutor for CheckoutStep {
    fn execute(
        &self,
        report_callback: ReportCallback,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        Box::pin(async move { Ok(()) })
    }
}
