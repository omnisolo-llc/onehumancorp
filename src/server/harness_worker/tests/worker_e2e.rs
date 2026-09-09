use server_omnisolo::harness_middleware::harness_worker_service_client::HarnessWorkerServiceClient;
use server_omnisolo::harness_middleware::{
    AttemptCommandEnvelope, SessionOperationEnvelope, WorkerHealthRequest,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::process::{Child, Command};
use tokio::time::{Duration, sleep, timeout};
use tokio_stream::StreamExt;
use tonic::Request;
use uuid::Uuid;

async fn free_port() -> u16 {
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
        .await
        .unwrap();
    listener.local_addr().unwrap().port()
}

async fn wait_for_health(port: u16) {
    let client = reqwest::Client::new();
    timeout(Duration::from_secs(10), async {
        loop {
            if let Ok(response) = client
                .get(format!("http://127.0.0.1:{port}/readyz"))
                .send()
                .await
            {
                if response.status().is_success() {
                    return;
                }
            }
            sleep(Duration::from_millis(50)).await;
        }
    })
    .await
    .expect("worker readiness endpoint did not become available");
}

async fn stop(child: &mut Child) {
    #[cfg(unix)]
    if let Some(process_id) = child.id() {
        let status = Command::new("/bin/kill")
            .args(["-TERM", &process_id.to_string()])
            .status()
            .await
            .expect("send SIGTERM to harness worker");
        assert!(status.success());
    }
    #[cfg(not(unix))]
    let _ = child.start_kill();

    if timeout(Duration::from_secs(5), child.wait()).await.is_err() {
        let _ = child.kill().await;
        let _ = child.wait().await;
    }
}

#[tokio::test]
async fn worker_binary_loads_public_environment_and_redacts_preflight_failures() {
    let grpc_port = free_port().await;
    let health_port = free_port().await;
    let secret = "worker-main-secret-canary";
    let mut command = Command::new(env!("CARGO_BIN_EXE_omnisolo-harness-worker"));
    command
        .env_clear()
        .env("OMNISOLO_HARNESS_WORKER_ID", "worker-main-e2e")
        .env("OMNISOLO_HARNESS_ID", "codex")
        .env("OMNISOLO_HARNESS_POOL_ID", "codex")
        .env(
            "OMNISOLO_HARNESS_GRPC_ADDR",
            format!("127.0.0.1:{grpc_port}"),
        )
        .env(
            "OMNISOLO_HARNESS_HEALTH_ADDR",
            format!("127.0.0.1:{health_port}"),
        )
        .env("OMNISOLO_HARNESS_EXECUTABLE", "/definitely/missing/codex")
        .env("OMNISOLO_HARNESS_ARGS_JSON", "[]")
        .env("OMNISOLO_HARNESS_PROTOCOL", "codex_app_server")
        .env("OPENAI_API_KEY", secret)
        .env("OPENAI_API_BASE_URL", "https://llmapi.omnisolo.co/v1")
        .env("OPENAI_MODEL", "gpt-5.6-luna")
        .env("OPENAI_REASONING_EFFORT", "max")
        .env("RUST_LOG", "info");
    if let Ok(profile_file) = std::env::var("LLVM_PROFILE_FILE") {
        command.env("LLVM_PROFILE_FILE", profile_file);
    }
    let output = command.output().await.unwrap();
    assert!(!output.status.success());
    let captured = format!(
        "{}\n{}",
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap()
    );
    assert!(captured.contains("worker-main-e2e"));
    assert!(captured.contains("starting OmniSolo harness worker"));
    assert!(!captured.contains(secret));
}

async fn responses_provider() -> (
    std::net::SocketAddr,
    tokio::task::JoinHandle<serde_json::Value>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let task = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut request = Vec::new();
        let mut buffer = [0_u8; 4096];
        loop {
            let read = stream.read(&mut buffer).await.unwrap();
            assert!(read > 0);
            request.extend_from_slice(&buffer[..read]);
            let Some(headers_end) = request.windows(4).position(|part| part == b"\r\n\r\n") else {
                continue;
            };
            let headers = String::from_utf8_lossy(&request[..headers_end]);
            let content_length = headers
                .lines()
                .find_map(|line| {
                    line.to_ascii_lowercase()
                        .strip_prefix("content-length:")
                        .map(str::trim)
                        .and_then(|value| value.parse::<usize>().ok())
                })
                .unwrap();
            if request.len() >= headers_end + 4 + content_length {
                let body: serde_json::Value =
                    serde_json::from_slice(&request[headers_end + 4..]).unwrap();
                let response = serde_json::to_vec(&serde_json::json!({
                    "id":"resp_worker_omni_1",
                    "status":"completed",
                    "model":"gpt-5.6-luna",
                    "output":[{"type":"message","role":"assistant","content":[{
                        "type":"output_text","text":"worker-provider-ok"
                    }]}],
                    "usage":{"input_tokens":9,"output_tokens":3,"total_tokens":12}
                }))
                .unwrap();
                stream
                    .write_all(
                        format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                            response.len()
                        )
                        .as_bytes(),
                    )
                    .await
                    .unwrap();
                stream.write_all(&response).await.unwrap();
                return body;
            }
        }
    });
    (address, task)
}

