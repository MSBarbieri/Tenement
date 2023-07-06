use std::collections::BTreeMap;

use anyhow::Result;
use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentSpec},
        core::v1::{
            Container, ContainerPort, PodSpec, PodTemplateSpec, Service, ServiceAccount,
            ServicePort, ServiceSpec,
        },
        rbac::v1::{ClusterRole, ClusterRoleBinding, PolicyRule, RoleRef, Subject},
    },
    apimachinery::pkg::{apis::meta::v1::LabelSelector, util::intstr::IntOrString},
};
use kube::{core::ObjectMeta, CustomResourceExt};
use serde_yaml::Value;

#[derive(Debug, Clone)]
pub(crate) struct Settings {
    namespace: String,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            namespace: "hubifier".to_string(),
        }
    }
}

/// Generate all Custom Resource definitions, and bindings for manage this resources
fn generate_crds(settings: &Settings, values: &mut Vec<Value>) -> Result<()> {
    {
        // Service Account
        let meta = ObjectMeta {
            namespace: Some(settings.namespace.clone()),
            name: Some("hubifier".to_string()),
            ..Default::default()
        };
        let sa = ServiceAccount {
            automount_service_account_token: Some(true),
            image_pull_secrets: None,
            secrets: None,
            metadata: meta,
        };
        values.push(serde_yaml::to_value(&sa)?);
    }
    {
        // ClusterRole
        let meta = ObjectMeta {
            name: Some("hubifier".to_string()),
            ..Default::default()
        };
        let role = ClusterRole {
            metadata: meta,
            aggregation_rule: None,
            rules: Some(vec![PolicyRule {
                api_groups: Some(vec!["".to_string(), "hubify.io".to_string()]),
                verbs: vec!["get".to_string(), "watch".to_string(), "list".to_string()],
                non_resource_urls: None,
                resource_names: None,
                resources: Some(vec![
                    "deployments".to_string(),
                    "services".to_string(),
                    "applications".to_string(),
                    "commands".to_string(),
                ]),
            }]),
        };
        values.push(serde_yaml::to_value(&role)?);
    }
    {
        // ClusterRoleBinding
        let meta = ObjectMeta {
            name: Some("hubifier".to_string()),
            ..Default::default()
        };
        let crb = ClusterRoleBinding {
            metadata: meta,
            role_ref: RoleRef {
                api_group: "rbac.authorization.k8s.io".to_string(),
                kind: "ClusterRole".to_string(),
                name: "hubifier".to_string(),
            },
            subjects: Some(vec![Subject {
                api_group: None,
                kind: "ServiceAccount".to_string(),
                name: "hubifier".to_string(),
                namespace: Some(settings.namespace.clone()),
            }]),
        };
        values.push(serde_yaml::to_value(&crb)?);
    }
    {
        // Custom Resources
        let mut app = common::k8s::crd::Application::crd();
        app.metadata.namespace = Some(settings.namespace.clone());
        values.push(serde_yaml::to_value(&app)?);
        let mut cmds = common::k8s::crd::Command::crd();
        cmds.metadata.namespace = Some(settings.namespace.clone());
        values.push(serde_yaml::to_value(&cmds)?);
    }
    Ok(())
}

/// Generate services to map ports of deployments
fn generate_services(settings: &Settings, values: &mut Vec<Value>) -> Result<()> {
    {
        // Service (Controller)
        let name = "hubifier-controller".to_string();
        let meta = ObjectMeta {
            namespace: Some(settings.namespace.clone()),
            name: Some(name.clone()),
            ..Default::default()
        };
        let s_controller = Service {
            metadata: meta,
            spec: Some(ServiceSpec {
                ports: Some(vec![ServicePort {
                    app_protocol: None,
                    name: Some("http".to_string()),
                    node_port: None,
                    port: 3000,
                    protocol: Some("TCP".to_string()),
                    target_port: Some(IntOrString::Int(3000)),
                }]),
                selector: Some({
                    let mut selector = BTreeMap::new();
                    selector.insert("app".to_string(), name);
                    selector
                }),
                ..Default::default()
            }),
            status: None,
        };
        values.push(serde_yaml::to_value(&s_controller)?);
    }
    {
        // Service (Web)
        let name = "hubifier-web".to_string();
        let meta = ObjectMeta {
            namespace: Some(settings.namespace.clone()),
            name: Some(name.clone()),
            ..Default::default()
        };
        let s_web = Service {
            metadata: meta,
            spec: Some(ServiceSpec {
                ports: Some(vec![ServicePort {
                    app_protocol: None,
                    name: Some("http".to_string()),
                    node_port: None,
                    port: 80,
                    protocol: Some("TCP".to_string()),
                    target_port: Some(IntOrString::Int(80)),
                }]),
                selector: Some({
                    let mut selector = BTreeMap::new();
                    selector.insert("app".to_string(), name);
                    selector
                }),
                ..ServiceSpec::default()
            }),
            status: None,
        };
        values.push(serde_yaml::to_value(&s_web)?);
    }
    Ok(())
}

