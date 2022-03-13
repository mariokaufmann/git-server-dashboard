use serde_derive::Deserialize;

pub struct BranchDetails {
    pub details_response: BranchResponse,
    pub pipeline_response: Option<PipelineResponse>,
}

pub struct MergeRequestDetails {
    pub details_response: SingleMergeRequestResponse,
    pub approvals_response: MergeRequestApprovalsResponse,
}

#[derive(Deserialize)]
pub struct BranchResponse {
    pub name: String,
}

#[derive(Deserialize)]
pub struct MergeRequestResponse {
    pub iid: u16,
}

#[derive(Deserialize)]
pub struct SingleMergeRequestResponse {
    pub source_branch: String,
    pub target_branch: String,
    pub author: GitlabUserResponse,
    pub updated_at: String,
    pub user_notes_count: u16,
    pub has_conflicts: bool,
    pub blocking_discussions_resolved: bool,
    pub pipeline: Option<PipelineResponse>,
}

#[derive(Deserialize)]
pub struct PipelineResponse {
    pub status: GitlabPipelineStatus,
}

#[derive(Deserialize)]
pub struct MergeRequestApprovalsResponse {
    pub approved: bool,
}

#[derive(Deserialize)]
pub struct GitlabUserResponse {
    pub avatar_url: String,
}

#[derive(Deserialize)]
pub enum GitlabPipelineStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "failed")]
    Failed,
}