async fn models_provider() -> (
    std::net::SocketAddr,
    tokio::task::JoinHandle<(String, String)>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let task = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut request = Vec::new();
        let mut buffer = [0_u8; 4096];
        loop {
            let read = stream.read(&mut buffer).await.unwrap();
            assert!(read > 0);
            request.extend_from_slice(&buffer[..read]);
            let Some(headers_end) = request.windows(4).position(|part| part == b"\r\n\r\n") else {
                continue;
            };
            let headers = String::from_utf8_lossy(&request[..headers_end]);
            let mut lines = headers.lines();
            let request_line = lines.next().unwrap_or_default().to_owned();
            let authorization = lines
                .find_map(|line| {
                    line.strip_prefix("Authorization:")
                        .or_else(|| line.strip_prefix("authorization:"))
                        .map(|value| value.trim().to_owned())
                })
                .unwrap_or_default();
            let response = serde_json::to_vec(&serde_json::json!({
                "object": "list",
                "data": [{"id": "gpt-5.6-luna", "object": "model"}]
            }))
            .unwrap();
            stream
                .write_all(
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        response.len()
                    )
                    .as_bytes(),
                )
                .await
                .unwrap();
            stream.write_all(&response).await.unwrap();
            return (request_line, authorization);
        }
    });
    (address, task)
}

