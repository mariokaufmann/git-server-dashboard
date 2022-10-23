use std::collections::HashSet;

use anyhow::Context;

use crate::data::gitlab::model::{
    BranchDetails, BranchResponse, GitlabPipelineStatus, JobResponse,
    MergeRequestApprovalsResponse, MergeRequestDetails, MergeRequestResponse, PipelineResponse,
    ProjectDetails, ProjectResponse, SingleMergeRequestResponse,
};
use crate::data::gitlab::GitlabClient;
use crate::data::model::{
    PipelineStatus, PullRequest, PullRequestTargetBranch, RepositoryBranchData, StandaloneBranch,
};

pub async fn load_repository_data(
    client: &GitlabClient,
    project_name: &str,
) -> anyhow::Result<RepositoryBranchData> {
    let project_response = get_project(client, project_name).await?;
    let project = ProjectDetails {
        id: project_response.id,
        name: project_name.to_owned(),
        url: project_response.web_url,
    };
    let merge_request_details = get_merge_requests(client, &project).await?;
    let branch_details = get_branches(client, &project).await?;

    let repository_branch_data =
        map_repository_data(&project, merge_request_details, branch_details)?;

    Ok(repository_branch_data)
}

async fn get_project(client: &GitlabClient, project_name: &str) -> anyhow::Result<ProjectResponse> {
    let encoded_project_id = encode_id_for_gitlab_url(project_name);
    client.request(&encoded_project_id).await.with_context(|| {
        format!(
            "Could not load project details for project {}.",
            project_name
        )
    })
}

async fn get_merge_requests(
    client: &GitlabClient,
    project: &ProjectDetails,
) -> anyhow::Result<Vec<MergeRequestDetails>> {
    let merqe_requests: Vec<MergeRequestResponse> = client
        .request(&format!("{}/merge_requests?state=opened", project.id))
        .await
        .with_context(|| {
            format!(
                "Could not load open merge requests for project: {}",
                project.name
            )
        })?;

    // merge request details
    let mut merge_request_details = Vec::new();
    for merge_request in merqe_requests {
        let single_merge_request_response: SingleMergeRequestResponse = client
            .request(&format!(
                "{}/merge_requests/{}",
                project.id, merge_request.iid
            ))
            .await
            .with_context(|| {
                format!(
                    "Could not load merge request details for project {} and MR {}.",
                    project.name, merge_request.iid,
                )
            })?;
        let merge_request_approvals: MergeRequestApprovalsResponse = client
            .request(&format!(
                "{}/merge_requests/{}/approvals",
                project.id, merge_request.iid
            ))
            .await
            .with_context(|| {
                format!(
                    "Could not load merge request approvals for project {} and MR {}.",
                    project.name, merge_request.iid,
                )
            })?;

        let latest_pipeline_job = match &single_merge_request_response.pipeline {
            Some(pipeline) => get_latest_pipeline_job(client, project, pipeline.id).await?,
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
    project: &ProjectDetails,
) -> anyhow::Result<Vec<BranchDetails>> {
    let branches: Vec<BranchResponse> = client
        .request(&format!("{}/repository/branches", project.id))
        .await
        .with_context(|| format!("Could not load branches for project: {}", project.name))?;

    let mut branch_details = Vec::new();
    for branch in branches.into_iter() {
        let encoded_branch = encode_id_for_gitlab_url(&branch.name);
        let pipelines_response: Vec<PipelineResponse> = client
            .request(&format!(
                "{}/pipelines?ref={}&per_page=1",
                project.id, encoded_branch
            ))
            .await
            .with_context(|| {
                format!(
                    "Could not load pipeline details for project {} and branch {}.",
                    project.name, branch.name,
                )
            })?;
        let pipeline_response = pipelines_response.into_iter().next();
        let job_response = match &pipeline_response {
            Some(pipeline) => get_latest_pipeline_job(client, project, pipeline.id).await?,
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
    project: &ProjectDetails,
    pipeline_id: u32,
) -> anyhow::Result<Option<JobResponse>> {
    let jobs: Vec<JobResponse> = client
        .request(&format!(
            "{}/pipelines/{}/jobs?per_page=1",
            project.id, pipeline_id
        ))
        .await
        .with_context(|| {
            format!(
                "Could not load jobs for project {} and pipeline {}.",
                project.name, pipeline_id
            )
        })?;
    let latest_job = jobs.into_iter().next();
    Ok(latest_job)
}

fn map_repository_data(
    project: &ProjectDetails,
    merge_requests: Vec<MergeRequestDetails>,
    branches: Vec<BranchDetails>,
) -> anyhow::Result<RepositoryBranchData> {
    let pull_request_target_branch_names: HashSet<&String> = merge_requests
        .iter()
        .map(|pr_details| &pr_details.details_response.target_branch)
        .collect();
    let pull_request_target_branches: Vec<PullRequestTargetBranch> =
        pull_request_target_branch_names
            .iter()
            .map(|name| {
                let pull_requests = merge_requests
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
                    pipeline_url: target_branch_details
                        .job_response
                        .as_ref()
                        .map(|job| job.web_url.to_owned()),
                    pull_requests,
                })
            })
            .collect::<anyhow::Result<Vec<PullRequestTargetBranch>>>()
            .context("Could not gather pull request details.")?;

    let standalone_branches = branches
        .iter()
        .filter(|branch| {
            !merge_requests.iter().any(|mr| {
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
        repository_name: project.name.to_string(),
        repository_url: project.url.to_string(),
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
