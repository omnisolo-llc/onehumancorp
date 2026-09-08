use std::collections::BTreeSet;

use server_harness::middleware::adapter::{OmniSoloHarnessAdapter, OmniSoloRunConfig};
use server_harness::middleware::codex_app_server::CodexAppServerV2Codec;
use server_harness::middleware::harness::{
    HarnessAdapterError, HarnessSessionRequest, NativeSession,
};
use server_harness::middleware::json_rpc::JsonRpcNotification;
use server_harness::middleware::protocol::{
    AttemptOperation, HarnessProtocolCodec, NativeTurnState, SessionOperation,
};
use server_harness::middleware::types::{ModelApiDialect, ReasoningEffort, ResolvedModelSelection};
use uuid::Uuid;

fn session_request() -> HarnessSessionRequest {
    HarnessSessionRequest::new("tenant", Uuid::new_v4(), Uuid::new_v4())
        .with_task(Uuid::new_v4(), "objective")
}

fn attempt_request() -> HarnessSessionRequest {
    HarnessSessionRequest::new("tenant", Uuid::new_v4(), Uuid::new_v4())
        .with_task(Uuid::new_v4(), "prompt")
}

fn resolved_model(reasoning_effort: Option<ReasoningEffort>) -> ResolvedModelSelection {
    ResolvedModelSelection {
        provider_route: "openai-compatible".to_owned(),
        model_id: "gpt-5.6-luna".to_owned(),
        reasoning_effort,
        api_dialect: ModelApiDialect::OpenAiResponses,
        context_window: None,
        max_output_tokens: None,
        capabilities: BTreeSet::from(["reasoning".to_owned(), "tools".to_owned()]),
        binding_revision: "binding-v1".to_owned(),
        binding_digest: "sha256:test".to_owned(),
        metadata: Default::default(),
    }
}

fn capsule() -> server_harness::middleware::capsule::SessionCapsule {
    let session_id = Uuid::new_v4();
    OmniSoloHarnessAdapter::start(
        OmniSoloRunConfig::new("tenant", "portable objective")
            .with_session_id(session_id)
            .with_task_id(Uuid::new_v4()),
    )
    .unwrap()
    .export_capsule("codex", Uuid::new_v4())
    .unwrap()
}

#[test]
fn codex_initialize_uses_v2_client_identity_and_capabilities() {
    let request = CodexAppServerV2Codec::new().initialize_request();
    assert_eq!(request.method, "initialize");
    assert_eq!(request.params["clientInfo"]["name"], "omnisolo");
    assert!(request.params.get("capabilities").is_some());
}

#[test]
fn codex_session_operations_use_native_thread_methods() {
    let codec = CodexAppServerV2Codec::new();
    let request = codec
        .session_request(SessionOperation::Create, &session_request())
        .unwrap();
    assert_eq!(request.method, "thread/start");
    assert_eq!(request.params["ephemeral"], false);

    let request = codec
        .session_request(
            SessionOperation::Resume {
                native_session_id: "thread-1".to_owned(),
            },
            &session_request(),
        )
        .unwrap();
    assert_eq!(request.method, "thread/resume");
    assert_eq!(request.params["threadId"], "thread-1");

    let request = codec
        .session_request(
            SessionOperation::Fork {
                native_session_id: Some("thread-1".to_owned()),
            },
            &session_request(),
        )
        .unwrap();
    assert_eq!(request.method, "thread/fork");
    assert_eq!(request.params["threadId"], "thread-1");

    for (operation, method) in [
        (SessionOperation::Snapshot, "thread/read"),
        (SessionOperation::Close, "thread/archive"),
        (SessionOperation::Delete, "thread/delete"),
    ] {
        let mut request_context = session_request();
        request_context.extensions.insert(
            "native_session_id".to_owned(),
            serde_json::Value::String("thread-1".to_owned()),
        );
        let request = codec.session_request(operation, &request_context).unwrap();
        assert_eq!(request.method, method);
    }
}

