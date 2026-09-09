use std::collections::BTreeSet;

use server_harness::middleware::adapter::{OmniSoloHarnessAdapter, OmniSoloRunConfig};
use server_harness::middleware::grpc::HarnessWorkerGrpcService;
use server_harness::middleware::harness::ProcessHarnessSpec;
use server_harness::middleware::local_services::{
    LocalServiceKind, LocalServiceRegistry, LocalServiceScopeContext,
};
use server_harness::middleware::types::{ModelApiDialect, ReasoningEffort, ResolvedModelSelection};
use server_omnisolo::harness_middleware::harness_worker_service_server::HarnessWorkerService;
use server_omnisolo::harness_middleware::{
    AttemptCommandEnvelope, SessionOperationEnvelope, WorkerControlEnvelope,
    WorkerExchangeEnvelope, WorkerHealthRequest,
};
use tokio_stream::StreamExt;
use tonic::{Request, Status};
use uuid::Uuid;

fn resolved_model(model_id: &str) -> ResolvedModelSelection {
    ResolvedModelSelection {
        provider_route: "openai-compatible".to_owned(),
        model_id: model_id.to_owned(),
        reasoning_effort: Some(ReasoningEffort::Max),
        api_dialect: ModelApiDialect::OpenAiResponses,
        context_window: None,
        max_output_tokens: None,
        capabilities: BTreeSet::from(["reasoning".to_owned(), "tools".to_owned()]),
        binding_revision: "worker-env-v1".to_owned(),
        binding_digest: "sha256:runtime-unbound".to_owned(),
        metadata: Default::default(),
    }
}

#[tokio::test]
async fn grpc_worker_preflight_rejects_a_missing_native_harness_before_serving() {
    let service = HarnessWorkerGrpcService::with_process_spec(
        ProcessHarnessSpec::command(
            "/definitely/missing/omnisolo-harness",
            std::iter::empty::<String>(),
            "codex",
        )
        .with_model_routing(resolved_model("gpt-5.6-luna"), None),
    );

    let error = service.preflight().await.unwrap_err();
    assert!(
        error.to_string().contains("No such file") || error.to_string().contains("not found"),
        "unexpected preflight error: {error}"
    );
    assert!(service.health_snapshot().unwrap().ready);
}

#[tokio::test]
async fn grpc_worker_service_routes_health_control_and_fenced_attempts() {
    let service = HarnessWorkerGrpcService::new("worker-1", "codex", "codex-pool");

    let health = service
        .clone()
        .health(Request::new(WorkerHealthRequest {
            protocol_version: 1,
            worker_id: "worker-1".to_owned(),
            harness_id: "codex".to_owned(),
            pool_id: "codex-pool".to_owned(),
        }))
        .await
        .unwrap()
        .into_inner();
    assert!(health.ready);
    assert!(health.accepting_new_attempts);

    let control = service
        .clone()
        .control(Request::new(WorkerControlEnvelope {
            protocol_version: 1,
            worker_id: "worker-1".to_owned(),
            runtime_id: "codex".to_owned(),
            kind: "heartbeat".to_owned(),
            correlation_id: String::new(),
            idempotency_key: String::new(),
            capability_version: 1,
            payload_schema: "omnisolo.worker.heartbeat.v1".to_owned(),
            payload_version: 1,
            payload: Vec::new(),
            extensions: Default::default(),
        }))
        .await
        .unwrap()
        .into_inner();
    assert!(control.accepted);

    let session_id = "00000000-0000-0000-0000-000000000001";
    let task_id = "00000000-0000-0000-0000-000000000002";
    let attempt_id = "00000000-0000-0000-0000-000000000003";
    let command = AttemptCommandEnvelope {
        protocol_version: 1,
        tenant_id: "tenant-1".to_owned(),
        session_id: session_id.to_owned(),
        task_id: task_id.to_owned(),
        attempt_id: attempt_id.to_owned(),
        turn_id: String::new(),
        command_id: "00000000-0000-0000-0000-000000000004".to_owned(),
        lease_id: "00000000-0000-0000-0000-000000000005".to_owned(),
        lease_generation: 1,
        fencing_token: "lease-fence-1".to_owned(),
        kind: "execute".to_owned(),
        correlation_id: String::new(),
        idempotency_key: Some("command-1".to_owned()).unwrap_or_default(),
        payload_schema: "omnisolo.attempt.command.v1".to_owned(),
        payload_version: 1,
        payload: Vec::new(),
        extensions: Default::default(),
        worker_id: "worker-1".to_owned(),
        harness_id: "codex".to_owned(),
        capability_version: 1,
        binding_id: String::new(),
        binding_generation: 0,
    };
    let mut stream = service
        .clone()
        .attempt_command(Request::new(command))
        .await
        .unwrap()
        .into_inner();
    let event = stream.next().await.unwrap().unwrap();
    assert_eq!(event.source_attempt_id, attempt_id);
    assert_eq!(event.lease_generation, 1);
}

