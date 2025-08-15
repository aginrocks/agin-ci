use chrono::{DateTime, Utc};
use color_eyre::eyre::Result;
use derive_builder::Builder;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    database::{Organization, OrganizationRole, PartialNotification, User},
    mongo_id::object_id_as_string_required,
};

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
#[serde(tag = "type", content = "body", rename_all = "kebab-case")]
pub enum NotificationBody {
    JobFailed(JobFail),
    ReceivedInvitation(InvitationEvent),
    RoleChanged(RoleChange),
    OfflineWorker(OfflineWorker),
    Other,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct JobFail {
    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    job: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct InvitationEvent {
    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    invitation: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct RoleChange {
    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    user: ObjectId,

    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    organization: ObjectId,

    old_role: OrganizationRole,

    new_role: OrganizationRole,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct OfflineWorker {
    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    worker: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
#[serde(rename_all = "lowercase")]
pub enum NotificationStatus {
    Unread,
    Read,
    Dismissed,
}

impl Default for NotificationStatus {
    fn default() -> Self {
        Self::Unread
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Builder)]
pub struct NotificationRecipient {
    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    user: ObjectId,

    #[builder(default)]
    status: NotificationStatus,

    #[builder(default)]
    read_at: Option<DateTime<Utc>>,
}

impl Default for PartialNotification {
    fn default() -> Self {
        Self {
            created_at: Utc::now(),
            message: "".to_string(),
            title: "".to_string(),
            recipients: vec![],
            body: NotificationBody::Other,
        }
    }
}

// TODO: Implement more constructors
impl PartialNotification {
    pub fn new_role_changed(
        user: User,
        organization: Organization,
        old_role: OrganizationRole,
        new_role: OrganizationRole,
    ) -> Result<Self> {
        let recipient = NotificationRecipientBuilder::default()
            .user(user.id)
            .build()?;

        Ok(Self {
            title: "Your role has been updated".to_string(),
            message: format!("Your role has been changed from {old_role:?} to {new_role:?}."),
            body: NotificationBody::RoleChanged(RoleChange {
                user: user.id,
                organization: organization.id,
                old_role,
                new_role,
            }),
            recipients: vec![recipient],
            ..Default::default()
        })
    }

    pub fn new_received_invitation(
        inviter: User,
        invitee: User,
        organization: Organization,
        invitation_id: ObjectId,
        role: OrganizationRole,
    ) -> Result<Self> {
        let recipient = NotificationRecipientBuilder::default()
            .user(invitee.id)
            .build()?;

        Ok(Self {
            title: format!("You've been invited to join {}", organization.name),
            message: format!(
                "{} invited you to join {} as {role:?}",
                inviter.name, organization.name,
            ),
            body: NotificationBody::ReceivedInvitation(InvitationEvent {
                invitation: invitation_id,
            }),
            recipients: vec![recipient],
            ..Default::default()
        })
    }
}
