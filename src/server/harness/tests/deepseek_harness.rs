use std::collections::BTreeSet;
use std::time::Duration;

use serde_json::{Value, json};
use server_harness::middleware::adapter::{OmniSoloHarnessAdapter, OmniSoloRunConfig};
use server_harness::middleware::deepseek_harness::DeepSeekHarnessCodec;
use server_harness::middleware::harness::{HarnessAdapterError, HarnessSessionRequest};
use server_harness::middleware::json_rpc::{
    JsonRpcError, JsonRpcId, JsonRpcNotification, JsonRpcProcessConfig, JsonRpcProcessRuntime,
    JsonRpcServerRequest,
};
use server_harness::middleware::protocol::{
    AttemptOperation, HarnessProtocolCodec, NativeTurnState, SessionOperation,
};
use server_harness::middleware::types::{ModelApiDialect, ReasoningEffort, ResolvedModelSelection};
use uuid::Uuid;

fn request() -> HarnessSessionRequest {
    let mut request = HarnessSessionRequest::new(
        "tenant-1",
        Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
        Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
    );
    request.resolved_model = Some(ResolvedModelSelection {
        provider_route: "openai-compatible".to_owned(),
        model_id: "gpt-5.6-luna".to_owned(),
        reasoning_effort: Some(ReasoningEffort::Max),
        api_dialect: ModelApiDialect::OpenAiResponses,
        context_window: Some(400_000),
        max_output_tokens: Some(128_000),
        capabilities: BTreeSet::from(["reasoning".to_owned(), "tools".to_owned()]),
        binding_revision: "binding-v1".to_owned(),
        binding_digest: "sha256:test".to_owned(),
        metadata: [
            ("api_key".to_owned(), json!("must-not-leak")),
            (
                "base_url".to_owned(),
                json!("https://private.example.test/v1"),
            ),
        ]
        .into_iter()
        .collect(),
    });
    request
}

fn notification(method: &str, params: Value) -> JsonRpcNotification {
    JsonRpcNotification {
        method: method.to_owned(),
        params,
    }
}

#[test]
fn initialize_prefers_resolved_route_without_credentials_or_reasoning_fields() {
    let codec = DeepSeekHarnessCodec::for_request("/workspace/project", &request()).unwrap();
    let initialize = codec.initialize_request();

    assert_eq!(initialize.method, "initialize");
    assert_eq!(initialize.params["cwd"], "/workspace/project");
    assert_eq!(initialize.params["provider"], "openai-compatible");
    assert_eq!(initialize.params["model"], "gpt-5.6-luna");
    assert_eq!(initialize.params["maxTokens"], 128_000);
    assert!(initialize.params.get("reasoningEffort").is_none());
    assert!(initialize.params.get("apiKey").is_none());
    assert!(initialize.params.get("baseURL").is_none());
    let wire = serde_json::to_string(&initialize.params).unwrap();
    assert!(!wire.contains("must-not-leak"));
    assert!(!wire.contains("private.example.test"));
}

#[test]
fn initialize_and_prompt_results_require_official_payload_shapes() {
    let codec = DeepSeekHarnessCodec::for_request("/workspace/project", &request()).unwrap();
    let server = codec
        .decode_initialize_result(json!({
            "serverInfo": {
                "name": "deepseek-harness-sdk-runtime",
                "version": "0.0.1"
            }
        }))
        .unwrap();
    assert_eq!(server.name, "deepseek-harness-sdk-runtime");
    assert_eq!(server.version, "0.0.1");
    assert_eq!(
        codec
            .decode_prompt_result(json!({"messageId": "message-1"}))
            .unwrap(),
        "message-1"
    );

    for malformed in [
        json!({}),
        json!({"serverInfo": {"name": "wrong", "version": "0.0.1"}}),
        json!({"serverInfo": {"name": "deepseek-harness-sdk-runtime"}}),
    ] {
        assert!(codec.decode_initialize_result(malformed).is_err());
    }
    for malformed in [json!({}), json!({"messageId": 1}), json!({"messageId": ""})] {
        assert!(codec.decode_prompt_result(malformed).is_err());
    }
}

