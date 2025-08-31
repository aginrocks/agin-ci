use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Triggers {
    pub push: Option<PushTrigger>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PushTrigger {
    pub branches: Option<Vec<String>>,
    pub paths: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}