#[tokio::test]
async fn worker_binary_routes_an_external_attempt_over_http_and_grpc() {
    let grpc_port = free_port().await;
    let health_port = free_port().await;
    let script = r#"
while IFS= read -r line; do
  request_id=$(printf '%s' "$line" | sed -n 's/.*"request_id":"\([^"]*\)".*/\1/p')
  operation=$(printf '%s' "$line" | sed -n 's/.*"operation":"\([^"]*\)".*/\1/p')
  if [ "$operation" = "execute" ]; then
    printf '{"request_id":"%s","ok":true,"payload":{"events":[{"event_type":"assistant.text","durable":true,"payload":{"text":"network-e2e"},"native_cursor":"cursor-1"}],"final_text":"network-e2e"}}\n' "$request_id"
  else
    printf '{"request_id":"%s","ok":true,"payload":{"native_session_id":"native-network-e2e","native_cursor":"cursor-1"}}\n' "$request_id"
  fi
done
"#;

    let mut child = Command::new(env!("CARGO_BIN_EXE_omnisolo-harness-worker"))
        .env("OMNISOLO_HARNESS_WORKER_ID", "worker-e2e")
        .env("OMNISOLO_HARNESS_ID", "codex")
        .env("OMNISOLO_HARNESS_POOL_ID", "codex")
        .env(
            "OMNISOLO_HARNESS_GRPC_ADDR",
            format!("127.0.0.1:{grpc_port}"),
        )
        .env(
            "OMNISOLO_HARNESS_HEALTH_ADDR",
            format!("127.0.0.1:{health_port}"),
        )
        .env("OMNISOLO_HARNESS_EXECUTABLE", "/bin/sh")
        .env(
            "OMNISOLO_HARNESS_ARGS_JSON",
            serde_json::to_string(&["-c", script]).unwrap(),
        )
        .env("OMNISOLO_HARNESS_PROTOCOL", "custom")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();

    wait_for_health(health_port).await;
    let endpoint = format!("http://127.0.0.1:{grpc_port}");
    let mut client = timeout(
        Duration::from_secs(10),
        HarnessWorkerServiceClient::connect(endpoint),
    )
    .await
    .expect("gRPC connection timed out")
    .unwrap();

    let health = client
        .health(Request::new(WorkerHealthRequest {
            protocol_version: 1,
            worker_id: "worker-e2e".to_owned(),
            harness_id: "codex".to_owned(),
            pool_id: "codex".to_owned(),
        }))
        .await
        .unwrap()
        .into_inner();
    assert!(health.ready);
    assert!(health.accepting_new_attempts);

    let session_id = Uuid::new_v4();
    let task_id = Uuid::new_v4();
    let attempt_id = Uuid::new_v4();
    let mut events = client
        .attempt_command(Request::new(AttemptCommandEnvelope {
            protocol_version: 1,
            tenant_id: "tenant-e2e".to_owned(),
            session_id: session_id.to_string(),
            task_id: task_id.to_string(),
            attempt_id: attempt_id.to_string(),
            turn_id: String::new(),
            command_id: Uuid::new_v4().to_string(),
            lease_id: Uuid::new_v4().to_string(),
            lease_generation: 1,
            fencing_token: "e2e-fence".to_owned(),
            kind: "execute".to_owned(),
            correlation_id: String::new(),
            idempotency_key: "e2e-command".to_owned(),
            payload_schema: "omnisolo.attempt.command.v1".to_owned(),
            payload_version: 1,
            payload: serde_json::to_vec(&serde_json::json!({"prompt": "hello"})).unwrap(),
            extensions: Default::default(),
            worker_id: "worker-e2e".to_owned(),
            harness_id: "codex".to_owned(),
            capability_version: 1,
            binding_id: String::new(),
            binding_generation: 0,
        }))
        .await
        .unwrap()
        .into_inner();
    let binding = events.next().await.unwrap().unwrap();
    assert_eq!(binding.session_id, session_id.to_string());
    assert_eq!(binding.payload_schema, "omnisolo.harness.event.v1");
    assert_eq!(binding.durable_sequence, 1);
    let binding_payload: serde_json::Value = serde_json::from_slice(&binding.payload).unwrap();
    assert_eq!(binding_payload["event_type"], "inference.model_binding");
    assert_eq!(binding_payload["payload"]["model_id"], "gpt-5.6-luna");
    assert_eq!(binding_payload["payload"]["reasoning_effort"], "max");

    let event = events.next().await.unwrap().unwrap();
    assert_eq!(event.durable_sequence, 2);
    assert!(
        String::from_utf8(event.payload)
            .unwrap()
            .contains("network-e2e")
    );
    let final_event = events.next().await.unwrap().unwrap();
    assert_eq!(final_event.durable_sequence, 3);
    assert!(
        String::from_utf8(final_event.payload)
            .unwrap()
            .contains("assistant.final")
    );
    assert!(events.next().await.is_none());

    stop(&mut child).await;
}

