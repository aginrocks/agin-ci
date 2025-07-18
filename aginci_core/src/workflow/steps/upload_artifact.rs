#[cfg(feature = "step_executor")]
use {
    crate::workflow::step_executor::{ReportCallback, StepExecutor},
    color_eyre::eyre::Result,
    std::pin::Pin,
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::define_step;

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
impl StepExecutor for UploadArtifactStep {
    fn execute(
        &self,
        report_callback: ReportCallback,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        Box::pin(async move { Ok(()) })
    }
}
