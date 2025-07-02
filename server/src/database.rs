use color_eyre::eyre::Result;
use mongodb::{Client, Database, bson::oid::ObjectId};
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

    // let client = SessionClient::with_uri_str(&settings.db.connection_string).await?;
    // let session_store = MongoDBStore::new(client, settings.db.database_name.clone());

    // let session_store = MemoryStore::default();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(120)));

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

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct Membership {
    #[schema(value_type = String)]
    #[serde(with = "object_id_as_string_required")]
    pub user_id: ObjectId,
    pub role: OrganizationRole,
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

// MutableOrganization is used for creating or updating an organization throught the API.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct MutableOrganization {
    pub name: String,
    pub description: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ProjectRepositorySource {
    GitHub,
    Forgejo,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ProjectRepository {
    pub url: String,
    pub source: ProjectRepositorySource,
    pub webhook_secret: String,
    pub deploy_key: String,
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
    pub organization_id: ObjectId,
    pub name: String,
    pub slug: String,
    pub repository: ProjectRepository,
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
