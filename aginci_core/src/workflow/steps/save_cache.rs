use crate::define_step;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(feature = "step_executor")]
use {
    crate::runner_messages::report_progress::ProgressReport, color_eyre::eyre::Result,
    tokio::sync::broadcast::Sender,
};

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct SaveCacheStepWith {
    /// Paths of folders to cache.
    pub paths: Vec<String>,
}

#[cfg(feature = "step_executor")]
async fn execute(step: SaveCacheStep, progress_tx: Sender<ProgressReport>) -> Result<()> {
    Ok(())
}

define_step!(
    "aginci/cache/save",
    SaveCacheStep {
        pub with: SaveCacheStepWith,
    },
    execute
);
