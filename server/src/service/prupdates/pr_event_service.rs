use crate::adapter::db::prupdates::PullRequestEventRepository;
use crate::service::prupdates::aggregate::aggregate_events;
use crate::service::prupdates::model::{PullRequestEvent, PullRequestUpdate};
use anyhow::Context;
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use std::ops::Sub;

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

        // sort by pr_id to achieve a stable order of pr updates
        let mut map_entries: Vec<(String, Vec<PullRequestEvent>)> =
            grouped_events.into_iter().collect();
        map_entries.sort_by_key(|(pr_id, _evts)| pr_id.clone());

        let updates = map_entries
            .into_iter()
            .map(|(pr_id, evts)| aggregate_events(pr_id, evts))
            .collect::<anyhow::Result<Vec<PullRequestUpdate>>>()
            .context("Could not aggregate events into update.")?;

        Ok(updates)
    }

    pub async fn clean_up_pr_updates(&self) -> anyhow::Result<()> {
        // TODO do this directly with where clause once date column is migrated
        let oldest_timestamp = Utc::now() - Duration::days(7);
        let events = self.pr_event_repository.get_events().await?;

        for event in events {
            if event.timestamp.lt(&oldest_timestamp) {
                self.pr_event_repository.delete_events()
            }
        }
        let events_to_delete: Vec<PullRequestEvent> = events.into_iter()
            .filter(|event| {
                event.timestamp.lt(&oldest_timestamp)
            }).collect();
        self.pr_event_repository.delete_events(&events_to_delete)
            .await
            .context
    }
}
