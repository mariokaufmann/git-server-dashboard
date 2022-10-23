use crate::data::bitbucket::model::{
    BitbucketBuildState, BranchResponse, BuildStatusResponse, PaginatedResponse,
    PullRequestCommentsResponse, PullRequestDetails, PullRequestResponse, RepositoryResponse,
    UserResponse,
};
use crate::data::bitbucket::BitbucketClient;
use crate::data::model::{
    DashboardData, PipelineStatus, PullRequest, PullRequestTargetBranch, RepositoryBranchData,
    StandaloneBranch,
};
use anyhow::{anyhow, Context};
use chrono::{TimeZone, Utc};
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
    let mut user_map: HashMap<String, UserResponse> = HashMap::new();
    // Option<BuildStatusResponse> because not every commit has a corresponding build
    let mut build_status_map: HashMap<String, Option<BuildStatusResponse>> = HashMap::new();

    let mut repositories: Vec<RepositoryBranchData> = Vec::new();

    for project in projects {
        let repository = get_repository(client, project).await?;
        let branches = get_branches(client, project).await?;

        for branch in &branches {
            let commit_id = branch.latest_commit.clone();
            if build_status_map.get(&commit_id).is_none() {
                let build_status = get_build_status(client, &commit_id).await?;
                build_status_map.insert(commit_id, build_status);
            }
        }

        let pull_requests = get_pull_requests(client, project).await?;

        for pull_request in &pull_requests {
            let commit_id = pull_request.details_response.from_ref.latest_commit.clone();
            if build_status_map.get(&commit_id).is_none() {
                let build_status = get_build_status(client, &commit_id).await?;
                build_status_map.insert(commit_id, build_status);
            }

            let user_slug = pull_request.details_response.author.user.slug.clone();
            if user_map.get(&user_slug).is_none() {
                let user = get_user(client, &user_slug).await?;
                user_map.insert(user_slug, user);
            }
        }

        let repository_branch_data = map_repository_data(
            repository,
            branches,
            pull_requests,
            &user_map,
            &build_status_map,
        )?;
        repositories.push(repository_branch_data);
    }

    // TODO remove duplication (e.g. just return array of repos)
    let last_updated_date = Utc::now();
    let formatted_last_updated_date = last_updated_date.format("%+").to_string();
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
    let response: PaginatedResponse<BuildStatusResponse> = client
        .request(&url)
        .await
        .with_context(|| format!("Could not load build status for commit {}.", commit_id))?;
    let build_status = response.values.into_iter().last();
    Ok(build_status)
}

fn map_repository_data(
    mut repository: RepositoryResponse,
    branches: Vec<BranchResponse>,
    pull_requests: Vec<PullRequestDetails>,
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
        .map(|pull_request| &pull_request.details_response.to_ref.display_id)
        .collect();

    let pull_request_target_branches: Vec<PullRequestTargetBranch> =
        pull_request_target_branch_names
            .iter()
            .map(|name| {
                let mapped_pull_requests = pull_requests
                    .iter()
                    .filter(|pr| pr.details_response.to_ref.display_id.eq(*name))
                    .map(|pr| {
                        let source_commit = &pr.details_response.from_ref.latest_commit;
                        let build_status = build_statuses.get(source_commit).ok_or_else(|| {
                            anyhow!(
                                "Did not find cached build status for commit {} and branch {}.",
                                source_commit,
                                pr.details_response.from_ref.display_id
                            )
                        })?;
                        let author_slug = &pr.details_response.author.user.slug;
                        let author = users.get(author_slug).ok_or_else(|| {
                            anyhow!("Did not find cached user for slug {}.", author_slug)
                        })?;
                        let approved = pr
                            .details_response
                            .reviewers
                            .iter()
                            .any(|reviewer| reviewer.approved);

                        let last_updated_date =
                            Utc.timestamp_millis(pr.details_response.updated_date as i64);
                        let formatted_last_updated_date =
                            last_updated_date.format("%+").to_string();
                        let link_response =
                            pr.details_response.links.self_link.first().ok_or_else(|| {
                                anyhow!("Did not find self link for pull request.")
                            })?;

                        Ok(PullRequest {
                            branch_name: pr.details_response.from_ref.display_id.to_owned(),
                            user_name: pr.details_response.author.user.display_name.to_owned(),
                            pipeline_status: map_pipeline_status(build_status),
                            pipeline_url: build_status.as_ref().map(|status| status.url.to_owned()),
                            comment_count: pr.comments_count,
                            approved,
                            user_profile_image: author.avatar_url.to_owned(),
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
                        format!("Could not find branch response for branch {}.", name)
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
                pr.details_response
                    .from_ref
                    .display_id
                    .eq(&branch.display_id)
                    || pr.details_response.to_ref.display_id.eq(&branch.display_id)
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