#[test]
fn shutdown_result_requires_the_official_empty_object() {
    let codec = DeepSeekHarnessCodec::for_request("/workspace/project", &request()).unwrap();

    assert!(codec.decode_shutdown_result(json!({})).is_ok());
    for incompatible in [Value::Null, json!([]), json!({"ok": true})] {
        assert!(codec.decode_shutdown_result(incompatible).is_err());
    }
}

#[test]
fn prompt_and_shutdown_use_only_official_methods() {
    let request = request();
    let session_id = request.session_id.to_string();
    let codec = DeepSeekHarnessCodec::for_request("/workspace/project", &request).unwrap();
    let prompt = codec
        .attempt_request(
            AttemptOperation::Execute,
            &request,
            "Inspect the repository",
            Some(&session_id),
            None,
        )
        .unwrap();

    assert_eq!(prompt.method, "session/prompt");
    assert_eq!(prompt.params["sessionId"], session_id);
    assert_eq!(
        prompt.params["contentBlocks"],
        json!([{
            "type": "text",
            "text": "Inspect the repository"
        }])
    );
    assert!(
        !serde_json::to_string(&prompt.params)
            .unwrap()
            .contains("must-not-leak")
    );

    let shutdown = codec.shutdown_request();
    assert_eq!(shutdown.method, "shutdown");
}

#[tokio::test]
async fn fake_process_runs_initialize_prompt_notifications_and_exact_shutdown_wire() {
    let runtime = JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell(
        r#"
read initialize
initialize_id=$(printf '%s' "$initialize" | sed -n 's/.*"id":\([0-9][0-9]*\).*/\1/p')
printf '{"jsonrpc":"2.0","id":%s,"result":{"serverInfo":{"name":"deepseek-harness-sdk-runtime","version":"0.0.1"}}}\n' "$initialize_id"

read prompt
prompt_id=$(printf '%s' "$prompt" | sed -n 's/.*"id":\([0-9][0-9]*\).*/\1/p')
printf '%s\n' '{"jsonrpc":"2.0","method":"session.event","params":{"sessionId":"00000000-0000-0000-0000-000000000001","event":{"type":"assistant/message","seq":1,"time":1,"data":{"turn":1,"step":1,"message":{"role":"assistant","content":[{"type":"text","text":"done"}]},"usage":{"inputTokens":3,"outputTokens":1}}}}}'
printf '%s\n' '{"jsonrpc":"2.0","method":"session.status","params":{"sessionId":"00000000-0000-0000-0000-000000000001","status":"idle"}}'
printf '{"jsonrpc":"2.0","id":%s,"result":{"messageId":"message-1"}}\n' "$prompt_id"

read shutdown
if [ "$shutdown" != '{"id":3,"jsonrpc":"2.0","method":"shutdown"}' ]; then
  printf '%s\n' '{"jsonrpc":"2.0","id":3,"error":{"code":-32602,"message":"shutdown must omit params"}}'
  exit 41
fi
printf '%s\n' '{"jsonrpc":"2.0","id":3,"result":{}}'
"#,
    ))
    .await
    .unwrap();
    let mut notifications = runtime.subscribe_notifications();
    let request = request();
    let codec = DeepSeekHarnessCodec::for_request("/workspace/project", &request).unwrap();

    let initialize = codec.initialize_request();
    let server = codec
        .decode_initialize_result(
            runtime
                .request(&initialize.method, initialize.params)
                .await
                .unwrap(),
        )
        .unwrap();
    assert_eq!(server.name, "deepseek-harness-sdk-runtime");

    let prompt = codec
        .attempt_request(
            AttemptOperation::Execute,
            &request,
            "Inspect the repository",
            Some(&request.session_id.to_string()),
            None,
        )
        .unwrap();
    let message_id = codec
        .decode_prompt_result(
            runtime
                .request(&prompt.method, prompt.params)
                .await
                .unwrap(),
        )
        .unwrap();
    assert_eq!(message_id, "message-1");

    let mut state = NativeTurnState::new(request.session_id.to_string());
    let message = codec
        .decode_notification(
            &tokio::time::timeout(Duration::from_secs(2), notifications.recv())
                .await
                .unwrap()
                .unwrap(),
            &mut state,
        )
        .unwrap();
    assert_eq!(message.final_text.as_deref(), Some("done"));
    assert_eq!(
        message.usage,
        Some(json!({"inputTokens": 3, "outputTokens": 1}))
    );
    let completed = codec
        .decode_notification(
            &tokio::time::timeout(Duration::from_secs(2), notifications.recv())
                .await
                .unwrap()
                .unwrap(),
            &mut state,
        )
        .unwrap();
    assert!(completed.terminal);

    codec
        .decode_shutdown_result(codec.shutdown_request().send(&runtime).await.unwrap())
        .unwrap();
    runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn fake_process_malformed_frame_fails_the_pending_deepseek_request() {
    let runtime = JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell(
        r#"read initialize; printf '%s\n' '{not-json}'"#,
    ))
    .await
    .unwrap();
    let codec = DeepSeekHarnessCodec::for_request("/workspace/project", &request()).unwrap();
    let initialize = codec.initialize_request();

    assert!(matches!(
        runtime.request(&initialize.method, initialize.params).await,
        Err(JsonRpcError::Malformed(_))
    ));
    runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn fake_process_exit_fails_the_pending_deepseek_request() {
    let runtime =
        JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell("read initialize; exit 17"))
            .await
            .unwrap();
    let codec = DeepSeekHarnessCodec::for_request("/workspace/project", &request()).unwrap();
    let initialize = codec.initialize_request();

    assert!(matches!(
        runtime.request(&initialize.method, initialize.params).await,
        Err(JsonRpcError::ProcessExited)
    ));
    runtime.shutdown().await.unwrap();
}

