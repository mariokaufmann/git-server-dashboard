use std::time::Duration;

use log::debug;
use tokio::time::Instant;

use crate::data::model::DashboardData;

pub struct DashboardDataCache {
    dashboard_data: DashboardData,
    last_load_instant: Option<Instant>,
}

impl DashboardDataCache {
    pub fn new() -> Self {
        let dashboard_data = DashboardData {
            last_updated_date: None,
            repositories: vec![],
            currently_refreshing: false,
        };

        Self {
            dashboard_data,
            last_load_instant: None,
        }
    }

    pub fn cache_data(&mut self, data: DashboardData) {
        self.dashboard_data = data;
        self.last_load_instant = Some(Instant::now());
    }

    pub fn get_cached_data(&mut self) -> DashboardData {
        self.dashboard_data.clone()
    }

    pub fn set_refreshing(&mut self, refreshing: bool) {
        self.dashboard_data.currently_refreshing = refreshing;
    }

    pub fn should_reload(&self) -> bool {
        match self.last_load_instant {
            Some(last_load) => {
                // check if data was loaded recently
                if last_load.elapsed() > Duration::from_secs(30) {
                    debug!("Reloading dashboard data.");
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
