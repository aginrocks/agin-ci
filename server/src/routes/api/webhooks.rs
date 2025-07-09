mod common;
mod project_id;

use serde::Serialize;
use utoipa::ToSchema;

use super::Route;

pub fn routes() -> Vec<Route> {
    [project_id::routes()].concat()
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"success": true}))]
pub struct WebhookHandlerSuccess {
    pub success: bool,
}
