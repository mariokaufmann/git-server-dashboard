extern crate core;

use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Context;
use log::{error, info, LevelFilter};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::sync::mpsc::UnboundedSender;

use migration::{Migrator, MigratorTrait};

use crate::adapter::db::prupdates::PullRequestEventRepository;
use crate::api::rest::routes::get_router;
use crate::config::Configuration;
use crate::service::prupdates::pr_event_service::PullRequestUpdateService;
use crate::service::repositories::cache::RepositoriesDataCache;
use crate::service::repositories::loader::DataLoader;
use crate::service::repositories::{keep_loading_repositories_data, LockableCache};

mod adapter;
mod api;
mod config;
mod logger;
mod service;

const DASHBOARD_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = config::load_configuration()
        .context("Could not load configuration from file or environment.")?;
    logger::init_logger(configuration.verbose);

    let mut connect_options = ConnectOptions::new("sqlite:test.sqlite?mode=rwc".to_string());
    connect_options.sqlx_logging_level(LevelFilter::Debug);
    let db_connection = Database::connect(connect_options).await.unwrap();

    Migrator::up(&db_connection, None).await.unwrap();

    let cache = Arc::new(tokio::sync::Mutex::new(RepositoriesDataCache::new()));
    let data_loader = DataLoader::new(&configuration)?;
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    tokio::spawn(keep_loading_repositories_data(
        rx,
        cache.clone(),
        data_loader,
    ));

    start_with_config(configuration.port, cache, db_connection, tx).await?;

    Ok(())
}

async fn start_with_config(
    port: u16,
    cache: LockableCache,
    db_connection: DatabaseConnection,
    reload_sender: UnboundedSender<()>,
) -> anyhow::Result<()> {
    info!("Starting branch dashboard server...");
    let pr_event_repository = PullRequestEventRepository::new(db_connection);
    let pr_event_service = PullRequestUpdateService::new(pr_event_repository);
    match get_router(cache, pr_event_service, reload_sender) {
        Ok(router) => {
            let addr = SocketAddr::from(([0, 0, 0, 0], port));
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
