use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use log::error;
use serde_json::json;

use crate::api::rest::AppServicesState;

pub async fn get_pr_updates(State(state): State<AppServicesState>) -> impl IntoResponse {
    match state
        .pr_update_service
        .get_pr_updates()
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
