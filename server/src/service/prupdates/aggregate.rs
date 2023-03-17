use crate::service::prupdates::model::{
    PullRequestEvent, PullRequestEventType, PullRequestId, PullRequestUpdate, PullRequestUpdateType,
};
use anyhow::Context;
use std::collections::HashMap;

pub fn aggregate_events(
    pr_id: PullRequestId,
    mut events: Vec<PullRequestEvent>,
) -> anyhow::Result<PullRequestUpdate> {
    events.sort_by_key(|a| a.timestamp);

    let details = get_update_details(&events);
    let last_event = events.last().context("Could not get last event.")?;

    let update_type = match events.len() {
        1 => PullRequestUpdateType::Aggregated,
        _ => map_event_type(&last_event.event_type),
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
            .or_insert_with(|| Vec::new());
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
        PullRequestEventType::PROpened => "PR opened".to_string(),
        PullRequestEventType::PRApproved => match events.len() {
            1 => "PR approved".to_string(),
            _ => format!("{} approvals on PR", events.len()),
        },
        PullRequestEventType::PRMerged => "PR merged".to_string(),
        PullRequestEventType::PRCommentAdded => match events.len() {
            1 => "New comment on PR".to_string(),
            _ => format!("{} new comments on PR", events.len()),
        },
    }
}

fn map_event_type(&event_type: &PullRequestEventType) -> PullRequestUpdateType {
    match event_type {
        PullRequestEventType::PROpened => PullRequestUpdateType::PROpened,
        PullRequestEventType::PRApproved => PullRequestUpdateType::PRApproved,
        PullRequestEventType::PRMerged => PullRequestUpdateType::PRMerged,
        PullRequestEventType::PRCommentAdded => PullRequestUpdateType::PRCommentAdded,
    }
}
