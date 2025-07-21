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
pub struct UploadArtifactStepWith {
    /// Path to the artifact to upload.
    pub path: String,

    /// Artifact name visible in the UI.
    pub name: String,
}

define_step!(
    "aginci/upload-artifact",
    UploadArtifactStep {
        with: UploadArtifactStepWith,
    }
);

#[cfg(feature = "step_executor")]
impl StepExecutorInner for UploadArtifactStep {
    fn execute(
        self: Arc<Self>,
        progress_tx: Sender<ProgressReport>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'static>> {
        Box::pin(async move { Ok(()) })
    }
}
