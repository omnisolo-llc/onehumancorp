use serde_json::{Value, json};
use server_harness::middleware::local_services::{LocalServiceRegistry, LocalServiceScopeContext};
use server_ohc::harness_middleware::harness_worker_service_client::HarnessWorkerServiceClient;
use server_ohc::harness_middleware::{
    AttemptCommandEnvelope, EventDeliveryEnvelope, SessionOperationEnvelope,
};
use tokio::time::{Duration, sleep, timeout};
use tokio_stream::StreamExt;
use tonic::Request;
use uuid::Uuid;

const MARKER: &str = "OMNISOLO_LIVE_HARNESS_OK";

#[tokio::test]
#[ignore = "requires OMNISOLO_LIVE_HARNESS_E2E=1 and a real configured worker"]
async fn live_harness_worker_uses_the_real_provider() {
    if std::env::var("OMNISOLO_LIVE_HARNESS_E2E").as_deref() != Ok("1") {
        eprintln!("live harness verification skipped: explicit live opt-in is absent");
        return;
    }

    timeout(Duration::from_secs(300), run())
        .await
        .unwrap_or_else(|_| panic!("live harness verification exceeded its five-minute deadline"));
}

async fn run() {
    let harness_id = required_env("OMNISOLO_LIVE_HARNESS_ID");
    let endpoint = required_env("OMNISOLO_LIVE_HARNESS_ENDPOINT");
    let worker_id = format!("harness-{harness_id}");
    let mut client = connect_when_ready(&endpoint).await;
    let session_id = Uuid::new_v4();
    let task_id = Uuid::new_v4();

    let create = client
        .session_operation(Request::new(session_operation(
            &worker_id,
            &harness_id,
            session_id,
            task_id,
            "create",
            serde_json::json!({
                "objective": format!("verify the {harness_id} native harness through OmniSolo")
            }),
        )))
        .await
        .expect("create live native session")
        .into_inner();
    assert!(create.accepted, "{harness_id} rejected session creation");
    let native: server_harness::middleware::harness::NativeSession =
        serde_json::from_slice(&create.payload).expect("decode native session");
    assert!(
        !native.native_session_id.trim().is_empty(),
        "{harness_id} returned an empty native session id"
    );

    let attempt_id = Uuid::new_v4();
    let local_services = LocalServiceRegistry::with_defaults()
        .resolve(live_service_scope(&harness_id, session_id, task_id, attempt_id))
        .expect("resolve shared live local services");
    let prompt =
        format!("Reply with exactly this marker and no markdown: {MARKER}. Do not call tools.");
    let mut stream = client
        .attempt_command(Request::new(AttemptCommandEnvelope {
            protocol_version: 1,
            tenant_id: "tenant-live-harness-matrix".to_owned(),
            session_id: session_id.to_string(),
            task_id: task_id.to_string(),
            attempt_id: attempt_id.to_string(),
            turn_id: String::new(),
            command_id: Uuid::new_v4().to_string(),
            lease_id: Uuid::new_v4().to_string(),
            lease_generation: 1,
            fencing_token: format!("live-{harness_id}-attempt-fence"),
            kind: "execute".to_owned(),
            correlation_id: String::new(),
            idempotency_key: format!("live-{harness_id}-attempt"),
            payload_schema: "omnisolo.attempt.command.v1".to_owned(),
            payload_version: 1,
            payload: serde_json::to_vec(&serde_json::json!({
                "prompt": prompt,
                "native_session_id": native.native_session_id.clone(),
                "context": {
                    "local_services": local_services,
                },
            }))
            .expect("serialize attempt"),
            extensions: Default::default(),
            worker_id: worker_id.clone(),
            harness_id: harness_id.clone(),
            capability_version: 1,
            binding_id: String::new(),
            binding_generation: 0,
        }))
        .await
        .expect("execute live native attempt")
        .into_inner();

    let mut deliveries = Vec::new();
    while let Some(delivery) = stream.next().await {
        deliveries.push(delivery.expect("receive live harness event"));
    }
    let model = required_env("OPENAI_MODEL");
    let reasoning_effort = required_env("OPENAI_REASONING_EFFORT");
    let api_key = required_env("OPENAI_API_KEY");
    let evidence = validate_live_deliveries(
        &harness_id,
        &model,
        &reasoning_effort,
        session_id,
        task_id,
        attempt_id,
        &deliveries,
        &api_key,
        true,
    )
    .unwrap_or_else(|error| panic!("{harness_id} live evidence validation failed: {error}"));

    let delete = client
        .session_operation(Request::new(session_operation(
            &worker_id,
            &harness_id,
            session_id,
            task_id,
            "delete",
            serde_json::json!({"native_session_id": native.native_session_id}),
        )))
        .await
        .expect("delete live native session")
        .into_inner();
    assert!(delete.accepted, "{harness_id} rejected session deletion");
    println!(
        "OMNISOLO_LIVE_RESULT={}",
        json!({
            "schema": "omnisolo.live_harness_result.v1",
            "status": "passed",
            "harness_id": harness_id,
            "model": model,
            "reasoning_effort": reasoning_effort,
            "native_session_deleted": true,
            "evidence": evidence,
        })
    );
}

