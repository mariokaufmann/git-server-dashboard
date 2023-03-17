use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use log::error;
use serde::de::DeserializeOwned;

use crate::api::rest::AppServicesState;
use crate::api::webhook::model::{
    CommonPullRequestEventPayload, EventType, PullRequestCommentEventPayload, PullRequestPayload,
};
use crate::service::prupdates::model::{
    PullRequestEvent, PullRequestEventType, PullRequestTimestamp,
};
use crate::service::prupdates::pr_event_service::PullRequestUpdateService;

mod model;

#[axum_macros::debug_handler]
pub async fn post_webhook_bitbucket(
    State(state): State<AppServicesState>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    match process_webhook_request(payload, &state.pr_update_service).await {
        Ok(()) => (StatusCode::OK, "Hello Bitbucket!"),
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
    event_type: EventType,
    value: serde_json::Value,
) -> anyhow::Result<PullRequestEvent> {
    let text = get_event_text(&event_type, &value).context("Could not map PR event text.")?;
    let payload = parse_event_payload::<CommonPullRequestEventPayload>(value)?;
    let pr_id = hash_pull_request(&payload.pull_request);

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
    };

    Ok(pull_request_event)
}

fn map_event_type(event_type: &EventType) -> PullRequestEventType {
    match event_type {
        EventType::PROpened => PullRequestEventType::PROpened,
        EventType::PRApproved => PullRequestEventType::PRApproved,
        EventType::PRMerged => PullRequestEventType::PRMerged,
        EventType::PRCommentAdded => PullRequestEventType::PRCommentAdded,
    }
}

fn get_event_text(event_type: &EventType, value: &serde_json::Value) -> anyhow::Result<String> {
    match event_type {
        EventType::PROpened => Ok("".to_string()),
        EventType::PRApproved => Ok("".to_string()),
        EventType::PRMerged => Ok("".to_string()),
        EventType::PRCommentAdded => {
            let payload = parse_event_payload::<PullRequestCommentEventPayload>(value.clone())
                .context("Could not parse PR event comment payload.")?;
            Ok(payload.comment.text)
        }
    }
}

fn hash_pull_request(pull_request: &PullRequestPayload) -> u64 {
    let mut hasher = DefaultHasher::new();
    pull_request.hash(&mut hasher);
    hasher.finish()
}

fn parse_event_payload<T: DeserializeOwned>(value: serde_json::Value) -> anyhow::Result<T> {
    serde_json::from_value::<T>(value).context("Could not parse pull request event payload.")
}

fn map_event_key(event_key: &str) -> Option<EventType> {
    match event_key {
        "pr:opened" => Some(EventType::PROpened),
        "pr:approved" => Some(EventType::PRApproved),
        "pr:merged" => Some(EventType::PRMerged),
        "pr:comment:added" => Some(EventType::PRCommentAdded),
        _ => None,
    }
}
