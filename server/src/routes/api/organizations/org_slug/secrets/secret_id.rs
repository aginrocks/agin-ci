use axum::{Extension, Json, extract::Path, response::IntoResponse};
use color_eyre::eyre;
use http::StatusCode;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{Secret, SecretScope},
    middlewares::{
        require_auth::UnauthorizedError,
        require_org_permissions::{ForbiddenError, OrgId},
    },
    routes::{RouteProtectionLevel, api::CreateSuccess},
    state::AppState,
};

use super::{CreateOrgSecretBody, Route};

const PATH: &str = "/api/organizations/{org_slug}/secrets/{secret_id}";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(delete_organization_secret, edit_organization_secret),
        RouteProtectionLevel::OrgMember,
    )]
}

/// Delete org secret
#[utoipa::path(
    method(delete),
    path = PATH,
    responses(
        (status = NO_CONTENT, description = "Success"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "Secrets"
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
            "scope": mongodb::bson::to_bson(&SecretScope::Organization)?,
            "_id": secret_id
        })
        .await?;

    if delete_result.is_none() {
        return Err(AxumError::not_found(eyre::eyre!("Secret not found")));
    }

    Ok((StatusCode::NO_CONTENT, ()))
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EditOrgSecretBody {
    pub name: Option<String>,
    pub secret: Option<String>,
}

/// Edit org secret
#[utoipa::path(
    method(patch),
    path = PATH,
    request_body = EditOrgSecretBody,
    responses(
        (status = OK, description = "Success", body = CreateSuccess),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "Secrets"
)]
async fn edit_organization_secret(
    Extension(org_id): Extension<OrgId>,
    Extension(state): Extension<AppState>,
    Path((_org_slug, secret_id)): Path<(String, String)>,
    Json(body): Json<EditOrgSecretBody>,
) -> AxumResult<Json<CreateSuccess>> {
    let secret_id = ObjectId::parse_str(&secret_id)?;

    let edit_result = state
        .database
        .collection::<Secret>("secrets")
        .find_one_and_update(
            doc! {
                "organization_id": org_id.0,
                "scope": mongodb::bson::to_bson(&SecretScope::Organization)?,
                "_id": secret_id
            },
            {
                let mut update_doc = mongodb::bson::Document::new();
                if let Some(name) = &body.name {
                    update_doc.insert("name", name);
                }
                if let Some(secret) = &body.secret {
                    update_doc.insert("secret", secret);
                }
                doc! { "$set": update_doc }
            },
        )
        .await?;

    if edit_result.is_none() {
        return Err(AxumError::not_found(eyre::eyre!("Secret not found")));
    }

    Ok(Json(CreateSuccess {
        success: true,
        id: secret_id.to_string(),
    }))
}
