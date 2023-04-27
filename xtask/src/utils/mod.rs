use anyhow::Result;
use std::{io::BufRead, process::Command};

pub fn has_cluster(name: &str) -> Result<bool> {
    Ok(Command::new("kind")
        .arg("get")
        .arg("clusters")
        .output()?
        .stdout
        .lines()
        .filter_map(|line| line.ok())
        .any(|line| line == name))
}
pub fn has_registry_name(registry_name: &str) -> Result<bool> {
    Ok(Command::new("docker")
        .arg("ps")
        .arg("-f")
        .arg(format!("name={}", registry_name))
        .arg("--format")
        .arg("{{.Names}}")
        .output()?
        .stdout
        .lines()
        .filter_map(|line| line.ok())
        .any(|line| line == registry_name))
}
