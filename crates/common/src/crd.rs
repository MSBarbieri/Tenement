use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(CustomResource, JsonSchema, Deserialize, Serialize, Debug, Clone)]
#[kube(
    kind = "Application",
    group = "hubify.io",
    version = "v1",
    namespaced,
    shortname = "app"
)]
#[kube(status = "ApplicationStatus")]
pub struct ApplicationSpec {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub url: Url,
    pub categories: Option<Vec<String>>,
    pub openapi_endpoint: Option<Url>,
    pub repository: Option<Repository>,
}

#[derive(CustomResource, JsonSchema, Deserialize, Serialize, Debug, Clone)]
#[kube(
    kind = "Command",
    group = "hubify.io",
    version = "v1",
    namespaced,
    shortname = "cmd"
)]
pub struct CommandSpec {
    name: String,
    version: Option<String>,
    description: Option<String>,
}

#[derive(JsonSchema, Deserialize, Serialize, Debug, Clone)]
pub struct Repository {
    pub name: String,
    pub url: Url,
    pub access_token: Option<String>,
    pub docs_url: Option<Url>,
    pub docs_dir: Option<String>,
}

#[derive(JsonSchema, Deserialize, Serialize, Debug, Clone, Default)]
pub struct ApplicationStatus {
    status: String,
}
