use crate::adapter::db::prupdates::PullRequestEventRepository;
use crate::service::prupdates::model::PullRequestEvent;

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

    pub async fn get_pr_updates(&self) -> anyhow::Result<Vec<PullRequestEvent>> {
        self.pr_event_repository.get_events().await
    }
}
