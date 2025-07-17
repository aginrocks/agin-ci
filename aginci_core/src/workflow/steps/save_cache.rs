#[cfg(feature = "step_executor")]
use color_eyre::eyre::Result;
#[cfg(feature = "step_executor")]
use std::pin::Pin;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::define_step;
#[cfg(feature = "step_executor")]
use crate::workflow::step_executor::StepExecutor;

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
impl StepExecutor for SaveCacheStep {
    fn execute(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        Box::pin(async move { Ok(()) })
    }
}