#[tokio::test]
async fn grpc_worker_service_executes_configured_external_harness_process() {
    let script = r#"
while IFS= read -r line; do
  request_id=$(printf '%s' "$line" | sed -n 's/.*"request_id":"\([^"]*\)".*/\1/p')
  printf '{"request_id":"%s","ok":true,"payload":{"events":[{"event_type":"assistant.text","durable":true,"payload":{"text":"external-result","name":"safe-name","api_key":"CANARY-NOTIFY-7KQ9","note":"Bearer CANARY-NOTIFY-7KQ9"}}],"final_text":"external-result"}}\n' "$request_id"
done
"#;
    let service = HarnessWorkerGrpcService::with_process_spec(
        ProcessHarnessSpec::command("/bin/sh", ["-c", script], "codex")
            .with_protocol(server_harness::middleware::harness::HarnessProtocolKind::Custom),
    );
    let command = AttemptCommandEnvelope {
        protocol_version: 1,
        tenant_id: "tenant-1".to_owned(),
        session_id: "00000000-0000-0000-0000-000000000011".to_owned(),
        task_id: "00000000-0000-0000-0000-000000000012".to_owned(),
        attempt_id: "00000000-0000-0000-0000-000000000013".to_owned(),
        turn_id: String::new(),
        command_id: "00000000-0000-0000-0000-000000000014".to_owned(),
        lease_id: "00000000-0000-0000-0000-000000000015".to_owned(),
        lease_generation: 1,
        fencing_token: "lease-fence-1".to_owned(),
        kind: "execute".to_owned(),
        correlation_id: String::new(),
        idempotency_key: String::new(),
        payload_schema: "omnisolo.attempt.command.v1".to_owned(),
        payload_version: 1,
        payload: serde_json::to_vec(&serde_json::json!({"prompt": "run it"})).unwrap(),
        extensions: Default::default(),
        worker_id: "worker-1".to_owned(),
        harness_id: "codex".to_owned(),
        capability_version: 1,
        binding_id: String::new(),
        binding_generation: 0,
    };
    let mut stream = service
        .attempt_command(Request::new(command.clone()))
        .await
        .unwrap()
        .into_inner();
    let event = stream.next().await.unwrap().unwrap();
    assert_eq!(event.payload_schema, "omnisolo.harness.event.v1");
    assert_eq!(event.durable_sequence, 1);
    let event_payload: serde_json::Value = serde_json::from_slice(&event.payload).unwrap();
    assert_eq!(event_payload["payload"]["name"], "safe-name");
    assert!(event_payload["payload"].get("api_key").is_none());
    assert_eq!(event_payload["payload"]["note"], "[REDACTED]");
    assert!(!event_payload.to_string().contains("CANARY-NOTIFY-7KQ9"));

    let mut duplicate_stream = service
        .attempt_command(Request::new(command.clone()))
        .await
        .unwrap()
        .into_inner();
    let duplicate = duplicate_stream.next().await.unwrap().unwrap();
    assert_eq!(
        duplicate.payload_schema,
        "omnisolo.worker.command.accepted.v1"
    );
    assert!(
        String::from_utf8(duplicate.payload)
            .unwrap()
            .contains("duplicate")
    );

    let mut steer = command.clone();
    steer.kind = "steer".to_owned();
    steer.command_id = uuid::Uuid::new_v4().to_string();
    steer.idempotency_key = "command-steer-1".to_owned();
    steer.payload = serde_json::to_vec(&serde_json::json!({"prompt": "steer it"})).unwrap();
    let mut steer_stream = service
        .clone()
        .attempt_command(Request::new(steer))
        .await
        .unwrap()
        .into_inner();
    assert_eq!(
        steer_stream.next().await.unwrap().unwrap().durable_sequence,
        3
    );

    let acknowledged_stream = command.command_id.clone();
    let mut replay = command;
    replay.kind = "reconcile".to_owned();
    replay.command_id = uuid::Uuid::new_v4().to_string();
    replay.idempotency_key = "command-replay-1".to_owned();
    replay.payload = serde_json::to_vec(&serde_json::json!({
        "replay_from": 1,
        "replay_to": 2,
        "ack_delivery_stream_id": acknowledged_stream,
        "ack_delivery_sequence": 2,
    }))
    .unwrap();
    let mut replay_stream = service
        .attempt_command(Request::new(replay))
        .await
        .unwrap()
        .into_inner();
    let replayed_first = replay_stream.next().await.unwrap().unwrap();
    let replayed_second = replay_stream.next().await.unwrap().unwrap();
    for replayed in [&replayed_first, &replayed_second] {
        let payload: serde_json::Value = serde_json::from_slice(&replayed.payload).unwrap();
        assert!(!payload.to_string().contains("CANARY-NOTIFY-7KQ9"));
        assert!(!payload.to_string().contains("api_key"));
    }
    assert_eq!(replayed_first.durable_sequence, 1);
    assert_eq!(replayed_second.durable_sequence, 2);
    assert_eq!(
        replayed_first.delivery_stream_id,
        replayed_second.delivery_stream_id
    );
    assert_eq!(
        service
            .acknowledged_delivery_sequence(&replayed_first.delivery_stream_id)
            .await,
        None
    );
    assert_eq!(
        service
            .acknowledged_delivery_sequence("00000000-0000-0000-0000-000000000014")
            .await,
        Some(2)
    );
    assert!(replay_stream.next().await.is_none());
}

#[tokio::test]
async fn grpc_process_requests_use_worker_model_fallback_but_preserve_explicit_selection() {
    let script = r#"
while IFS= read -r line; do
  request_id=$(printf '%s' "$line" | sed -n 's/.*"request_id":"\([^"]*\)".*/\1/p')
  if printf '%s' "$line" | grep -q '"model_id":"task-explicit"' \
    && ! printf '%s' "$line" | grep -q '"model_id":"worker-default"'; then
    result=explicit
  elif printf '%s' "$line" | grep -q '"model_id":"worker-default"'; then
    result=default
  else
    printf '{"request_id":"%s","ok":false,"error":"resolved model missing"}\n' "$request_id"
    continue
  fi
  printf '{"request_id":"%s","ok":true,"payload":{"events":[{"event_type":"assistant.text","durable":true,"payload":{"text":"%s"}}],"final_text":"%s"}}\n' "$request_id" "$result" "$result"
done
"#;
    let worker_default = resolved_model("worker-default");
    let service = HarnessWorkerGrpcService::with_process_spec(
        ProcessHarnessSpec::command("/bin/sh", ["-c", script], "codex")
            .with_protocol(server_harness::middleware::harness::HarnessProtocolKind::Custom)
            .with_model_routing(worker_default, None),
    );
    let command = |payload: serde_json::Value| AttemptCommandEnvelope {
        protocol_version: 1,
        tenant_id: "tenant-model-routing".to_owned(),
        session_id: uuid::Uuid::new_v4().to_string(),
        task_id: uuid::Uuid::new_v4().to_string(),
        attempt_id: uuid::Uuid::new_v4().to_string(),
        turn_id: String::new(),
        command_id: uuid::Uuid::new_v4().to_string(),
        lease_id: uuid::Uuid::new_v4().to_string(),
        lease_generation: 1,
        fencing_token: "model-routing-fence".to_owned(),
        kind: "execute".to_owned(),
        correlation_id: String::new(),
        idempotency_key: uuid::Uuid::new_v4().to_string(),
        payload_schema: "omnisolo.attempt.command.v1".to_owned(),
        payload_version: 1,
        payload: serde_json::to_vec(&payload).unwrap(),
        extensions: Default::default(),
        worker_id: "worker-1".to_owned(),
        harness_id: "codex".to_owned(),
        capability_version: 1,
        binding_id: String::new(),
        binding_generation: 0,
    };

    let mut fallback = service
        .clone()
        .attempt_command(Request::new(command(
            serde_json::json!({"prompt": "fallback"}),
        )))
        .await
        .unwrap()
        .into_inner();
    let fallback_event = fallback.next().await.unwrap().unwrap();
    assert!(
        String::from_utf8(fallback_event.payload)
            .unwrap()
            .contains("default")
    );

    let mut explicit = service
        .attempt_command(Request::new(command(serde_json::json!({
            "prompt": "explicit",
            "context": {"resolved_model": resolved_model("task-explicit")}
        }))))
        .await
        .unwrap()
        .into_inner();
    let explicit_event = explicit.next().await.unwrap().unwrap();
    assert!(
        String::from_utf8(explicit_event.payload)
            .unwrap()
            .contains("explicit")
    );
}

