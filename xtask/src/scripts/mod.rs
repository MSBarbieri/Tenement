use anyhow::Result;
use clap::Subcommand;
use std::{io::Write, process::Command};

fn kind_config(registry_host: &str, registry_port: &str) -> String {
    format!(
        "kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
containerdConfigPatches:
- |-
  [plugins.\"io.containerd.grpc.v1.cri\".registry.mirrors.\"localhost:{registry_port}\"]
    endpoint = [\"http://{registry_host}:5000\"]"
    )
}
fn generate_config_map(registry_port: &str) -> String {
    format!(
        "apiVersion: v1
kind: ConfigMap
metadata:
  name: local-registry-hosting
  namespace: kube-public
data:
  localRegistryHosting.v1: |
    host: \"localhost:{}\"
    help: \"https://kind.sigs.k8s.io/docs/user/local-registry/\"",
        registry_port
    )
}

#[derive(Debug, Subcommand)]
pub enum Scripts {
    #[clap(
        name = "create-cluster",
        about = "Create kind cluster,and container registry"
    )]
    CreateCluster {
        #[clap(long, short, default_value = "tenement")]
        name: String,
        #[clap(long, default_value = "registry")]
        registry_name: String,
        #[clap(long, default_value = "5000")]
        registry_port: String,
    },
}

pub fn create_cluster(name: &str, registry_name: &str, registry_port: &str) -> Result<()> {
    if !crate::utils::has_registry_name(registry_name)? {
        log::info!("registry not found, creating one");
        let mut registry = Command::new("docker")
            .arg("run")
            .arg("-d")
            .arg("-p")
            .arg(format!("{}:5000", registry_port))
            .arg("--restart=always")
            .arg(format!("--name={}", registry_name))
            .arg("registry:2")
            .spawn()
            .expect("failed to execute child");
        registry.wait()?;
    }
    if crate::utils::has_cluster(name)? {
        log::warn!("kind cluster already exists");
        return Ok(());
    }
    let mut kind = Command::new("kind")
        .arg("create")
        .arg("cluster")
        .arg("--name")
        .arg(name)
        .arg("--config=-")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    let registry_host = registry_name;

    kind.stdin
        .as_mut()
        .expect("Failed to open stdin")
        .write_all(kind_config(registry_host, registry_port).as_bytes())?;

    kind.wait()?;

    log::info!("kind cluster created, applying config map for local registry");
    let mut config_map = Command::new("kubectl")
        .arg("apply")
        .arg("-f")
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    config_map
        .stdin
        .as_mut()
        .expect("Failed to open stdin")
        .write_all(generate_config_map(registry_port).as_bytes())?;
    config_map.wait()?;

    let mut connect = Command::new("docker")
        .arg("network")
        .arg("connect")
        .arg("kind")
        .arg(registry_name)
        .spawn()
        .expect("failed to execute child");
    connect.wait()?;

    Ok(())
}

pub fn parse(args: Scripts) -> Result<()> {
    match args {
        Scripts::CreateCluster {
            name,
            registry_name,
            registry_port,
        } => create_cluster(&name, &registry_name, &registry_port)?,
    }

    Ok(())
}