#[allow(clippy::too_many_arguments)]
fn validate_live_deliveries(
    harness_id: &str,
    model: &str,
    reasoning_effort: &str,
    session_id: Uuid,
    task_id: Uuid,
    attempt_id: Uuid,
    deliveries: &[EventDeliveryEnvelope],
    api_key: &str,
    require_local_services: bool,
) -> Result<Value, String> {
    if deliveries.is_empty() {
        return Err("worker returned no event deliveries".to_owned());
    }
    let mut event_types = Vec::with_capacity(deliveries.len());
    let mut binding = None;
    let mut downgrade = None;
    let mut local_services = None;
    let mut usages = Vec::new();
    let mut assistant_text = String::new();
    let mut saw_terminal_success = false;
    let mut last_durable_sequence = 0_i64;

    for (index, delivery) in deliveries.iter().enumerate() {
        let expected_delivery_sequence = (index + 1) as i64;
        if delivery.delivery_sequence != expected_delivery_sequence {
            return Err(format!(
                "delivery sequence {} was {}, expected {expected_delivery_sequence}",
                index + 1,
                delivery.delivery_sequence
            ));
        }
        if delivery.session_id != session_id.to_string()
            || delivery.task_id != task_id.to_string()
            || delivery.source_attempt_id != attempt_id.to_string()
            || delivery.ingest_attempt_id != attempt_id.to_string()
        {
            return Err("delivery correlation did not match the live attempt".to_owned());
        }
        if delivery.payload_schema != "omnisolo.harness.event.v1" || delivery.payload_version != 1 {
            return Err("delivery did not use the canonical harness event schema".to_owned());
        }
        if delivery.durable_sequence > 0 {
            if delivery.durable_sequence != last_durable_sequence + 1 {
                return Err("durable event sequence was not contiguous".to_owned());
            }
            last_durable_sequence = delivery.durable_sequence;
        }

        let payload: Value = serde_json::from_slice(&delivery.payload)
            .map_err(|error| format!("event payload was not JSON: {error}"))?;
        if !api_key.is_empty() && payload.to_string().contains(api_key) {
            return Err("provider credential leaked into a canonical event".to_owned());
        }
        let event_type = payload
            .get("event_type")
            .and_then(Value::as_str)
            .ok_or_else(|| "canonical event did not contain event_type".to_owned())?;
        event_types.push(event_type.to_owned());
        let event_payload = payload.get("payload").cloned().unwrap_or(Value::Null);
        if matches!(
            event_type,
            "turn.failed" | "turn.cancelled" | "attempt.failed" | "error"
        ) || event_payload
            .get("status")
            .and_then(Value::as_str)
            .is_some_and(|status| {
                matches!(status, "failed" | "error" | "cancelled" | "interrupted")
            })
            || event_payload
                .pointer("/turn/status")
                .and_then(Value::as_str)
                .is_some_and(|status| {
                    matches!(status, "failed" | "error" | "cancelled" | "interrupted")
                })
        {
            return Err("attempt emitted a failed or cancelled terminal event".to_owned());
        }
        if matches!(
            event_type,
            "assistant.final" | "assistant.text_chunk" | "assistant.message"
        ) {
            for field in ["/text", "/content", "/message/content", "/item/text"] {
                if let Some(content) = event_payload.pointer(field) {
                    append_assistant_text(content, &mut assistant_text);
                    break;
                }
            }
        }
        let usage = event_payload
            .get("usage")
            .or_else(|| (event_type == "usage.recorded").then_some(&event_payload));
        if let Some(usage) = usage.filter(|value| has_token_usage(value)) {
            usages.push(usage.clone());
        }
        saw_terminal_success |= matches!(event_type, "turn.completed" | "assistant.final");
        if event_type == "inference.model_binding" {
            if binding.is_some() {
                return Err("attempt emitted more than one model binding event".to_owned());
            }
            binding = Some(event_payload);
        } else if event_type == "local_services.bound" {
            if local_services.is_some() {
                return Err("attempt emitted more than one local service binding event".to_owned());
            }
            local_services = Some(event_payload);
        } else if event_type == "capability.downgraded"
            && event_payload.get("capability").and_then(Value::as_str) == Some("reasoning_effort")
        {
            downgrade = Some(event_payload);
        }
    }

    let binding =
        binding.ok_or_else(|| "attempt emitted no model binding provenance".to_owned())?;
    let expected_integration_mode = match harness_id {
        "aider" | "goose" | "open-interpreter" | "plandex" => "openai_compatible",
        _ => "native",
    };
    if binding.get("integration_mode").and_then(Value::as_str) != Some(expected_integration_mode) {
        return Err(format!(
            "model binding integration mode did not match {harness_id}: expected {expected_integration_mode}"
        ));
    }
    let shared_services = if require_local_services {
        let event = local_services
            .as_ref()
            .ok_or_else(|| "attempt emitted no shared local service binding".to_owned())?;
        if event.get("schema").and_then(Value::as_str) != Some("omnisolo.local_service_bundle.v1") {
            return Err("local service binding used an unexpected schema".to_owned());
        }
        let bindings = event
            .get("bindings")
            .and_then(Value::as_array)
            .ok_or_else(|| "local service binding did not contain bindings".to_owned())?;
        if bindings.len() != 8 {
            return Err(format!(
                "local service binding contained {}, expected 8 services",
                bindings.len()
            ));
        }
        let service_ids = bindings
            .iter()
            .filter_map(|binding| binding.get("service_id").and_then(Value::as_str))
            .collect::<std::collections::BTreeSet<_>>();
        let expected = [
            "omnisolo.artifact",
            "omnisolo.browser",
            "omnisolo.cache",
            "omnisolo.integration",
            "omnisolo.mcp",
            "omnisolo.memory",
            "omnisolo.provider_facade",
            "omnisolo.workspace",
        ]
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>();
        if service_ids != expected {
            return Err(
                "shared local service binding did not cover the full service set".to_owned(),
            );
        }
        json!({
            "status": "bound",
            "binding_count": bindings.len(),
            "memory_service_id": "omnisolo.memory",
            "memory_scope": "workspace",
        })
    } else {
        Value::Null
    };
    if binding.get("model_id").and_then(Value::as_str) != Some(model) {
        return Err("model binding did not preserve OPENAI_MODEL".to_owned());
    }
    if binding.get("reasoning_effort").and_then(Value::as_str) != Some(reasoning_effort) {
        return Err("model binding did not preserve OPENAI_REASONING_EFFORT".to_owned());
    }
    if !assistant_text.contains(MARKER) {
        return Err("provider marker was absent from the final transcript".to_owned());
    }
    if !saw_terminal_success {
        return Err("attempt emitted no canonical successful terminal event".to_owned());
    }
    if usages.is_empty() {
        return Err("attempt emitted no usage evidence".to_owned());
    }

    let reasoning_translation = downgrade.as_ref().map_or_else(
        || json!({"kind":"native","requested":reasoning_effort,"effective":
            if harness_id == "pi" && reasoning_effort == "max" { "xhigh" } else { reasoning_effort }}),
        |event| {
            json!({
                "kind":"explicit_downgrade",
                "requested":event.get("requested"),
                "effective":event.get("effective").or_else(|| event.get("applied")),
                "reason":event.get("reason"),
            })
        },
    );
    Ok(json!({
        "harness_id": harness_id,
        "event_types": event_types,
        "delivery_count": deliveries.len(),
        "last_durable_sequence": last_durable_sequence,
        "model_binding": binding,
        "integration_mode": expected_integration_mode,
        "shared_local_services": shared_services,
        "reasoning_translation": reasoning_translation,
        "assistant_text": assistant_text,
        "usage": usages,
        "usage_observed": true,
        "terminal_success_observed": true,
        "provider_marker_observed": true,
        "credential_leak_observed": false,
    }))
}

