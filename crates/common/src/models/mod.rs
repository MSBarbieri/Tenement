use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(JsonSchema, Deserialize, Serialize, Debug, Clone)]
pub struct Repository {
    pub name: String,
    pub url: Url,
    pub access_token: Option<String>,
    pub docs_url: Option<Url>,
    pub docs_dir: Option<String>,
}

#[derive(JsonSchema, Deserialize, Serialize, Debug, Clone)]
pub struct Application {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub url: Url,
    pub categories: Option<Vec<String>>,
    pub openapi_endpoint: Option<Url>,
    pub repository: Option<Repository>,
}

#[derive(JsonSchema, Deserialize, Serialize, Debug, Clone)]
pub struct Command {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
}
