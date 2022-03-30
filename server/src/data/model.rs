use serde_derive::Serialize;

#[derive(Serialize, Clone)]
pub struct DashboardData {
    pub last_updated_date: Option<String>,
    pub repositories: Vec<RepositoryBranchData>,
}

#[derive(Serialize, Clone)]
pub struct RepositoryBranchData {
    pub repository_name: String,
    pub pull_request_target_branches: Vec<PullRequestTargetBranch>,
    pub standalone_branches: Vec<StandaloneBranch>,
}

#[derive(Serialize, Clone)]
pub struct PullRequestTargetBranch {
    pub branch_name: String,
    pub pipeline_status: PipelineStatus,
    pub pull_requests: Vec<PullRequest>,
}

#[derive(Serialize, Clone)]
pub struct PullRequest {
    pub branch_name: String,
    pub user_name: String,
    pub user_profile_image: String,
    pub comment_count: u32,
    pub last_activity_date: String,
    pub approved: bool,
    pub pipeline_status: PipelineStatus,
    pub pipeline_url: Option<String>,
    pub link_url: String,
}

#[derive(Serialize, Clone)]
pub struct StandaloneBranch {
    pub branch_name: String,
    pub pipeline_status: PipelineStatus,
    pub pipeline_url: Option<String>,
}

#[derive(Serialize, Clone)]
pub enum PipelineStatus {
    Running,
    Successful,
    Failed,
    Queued,
    Canceled,
    None,
}