#[test]
fn unsupported_session_and_attempt_controls_are_typed_errors() {
    let request = request();
    let codec = DeepSeekHarnessCodec::for_request("/workspace/project", &request).unwrap();

    for operation in [
        SessionOperation::Create,
        SessionOperation::Snapshot,
        SessionOperation::Cancel,
        SessionOperation::Close,
        SessionOperation::Delete,
    ] {
        assert!(matches!(
            codec.session_request(operation, &request),
            Err(HarnessAdapterError::InvalidRequest(message))
                if message.contains("not supported")
        ));
    }
    for operation in [
        AttemptOperation::Steer,
        AttemptOperation::Cancel,
        AttemptOperation::Quiesce,
        AttemptOperation::Reconcile,
    ] {
        assert!(matches!(
            codec.attempt_request(operation, &request, "prompt", Some("session-1"), None),
            Err(HarnessAdapterError::InvalidRequest(message))
                if message.contains("not supported")
        ));
    }
}

#[test]
fn session_events_map_text_tools_and_usage() {
    let codec = DeepSeekHarnessCodec::for_request("/workspace/project", &request()).unwrap();
    let mut state = NativeTurnState::new("session-1");

    let text = codec
        .decode_notification(
            &notification(
                "session.event",
                json!({
                    "sessionId": "session-1",
                    "event": {
                        "type": "assistant/chunk",
                        "seq": 4,
                        "time": 1,
                        "data": {"turn": 1, "step": 1, "chunk": {
                            "type": "text-delta", "index": 0, "text": "hello"
                        }}
                    }
                }),
            ),
            &mut state,
        )
        .unwrap();
    assert_eq!(text.event.event_type, "assistant.text_chunk");
    assert_eq!(text.event.payload["content"], "hello");
    assert_eq!(text.native_cursor.as_deref(), Some("session-1:4"));

    let tool = codec
        .decode_notification(
            &notification(
                "session.event",
                json!({
                    "sessionId": "session-1",
                    "event": {
                        "type": "tool/call", "seq": 5, "time": 2,
                        "data": {"turn": 1, "step": 1, "callId": "call-1", "name": "shell", "arguments": "{}"}
                    }
                }),
            ),
            &mut state,
        )
        .unwrap();
    assert_eq!(tool.event.event_type, "tool.started");
    assert_eq!(tool.event.payload["call_id"], "call-1");

    let result = codec
        .decode_notification(
            &notification(
                "session.event",
                json!({
                    "sessionId": "session-1",
                    "event": {
                        "type": "tool/result", "seq": 6, "time": 3,
                        "data": {"turn": 1, "step": 1, "message": {
                            "role": "tool", "toolCallId": "call-1", "content": [{"type": "text", "text": "ok"}]
                        }}
                    }
                }),
            ),
            &mut state,
        )
        .unwrap();
    assert_eq!(result.event.event_type, "tool.completed");

    let usage = codec
        .decode_notification(
            &notification(
                "session.event",
                json!({
                    "sessionId": "session-1",
                    "event": {
                        "type": "assistant/chunk", "seq": 7, "time": 4,
                        "data": {"turn": 1, "step": 1, "chunk": {
                            "type": "usage", "usage": {"inputTokens": 10, "outputTokens": 3}
                        }}
                    }
                }),
            ),
            &mut state,
        )
        .unwrap();
    assert_eq!(usage.event.event_type, "usage.recorded");
    assert_eq!(
        usage.usage,
        Some(json!({"inputTokens": 10, "outputTokens": 3}))
    );
}

