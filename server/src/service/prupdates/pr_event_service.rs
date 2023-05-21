use crate::adapter::db::prupdates::PullRequestEventRepository;
use crate::service::prupdates::aggregate::aggregate_events;
use crate::service::prupdates::model::{PullRequestEvent, PullRequestUpdate};
use anyhow::Context;
use chrono::{DateTime, Duration, Utc};
use log::info;
use std::collections::HashMap;

const EVENT_MAX_AGE_DAYS: i64 = 7;

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
        let oldest_timestamp = Utc::now() - Duration::days(EVENT_MAX_AGE_DAYS);
        let events = self.pr_event_repository.get_events().await?;

        let events_to_delete: Vec<PullRequestEvent> = events
            .into_iter()
            .filter(|event| event.timestamp.lt(&oldest_timestamp))
            .collect();

        if !events_to_delete.is_empty() {
            info!("Will delete {} old PR events", events_to_delete.len());
        }

        self.pr_event_repository
            .delete_events(&events_to_delete)
            .await
            .context("Could not delete PR updates.")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::adapter::db::prupdates::PullRequestEventRepository;
    use crate::service::prupdates::model::{
        PullRequestEvent, PullRequestEventType, PullRequestTimestamp,
    };
    use crate::service::prupdates::pr_event_service::{
        PullRequestUpdateService, EVENT_MAX_AGE_DAYS,
    };
    use chrono::{Duration, Utc};
    use migration::{Migrator, MigratorTrait};
    use sea_orm::{ConnectOptions, Database};
    use std::collections::HashMap;

    async fn get_in_memory_repository() -> PullRequestEventRepository {
        let connect_options = ConnectOptions::new("sqlite::memory:".to_owned());
        let db_connection = Database::connect(connect_options).await.unwrap();
        Migrator::up(&db_connection, None).await.unwrap();
        PullRequestEventRepository::new(db_connection)
    }

    fn get_pr_event_with_timestamp(id: &str, timestamp: PullRequestTimestamp) -> PullRequestEvent {
        PullRequestEvent {
            id: None,
            pr_id: id.to_owned(),
            event_type: PullRequestEventType::Opened,
            author: "author".to_string(),
            title: "title".to_string(),
            repository: "repository".to_owned(),
            text: "text".to_string(),
            timestamp,
            pr_link: "pr_link".to_string(),
        }
    }

    #[tokio::test]
    async fn clean_up_pr_updates_everything_new() {
        let repository = get_in_memory_repository().await;
        let service = PullRequestUpdateService::new(repository);
        service
            .save_pr_event(get_pr_event_with_timestamp(
                "id1",
                Utc::now() - Duration::days(1),
            ))
            .await
            .unwrap();
        service
            .save_pr_event(get_pr_event_with_timestamp(
                "id2",
                Utc::now() - Duration::days(EVENT_MAX_AGE_DAYS - 1),
            ))
            .await
            .unwrap();

        service.clean_up_pr_updates().await.unwrap();

        let events = service.get_pr_updates(HashMap::new()).await.unwrap();

        assert_eq!(2, events.len());
        assert_eq!("id1", events.get(0).unwrap().pr_id);
        assert_eq!("id2", events.get(1).unwrap().pr_id);
    }

    #[tokio::test]
    async fn clean_up_pr_updates_cleaned_up() {
        let repository = get_in_memory_repository().await;
        let service = PullRequestUpdateService::new(repository);
        service
            .save_pr_event(get_pr_event_with_timestamp(
                "id1",
                Utc::now() - Duration::days(1),
            ))
            .await
            .unwrap();
        service
            .save_pr_event(get_pr_event_with_timestamp(
                "id2",
                Utc::now() - Duration::days(EVENT_MAX_AGE_DAYS),
            ))
            .await
            .unwrap();
        service
            .save_pr_event(get_pr_event_with_timestamp(
                "id3",
                Utc::now() - Duration::days(EVENT_MAX_AGE_DAYS + 1),
            ))
            .await
            .unwrap();

        service.clean_up_pr_updates().await.unwrap();

        let events = service.get_pr_updates(HashMap::new()).await.unwrap();

        assert_eq!(1, events.len());
        assert_eq!("id1", events.get(0).unwrap().pr_id);
    }
}
