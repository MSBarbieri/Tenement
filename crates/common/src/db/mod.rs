use url::Url;

use crate::models::{Application, Command};

#[derive(thiserror::Error, Debug)]
pub enum CommonError {
    #[error("Base error")]
    ClientError(String),
}

pub type Result<T> = std::result::Result<T, CommonError>;

pub async fn get_applications() -> Result<Vec<Application>> {
    log::info!("inside common get_applications");
    let app = Application {
        url: Url::parse("http://localhost:8080").unwrap(),
        name: "".to_string(),
        version: None,
        description: None,
        icon: None,
        categories: None,
        openapi_endpoint: None,
        repository: None,
    };

    Ok(vec![app])
}

pub async fn get_commnads() -> Result<Vec<Command>> {
    log::info!(" inside common get_commnads");
    let command = Command {
        name: "".to_string(),
        version: Some("".to_string()),
        description: None,
    };

    Ok(vec![command])
}

#[cfg(test)]
mod test {
    use hb_prisma::PrismaClient;
    use prisma_client_rust::{NewClientError, PrismaClient};

    #[tokio::test]
    async fn test_get_applications() {
        let client: PrismaClient = PrismaClient::_builder()
            .build()
            .await
            .expect("failed to create client");
        let apps = client.application().find_many(vec![]).exec().await;
        println!("{:?}", apps);
        // .expect("getting applications");
    }
}
