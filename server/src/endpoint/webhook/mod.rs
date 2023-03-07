use crate::endpoint::webhook::db::save_event;
use crate::endpoint::webhook::model::{
    EventType, PullRequestEvent, PullRequestEventType, PullRequestOpenedPayload,
};
use crate::endpoint::AppState;
use anyhow::{anyhow, Context};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use log::{error, info};
use sea_orm::DatabaseConnection;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// TODO move this to db adapter
mod db;
mod model;

#[axum_macros::debug_handler]
pub async fn post_webhook_bitbucket(
    State(mut state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    match parse_webhook_body(payload, &mut state.db_connection).await {
        Ok(()) => (StatusCode::OK, "Hello Bitbucket!"),
        Err(err) => {
            error!("Could not process webhook from Bitbucket: {:#}", err);
            (StatusCode::BAD_REQUEST, "Could not parse request body.")
        }
    }
}

async fn parse_webhook_body(
    value: serde_json::Value,
    db: &mut DatabaseConnection,
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
        save_event(db, pull_request_event).await?;
    }
    Ok(())
}

fn handle_pr_opened_payload(value: serde_json::Value) -> anyhow::Result<PullRequestEvent> {
    let payload = serde_json::from_value::<PullRequestOpenedPayload>(value)
        .context("Could not parse pr:opened event from payload.")?;

    let mut hasher = DefaultHasher::new();
    payload.pull_request.hash(&mut hasher);
    let pr_hash = hasher.finish();

    let pull_request_event = PullRequestEvent {
        id: None,
        event_type: PullRequestEventType::PROpened,
        // TODO fix this
        hash: pr_hash as i64,
        title: payload.pull_request.title,
        text: "".to_string(),
    };

    Ok(pull_request_event)
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