#[tokio::test]
async fn grpc_attempt_context_carries_validated_local_service_bindings_to_the_worker() {
    let session_id = Uuid::from_u128(300);
    let task_id = Uuid::from_u128(301);
    let attempt_id = Uuid::from_u128(302);
    use server_harness::middleware::local_service_gateway::{
        LocalServiceGateway, SqliteAgentMemoryBackend,
    };
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::query("CREATE VIRTUAL TABLE agent_memory USING fts5(content,tags,created_at UNINDEXED)")
        .execute(&pool)
        .await
        .unwrap();
    let scope = LocalServiceScopeContext::for_attempt(
        "tenant-local-services",
        Some("project-local"),
        Some("workspace-local"),
        session_id,
        Some(task_id),
        Some(attempt_id),
    );
    let mut gateway = LocalServiceGateway::new(LocalServiceRegistry::with_defaults());
    gateway.register(
        LocalServiceKind::Memory,
        std::sync::Arc::new(SqliteAgentMemoryBackend::new(pool)),
    );
    let bundle = gateway.resolve(scope.clone()).unwrap();
    let memory_binding_id = bundle
        .binding(LocalServiceKind::Memory)
        .unwrap()
        .binding_id
        .to_string();
    let script = format!(
        r#"
while IFS= read -r line; do
  request_id=$(printf '%s' "$line" | sed -n 's/.*"request_id":"\([^"]*\)".*/\1/p')
  if printf '%s' "$line" | grep -q '"local_services"' \
    && printf '%s' "$line" | grep -q '"service_id":"omnisolo.memory"'; then
    printf '{{"request_id":"%s","ok":true,"payload":{{"native_session_id":"local-services-session","events":[],"final_text":"local-services-bound"}}}}\n' "$request_id"
  else
    printf '{{"request_id":"%s","ok":false,"error":"local service binding missing"}}\n' "$request_id"
  fi
done
"#
    );
    let service = HarnessWorkerGrpcService::with_process_spec(
        ProcessHarnessSpec::command("/bin/sh", ["-c".to_owned(), script], "codex")
            .with_protocol(server_harness::middleware::harness::HarnessProtocolKind::Custom),
    )
    .with_local_service_gateway(std::sync::Arc::new(gateway))
    .with_trusted_service_scope(scope);
    let command = AttemptCommandEnvelope {
        protocol_version: 1,
        tenant_id: "tenant-local-services".to_owned(),
        session_id: session_id.to_string(),
        task_id: task_id.to_string(),
        attempt_id: attempt_id.to_string(),
        turn_id: String::new(),
        command_id: Uuid::from_u128(303).to_string(),
        lease_id: Uuid::from_u128(304).to_string(),
        lease_generation: 1,
        fencing_token: "local-services-fence".to_owned(),
        kind: "execute".to_owned(),
        correlation_id: String::new(),
        idempotency_key: "local-services-command".to_owned(),
        payload_schema: "omnisolo.attempt.command.v1".to_owned(),
        payload_version: 1,
        payload: serde_json::to_vec(&serde_json::json!({
            "prompt": "use shared services",
            "context": {"local_services": bundle}
        }))
        .unwrap(),
        extensions: Default::default(),
        worker_id: "worker-1".to_owned(),
        harness_id: "codex".to_owned(),
        capability_version: 1,
        binding_id: String::new(),
        binding_generation: 0,
    };

    let mut stream = service
        .attempt_command(Request::new(command))
        .await
        .unwrap()
        .into_inner();
    let mut payloads = Vec::new();
    while let Some(event) = stream.next().await {
        payloads.push(String::from_utf8(event.unwrap().payload).unwrap());
    }
    let event_types = payloads
        .iter()
        .map(|payload| {
            serde_json::from_str::<serde_json::Value>(payload).unwrap()["event_type"]
                .as_str()
                .unwrap()
                .to_owned()
        })
        .collect::<Vec<_>>();
    let bound_index = event_types
        .iter()
        .position(|kind| kind == "local_services.bound")
        .unwrap();
    assert!(
        event_types[..bound_index]
            .iter()
            .all(|kind| kind == "user.message")
    );
    let bound: serde_json::Value = serde_json::from_str(&payloads[bound_index]).unwrap();
    assert_ne!(
        bound["payload"]["bindings"][0]["binding_id"].as_str(),
        Some(memory_binding_id.as_str())
    );
    assert_eq!(bound["payload"]["bindings"][0]["generation"], 2);
    assert!(
        payloads
            .iter()
            .any(|payload| payload.contains("local-services-bound"))
    );
}

