use crate::models::{Application, Command};

#[derive(thiserror::Error, Debug)]
pub enum CommonError {
    #[error("k8s error")]
    ClientError(#[from] kube::Error),
}

pub type Result<T> = std::result::Result<T, CommonError>;

pub async fn get_applications(client: kube::Client) -> Result<Vec<Application>> {
    log::info!(" inside common get_applications");
    let api_apps: kube::Api<super::crd::Application> = kube::Api::namespaced(client, "hubify");
    let lp = kube::api::ListParams::default();
    let apps = api_apps.list(&lp).await?;
    Ok(apps.items.into_iter().map(|x| x.spec.into()).collect())
}

pub async fn get_commnads(client: kube::Client) -> Result<Vec<Command>> {
    log::info!(" inside common get_commnads");
    let api_cmds: kube::Api<super::crd::Command> = kube::Api::namespaced(client, "hubify");
    let lp = kube::api::ListParams::default();
    let cmds = api_cmds.list(&lp).await?;
    Ok(cmds.items.into_iter().map(|x| x.spec.into()).collect())
}
