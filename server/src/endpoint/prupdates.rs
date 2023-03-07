use crate::endpoint::AppState;
use anyhow::{anyhow, Context};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use log::error;
use sea_orm::EntityTrait;
use serde_derive::Serialize;
use serde_json::json;
use std::str::FromStr;

pub async fn get_pr_updates(State(mut state): State<AppState>) -> impl IntoResponse {
    match entity::pull_request_event::Entity::find()
        .all(&mut state.db_connection)
        .await
        .context("Could not load pull request events from DB.")
    {
        Ok(events) => {
            let events = events
                .into_iter()
                .map(|model| {
                    let event_type = PullRequestEventType::from_str(&model.event_type)
                        .map_err(|_| anyhow!("Could not parse event type from DB."))?;
                    Ok(PullRequestEvent {
                        id: Some(model.id),
                        hash: model.hash,
                        event_type,
                        title: model.title,
                        text: model.text,
                    })
                })
                .collect::<anyhow::Result<Vec<PullRequestEvent>>>()
                .context("Could not map DB entities to DTOs.");
            match events {
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
        Err(err) => {
            error!("{:#}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!("Could not load events.")),
            )
        }
    }
}

// TODO move these models to domain
#[derive(Serialize, sea_orm::strum::EnumString)]
pub enum PullRequestEventType {
    Ignored,
    PROpened,
    PRApproved,
    PRMerged,
    PRCommentAdded,
}

#[derive(Serialize)]
pub struct PullRequestEvent {
    pub id: Option<i32>,
    pub hash: i64,
    pub event_type: PullRequestEventType,
    pub title: String,
    pub text: String,
}
