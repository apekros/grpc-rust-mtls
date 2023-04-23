use crate::start_service;
use tokio::task;

use tonic_health::pb::health_client::HealthClient;
use tonic_health::pb::HealthCheckRequest;

#[tokio::test]
async fn test_health_service() {
    task::spawn(start_service());

    let conn = tonic::transport::Endpoint::from_static("http://localhost:8080")
        .connect()
        .await
        .unwrap();

    let mut client = HealthClient::new(conn);

    let request = HealthCheckRequest {
        service: "".to_string(),
    };

    client.check(request).await.unwrap();
}
