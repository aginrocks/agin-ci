use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use axum_valid::Valid;
use color_eyre::eyre;
use http::StatusCode;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::routes;
use validator::Validate;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{Secret, SecretScope},
    middlewares::{
        require_auth::UnauthorizedError,
        require_org_permissions::{ForbiddenError, OrgDataMember},
    },
    routes::{RouteProtectionLevel, api::CreateSuccess},
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/organizations/{org_slug}/secrets/{secret_id}";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(delete_organization_secret, edit_organization_secret),
        RouteProtectionLevel::Authenticated,
    )]
}

/// Delete org secret
#[utoipa::path(
    method(delete),
    path = PATH,
    params(
        ("org_slug" = String, Path, description = "Organization slug"),
        ("secret_id" = String, Path, description = "Secret ID"),
    ),
    responses(
        (status = NO_CONTENT, description = "Success"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "Secrets"
)]
async fn delete_organization_secret(
    org: OrgDataMember,
    State(state): State<AppState>,
    Path((_org_slug, secret_id)): Path<(String, String)>,
) -> AxumResult<impl IntoResponse> {
    let secret_id = ObjectId::parse_str(&secret_id)?;

    let delete_result = state
        .database
        .collection::<Secret>("secrets")
        .find_one_and_delete(doc! {
            "organization_id": org.id,
            "scope": SecretScope::Organization,
            "_id": secret_id
        })
        .await?;

    if delete_result.is_none() {
        return Err(AxumError::not_found(eyre::eyre!("Secret not found")));
    }

    Ok((StatusCode::NO_CONTENT, ()))
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct EditOrgSecretBody {
    #[validate(length(min = 1, max = 64))]
    pub name: Option<String>,
    pub secret: Option<String>,
}

/// Edit org secret
#[utoipa::path(
    method(patch),
    path = PATH,
    params(
        ("org_slug" = String, Path, description = "Organization slug"),
        ("secret_id" = String, Path, description = "Secret ID"),
    ),
    request_body = EditOrgSecretBody,
    responses(
        (status = OK, description = "Success", body = CreateSuccess),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "Secrets"
)]
async fn edit_organization_secret(
    org: OrgDataMember,
    State(state): State<AppState>,
    Path((_org_slug, secret_id)): Path<(String, String)>,
    Valid(Json(body)): Valid<Json<EditOrgSecretBody>>,
) -> AxumResult<Json<CreateSuccess>> {
    let secret_id = ObjectId::parse_str(&secret_id)?;

    let edit_result = state
        .database
        .collection::<Secret>("secrets")
        .find_one_and_update(
            doc! {
                "organization_id": org.id,
                "scope": SecretScope::Organization,
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