#[tokio::test]
async fn grpc_worker_service_routes_exchange_payloads_to_external_harness() {
    let script = r#"
while IFS= read -r line; do
  request_id=$(printf '%s' "$line" | sed -n 's/.*"request_id":"\([^"]*\)".*/\1/p')
  printf '{"request_id":"%s","ok":true,"payload":{"handled":true,"source":"external"}}\n' "$request_id"
done
"#;
    let service = HarnessWorkerGrpcService::with_process_spec(
        ProcessHarnessSpec::command("/bin/sh", ["-c", script], "codex")
            .with_protocol(server_harness::middleware::harness::HarnessProtocolKind::Custom),
    );
    let exchange = WorkerExchangeEnvelope {
        protocol_version: 1,
        tenant_id: "tenant-1".to_owned(),
        session_id: "00000000-0000-0000-0000-000000000031".to_owned(),
        task_id: "00000000-0000-0000-0000-000000000032".to_owned(),
        attempt_id: "00000000-0000-0000-0000-000000000033".to_owned(),
        worker_id: "worker-1".to_owned(),
        harness_id: "codex".to_owned(),
        pool_id: "codex".to_owned(),
        lease_id: "00000000-0000-0000-0000-000000000034".to_owned(),
        lease_generation: 1,
        fencing_token: "exchange-fence".to_owned(),
        kind: "approval.requested".to_owned(),
        idempotency_key: "external-exchange-1".to_owned(),
        payload_schema: "omnisolo.interaction.v1".to_owned(),
        payload_version: 1,
        payload: serde_json::to_vec(&serde_json::json!({"risk":"medium"})).unwrap(),
        artifact_digest: "".to_owned(),
        extensions: [("request_tag".to_owned(), "tag-1".to_owned())]
            .into_iter()
            .collect(),
    };
    let response = service
        .clone()
        .interaction(Request::new(exchange.clone()))
        .await
        .unwrap()
        .into_inner();
    assert!(response.accepted);
    assert!(!response.duplicate);
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&response.payload).unwrap(),
        serde_json::json!({"handled":true,"source":"external"})
    );

    let duplicate = service
        .artifact(Request::new(exchange))
        .await
        .unwrap()
        .into_inner();
    assert!(duplicate.duplicate);
    assert!(duplicate.payload.is_empty());
}

#[tokio::test]
async fn grpc_worker_service_routes_every_session_operation_to_external_harness() {
    let script = r#"
while IFS= read -r line; do
  request_id=$(printf '%s' "$line" | sed -n 's/.*"request_id":"\([^"]*\)".*/\1/p')
  printf '{"request_id":"%s","ok":true,"payload":{"native_session_id":"native-session","native_cursor":"cursor"}}\n' "$request_id"
done
"#;
    let service = HarnessWorkerGrpcService::with_process_spec(
        ProcessHarnessSpec::command("/bin/sh", ["-c", script], "codex")
            .with_protocol(server_harness::middleware::harness::HarnessProtocolKind::Custom),
    );
    let session_id = uuid::Uuid::new_v4();
    let task_id = uuid::Uuid::new_v4();
    let capsule = OmniSoloHarnessAdapter::start(
        OmniSoloRunConfig::new("tenant-session-ops", "portable objective")
            .with_session_id(session_id)
            .with_task_id(task_id),
    )
    .unwrap()
    .export_capsule("codex", uuid::Uuid::new_v4())
    .unwrap();

    let operation = |kind: &str, payload: serde_json::Value| SessionOperationEnvelope {
        protocol_version: 1,
        tenant_id: "tenant-session-ops".to_owned(),
        session_id: session_id.to_string(),
        operation_id: uuid::Uuid::new_v4().to_string(),
        operation_generation: 1,
        fencing_token: format!("{kind}-fence"),
        kind: kind.to_owned(),
        task_id: task_id.to_string(),
        correlation_id: String::new(),
        idempotency_key: format!("{kind}-idempotency"),
        payload_schema: "omnisolo.session.operation.v1".to_owned(),
        payload_version: 1,
        payload: serde_json::to_vec(&payload).unwrap(),
        extensions: Default::default(),
        worker_id: "worker-1".to_owned(),
        pool_id: "codex".to_owned(),
        harness_id: "codex".to_owned(),
        capability_version: 1,
        binding_id: String::new(),
        binding_generation: 0,
        workspace_mutation_scope_id: "workspace-session-ops".to_owned(),
    };

    for (kind, payload) in [
        ("create", serde_json::json!({"objective":"create"})),
        ("import", serde_json::json!({"capsule": capsule.clone()})),
        ("handoff", serde_json::json!({"capsule": capsule.clone()})),
        (
            "fork",
            serde_json::json!({"objective":"fork","native_session_id":"native-session"}),
        ),
        (
            "resume",
            serde_json::json!({"native_session_id":"native-session"}),
        ),
        ("quiesce", serde_json::json!({})),
        ("snapshot", serde_json::json!({})),
        ("cancel", serde_json::json!({})),
        ("close", serde_json::json!({})),
        ("delete", serde_json::json!({})),
    ] {
        let response = service
            .clone()
            .session_operation(Request::new(operation(kind, payload)))
            .await
            .unwrap()
            .into_inner();
        assert!(response.accepted, "{kind} was not accepted");
        assert_eq!(
            response.payload_schema,
            "omnisolo.harness.native_session.v1"
        );
        let native: server_harness::middleware::harness::NativeSession =
            serde_json::from_slice(&response.payload).unwrap();
        if matches!(kind, "close" | "delete") {
            assert!(native.native_session_id.is_empty());
        } else {
            assert_eq!(native.native_session_id, "native-session");
        }
    }
}