#[test]
fn codex_options_are_forwarded_only_to_operations_that_accept_them() {
    let codec = CodexAppServerV2Codec::new();
    let mut request_context = session_request();
    request_context.extensions.insert(
        "codex.cwd".to_owned(),
        serde_json::Value::String("/workspace".to_owned()),
    );
    request_context.extensions.insert(
        "codex.model".to_owned(),
        serde_json::Value::String("gpt-5-codex".to_owned()),
    );
    request_context.extensions.insert(
        "codex.model_provider".to_owned(),
        serde_json::Value::String("openai".to_owned()),
    );
    request_context.extensions.insert(
        "codex.personality".to_owned(),
        serde_json::Value::String("focused".to_owned()),
    );
    request_context.extensions.insert(
        "codex.reasoning_effort".to_owned(),
        serde_json::Value::String("high".to_owned()),
    );

    let start = codec
        .session_request(SessionOperation::Create, &request_context)
        .unwrap();
    assert_eq!(start.params["cwd"], "/workspace");
    assert_eq!(start.params["modelProvider"], "openai");
    assert_eq!(start.params["sandbox"], "read-only");

    let turn = codec
        .attempt_request(
            AttemptOperation::Execute,
            &request_context,
            "prompt",
            Some("thread-1"),
            None,
        )
        .unwrap();
    assert_eq!(turn.params["sandboxPolicy"]["type"], "readOnly");
    assert_eq!(turn.params["effort"], "high");
    assert!(turn.params.get("text_elements").is_none());

    let read = codec
        .session_request(
            SessionOperation::Snapshot,
            &HarnessSessionRequest {
                extensions: [(
                    "native_session_id".to_owned(),
                    serde_json::json!("thread-1"),
                )]
                .into_iter()
                .collect(),
                ..request_context
            },
        )
        .unwrap();
    assert_eq!(
        read.params,
        serde_json::json!({
            "threadId": "thread-1",
            "includeTurns": true,
        })
    );
}

#[test]
fn codex_typed_model_selection_wins_and_uses_exact_v2_request_shapes() {
    let codec = CodexAppServerV2Codec::new();
    let mut selection = resolved_model(Some(ReasoningEffort::Max));
    selection.metadata.insert(
        "api_key".to_owned(),
        serde_json::json!("typed-secret-canary"),
    );
    let mut request = session_request().with_resolved_model(selection);
    for (key, value) in [
        ("codex.model", "legacy-model"),
        ("codex.model_provider", "legacy-provider"),
        ("codex.reasoning_effort", "low"),
        ("codex.api_key", "extension-secret-canary"),
        ("codex.base_url", "https://secret-canary.invalid/v1"),
    ] {
        request
            .extensions
            .insert(key.to_owned(), serde_json::json!(value));
    }

    let start = codec
        .session_request(SessionOperation::Create, &request)
        .unwrap();
    assert_eq!(
        serde_json::json!({"method": start.method, "params": start.params}),
        serde_json::json!({
            "method": "thread/start",
            "params": {
                "approvalPolicy": "never",
                "ephemeral": false,
                "model": "gpt-5.6-luna",
                "modelProvider": "omnisolo",
                "sandbox": "read-only"
            }
        })
    );

    let turn = codec
        .attempt_request(
            AttemptOperation::Execute,
            &request,
            "typed prompt",
            Some("thread-1"),
            None,
        )
        .unwrap();
    let exact_turn = serde_json::json!({"method": turn.method, "params": turn.params});
    assert_eq!(
        exact_turn,
        serde_json::json!({
            "method": "turn/start",
            "params": {
                "approvalPolicy": "never",
                "effort": "max",
                "input": [{"type": "text", "text": "typed prompt"}],
                "model": "gpt-5.6-luna",
                "sandboxPolicy": {"type": "readOnly"},
                "threadId": "thread-1"
            }
        })
    );
    let wire = exact_turn.to_string();
    for secret in [
        "typed-secret-canary",
        "extension-secret-canary",
        "secret-canary.invalid",
    ] {
        assert!(!wire.contains(secret));
    }
}

