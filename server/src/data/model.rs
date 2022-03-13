use serde_derive::Serialize;

#[derive(Serialize)]
pub struct DashboardData {
    pub repositories: Vec<RepositoryBranchData>,
}

#[derive(Serialize)]
pub struct RepositoryBranchData {
    pub repository_name: String,
    pub pull_request_target_branches: Vec<PullRequestTargetBranch>,
    pub standalone_branches: Vec<StandaloneBranch>,
}

#[derive(Serialize)]
pub struct PullRequestTargetBranch {
    pub branch_name: String,
    pub pipeline_status: PipelineStatus,
    pub pull_requests: Vec<PullRequest>,
}

#[derive(Serialize)]
pub struct PullRequest {
    pub branch_name: String,
    pub user_profile_image: String,
    pub comment_count: u16,
    pub last_activity_date: String,
    pub approved: bool,
    pub pipeline_status: PipelineStatus,
}

#[derive(Serialize)]
pub struct StandaloneBranch {
    pub branch_name: String,
    pub pipeline_status: PipelineStatus,
}

#[derive(Serialize)]
pub enum PipelineStatus {
    Running,
    Successful,
    Failed,
    None,
}
