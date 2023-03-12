use std::fmt::{Display, Formatter};

use anyhow::anyhow;
use serde_derive::Serialize;

#[derive(Serialize, Clone)]
pub struct RepositoriesData {
    pub last_updated_date: Option<String>,
    pub currently_refreshing: bool,
    pub repositories: Vec<RepositoryBranchData>,
}

#[derive(Serialize, Clone)]
pub struct RepositoryBranchData {
    pub repository_name: String,
    pub repository_url: String,
    pub pull_request_target_branches: Vec<PullRequestTargetBranch>,
    pub standalone_branches: Vec<StandaloneBranch>,
}

#[derive(Serialize, Clone)]
pub struct PullRequestTargetBranch {
    pub branch_name: String,
    pub pipeline_url: Option<String>,
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

#[derive(Clone)]
pub struct Repository {
    pub name: String,
    pub group: String,
}

impl Repository {
    pub fn from_slug(slug: &str) -> anyhow::Result<Self> {
        let parts: Vec<&str> = slug.split('/').collect();

        let group = parts
            .first()
            .ok_or_else(|| anyhow!("Could not parse group name from {}.", slug))?;
        let name = parts
            .last()
            .ok_or_else(|| anyhow!("Could not parse repository name from {}.", slug))?;

        Ok(Self {
            name: name.to_string(),
            group: group.to_string(),
        })
    }
}

impl Display for Repository {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.group, self.name)
    }
}