#[test]
fn turn_end_outcomes_survive_idle_and_emit_one_canonical_terminal_result() {
    let codec = DeepSeekHarnessCodec::for_request("/workspace/project", &request()).unwrap();
    let cases = [
        (json!({"kind": "completed"}), "turn.completed"),
        (
            json!({"kind": "error", "error": {"message": "provider failed", "code": "UPSTREAM"}}),
            "turn.failed",
        ),
        (json!({"kind": "blocked"}), "turn.failed"),
        (json!({"kind": "max-tokens"}), "turn.failed"),
        (
            json!({"kind": "aborted", "reason": {"kind": "user"}}),
            "turn.cancelled",
        ),
        (json!({"kind": "interrupted"}), "turn.failed"),
        (json!({"kind": "future-reason"}), "turn.failed"),
    ];

    for (reason, expected_terminal) in cases {
        let kind = reason["kind"].as_str().unwrap();
        let mut state = NativeTurnState::new("session-1");
        let turn_end = codec
            .decode_notification(
                &notification(
                    "session.event",
                    json!({
                        "sessionId": "session-1",
                        "event": {
                            "type": "turn/end", "seq": 8, "time": 5,
                            "data": {"turn": 1, "reason": reason}
                        }
                    }),
                ),
                &mut state,
            )
            .unwrap();
        assert_eq!(turn_end.event.event_type, "native.deepseek.turn_end");
        assert!(!turn_end.terminal);

        let idle = codec
            .decode_notification(
                &notification(
                    "session.status",
                    json!({"sessionId": "session-1", "status": "idle"}),
                ),
                &mut state,
            )
            .unwrap();
        assert_eq!(idle.event.event_type, expected_terminal, "reason {kind}");
        assert!(idle.terminal, "reason {kind}");

        let duplicate_idle = codec
            .decode_notification(
                &notification(
                    "session.status",
                    json!({"sessionId": "session-1", "status": "idle"}),
                ),
                &mut state,
            )
            .unwrap();
        assert_eq!(
            duplicate_idle.event.event_type, "native.deepseek.session_status",
            "reason {kind}"
        );
        assert!(!duplicate_idle.terminal, "reason {kind}");
    }
}

#[test]
fn assistant_message_and_idle_status_supply_final_completion() {
    let codec = DeepSeekHarnessCodec::for_request("/workspace/project", &request()).unwrap();
    let mut state = NativeTurnState::new("session-1");
    let message = codec
        .decode_notification(
            &notification(
                "session.event",
                json!({
                    "sessionId": "session-1",
                    "event": {
                        "type": "assistant/message", "seq": 9, "time": 6,
                        "data": {
                            "turn": 1, "step": 1,
                            "message": {"role": "assistant", "content": [
                                {"type": "reasoning", "text": "private thought"},
                                {"type": "text", "text": "final answer"}
                            ]},
                            "usage": {"inputTokens": 12, "outputTokens": 4}
                        }
                    }
                }),
            ),
            &mut state,
        )
        .unwrap();
    assert_eq!(message.event.event_type, "assistant.final");
    assert_eq!(message.final_text.as_deref(), Some("final answer"));
    assert_eq!(
        message.usage,
        Some(json!({"inputTokens": 12, "outputTokens": 4}))
    );
    assert!(!message.terminal);

    let idle = codec
        .decode_notification(
            &notification(
                "session.status",
                json!({"sessionId": "session-1", "status": "idle"}),
            ),
            &mut state,
        )
        .unwrap();
    assert_eq!(idle.event.event_type, "turn.completed");
    assert!(idle.terminal);
}

