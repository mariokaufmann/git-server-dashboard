use crate::data::bitbucket::model::{
    BranchDetails, BranchResponse, BuildStatusResponse, PaginatedResponse,
    PullRequestCommentsResponse, PullRequestDetails, PullRequestResponse, RepositoryResponse,
    UserResponse,
};
use crate::data::bitbucket::BitbucketClient;
use crate::data::gitlab::GitlabClient;
use crate::data::model::{DashboardData, RepositoryBranchData};
use anyhow::{anyhow, Context};
use chrono::format::format;
use chrono::Utc;
use log::info;
use std::collections::{HashMap, HashSet};

fn get_repo_sub_url(project_name: &str, repository_name: &str, suffix: &str) -> String {
    format!(
        "api/1.0/projects/{}/repos/{}/{}",
        project_name, repository_name, suffix
    )
}

fn get_build_status_url(commit_id: &str) -> String {
    format!("build-status/1.0/commits/{}", commit_id)
}

fn get_user_url(user_slug: &str) -> String {
    format!("api/1.0/users/{}?avatarSize=32", user_slug)
}

pub async fn load_dashboard_data(
    client: &BitbucketClient,
    projects: &[(String, String)],
) -> anyhow::Result<DashboardData> {
    let mut user_map: HashMap<&str, UserResponse> = HashMap::new();
    // Option<BuildStatusResponse> because not every commit has a corresponding build
    let mut build_status_map: HashMap<&str, Option<BuildStatusResponse>> = HashMap::new();

    let mut repositories: Vec<RepositoryBranchData> = Vec::new();

    for project in projects {
        let repository = get_repository(client, project).await?;
        let branches = get_branches(client, project).await?;

        for branch in branches {
            let commit_id = &branch.latest_commit;
            if build_status_map.get(commit_id).is_none() {
                let build_status = get_build_status(client, commit_id).await?;
                build_status_map.insert(commit_id, build_status);
            }
        }

        let pull_requests = get_pull_requests(client, project).await?;

        for pull_request in pull_requests {
            let commit_id = &pull_request.details_response.from_ref.latest_commit;
            if build_status_map.get(commit_id).is_none() {
                let build_status = get_build_status(client, commit_id).await?;
                build_status_map.insert(commit_id, build_status);
            }

            let user_slug = &pull_request.details_response.author.user.slug;
            if user_map.get(user_slug).is_none() {
                let user = get_user(client, user_slug).await?;
                user_map.insert(user_slug, user);
            }
        }

        let repository_branch_data = map_repository_data(
            repository,
            branches,
            pull_requests,
            &user_map,
            &build_status_map,
        );
        repositories.push(repository_branch_data);
    }

    // TODO remove duplication (e.g. just return array of repos)
    let last_updated_date = Utc::now();
    let formatted_last_updated_date = format!("{}", last_updated_date.format("%+"));
    Ok(DashboardData {
        last_updated_date: Some(formatted_last_updated_date),
        repositories,
    })
}

async fn get_repository(
    client: &BitbucketClient,
    project: &(String, String),
) -> anyhow::Result<RepositoryResponse> {
    let url = get_repo_sub_url(&project.0, &project.1, "");
    let response = client.request(&url).await.with_context(|| {
        format!(
            "Could not load repository details for project: {}/{}",
            project.0, project.1
        )
    })?;
    Ok(response)
}

async fn get_branches(
    client: &BitbucketClient,
    project: &(String, String),
) -> anyhow::Result<Vec<BranchResponse>> {
    // TODO switch away from tuple
    let url = get_repo_sub_url(&project.0, &project.1, "branches");
    let response: PaginatedResponse<BranchResponse> =
        client.request(&url).await.with_context(|| {
            format!(
                "Could not load branches for project: {}/{}",
                project.0, project.1
            )
        })?;

    Ok(response.values)
}

// TODO load build statuses after loading branches and PRs only once
// TODO load users after loading branches and PRs only once

async fn get_pull_requests(
    client: &BitbucketClient,
    project: &(String, String),
) -> anyhow::Result<Vec<PullRequestDetails>> {
    // TODO switch away from tuple
    let pull_request_url = get_repo_sub_url(&project.0, &project.1, "pull-requests");
    let pull_request_response: PaginatedResponse<PullRequestResponse> =
        client.request(&pull_request_url).await.with_context(|| {
            format!(
                "Could not load pull requests for project: {}/{}",
                project.0, project.1
            )
        })?;

    let mut pull_request_details = Vec::new();
    for pull_request in pull_request_response.values.into_iter() {
        let comments_url = get_repo_sub_url(
            &project.0,
            &project.1,
            &format!("pull-requests/{}/blocker-comments", pull_request.id),
        );
        let comments_response: PaginatedResponse<PullRequestCommentsResponse> =
            client.request(&comments_url).await.with_context(|| {
                format!(
                    "Could not load comments for project: {}/{}",
                    project.0, project.1
                )
            })?;
        let pull_request_detail = PullRequestDetails {
            details_response: pull_request,
            comments_count: comments_response.values.len() as u32,
        };
        pull_request_details.push(pull_request_detail);
    }

    Ok(pull_request_details)
}

async fn get_user(client: &BitbucketClient, user_slug: &str) -> anyhow::Result<UserResponse> {
    let url = get_user_url(user_slug);
    let response = client
        .request(&url)
        .await
        .with_context(|| format!("Could not load user with slug: {}", user_slug))?;
    Ok(response)
}

async fn get_build_status(
    client: &BitbucketClient,
    commit_id: &str,
) -> anyhow::Result<Option<BuildStatusResponse>> {
    let url = get_build_status_url(commit_id);
    let response: PaginatedResponse<BuildStatusResponse> =
        client.request(&url).await.with_context(|| {
            format!(
                "Could not load build status for commit {}.",
                &branch.latest_commit
            )
        })?;
    let build_status = response.values.into_iter().last();
    Ok(build_status)
}

fn map_repository_data(
    mut repository: RepositoryResponse,
    branches: Vec<BranchResponse>,
    pull_requests: Vec<PullRequestDetails>,
    users: &HashMap<&str, UserResponse>,
    build_statuses: &HashMap<&str, Option<BuildStatusResponse>>,
) -> anyhow::Result<RepositoryBranchData> {
    let repository_url = repository
        .links
        .self_link
        .pop()
        .ok_or_else(|| Err(anyhow!("Did not find self link for repository.")))?;

    Ok(RepositoryBranchData {
        repository_name: repository.name,
        repository_url: repository_url.href,
        standalone_branches: vec![],
        pull_request_target_branches: vec![],
    })
}
