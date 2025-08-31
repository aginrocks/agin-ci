// This file contains simplified events from webhooks.
// Only fields used by Agin CI for filtering workflows are included.

#[derive(Clone)]
pub enum WebhookEvent {
    /// A push event. If user pushed to multiple branches, multiple events should be created.
    Push(Push),
}

#[derive(Clone)]
pub struct Push {
    pub branch: String,
    pub head_commit_id: String,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}
