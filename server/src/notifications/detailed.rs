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
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
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

macro_rules! create_pipeline {
    (collection = $collection:literal, field = $field:literal, $($fields:literal),*) => {
        vec![
            doc! {
                "$lookup": {
                    "from": $collection,
                    "localField": format!("body.{}", $field),
                    "foreignField": "_id",
                    "as": format!("body.{}", $field),
                }
            },
            doc! {
                "$unwind": {
                    "path": format!("$body.{}", $field),
                    "preserveNullAndEmptyArrays": true,
                }
            },
            doc! {
                "$addFields": {
                    (format!("body.{}", $field)): {
                        "$cond": {
                            "if": { "$ne": [format!("$body.{}", $field), null] },
                            "then": {
                                $(
                                    $fields: format!("$body.{}.{}", $field, $fields)
                                ),*
                            },
                            "else": null
                        }
                    }
                }
            }
        ]
    };
}

pub fn query_detailed_notifications() -> Vec<Document> {
    [create_pipeline!(
        collection = "organizations",
        field = "organization",
        "_id",
        "name",
        "slug",
        "avatar_email"
    )]
    .concat()
}
