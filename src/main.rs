use example::{
    example_server::{Example, ExampleServer},
    HelloRequest, HelloResponse,
};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();

    health_reporter
        .set_serving::<ExampleServer<ExampleService>>()
        .await;

    let address = "[::1]:8080".parse().unwrap();
    let example_service = ExampleService::default();

    Server::builder()
        .add_service(ExampleServer::new(example_service))
        .add_service(health_service)
        .serve(address)
        .await?;
    Ok(())
}
