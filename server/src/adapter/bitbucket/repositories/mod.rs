use anyhow::{anyhow, Context};
use axum::http::Method;

use crate::adapter::bitbucket::repositories::request::load_repositories_data;
use crate::service::repositories::model::{RepositoriesData, Repository};

mod model;
mod request;

pub struct BitbucketClient {
    client: reqwest::Client,
    url: String,
    user: String,
    password: String,
    repositories: Vec<Repository>,
}

impl BitbucketClient {
    pub fn new(
        repositories: &[Repository],
        url: String,
        user: String,
        password: String,
    ) -> anyhow::Result<BitbucketClient> {
        Ok(BitbucketClient {
            client: reqwest::Client::new(),
            url,
            user,
            password,
            repositories: Vec::from(repositories),
        })
    }

    pub async fn request<T>(&self, url: &str) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let full_url = format!("{}/rest/{}", self.url, url);
        let response = self
            .client
            .request(Method::GET, &full_url)
            .basic_auth(&self.user, Some(&self.password))
            .send()
            .await
            .context("Could not make request to Bitbucket.")?;
        if !response.status().is_success() {
            return Err(anyhow!(
                "Unsuccessful response from bitbucket for url {}: {}",
                full_url,
                response.status()
            ));
        };

        let parsed_body: T = response
            .json()
            .await
            .context("Could not parse response body from JSON.")?;
        Ok(parsed_body)
    }

    pub async fn load_repositories_data(&self) -> anyhow::Result<RepositoriesData> {
        load_repositories_data(&self.url, self, &self.repositories).await
    }
}
