use std::cmp::Reverse;
use std::collections::HashMap;

use anyhow::Context;
use chrono::{DateTime, Utc};

use crate::adapter::db::prupdates::PullRequestEventRepository;
use crate::service::prupdates::aggregate::aggregate_events;
use crate::service::prupdates::model::{PullRequestEvent, PullRequestTimestamp, PullRequestUpdate};

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
        let filtered_events = Self::filter_unseen_events(last_seen_timestamps, events);
        let grouped_events = Self::group_events_by_pr(filtered_events);
        let sorted_events = Self::sort_by_latest_timestamp(grouped_events);
        let updates = Self::aggregate_events_into_updates(sorted_events)?;

        Ok(updates)
    }

    fn filter_unseen_events(
        last_seen_timestamps: HashMap<String, DateTime<Utc>>,
        events: Vec<PullRequestEvent>,
    ) -> Vec<PullRequestEvent> {
        events
            .into_iter()
            .filter(|event| {
                last_seen_timestamps
                    .get(&event.pr_id)
                    .map_or(true, |timestamp| event.timestamp.gt(timestamp))
            })
            .collect()
    }

    fn group_events_by_pr(events: Vec<PullRequestEvent>) -> HashMap<String, Vec<PullRequestEvent>> {
        events
            .into_iter()
            .fold(HashMap::new(), |mut grouped_events, event| {
                grouped_events
                    .entry(event.pr_id.clone())
                    .or_insert_with(Vec::new)
                    .push(event);
                grouped_events
            })
    }

    fn sort_by_latest_timestamp(
        grouped_events: HashMap<String, Vec<PullRequestEvent>>,
    ) -> Vec<(String, Vec<PullRequestEvent>)> {
        let mut sorted_grouped_events: Vec<(String, Vec<PullRequestEvent>)> =
            grouped_events.into_iter().collect();
        sorted_grouped_events
            .sort_by_key(|(_pr, events)| Reverse(Self::get_latest_timestamp(events)));
        sorted_grouped_events
    }

    fn get_latest_timestamp(events: &Vec<PullRequestEvent>) -> PullRequestTimestamp {
        events
            .iter()
            .map(|event| event.timestamp)
            .max()
            .unwrap_or_default()
    }

    fn aggregate_events_into_updates(
        sorted_events: Vec<(String, Vec<PullRequestEvent>)>,
    ) -> anyhow::Result<Vec<PullRequestUpdate>> {
        sorted_events
            .into_iter()
            .map(|(pr_id, evts)| aggregate_events(pr_id, evts))
            .collect::<anyhow::Result<Vec<PullRequestUpdate>>>()
            .context("Could not aggregate events into update.")
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use crate::service::prupdates::model::{PullRequestEvent, PullRequestEventType};

    use super::*;

    #[test]
    fn test_sort_by_latest_timestamp() {
        let mut grouped_events: HashMap<String, Vec<PullRequestEvent>> = HashMap::new();

        let mut events_pr1: Vec<PullRequestEvent> = Vec::new();
        events_pr1.push(PullRequestEvent {
            timestamp: Utc.with_ymd_and_hms(2023, 5, 10, 0, 0, 0).unwrap(),
            ..get_pull_request_event()
        });
        events_pr1.push(PullRequestEvent {
            timestamp: Utc.with_ymd_and_hms(2023, 5, 11, 0, 0, 0).unwrap(),
            ..get_pull_request_event()
        });
        grouped_events.insert("pr1".to_string(), events_pr1);

        let mut events_pr2: Vec<PullRequestEvent> = Vec::new();
        events_pr2.push(PullRequestEvent {
            timestamp: Utc.with_ymd_and_hms(2023, 5, 8, 0, 0, 0).unwrap(),
            ..get_pull_request_event()
        });
        events_pr2.push(PullRequestEvent {
            timestamp: Utc.with_ymd_and_hms(2023, 5, 13, 0, 0, 0).unwrap(),
            ..get_pull_request_event()
        });
        grouped_events.insert("pr2".to_string(), events_pr2);

        let sorted_grouped_events =
            PullRequestUpdateService::sort_by_latest_timestamp(grouped_events);

        assert_eq!(sorted_grouped_events[0].0, "pr2");
        assert_eq!(sorted_grouped_events[1].0, "pr1");
    }

    #[test]
    fn test_get_latest_timestamp() {
        let events = vec![
            PullRequestEvent {
                timestamp: Utc.with_ymd_and_hms(2023, 5, 10, 0, 0, 0).unwrap(),
                ..get_pull_request_event()
            },
            PullRequestEvent {
                timestamp: Utc.with_ymd_and_hms(2023, 5, 12, 0, 0, 0).unwrap(),
                ..get_pull_request_event()
            },
            PullRequestEvent {
                timestamp: Utc.with_ymd_and_hms(2023, 5, 11, 0, 0, 0).unwrap(),
                ..get_pull_request_event()
            },
        ];

        let latest_timestamp = PullRequestUpdateService::get_latest_timestamp(&events);

        assert_eq!(
            latest_timestamp,
            Utc.with_ymd_and_hms(2023, 5, 12, 0, 0, 0).unwrap(),
        );
    }

    fn get_pull_request_event() -> PullRequestEvent {
        PullRequestEvent {
            id: Some(1),
            pr_id: String::from(""),
            event_type: PullRequestEventType::Opened,
            author: String::from(""),
            title: String::from(""),
            repository: String::from(""),
            text: String::from(""),
            timestamp: Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap(),
            pr_link: String::from(""),
        }
    }
}