#[test]
fn codex_preserves_non_compatible_provider_ids_and_trait_initialize_dispatch() {
    let codec = CodexAppServerV2Codec::new();
    let mut selection = resolved_model(Some(ReasoningEffort::High));
    selection.provider_route = "private-provider".to_owned();
    let request = session_request().with_resolved_model(selection);

    let initialize = HarnessProtocolCodec::initialize_request(&codec);
    assert_eq!(initialize.method, "initialize");
    let start = codec
        .session_request(SessionOperation::Create, &request)
        .unwrap();
    assert_eq!(start.params["modelProvider"], "private-provider");
}

#[test]
fn codex_legacy_model_extensions_are_fallback_only_when_typed_selection_is_absent() {
    let codec = CodexAppServerV2Codec::new();
    let mut request = session_request();
    request
        .extensions
        .insert("codex.model".to_owned(), serde_json::json!("legacy-model"));
    request.extensions.insert(
        "codex.model_provider".to_owned(),
        serde_json::json!("legacy-provider"),
    );
    request.extensions.insert(
        "codex.reasoning_effort".to_owned(),
        serde_json::json!("high"),
    );

    let start = codec
        .session_request(SessionOperation::Create, &request)
        .unwrap();
    assert_eq!(start.params["model"], "legacy-model");
    assert_eq!(start.params["modelProvider"], "legacy-provider");
    assert!(start.params.get("effort").is_none());

    let turn = codec
        .attempt_request(
            AttemptOperation::Execute,
            &request,
            "prompt",
            Some("thread-1"),
            None,
        )
        .unwrap();
    assert_eq!(turn.params["model"], "legacy-model");
    assert!(turn.params.get("modelProvider").is_none());
    assert_eq!(turn.params["effort"], "high");
}

#[test]
fn codex_rejects_blank_or_malformed_typed_and_legacy_routing_values() {
    let codec = CodexAppServerV2Codec::new();
    let mut typed_selection = resolved_model(Some(ReasoningEffort::Max));
    typed_selection.model_id = "  ".to_owned();
    let typed = session_request().with_resolved_model(typed_selection);
    assert!(matches!(
        codec.session_request(SessionOperation::Create, &typed),
        Err(HarnessAdapterError::InvalidRequest(message)) if message.contains("model")
    ));

    for (key, value) in [
        ("codex.model", " "),
        ("codex.model_provider", "bad\nprovider"),
        ("codex.reasoning_effort", "extreme"),
    ] {
        let mut legacy = session_request();
        legacy
            .extensions
            .insert(key.to_owned(), serde_json::json!(value));
        let result = if key == "codex.reasoning_effort" {
            codec.attempt_request(
                AttemptOperation::Execute,
                &legacy,
                "prompt",
                Some("thread-1"),
                None,
            )
        } else {
            codec.session_request(SessionOperation::Create, &legacy)
        };
        assert!(matches!(
            result,
            Err(HarnessAdapterError::InvalidRequest(_))
        ));
    }
}

#[test]
fn codex_omits_model_provider_and_effort_when_no_selection_or_fallback_exists() {
    let codec = CodexAppServerV2Codec::new();
    let request = session_request();

    let start = codec
        .session_request(SessionOperation::Create, &request)
        .unwrap();
    for field in ["model", "modelProvider", "effort"] {
        assert!(start.params.get(field).is_none(), "unexpected {field}");
    }

    let turn = codec
        .attempt_request(
            AttemptOperation::Execute,
            &request,
            "prompt",
            Some("thread-1"),
            None,
        )
        .unwrap();
    for field in ["model", "modelProvider", "effort"] {
        assert!(turn.params.get(field).is_none(), "unexpected {field}");
    }
}

