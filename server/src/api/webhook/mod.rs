use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use anyhow::{anyhow, Context};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use log::{error, info};
use serde::de::DeserializeOwned;

use crate::api::rest::AppServicesState;
use crate::api::webhook::model::{
    CommonPullRequestEventPayload, PREventType, PullRequestCommentEventPayload, PullRequestPayload,
};
use crate::service::prupdates::model::{PullRequestEvent, PullRequestEventType};
use crate::service::prupdates::pr_event_service::PullRequestUpdateService;

mod model;

#[axum_macros::debug_handler]
pub async fn post_webhook_bitbucket(
    State(state): State<AppServicesState>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    match process_webhook_request(payload, &state.pr_update_service).await {
        Ok(()) => (StatusCode::OK, ""),
        Err(err) => {
            error!("Could not process webhook from Bitbucket: {:#}", err);
            (StatusCode::BAD_REQUEST, "Could not parse request body.")
        }
    }
}

async fn process_webhook_request(
    value: serde_json::Value,
    pr_update_service: &PullRequestUpdateService,
) -> anyhow::Result<()> {
    let object = value.as_object().context("Payload was not an object.")?;

    let test_key = object.get("test");
    if test_key.is_some() {
        info!("Received test webhook from bitbucket.");
        return Ok(());
    }

    let event_key = object
        .get("eventKey")
        .context("Could not find event key in Webhook payload.")?;

    let parsed_event_key = serde_json::from_value::<String>(event_key.clone())
        .context("Could not parse event key.")?;
    let event_type = map_event_key(&parsed_event_key);

    match event_type {
        Some(event_type) => {
            let pull_request_event = parse_pr_event_payload(event_type, value)
                .context("Could parse PR event payload.")?;
            pr_update_service.save_pr_event(pull_request_event).await?;
            Ok(())
        }
        None => Ok(()),
    }
}

// TODO parse pr link and link it in frontend
fn parse_pr_event_payload(
    event_type: PREventType,
    value: serde_json::Value,
) -> anyhow::Result<PullRequestEvent> {
    let text = get_event_text(&event_type, &value).context("Could not map PR event text.")?;
    let payload = parse_event_payload::<CommonPullRequestEventPayload>(value)?;
    let pr_id = hash_pull_request(&payload.pull_request);
    let pr_link = get_pull_request_link(&payload.pull_request)?;

    let timestamp = chrono::offset::Utc::now();

    // TODO also parse author of change if possible (who commented? who approved?)

    let pull_request_event = PullRequestEvent {
        id: None,
        event_type: map_event_type(&event_type),
        pr_id: pr_id.to_string(),
        author: payload.actor.display_name,
        timestamp,
        repository: payload.pull_request.from_ref.repository.name,
        title: payload.pull_request.title,
        text,
        pr_link,
    };

    Ok(pull_request_event)
}

fn hash_pull_request(pull_request: &PullRequestPayload) -> u64 {
    let mut hasher = DefaultHasher::new();
    pull_request.hash(&mut hasher);
    hasher.finish()
}

fn parse_event_payload<T: DeserializeOwned>(value: serde_json::Value) -> anyhow::Result<T> {
    serde_json::from_value::<T>(value).context("Could not parse pull request event payload.")
}

fn get_event_text(event_type: &PREventType, value: &serde_json::Value) -> anyhow::Result<String> {
    match event_type {
        PREventType::Opened => Ok("".to_string()),
        PREventType::Approved => Ok("".to_string()),
        PREventType::Merged => Ok("".to_string()),
        PREventType::CommentAdded => {
            let payload = parse_event_payload::<PullRequestCommentEventPayload>(value.clone())
                .context("Could not parse PR event comment payload.")?;
            Ok(payload.comment.text)
        }
        PREventType::SourceBranchUpdated => Ok("".to_string()),
    }
}

fn map_event_type(event_type: &PREventType) -> PullRequestEventType {
    match event_type {
        PREventType::Opened => PullRequestEventType::Opened,
        PREventType::Approved => PullRequestEventType::Approved,
        PREventType::Merged => PullRequestEventType::Merged,
        PREventType::CommentAdded => PullRequestEventType::CommentAdded,
        PREventType::SourceBranchUpdated => PullRequestEventType::SourceBranchUpdated,
    }
}

fn map_event_key(event_key: &str) -> Option<PREventType> {
    match event_key {
        "pr:opened" => Some(PREventType::Opened),
        "pr:approved" => Some(PREventType::Approved),
        "pr:merged" => Some(PREventType::Merged),
        "pr:comment:added" => Some(PREventType::CommentAdded),
        "pr:from_ref_updated" => Some(PREventType::SourceBranchUpdated),
        _ => None,
    }
}

fn get_pull_request_link(pull_request: &PullRequestPayload) -> anyhow::Result<String> {
    pull_request
        .links
        .self_links
        .first()
        .map(|link| link.href.clone())
        .ok_or_else(|| anyhow!("Could not find self link on Bitbucket webhook payload."))
}
