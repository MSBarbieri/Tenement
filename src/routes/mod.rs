use axum::{response::IntoResponse, routing::get, Router};

pub mod k8s;

async fn root() -> impl IntoResponse {
    "Hello, World!"
}

async fn health_check() -> impl IntoResponse {
    "OK"
}

pub fn configure_routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .nest("/k8s", crate::routes::k8s::router())
}