#[test]
fn codex_maps_every_portable_reasoning_effort_and_rejects_custom() {
    let codec = CodexAppServerV2Codec::new();
    for (effort, expected) in [
        (ReasoningEffort::None, "none"),
        (ReasoningEffort::Minimal, "minimal"),
        (ReasoningEffort::Low, "low"),
        (ReasoningEffort::Medium, "medium"),
        (ReasoningEffort::High, "high"),
        (ReasoningEffort::Max, "max"),
    ] {
        let request = attempt_request().with_resolved_model(resolved_model(Some(effort.clone())));
        let turn = codec
            .attempt_request(
                AttemptOperation::Execute,
                &request,
                "prompt",
                Some("thread-1"),
                None,
            )
            .unwrap();
        assert_eq!(turn.params["effort"], expected, "{effort:?}");
    }

    let mut without_effort = attempt_request().with_resolved_model(resolved_model(None));
    without_effort.extensions.insert(
        "codex.reasoning_effort".to_owned(),
        serde_json::json!("high"),
    );
    let turn = codec
        .attempt_request(
            AttemptOperation::Execute,
            &without_effort,
            "prompt",
            Some("thread-1"),
            None,
        )
        .unwrap();
    assert!(turn.params.get("effort").is_none());

    let custom =
        attempt_request().with_resolved_model(resolved_model(Some(ReasoningEffort::Custom)));
    let error = codec
        .attempt_request(
            AttemptOperation::Execute,
            &custom,
            "prompt",
            Some("thread-1"),
            None,
        )
        .unwrap_err();
    assert!(error.to_string().contains("custom reasoning effort"));
}

#[test]
fn codex_turn_input_is_text_and_steer_requires_expected_turn() {
    let codec = CodexAppServerV2Codec::new();
    let start = codec
        .attempt_request(
            AttemptOperation::Execute,
            &attempt_request(),
            "turn prompt",
            Some("thread-1"),
            None,
        )
        .unwrap();
    assert_eq!(start.method, "turn/start");
    assert_eq!(start.params["threadId"], "thread-1");
    assert_eq!(start.params["input"][0]["type"], "text");
    assert_eq!(start.params["input"][0]["text"], "turn prompt");

    let steer = codec
        .attempt_request(
            AttemptOperation::Steer,
            &attempt_request(),
            "new prompt",
            Some("thread-1"),
            Some("turn-1"),
        )
        .unwrap();
    assert_eq!(steer.method, "turn/steer");
    assert_eq!(steer.params["expectedTurnId"], "turn-1");

    let interrupt = codec
        .attempt_request(
            AttemptOperation::Cancel,
            &attempt_request(),
            "",
            Some("thread-1"),
            Some("turn-1"),
        )
        .unwrap();
    assert_eq!(interrupt.method, "turn/interrupt");
    assert_eq!(interrupt.params["turnId"], "turn-1");
}

#[test]
fn codex_thread_result_becomes_native_session_and_capsule_items_are_historical() {
    let codec = CodexAppServerV2Codec::new();
    let native: NativeSession = codec
        .parse_thread_result(serde_json::json!({
            "thread": {"id": "thread-1"}
        }))
        .unwrap();
    assert_eq!(native.native_session_id, "thread-1");

    let items = codec.capsule_items(&capsule()).unwrap();
    assert!(!items.is_empty());
    assert!(
        items
            .iter()
            .all(|item| item["metadata"]["source"] == "omnisolo.historical")
    );
    assert!(
        items
            .iter()
            .all(|item| item.get("developerInstructions").is_none())
    );
}

#[test]
fn codex_notifications_map_deltas_completion_and_usage_with_native_cursor() {
    let codec = CodexAppServerV2Codec::new();
    let mut state = NativeTurnState::new("thread-1");

    let delta = codec
        .decode_notification(
            &JsonRpcNotification {
                method: "item/agentMessage/delta".to_owned(),
                params: serde_json::json!({
                    "threadId": "thread-1",
                    "turnId": "turn-1",
                    "itemId": "item-1",
                    "delta": "hello"
                }),
            },
            &mut state,
        )
        .unwrap();
    assert_eq!(delta.event.event_type, "assistant.text_chunk");
    assert!(!delta.event.durable);
    assert_eq!(delta.event.payload["content"], "hello");

    let completion = codec
        .decode_notification(
            &JsonRpcNotification {
                method: "item/completed".to_owned(),
                params: serde_json::json!({
                    "threadId": "thread-1",
                    "turnId": "turn-1",
                    "item": {"type": "agentMessage", "id": "item-1", "text": "hello"}
                }),
            },
            &mut state,
        )
        .unwrap();
    assert_eq!(completion.final_text.as_deref(), Some("hello"));
    assert!(completion.terminal == false);

    let usage = codec
        .decode_notification(
            &JsonRpcNotification {
                method: "thread/tokenUsage/updated".to_owned(),
                params: serde_json::json!({
                    "threadId": "thread-1",
                    "turnId": "turn-1",
                    "tokenUsage": {"total": {"inputTokens": 1, "outputTokens": 2}}
                }),
            },
            &mut state,
        )
        .unwrap();
    assert_eq!(usage.event.event_type, "usage.recorded");
    assert!(usage.usage.is_some());
}

