use aginci_core::workflow::WORKFLOW_SCHEMA;
use axum::Json;
use serde_json::Value;
use utoipa_axum::routes;

use crate::routes::RouteProtectionLevel;

use super::Route;

const PATH: &str = "/api/schema/workflow";

pub fn routes() -> Vec<Route> {
    vec![(routes!(get_workflow_schema), RouteProtectionLevel::Public)]
}

/// Get workflow schema
///
/// Returns the JSON schema for the workflow definition file.
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = str, content_type = "application/json")
    ),
    tag = "Schema"
)]
async fn get_workflow_schema() -> Json<Value> {
    Json(WORKFLOW_SCHEMA.clone())
}
