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

    // sort by event type to achieve a stable order of details
    let mut map_entries: Vec<(PullRequestEventType, Vec<&PullRequestEvent>)> =
        grouped_events.into_iter().collect();
    map_entries.sort_by_key(|(event_type, _evts)| event_type.clone());

    map_entries
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
        PullRequestEventType::SourceBranchUpdated => match events.len() {
            1 => "New update on PR".to_string(),
            _ => format!("{} new updates on PR", events.len()),
        },
    }
}

fn map_event_type(&event_type: &PullRequestEventType) -> PullRequestUpdateType {
    match event_type {
        PullRequestEventType::Opened => PullRequestUpdateType::Opened,
        PullRequestEventType::Approved => PullRequestUpdateType::Approved,
        PullRequestEventType::Merged => PullRequestUpdateType::Merged,
        PullRequestEventType::CommentAdded => PullRequestUpdateType::CommentAdded,
        PullRequestEventType::SourceBranchUpdated => PullRequestUpdateType::SourceBranchUpdated,
    }
}

#[cfg(test)]
mod tests {
    use crate::service::prupdates::aggregate::get_update_details;
    use crate::service::prupdates::model::{PullRequestEvent, PullRequestEventType};

    fn get_pr_event(event_type: PullRequestEventType) -> PullRequestEvent {
        PullRequestEvent {
            id: Some(1),
            repository: "repo1".to_string(),
            pr_id: "pr_1".to_string(),
            event_type,
            timestamp: chrono::offset::Utc::now(),
            author: "author1".to_string(),
            text: "text".to_string(),
            title: "title".to_string(),
        }
    }

    #[test]
    fn get_update_details_sorted() {
        let events: Vec<PullRequestEvent> = vec![
            get_pr_event(PullRequestEventType::SourceBranchUpdated),
            get_pr_event(PullRequestEventType::CommentAdded),
            get_pr_event(PullRequestEventType::CommentAdded),
        ];

        let update_details = get_update_details(&events);

        assert_eq!(
            update_details,
            vec![
                "2 new comments on PR".to_string(),
                "New update on PR".to_string(),
            ]
        )
    }
}
