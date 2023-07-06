use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use tracing::*;

#[derive(thiserror::Error, Debug)]
pub enum BaseError {
    #[error("Common error:")]
    BaseError(#[from] common::db::CommonError),
}
pub type Result<T> = std::result::Result<T, BaseError>;

#[instrument]
pub async fn get_services() -> Result<Json<Vec<common::models::Application>>> {
    let apps = common::db::get_applications().await?;
    Ok(Json(apps))
}

#[instrument]
pub async fn get_commands() -> Result<Json<Vec<common::models::Command>>> {
    let cmds = common::db::get_commnads().await?;
    Ok(Json(cmds))
}

impl IntoResponse for BaseError {
    fn into_response(self) -> axum::response::Response {
        let err_msg = format!("{}", self.to_string());
        let (status, body) = match self {
            BaseError::BaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::json!({ "error": err_msg, "message": format!("{:?}",msg) }),
            ),
        };
        (status, Json(body)).into_response()
    }
}

// Module: routes
pub fn router() -> Router {
    Router::new()
        .route("/services", get(get_services))
        .route("/commands", get(get_commands))
}
