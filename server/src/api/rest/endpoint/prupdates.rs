use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{DateTime, Utc};
use log::error;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

use crate::api::rest::AppServicesState;
use crate::service::prupdates::model::PullRequestTimestamp;

pub async fn get_pr_updates(
    State(state): State<AppServicesState>,
    Json(payload): Json<GetPullRequestUpdatesPayload>,
) -> impl IntoResponse {
    let last_seen_timestamps = payload
        .pull_requests_last_seen
        .into_iter()
        .map(|mapping| (mapping.pr_id, mapping.last_seen_timestamp))
        .collect::<HashMap<String, PullRequestTimestamp>>();

    match state
        .pr_update_service
        .get_pr_updates(last_seen_timestamps)
        .await
        .context("Could not load pull request events from DB.")
    {
        Ok(events) => (StatusCode::OK, Json(json!(events))),
        Err(err) => {
            error!("{:#}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!("Could not load events.")),
            )
        }
    }
}

#[derive(Deserialize)]
pub struct GetPullRequestUpdatesPayload {
    pub pull_requests_last_seen: Vec<PullRequestLastSeenPayload>,
}

#[derive(Deserialize)]
pub struct PullRequestLastSeenPayload {
    pub pr_id: String,
    pub last_seen_timestamp: DateTime<Utc>,
}