#[tokio::test]
async fn worker_binary_routes_native_process_through_provider_facade_without_leaking_upstream_credentials()
 {
    let grpc_port = free_port().await;
    let health_port = free_port().await;
    let (provider_address, provider) = models_provider().await;
    let secret = "worker-native-upstream-secret-canary";
    let upstream_base_url = format!("http://{provider_address}/v1");
    let script = r#"
while IFS= read -r line; do
  request_id=$(printf '%s' "$line" | sed -n 's/.*"request_id":"\([^"]*\)".*/\1/p')
  operation=$(printf '%s' "$line" | sed -n 's/.*"operation":"\([^"]*\)".*/\1/p')
  if [ "$operation" = "execute" ]; then
    models_status=$(/usr/bin/python3 -c 'import sys,urllib.request; request=urllib.request.Request(sys.argv[1] + "/models", headers={"Authorization":"Bearer " + sys.argv[2]}); print(urllib.request.urlopen(request, timeout=5).status)' "$OPENAI_API_BASE_URL" "$OPENAI_API_KEY" 2>/dev/null)
    case "$OPENAI_API_BASE_URL" in http://127.0.0.1:*) base_ok=1 ;; *) base_ok=0 ;; esac
    if [ "$OPENAI_API_KEY" = "worker-native-upstream-secret-canary" ]; then token_scoped=0; else token_scoped=1; fi
    printf '{"request_id":"%s","ok":true,"payload":{"events":[{"event_type":"assistant.text","durable":true,"payload":{"text":"child-facade-ok base_ok=%s token_scoped=%s models=%s"},"native_cursor":"cursor-1"}],"final_text":"child-facade-ok"}}\n' "$request_id" "$base_ok" "$token_scoped" "$models_status"
  else
    printf '{"request_id":"%s","ok":true,"payload":{"native_session_id":"native-facade-e2e","native_cursor":"cursor-1"}}\n' "$request_id"
  fi
done
"#;

    let mut child = Command::new(env!("CARGO_BIN_EXE_omnisolo-harness-worker"))
        .env("OMNISOLO_HARNESS_WORKER_ID", "worker-native-facade-e2e")
        .env("OMNISOLO_HARNESS_ID", "codex")
        .env("OMNISOLO_HARNESS_POOL_ID", "codex")
        .env(
            "OMNISOLO_HARNESS_GRPC_ADDR",
            format!("127.0.0.1:{grpc_port}"),
        )
        .env(
            "OMNISOLO_HARNESS_HEALTH_ADDR",
            format!("127.0.0.1:{health_port}"),
        )
        .env("OMNISOLO_HARNESS_EXECUTABLE", "/bin/sh")
        .env(
            "OMNISOLO_HARNESS_ARGS_JSON",
            serde_json::to_string(&["-c", script]).unwrap(),
        )
        .env("OMNISOLO_HARNESS_PROTOCOL", "custom")
        .env("OPENAI_API_KEY", secret)
        .env("OPENAI_API_BASE_URL", &upstream_base_url)
        .env("OPENAI_MODEL", "gpt-5.6-luna")
        .env("OPENAI_REASONING_EFFORT", "max")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();

    wait_for_health(health_port).await;
    let mut client = HarnessWorkerServiceClient::connect(format!("http://127.0.0.1:{grpc_port}"))
        .await
        .unwrap();
    let session_id = Uuid::new_v4();
    let task_id = Uuid::new_v4();
    client
        .session_operation(Request::new(SessionOperationEnvelope {
            protocol_version: 1,
            tenant_id: "tenant-native-facade-e2e".to_owned(),
            session_id: session_id.to_string(),
            operation_id: Uuid::new_v4().to_string(),
            operation_generation: 1,
            fencing_token: "native-facade-session-fence".to_owned(),
            kind: "create".to_owned(),
            task_id: task_id.to_string(),
            correlation_id: String::new(),
            idempotency_key: "native-facade-create".to_owned(),
            payload_schema: "omnisolo.session.create.v1".to_owned(),
            payload_version: 1,
            payload: serde_json::to_vec(&serde_json::json!({"objective":"native facade"})).unwrap(),
            extensions: Default::default(),
            worker_id: "worker-native-facade-e2e".to_owned(),
            pool_id: "codex".to_owned(),
            harness_id: "codex".to_owned(),
            capability_version: 1,
            binding_id: String::new(),
            binding_generation: 0,
            workspace_mutation_scope_id: "native-facade-workspace".to_owned(),
        }))
        .await
        .unwrap();

    let mut events = client
        .attempt_command(Request::new(AttemptCommandEnvelope {
            protocol_version: 1,
            tenant_id: "tenant-native-facade-e2e".to_owned(),
            session_id: session_id.to_string(),
            task_id: task_id.to_string(),
            attempt_id: Uuid::new_v4().to_string(),
            turn_id: String::new(),
            command_id: Uuid::new_v4().to_string(),
            lease_id: Uuid::new_v4().to_string(),
            lease_generation: 1,
            fencing_token: "native-facade-attempt-fence".to_owned(),
            kind: "execute".to_owned(),
            correlation_id: String::new(),
            idempotency_key: "native-facade-execute".to_owned(),
            payload_schema: "omnisolo.attempt.command.v1".to_owned(),
            payload_version: 1,
            payload: serde_json::to_vec(&serde_json::json!({"prompt":"facade check"})).unwrap(),
            extensions: Default::default(),
            worker_id: "worker-native-facade-e2e".to_owned(),
            harness_id: "codex".to_owned(),
            capability_version: 1,
            binding_id: String::new(),
            binding_generation: 0,
        }))
        .await
        .unwrap()
        .into_inner();
    let mut payloads = Vec::new();
    while let Some(event) = events.next().await {
        payloads.push(String::from_utf8(event.unwrap().payload).unwrap());
    }
    let wire = payloads.join("\n");
    assert!(wire.contains("child-facade-ok"));
    assert!(wire.contains("models=200"), "wire={wire}");
    assert!(wire.contains("base_ok=1"));
    assert!(wire.contains("token_scoped=1"));
    assert!(!wire.contains(&provider_address.to_string()));
    assert!(!wire.contains(secret));
    let (request_line, authorization) = provider.await.unwrap();
    assert_eq!(request_line, "GET /v1/models HTTP/1.1");
    assert_eq!(authorization, format!("Bearer {secret}"));
    stop(&mut child).await;
}

