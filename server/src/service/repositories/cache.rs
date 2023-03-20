use std::time::Duration;

use log::debug;
use tokio::time::Instant;

use crate::service::repositories::model::RepositoriesData;

pub struct RepositoriesDataCache {
    repositories_data: RepositoriesData,
    last_load_instant: Option<Instant>,
}

impl RepositoriesDataCache {
    pub fn new() -> Self {
        let repositories_data = RepositoriesData {
            last_updated_date: None,
            repositories: vec![],
            currently_refreshing: false,
        };

        Self {
            repositories_data,
            last_load_instant: None,
        }
    }

    pub fn cache_data(&mut self, data: RepositoriesData) {
        self.repositories_data = data;
        self.last_load_instant = Some(Instant::now());
    }

    pub fn get_cached_data(&mut self) -> RepositoriesData {
        self.repositories_data.clone()
    }

    pub fn set_refreshing(&mut self, refreshing: bool) {
        self.repositories_data.currently_refreshing = refreshing;
    }

    pub fn should_reload(&self) -> bool {
        match self.last_load_instant {
            Some(last_load) => {
                // check if data was loaded recently
                if last_load.elapsed() > Duration::from_secs(30) {
                    debug!("Reloading repositories data.");
                    true
                } else {
                    debug!("Won't reload the data as it has been loaded recently.");
                    false
                }
            }
            // data has never been loaded yet
            None => {
                debug!("Loading data for the first time.");
                true
            }
        }
    }
}
