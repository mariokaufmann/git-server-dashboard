use std::path::Path;

use anyhow::Context;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum VCSServerType {
    Gitlab,
    Bitbucket,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GitlabConfiguration {
    pub url: String,
    pub token: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BitbucketConfiguration {
    pub url: String,
    pub user: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub verbose: bool,
    pub port: u16,
    pub gitlab: Option<GitlabConfiguration>,
    pub bitbucket: Option<BitbucketConfiguration>,
    pub repositories: Vec<String>,
}

pub fn load_configuration() -> anyhow::Result<Configuration> {
    let mut config_builder = config::Config::builder()
        .add_source(config::Environment::with_prefix("BRANCHDASHBOARD").separator("_"));

    let cwd_file_path = Path::new("./config.json5");
    if cwd_file_path.exists() {
        config_builder = config_builder.add_source(config::File::from(cwd_file_path));
    }

    let docker_file_path = Path::new("/app/config.json5");
    if docker_file_path.exists() {
        config_builder = config_builder.add_source(config::File::from(docker_file_path));
    }

    let config = config_builder
        .build()
        .context("Could not load configuration.")?;

    let configuration: Configuration = config
        .try_deserialize()
        .context("Could not read configuration.")?;

    println!("Read configuration values: {:?}", configuration);

    Ok(configuration)
}
