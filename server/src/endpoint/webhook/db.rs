use crate::endpoint::webhook::model::PullRequestEvent;
use anyhow::Context;
use log::error;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, NotSet};

pub async fn save_event(
    db: &mut DatabaseConnection,
    event: PullRequestEvent,
) -> anyhow::Result<()> {
    let event_entity = entity::pull_request_event::ActiveModel {
        id: NotSet,
        hash: Set(event.hash),
        event_type: Set(event.event_type.to_string()),
        title: Set(event.title),
        text: Set(event.text),
    };

    event_entity
        .insert(db)
        .await
        .context("Could not insert pull request event into DB.")?;
    Ok(())
}
