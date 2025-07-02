use crate::{database, validators::slug_validator};
use color_eyre::eyre::Result;
use mongodb::{
    Client, Database,
    bson::{doc, oid::ObjectId},
};
use serde::{Deserialize, Serialize};
use tower_sessions::{
    Expiry, SessionManagerLayer,
    cookie::{SameSite, time::Duration},
};
use tower_sessions_redis_store::{
    RedisStore,
    fred::prelude::{ClientLike, Config, Pool},
};
use utoipa::ToSchema;
use validator::Validate;

use crate::mongo_id::{object_id_as_string, object_id_as_string_required};
use crate::settings::Settings;

pub async fn init_database(settings: &Settings) -> Result<Database> {
    let client = Client::with_uri_str(&settings.db.connection_string).await?;
    let database = client.database(&settings.db.database_name);

    Ok(database)
}

pub async fn init_session_store(
    settings: &Settings,
) -> Result<SessionManagerLayer<RedisStore<Pool>>> {
    let config = Config::from_url(&settings.redis.connection_string)?;
    let pool = Pool::new(config, None, None, None, 6)?;

    let _redis_conn = pool.connect();
    pool.wait_for_connect().await?;

    let session_store = RedisStore::<Pool>::new(pool);

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::days(7)));

    Ok(session_layer)
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct User {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        with = "object_id_as_string"
    )]
    #[schema(value_type = Option<String>)]
    pub id: Option<ObjectId>,
    pub subject: String,
    pub name: String,
    pub email: String,
}

// NOTE: The order is VERY IMPORTANT, from least to most privileged.
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum OrganizationRole {
    Viewer,
    Member,
    Admin,
    Owner,
}

impl From<OrganizationRole> for mongodb::bson::Bson {
    fn from(scope: OrganizationRole) -> Self {
        mongodb::bson::to_bson(&scope).expect("Failed to convert to BSON")
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct Membership {
    #[schema(value_type = String)]
    #[serde(with = "object_id_as_string_required")]
    pub user_id: ObjectId,
    pub role: OrganizationRole,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum SecretScope {
    Organization,
    Project,
}

impl From<SecretScope> for mongodb::bson::Bson {
    fn from(scope: SecretScope) -> Self {
        mongodb::bson::to_bson(&scope).expect("Failed to convert to BSON")
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct Secret {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        with = "object_id_as_string"
    )]
    #[schema(value_type = Option<String>)]
    pub id: Option<ObjectId>,

    pub name: String,

    pub scope: SecretScope,

    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    pub organization_id: ObjectId,

    #[serde(with = "object_id_as_string")]
    #[schema(value_type = Option<String>)]
    pub project_id: Option<ObjectId>,

    pub secret: String,
}

/// Secret object that can be safely sent to the client
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct PublicSecret {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        with = "object_id_as_string"
    )]
    #[schema(value_type = Option<String>)]
    pub id: Option<ObjectId>,

    pub name: String,

    pub scope: SecretScope,

    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    pub organization_id: ObjectId,

    #[serde(with = "object_id_as_string")]
    #[schema(value_type = Option<String>)]
    pub project_id: Option<ObjectId>,
}

impl Secret {
    pub fn to_public(&self) -> PublicSecret {
        PublicSecret {
            id: self.id,
            name: self.name.clone(),
            scope: self.scope.clone(),
            organization_id: self.organization_id,
            project_id: self.project_id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct Organization {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        with = "object_id_as_string"
    )]
    #[schema(value_type = Option<String>)]
    pub id: Option<ObjectId>,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub members: Vec<Membership>,
}

// TODO: Make a custom validation error handler to properly serialize errors to JSON

// MutableOrganization is used for creating or updating organization throught the API.
#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct MutableOrganization {
    #[validate(length(max = 32))]
    pub name: String,

    #[validate(length(max = 2048))]
    pub description: String,

    #[validate(custom(function = "slug_validator"))]
    pub slug: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ProjectRepositorySource {
    GitHub,
    Forgejo,
}

impl From<ProjectRepositorySource> for mongodb::bson::Bson {
    fn from(scope: ProjectRepositorySource) -> Self {
        mongodb::bson::to_bson(&scope).expect("Failed to convert to BSON")
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ProjectRepository {
    pub url: String,
    pub source: ProjectRepositorySource,
    pub webhook_secret: Option<String>,
    pub deploy_key: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Project {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        with = "object_id_as_string"
    )]
    #[schema(value_type = Option<String>)]
    pub id: Option<ObjectId>,

    #[schema(value_type = Option<String>)]
    #[serde(with = "object_id_as_string_required")]
    pub organization_id: ObjectId,

    pub name: String,

    pub slug: String,

    pub repository: ProjectRepository,
}

impl Project {
    pub fn to_public(&self) -> PublicProject {
        PublicProject {
            id: self.id,
            organization_id: self.organization_id,
            name: self.name.clone(),
            slug: self.slug.clone(),
            repository: PublicProjectRepository {
                url: self.repository.url.clone(),
                source: self.repository.source.clone(),
                webhook_secret_generated: self.repository.webhook_secret.is_some(),
                deploy_key_generated: self.repository.deploy_key.is_some(),
            },
        }
    }
}

/// ProjectRepository object that can be safely sent to the client
#[derive(Serialize, Deserialize, ToSchema)]
pub struct PublicProjectRepository {
    pub url: String,
    pub source: ProjectRepositorySource,
    pub webhook_secret_generated: bool,
    pub deploy_key_generated: bool,
}

/// Project object that can be safely sent to the client
#[derive(Serialize, Deserialize, ToSchema)]
pub struct PublicProject {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        with = "object_id_as_string"
    )]
    #[schema(value_type = Option<String>)]
    pub id: Option<ObjectId>,

    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    pub organization_id: ObjectId,

    pub name: String,

    pub slug: String,

    pub repository: PublicProjectRepository,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Worker {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        with = "object_id_as_string"
    )]
    #[schema(value_type = Option<String>)]
    pub id: Option<ObjectId>,
    pub token: String,
}

pub async fn fetch_project(
    database: &Database,
    org_id: ObjectId,
    project_slug: String,
) -> Result<Option<Project>> {
    let project = database
        .collection::<Project>("projects")
        .find_one(doc! {
            "organization_id": org_id,
            "slug": project_slug,
        })
        .await?;

    Ok(project)
}
