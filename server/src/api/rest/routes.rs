use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::Router;
use log::error;
use tokio::sync::mpsc::UnboundedSender;
use tower::ServiceBuilder;

use crate::api::rest::endpoint::prupdates::get_pr_updates;
use crate::api::rest::endpoint::{get_dashboard_data, get_server_version};
use crate::api::rest::AppServicesState;
use crate::api::webhook::post_webhook_bitbucket;
use crate::service::prupdates::pr_event_service::PullRequestUpdateService;
use crate::LockableCache;

pub fn get_router(
    cache: LockableCache,
    pr_event_service: PullRequestUpdateService,
    reload_sender: UnboundedSender<()>,
) -> anyhow::Result<Router> {
    let state = AppServicesState {
        pr_update_service: pr_event_service,
    };

    let api_router = Router::new()
        .route("/api/version", axum::routing::get(get_server_version))
        .route(
            "/api/dashboard-data",
            axum::routing::get(get_dashboard_data),
        )
        .route(
            "/webhook/bitbucket",
            axum::routing::post(post_webhook_bitbucket),
        )
        .route("/api/pr-updates", axum::routing::get(get_pr_updates))
        .layer(axum::extract::Extension(cache))
        .layer(axum::extract::Extension(reload_sender))
        .with_state(state);

    // routes (matched from bottom to top from more specific to less specific)
    let router = Router::new()
        .fallback_service(
            axum::routing::get_service(tower_http::services::ServeDir::new("static")).handle_error(
                |err| async move { error!("error occurred when serving static file: {}.", err) },
            ),
        )
        .merge(api_router);

    let middleware_stack = ServiceBuilder::new().layer(HandleErrorLayer::new(|error| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {error}"),
        )
    }));

    Ok(router.layer(middleware_stack))
}
