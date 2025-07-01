mod health;
mod login;
mod organizations;
mod user;

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
