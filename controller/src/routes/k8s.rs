use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use k8s_openapi::api::core::v1::Service;
use kube::{
    api::ListParams,
    core::{ObjectList, PartialObjectMeta},
    Api, Client,
};
use thiserror::Error;
use tracing::*;

#[derive(Error, Debug)]
pub enum K8SERROR {
    #[error("k8s error")]
    ClientError(#[from] kube::Error),
}
pub type Result<T> = std::result::Result<T, K8SERROR>;

#[instrument]
pub async fn get_services() -> Result<Json<String>> {
    Ok(Json("Hello k8s".to_string()))
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
