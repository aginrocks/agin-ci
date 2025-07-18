#[cfg(feature = "step_executor")]
use {color_eyre::eyre::Result, std::pin::Pin};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::define_step;
#[cfg(feature = "step_executor")]
use crate::workflow::step_executor::{ReportCallback, StepExecutor};

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct RestoreCacheStepWith {
    /// Paths of folders to cache.
    pub paths: Vec<String>,
}

define_step!(
    "aginci/cache/restore",
    RestoreCacheStep {
        with: RestoreCacheStepWith,
    }
);

#[cfg(feature = "step_executor")]
impl StepExecutor for RestoreCacheStep {
    fn execute(
        &self,
        report_callback: ReportCallback,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        Box::pin(async move { Ok(()) })
    }
}