#[test]
fn codex_notification_credentials_are_sanitized_without_dropping_safe_fields() {
    let codec = CodexAppServerV2Codec::new();
    let mut state = NativeTurnState::new("thread-1");
    let event = codec
        .decode_notification(
            &JsonRpcNotification {
                method: "warning".to_owned(),
                params: serde_json::json!({
                    "threadId": "thread-1",
                    "warning": "Bearer CANARY-NOTIFY-7KQ9",
                    "details": {
                        "api_key": "CANARY-NOTIFY-7KQ9",
                        "name": "safe-name",
                    },
                }),
            },
            &mut state,
        )
        .unwrap();

    let payload = event.event.payload;
    assert_eq!(payload["details"]["details"]["name"], "safe-name");
    assert!(payload["details"]["details"].get("api_key").is_none());
    assert_eq!(payload["native"]["params"]["warning"], "[REDACTED]");
    assert!(!payload.to_string().contains("CANARY-NOTIFY-7KQ9"));
}

#[test]
fn codex_notifications_cover_nested_turns_tools_diffs_errors_and_compaction() {
    let codec = CodexAppServerV2Codec::new();
    let mut state = NativeTurnState::new("thread-1");

    let started = codec
        .decode_notification(
            &JsonRpcNotification {
                method: "turn/started".to_owned(),
                params: serde_json::json!({
                    "turn": {"id": "turn-1", "threadId": "thread-1"}
                }),
            },
            &mut state,
        )
        .unwrap();
    assert_eq!(started.event.event_type, "turn.started");
    assert_eq!(started.native_cursor.as_deref(), Some("turn-1:1"));

    for (method, event_type) in [
        ("item/reasoning/textDelta", "assistant.reasoning"),
        ("item/plan/delta", "plan.updated"),
        ("item/commandExecution/outputDelta", "tool.output"),
        ("item/fileChange/outputDelta", "file.change"),
        ("turn/diff/updated", "turn.diff.updated"),
        ("turn/plan/updated", "plan.updated"),
        ("error", "error"),
        ("contextCompaction", "context.compacted"),
    ] {
        let event = codec
            .decode_notification(
                &JsonRpcNotification {
                    method: method.to_owned(),
                    params: serde_json::json!({
                        "threadId": "thread-1",
                        "turnId": "turn-1",
                        "delta": "details",
                        "diff": "diff",
                        "plan": [],
                    }),
                },
                &mut state,
            )
            .unwrap();
        assert_eq!(event.event.event_type, event_type, "{method}");
    }

    let command = codec
        .decode_notification(
            &JsonRpcNotification {
                method: "item/completed".to_owned(),
                params: serde_json::json!({
                    "threadId": "thread-1",
                    "turnId": "turn-1",
                    "item": {"type": "commandExecution", "id": "command-1"},
                }),
            },
            &mut state,
        )
        .unwrap();
    assert_eq!(command.event.event_type, "tool.completed");

    let failed = codec
        .decode_notification(
            &JsonRpcNotification {
                method: "turn/completed".to_owned(),
                params: serde_json::json!({
                    "turn": {"id": "turn-1", "threadId": "thread-1", "status": "failed"}
                }),
            },
            &mut state,
        )
        .unwrap();
    assert_eq!(failed.event.event_type, "turn.failed");
    assert!(failed.terminal);
}

