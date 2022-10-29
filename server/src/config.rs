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

pub fn load_configuration_from_environment() -> anyhow::Result<Configuration> {
    let config = config::Config::builder()
        .add_source(config::File::with_name("./config"))
        .add_source(config::Environment::with_prefix("BRANCHDASHBOARD").separator("_"))
        .build()
        .context("Could not load configuration.")?;

    let configuration: Configuration = config
        .try_deserialize()
        .context("Could not read configuration.")?;

    println!("Read configuration values: {:?}", configuration);

    Ok(configuration)
}
