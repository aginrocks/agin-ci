mod health;
mod login;
mod organizations;
mod user;

use serde::Serialize;
use utoipa::{ToSchema, schema};

use super::Route;

pub fn routes() -> Vec<Route> {
    [
        health::routes(),
        user::routes(),
        login::routes(),
        organizations::routes(),
    ]
    .concat()
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"success": true}))]
pub struct CreateSuccess {
    success: bool,
    id: String,
}
