use serde::Deserialize;
use std::hash::{Hash, Hasher};

// TODO move these models to domain
#[derive(sea_orm::strum::Display)]
pub enum PullRequestEventType {
    Ignored,
    PROpened,
    PRApproved,
    PRMerged,
    PRCommentAdded,
}

pub struct PullRequestEvent {
    pub id: Option<i32>,
    pub hash: i64,
    pub event_type: PullRequestEventType,
    pub title: String,
    pub text: String,
}

pub(super) enum EventType {
    Ignored,
    PROpened,
    PRApproved,
    PRMerged,
    PRCommentAdded,
}

type EpochDateMillis = u64;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ActorPayload {
    pub display_name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PullRequestPayload {
    pub id: u32,
    pub title: String,
    pub open: bool,
    pub updated_date: EpochDateMillis,
    pub from_ref: GitRefPayload,
    pub to_ref: GitRefPayload,
}

impl Hash for PullRequestPayload {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.from_ref.repository.id.hash(state);
        self.from_ref.repository.project.id.hash(state);
        self.to_ref.repository.id.hash(state);
        self.to_ref.repository.project.id.hash(state);
    }
}

#[derive(Deserialize)]
pub(super) struct GitRefPayload {
    pub repository: GitRepositoryPayload,
}

#[derive(Deserialize)]
pub(super) struct GitRepositoryPayload {
    pub id: u32,
    pub project: GitProjectPayload,
}

#[derive(Deserialize)]
pub(super) struct GitProjectPayload {
    pub id: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PullRequestOpenedPayload {
    pub actor: ActorPayload,
    pub pull_request: PullRequestPayload,
}
