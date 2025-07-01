use axum::{Extension, Json};
use mongodb::bson::doc;
use utoipa_axum::routes;

use crate::{
    axum_error::AxumResult,
    database::Organization,
    middlewares::require_auth::{UnauthorizedError, UserData},
    routes::RouteProtectionLevel,
    state::AppState,
};
use futures::TryStreamExt;

use super::Route;

const PATH: &str = "/api/organizations";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(get_organizations),
        RouteProtectionLevel::Authenticated,
    )]
}

/// Get all organizations you have access to
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = Vec<Organization>),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    )
)]
async fn get_organizations(
    Extension(user): Extension<UserData>,
    Extension(state): Extension<AppState>,
) -> AxumResult<Json<Vec<Organization>>> {
    let cursor = state
        .database
        .collection::<Organization>("organizations")
        .find(doc! {
            "members.user_id": user.0.id
        })
        .await?;

    let results = cursor.try_collect().await?;

    Ok(Json(results))
}
