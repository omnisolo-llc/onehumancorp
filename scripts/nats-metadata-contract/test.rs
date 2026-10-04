use crate::{MyIntegrationService, integrations};
use prost::Message;
use server_omnisolo::orchestration::{
    ConnectIntegrationRequest, GetIntegrationsRequest, GetIntegrationsResponse, IntegrationInstance,
};
use std::sync::Arc;

const SECRET: &str = "nats-metadata-canary-password";
const USER: &str = "nats-metadata-canary-user";

fn service() -> MyIntegrationService {
    MyIntegrationService {
        registry: Arc::new(integrations::registry::IntegrationsRegistry::default()),
    }
}

fn assert_instance(instance: &IntegrationInstance, expected: &str) {
    let json = serde_json::to_string(instance).unwrap();
    assert!(
        !json.contains(SECRET),
        "JSON response contains the password canary"
    );
    assert!(!json.contains(USER), "JSON response contains URL userinfo");
    let wire = instance.encode_to_vec();
    assert!(
        !wire
            .windows(SECRET.len())
            .any(|part| part == SECRET.as_bytes()),
        "protobuf response contains the password canary"
    );
    assert_eq!(
        IntegrationInstance::decode(wire.as_slice())
            .unwrap()
            .base_url,
        expected
    );
    assert_eq!(instance.base_url, expected);
    assert_eq!(instance.status, "configured");
}

#[tokio::test]
async fn connect_response_redacts_userinfo_in_json_and_protobuf() {
    let service = service();
    let raw = format!("nats://{USER}:{SECRET}@127.0.0.1:4222/team?mode=test#status");
    let result = service
        .connect_integration(tonic::Request::new(ConnectIntegrationRequest {
            integration_id: "nats".into(),
            base_url: raw,
            ..Default::default()
        }))
        .await
        .unwrap()
        .into_inner();
    assert_instance(&result, "nats://127.0.0.1:4222/team?mode=test#status");
}

#[tokio::test]
async fn list_and_category_responses_do_not_reexpose_stored_userinfo() {
    let service = service();
    service
        .connect_integration(tonic::Request::new(ConnectIntegrationRequest {
            integration_id: "nats".into(),
            base_url: format!("nats://{USER}:{SECRET}@127.0.0.1:4222"),
            ..Default::default()
        }))
        .await
        .unwrap();
    for category in ["", "event_mesh"] {
        let result = service
            .get_integrations(tonic::Request::new(GetIntegrationsRequest {
                category: category.into(),
            }))
            .await
            .unwrap()
            .into_inner();
        assert_eq!(result.instances.len(), 1);
        let json = serde_json::to_string(&result).unwrap();
        assert!(!json.contains(SECRET));
        let wire = result.encode_to_vec();
        let result = GetIntegrationsResponse::decode(wire.as_slice()).unwrap();
        assert_instance(&result.instances[0], "nats://127.0.0.1:4222");
    }
}

#[tokio::test]
async fn provider_metadata_redacts_userinfo() {
    let raw = format!("nats://{USER}:{SECRET}@127.0.0.1:4222");
    let provider = integrations::nats::provider::NatsProvider::with_client(
        Arc::new(integrations::nats::client::RealNatsClient::dummy()),
        &raw,
    )
    .into_integration_provider();
    let json = serde_json::json!({"base_url":provider.metadata.base_url});
    assert_eq!(json["base_url"], "nats://127.0.0.1:4222");
    assert!(!json.to_string().contains(SECRET));
}

#[tokio::test]
async fn nonsecret_endpoint_configuration_is_preserved() {
    for url in [
        "nats://127.0.0.1:4222",
        "tls://localhost:4222",
        "wss://localhost:8443/nats?mode=test",
        "127.0.0.1:4222",
        "nats://[::1]:4222",
        "nats://localhost/path@value",
    ] {
        let result = service()
            .connect_integration(tonic::Request::new(ConnectIntegrationRequest {
                integration_id: "nats".into(),
                base_url: url.into(),
                ..Default::default()
            }))
            .await
            .unwrap()
            .into_inner();
        assert_instance(&result, url);
    }
}

