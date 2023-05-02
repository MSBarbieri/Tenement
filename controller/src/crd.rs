use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, JsonSchema, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(test, derive(Default))]
#[kube(
    kind = "Application",
    group = "applications.hubify.io",
    version = "v1",
    namespaced,
    shortname = "app"
)]
#[kube(status = "ApplicationStatus")]
pub struct ApplicationSpec {
    name: String,
    version: Option<String>,
    description: Option<String>,
    icon: Option<String>,
    url: String,
    categories: Option<Vec<String>>,
}

#[derive(JsonSchema, Deserialize, Serialize, Debug, Clone, Default)]
pub struct ApplicationStatus {
    status: String,
}
