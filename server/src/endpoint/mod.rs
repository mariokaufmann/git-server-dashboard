use axum::extract::Extension;
use axum::response::IntoResponse;
use axum::Json;

use crate::{LockableCache, DASHBOARD_VERSION};

pub mod routes;

async fn get_server_version() -> impl IntoResponse {
    DASHBOARD_VERSION
}

async fn get_dashboard_data(Extension(cache): Extension<LockableCache>) -> impl IntoResponse {
    let mut locked_cache = cache.lock().await;
    let data = locked_cache.get_cached_data();
    Json(data)
}
