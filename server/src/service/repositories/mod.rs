use std::sync::Arc;

use log::{error, info, warn};
use tokio::sync::mpsc::UnboundedReceiver;

use crate::service::repositories::cache::RepositoriesDataCache;
use crate::service::repositories::loader::DataLoader;

pub mod cache;
pub mod loader;
pub mod model;

pub type LockableCache = Arc<tokio::sync::Mutex<RepositoriesDataCache>>;

pub async fn keep_loading_repositories_data(
    mut reload_receiver: UnboundedReceiver<()>,
    cache: LockableCache,
    data_loader: DataLoader,
) {
    loop {
        match reload_receiver.recv().await {
            Some(()) => {
                let mut locked_cache = cache.lock().await;
                let should_reload = locked_cache.should_reload();
                if should_reload {
                    info!("Reloading dashboard data.");
                    locked_cache.set_refreshing(true);
                }
                drop(locked_cache);
                if should_reload {
                    match data_loader.load_data().await {
                        Ok(data) => {
                            let mut locked_cache = cache.lock().await;
                            locked_cache.cache_data(data);
                            locked_cache.set_refreshing(false);
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