#[tokio::test]
async fn username_only_password_only_encoded_and_ipv6_userinfo_are_redacted() {
    for (raw, wanted) in [
        (
            format!("nats://{USER}@localhost:4222"),
            "nats://localhost:4222",
        ),
        (
            format!("nats://:{SECRET}@localhost:4222"),
            "nats://localhost:4222",
        ),
        (
            format!("nats://{USER}:{SECRET}%40value@[::1]:4222"),
            "nats://[::1]:4222",
        ),
        (
            format!("nats://%6eats-metadata-canary-user:{SECRET}%3Avalue@localhost:4222"),
            "nats://localhost:4222",
        ),
        (format!("{USER}:{SECRET}@127.0.0.1:4222"), "127.0.0.1:4222"),
        (
            format!("tls://{USER}:{SECRET}@localhost:4222"),
            "tls://localhost:4222",
        ),
        (
            format!("wss://{USER}:{SECRET}@localhost:8443/nats?mode=test#status"),
            "wss://localhost:8443/nats?mode=test#status",
        ),
    ] {
        let result = service()
            .connect_integration(tonic::Request::new(ConnectIntegrationRequest {
                integration_id: "nats".into(),
                base_url: raw,
                ..Default::default()
            }))
            .await
            .unwrap()
            .into_inner();
        assert_instance(&result, wanted);
    }
}

#[tokio::test]
async fn malformed_credential_url_does_not_leak_through_metadata_or_reject_configuration() {
    let raw = format!("nats://{USER}:{SECRET}@[invalid");
    let result = service()
        .connect_integration(tonic::Request::new(ConnectIntegrationRequest {
            integration_id: "nats".into(),
            base_url: raw,
            ..Default::default()
        }))
        .await
        .unwrap()
        .into_inner();
    assert_instance(&result, "[invalid NATS endpoint]");
}

#[tokio::test]
async fn provider_handshake_errors_and_logs_redact_configured_userinfo() {
    use std::time::Duration;
    const CHILD: &str = "OHC_NATS_METADATA_LOG_CHILD";
    if std::env::var_os(CHILD).is_some() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_ansi(false)
            .try_init();
        let unrelated = tokio::spawn(async {
            tokio::time::sleep(Duration::from_millis(50)).await;
            tracing::warn!("nats-metadata-concurrent-security-event");
        });
        let result =
            integrations::nats::provider::NatsProvider::new(&std::env::var("NATS_URL").unwrap())
                .await;
        unrelated.await.unwrap();
        let Err(error) = result else {
            panic!("owned fixture must reject the handshake")
        };
        assert!(
            !error.contains(SECRET),
            "provider error contains credential canary"
        );
        tracing::warn!("nats-metadata-safe-error-boundary");
        return;
    }
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    listener.set_nonblocking(true).unwrap();
    let fixture = std::thread::spawn(move || {
        use std::io::{Read, Write};
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    stream
                        .set_read_timeout(Some(Duration::from_secs(3)))
                        .unwrap();
                    stream.write_all(b"INFO {\"server_id\":\"owned-fixture\",\"version\":\"2.10.0\",\"proto\":1,\"host\":\"127.0.0.1\",\"port\":4222,\"auth_required\":true,\"headers\":true,\"max_payload\":1048576}\r\n").unwrap();
                    let mut data = Vec::new();
                    while !data.windows(6).any(|chunk| chunk == b"PING\r\n") {
                        let mut chunk = [0; 1024];
                        let n = stream.read(&mut chunk).unwrap();
                        assert!(n > 0);
                        data.extend_from_slice(&chunk[..n]);
                        assert!(data.len() < 8192);
                    }
                    assert!(String::from_utf8_lossy(&data).contains("CONNECT "));
                    std::thread::sleep(Duration::from_millis(200));
                    stream
                        .write_all(format!("-ERR '{SECRET}'\r\n").as_bytes())
                        .unwrap();
                    return;
                }
                Err(error)
                    if error.kind() == std::io::ErrorKind::WouldBlock
                        && std::time::Instant::now() < deadline =>
                {
                    std::thread::sleep(Duration::from_millis(5))
                }
                other => panic!("Owned NATS handshake fixture failed: {other:?}"),
            }
        }
    });
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "tests::provider_handshake_errors_and_logs_redact_configured_userinfo",
            "--nocapture",
        ])
        .env(CHILD, "1")
        .env("NATS_URL", format!("nats://{USER}:{SECRET}@{address}"))
        .output()
        .unwrap();
    fixture.join().unwrap();
    let log = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !log.contains(SECRET),
        "provider logs contain credential canary"
    );
    assert!(!log.contains(USER), "provider logs contain URL userinfo");
    assert!(output.status.success(), "{log}");
    assert!(log.contains("test result: ok. 1 passed; 0 failed;"));
    assert!(
        log.find("nats-metadata-concurrent-security-event").unwrap()
            < log.find("nats-metadata-safe-error-boundary").unwrap()
    );
}
