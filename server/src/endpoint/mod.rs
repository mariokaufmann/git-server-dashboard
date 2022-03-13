use std::sync::Arc;

use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use log::error;
use serde_json::json;

use crate::data::gitlab::GitlabClient;
use crate::{Configuration, DASHBOARD_VERSION};

pub mod routes;

async fn get_server_version() -> impl IntoResponse {
    DASHBOARD_VERSION
}

async fn get_dashboard_data(
    Extension(configuration): Extension<Configuration>,
    Extension(gitlab_client): Extension<Arc<GitlabClient>>,
) -> impl IntoResponse {
    match crate::data::load_dashboard_data(&gitlab_client, &configuration.projects).await {
        Ok(data) => (StatusCode::OK, Json(json!(data))),
        Err(err) => {
            error!("Could not load dashboard data: {:#}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!("Could not load dashboard data.")),
            )
        }
    }
}
