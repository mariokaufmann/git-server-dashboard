use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Context};
use chrono::{TimeZone, Utc};

use crate::adapter::bitbucket::repositories::model::{
    BitbucketBuildState, BranchResponse, BuildStatusResponse, PaginatedResponse,
    PullRequestResponse, RepositoryResponse, UserResponse,
};
use crate::adapter::bitbucket::repositories::BitbucketClient;
use crate::service::repositories::model::{
    PipelineStatus, PullRequest, PullRequestTargetBranch, RepositoriesData, Repository,
    RepositoryBranchData, StandaloneBranch,
};

fn get_repo_sub_url(repository: &Repository, suffix: &str) -> String {
    format!(
        "api/latest/projects/{}/repos/{}/{}",
        repository.group, repository.name, suffix
    )
}

fn get_build_status_url(commit_id: &str) -> String {
    format!("build-status/latest/commits/{commit_id}")
}

fn get_user_url(user_slug: &str) -> String {
    format!("api/latest/users/{user_slug}?avatarSize=32")
}

pub async fn load_repositories_data(
    bitbucket_url: &str,
    client: &BitbucketClient,
    repositories: &[Repository],
) -> anyhow::Result<RepositoriesData> {
    let mut user_map: HashMap<String, UserResponse> = HashMap::new();
    // Option<BuildStatusResponse> because not every commit has a corresponding build
    let mut build_status_map: HashMap<String, Option<BuildStatusResponse>> = HashMap::new();

    let mut repository_branch_datas: Vec<RepositoryBranchData> = Vec::new();

    for repository in repositories {
        let repository_response = get_repository(client, repository).await?;
        let branches = get_branches(client, repository).await?;

        for branch in &branches {
            let commit_id = branch.latest_commit.clone();
            if build_status_map.get(&commit_id).is_none() {
                let build_status = get_build_status(client, &commit_id).await?;
                build_status_map.insert(commit_id, build_status);
            }
        }

        let pull_requests = get_pull_requests(client, repository).await?;

        for pull_request in &pull_requests {
            let commit_id = pull_request.from_ref.latest_commit.clone();
            if build_status_map.get(&commit_id).is_none() {
                let build_status = get_build_status(client, &commit_id).await?;
                build_status_map.insert(commit_id, build_status);
            }

            let user_slug = pull_request.author.user.slug.clone();
            if user_map.get(&user_slug).is_none() {
                let user = get_user(client, &user_slug).await?;
                user_map.insert(user_slug, user);
            }
        }

        let repository_branch_data = map_repository_data(
            bitbucket_url,
            repository_response,
            branches,
            pull_requests,
            &user_map,
            &build_status_map,
        )?;
        repository_branch_datas.push(repository_branch_data);
    }

    let last_updated_date = Utc::now().format("%+").to_string();
    Ok(RepositoriesData {
        last_updated_date: Some(last_updated_date),
        repositories: repository_branch_datas,
        currently_refreshing: false,
    })
}

async fn get_repository(
    client: &BitbucketClient,
    repository: &Repository,
) -> anyhow::Result<RepositoryResponse> {
    let url = get_repo_sub_url(repository, "");
    let response = client.request(&url).await.with_context(|| {
        format!("Could not load repository details for repository: {repository}")
    })?;
    Ok(response)
}

async fn get_branches(
    client: &BitbucketClient,
    repository: &Repository,
) -> anyhow::Result<Vec<BranchResponse>> {
    let url = get_repo_sub_url(repository, "branches");
    let response: PaginatedResponse<BranchResponse> = client
        .request(&url)
        .await
        .with_context(|| format!("Could not load branches for repository: {repository}"))?;

    Ok(response.values)
}

async fn get_pull_requests(
    client: &BitbucketClient,
    repository: &Repository,
) -> anyhow::Result<Vec<PullRequestResponse>> {
    let pull_request_url = get_repo_sub_url(repository, "pull-requests");
    let pull_request_response: PaginatedResponse<PullRequestResponse> = client
        .request(&pull_request_url)
        .await
        .with_context(|| format!("Could not load pull requests for repository: {repository}"))?;

    Ok(pull_request_response.values)
}

async fn get_user(client: &BitbucketClient, user_slug: &str) -> anyhow::Result<UserResponse> {
    let url = get_user_url(user_slug);
    let response = client
        .request(&url)
        .await
        .with_context(|| format!("Could not load user with slug: {user_slug}"))?;
    Ok(response)
}

async fn get_build_status(
    client: &BitbucketClient,
    commit_id: &str,
) -> anyhow::Result<Option<BuildStatusResponse>> {
    let url = get_build_status_url(commit_id);
    let response: PaginatedResponse<BuildStatusResponse> = client
        .request(&url)
        .await
        .with_context(|| format!("Could not load build status for commit {commit_id}."))?;
    // get first element
    let build_status = response.values.into_iter().take(1).last();
    Ok(build_status)
}

