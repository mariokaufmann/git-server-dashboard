use anyhow::{anyhow, Context};
use axum::http;
use chrono::Utc;
use reqwest::Method;

use crate::data::gitlab::request::load_repository_data;
use crate::data::model::DashboardData;

mod model;
mod request;

pub struct GitlabClient {
    client: reqwest::Client,
    gitlab_url: String,
    gitlab_token: String,
}

impl GitlabClient {
    pub fn new(gitlab_url: String, gitlab_token: String) -> GitlabClient {
        GitlabClient {
            client: reqwest::Client::new(),
            gitlab_url,
            gitlab_token,
        }
    }

    pub async fn request<T>(&self, url: &str) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let full_url = format!("{}/api/v4/projects/{}", self.gitlab_url, url);
        let response = self
            .client
            .request(Method::GET, full_url)
            .header(
                http::header::AUTHORIZATION,
                format!("Bearer {}", self.gitlab_token),
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
}

pub async fn load_dashboard_data(
    client: &GitlabClient,
    projects: &[String],
) -> anyhow::Result<DashboardData> {
    let mut repositories = Vec::new();
    for project in projects {
        let repository_data = load_repository_data(client, project)
            .await
            .with_context(|| format!("Could not load data for repository {}.", project))?;
        repositories.push(repository_data);
    }
    let last_updated_date = Utc::now();
    let formatted_last_updated_date = format!("{}", last_updated_date.format("%+"));
    Ok(DashboardData {
        last_updated_date: Some(formatted_last_updated_date),
        repositories,
    })
}
