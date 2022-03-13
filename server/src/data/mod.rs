use anyhow::Context;

use crate::data::gitlab::GitlabClient;
use crate::data::model::DashboardData;

pub mod gitlab;
mod model;

pub async fn load_dashboard_data(
    client: &GitlabClient,
    projects: &[String],
) -> anyhow::Result<DashboardData> {
    let dashboard_data = gitlab::load_dashboard_data(client, projects)
        .await
        .context("Could not load dashboard data from Gitlab.")?;
    Ok(dashboard_data)
}