fn map_repository_data(
    bitbucket_url: &str,
    mut repository: RepositoryResponse,
    branches: Vec<BranchResponse>,
    pull_requests: Vec<PullRequestResponse>,
    users: &HashMap<String, UserResponse>,
    build_statuses: &HashMap<String, Option<BuildStatusResponse>>,
) -> anyhow::Result<RepositoryBranchData> {
    let repository_url = repository
        .links
        .self_link
        .pop()
        .ok_or_else(|| anyhow!("Did not find self link for repository."))?;

    let pull_request_target_branch_names: HashSet<&String> = pull_requests
        .iter()
        .map(|pull_request| &pull_request.to_ref.display_id)
        .collect();

    let pull_request_target_branches: Vec<PullRequestTargetBranch> =
        pull_request_target_branch_names
            .iter()
            .map(|name| {
                let mapped_pull_requests = pull_requests
                    .iter()
                    .filter(|pr| pr.to_ref.display_id.eq(*name))
                    .map(|pr| {
                        let source_commit = &pr.from_ref.latest_commit;
                        let build_status = build_statuses.get(source_commit).ok_or_else(|| {
                            anyhow!(
                                "Did not find cached build status for commit {} and branch {}.",
                                source_commit,
                                pr.from_ref.display_id
                            )
                        })?;
                        let author_slug = &pr.author.user.slug;
                        let author = users.get(author_slug).ok_or_else(|| {
                            anyhow!("Did not find cached user for slug {}.", author_slug)
                        })?;
                        let approved = pr.reviewers.iter().any(|reviewer| reviewer.approved);

                        let last_updated_date = Utc
                            .timestamp_millis_opt(pr.updated_date as i64)
                            .single()
                            .ok_or_else(|| anyhow!("Could not parse milliseconds PR timestamp."))?;
                        let formatted_last_updated_date =
                            last_updated_date.format("%+").to_string();
                        let link_response =
                            pr.links.self_link.first().ok_or_else(|| {
                                anyhow!("Did not find self link for pull request.")
                            })?;

                        let mut avatar_url = author.avatar_url.to_owned();
                        if avatar_url.starts_with('/') {
                            avatar_url = format!("{bitbucket_url}{avatar_url}");
                        }

                        Ok(PullRequest {
                            branch_name: pr.from_ref.display_id.to_owned(),
                            user_name: pr.author.user.display_name.to_owned(),
                            pipeline_status: map_pipeline_status(build_status),
                            pipeline_url: build_status.as_ref().map(|status| status.url.to_owned()),
                            comment_count: pr.properties.comment_count.unwrap_or(0),
                            approved,
                            user_profile_image: avatar_url,
                            last_activity_date: formatted_last_updated_date,
                            link_url: link_response.href.to_owned(),
                        })
                    })
                    .collect::<anyhow::Result<Vec<PullRequest>>>()
                    .context("Could not map pull requests.")?;

                let target_branch_response = branches
                    .iter()
                    .find(|branch| branch.display_id.eq(*name))
                    .with_context(|| {
                        format!("Could not find branch response for branch {name}.")
                    })?;

                let target_branch_commit = &target_branch_response.latest_commit;
                let target_branch_build_status =
                    build_statuses.get(target_branch_commit).ok_or_else(|| {
                        anyhow!(
                            "Did not find build status for commit {} and branch {}.",
                            target_branch_commit,
                            target_branch_response.display_id
                        )
                    })?;

                Ok(PullRequestTargetBranch {
                    branch_name: name.to_string(),
                    pipeline_status: map_pipeline_status(target_branch_build_status),
                    pipeline_url: target_branch_build_status
                        .as_ref()
                        .map(|status| status.url.to_owned()),
                    pull_requests: mapped_pull_requests,
                })
            })
            .collect::<anyhow::Result<Vec<PullRequestTargetBranch>>>()
            .context("Could not map pull request details.")?;

    let standalone_branches = branches
        .iter()
        .filter(|branch| {
            !pull_requests.iter().any(|pr| {
                pr.from_ref.display_id.eq(&branch.display_id)
                    || pr.to_ref.display_id.eq(&branch.display_id)
            })
        })
        .map(|branch| {
            let commit = &branch.latest_commit;
            let build_status = build_statuses.get(commit).ok_or_else(|| {
                anyhow!(
                    "Did not find cached build status for commit {} and branch {}.",
                    commit,
                    branch.display_id
                )
            })?;

            Ok(StandaloneBranch {
                branch_name: branch.display_id.to_owned(),
                pipeline_status: map_pipeline_status(build_status),
                pipeline_url: build_status.as_ref().map(|status| status.url.to_owned()),
            })
        })
        .collect::<anyhow::Result<Vec<StandaloneBranch>>>()
        .context("Could not map standalone branches.")?;

    Ok(RepositoryBranchData {
        repository_name: repository.name,
        repository_url: repository_url.href,
        standalone_branches,
        pull_request_target_branches,
    })
}

fn map_pipeline_status(response: &Option<BuildStatusResponse>) -> PipelineStatus {
    match response {
        Some(response) => match response.state {
            BitbucketBuildState::Successful => PipelineStatus::Successful,
            BitbucketBuildState::InProgress => PipelineStatus::Running,
            BitbucketBuildState::Failed => PipelineStatus::Failed,
        },
        None => PipelineStatus::None,
    }
}
