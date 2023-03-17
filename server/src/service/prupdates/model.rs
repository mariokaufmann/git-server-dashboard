use chrono::{DateTime, Utc};
use serde_derive::Serialize;

pub type PullRequestTimestamp = DateTime<Utc>;

#[derive(
    Clone, Copy, Eq, Hash, PartialEq, Serialize, sea_orm::strum::Display, sea_orm::strum::EnumString,
)]
pub enum PullRequestEventType {
    Opened,
    Approved,
    Merged,
    CommentAdded,
}

#[derive(Serialize)]
pub struct PullRequestEvent {
    pub id: Option<i32>,
    pub pr_id: String,
    pub event_type: PullRequestEventType,
    pub author: String,
    pub title: String,
    pub repository: String,
    pub text: String,
    pub timestamp: PullRequestTimestamp,
}

#[derive(Serialize, sea_orm::strum::Display, sea_orm::strum::EnumString)]
pub enum PullRequestUpdateType {
    Aggregated,
    Opened,
    Approved,
    Merged,
    CommentAdded,
}

#[derive(Serialize)]
pub struct PullRequestUpdate {
    pub pr_id: String,
    pub update_type: PullRequestUpdateType,
    pub author: String,
    pub title: String,
    pub repository: String,
    pub details: Vec<String>,
    pub timestamp: PullRequestTimestamp,
}
