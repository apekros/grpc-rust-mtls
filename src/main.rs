#[cfg(test)]
mod health_tests;

use example::{
    example_server::{Example, ExampleServer},
    HelloRequest, HelloResponse,
};

use tonic::transport::{Identity, ServerTlsConfig};
use tonic::{transport::Server, Request, Response, Status};

pub mod example {
    tonic::include_proto!("example");
}

#[derive(Debug, Default)]
pub struct ExampleService {}

#[tonic::async_trait]
impl Example for ExampleService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let r = request.into_inner();
        Ok(Response::new(example::HelloResponse {
            message: { format!("Hello {}", r.name) },
        }))
    }
}

pub async fn start_service() {
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();

    health_reporter
        .set_serving::<ExampleServer<ExampleService>>()
        .await;

    let address = "[::1]:8080".parse().unwrap();
    let example_service = ExampleService::default();

    let data_dir = std::path::PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), "tls"]);
    let cert = std::fs::read_to_string(data_dir.join("server.pem")).unwrap();
    let key = std::fs::read_to_string(data_dir.join("server.key")).unwrap();
    let server_identity = Identity::from_pem(cert, key);

    let tls = ServerTlsConfig::new().identity(server_identity);

    Server::builder()
        .tls_config(tls)
        .unwrap()
        .add_service(ExampleServer::new(example_service))
        .add_service(health_service)
        .serve(address)
        .await
        .expect("Start service failed.");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_service().await;
    Ok(())
}
