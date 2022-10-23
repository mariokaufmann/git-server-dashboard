use anyhow::{anyhow, Context};
use axum::http;
use chrono::Utc;
use reqwest::Method;

use crate::data::gitlab::request::load_repository_data;
use crate::data::model::DashboardData;
use crate::model::Repository;

mod model;
mod request;

pub struct GitlabClient {
    client: reqwest::Client,
    url: String,
    token: String,
    repositories: Vec<Repository>,
}

impl GitlabClient {
    pub fn new(repositories: &[Repository], url: String, token: String) -> GitlabClient {
        GitlabClient {
            client: reqwest::Client::new(),
            url,
            token,
            repositories: Vec::from(repositories),
        }
    }

    pub async fn request<T>(&self, url: &str) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let full_url = format!("{}/api/v4/projects/{}", self.url, url);
        let response = self
            .client
            .request(Method::GET, full_url)
            .header(
                http::header::AUTHORIZATION,
                format!("Bearer {}", self.token),
            )
            .send()
            .await
            .context("Could not make request to Gitlab.")?;
        if !response.status().is_success() {
            return Err(anyhow!(
                "Unsuccessful response from gitlab for url {}: {}",
                url,
                response.status()
            ));
        };

        let parsed_body: T = response
            .json()
            .await
            .context("Could not parse response body from JSON.")?;
        Ok(parsed_body)
    }

    pub async fn load_dashboard_data(&self) -> anyhow::Result<DashboardData> {
        let mut repositories = Vec::new();
        for repository in &self.repositories {
            let repository_data = load_repository_data(self, repository)
                .await
                .with_context(|| format!("Could not load data for repository {}.", repository))?;
            repositories.push(repository_data);
        }

        let last_updated_date = Utc::now().format("%+").to_string();
        Ok(DashboardData {
            last_updated_date: Some(last_updated_date),
            repositories,
        })
    }
}
