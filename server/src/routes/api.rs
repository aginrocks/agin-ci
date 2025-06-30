mod health;
mod login;
mod user;

use super::Route;

pub fn routes() -> Vec<Route> {
    [health::routes(), user::routes(), login::routes()].concat()
}