#[test]
fn subagent_notifications_preserve_lineage_and_outcome() {
    let codec = DeepSeekHarnessCodec::for_request("/workspace/project", &request()).unwrap();
    let mut state = NativeTurnState::new("session-1");
    let started = codec
        .decode_notification(
            &notification(
                "subagent.started",
                json!({"parentSessionId": "session-1", "childSessionId": "child-1"}),
            ),
            &mut state,
        )
        .unwrap();
    assert_eq!(started.event.event_type, "subagent.started");
    assert_eq!(started.event.payload["child_session_id"], "child-1");

    let finished = codec
        .decode_notification(
            &notification(
                "subagent.finished",
                json!({
                    "provider": "openai-compatible",
                    "agentId": "child-1",
                    "parentSessionId": "session-1",
                    "childSessionId": "child-1",
                    "status": "error",
                    "stopReason": "max-tokens",
                    "lastAssistantMessage": [{"type": "text", "text": "partial"}]
                }),
            ),
            &mut state,
        )
        .unwrap();
    assert_eq!(finished.event.event_type, "subagent.failed");
    assert_eq!(finished.event.payload["stop_reason"], "max-tokens");
}

#[test]
fn malformed_and_cross_session_notifications_are_rejected() {
    let codec = DeepSeekHarnessCodec::for_request("/workspace/project", &request()).unwrap();
    for params in [
        json!({"sessionId": "session-2", "status": "idle"}),
        json!({"sessionId": "session-1", "status": "paused"}),
        json!({"sessionId": "session-1", "event": {"type": "assistant/message"}}),
    ] {
        let method = if params.get("event").is_some() {
            "session.event"
        } else {
            "session.status"
        };
        assert!(
            codec
                .decode_notification(
                    &notification(method, params),
                    &mut NativeTurnState::new("session-1"),
                )
                .is_err()
        );
    }
}

