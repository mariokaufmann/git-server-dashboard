extern crate core;

use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Context;
use log::{error, info, warn};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::config::Configuration;
use crate::data::cache::DashboardDataCache;
use crate::data::loader::DataLoader;

mod config;
mod data;
mod endpoint;
mod logger;

const DASHBOARD_VERSION: &str = env!("CARGO_PKG_VERSION");

type LockableCache = Arc<tokio::sync::Mutex<DashboardDataCache>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = config::load_configuration_from_environment()
        .context("Could not load configuration from environment")?;
    logger::init_logger(configuration.verbose);

    let cache = Arc::new(tokio::sync::Mutex::new(DashboardDataCache::new()));
    let data_loader = DataLoader::new(&configuration)?;
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    tokio::spawn(keep_loading_data(rx, cache.clone(), data_loader));

    start_with_config(cache, tx).await?;

    Ok(())
}

async fn start_with_config(
    cache: LockableCache,
    reload_sender: UnboundedSender<()>,
) -> anyhow::Result<()> {
    info!("Starting branch dashboard server...");
    match endpoint::routes::get_router(cache, reload_sender) {
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

async fn keep_loading_data(
    mut reload_receiver: UnboundedReceiver<()>,
    cache: LockableCache,
    data_loader: DataLoader,
) {
    loop {
        match reload_receiver.recv().await {
            Some(()) => {
                let locked_cache = cache.lock().await;
                let should_reload = locked_cache.should_reload();
                drop(locked_cache);
                if should_reload {
                    info!("Reloading dashboard data.");
                    match data_loader.load_data().await {
                        Ok(data) => {
                            let mut locked_cache = cache.lock().await;
                            locked_cache.cache_data(data);
                            drop(locked_cache);
                        }
                        Err(err) => {
                            error!("Could not reload dashboard data: {:#}", err);
                        }
                    }
                    info!("Reloaded dashboard data.");
                }
            }
            None => {
                warn!("Could not receive reload event anymore.");
                break;
            }
        }
    }
}
