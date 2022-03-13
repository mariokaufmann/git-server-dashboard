use anyhow::Context;

#[derive(Clone)]
pub struct Configuration {
    pub verbose: bool,
    pub gitlab_url: String,
    pub gitlab_token: String,
    pub projects: Vec<String>,
}

pub fn load_configuration_from_environment() -> anyhow::Result<Configuration> {
    let verbose_text =
        std::env::var("GITLAB-BRANCH-DASHBOARD_VERBOSE").unwrap_or_else(|_| "false".to_owned());
    let verbose: bool = verbose_text
        .parse()
        .context("Could not parse 'verbose' configuration value.")?;
    let gitlab_url = std::env::var("GITLAB-BRANCH-DASHBOARD_GITLAB-URL").context(
        "Gitlab URL must be set in environment variable 'GITLAB-BRANCH-DASHBOARD_GITLAB-URL'",
    )?;
    let gitlab_token = std::env::var("GITLAB-BRANCH-DASHBOARD_GITLAB-TOKEN").context(
        "Gitlab authentication token must be set in environment variable 'GITLAB-BRANCH-DASHBOARD_GITLAB-TOKEN'",
    )?;
    let projects_string = std::env::var("GITLAB-BRANCH-DASHBOARD_GITLAB-PROJECTS").context(
        "Gitlab projects must be set in environment variable 'GITLAB-BRANCH-DASHBOARD_GITLAB-PROJECTS'",
    )?;
    let projects = projects_string
        .split(';')
        .map(|split| split.to_owned())
        .collect();

    Ok(Configuration {
        verbose,
        gitlab_url,
        gitlab_token,
        projects,
    })
}
