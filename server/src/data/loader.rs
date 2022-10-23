use crate::data::bitbucket::BitbucketClient;
use crate::data::gitlab::GitlabClient;
use crate::data::model::DashboardData;
use crate::Configuration;
use anyhow::{anyhow, Context};

pub struct DataLoader {
    gitlab_client: Option<GitlabClient>,
    bitbucket_client: Option<BitbucketClient>,
}

impl DataLoader {
    pub fn new(configuration: &Configuration) -> anyhow::Result<Self> {
        let gitlab_client = configuration.gitlab.as_ref().map(|gitlab_config| {
            GitlabClient::new(
                &configuration.projects,
                gitlab_config.url.clone(),
                gitlab_config.token.clone(),
            )
        });
        let bitbucket_client = configuration.bitbucket.as_ref().map(|bitbucket_config| {
            // TODO properly
            BitbucketClient::new(
                &configuration.projects,
                bitbucket_config.url.clone(),
                bitbucket_config.user.clone(),
                bitbucket_config.password.clone(),
            )
            .unwrap()
        });

        if gitlab_client.is_none() && bitbucket_client.is_none() {
            return Err(anyhow!("Invalid configuration: No VCS server configured."));
        }

        Ok(Self {
            gitlab_client,
            bitbucket_client,
        })
    }

    pub async fn load_data(&self) -> anyhow::Result<DashboardData> {
        if let Some(gitlab_client) = &self.gitlab_client {
            let data = gitlab_client
                .load_dashboard_data()
                .await
                .context("Could not load dashboard data from Gitlab.")?;
            return Ok(data);
        }
        if let Some(bitbucket_client) = &self.bitbucket_client {
            let data = bitbucket_client
                .load_dashboard_data()
                .await
                .context("Could not load dashboard data from Bitbucket.")?;
            return Ok(data);
        }
        Err(anyhow!(
            "Could not load data from VCS server: no server configured."
        ))
    }
}
