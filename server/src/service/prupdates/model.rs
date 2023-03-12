use chrono::{DateTime, Utc};
use serde_derive::Serialize;

pub type PullRequestId = i64;
pub type PullRequestEventTimestamp = DateTime<Utc>;

#[derive(Serialize, sea_orm::strum::Display, sea_orm::strum::EnumString)]
pub enum PullRequestEventType {
    Ignored,
    PROpened,
    PRApproved,
    PRMerged,
    PRCommentAdded,
}

#[derive(Serialize)]
pub struct PullRequestEvent {
    pub id: Option<i32>,
    pub pr_id: PullRequestId,
    pub event_type: PullRequestEventType,
    pub author: String,
    pub title: String,
    pub repository: String,
    pub text: String,
    pub timestamp: PullRequestEventTimestamp,
}