#[tokio::test]
async fn grpc_worker_runs_native_codex_session_lifecycle_without_legacy_envelopes() {
    let service = HarnessWorkerGrpcService::with_process_spec(
        ProcessHarnessSpec::command(
            "/bin/sh",
            ["-c", native_session_lifecycle_script()],
            "codex",
        )
        .with_protocol(server_harness::middleware::harness::HarnessProtocolKind::CodexAppServer),
    );
    let session_id = uuid::Uuid::new_v4();
    let task_id = uuid::Uuid::new_v4();
    let capsule = OmniSoloHarnessAdapter::start(
        OmniSoloRunConfig::new("tenant-native-lifecycle", "portable objective")
            .with_session_id(session_id)
            .with_task_id(task_id),
    )
    .unwrap()
    .export_capsule("codex", uuid::Uuid::new_v4())
    .unwrap();
    let operation = |kind: &str, payload: serde_json::Value| SessionOperationEnvelope {
        protocol_version: 1,
        tenant_id: "tenant-native-lifecycle".to_owned(),
        session_id: session_id.to_string(),
        operation_id: uuid::Uuid::new_v4().to_string(),
        operation_generation: 1,
        fencing_token: format!("{kind}-fence"),
        kind: kind.to_owned(),
        task_id: task_id.to_string(),
        correlation_id: String::new(),
        idempotency_key: format!("native-{kind}-{}", uuid::Uuid::new_v4()),
        payload_schema: "omnisolo.session.operation.v1".to_owned(),
        payload_version: 1,
        payload: serde_json::to_vec(&payload).unwrap(),
        extensions: Default::default(),
        worker_id: "worker-1".to_owned(),
        pool_id: "codex".to_owned(),
        harness_id: "codex".to_owned(),
        capability_version: 1,
        binding_id: String::new(),
        binding_generation: 0,
        workspace_mutation_scope_id: "workspace-native-lifecycle".to_owned(),
    };

    let create = service
        .clone()
        .session_operation(Request::new(operation(
            "create",
            serde_json::json!({"objective":"create"}),
        )))
        .await
        .unwrap()
        .into_inner();
    let created: server_harness::middleware::harness::NativeSession =
        serde_json::from_slice(&create.payload).unwrap();
    assert_eq!(created.native_session_id, "thread-1");

    let import = service
        .clone()
        .session_operation(Request::new(operation(
            "import",
            serde_json::json!({"capsule": capsule}),
        )))
        .await
        .unwrap()
        .into_inner();
    let imported: server_harness::middleware::harness::NativeSession =
        serde_json::from_slice(&import.payload).unwrap();
    assert_eq!(imported.native_session_id, "thread-2");

    for (kind, payload, expected) in [
        (
            "resume",
            serde_json::json!({"native_session_id":"thread-1"}),
            "thread-1",
        ),
        (
            "fork",
            serde_json::json!({"native_session_id":"thread-1"}),
            "thread-fork",
        ),
        (
            "snapshot",
            serde_json::json!({"native_session_id":"thread-2"}),
            "thread-2",
        ),
        (
            "quiesce",
            serde_json::json!({"native_session_id":"thread-2"}),
            "thread-2",
        ),
        (
            "cancel",
            serde_json::json!({"native_session_id":"thread-2"}),
            "thread-2",
        ),
    ] {
        let response = service
            .clone()
            .session_operation(Request::new(operation(kind, payload)))
            .await
            .unwrap()
            .into_inner();
        let native: server_harness::middleware::harness::NativeSession =
            serde_json::from_slice(&response.payload).unwrap();
        assert_eq!(native.native_session_id, expected, "{kind}");
    }

    for kind in ["close", "delete"] {
        let response = service
            .clone()
            .session_operation(Request::new(operation(
                kind,
                serde_json::json!({"native_session_id":"thread-2"}),
            )))
            .await
            .unwrap()
            .into_inner();
        let native: server_harness::middleware::harness::NativeSession =
            serde_json::from_slice(&response.payload).unwrap();
        assert!(native.native_session_id.is_empty(), "{kind}");
    }
}

#[tokio::test]
async fn grpc_worker_service_fences_interaction_artifact_workspace_and_native_exchanges() {
    let service = HarnessWorkerGrpcService::new("worker-1", "codex", "codex-pool");
    let base = WorkerExchangeEnvelope {
        protocol_version: 1,
        tenant_id: "tenant-1".to_owned(),
        session_id: "00000000-0000-0000-0000-000000000021".to_owned(),
        task_id: "00000000-0000-0000-0000-000000000022".to_owned(),
        attempt_id: "00000000-0000-0000-0000-000000000023".to_owned(),
        worker_id: "worker-1".to_owned(),
        harness_id: "codex".to_owned(),
        pool_id: "codex-pool".to_owned(),
        lease_id: "00000000-0000-0000-0000-000000000024".to_owned(),
        lease_generation: 1,
        fencing_token: "exchange-fence".to_owned(),
        kind: "approval.requested".to_owned(),
        idempotency_key: "exchange-1".to_owned(),
        payload_schema: "omnisolo.interaction.v1".to_owned(),
        payload_version: 1,
        payload: serde_json::to_vec(&serde_json::json!({"risk": "medium"})).unwrap(),
        artifact_digest: "".to_owned(),
        extensions: Default::default(),
    };
    let interaction = service
        .clone()
        .interaction(Request::new(base.clone()))
        .await
        .unwrap()
        .into_inner();
    assert!(interaction.accepted);
    assert!(!interaction.duplicate);

    let duplicate = service
        .clone()
        .artifact(Request::new(base.clone()))
        .await
        .unwrap()
        .into_inner();
    assert!(duplicate.duplicate);

    let mut workspace = base.clone();
    workspace.attempt_id = "00000000-0000-0000-0000-000000000025".to_owned();
    workspace.lease_id = "00000000-0000-0000-0000-000000000026".to_owned();
    workspace.kind = "workspace.snapshot".to_owned();
    workspace.artifact_digest = "tree-digest".to_owned();
    assert!(
        service
            .clone()
            .workspace(Request::new(workspace))
            .await
            .unwrap()
            .into_inner()
            .accepted
    );

    let mut native = base;
    native.attempt_id = "00000000-0000-0000-0000-000000000027".to_owned();
    native.lease_id = "00000000-0000-0000-0000-000000000028".to_owned();
    native.kind = "native_record.export".to_owned();
    assert!(
        service
            .native_record(Request::new(native))
            .await
            .unwrap()
            .into_inner()
            .accepted
    );
}

