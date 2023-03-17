use crate::service::prupdates::model::{
    PullRequestEvent, PullRequestEventType, PullRequestUpdate, PullRequestUpdateType,
};
use anyhow::Context;
use std::collections::HashMap;

pub fn aggregate_events(
    pr_id: String,
    mut events: Vec<PullRequestEvent>,
) -> anyhow::Result<PullRequestUpdate> {
    events.sort_by_key(|a| a.timestamp);

    let details = get_update_details(&events);
    let last_event = events.last().context("Could not get last event.")?;

    let update_type = match events.len() {
        1 => map_event_type(&last_event.event_type),
        _ => PullRequestUpdateType::Aggregated,
    };

    Ok(PullRequestUpdate {
        pr_id,
        timestamp: last_event.timestamp,
        repository: last_event.repository.clone(),
        title: last_event.title.clone(),
        author: last_event.author.clone(),
        update_type,
        details,
    })
}

fn get_update_details(events: &[PullRequestEvent]) -> Vec<String> {
    let mut grouped_events: HashMap<PullRequestEventType, Vec<&PullRequestEvent>> = HashMap::new();
    events.iter().for_each(|evt| {
        grouped_events
            .entry(evt.event_type)
            .or_insert_with(Vec::new)
            .push(evt);
    });

    grouped_events
        .into_iter()
        .map(|(event_type, evts)| get_update_detail_for_event_type(event_type, &evts))
        .collect()
}

fn get_update_detail_for_event_type(
    event_type: PullRequestEventType,
    events: &[&PullRequestEvent],
) -> String {
    match event_type {
        PullRequestEventType::Opened => "PR opened".to_string(),
        PullRequestEventType::Approved => match events.len() {
            1 => "PR approved".to_string(),
            _ => format!("{} approvals on PR", events.len()),
        },
        PullRequestEventType::Merged => "PR merged".to_string(),
        PullRequestEventType::CommentAdded => match events.len() {
            1 => "New comment on PR".to_string(),
            _ => format!("{} new comments on PR", events.len()),
        },
    }
}

fn map_event_type(&event_type: &PullRequestEventType) -> PullRequestUpdateType {
    match event_type {
        PullRequestEventType::Opened => PullRequestUpdateType::Opened,
        PullRequestEventType::Approved => PullRequestUpdateType::Approved,
        PullRequestEventType::Merged => PullRequestUpdateType::Merged,
        PullRequestEventType::CommentAdded => PullRequestUpdateType::CommentAdded,
    }
}