mod health;
mod login;
mod organizations;
mod user;
mod webhook_handler;

use serde::Serialize;
use utoipa::{ToSchema, schema};

use super::Route;

pub fn routes() -> Vec<Route> {
    [
        health::routes(),
        user::routes(),
        login::routes(),
        organizations::routes(),
        webhook_handler::routes(),
    ]
    .concat()
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"success": true,"id": "60c72b2f9b1d8c001c8e4f5a"}))]
pub struct CreateSuccess {
    success: bool,
    id: String,
}