#[tokio::test]
async fn grpc_worker_resolves_native_codex_server_request_while_attempt_streams() {
    let script = r#"
while IFS= read -r line; do
  if printf '%s' "$line" | grep -qE '"request_id"|"protocol_version"'; then
    exit 23
  fi
  id=$(printf '%s' "$line" | sed -n 's/.*"id":\([0-9][0-9]*\).*/\1/p')
  method=$(printf '%s' "$line" | sed -n 's/.*"method":"\([^"]*\)".*/\1/p')
  case "$method" in
    initialize)
      printf '{"jsonrpc":"2.0","id":%s,"result":{"capabilities":{}}}\n' "$id" ;;
    initialized)
      : ;;
    thread/start|thread/resume|thread/fork|thread/inject_items|thread/read)
      printf '{"jsonrpc":"2.0","id":%s,"result":{"thread":{"id":"thread-1"}}}\n' "$id" ;;
    turn/start)
      printf '%s\n' '{"jsonrpc":"2.0","id":9001,"method":"item/commandExecution/requestApproval","params":{"threadId":"thread-1","turnId":"turn-1","command":"echo test"}}'
      read response
      printf '{"jsonrpc":"2.0","id":%s,"result":{"turn":{"id":"turn-1"}}}\n' "$id"
      printf '%s\n' '{"jsonrpc":"2.0","method":"turn/started","params":{"threadId":"thread-1","turnId":"turn-1"}}'
      printf '%s\n' '{"jsonrpc":"2.0","method":"item/agentMessage/delta","params":{"threadId":"thread-1","turnId":"turn-1","delta":"worker native answer"}}'
      printf '%s\n' '{"jsonrpc":"2.0","method":"item/completed","params":{"threadId":"thread-1","turnId":"turn-1","item":{"type":"agentMessage","text":"worker native answer"}}}'
      printf '%s\n' '{"jsonrpc":"2.0","method":"turn/completed","params":{"threadId":"thread-1","turnId":"turn-1"}}' ;;
    thread/archive|thread/delete|turn/interrupt)
      printf '{"jsonrpc":"2.0","id":%s,"result":{"thread":{"id":"thread-1"}}}\n' "$id" ;;
    *)
      printf '{"jsonrpc":"2.0","id":%s,"result":{}}\n' "$id" ;;
  esac
done
"#;
    let service = HarnessWorkerGrpcService::with_process_spec(
        ProcessHarnessSpec::command("/bin/sh", ["-c", script], "codex").with_protocol(
            server_harness::middleware::harness::HarnessProtocolKind::CodexAppServer,
        ),
    );
    let session_id = uuid::Uuid::new_v4();
    let task_id = uuid::Uuid::new_v4();
    let session_operation = SessionOperationEnvelope {
        protocol_version: 1,
        tenant_id: "tenant-native-worker".to_owned(),
        session_id: session_id.to_string(),
        operation_id: uuid::Uuid::new_v4().to_string(),
        operation_generation: 1,
        fencing_token: "native-session-fence".to_owned(),
        kind: "create".to_owned(),
        task_id: task_id.to_string(),
        correlation_id: String::new(),
        idempotency_key: "native-session-create".to_owned(),
        payload_schema: "omnisolo.session.create.v1".to_owned(),
        payload_version: 1,
        payload: serde_json::to_vec(&serde_json::json!({"objective":"native worker"})).unwrap(),
        extensions: Default::default(),
        worker_id: "worker-1".to_owned(),
        pool_id: "codex".to_owned(),
        harness_id: "codex".to_owned(),
        capability_version: 1,
        binding_id: String::new(),
        binding_generation: 0,
        workspace_mutation_scope_id: "workspace-native".to_owned(),
    };
    let session_response = service
        .clone()
        .session_operation(Request::new(session_operation))
        .await
        .unwrap()
        .into_inner();
    let native: server_harness::middleware::harness::NativeSession =
        serde_json::from_slice(&session_response.payload).unwrap();
    assert_eq!(native.native_session_id, "thread-1");

    let attempt_id = uuid::Uuid::new_v4();
    let command = AttemptCommandEnvelope {
        protocol_version: 1,
        tenant_id: "tenant-native-worker".to_owned(),
        session_id: session_id.to_string(),
        task_id: task_id.to_string(),
        attempt_id: attempt_id.to_string(),
        turn_id: String::new(),
        command_id: uuid::Uuid::new_v4().to_string(),
        lease_id: uuid::Uuid::new_v4().to_string(),
        lease_generation: 1,
        fencing_token: "native-attempt-fence".to_owned(),
        kind: "execute".to_owned(),
        correlation_id: String::new(),
        idempotency_key: "native-attempt".to_owned(),
        payload_schema: "omnisolo.attempt.command.v1".to_owned(),
        payload_version: 1,
        payload: serde_json::to_vec(&serde_json::json!({
            "prompt": "continue natively",
            "native_session_id": "thread-1",
        }))
        .unwrap(),
        extensions: Default::default(),
        worker_id: "worker-1".to_owned(),
        harness_id: "codex".to_owned(),
        capability_version: 1,
        binding_id: String::new(),
        binding_generation: 0,
    };
    let mut stream = service
        .clone()
        .attempt_command(Request::new(command.clone()))
        .await
        .unwrap()
        .into_inner();
    let approval = tokio::time::timeout(std::time::Duration::from_secs(2), stream.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    let approval_payload: serde_json::Value = serde_json::from_slice(&approval.payload).unwrap();
    assert_eq!(approval_payload["event_type"], "interaction.required");
    assert_eq!(approval_payload["payload"]["native_request_id"], 9001);

    let exchange = WorkerExchangeEnvelope {
        protocol_version: 1,
        tenant_id: "tenant-native-worker".to_owned(),
        session_id: session_id.to_string(),
        task_id: task_id.to_string(),
        attempt_id: attempt_id.to_string(),
        worker_id: "worker-1".to_owned(),
        harness_id: "codex".to_owned(),
        pool_id: "codex".to_owned(),
        lease_id: command.lease_id.clone(),
        lease_generation: 1,
        fencing_token: "native-attempt-fence".to_owned(),
        kind: "approval.requested".to_owned(),
        idempotency_key: "native-approval".to_owned(),
        payload_schema: "omnisolo.interaction.v1".to_owned(),
        payload_version: 1,
        payload: serde_json::to_vec(&serde_json::json!({
            "native_request_id": 9001,
            "result": {"decision":"accept"},
        }))
        .unwrap(),
        artifact_digest: String::new(),
        extensions: Default::default(),
    };
    let exchange_response = service
        .clone()
        .interaction(Request::new(exchange))
        .await
        .unwrap()
        .into_inner();
    assert!(exchange_response.accepted);
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&exchange_response.payload).unwrap()["native_request_id"],
        9001
    );

    let mut saw_delta = false;
    let mut saw_final = false;
    while let Some(event) = tokio::time::timeout(std::time::Duration::from_secs(2), stream.next())
        .await
        .unwrap()
    {
        let event = event.unwrap();
        let payload: serde_json::Value = serde_json::from_slice(&event.payload).unwrap();
        saw_delta |= payload["event_type"] == "assistant.text_chunk";
        saw_final |= payload["event_type"] == "assistant.final";
    }
    assert!(saw_delta && saw_final);
}

