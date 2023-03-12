use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use log::error;

use crate::api::rest::AppServicesState;
use crate::api::webhook::model::{CommonPullRequestEventPayload, EventType, PullRequestPayload};
use crate::service::prupdates::model::{PullRequestEvent, PullRequestEventType};
use crate::service::prupdates::pr_event_service::PullRequestUpdateService;

mod model;

#[axum_macros::debug_handler]
pub async fn post_webhook_bitbucket(
    State(state): State<AppServicesState>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    match parse_webhook_body(payload, &state.pr_update_service).await {
        Ok(()) => (StatusCode::OK, "Hello Bitbucket!"),
        Err(err) => {
            error!("Could not process webhook from Bitbucket: {:#}", err);
            (StatusCode::BAD_REQUEST, "Could not parse request body.")
        }
    }
}

async fn parse_webhook_body(
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
    let pull_request_event = match event_type {
        EventType::PROpened => Some(handle_pr_opened_payload(value)),
        _ => None,
    };
    if let Some(pull_request_event) = pull_request_event {
        let pull_request_event = pull_request_event?;
        pr_update_service.save_pr_event(pull_request_event).await?;
    }
    Ok(())
}

fn handle_pr_opened_payload(value: serde_json::Value) -> anyhow::Result<PullRequestEvent> {
    let payload = parse_common_payload_event(value)?;
    let pr_hash = hash_pull_request(&payload.pull_request);

    let pull_request_event = PullRequestEvent {
        id: None,
        event_type: PullRequestEventType::PROpened,
        // TODO fix this
        hash: pr_hash as i64,
        author: payload.actor.display_name,
        date: payload.date,
        repository: payload.pull_request.from_ref.repository.name,
        title: payload.pull_request.title,
        text: "".to_string(),
    };

    Ok(pull_request_event)
}

fn hash_pull_request(pull_request: &PullRequestPayload) -> u64 {
    let mut hasher = DefaultHasher::new();
    pull_request.hash(&mut hasher);
    hasher.finish()
}

fn parse_common_payload_event(
    value: serde_json::Value,
) -> anyhow::Result<CommonPullRequestEventPayload> {
    serde_json::from_value::<CommonPullRequestEventPayload>(value)
        .context("Could not parse common pull request event payload.")
}

fn map_event_key(event_key: &str) -> EventType {
    match event_key {
        "pr:opened" => EventType::PROpened,
        "pr:approved" => EventType::PRApproved,
        "pr:merged" => EventType::PRMerged,
        "pr:comment:added" => EventType::PRCommentAdded,
        _ => EventType::Ignored,
    }
}