#[tokio::test]
async fn worker_binary_runs_the_first_party_omnisolo_adapter_over_grpc() {
    let grpc_port = free_port().await;
    let health_port = free_port().await;
    let mut child = Command::new(env!("CARGO_BIN_EXE_omnisolo-harness-worker"))
        .env("OMNISOLO_HARNESS_WORKER_ID", "worker-omni-e2e")
        .env("OMNISOLO_HARNESS_ID", "omnisolo")
        .env("OMNISOLO_HARNESS_POOL_ID", "omnisolo")
        .env(
            "OMNISOLO_HARNESS_GRPC_ADDR",
            format!("127.0.0.1:{grpc_port}"),
        )
        .env(
            "OMNISOLO_HARNESS_HEALTH_ADDR",
            format!("127.0.0.1:{health_port}"),
        )
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();

    wait_for_health(health_port).await;
    let endpoint = format!("http://127.0.0.1:{grpc_port}");
    let mut client = timeout(
        Duration::from_secs(10),
        HarnessWorkerServiceClient::connect(endpoint),
    )
    .await
    .expect("gRPC connection timed out")
    .unwrap();

    let session_id = Uuid::new_v4();
    let task_id = Uuid::new_v4();
    let operation = client
        .session_operation(Request::new(SessionOperationEnvelope {
            protocol_version: 1,
            tenant_id: "tenant-omni-e2e".to_owned(),
            session_id: session_id.to_string(),
            operation_id: Uuid::new_v4().to_string(),
            operation_generation: 1,
            fencing_token: "omni-operation-fence".to_owned(),
            kind: "create".to_owned(),
            task_id: task_id.to_string(),
            correlation_id: String::new(),
            idempotency_key: "omni-create-e2e".to_owned(),
            payload_schema: "omnisolo.session.create.v1".to_owned(),
            payload_version: 1,
            payload: serde_json::to_vec(&serde_json::json!({
                "objective": "run first-party binary"
            }))
            .unwrap(),
            extensions: Default::default(),
            worker_id: "worker-omni-e2e".to_owned(),
            pool_id: "omnisolo".to_owned(),
            harness_id: "omnisolo".to_owned(),
            capability_version: 1,
            binding_id: String::new(),
            binding_generation: 0,
            workspace_mutation_scope_id: "workspace-omni-e2e".to_owned(),
        }))
        .await
        .unwrap()
        .into_inner();
    assert!(operation.accepted);
    assert_eq!(
        operation.payload_schema,
        "omnisolo.harness.native_session.v1"
    );
    assert!(
        String::from_utf8(operation.payload)
            .unwrap()
            .contains(&format!("omnisolo:{session_id}"))
    );

    let mut events = client
        .attempt_command(Request::new(AttemptCommandEnvelope {
            protocol_version: 1,
            tenant_id: "tenant-omni-e2e".to_owned(),
            session_id: session_id.to_string(),
            task_id: task_id.to_string(),
            attempt_id: Uuid::new_v4().to_string(),
            turn_id: String::new(),
            command_id: Uuid::new_v4().to_string(),
            lease_id: Uuid::new_v4().to_string(),
            lease_generation: 1,
            fencing_token: "omni-attempt-fence".to_owned(),
            kind: "execute".to_owned(),
            correlation_id: String::new(),
            idempotency_key: "omni-execute-e2e".to_owned(),
            payload_schema: "omnisolo.attempt.command.v1".to_owned(),
            payload_version: 1,
            payload: serde_json::to_vec(&serde_json::json!({
                "prompt": "first-party binary output"
            }))
            .unwrap(),
            extensions: Default::default(),
            worker_id: "worker-omni-e2e".to_owned(),
            harness_id: "omnisolo".to_owned(),
            capability_version: 1,
            binding_id: String::new(),
            binding_generation: 0,
        }))
        .await
        .unwrap()
        .into_inner();
    let event = events.next().await.unwrap().unwrap();
    assert_eq!(event.payload_schema, "omnisolo.harness.event.v1");
    assert!(
        String::from_utf8(event.payload)
            .unwrap()
            .contains("first-party binary output")
    );
    assert!(events.next().await.is_none());

    stop(&mut child).await;
}