#[tokio::test]
async fn grpc_worker_service_runs_the_first_party_omnisolo_adapter() {
    let service = HarnessWorkerGrpcService::new("worker-omni", "omnisolo", "omnisolo-pool");
    let session_id = uuid::Uuid::new_v4();
    let task_id = uuid::Uuid::new_v4();
    let operation = service
        .clone()
        .session_operation(Request::new(SessionOperationEnvelope {
            protocol_version: 1,
            tenant_id: "tenant-omni".to_owned(),
            session_id: session_id.to_string(),
            operation_id: uuid::Uuid::new_v4().to_string(),
            operation_generation: 1,
            fencing_token: "operation-fence".to_owned(),
            kind: "create".to_owned(),
            task_id: task_id.to_string(),
            correlation_id: String::new(),
            idempotency_key: "omni-create".to_owned(),
            payload_schema: "omnisolo.session.create.v1".to_owned(),
            payload_version: 1,
            payload: serde_json::to_vec(&serde_json::json!({
                "objective": "run the native harness"
            }))
            .unwrap(),
            extensions: Default::default(),
            worker_id: "worker-omni".to_owned(),
            pool_id: "omnisolo-pool".to_owned(),
            harness_id: "omnisolo".to_owned(),
            capability_version: 1,
            binding_id: String::new(),
            binding_generation: 0,
            workspace_mutation_scope_id: "workspace-omni".to_owned(),
        }))
        .await
        .unwrap()
        .into_inner();
    assert!(operation.accepted);
    assert_eq!(
        operation.payload_schema,
        "omnisolo.harness.native_session.v1"
    );
    let native: server_harness::middleware::harness::NativeSession =
        serde_json::from_slice(&operation.payload).unwrap();
    assert_eq!(native.native_session_id, format!("omnisolo:{session_id}"));

    let mut stream = service
        .attempt_command(Request::new(AttemptCommandEnvelope {
            protocol_version: 1,
            tenant_id: "tenant-omni".to_owned(),
            session_id: session_id.to_string(),
            task_id: task_id.to_string(),
            attempt_id: uuid::Uuid::new_v4().to_string(),
            turn_id: String::new(),
            command_id: uuid::Uuid::new_v4().to_string(),
            lease_id: uuid::Uuid::new_v4().to_string(),
            lease_generation: 1,
            fencing_token: "attempt-fence".to_owned(),
            kind: "execute".to_owned(),
            correlation_id: String::new(),
            idempotency_key: "omni-execute".to_owned(),
            payload_schema: "omnisolo.attempt.command.v1".to_owned(),
            payload_version: 1,
            payload: serde_json::to_vec(&serde_json::json!({
                "prompt": "native output"
            }))
            .unwrap(),
            extensions: Default::default(),
            worker_id: "worker-omni".to_owned(),
            harness_id: "omnisolo".to_owned(),
            capability_version: 1,
            binding_id: String::new(),
            binding_generation: 0,
        }))
        .await
        .unwrap()
        .into_inner();
    let event = stream.next().await.unwrap().unwrap();
    assert_eq!(event.payload_schema, "omnisolo.harness.event.v1");
    assert!(
        String::from_utf8(event.payload)
            .unwrap()
            .contains("native output")
    );
}

