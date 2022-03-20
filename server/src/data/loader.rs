use crate::data::gitlab::{load_dashboard_data, GitlabClient};
use crate::data::model::DashboardData;
use crate::Configuration;
use anyhow::Context;

pub struct DataLoader {
    projects: Vec<String>,
    gitlab_client: GitlabClient,
}

impl DataLoader {
    pub fn new(configuration: &Configuration) -> Self {
        let gitlab_client = GitlabClient::new(
            configuration.gitlab_url.to_string(),
            configuration.gitlab_token.to_string(),
        );
        let projects = configuration.projects.clone();

        Self {
            gitlab_client,
            projects,
        }
    }

    pub async fn load_data(&self) -> anyhow::Result<DashboardData> {
        let data = load_dashboard_data(&self.gitlab_client, &self.projects)
            .await
            .context("Could not load dashboard data.")?;
        Ok(data)
    }
}
