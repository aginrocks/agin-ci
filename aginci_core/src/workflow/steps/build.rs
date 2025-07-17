#[cfg(feature = "step_executor")]
use async_trait::async_trait;
#[cfg(feature = "step_executor")]
use color_eyre::eyre::Result;
#[cfg(feature = "step_executor")]
use std::pin::Pin;

use crate::define_step;
#[cfg(feature = "step_executor")]
use crate::workflow::step_executor::StepExecutor;

define_step!("aginci/build", BuildStep { test: String });

#[cfg(feature = "step_executor")]
#[async_trait]
impl StepExecutor for BuildStep {
    fn execute(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        Box::pin(async move { Ok(()) })
    }
}
