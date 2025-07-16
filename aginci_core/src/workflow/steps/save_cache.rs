use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::define_step;

#[derive(Serialize, Deserialize, JsonSchema)]
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
