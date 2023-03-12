use serde_derive::Serialize;

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
    pub hash: i64,
    pub event_type: PullRequestEventType,
    pub author: String,
    pub title: String,
    pub repository: String,
    pub text: String,
    pub date: String,
}
