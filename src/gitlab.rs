use serde::{Deserialize, Serialize};
use serde_json::Value;
// use std::collections::HashMap;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    #[serde(rename = "object_kind")]
    pub object_kind: String,
    #[serde(rename = "event_name")]
    pub event_name: String,
    pub before: String,
    pub after: String,
    #[serde(rename = "ref")]
    pub ref_field: Option<String>,
    #[serde(rename = "checkout_sha")]
    pub checkout_sha: String,
    pub message: Value,
    #[serde(rename = "user_id")]
    pub user_id: i64,
    #[serde(rename = "user_name")]
    pub user_name: String,
    #[serde(rename = "user_username")]
    pub user_username: String,
    #[serde(rename = "user_email")]
    pub user_email: Value,
    #[serde(rename = "user_avatar")]
    pub user_avatar: String,
    #[serde(rename = "project_id")]
    pub project_id: i64,
    pub project: Project,
    pub commits: Vec<Commit>,
    #[serde(rename = "total_commits_count")]
    pub total_commits_count: i64,
    #[serde(rename = "push_options")]
    pub push_options: PushOptions,
    pub repository: Repository,
}

impl Payload {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: Value,
    #[serde(rename = "web_url")]
    pub web_url: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: Value,
    #[serde(rename = "git_ssh_url")]
    pub git_ssh_url: String,
    #[serde(rename = "git_http_url")]
    pub git_http_url: String,
    pub namespace: String,
    #[serde(rename = "visibility_level")]
    pub visibility_level: i64,
    #[serde(rename = "path_with_namespace")]
    pub path_with_namespace: String,
    #[serde(rename = "default_branch")]
    pub default_branch: String,
    #[serde(rename = "ci_config_path")]
    pub ci_config_path: Value,
    pub homepage: String,
    pub url: String,
    #[serde(rename = "ssh_url")]
    pub ssh_url: String,
    #[serde(rename = "http_url")]
    pub http_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Commit {
    pub id: String,
    pub message: String,
    pub title: String,
    pub timestamp: String,
    pub url: String,
    pub author: Author,
    pub added: Vec<Value>,
    pub modified: Vec<String>,
    pub removed: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushOptions {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub description: Value,
    pub homepage: String,
    #[serde(rename = "git_http_url")]
    pub git_http_url: String,
    #[serde(rename = "git_ssh_url")]
    pub git_ssh_url: String,
    #[serde(rename = "visibility_level")]
    pub visibility_level: i64,
}
