use axum::{Extension, extract::Path, response::IntoResponse};
use color_eyre::eyre;
use http::StatusCode;
use mongodb::bson::{doc, oid::ObjectId};
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::Secret,
    middlewares::{
        require_auth::UnauthorizedError,
        require_org_permissions::{ForbiddenError, OrgId},
    },
    routes::RouteProtectionLevel,
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/organizations/{org_slug}/secrets/{secret_id}";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(delete_organization_secret),
        RouteProtectionLevel::OrgMember,
    )]
}

/// Delete organization secret
#[utoipa::path(
    method(delete),
    path = PATH,
    responses(
        (status = NO_CONTENT, description = "Success"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    )
)]
async fn delete_organization_secret(
    Extension(org_id): Extension<OrgId>,
    Extension(state): Extension<AppState>,
    Path((_org_slug, secret_id)): Path<(String, String)>,
) -> AxumResult<impl IntoResponse> {
    let secret_id = ObjectId::parse_str(&secret_id)?;

    let delete_result = state
        .database
        .collection::<Secret>("secrets")
        .find_one_and_delete(doc! {
            "organization_id": org_id.0,
            "_id": secret_id
        })
        .await?;

    if delete_result.is_none() {
        return Err(AxumError::not_found(eyre::eyre!("Secret not found")));
    }

    Ok((StatusCode::NO_CONTENT, ()))
}
