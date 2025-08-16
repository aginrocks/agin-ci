use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    database::{Notification, OrganizationRole},
    mongo_id::object_id_as_string_required,
};

pub type SimpleNotification = Notification<Simple>;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
#[serde(tag = "type", content = "body", rename_all = "kebab-case")]
pub enum Simple {
    JobFailed(SimpleJobFail),
    ReceivedInvitation(SimpleInvitationEvent),
    RoleChanged(SimpleRoleChange),
    OfflineWorker(SimpleOfflineWorker),
    Other,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct SimpleJobFail {
    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    pub job: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct SimpleInvitationEvent {
    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    pub invitation: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct SimpleRoleChange {
    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    pub user: ObjectId,

    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    pub organization: ObjectId,

    pub old_role: OrganizationRole,

    pub new_role: OrganizationRole,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct SimpleOfflineWorker {
    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    pub worker: ObjectId,
}
