use crate::start_service;
use tokio::task;
use tonic::transport::{Certificate, ClientTlsConfig};
use tonic_health::pb::health_client::HealthClient;
use tonic_health::pb::HealthCheckRequest;

#[tokio::test]
async fn test_health_service() {
    task::spawn(start_service());

    let data_dir = std::path::PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), "tls"]);
    let server_root_ca_cert = std::fs::read_to_string(data_dir.join("ca.pem")).unwrap();
    let server_root_ca_cert = Certificate::from_pem(server_root_ca_cert);

    let tls = ClientTlsConfig::new()
        .domain_name("localhost")
        .ca_certificate(server_root_ca_cert);

    let conn = tonic::transport::Endpoint::from_static("https://localhost:8080")
        .tls_config(tls)
        .unwrap()
        .connect()
        .await
        .unwrap();

    let mut client = HealthClient::new(conn);

    let request = HealthCheckRequest {
        service: "".to_string(),
    };

    client.check(request).await.unwrap();
}
