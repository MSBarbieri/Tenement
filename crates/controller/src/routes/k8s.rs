use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use tracing::*;

#[derive(thiserror::Error, Debug)]
pub enum K8SERROR {
    #[error("k8s error")]
    ClientError(#[from] kube::Error),
    #[error("Common error: {0}")]
    CommonError(#[from] hb_common::k8s::methods::CommonError),
}
pub type Result<T> = std::result::Result<T, K8SERROR>;

#[instrument]
pub async fn get_services() -> Result<Json<Vec<hb_common::models::Application>>> {
    let client = kube::Client::try_default().await?;
    let apps = hb_common::k8s::methods::get_applications(client).await?;
    Ok(Json(apps))
}

#[instrument]
pub async fn get_commands() -> Result<Json<Vec<hb_common::models::Command>>> {
    let client = kube::Client::try_default().await?;
    let commands = hb_common::k8s::methods::get_commnads(client).await?;
    Ok(Json(commands))
}

impl IntoResponse for K8SERROR {
    fn into_response(self) -> axum::response::Response {
        let err_msg = format!("{}", self.to_string());
        let (status, body) = match self {
            K8SERROR::ClientError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::json!({ "error": err_msg, "message": format!("{:?}",msg) }),
            ),
            K8SERROR::CommonError(msg) => (
                StatusCode::from_u16(400).unwrap(),
                serde_json::json!({ "error": err_msg, "message": format!("{:?}",msg) }),
            ),
        };
        (status, Json(body)).into_response()
    }
}

// Module: routes
pub fn router() -> Router {
    Router::new()
        .route("/", get(get_services))
        .route("/commands", get(get_commands))
}
