use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::Router;
use log::error;
use std::sync::Arc;
use tower::ServiceBuilder;

use crate::data::gitlab::GitlabClient;
use crate::endpoint::{get_dashboard_data, get_server_version};
use crate::Configuration;

pub fn get_router(configuration: &Configuration) -> anyhow::Result<Router> {
    let gitlab_client = Arc::new(GitlabClient::new(
        configuration.gitlab_url.to_owned(),
        configuration.gitlab_token.to_owned(),
    ));

    let api_router = Router::new()
        .route("/api/version", axum::routing::get(get_server_version))
        .route(
            "/api/dashboard-data",
            axum::routing::get(get_dashboard_data),
        )
        .layer(axum::extract::Extension(configuration.clone()))
        .layer(axum::extract::Extension(gitlab_client));

    // routes (matched from bottom to top from more specific to less specific)
    let router = Router::new()
        .fallback(
            axum::routing::get_service(tower_http::services::ServeDir::new("static")).handle_error(
                |err| async move { error!("error occurred when serving static file: {}.", err) },
            ),
        )
        .merge(api_router);

    let middleware_stack = ServiceBuilder::new().layer(HandleErrorLayer::new(|error| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", error),
        )
    }));

    Ok(router.layer(middleware_stack))
}
