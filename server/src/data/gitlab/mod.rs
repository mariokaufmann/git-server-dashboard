use std::collections::HashSet;

use anyhow::{anyhow, Context};
use axum::http;
use reqwest::Method;

use crate::data::gitlab::model::{
    BranchDetails, BranchResponse, GitlabPipelineStatus, MergeRequestApprovalsResponse,
    MergeRequestDetails, MergeRequestResponse, PipelineResponse, SingleMergeRequestResponse,
};
use crate::data::model::{
    DashboardData, PipelineStatus, PullRequest, PullRequestTargetBranch, RepositoryBranchData,
    StandaloneBranch,
};

mod model;

pub struct GitlabClient {
    client: reqwest::Client,
    gitlab_url: String,
    gitlab_token: String,
}

impl GitlabClient {
    pub fn new(gitlab_url: String, gitlab_token: String) -> GitlabClient {
        GitlabClient {
            client: reqwest::Client::new(),
            gitlab_url,
            gitlab_token,
        }
    }

    pub async fn request<T>(&self, url: &str) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let full_url = format!("{}/api/v4/projects/{}", self.gitlab_url, url);
        let response = self
            .client
            .request(Method::GET, full_url)
            .header(
                http::header::AUTHORIZATION,
                format!("Bearer {}", self.gitlab_token),
            )
            .send()
            .await
            .context("Could not make request to Gitlab.")?;
        if !response.status().is_success() {
            return Err(anyhow!(
                "Unsuccessful response from gitlab for url {}: {}",
                url,
                response.status()
            ));
        };

        let parsed_body: T = response
            .json()
            .await
            .context("Could not parse response body from JSON.")?;
        Ok(parsed_body)
    }
}

pub async fn load_dashboard_data(
    client: &GitlabClient,
    projects: &[String],
) -> anyhow::Result<DashboardData> {
    let mut repositories = Vec::new();
    for project in projects {
        let repository_data = load_repository_data(client, project)
            .await
            .with_context(|| format!("Could not load data for repository {}.", project))?;
        repositories.push(repository_data);
    }
    Ok(DashboardData { repositories })
}

pub async fn load_repository_data(
    client: &GitlabClient,
    project_id: &str,
) -> anyhow::Result<RepositoryBranchData> {
    // get list of all open merge requests for project -https://gitlab.com/api/v4/projects/w1102%2Fwackertv-test/merge_requests?state=opened
    let encoded_project_id = encode_id_for_gitlab_url(project_id);
    let merqe_requests: Vec<MergeRequestResponse> = client
        .request(&format!(
            "{}/merge_requests?state=opened",
            encoded_project_id
        ))
        .await
        .with_context(|| {
            format!(
                "Could not load open merge requests for project: {}",
                project_id
            )
        })?;

    // merge request details
    let mut merge_request_details = Vec::new();
    for merge_request in merqe_requests {
        let single_merge_request_response: SingleMergeRequestResponse = client
            .request(&format!(
                "{}/merge_requests/{}",
                encoded_project_id, merge_request.iid
            ))
            .await
            .with_context(|| {
                format!(
                    "Could not load merge request details for project {} and MR {}.",
                    project_id, merge_request.iid,
                )
            })?;
        let merge_request_approvals: MergeRequestApprovalsResponse = client
            .request(&format!(
                "{}/merge_requests/{}/approvals",
                encoded_project_id, merge_request.iid
            ))
            .await
            .with_context(|| {
                format!(
                    "Could not load merge request approvals for project {} and MR {}.",
                    project_id, merge_request.iid,
                )
            })?;

        let merge_request_detail = MergeRequestDetails {
            details_response: single_merge_request_response,
            approvals_response: merge_request_approvals,
        };

        merge_request_details.push(merge_request_detail);
    }

    let branches: Vec<BranchResponse> = client
        .request(&format!("{}/repository/branches", encoded_project_id))
        .await
        .with_context(|| format!("Could not load branches for project: {}", project_id))?;

    let mut branch_details = Vec::new();
    for branch in branches.into_iter() {
        let encoded_branch = encode_id_for_gitlab_url(&branch.name);
        let pipelines_response: Vec<PipelineResponse> = client
            .request(&format!(
                "{}/pipelines?ref={}&per_page=1",
                encoded_project_id, encoded_branch
            ))
            .await
            .with_context(|| {
                format!(
                    "Could not load pipeline details for project {} and branch {}.",
                    project_id, branch.name,
                )
            })?;
        let single_branch_details = BranchDetails {
            pipeline_response: pipelines_response.into_iter().next(),
            details_response: branch,
        };
        branch_details.push(single_branch_details);
    }

    let pull_request_target_branch_names: HashSet<&String> = merge_request_details
        .iter()
        .map(|pr_details| &pr_details.details_response.target_branch)
        .collect();
    let pull_request_target_branches: Vec<PullRequestTargetBranch> =
        pull_request_target_branch_names
            .iter()
            .map(|name| {
                let pull_requests = merge_request_details
                    .iter()
                    .filter(|pr| pr.details_response.target_branch.eq(*name))
                    .map(|pr| PullRequest {
                        branch_name: pr.details_response.source_branch.to_owned(),
                        pipeline_status: map_pipeline_status(&pr.details_response.pipeline),
                        comment_count: pr.details_response.user_notes_count,
                        approved: pr.approvals_response.approved,
                        user_profile_image: pr.details_response.author.avatar_url.to_owned(),
                        last_activity_date: pr.details_response.updated_at.to_owned(),
                    })
                    .collect();
                let target_branch_details = branch_details
                    .iter()
                    .find(|branch| branch.details_response.name.eq(*name))
                    .with_context(|| {
                        format!("Could not find branch details for branch {}.", name)
                    })?;
                Ok(PullRequestTargetBranch {
                    branch_name: name.to_string(),
                    pipeline_status: map_pipeline_status(&target_branch_details.pipeline_response),
                    pull_requests,
                })
            })
            .collect::<anyhow::Result<Vec<PullRequestTargetBranch>>>()
            .context("Could not gather pull request details.")?;

    let standalone_branches = branch_details
        .iter()
        .filter(|branch| {
            !merge_request_details.iter().any(|mr| {
                mr.details_response
                    .source_branch
                    .eq(&branch.details_response.name)
                    || mr
                        .details_response
                        .target_branch
                        .eq(&branch.details_response.name)
            })
        })
        .map(|branch| StandaloneBranch {
            branch_name: branch.details_response.name.to_string(),
            pipeline_status: map_pipeline_status(&branch.pipeline_response),
        })
        .collect();

    Ok(RepositoryBranchData {
        repository_name: project_id.to_string(),
        pull_request_target_branches,
        standalone_branches,
    })
}

/// Encodes the given id for a Gitlab API request -> e.g. project/abcd becomes project%2Fabcd
fn encode_id_for_gitlab_url(id: &str) -> String {
    id.replace('/', "%2F")
}

fn map_pipeline_status(response: &Option<PipelineResponse>) -> PipelineStatus {
    match response {
        Some(response) => match response.status {
            GitlabPipelineStatus::Running => PipelineStatus::Running,
            GitlabPipelineStatus::Success => PipelineStatus::Successful,
            GitlabPipelineStatus::Failed => PipelineStatus::Failed,
        },
        None => PipelineStatus::None,
    }
}
