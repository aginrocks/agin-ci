#[cfg(feature = "step_executor")]
use {color_eyre::eyre::Result, std::pin::Pin};

use crate::define_step;
#[cfg(feature = "step_executor")]
use crate::workflow::step_executor::StepExecutor;

define_step!("aginci/checkout", CheckoutStep {});

#[cfg(feature = "step_executor")]
impl StepExecutor for CheckoutStep {
    fn execute(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        Box::pin(async move { Ok(()) })
    }
}
