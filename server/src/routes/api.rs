mod health;

use super::Route;

pub fn routes() -> Vec<Route> {
    [health::routes()].concat()
}
