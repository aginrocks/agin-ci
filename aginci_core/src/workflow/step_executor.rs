use color_eyre::eyre::Result;
use enum_dispatch::enum_dispatch;
use std::pin::Pin;

#[enum_dispatch(Step)]
#[cfg(feature = "step_executor")]
pub trait StepExecutor {
    // TODO: Add execution context and results
    fn execute(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;
}
