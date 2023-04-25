use axum::{routing::get, Json, Router};
use k8s_openapi::api::core::v1::{Pod, Service};
use kube::{Api, Client};

pub async fn get_services() -> Json<Vec<String>> {
    let client = Client::try_default().await.unwrap();
    let pods: Api<Service> = Api::default_namespaced(client);
    let pod = pods.get_metadata("name").await.unwrap();
    log::info!("{:?}", pod);

    Json(vec!["k8s".to_string()])
}

// Module: routes
pub fn router() -> Router {
    Router::new().route("/", get(get_services))
}
