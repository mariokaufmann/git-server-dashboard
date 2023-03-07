use axum::extract::{Extension, State};
use axum::response::IntoResponse;
use axum::Json;
use log::warn;
use sea_orm::DatabaseConnection;
use tokio::sync::mpsc::UnboundedSender;

use crate::{LockableCache, DASHBOARD_VERSION};

mod prupdates;
pub mod routes;
mod webhook;

#[derive(Clone)]
pub struct AppState {
    db_connection: DatabaseConnection,
}

async fn get_server_version() -> impl IntoResponse {
    DASHBOARD_VERSION
}

async fn get_dashboard_data(
    Extension(cache): Extension<LockableCache>,
    reload_sender: Extension<UnboundedSender<()>>,
) -> impl IntoResponse {
    let mut locked_cache = cache.lock().await;
    let data = locked_cache.get_cached_data();
    if let Err(err) = reload_sender.send(()) {
        warn!("Could not send reload event: {}.", err);
    }
    Json(data)
}
