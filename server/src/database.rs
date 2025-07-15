use crate::validators::slug_validator;
use color_eyre::eyre::Result;
use mongodb::{
    Client, Database,
    bson::{doc, oid::ObjectId},
};
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use ssh_key::{Algorithm, PrivateKey, rand_core::OsRng, sec1::der::zeroize::Zeroizing};
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
use visible::StructFields;

use crate::mongo_id::{object_id_as_string, object_id_as_string_required};
use crate::settings::Settings;

macro_rules! database_object {
    ($name:ident { $($field:tt)* }$(, $($omitfield:ident),*)?) => {
        #[derive(Partial, Debug, Serialize, Deserialize, ToSchema, Clone)]
        #[partial(omit(id $(, $($omitfield),* )?), derive(Debug, Serialize, Deserialize, ToSchema, Clone))]
        #[StructFields(pub)]
        pub struct $name {
            $($field)*
        }
    };
}

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

database_object!(User {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,
    subject: String,
    name: String,
    email: String,
});

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum OrganizationRole {
    Viewer = 0,
    Member = 1,
    Admin = 2,
    Owner = 3,
}

impl From<OrganizationRole> for mongodb::bson::Bson {
    fn from(scope: OrganizationRole) -> Self {
        mongodb::bson::to_bson(&scope).expect("Failed to convert to BSON")
    }
}

database_object!(Membership {
    #[schema(value_type = String)]
    #[serde(with = "object_id_as_string_required")]
    user_id: ObjectId,
    role: OrganizationRole,
});

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

database_object!(Secret {
    #[serde(
        rename = "_id",
        with = "object_id_as_string_required"
    )]
    #[schema(value_type = String)]
    id: ObjectId,

    name: String,

    scope: SecretScope,

    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    organization_id: ObjectId,

    #[serde(with = "object_id_as_string")]
    #[schema(value_type = Option<String>)]
    project_id: Option<ObjectId>,

    secret: String,
});

// Secret object that can be safely sent to the client
database_object!(PublicSecret {
    #[serde(
        rename = "_id",
        with = "object_id_as_string_required"
    )]
    #[schema(value_type = String)]
    id: ObjectId,

    name: String,

    scope: SecretScope,

    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    organization_id: ObjectId,

    #[serde(with = "object_id_as_string")]
    #[schema(value_type = Option<String>)]
    project_id: Option<ObjectId>,
});

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

database_object!(Organization {
    #[serde(
        rename = "_id",
        with = "object_id_as_string_required"
    )]
    #[schema(value_type = Option<String>)]
    id: ObjectId,
    name: String,
    description: String,
    slug: String,
    members: Vec<Membership>,
    avatar_email: Option<String>,
});

// TODO: Make a custom validation error handler to properly serialize errors to JSON

/// MutableOrganization is used for creating or updating organization throught the API.
#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct MutableOrganization {
    #[validate(length(min = 1, max = 32))]
    pub name: String,

    #[validate(length(max = 2048))]
    pub description: String,

    #[validate(custom(function = "slug_validator"), length(min = 1, max = 32))]
    pub slug: String,

    #[validate(email, length(max = 64))]
    pub avatar_email: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ProjectRepositorySource {
    GitHub,
    Gitea,
    GenericGit,
}

impl From<ProjectRepositorySource> for mongodb::bson::Bson {
    fn from(scope: ProjectRepositorySource) -> Self {
        mongodb::bson::to_bson(&scope).expect("Failed to convert to BSON")
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct ProjectRepository {
    pub url: String,
    pub source: ProjectRepositorySource,
    pub webhook_secret: Option<String>,
    pub deploy_private_key: Option<String>,
    pub deploy_public_key: Option<String>,
}

// TODO: Encryption of the private key
impl ProjectRepository {
    pub fn generate_deploy_keys(&self) -> Result<(String, Zeroizing<String>)> {
        let mut rng = OsRng;
        let private_key = PrivateKey::random(&mut rng, Algorithm::Ed25519)?;
        let public_key = private_key.public_key();

        let public_key_openssh = public_key.to_openssh()?;
        let private_key_openssh = private_key.to_openssh(ssh_key::LineEnding::LF)?;

        Ok((public_key_openssh, private_key_openssh))
    }
}

database_object!(Project {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,

    #[schema(value_type = Option<String>)]
    #[serde(with = "object_id_as_string_required")]
    organization_id: ObjectId,

    name: String,

    slug: String,

    repository: ProjectRepository,
});

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
                deploy_key_generated: self.repository.deploy_private_key.is_some(),
                deploy_public_key: self.repository.deploy_public_key.clone(),
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
    pub deploy_public_key: Option<String>,
}

/// Project object that can be safely sent to the client
#[derive(Serialize, Deserialize, ToSchema)]
pub struct PublicProject {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    pub id: ObjectId,

    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    pub organization_id: ObjectId,

    pub name: String,

    pub slug: String,

    pub repository: PublicProjectRepository,
}

database_object!(Worker {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,
    hashed_token: String,
});

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Permission {
    Read,
    Write,
    Admin,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", content = "slug", rename_all = "lowercase")]
pub enum ScopeType {
    /// Allows access to the user account
    User,
    /// Allows access to all organizations and projects
    Global,
    /// Allows access to an organization
    Org(String),
    // TODO: Implement project access
    // /// Allows access to a project
    // Project(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Scope {
    pub permission: Permission,
    pub scope: ScopeType,
}

database_object!(AccessToken {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,

    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    user_id: ObjectId,

    hashed_token: String,

    display_name: String,

    scopes: Vec<Scope>,
});

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct AccessTokenCreateBody {
    #[validate(length(min = 1, max = 64))]
    pub display_name: String,

    pub scopes: Vec<Scope>,
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
