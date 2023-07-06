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
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
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

impl Into<crate::models::Repository> for Repository {
    fn into(self) -> crate::models::Repository {
        crate::models::Repository {
            name: self.name,
            url: self.url,
            access_token: self.access_token,
            docs_url: self.docs_url,
            docs_dir: self.docs_dir,
        }
    }
}

impl Into<crate::models::Application> for ApplicationSpec {
    fn into(self) -> crate::models::Application {
        crate::models::Application {
            name: self.name,
            version: self.version,
            description: self.description,
            icon: self.icon,
            url: self.url,
            categories: self.categories,
            openapi_endpoint: self.openapi_endpoint,
            repository: self.repository.map(|x| x.into()),
        }
    }
}
impl Into<crate::models::Command> for CommandSpec {
    fn into(self) -> crate::models::Command {
        crate::models::Command {
            name: self.name,
            version: self.version,
            description: self.description,
        }
    }
}
