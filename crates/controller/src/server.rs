use crate::{config::Config, routes::configure_routes};
use axum::Router;
use std::net::SocketAddr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Port Used")]
    ConnnectionError,
    #[error("Database Not Found")]
    DatabaseNotFound,
    #[error("Cache Server not Found")]
    CacheDatabesNotFound,
    #[error("Server creation Error")]
    AxumError(#[from] hyper::Error),
    #[error("k8s error")]
    ClientError(#[from] kube::Error),
    #[error("Unknown Start Server Error")]
    Unknown,
}

pub fn set_layers(router: Router) -> Router {
    use axum_tracing_opentelemetry::{opentelemetry_tracing_layer, response_with_trace_layer};
    router
        .layer(response_with_trace_layer())
        .layer(opentelemetry_tracing_layer())
}

pub(crate) async fn create_server(config: Config) -> Result<(), ServerError> {
    let addr: SocketAddr = config.address.parse().unwrap();
    let mut router = configure_routes();
    router = set_layers(router);

    tracing::info!("Server Started with address: {:?}", config.address.clone());
    let server = axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install CTRL+C signal handler");
        });
    tokio::select! {
        _ = server => tracing::info!("Server closed"),
    }
    Ok(())
}
