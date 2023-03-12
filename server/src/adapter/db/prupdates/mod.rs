use std::str::FromStr;

use anyhow::{anyhow, Context};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, NotSet};

use crate::service::prupdates::model::{PullRequestEvent, PullRequestEventType};

mod pull_request_event;

#[derive(Clone)]
pub struct PullRequestEventRepository {
    db: DatabaseConnection,
}

impl PullRequestEventRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        PullRequestEventRepository { db }
    }

    pub async fn save_event(&self, event: PullRequestEvent) -> anyhow::Result<()> {
        let event_entity = pull_request_event::ActiveModel {
            id: NotSet,
            hash: Set(event.hash),
            event_type: Set(event.event_type.to_string()),
            author: Set(event.author),
            date: Set(event.date),
            repository: Set(event.repository),
            title: Set(event.title),
            text: Set(event.text),
        };

        event_entity
            .insert(&self.db)
            .await
            .context("Could not insert pull request event into DB.")?;
        Ok(())
    }

    pub async fn get_events(&self) -> anyhow::Result<Vec<PullRequestEvent>> {
        let events = pull_request_event::Entity::find()
            .all(&self.db)
            .await
            .context("Could not load pull request events from DB.")?;
        let mapped_events = events
            .into_iter()
            .map(|model| {
                let event_type = PullRequestEventType::from_str(&model.event_type)
                    .map_err(|_| anyhow!("Could not parse event type from DB."))?;
                Ok(PullRequestEvent {
                    id: Some(model.id),
                    hash: model.hash,
                    event_type,
                    author: model.author,
                    date: model.date,
                    repository: model.repository,
                    title: model.title,
                    text: model.text,
                })
            })
            .collect::<anyhow::Result<Vec<PullRequestEvent>>>()
            .context("Could not map DB entities to service entities.")?;
        Ok(mapped_events)
    }
}
