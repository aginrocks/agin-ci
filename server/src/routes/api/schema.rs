mod workflow;

use super::Route;

pub fn routes() -> Vec<Route> {
    [workflow::routes()].concat()
}
