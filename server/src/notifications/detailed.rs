use mongo_utils::{JoinPipeline, JoinPipelineBuilder};
use mongodb::bson::{Document, doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    database::{Invitation, Notification, OrganizationRole},
    mongo_id::object_id_as_string_required,
};

pub type DetailedNotification = Notification<Detailed>;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
#[serde(tag = "type", content = "body", rename_all = "kebab-case")]
pub enum Detailed {
    JobFailed(DetailedJobFail),
    ReceivedInvitation(DetailedInvitationEvent),
    RoleChanged(DetailedRoleChange),
    OfflineWorker(DetailedOfflineWorker),
    Other,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct DetailedJobFail {
    pub job: DetailedJob,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct DetailedJob {
    // TODO
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct DetailedInvitationEvent {
    pub invitation: Invitation,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct DetailedRoleChange {
    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    pub user: ObjectId,

    pub organization: SimpleOrganization,

    pub old_role: OrganizationRole,

    pub new_role: OrganizationRole,
}

/// Organization info used in joins
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, JoinPipeline)]
#[mongo_utils(collection = "organizations")]
pub struct SimpleOrganization {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,
    name: String,
    slug: String,
    avatar_email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct DetailedOfflineWorker {
    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    pub worker: ObjectId,
}

pub fn query_detailed_notifications() -> Vec<Document> {
    [SimpleOrganization::join_pipeline(
        "body.organization",
        "_id",
    )]
    .concat()
}
