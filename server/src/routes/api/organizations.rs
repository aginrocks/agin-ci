use axum::{Extension, Json};
use color_eyre::eyre::{self, Context, ContextCompat};
use mongodb::bson::doc;
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{MutableOrganization, Organization},
    middlewares::require_auth::{UnauthorizedError, UserData},
    routes::{RouteProtectionLevel, api::CreateSuccess},
    state::AppState,
};
use futures::TryStreamExt;

use super::Route;

const PATH: &str = "/api/organizations";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(get_organizations, create_organization),
        RouteProtectionLevel::Authenticated,
    )]
}

/// Get all organizations
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

/// Create a new organization
#[utoipa::path(
    method(post),
    path = PATH,
    request_body = MutableOrganization,
    responses(
        (status = OK, description = "Success", body = CreateSuccess),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    )
)]
async fn create_organization(
    Extension(user): Extension<UserData>,
    Extension(state): Extension<AppState>,
    body: Json<MutableOrganization>,
) -> AxumResult<Json<CreateSuccess>> {
    let already_exists = state
        .database
        .collection::<Organization>("organizations")
        .find_one(doc! { "slug": &body.slug })
        .await?;

    if already_exists.is_some() {
        return Err(AxumError::forbidden(eyre::eyre!(
            "Organization with this slug already exists"
        )));
    }

    let user_id = user.0.id.wrap_err("Missing user ID")?;

    let organization = Organization {
        id: None,
        name: body.name.clone(),
        description: body.description.clone(),
        slug: body.slug.clone(),
        members: vec![crate::database::Membership {
            user_id,
            role: crate::database::OrganizationRole::Admin,
        }],
    };

    let inserted_org = state
        .database
        .collection::<Organization>("organizations")
        .insert_one(organization)
        .await
        .wrap_err("Failed to create organization")?;

    let id = inserted_org
        .inserted_id
        .as_object_id()
        .wrap_err("Failed to fetch organization ID")?
        .to_string();

    Ok(Json(CreateSuccess { success: true, id }))
}
