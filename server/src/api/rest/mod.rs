use crate::service::prupdates::pr_event_service::PullRequestUpdateService;

mod endpoint;
pub mod routes;

#[derive(Clone)]
pub struct AppServicesState {
    pub pr_update_service: PullRequestUpdateService,
}