#[tokio::test]
async fn worker_binary_routes_omnisolo_through_the_configured_responses_provider() {
    let grpc_port = free_port().await;
    let health_port = free_port().await;
    let (provider_address, provider) = responses_provider().await;
    let secret = "worker-provider-secret-canary";
    let mut child = Command::new(env!("CARGO_BIN_EXE_omnisolo-harness-worker"))
        .env("OMNISOLO_HARNESS_WORKER_ID", "worker-omni-provider-e2e")
        .env("OMNISOLO_HARNESS_ID", "omnisolo")
        .env("OMNISOLO_HARNESS_POOL_ID", "omnisolo")
        .env(
            "OMNISOLO_HARNESS_GRPC_ADDR",
            format!("127.0.0.1:{grpc_port}"),
        )
        .env(
            "OMNISOLO_HARNESS_HEALTH_ADDR",
            format!("127.0.0.1:{health_port}"),
        )
        .env("OPENAI_API_KEY", secret)
        .env(
            "OPENAI_API_BASE_URL",
            format!("http://{provider_address}/v1"),
        )
        .env("OPENAI_MODEL", "gpt-5.6-luna")
        .env("OPENAI_REASONING_EFFORT", "max")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();

    wait_for_health(health_port).await;
    let mut client = HarnessWorkerServiceClient::connect(format!("http://127.0.0.1:{grpc_port}"))
        .await
        .unwrap();
    let session_id = Uuid::new_v4();
    let task_id = Uuid::new_v4();
    client
        .session_operation(Request::new(SessionOperationEnvelope {
            protocol_version: 1,
            tenant_id: "tenant-omni-provider-e2e".to_owned(),
            session_id: session_id.to_string(),
            operation_id: Uuid::new_v4().to_string(),
            operation_generation: 1,
            fencing_token: "provider-operation-fence".to_owned(),
            kind: "create".to_owned(),
            task_id: task_id.to_string(),
            correlation_id: String::new(),
            idempotency_key: "provider-create-e2e".to_owned(),
            payload_schema: "omnisolo.session.create.v1".to_owned(),
            payload_version: 1,
            payload: serde_json::to_vec(&serde_json::json!({"objective":"provider e2e"})).unwrap(),
            extensions: Default::default(),
            worker_id: "worker-omni-provider-e2e".to_owned(),
            pool_id: "omnisolo".to_owned(),
            harness_id: "omnisolo".to_owned(),
            capability_version: 1,
            binding_id: String::new(),
            binding_generation: 0,
            workspace_mutation_scope_id: "provider-workspace".to_owned(),
        }))
        .await
        .unwrap();

    let mut events = client
        .attempt_command(Request::new(AttemptCommandEnvelope {
            protocol_version: 1,
            tenant_id: "tenant-omni-provider-e2e".to_owned(),
            session_id: session_id.to_string(),
            task_id: task_id.to_string(),
            attempt_id: Uuid::new_v4().to_string(),
            turn_id: String::new(),
            command_id: Uuid::new_v4().to_string(),
            lease_id: Uuid::new_v4().to_string(),
            lease_generation: 1,
            fencing_token: "provider-attempt-fence".to_owned(),
            kind: "execute".to_owned(),
            correlation_id: String::new(),
            idempotency_key: "provider-execute-e2e".to_owned(),
            payload_schema: "omnisolo.attempt.command.v1".to_owned(),
            payload_version: 1,
            payload: serde_json::to_vec(&serde_json::json!({
                "prompt":"Return the worker provider marker only",
                "native_session_id":format!("omnisolo:{session_id}")
            }))
            .unwrap(),
            extensions: Default::default(),
            worker_id: "worker-omni-provider-e2e".to_owned(),
            harness_id: "omnisolo".to_owned(),
            capability_version: 1,
            binding_id: String::new(),
            binding_generation: 0,
        }))
        .await
        .unwrap()
        .into_inner();
    let mut payloads = Vec::new();
    while let Some(event) = events.next().await {
        payloads.push(String::from_utf8(event.unwrap().payload).unwrap());
    }
    let wire = payloads.join("\n");
    assert!(wire.contains("inference.model_binding"));
    assert!(wire.contains("worker-provider-ok"));
    assert!(wire.contains("assistant.final"));
    assert!(wire.contains("usage.recorded"));
    assert!(!wire.contains(secret));
    let provider_body = provider.await.unwrap();
    assert_eq!(provider_body["model"], "gpt-5.6-luna");
    assert_eq!(provider_body["reasoning"]["effort"], "max");
    assert!(!provider_body.to_string().contains(secret));
    stop(&mut child).await;
}
