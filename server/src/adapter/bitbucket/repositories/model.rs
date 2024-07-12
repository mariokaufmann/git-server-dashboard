use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct PaginatedResponse<T> {
    pub values: Vec<T>,
}

#[derive(Deserialize)]
pub struct RepositoryResponse {
    pub name: String,
    pub links: LinksResponse,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BranchResponse {
    pub display_id: String,
    pub latest_commit: String,
}

#[derive(Deserialize)]
pub struct BuildStatusResponse {
    pub state: BitbucketBuildState,
    pub url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequestResponse {
    pub from_ref: GitRefResponse,
    pub to_ref: GitRefResponse,
    pub author: PullRequestUserResponse,
    pub reviewers: Vec<PullRequestUserResponse>,
    // epoch time in millis
    pub updated_date: u64,
    pub links: LinksResponse,
    // this property is not officially documented but presumably used by the Bitbucket UI
    pub properties: PullRequestPropertiesResponse,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequestPropertiesResponse {
    pub comment_count: Option<u32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitRefResponse {
    pub display_id: String,
    pub latest_commit: String,
}

#[derive(Deserialize)]
pub struct PullRequestUserResponse {
    pub user: PullRequestUserDetailsResponse,
    pub approved: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequestUserDetailsResponse {
    pub display_name: String,
    pub slug: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub avatar_url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinksResponse {
    #[serde(rename = "self")]
    pub self_link: Vec<LinkResponse>,
}

#[derive(Deserialize)]
pub struct LinkResponse {
    pub href: String,
}

#[derive(Deserialize)]
pub enum BitbucketBuildState {
    #[serde(rename = "SUCCESSFUL")]
    Successful,
    #[serde(rename = "INPROGRESS")]
    InProgress,
    #[serde(rename = "FAILED")]
    Failed,
}
