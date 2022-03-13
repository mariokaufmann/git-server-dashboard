use std::net::SocketAddr;

use crate::config::Configuration;
use anyhow::Context;
use log::{error, info};

mod config;
mod data;
mod endpoint;
mod logger;

const DASHBOARD_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = config::load_configuration_from_environment()
        .context("Could not load configuration from environment")?;
    logger::init_logger(configuration.verbose);
    start_with_config(configuration).await?;

    Ok(())
}

async fn start_with_config(configuration: Configuration) -> anyhow::Result<()> {
    info!("Starting gitlab branch dashboard server...");
    match endpoint::routes::get_router(&configuration) {
        Ok(router) => {
            let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
            if let Err(err) = axum_server::bind(addr)
                .serve(router.into_make_service())
                .await
            {
                error!("Could not start server: {}", err);
            }
        }
        Err(err) => error!("Could not configure server routes: {}", err),
    }
    Ok(())
}
