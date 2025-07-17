#[cfg(feature = "step_executor")]
use std::pin::Pin;

#[cfg(feature = "step_executor")]
use color_eyre::eyre::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::define_step;
#[cfg(feature = "step_executor")]
use crate::workflow::step_executor::StepExecutor;

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
    fn execute(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        Box::pin(async move { Ok(()) })
    }
}