#[tokio::test]
async fn grpc_worker_service_releases_fences_after_validation_and_adapter_failures() {
    let rejecting_script = r#"
while IFS= read -r line; do
  request_id=$(printf '%s' "$line" | sed -n 's/.*"request_id":"\([^"]*\)".*/\1/p')
  printf '{"request_id":"%s","ok":false}\n' "$request_id"
done
"#;
    let service = HarnessWorkerGrpcService::with_process_spec(
        ProcessHarnessSpec::command("/bin/sh", ["-c", rejecting_script], "codex")
            .with_protocol(server_harness::middleware::harness::HarnessProtocolKind::Custom),
    );

    let health_error = service
        .health(Request::new(WorkerHealthRequest {
            protocol_version: 1,
            worker_id: "wrong-worker".to_owned(),
            harness_id: "codex".to_owned(),
            pool_id: "codex".to_owned(),
        }))
        .await
        .unwrap_err();
    assert_eq!(health_error.code(), tonic::Code::NotFound);

    let session_id = uuid::Uuid::new_v4().to_string();
    let task_id = uuid::Uuid::new_v4().to_string();
    let mut resume = SessionOperationEnvelope {
        protocol_version: 1,
        tenant_id: "tenant-failure".to_owned(),
        session_id: session_id.clone(),
        operation_id: uuid::Uuid::new_v4().to_string(),
        operation_generation: 1,
        fencing_token: "resume-fence".to_owned(),
        kind: "resume".to_owned(),
        task_id: task_id.clone(),
        correlation_id: String::new(),
        idempotency_key: "resume-failure".to_owned(),
        payload_schema: "omnisolo.session.operation.v1".to_owned(),
        payload_version: 1,
        payload: serde_json::to_vec(&serde_json::json!({})).unwrap(),
        extensions: Default::default(),
        worker_id: "worker-1".to_owned(),
        pool_id: "codex".to_owned(),
        harness_id: "codex".to_owned(),
        capability_version: 1,
        binding_id: String::new(),
        binding_generation: 0,
        workspace_mutation_scope_id: "workspace-failure".to_owned(),
    };
    let invalid_resume = service
        .session_operation(Request::new(resume.clone()))
        .await
        .unwrap_err();
    assert_eq!(invalid_resume.code(), tonic::Code::InvalidArgument);
    resume.payload = serde_json::to_vec(&serde_json::json!({
        "native_session_id": "native"
    }))
    .unwrap();
    let remote_resume = service
        .session_operation(Request::new(resume))
        .await
        .unwrap_err();
    assert_eq!(remote_resume.code(), tonic::Code::Internal);

    let session_id = uuid::Uuid::new_v4().to_string();
    let task_id = uuid::Uuid::new_v4().to_string();
    let command = AttemptCommandEnvelope {
        protocol_version: 1,
        tenant_id: "tenant-failure".to_owned(),
        session_id,
        task_id,
        attempt_id: uuid::Uuid::new_v4().to_string(),
        turn_id: String::new(),
        command_id: uuid::Uuid::new_v4().to_string(),
        lease_id: uuid::Uuid::new_v4().to_string(),
        lease_generation: 1,
        fencing_token: "attempt-failure-fence".to_owned(),
        kind: "execute".to_owned(),
        correlation_id: String::new(),
        idempotency_key: String::new(),
        payload_schema: "omnisolo.attempt.command.v1".to_owned(),
        payload_version: 1,
        payload: serde_json::to_vec(&serde_json::json!({"prompt": "fail"})).unwrap(),
        extensions: Default::default(),
        worker_id: "worker-1".to_owned(),
        harness_id: "codex".to_owned(),
        capability_version: 1,
        binding_id: String::new(),
        binding_generation: 0,
    };
    let command_error = service
        .attempt_command(Request::new(command.clone()))
        .await
        .unwrap_err();
    assert_eq!(command_error.code(), tonic::Code::Internal);
    let retry_error = service
        .attempt_command(Request::new(command))
        .await
        .unwrap_err();
    assert_eq!(retry_error.code(), tonic::Code::Internal);

    let exchange = WorkerExchangeEnvelope {
        protocol_version: 1,
        tenant_id: "tenant-failure".to_owned(),
        session_id: uuid::Uuid::new_v4().to_string(),
        task_id: uuid::Uuid::new_v4().to_string(),
        attempt_id: uuid::Uuid::new_v4().to_string(),
        worker_id: "worker-1".to_owned(),
        harness_id: "codex".to_owned(),
        pool_id: "codex".to_owned(),
        lease_id: uuid::Uuid::new_v4().to_string(),
        lease_generation: 1,
        fencing_token: "exchange-failure-fence".to_owned(),
        kind: "artifact".to_owned(),
        idempotency_key: "exchange-failure".to_owned(),
        payload_schema: "omnisolo.exchange.v1".to_owned(),
        payload_version: 1,
        payload: serde_json::to_vec(&serde_json::json!({"artifact": true})).unwrap(),
        artifact_digest: String::new(),
        extensions: Default::default(),
    };
    let exchange_error = service
        .interaction(Request::new(exchange.clone()))
        .await
        .unwrap_err();
    assert_eq!(exchange_error.code(), tonic::Code::Internal);
    let exchange_retry_error = service
        .interaction(Request::new(exchange))
        .await
        .unwrap_err();
    assert_eq!(exchange_retry_error.code(), tonic::Code::Internal);
}

fn native_session_lifecycle_script() -> &'static str {
    r#"
thread_count=0
while IFS= read -r line; do
  if printf '%s' "$line" | grep -qE '"request_id"|"protocol_version"|"jsonrpc"'; then exit 31; fi
  id=$(printf '%s' "$line" | sed -n 's/.*"id":\([0-9][0-9]*\).*/\1/p')
  method=$(printf '%s' "$line" | sed -n 's/.*"method":"\([^" ]*\)".*/\1/p')
  thread_id=$(printf '%s' "$line" | sed -n 's/.*"threadId":"\([^"]*\)".*/\1/p')
  case "$method" in
    initialize) printf '{"id":%s,"result":{"capabilities":{}}}\n' "$id" ;;
    initialized) : ;;
    thread/start)
      thread_count=$((thread_count + 1)); thread_id="thread-$thread_count"
      printf '{"id":%s,"result":{"thread":{"id":"%s"}}}\n' "$id" "$thread_id" ;;
    thread/inject_items)
      printf '%s' "$line" | grep -q '"source":"omnisolo.historical"' || exit 32
      printf '{"id":%s,"result":{}}\n' "$id" ;;
    thread/resume|thread/read)
      printf '{"id":%s,"result":{"thread":{"id":"%s"}}}\n' "$id" "$thread_id" ;;
    thread/fork)
      printf '{"id":%s,"result":{"thread":{"id":"thread-fork"}}}\n' "$id" ;;
    thread/archive|thread/delete)
      if printf '%s' "$line" | grep -qE '"approvalPolicy"|"sandbox"|"sandboxPolicy"'; then exit 33; fi
      printf '{"id":%s,"result":{}}\n' "$id" ;;
    *) printf '{"id":%s,"result":{}}\n' "$id" ;;
  esac
done
"#
}

#[allow(dead_code)]
fn service_is_tonic_server_compatible<T: HarnessWorkerService>(
    service: T,
) -> server_omnisolo::harness_middleware::harness_worker_service_server::HarnessWorkerServiceServer<T> {
    server_omnisolo::harness_middleware::harness_worker_service_server::HarnessWorkerServiceServer::new(
        service,
    )
}

#[allow(dead_code)]
fn status_is_sendable(_: Status) {}
