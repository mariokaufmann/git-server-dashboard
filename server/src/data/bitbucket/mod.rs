use crate::data::bitbucket::request::load_dashboard_data;
use crate::data::model::DashboardData;
use anyhow::{anyhow, Context};
use axum::http::Method;

mod model;
mod request;

pub struct BitbucketClient {
    client: reqwest::Client,
    url: String,
    user: String,
    password: String,
    repositories: Vec<(String, String)>,
}

fn parse_project(project: &String) -> anyhow::Result<(String, String)> {
    let mut parts: Vec<&str> = project.split('/').collect();
    let repository_name = parts
        .pop()
        .ok_or_else(|| anyhow!("Could not parse repository name from {}.", project))?;
    let project_name = parts
        .pop()
        .ok_or_else(|| anyhow!("Could not parse project name from {}.", project))?;
    Ok((project_name.to_string(), repository_name.to_string()))
}

impl BitbucketClient {
    pub fn new(
        projects: &[String],
        url: String,
        user: String,
        password: String,
    ) -> anyhow::Result<BitbucketClient> {
        let repositories: anyhow::Result<Vec<(String, String)>> =
            projects.iter().map(parse_project).collect();
        let repositories = repositories?;
        Ok(BitbucketClient {
            client: reqwest::Client::new(),
            url,
            user,
            password,
            repositories,
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

    pub async fn load_dashboard_data(&self) -> anyhow::Result<DashboardData> {
        load_dashboard_data(self, &self.repositories).await
    }
}
