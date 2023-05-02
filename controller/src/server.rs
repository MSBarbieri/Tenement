use crate::routes::configure_routes;
use axum::Router;
use futures::StreamExt;
use k8s_openapi::api::apps::v1::Deployment;
use kube::{
    runtime::{reflector, watcher, WatchStreamExt},
    Api, Client, ResourceExt,
};
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

use axum::Extension;
#[cfg(feature = "trace")]
pub fn set_layers(router: Router) -> Router {
    use axum_tracing_opentelemetry::{opentelemetry_tracing_layer, response_with_trace_layer};
    router
        .layer(response_with_trace_layer())
        .layer(opentelemetry_tracing_layer())
}

#[cfg(not(feature = "trace"))]
pub fn set_layers(router: Router) -> Router {
    router
}

pub async fn create_server(cli: crate::cli::Cli) -> Result<(), ServerError> {
    let client = Client::try_default().await?;
    let api: Api<Deployment> = Api::all(client);

    let (reader, writer) = reflector::store();
    let watch = reflector(writer, watcher(api, Default::default()))
        .backoff(backoff::ExponentialBackoff::default())
        .touched_objects()
        .filter_map(|x| async move { Result::ok(x) })
        .for_each(|o| async move {
            let name = o.name_any();
            let namespace = o.namespace().unwrap();
            tracing::info!("{} in namespace {}", name, namespace);
        });

    let addr: SocketAddr = cli.address.parse().unwrap();
    let mut router = configure_routes();
    router = set_layers(router);
    router = router.layer(Extension(reader.clone()));

    tracing::info!("Server Started with address: {:?}", cli.address.clone());
    let server = axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install CTRL+C signal handler");
        });
    tokio::select! {
        _ = watch => tracing::warn!("Watch stream closed"),
        _ = server => tracing::info!("Server closed"),
    }
    Ok(())
}