#[test]
fn exhaustive_validation_covers_configuration_controls_and_native_event_fallbacks() {
    let base_request = request();
    assert!(DeepSeekHarnessCodec::for_request(" ", &base_request).is_err());

    let default_request = HarnessSessionRequest::new("tenant", Uuid::new_v4(), Uuid::new_v4());
    let defaults = DeepSeekHarnessCodec::for_request("/workspace", &default_request).unwrap();
    assert_eq!(
        defaults.initialize_request().params["provider"],
        "deepseek-official"
    );
    assert!(
        defaults
            .initialize_request()
            .params
            .get("maxTokens")
            .is_none()
    );

    for mutation in ["provider", "model", "zero_tokens", "unsafe_tokens"] {
        let mut invalid = request();
        let selection = invalid.resolved_model.as_mut().unwrap();
        match mutation {
            "provider" => selection.provider_route = " ".to_owned(),
            "model" => selection.model_id = " ".to_owned(),
            "zero_tokens" => selection.max_output_tokens = Some(0),
            "unsafe_tokens" => selection.max_output_tokens = Some(9_007_199_254_740_992),
            _ => unreachable!(),
        }
        assert!(DeepSeekHarnessCodec::for_request("/workspace", &invalid).is_err());
    }

    let codec = DeepSeekHarnessCodec::for_request("/workspace", &base_request).unwrap();
    let trait_initialize = HarnessProtocolCodec::initialize_request(&codec);
    assert_eq!(trait_initialize.method, "initialize");
    let capsule = OmniSoloHarnessAdapter::start(
        OmniSoloRunConfig::new("tenant", "portable").with_session_id(base_request.session_id),
    )
    .unwrap()
    .export_capsule("deepseek", Uuid::new_v4())
    .unwrap();
    for operation in [
        SessionOperation::Import(capsule),
        SessionOperation::Resume {
            native_session_id: "session-1".to_owned(),
        },
        SessionOperation::Fork {
            native_session_id: Some("session-1".to_owned()),
        },
        SessionOperation::Quiesce,
    ] {
        assert!(codec.session_request(operation, &base_request).is_err());
    }
    assert!(
        codec
            .attempt_request(
                AttemptOperation::Execute,
                &base_request,
                " ",
                Some("session-1"),
                None,
            )
            .is_err()
    );
    assert!(
        codec
            .attempt_request(
                AttemptOperation::Execute,
                &base_request,
                "prompt",
                None,
                None,
            )
            .is_err()
    );
    assert!(
        codec
            .decode_session_result(SessionOperation::Create, json!({}))
            .is_err()
    );
    assert!(
        codec
            .server_request_event(&JsonRpcServerRequest {
                id: JsonRpcId::from_number(1),
                method: "interaction/request".to_owned(),
                params: json!({}),
            })
            .is_err()
    );
    assert!(
        codec
            .encode_server_response("interaction/request", &json!({}))
            .is_err()
    );

    let mut state = NativeTurnState::new("session-1");
    assert!(
        codec
            .decode_notification(&notification("future.notification", json!({})), &mut state)
            .is_err()
    );
    let event = |event_type: &str, data: Value, sequence: u64| {
        notification(
            "session.event",
            json!({
                "sessionId":"session-1",
                "event":{"type":event_type,"seq":sequence,"time":1,"data":data}
            }),
        )
    };
    for (native, expected) in [
        (event("turn/start", json!({"turn":1}), 20), "turn.started"),
        (
            event(
                "assistant/chunk",
                json!({"chunk":{"type":"reasoning-delta","text":"reason"}}),
                21,
            ),
            "assistant.reasoning",
        ),
        (
            event(
                "assistant/chunk",
                json!({"chunk":{"type":"tool-call-delta","name":"shell"}}),
                22,
            ),
            "tool.output",
        ),
        (
            event(
                "assistant/chunk",
                json!({"chunk":{"type":"future-chunk","value":1}}),
                23,
            ),
            "native.deepseek.assistant_chunk",
        ),
        (
            event("future/event", json!({"value":1}), 24),
            "native.deepseek.session_event",
        ),
    ] {
        assert_eq!(
            codec
                .decode_notification(&native, &mut state)
                .unwrap()
                .event
                .event_type,
            expected
        );
    }
    assert_eq!(
        codec
            .decode_notification(
                &notification(
                    "session.status",
                    json!({"sessionId":"session-1","status":"running"}),
                ),
                &mut state,
            )
            .unwrap()
            .event
            .event_type,
        "turn.started"
    );
    assert_eq!(
        codec
            .decode_notification(
                &notification(
                    "subagent.finished",
                    json!({
                        "provider":"provider","agentId":"agent",
                        "parentSessionId":"session-1","childSessionId":"child",
                        "status":"ok","stopReason":"end_turn"
                    }),
                ),
                &mut state,
            )
            .unwrap()
            .event
            .event_type,
        "subagent.finished"
    );

    for malformed in [
        notification(
            "session.event",
            json!({
                "sessionId":"session-1",
                "event":{"type":"turn/start","seq":29,"time":"wrong","data":{}}
            }),
        ),
        event(
            "assistant/chunk",
            json!({"chunk":{"type":"usage","usage":1}}),
            30,
        ),
        event("assistant/message", json!({"message":{"content":[1]}}), 31),
        event(
            "assistant/message",
            json!({"message":{"content":[{"type":"text"}]}}),
            32,
        ),
        event(
            "assistant/message",
            json!({"message":{"content":[]},"usage":1}),
            33,
        ),
        notification("subagent.started", json!({"parentSessionId":"session-1"})),
        notification(
            "subagent.finished",
            json!({
                "provider":"provider","agentId":"agent","parentSessionId":"session-1",
                "status":"ok","stopReason":"stop"
            }),
        ),
        notification(
            "subagent.finished",
            json!({
                "provider":"provider","agentId":"agent","parentSessionId":"session-1",
                "childSessionId":"child","status":"future","stopReason":"stop"
            }),
        ),
        notification(
            "subagent.finished",
            json!({
                "provider":"provider","agentId":"agent","parentSessionId":"session-1",
                "childSessionId":"child","status":"ok","stopReason":"stop",
                "lastAssistantMessage":{}
            }),
        ),
    ] {
        assert!(codec.decode_notification(&malformed, &mut state).is_err());
    }
}
