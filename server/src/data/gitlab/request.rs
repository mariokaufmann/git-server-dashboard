use std::collections::HashSet;

use anyhow::Context;

use crate::data::gitlab::model::{
    BranchDetails, BranchResponse, GitlabPipelineStatus, JobResponse,
    MergeRequestApprovalsResponse, MergeRequestDetails, MergeRequestResponse, PipelineResponse,
    SingleMergeRequestResponse,
};
use crate::data::gitlab::GitlabClient;
use crate::data::model::{
    PipelineStatus, PullRequest, PullRequestTargetBranch, RepositoryBranchData, StandaloneBranch,
};

pub async fn load_repository_data(
    client: &GitlabClient,
    project_id: &str,
) -> anyhow::Result<RepositoryBranchData> {
    let encoded_project_id = encode_id_for_gitlab_url(project_id);

    let merge_request_details = get_merge_requests(client, &encoded_project_id, project_id).await?;
    let branch_details = get_branches(client, &encoded_project_id, project_id).await?;

    let repository_branch_data =
        map_repository_data(project_id, merge_request_details, branch_details)?;

    Ok(repository_branch_data)
}

async fn get_merge_requests(
    client: &GitlabClient,
    encoded_project_id: &str,
    project_id: &str,
) -> anyhow::Result<Vec<MergeRequestDetails>> {
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

        let latest_pipeline_job = match &single_merge_request_response.pipeline {
            Some(pipeline) => Some(
                get_latest_pipeline_job(client, encoded_project_id, project_id, pipeline.id)
                    .await?,
            ),
            None => None,
        };

        let merge_request_detail = MergeRequestDetails {
            details_response: single_merge_request_response,
            approvals_response: merge_request_approvals,
            job_response: latest_pipeline_job,
        };

        merge_request_details.push(merge_request_detail);
    }

    Ok(merge_request_details)
}

async fn get_branches(
    client: &GitlabClient,
    encoded_project_id: &str,
    project_id: &str,
) -> anyhow::Result<Vec<BranchDetails>> {
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
        let pipeline_response = pipelines_response.into_iter().next();
        let job_response = match &pipeline_response {
            Some(pipeline) => Some(
                get_latest_pipeline_job(client, encoded_project_id, project_id, pipeline.id)
                    .await?,
            ),
            None => None,
        };

        let single_branch_details = BranchDetails {
            pipeline_response,
            details_response: branch,
            job_response,
        };
        branch_details.push(single_branch_details);
    }

    Ok(branch_details)
}

async fn get_latest_pipeline_job(
    client: &GitlabClient,
    encoded_project_id: &str,
    project_id: &str,
    pipeline_id: u32,
) -> anyhow::Result<JobResponse> {
    let jobs: Vec<JobResponse> = client
        .request(&format!(
            "{}/pipelines/{}/jobs",
            encoded_project_id, pipeline_id
        ))
        .await
        .with_context(|| {
            format!(
                "Could not load jobs for project {} and pipeline {}.",
                project_id, pipeline_id
            )
        })?;
    let latest_job = jobs.into_iter().next().with_context(|| {
        format!(
            "Did not find any jobs for project {} and pipeline {}.",
            project_id, pipeline_id
        )
    })?;
    Ok(latest_job)
}

fn map_repository_data(
    project_id: &str,
    merge_request: Vec<MergeRequestDetails>,
    branches: Vec<BranchDetails>,
) -> anyhow::Result<RepositoryBranchData> {
    let pull_request_target_branch_names: HashSet<&String> = merge_request
        .iter()
        .map(|pr_details| &pr_details.details_response.target_branch)
        .collect();
    let pull_request_target_branches: Vec<PullRequestTargetBranch> =
        pull_request_target_branch_names
            .iter()
            .map(|name| {
                let pull_requests = merge_request
                    .iter()
                    .filter(|pr| pr.details_response.target_branch.eq(*name))
                    .map(|pr| PullRequest {
                        branch_name: pr.details_response.source_branch.to_owned(),
                        user_name: pr.details_response.author.name.to_owned(),
                        pipeline_status: map_pipeline_status(&pr.details_response.pipeline),
                        pipeline_url: pr.job_response.as_ref().map(|job| job.web_url.to_owned()),
                        comment_count: pr.details_response.user_notes_count,
                        approved: pr.approvals_response.approved,
                        user_profile_image: pr.details_response.author.avatar_url.to_owned(),
                        last_activity_date: pr.details_response.updated_at.to_owned(),
                        link_url: pr.details_response.web_url.to_owned(),
                    })
                    .collect();
                let target_branch_details = branches
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

    let standalone_branches = branches
        .iter()
        .filter(|branch| {
            !merge_request.iter().any(|mr| {
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
            pipeline_url: branch
                .job_response
                .as_ref()
                .map(|job| job.web_url.to_owned()),
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
            GitlabPipelineStatus::Created => PipelineStatus::Queued,
            GitlabPipelineStatus::WaitingForResource => PipelineStatus::Queued,
            GitlabPipelineStatus::Preparing => PipelineStatus::Queued,
            GitlabPipelineStatus::Pending => PipelineStatus::Queued,
            GitlabPipelineStatus::Canceled => PipelineStatus::Canceled,
            GitlabPipelineStatus::Skipped => PipelineStatus::None,
            GitlabPipelineStatus::Manual => PipelineStatus::None,
            GitlabPipelineStatus::Scheduled => PipelineStatus::Queued,
        },
        None => PipelineStatus::None,
    }
}
