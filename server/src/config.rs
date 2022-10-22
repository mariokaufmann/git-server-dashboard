use anyhow::Context;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub enum VCSServerType {
    Gitlab,
    Bitbucket,
}

#[derive(Clone)]
pub struct Configuration {
    pub verbose: bool,
    pub vcs_server_type: VCSServerType,
    pub vcs_server_url: String,
    pub vcs_server_token: String,
    pub projects: Vec<String>,
}

pub fn load_configuration_from_environment() -> anyhow::Result<Configuration> {
    let verbose_text =
        std::env::var("BRANCH-DASHBOARD_VERBOSE").unwrap_or_else(|_| "false".to_owned());
    let verbose: bool = verbose_text
        .parse()
        .context("Could not parse 'verbose' configuration value.")?;
    let vcs_server_type = std::env::var("BRANCH-DASHBOARD_VCS-SERVER-TYPE").context(
        "VCS Server Type (e.g. 'Gitlab', 'Bitbucket') must be set in environment variable 'BRANCH-DASHBOARD_VCS-SERVER-TYPE'",
    )?;
    let vcs_server_type: VCSServerType = serde_json::from_str(&vcs_server_type).context(
        "Could not deserialize value of environment variable 'BRANCH-DASHBOARD_VCS-SERVER-TYPE' to VCS server type."
    )?;
    let vcs_server_url = std::env::var("BRANCH-DASHBOARD_VCS-SERVER-URL").context(
        "VCS Server (e.g. Gitlab, Bitbucket) URL must be set in environment variable 'BRANCH-DASHBOARD_VCS-SERVER-URL'",
    )?;
    let vcs_server_token = std::env::var("BRANCH-DASHBOARD_VCS-SERVER-TOKEN").context(
        "Server authentication token must be set in environment variable 'BRANCH-DASHBOARD_VCS-SERVER-TOKEN'",
    )?;
    let projects_string = std::env::var("BRANCH-DASHBOARD-PROJECTS")
        .context("Projects must be set in environment variable 'BRANCH-DASHBOARD-PROJECTS'")?;
    let projects = projects_string
        .split(';')
        .map(|split| split.to_owned())
        .collect();

    Ok(Configuration {
        verbose,
        vcs_server_type,
        vcs_server_url,
        vcs_server_token,
        projects,
    })
}