/// Deployments
fn generate_deployment(settings: &Settings, values: &mut Vec<Value>) -> Result<()> {
    {
        // Deployment (Controller)
        let name = "hubifier-controller".to_string();
        let meta = ObjectMeta {
            namespace: Some(settings.namespace.clone()),
            name: Some(name.clone()),
            ..Default::default()
        };
        let mut selector = BTreeMap::new();
        selector.insert("app".to_string(), name.clone());
        let deployment_controller = Deployment {
            metadata: meta,
            spec: Some(DeploymentSpec {
                replicas: Some(1),
                selector: LabelSelector {
                    match_expressions: None,
                    match_labels: Some(selector),
                },
                template: PodTemplateSpec {
                    metadata: Some(ObjectMeta {
                        labels: Some({
                            let mut tree = BTreeMap::new();
                            tree.insert("app".to_string(), name.clone());
                            tree
                        }),
                        ..Default::default()
                    }),
                    spec: Some(PodSpec {
                        service_account_name: Some("hubifier-sa".to_string()),
                        containers: vec![Container {
                            name,
                            image: Some("hubifier.io/hubifier-controller".to_string()),
                            ports: Some(vec![ContainerPort {
                                container_port: 3000,
                                host_ip: None,
                                host_port: None,
                                name: Some("http".to_string()),
                                protocol: Some("TCP".to_string()),
                            }]),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                },
                ..Default::default()
            }),
            status: None,
        };
        values.push(serde_yaml::to_value(&deployment_controller)?);
    }
    {
        // Deloyment (Web)
        let name = "hubifier-web".to_string();
        let meta = ObjectMeta {
            namespace: Some(settings.namespace.clone()),
            name: Some(name.clone()),
            ..Default::default()
        };
        let mut selector = BTreeMap::new();
        selector.insert("app".to_string(), name.clone());
        let deployment_web = Deployment {
            metadata: meta,
            spec: Some(DeploymentSpec {
                replicas: Some(1),
                selector: LabelSelector {
                    match_expressions: None,
                    match_labels: Some(selector),
                },
                template: PodTemplateSpec {
                    metadata: Some({
                        let mut metadata = ObjectMeta::default();
                        let mut selector = BTreeMap::new();
                        selector.insert("app".to_string(), name.clone());
                        metadata.labels = Some(selector);
                        metadata
                    }),
                    spec: Some(PodSpec {
                        containers: vec![Container {
                            name,
                            image: Some("hubifier.io/hubifier-web".to_string()),
                            ports: Some(vec![ContainerPort {
                                container_port: 80,
                                host_ip: None,
                                host_port: None,
                                name: Some("http".to_string()),
                                protocol: Some("TCP".to_string()),
                            }]),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                },
                ..Default::default()
            }),
            status: None,
        };
        values.push(serde_yaml::to_value(&deployment_web)?);
    }
    Ok(())
}

pub(crate) fn generate_resources(settings: Option<Settings>) -> Result<String> {
    let mut values = vec![];
    let settings = settings.unwrap_or_default();
    generate_crds(&settings, &mut values)?;
    generate_services(&settings, &mut values)?;
    generate_deployment(&settings, &mut values)?;
    Ok(values.iter_mut().fold(String::default(), |mut acc, t| {
        acc.push_str("---\n");
        acc.push_str(&serde_yaml::to_string(t).expect("failed to parse to yaml"));
        acc
    }))
}