fn append_assistant_text(content: &Value, text: &mut String) {
    match content {
        Value::String(value) => text.push_str(value),
        Value::Array(parts) => {
            for part in parts {
                if matches!(
                    part.get("type").and_then(Value::as_str),
                    Some("text" | "output_text")
                ) {
                    if let Some(value) = part.get("text").and_then(Value::as_str) {
                        text.push_str(value);
                    }
                }
            }
        }
        _ => {}
    }
}

fn live_service_scope(
    harness_id: &str,
    session_id: Uuid,
    task_id: Uuid,
    attempt_id: Uuid,
) -> LocalServiceScopeContext {
    LocalServiceScopeContext::for_attempt(
        "tenant-live-harness-matrix",
        Some("project-live-harness-matrix"),
        Some(&format!("workspace-live-{harness_id}")),
        session_id,
        Some(task_id),
        Some(attempt_id),
    )
}

#[test]
fn live_scope_resolves_every_required_service_before_provider_execution() {
    let scope = live_service_scope("codex", Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
    let registry = LocalServiceRegistry::with_defaults();
    let bundle = registry.resolve(scope.clone()).expect("live scope is complete");
    assert_eq!(bundle.bindings.len(), 8);
    registry.validate(&bundle, &scope).unwrap();
}

fn has_token_usage(value: &Value) -> bool {
    let Some(object) = value.as_object() else {
        return false;
    };
    object.iter().any(|(key, value)| {
        matches!(
            key.as_str(),
            "input_tokens"
                | "output_tokens"
                | "total_tokens"
                | "prompt_tokens"
                | "completion_tokens"
                | "inputTokens"
                | "outputTokens"
                | "totalTokens"
                | "input"
                | "output"
                | "cacheRead"
                | "cacheWrite"
        ) && value.as_u64().is_some_and(|count| count > 0)
    }) || ["total", "last", "token_usage", "tokens", "usage"]
        .iter()
        .any(|key| object.get(*key).is_some_and(has_token_usage))
}

#[test]
fn live_evidence_extracts_native_text_and_requires_positive_token_counts() {
    let mut text = String::new();
    append_assistant_text(
        &json!([
            {"type":"text", "text":"OMNISOLO_"},
            {"type":"thinking", "text":"not assistant output"},
            {"type":"output_text", "text":"LIVE_HARNESS_OK"}
        ]),
        &mut text,
    );
    assert_eq!(text, MARKER);
    for usage in [
        json!({"total":{"inputTokens":4}}),
        json!({"input":2}),
        json!({"prompt_tokens":1}),
        json!({"tokens":{"output":3}}),
    ] {
        assert!(has_token_usage(&usage));
    }
    for usage in [
        Value::Null,
        json!({}),
        json!({"total_tokens":0}),
        json!({"total_tokens":-1}),
        json!({"total_tokens":"3"}),
        json!({"unrelated_number":3}),
    ] {
        assert!(!has_token_usage(&usage));
    }
}

#[test]
fn live_evidence_requires_ordered_model_usage_terminal_and_secret_safe_events() {
    let session_id = Uuid::new_v4();
    let task_id = Uuid::new_v4();
    let attempt_id = Uuid::new_v4();
    let event = |delivery_sequence: i64,
                 durable_sequence: i64,
                 event_type: &str,
                 payload: Value| EventDeliveryEnvelope {
        protocol_version: 1,
        tenant_id: "tenant-live-test".to_owned(),
        session_id: session_id.to_string(),
        task_id: task_id.to_string(),
        turn_id: String::new(),
        source_attempt_id: attempt_id.to_string(),
        ingest_attempt_id: attempt_id.to_string(),
        event_id: Uuid::new_v4().to_string(),
        durable_sequence,
        delivery_stream_id: "stream-live-test".to_owned(),
        delivery_sequence,
        payload_schema: "omnisolo.harness.event.v1".to_owned(),
        payload_version: 1,
        payload: serde_json::to_vec(&json!({
            "event_type": event_type,
            "payload": payload,
            "native_cursor": null,
        }))
        .unwrap(),
        lease_generation: 1,
        fencing_token: "fence-live-test".to_owned(),
    };
    let valid = vec![
        event(
            1,
            1,
            "inference.model_binding",
            json!({
                "model_id":"gpt-5.6-luna",
                "reasoning_effort":"max",
                "integration_mode":"native"
            }),
        ),
        event(2, 2, "usage.recorded", json!({"total_tokens":3})),
        event(3, 3, "assistant.final", json!({"text":MARKER})),
    ];
    let evidence = validate_live_deliveries(
        "pi",
        "gpt-5.6-luna",
        "max",
        session_id,
        task_id,
        attempt_id,
        &valid,
        "secret-canary",
        false,
    )
    .unwrap();
    assert_eq!(evidence["usage_observed"], true);
    assert_eq!(evidence["assistant_text"], MARKER);
    assert_eq!(evidence["usage"], json!([{"total_tokens":3}]));
    assert_eq!(evidence["reasoning_translation"]["effective"], "xhigh");
    let mut downgraded = valid.clone();
    downgraded.push(event(
        4,
        4,
        "capability.downgraded",
        json!({
            "capability":"reasoning_effort", "requested":"max", "applied":"high",
            "reason":"SDK supports high"
        }),
    ));
    let evidence = validate_live_deliveries(
        "openhands",
        "gpt-5.6-luna",
        "max",
        session_id,
        task_id,
        attempt_id,
        &downgraded,
        "secret-canary",
        false,
    )
    .unwrap();
    assert_eq!(evidence["reasoning_translation"]["effective"], "high");
    assert_eq!(
        evidence["reasoning_translation"]["kind"],
        "explicit_downgrade"
    );

    for (name, mutation) in [
        ("delivery order", 0_u8),
        ("missing binding", 1),
        ("missing usage", 2),
        ("missing terminal", 3),
        ("credential leak", 4),
        ("prompt echo is not assistant output", 5),
        ("null usage", 6),
        ("failed terminal", 7),
    ] {
        let mut invalid = valid.clone();
        match mutation {
            0 => invalid[1].delivery_sequence = 9,
            1 => {
                invalid[0].payload = serde_json::to_vec(&json!({
                    "event_type":"turn.started","payload":{},"native_cursor":null
                }))
                .unwrap()
            }
            2 => {
                invalid.remove(1);
            }
            3 => {
                invalid[2].payload = serde_json::to_vec(&json!({
                    "event_type":"assistant.text_chunk","payload":{"text":MARKER},
                    "native_cursor":null
                }))
                .unwrap()
            }
            4 => {
                invalid[2].payload = serde_json::to_vec(&json!({
                    "event_type":"assistant.final",
                    "payload":{"text":format!("{MARKER} secret-canary")},
                    "native_cursor":null
                }))
                .unwrap()
            }
            5 => {
                invalid[0].payload = serde_json::to_vec(&json!({
                    "event_type":"inference.model_binding", "payload":{
                        "model_id":"gpt-5.6-luna", "reasoning_effort":"max",
                        "integration_mode":"native", "prompt":MARKER
                    }
                }))
                .unwrap();
                invalid[2].payload = serde_json::to_vec(&json!({
                    "event_type":"assistant.final", "payload":{"text":"wrong answer"}
                }))
                .unwrap();
            }
            6 => {
                invalid[1].payload = serde_json::to_vec(&json!({
                    "event_type":"usage.recorded", "payload":{"usage":null}
                }))
                .unwrap()
            }
            7 => invalid.push(event(
                4,
                4,
                "turn.failed",
                json!({"error":"provider failed"}),
            )),
            _ => unreachable!(),
        }
        assert!(
            validate_live_deliveries(
                "pi",
                "gpt-5.6-luna",
                "max",
                session_id,
                task_id,
                attempt_id,
                &invalid,
                "secret-canary",
                false,
            )
            .is_err(),
            "{name} must be rejected"
        );
    }
}

async fn connect_when_ready(
    endpoint: &str,
) -> HarnessWorkerServiceClient<tonic::transport::Channel> {
    timeout(Duration::from_secs(180), async {
        loop {
            if let Ok(client) = HarnessWorkerServiceClient::connect(endpoint.to_owned()).await {
                return client;
            }
            sleep(Duration::from_millis(250)).await;
        }
    })
    .await
    .unwrap_or_else(|_| panic!("live harness worker did not become ready at {endpoint}"))
}

fn session_operation(
    worker_id: &str,
    harness_id: &str,
    session_id: Uuid,
    task_id: Uuid,
    kind: &str,
    payload: serde_json::Value,
) -> SessionOperationEnvelope {
    SessionOperationEnvelope {
        protocol_version: 1,
        tenant_id: "tenant-live-harness-matrix".to_owned(),
        session_id: session_id.to_string(),
        operation_id: Uuid::new_v4().to_string(),
        operation_generation: 1,
        fencing_token: format!("live-{harness_id}-{kind}-fence"),
        kind: kind.to_owned(),
        task_id: task_id.to_string(),
        correlation_id: String::new(),
        idempotency_key: format!("live-{harness_id}-{kind}-{}", Uuid::new_v4()),
        payload_schema: "omnisolo.session.operation.v1".to_owned(),
        payload_version: 1,
        payload: serde_json::to_vec(&payload).expect("serialize session operation"),
        extensions: Default::default(),
        worker_id: worker_id.to_owned(),
        pool_id: harness_id.to_owned(),
        harness_id: harness_id.to_owned(),
        capability_version: 1,
        binding_id: String::new(),
        binding_generation: 0,
        workspace_mutation_scope_id: format!("workspace-live-{harness_id}"),
    }
}

fn required_env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{name} is required"))
}
