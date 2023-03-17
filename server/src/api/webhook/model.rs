use std::hash::{Hash, Hasher};

use serde::Deserialize;

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
    // TODO use this
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
    pub name: String,
    pub project: GitProjectPayload,
}

#[derive(Deserialize)]
pub(super) struct GitProjectPayload {
    pub id: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CommonPullRequestEventPayload {
    pub date: String,
    pub actor: ActorPayload,
    pub pull_request: PullRequestPayload,
}
