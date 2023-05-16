use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use thiserror::Error;
use tracing::*;

#[derive(Error, Debug)]
pub enum K8SERROR {
    #[error("k8s error")]
    ClientError(#[from] kube::Error),
}
pub type Result<T> = std::result::Result<T, K8SERROR>;

#[instrument]
pub async fn get_services() -> Result<Json<Vec<common::crd::ApplicationSpec>>> {
    let client = kube::Client::try_default().await?;
    let api_apps: kube::Api<common::crd::Application> = kube::Api::namespaced(client, "hubify");
    let lp = kube::api::ListParams::default();
    let apps = api_apps.list(&lp).await?;
    Ok(Json(apps.items.into_iter().map(|x| x.spec).collect()))
}

#[instrument]
pub async fn get_commands() -> Result<Json<Vec<common::crd::CommandSpec>>> {
    let client = kube::Client::try_default().await?;
    let api_cmds: kube::Api<common::crd::Command> = kube::Api::namespaced(client, "hubify");
    let lp = kube::api::ListParams::default();
    let cmds = api_cmds.list(&lp).await?;
    Ok(Json(cmds.items.into_iter().map(|x| x.spec).collect()))
}

impl IntoResponse for K8SERROR {
    fn into_response(self) -> axum::response::Response {
        let err_msg = format!("{}", self.to_string());
        let (status, body) = match self {
            K8SERROR::ClientError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::json!({ "error": err_msg, "message": format!("{:?}",msg) }),
            ),
        };
        (status, Json(body)).into_response()
    }
}

// Module: routes
pub fn router() -> Router {
    Router::new().route("/", get(get_services))
}
