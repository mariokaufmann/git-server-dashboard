use std::hash::{Hash, Hasher};

use serde::Deserialize;

pub(super) enum PREventType {
    Opened,
    Approved,
    Merged,
    CommentAdded,
}

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
    // TODO use this (implement cleanup)
    #[allow(unused)]
    pub open: bool,
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
    pub actor: ActorPayload,
    pub pull_request: PullRequestPayload,
}

#[derive(Deserialize)]
pub(super) struct PullRequestCommentEventPayload {
    pub comment: PullRequestCommentPayload,
}

#[derive(Deserialize)]
pub(super) struct PullRequestCommentPayload {
    pub text: String,
}
