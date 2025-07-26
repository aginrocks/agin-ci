mod runners;
mod users;

use super::Route;

pub fn routes() -> Vec<Route> {
    [users::routes(), runners::routes()].concat()
}
