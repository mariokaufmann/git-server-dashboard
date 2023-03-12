use crate::adapter::db::prupdates::PullRequestEventRepository;
use crate::service::prupdates::model::{PullRequestEvent, PullRequestId};
use chrono::{DateTime, Utc};
use log::info;
use serde::de::Unexpected::Map;
use std::collections::HashMap;

#[derive(Clone)]
pub struct PullRequestUpdateService {
    pr_event_repository: PullRequestEventRepository,
}

impl PullRequestUpdateService {
    pub fn new(pr_event_repository: PullRequestEventRepository) -> Self {
        Self {
            pr_event_repository,
        }
    }

    pub async fn save_pr_event(&self, event: PullRequestEvent) -> anyhow::Result<()> {
        self.pr_event_repository.save_event(event).await
    }

    pub async fn get_pr_updates(
        &self,
        last_seen_timestamps: HashMap<PullRequestId, DateTime<Utc>>,
    ) -> anyhow::Result<Vec<PullRequestEvent>> {
        let events = self.pr_event_repository.get_events().await?;

        let filtered_events = events
            .into_iter()
            .filter(|event| match last_seen_timestamps.get(&event.pr_id) {
                Some(timestamp) => event.timestamp.gt(timestamp),
                None => true,
            })
            .collect();

        Ok(filtered_events)
    }
}
