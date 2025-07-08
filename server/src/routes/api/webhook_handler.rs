mod common;
mod github;

use serde::Serialize;
use utoipa::ToSchema;

use super::Route;

pub fn routes() -> Vec<Route> {
    [github::routes()].concat()
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"success": true}))]
pub struct WebhookHandlerSuccess {
    pub success: bool,
}
