use axum::{response::IntoResponse, routing::get, Router};

#[cfg(feature = "db")]
mod base;

#[cfg(feature = "k8s")]
mod k8s;

async fn root() -> impl IntoResponse {
    "Hello, World!"
}

async fn health_check() -> impl IntoResponse {
    "OK"
}

pub fn configure_routes() -> Router {
    let routes = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check));

    cfg_if::cfg_if! {
        if #[cfg(feature = "k8s")] {
            routes.clone().nest("/api", k8s::router())
        } else {
            routes.clone().nest("/api", base::router())
        }
    }
}
