use serde_json::{Value, json};
use server_ohc::harness_middleware::harness_worker_service_client::HarnessWorkerServiceClient;
use server_ohc::harness_middleware::{
    AttemptCommandEnvelope, EventDeliveryEnvelope, SessionOperationEnvelope,
};
use server_harness::middleware::local_services::{
    LocalServiceRegistry, LocalServiceScopeContext,
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
        .resolve(LocalServiceScopeContext::for_attempt(
            "tenant-live-harness-matrix",
            None,
            Some(&format!("workspace-live-{harness_id}")),
            session_id,
            Some(task_id),
            Some(attempt_id),
        ))
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
    let mut saw_usage = false;
    let mut saw_final_marker = false;
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
        saw_final_marker |= event_payload.to_string().contains(MARKER);
        saw_usage |= event_type == "usage.recorded" || event_payload.get("usage").is_some();
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
    if binding.get("integration_mode").and_then(Value::as_str)
        != Some(expected_integration_mode)
    {
        return Err(format!(
            "model binding integration mode did not match {harness_id}: expected {expected_integration_mode}"
        ));
    }
    let shared_services = if require_local_services {
        let event = local_services
            .as_ref()
            .ok_or_else(|| "attempt emitted no shared local service binding".to_owned())?;
        if event.get("schema").and_then(Value::as_str)
            != Some("omnisolo.local_service_bundle.v1")
        {
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
            return Err("shared local service binding did not cover the full service set".to_owned());
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
    if !saw_final_marker {
        return Err("provider marker was absent from the final transcript".to_owned());
    }
    if !saw_terminal_success {
        return Err("attempt emitted no canonical successful terminal event".to_owned());
    }
    if !saw_usage {
        return Err("attempt emitted no usage evidence".to_owned());
    }

    let reasoning_translation = downgrade.as_ref().map_or_else(
        || json!({"kind":"native","requested":reasoning_effort,"effective":reasoning_effort}),
        |event| {
            json!({
                "kind":"explicit_downgrade",
                "requested":event.get("requested"),
                "effective":event.get("effective"),
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
        "usage_observed": true,
        "terminal_success_observed": true,
        "provider_marker_observed": true,
        "credential_leak_observed": false,
    }))
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
    assert_eq!(evidence["reasoning_translation"]["kind"], "native");

    for (name, mutation) in [
        ("delivery order", 0_u8),
        ("missing binding", 1),
        ("missing usage", 2),
        ("missing terminal", 3),
        ("credential leak", 4),
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
