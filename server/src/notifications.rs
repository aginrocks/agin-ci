mod detailed;
pub mod sender;
mod simple;

pub use detailed::*;
pub use simple::*;

use chrono::{DateTime, Utc};
use color_eyre::eyre::Result;
use derive_builder::Builder;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    database::{Notification, Organization, OrganizationRole, PartialNotification, User},
    mongo_id::object_id_as_string_required,
};

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
    pub user: ObjectId,

    #[builder(default)]
    pub status: NotificationStatus,

    #[builder(default)]
    pub read_at: Option<DateTime<Utc>>,
}

impl Default for PartialNotification<Simple> {
    fn default() -> Self {
        Self {
            created_at: Utc::now(),
            message: "".to_string(),
            title: "".to_string(),
            recipients: vec![],
            body: Simple::Other,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
#[schema(value_type = Vec<Notification<T>>)]
pub struct VecNotification<T: ToSchema + 'static>(Vec<Notification<T>>);

// TODO: Implement more constructors
impl PartialNotification<Simple> {
    pub fn new_role_changed(
        user_id: ObjectId,
        organization: Organization,
        old_role: OrganizationRole,
        new_role: OrganizationRole,
    ) -> Result<Self> {
        let recipient = NotificationRecipientBuilder::default()
            .user(user_id)
            .build()?;

        Ok(Self {
            title: "Your role has been updated".to_string(),
            message: format!(
                "Your role in {} has been changed from {old_role:?} to {new_role:?}.",
                organization.name
            ),
            body: Simple::RoleChanged(SimpleRoleChange {
                user: user_id,
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
            body: Simple::ReceivedInvitation(SimpleInvitationEvent {
                invitation: invitation_id,
            }),
            recipients: vec![recipient],
            ..Default::default()
        })
    }
}
