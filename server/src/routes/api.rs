mod health;
mod login;
mod organizations;
mod schema;
mod tokens;
mod user;
mod webhooks;

use serde::Serialize;
use utoipa::{ToSchema, schema};

use super::Route;

pub fn routes() -> Vec<Route> {
    [
        health::routes(),
        user::routes(),
        login::routes(),
        organizations::routes(),
        webhooks::routes(),
        tokens::routes(),
        schema::routes(),
    ]
    .concat()
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"success": true,"id": "60c72b2f9b1d8c001c8e4f5a"}))]
pub struct CreateSuccess {
    success: bool,
    id: String,
}