#[test]
fn codex_rejects_missing_native_ids_and_preserves_server_request_params() {
    let codec = CodexAppServerV2Codec::new();
    assert!(
        codec
            .attempt_request(
                AttemptOperation::Execute,
                &attempt_request(),
                "prompt",
                None,
                None,
            )
            .is_err()
    );

    let event = codec
        .server_request_event(
            &server_harness::middleware::json_rpc::JsonRpcServerRequest {
                id: server_harness::middleware::json_rpc::JsonRpcId::from_number(7),
                method: "item/commandExecution/requestApproval".to_owned(),
                params: serde_json::json!({
                    "command":"echo safe",
                    "credentials": {"api_key": "do-not-persist"},
                }),
            },
        )
        .unwrap();
    assert_eq!(event.event_type, "interaction.required");
    assert_eq!(event.payload["native_request_id"], 7);
    assert_eq!(
        event.payload["native_method"],
        "item/commandExecution/requestApproval"
    );
    assert!(
        event.payload["native_params"]["credentials"]
            .get("api_key")
            .is_none()
    );
}

#[test]
fn codex_validates_session_and_attempt_inputs_and_fallbacks() {
    let codec = CodexAppServerV2Codec::new();
    let mut request = session_request();
    request.extensions.insert(
        "native_session_id".to_owned(),
        serde_json::json!("thread-fallback"),
    );
    request.extensions.insert(
        "codex.last_turn_id".to_owned(),
        serde_json::json!("turn-last"),
    );
    request
        .extensions
        .insert("codex.ephemeral".to_owned(), serde_json::json!(true));
    request.extensions.insert(
        "codex.service_name".to_owned(),
        serde_json::json!("omnisolo-test"),
    );

    let fork = codec
        .session_request(
            SessionOperation::Fork {
                native_session_id: None,
            },
            &request,
        )
        .unwrap();
    assert_eq!(fork.params["threadId"], "thread-fallback");
    assert_eq!(fork.params["lastTurnId"], "turn-last");
    assert_eq!(fork.params["ephemeral"], true);

    let import = codec
        .session_request(SessionOperation::Import(capsule()), &request)
        .unwrap();
    assert_eq!(import.method, "thread/start");
    assert_eq!(import.params["serviceName"], "omnisolo-test");
    assert_eq!(import.params["ephemeral"], true);

    for operation in [
        SessionOperation::Quiesce,
        SessionOperation::Cancel,
        SessionOperation::Snapshot,
    ] {
        let read = codec.session_request(operation, &request).unwrap();
        assert_eq!(read.method, "thread/read");
    }

    assert!(
        codec
            .session_request(
                SessionOperation::Resume {
                    native_session_id: String::new(),
                },
                &request,
            )
            .is_err()
    );
    assert!(
        codec
            .session_request(
                SessionOperation::Fork {
                    native_session_id: None,
                },
                &HarnessSessionRequest::new("tenant", Uuid::new_v4(), Uuid::new_v4()),
            )
            .is_err()
    );
    assert!(
        codec
            .attempt_request(
                AttemptOperation::Execute,
                &request,
                "   ",
                Some("thread-1"),
                None,
            )
            .is_err()
    );
    assert!(
        codec
            .attempt_request(
                AttemptOperation::Steer,
                &request,
                "prompt",
                Some("thread-1"),
                None,
            )
            .is_err()
    );
    assert!(
        codec
            .attempt_request(
                AttemptOperation::Cancel,
                &request,
                "",
                Some("thread-1"),
                Some(" "),
            )
            .is_err()
    );
    assert_eq!(
        codec
            .attempt_request(
                AttemptOperation::Reconcile,
                &request,
                "",
                Some("thread-1"),
                None,
            )
            .unwrap()
            .method,
        "thread/read"
    );

    assert_eq!(
        codec
            .parse_thread_result(serde_json::json!({"id":"thread-fallback"}))
            .unwrap()
            .native_session_id,
        "thread-fallback"
    );
    for result in [
        serde_json::json!({}),
        serde_json::json!({"thread":{"id":""}}),
    ] {
        assert!(codec.parse_thread_result(result).is_err());
    }
}

