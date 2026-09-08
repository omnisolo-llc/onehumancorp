use std::process::Stdio;

use server_omnisolo::harness_middleware::harness_worker_service_client::HarnessWorkerServiceClient;
use server_omnisolo::harness_middleware::{AttemptCommandEnvelope, SessionOperationEnvelope};
use tokio::process::{Child, Command};
use tokio::time::{Duration, sleep, timeout};
use tokio_stream::StreamExt;
use tonic::Request;
use uuid::Uuid;

async fn free_port() -> Result<u16, String> {
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
        .await
        .map_err(|error| error.to_string())?;
    listener
        .local_addr()
        .map(|address| address.port())
        .map_err(|error| error.to_string())
}

async fn wait_for_health(port: u16) -> Result<(), String> {
    let client = reqwest::Client::new();
    timeout(Duration::from_secs(30), async {
        loop {
            if let Ok(response) = client
                .get(format!("http://127.0.0.1:{port}/readyz"))
                .send()
                .await
            {
                if response.status().is_success() {
                    return Ok(());
                }
            }
            sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .map_err(|_| "worker readiness endpoint did not become available".to_owned())?
}

async fn stop(child: &mut Child) {
    let _ = child.kill().await;
    let _ = child.wait().await;
}

#[tokio::test]
#[ignore = "requires OMNISOLO_LIVE_CODEX_E2E=1 and Codex CLI authentication"]
async fn live_codex_app_server_worker_e2e() {
    if std::env::var("OMNISOLO_LIVE_CODEX_E2E").as_deref() != Ok("1") {
        return;
    }

    let result = run_live_codex_app_server_e2e().await;
    if let Err(error) = result {
        panic!("live Codex app-server E2E failed: {error}");
    }
}

async fn run_live_codex_app_server_e2e() -> Result<(), String> {
    Command::new("codex")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .map_err(|error| format!("Codex CLI is unavailable: {error}"))?
        .success()
        .then_some(())
        .ok_or_else(|| "Codex CLI version check failed".to_owned())?;

    let grpc_port = free_port().await?;
    let health_port = free_port().await?;
    let mut child = Command::new(env!("CARGO_BIN_EXE_omnisolo-harness-worker"))
        .env("OMNISOLO_HARNESS_WORKER_ID", "worker-live-codex")
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
        .env("OMNISOLO_HARNESS_EXECUTABLE", "codex")
        .env(
            "OMNISOLO_HARNESS_ARGS_JSON",
            serde_json::to_string(&["app-server", "--stdio"]).map_err(|error| error.to_string())?,
        )
        .env("OMNISOLO_HARNESS_PROTOCOL", "codex_app_server")
        .env("OMNISOLO_HARNESS_REQUEST_TIMEOUT_SECS", "180")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| format!("failed to start worker binary: {error}"))?;

    let result = async {
        wait_for_health(health_port).await?;
        let endpoint = format!("http://127.0.0.1:{grpc_port}");
        let mut client = timeout(
            Duration::from_secs(30),
            HarnessWorkerServiceClient::connect(endpoint),
        )
        .await
        .map_err(|_| "gRPC connection timed out".to_owned())?
        .map_err(|error| error.to_string())?;

        let session_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();
        let create = client
            .session_operation(Request::new(SessionOperationEnvelope {
                protocol_version: 1,
                tenant_id: "tenant-live-codex".to_owned(),
                session_id: session_id.to_string(),
                operation_id: Uuid::new_v4().to_string(),
                operation_generation: 1,
                fencing_token: "live-session-fence".to_owned(),
                kind: "create".to_owned(),
                task_id: task_id.to_string(),
                correlation_id: String::new(),
                idempotency_key: "live-session-create".to_owned(),
                payload_schema: "omnisolo.session.create.v1".to_owned(),
                payload_version: 1,
                payload: serde_json::to_vec(&serde_json::json!({
                    "objective": "verify the native Codex app-server worker"
                }))
                .map_err(|error| error.to_string())?,
                extensions: Default::default(),
                worker_id: "worker-live-codex".to_owned(),
                pool_id: "codex".to_owned(),
                harness_id: "codex".to_owned(),
                capability_version: 1,
                binding_id: String::new(),
                binding_generation: 0,
                workspace_mutation_scope_id: "workspace-live-codex".to_owned(),
            }))
            .await
            .map_err(|error| error.to_string())?
            .into_inner();
        if !create.accepted {
            return Err("native Codex session was not accepted".to_owned());
        }
        let native: server_harness::middleware::harness::NativeSession =
            serde_json::from_slice(&create.payload).map_err(|error| error.to_string())?;
        if native.native_session_id.trim().is_empty() {
            return Err("native Codex session id was empty".to_owned());
        }

        let attempt_id = Uuid::new_v4();
        let mut events = client
            .attempt_command(Request::new(AttemptCommandEnvelope {
                protocol_version: 1,
                tenant_id: "tenant-live-codex".to_owned(),
                session_id: session_id.to_string(),
                task_id: task_id.to_string(),
                attempt_id: attempt_id.to_string(),
                turn_id: String::new(),
                command_id: Uuid::new_v4().to_string(),
                lease_id: Uuid::new_v4().to_string(),
                lease_generation: 1,
                fencing_token: "live-attempt-fence".to_owned(),
                kind: "execute".to_owned(),
                correlation_id: String::new(),
                idempotency_key: "live-attempt-execute".to_owned(),
                payload_schema: "omnisolo.attempt.command.v1".to_owned(),
                payload_version: 1,
                payload: serde_json::to_vec(&serde_json::json!({
                    "prompt": "Reply with the marker OMNISOLO_LIVE_E2E_OK. Include that exact marker in your response.",
                    "native_session_id": native.native_session_id.clone(),
                }))
                .map_err(|error| error.to_string())?,
                extensions: Default::default(),
                worker_id: "worker-live-codex".to_owned(),
                harness_id: "codex".to_owned(),
                capability_version: 1,
                binding_id: String::new(),
                binding_generation: 0,
            }))
            .await
            .map_err(|error| error.to_string())?
            .into_inner();

        let mut saw_marker = false;
        while let Some(event) = timeout(Duration::from_secs(180), events.next())
            .await
            .map_err(|_| "native Codex attempt timed out".to_owned())?
            .transpose()
            .map_err(|error| error.to_string())?
        {
            let payload: serde_json::Value =
                serde_json::from_slice(&event.payload).map_err(|error| error.to_string())?;
            let serialized = payload.to_string();
            saw_marker |= serialized.contains("OMNISOLO_LIVE_E2E_OK");
        }
        if !saw_marker {
            return Err("native Codex response did not contain the verification marker".to_owned());
        }

        for (kind, idempotency_key) in [("close", "live-session-close"), ("delete", "live-session-delete")] {
            let response = client
                .session_operation(Request::new(SessionOperationEnvelope {
                    protocol_version: 1,
                    tenant_id: "tenant-live-codex".to_owned(),
                    session_id: session_id.to_string(),
                    operation_id: Uuid::new_v4().to_string(),
                    operation_generation: 1,
                    fencing_token: format!("live-{kind}-fence"),
                    kind: kind.to_owned(),
                    task_id: task_id.to_string(),
                    correlation_id: String::new(),
                    idempotency_key: idempotency_key.to_owned(),
                    payload_schema: "omnisolo.session.operation.v1".to_owned(),
                    payload_version: 1,
                    payload: serde_json::to_vec(&serde_json::json!({
                        "native_session_id": native.native_session_id.clone(),
                    }))
                    .map_err(|error| error.to_string())?,
                    extensions: Default::default(),
                    worker_id: "worker-live-codex".to_owned(),
                    pool_id: "codex".to_owned(),
                    harness_id: "codex".to_owned(),
                    capability_version: 1,
                    binding_id: String::new(),
                    binding_generation: 0,
                    workspace_mutation_scope_id: "workspace-live-codex".to_owned(),
                }))
                .await
                .map_err(|error| error.to_string())?
                .into_inner();
            if !response.accepted {
                return Err(format!("native Codex {kind} was not accepted"));
            }
        }
        Ok::<(), String>(())
    }
    .await;

    stop(&mut child).await;
    result
}
