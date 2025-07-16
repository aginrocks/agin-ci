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
