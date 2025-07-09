mod gitea;
mod github;

use super::Route;

pub fn routes() -> Vec<Route> {
    [github::routes(), gitea::routes()].concat()
}