#[test]
fn codex_maps_all_remaining_notification_families_and_rejects_cross_thread_events() {
    let codec = CodexAppServerV2Codec::new();
    let mut state = NativeTurnState::new("thread-1");

    for (method, item_type, expected) in [
        ("item/reasoning/summaryTextDelta", "", "assistant.reasoning"),
        ("item/reasoning/summaryPartAdded", "", "assistant.reasoning"),
        ("item/started", "commandExecution", "tool.started"),
        ("item/started", "fileChange", "file.change"),
        ("item/started", "agentMessage", "assistant.started"),
        ("item/started", "reasoning", "assistant.reasoning.started"),
        ("item/started", "other", "item.started"),
        ("item/completed", "reasoning", "assistant.reasoning"),
        ("item/completed", "plan", "plan.updated"),
        ("item/completed", "fileChange", "file.change"),
        ("item/completed", "mcpToolCall", "tool.completed"),
        ("item/completed", "dynamicToolCall", "tool.completed"),
        ("item/completed", "collabToolCall", "tool.completed"),
        ("item/completed", "unknown", "native.item.completed"),
    ] {
        let item = if item_type.is_empty() {
            serde_json::Value::Null
        } else {
            serde_json::json!({"type": item_type, "text": "final"})
        };
        let event = codec
            .decode_notification(
                &JsonRpcNotification {
                    method: method.to_owned(),
                    params: serde_json::json!({
                        "threadId": "thread-1",
                        "item": item,
                        "delta": "summary",
                        "text": "summary",
                        "summaryText": "summary",
                    }),
                },
                &mut state,
            )
            .unwrap();
        assert_eq!(event.event.event_type, expected, "{method}/{item_type}");
    }

    for (method, expected) in [
        ("warning", "warning"),
        ("configWarning", "warning"),
        ("serverRequest/resolved", "interaction.resolved"),
        ("thread/updated", "native.thread.updated"),
        ("model/listUpdated", "native.model.listUpdated"),
        ("unhandled/event", "native.unhandled.event"),
    ] {
        let event = codec
            .decode_notification(
                &JsonRpcNotification {
                    method: method.to_owned(),
                    params: serde_json::json!({"threadId": "thread-1"}),
                },
                &mut state,
            )
            .unwrap();
        assert_eq!(event.event.event_type, expected, "{method}");
    }

    let completed = codec
        .decode_notification(
            &JsonRpcNotification {
                method: "turn/completed".to_owned(),
                params: serde_json::json!({"threadId":"thread-1","status":"completed"}),
            },
            &mut state,
        )
        .unwrap();
    assert_eq!(completed.event.event_type, "turn.completed");
    assert!(completed.terminal);

    let mismatch = codec.decode_notification(
        &JsonRpcNotification {
            method: "thread/updated".to_owned(),
            params: serde_json::json!({"threadId":"other-thread"}),
        },
        &mut state,
    );
    assert!(mismatch.is_err());
}

#[test]
fn codex_encodes_results_and_native_errors() {
    let codec = CodexAppServerV2Codec::new();
    let result = codec
        .encode_server_response("approval", &serde_json::json!({"result":{"accepted":true}}))
        .unwrap();
    assert!(
        matches!(result, server_harness::middleware::protocol::ServerResponse::Result(value) if value["accepted"] == true)
    );

    let raw = codec
        .encode_server_response("approval", &serde_json::json!({"accepted":true}))
        .unwrap();
    assert!(
        matches!(raw, server_harness::middleware::protocol::ServerResponse::Result(value) if value["accepted"] == true)
    );

    let error = codec
        .encode_server_response(
            "approval",
            &serde_json::json!({"error":{"code":-32001,"message":"denied","data":{"reason":"policy"}}}),
        )
        .unwrap();
    assert!(
        matches!(error, server_harness::middleware::protocol::ServerResponse::Error(error) if error.code == -32001 && error.message == "denied" && error.data.is_some())
    );
    assert!(
        codec
            .encode_server_response("approval", &serde_json::json!({"error":[]}),)
            .is_err()
    );
}
