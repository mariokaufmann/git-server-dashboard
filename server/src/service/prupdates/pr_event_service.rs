use crate::adapter::db::prupdates::PullRequestEventRepository;
use crate::service::prupdates::aggregate::aggregate_events;
use crate::service::prupdates::model::{PullRequestEvent, PullRequestUpdate};
use anyhow::Context;
use chrono::{DateTime, Utc};
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
        last_seen_timestamps: HashMap<String, DateTime<Utc>>,
    ) -> anyhow::Result<Vec<PullRequestUpdate>> {
        let events = self.pr_event_repository.get_events().await?;

        let mut grouped_events: HashMap<String, Vec<PullRequestEvent>> = HashMap::new();
        events
            .into_iter()
            .filter(|event| match last_seen_timestamps.get(&event.pr_id) {
                Some(timestamp) => event.timestamp.gt(timestamp),
                None => true,
            })
            .for_each(|event| {
                grouped_events
                    .entry(event.pr_id.clone())
                    .or_insert_with(Vec::new)
                    .push(event)
            });

        // sort by latest event timestamp
        let mut map_entries: Vec<(String, Vec<PullRequestEvent>)> =
            grouped_events.into_iter().collect();
        map_entries.sort_by_key(|(_pr_id, evts)| {
            evts
                .iter()
                .map(|event| event.timestamp)
                .max()
                .unwrap_or_default()
        });
        map_entries.reverse();

        let updates = map_entries
            .into_iter()
            .map(|(pr_id, evts)| aggregate_events(pr_id, evts))
            .collect::<anyhow::Result<Vec<PullRequestUpdate>>>()
            .context("Could not aggregate events into update.")?;

        Ok(updates)
    }
}
