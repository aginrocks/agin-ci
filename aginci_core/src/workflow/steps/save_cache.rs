#[cfg(feature = "step_executor")]
use std::sync::Arc;

#[cfg(feature = "step_executor")]
use {crate::workflow::step_executor::StepExecutorInner, color_eyre::eyre::Result, std::pin::Pin};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "step_executor")]
use tokio::sync::broadcast::Sender;

use crate::define_step;
#[cfg(feature = "step_executor")]
use crate::runner_messages::report_progress::ProgressReport;

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct SaveCacheStepWith {
    /// Paths of folders to cache.
    pub paths: Vec<String>,
}

define_step!(
    "aginci/cache/save",
    SaveCacheStep {
        pub with: SaveCacheStepWith,
    }
);

#[cfg(feature = "step_executor")]
impl StepExecutorInner for SaveCacheStep {
    fn execute(
        self: Arc<Self>,
        progress_tx: Sender<ProgressReport>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'static>> {
        Box::pin(async move { Ok(()) })
    }
}
