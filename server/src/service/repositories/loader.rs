use anyhow::{anyhow, Context};

use crate::adapter::bitbucket::repositories::BitbucketClient;
use crate::adapter::gitlab::repositories::GitlabClient;
use crate::service::repositories::model::{RepositoriesData, Repository};
use crate::Configuration;

pub struct DataLoader {
    gitlab_client: Option<GitlabClient>,
    bitbucket_client: Option<BitbucketClient>,
}

impl DataLoader {
    pub fn new(configuration: &Configuration) -> anyhow::Result<Self> {
        let repositories = configuration
            .repositories
            .iter()
            .map(|repo| Repository::from_slug(repo))
            .collect::<anyhow::Result<Vec<Repository>>>()
            .context("Could not parse repositories from configuration.")?;

        let mut gitlab_client = None;
        if let Some(gitlab_config) = &configuration.gitlab {
            gitlab_client = Some(GitlabClient::new(
                &repositories,
                gitlab_config.url.clone(),
                gitlab_config.token.clone(),
            ));
        }

        let mut bitbucket_client = None;
        if let Some(bitbucket_config) = &configuration.bitbucket {
            bitbucket_client = Some(
                BitbucketClient::new(
                    &repositories,
                    bitbucket_config.url.clone(),
                    bitbucket_config.user.clone(),
                    bitbucket_config.password.clone(),
                )
                .context("Could not create bitbucket client.")?,
            );
        }

        if gitlab_client.is_none() && bitbucket_client.is_none() {
            return Err(anyhow!("Invalid configuration: No VCS server configured."));
        }

        Ok(Self {
            gitlab_client,
            bitbucket_client,
        })
    }

    pub async fn load_data(&self) -> anyhow::Result<RepositoriesData> {
        if let Some(gitlab_client) = &self.gitlab_client {
            let data = gitlab_client
                .load_repositories_data()
                .await
                .context("Could not load dashboard data from Gitlab.")?;
            return Ok(data);
        }
        if let Some(bitbucket_client) = &self.bitbucket_client {
            let data = bitbucket_client
                .load_repositories_data()
                .await
                .context("Could not load dashboard data from Bitbucket.")?;
            return Ok(data);
        }
        Err(anyhow!(
            "Could not load data from VCS server: no server configured."
        ))
    }
}
