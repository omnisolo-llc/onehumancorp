use std::collections::BTreeSet;

use serde_json::{Value, json};
use server_harness::middleware::acp::{AcpModelCompatibility, AcpV1Codec};
use server_harness::middleware::harness::{HarnessAdapterError, HarnessSessionRequest};
use server_harness::middleware::json_rpc::{
    JsonRpcId, JsonRpcNotification, JsonRpcProcessConfig, JsonRpcProcessRuntime,
    JsonRpcServerRequest,
};
use server_harness::middleware::protocol::{
    AttemptOperation, HarnessProtocolCodec, NativeTurnState, ProtocolAttemptInstruction,
    ServerResponse, SessionOperation,
};
use server_harness::middleware::types::{ModelApiDialect, ReasoningEffort, ResolvedModelSelection};
use uuid::Uuid;

fn resolved_model(effort: ReasoningEffort) -> ResolvedModelSelection {
    ResolvedModelSelection {
        provider_route: "openai-compatible".to_owned(),
        model_id: "gpt-5.6-luna".to_owned(),
        reasoning_effort: Some(effort),
        api_dialect: ModelApiDialect::OpenAiResponses,
        context_window: Some(262_144),
        max_output_tokens: Some(32_768),
        capabilities: BTreeSet::from(["reasoning".to_owned(), "tools".to_owned()]),
        binding_revision: "binding-v1".to_owned(),
        binding_digest: "sha256:test".to_owned(),
        metadata: [("api_key".to_owned(), json!("credential-canary"))]
            .into_iter()
            .collect(),
    }
}

fn request(effort: ReasoningEffort) -> HarnessSessionRequest {
    HarnessSessionRequest::new("tenant", Uuid::new_v4(), Uuid::new_v4())
        .with_task(Uuid::new_v4(), "objective")
        .with_resolved_model(resolved_model(effort))
}

fn codec(effort: ReasoningEffort) -> AcpV1Codec {
    AcpV1Codec::for_kimi("/workspace/project", &request(effort)).unwrap()
}

fn notification(params: Value) -> JsonRpcNotification {
    JsonRpcNotification {
        method: "session/update".to_owned(),
        params,
    }
}

fn assert_no_credentials(value: &Value) {
    let encoded = serde_json::to_string(value).unwrap();
    for forbidden in [
        "credential-canary",
        "OPENAI_API_KEY",
        "OPENAI_API_BASE_URL",
        "api_key",
        "base_url",
    ] {
        assert!(
            !encoded.contains(forbidden),
            "leaked {forbidden}: {encoded}"
        );
    }
}

#[test]
fn initialize_is_acp_v1_and_advertises_only_implemented_client_capabilities() {
    let codec = codec(ReasoningEffort::Max);
    let initialize = codec.initialize_request();

    assert_eq!(initialize.method, "initialize");
    assert_eq!(initialize.params["protocolVersion"], 1);
    assert_eq!(initialize.params["clientInfo"]["name"], "omnisolo");
    assert_eq!(
        initialize.params["clientCapabilities"],
        json!({
            "fs": {"readTextFile": false, "writeTextFile": false},
            "terminal": false,
            "auth": {"terminal": false}
        })
    );
    assert_no_credentials(&initialize.params);

    let info = codec
        .decode_initialize_result(json!({
            "protocolVersion": 1,
            "agentCapabilities": {"loadSession": true},
            "agentInfo": {"name": "kimi-cli", "version": "1.49.0"}
        }))
        .unwrap();
    assert_eq!(info.protocol_version, 1);
    assert_eq!(info.agent_name.as_deref(), Some("kimi-cli"));
    assert!(info.load_session);

    assert!(
        codec
            .decode_initialize_result(json!({"protocolVersion": 2}))
            .is_err()
    );
    assert!(codec.decode_initialize_result(json!({})).is_err());
}

#[test]
fn new_and_load_sessions_use_official_shapes_and_preserve_correlation() {
    let codec = codec(ReasoningEffort::Max);
    let request = request(ReasoningEffort::Max);

    let create = codec
        .session_request(SessionOperation::Create, &request)
        .unwrap();
    assert_eq!(create.method, "session/new");
    assert_eq!(
        create.params,
        json!({"cwd": "/workspace/project", "mcpServers": []})
    );
    assert_no_credentials(&create.params);
    let created = codec
        .decode_session_result(
            SessionOperation::Create,
            json!({"sessionId": "kimi-session-1"}),
        )
        .unwrap();
    assert_eq!(created.native_session_id, "kimi-session-1");

    let load_operation = SessionOperation::Resume {
        native_session_id: "kimi-session-1".to_owned(),
    };
    let load = codec
        .session_request(load_operation.clone(), &request)
        .unwrap();
    assert_eq!(load.method, "session/load");
    assert_eq!(
        load.params,
        json!({
            "cwd": "/workspace/project",
            "sessionId": "kimi-session-1",
            "mcpServers": []
        })
    );
    let loaded = codec
        .decode_session_result(load_operation, json!({}))
        .unwrap();
    assert_eq!(loaded.native_session_id, "kimi-session-1");

    assert!(
        codec
            .decode_session_result(SessionOperation::Create, json!({"sessionId": ""}))
            .is_err()
    );
}

#[test]
fn stable_config_options_are_preferred_and_select_strongest_thought_level() {
    let codec = codec(ReasoningEffort::Max);
    codec
        .decode_session_result(
            SessionOperation::Create,
            json!({
                "sessionId": "kimi-session-1",
                "configOptions": [
                    {
                        "id": "model",
                        "name": "Model",
                        "category": "model",
                        "type": "select",
                        "currentValue": "other-model",
                        "options": [
                            {"value": "other-model", "name": "Other"},
                            {"value": "gpt-5.6-luna", "name": "GPT 5.6 Luna"}
                        ]
                    },
                    {
                        "id": "thinking",
                        "name": "Thinking",
                        "category": "thought_level",
                        "type": "select",
                        "currentValue": "low",
                        "options": [
                            {"value": "low", "name": "Low"},
                            {"value": "medium", "name": "Medium"},
                            {"value": "high", "name": "High"}
                        ]
                    }
                ],
                "models": {
                    "availableModels": [
                        {"modelId": "gpt-5.6-luna,thinking", "name": "Legacy"}
                    ],
                    "currentModelId": "gpt-5.6-luna,thinking"
                }
            }),
        )
        .unwrap();

    let plan = codec.model_configuration_plan("kimi-session-1").unwrap();
    assert_eq!(
        plan.compatibility,
        AcpModelCompatibility::StableConfigOptions
    );
    assert_eq!(plan.requests.len(), 2);
    assert_eq!(plan.requests[0].method, "session/set_config_option");
    assert_eq!(
        plan.requests[0].params,
        json!({
            "sessionId": "kimi-session-1",
            "configId": "model",
            "value": "gpt-5.6-luna"
        })
    );
    assert_eq!(plan.requests[1].method, "session/set_config_option");
    assert_eq!(
        plan.requests[1].params,
        json!({
            "sessionId": "kimi-session-1",
            "configId": "thinking",
            "value": "high"
        })
    );
    assert_eq!(plan.events.len(), 1);
    assert_eq!(plan.events[0].event_type, "capability.downgraded");
    assert_eq!(plan.events[0].payload["capability"], "reasoning_effort");
    assert_eq!(plan.events[0].payload["requested"], "max");
    assert_eq!(plan.events[0].payload["effective"], "high");
    for request in &plan.requests {
        assert_no_credentials(&request.params);
        assert_ne!(request.method, "session/set_model");
    }
}

#[test]
fn stable_model_without_reasoning_option_records_maximum_effort_loss() {
    let codec = codec(ReasoningEffort::Max);
    codec
        .decode_session_result(
            SessionOperation::Create,
            json!({
                "sessionId": "model-only-session",
                "configOptions": [{
                    "id": "model",
                    "name": "Model",
                    "category": "model",
                    "type": "select",
                    "currentValue": "other-model",
                    "options": [
                        {"value": "gpt-5.6-luna", "name": "GPT 5.6 Luna"}
                    ]
                }]
            }),
        )
        .unwrap();

    let plan = codec
        .model_configuration_plan("model-only-session")
        .unwrap();
    assert_eq!(plan.requests.len(), 1);
    assert!(plan.events.iter().any(|event| {
        event.payload["capability"] == "reasoning_effort"
            && event.payload["requested"] == "max"
            && event.payload["effective"] == "none"
            && event.payload["reason"] == "agent_advertised_no_reasoning_config_option"
    }));
}

#[test]
fn stable_reasoning_never_upgrades_and_rejects_unknown_advertised_levels() {
    for (effort, expected) in [
        (ReasoningEffort::None, "none"),
        (ReasoningEffort::Minimal, "minimal"),
        (ReasoningEffort::Low, "low"),
        (ReasoningEffort::Medium, "medium"),
        (ReasoningEffort::High, "high"),
        (ReasoningEffort::Max, "max"),
    ] {
        let codec = codec(effort);
        codec
            .decode_session_result(
                SessionOperation::Create,
                json!({
                    "sessionId": format!("session-{expected}"),
                    "configOptions": [
                        {
                            "id": "model",
                            "category": "model",
                            "type": "select",
                            "options": [{"value": "gpt-5.6-luna"}]
                        },
                        {
                            "id": "thinking",
                            "category": "thought_level",
                            "type": "select",
                            "options": [
                                {"value": "none"},
                                {"value": "minimal"},
                                {"value": "low"},
                                {"value": "medium"},
                                {"value": "high"},
                                {"value": "max"}
                            ]
                        }
                    ]
                }),
            )
            .unwrap();
        let plan = codec
            .model_configuration_plan(&format!("session-{expected}"))
            .unwrap();
        assert_eq!(plan.requests[1].params["value"], expected);
        assert!(
            plan.events.is_empty(),
            "unexpected downgrade for {expected}"
        );
    }

    let low = codec(ReasoningEffort::Low);
    low.decode_session_result(
        SessionOperation::Create,
        json!({
            "sessionId": "no-safe-level",
            "configOptions": [
                {
                    "id": "model",
                    "category": "model",
                    "type": "select",
                    "options": [{"value": "gpt-5.6-luna"}]
                },
                {
                    "id": "thinking",
                    "category": "thought_level",
                    "type": "select",
                    "options": [{"value": "high"}]
                }
            ]
        }),
    )
    .unwrap();
    let plan = low.model_configuration_plan("no-safe-level").unwrap();
    assert_eq!(plan.requests.len(), 1, "must not upgrade low to high");
    assert!(plan.events.iter().any(|event| {
        event.payload["requested"] == "low" && event.payload["effective"] == "none"
    }));

    let unknown = codec(ReasoningEffort::Max);
    assert!(matches!(
        unknown.decode_session_result(
            SessionOperation::Create,
            json!({
                "sessionId": "unknown-level",
                "configOptions": [
                    {
                        "id": "model",
                        "category": "model",
                        "type": "select",
                        "options": [{"value": "gpt-5.6-luna"}]
                    },
                    {
                        "id": "thinking",
                        "category": "thought_level",
                        "type": "select",
                        "options": [{"value": "turbo"}]
                    }
                ]
            }),
        ),
        Err(HarnessAdapterError::InvalidResponse(message)) if message.contains("reasoning")
    ));
}

#[test]
fn durable_native_payloads_are_recursively_sanitized() {
    let codec = codec(ReasoningEffort::Max);
    let mut state = NativeTurnState::new("kimi-session-1");
    let update = codec
        .decode_notification(
            &notification(json!({
                "sessionId": "kimi-session-1",
                "update": {
                    "sessionUpdate": "session_info_update",
                    "_meta": {
                        "api_key": "credential-canary",
                        "nested": ["authorization=credential-canary"]
                    }
                }
            })),
            &mut state,
        )
        .unwrap();
    assert_no_credentials(&update.event.payload);

    let permission = codec
        .server_request_event(&JsonRpcServerRequest {
            id: JsonRpcId::from_number(99),
            method: "session/request_permission".to_owned(),
            params: json!({
                "sessionId": "kimi-session-1",
                "toolCall": {"toolCallId": "tool-1"},
                "options": [{"optionId": "reject", "name": "Reject", "kind": "reject_once"}],
                "_meta": {"access_token": "credential-canary"}
            }),
        })
        .unwrap();
    assert_no_credentials(&permission.payload);

    let completion = codec
        .decode_prompt_result(
            "kimi-session-1",
            json!({
                "stopReason": "end_turn",
                "usage": {"inputTokens": 1},
                "_meta": {"password": "credential-canary"}
            }),
        )
        .unwrap();
    assert_no_credentials(&completion.event.payload);
}

#[test]
fn pinned_kimi_legacy_models_enable_only_deprecated_set_model_fallback() {
    let codec = codec(ReasoningEffort::Max);
    codec
        .decode_session_result(
            SessionOperation::Create,
            json!({
                "sessionId": "kimi-session-1",
                "models": {
                    "availableModels": [
                        {"modelId": "gpt-5.6-luna", "name": "GPT 5.6 Luna"},
                        {"modelId": "gpt-5.6-luna,thinking", "name": "GPT 5.6 Luna (thinking)"}
                    ],
                    "currentModelId": "gpt-5.6-luna"
                }
            }),
        )
        .unwrap();

    let plan = codec.model_configuration_plan("kimi-session-1").unwrap();
    assert_eq!(plan.compatibility, AcpModelCompatibility::LegacySetModel);
    assert_eq!(plan.requests.len(), 1);
    assert_eq!(plan.requests[0].method, "session/set_model");
    assert_eq!(
        plan.requests[0].params,
        json!({
            "sessionId": "kimi-session-1",
            "modelId": "gpt-5.6-luna,thinking"
        })
    );
    assert!(
        plan.requests
            .iter()
            .all(|request| request.method != "session/set_config_option")
    );
    assert!(plan.events.iter().any(|event| {
        event.event_type == "capability.downgraded"
            && event.payload["capability"] == "acp_model_configuration"
            && event.payload["requested"] == "session/set_config_option"
            && event.payload["effective"] == "session/set_model"
    }));
    assert!(plan.events.iter().any(|event| {
        event.payload["capability"] == "reasoning_effort"
            && event.payload["effective"] == "thinking"
    }));
    let requests = plan
        .requests
        .iter()
        .map(|request| json!({"method": request.method, "params": request.params}))
        .collect::<Vec<_>>();
    assert_no_credentials(&json!({"requests": requests, "events": plan.events}));
}

#[test]
fn malformed_or_unadvertised_model_capabilities_are_rejected() {
    let stable = codec(ReasoningEffort::Max);
    stable
        .decode_session_result(
            SessionOperation::Create,
            json!({
                "sessionId": "stable-session",
                "configOptions": [{
                    "id": "model",
                    "name": "Model",
                    "category": "model",
                    "type": "select",
                    "currentValue": "other-model",
                    "options": [{"value": "other-model", "name": "Other"}]
                }]
            }),
        )
        .unwrap();
    assert!(matches!(
        stable.model_configuration_plan("stable-session"),
        Err(HarnessAdapterError::InvalidRequest(message))
            if message.contains("not advertised")
    ));

    let legacy = codec(ReasoningEffort::Max);
    legacy
        .decode_session_result(
            SessionOperation::Create,
            json!({
                "sessionId": "legacy-session",
                "models": {
                    "availableModels": [
                        {"modelId": "gpt-5.6-luna", "name": "GPT 5.6 Luna"}
                    ],
                    "currentModelId": "gpt-5.6-luna"
                }
            }),
        )
        .unwrap();
    assert!(matches!(
        legacy.model_configuration_plan("legacy-session"),
        Err(HarnessAdapterError::InvalidRequest(message))
            if message.contains("not advertised")
    ));

    let malformed = codec(ReasoningEffort::Max);
    assert!(matches!(
        malformed.decode_session_result(
            SessionOperation::Create,
            json!({
                "sessionId": "malformed-session",
                "configOptions": [{
                    "id": "model",
                    "name": "Model",
                    "category": "model",
                    "type": "select",
                    "currentValue": "gpt-5.6-luna",
                    "options": "not-an-array"
                }]
            }),
        ),
        Err(HarnessAdapterError::InvalidResponse(_))
    ));
}

#[test]
fn prompt_and_cancellation_use_official_acp_methods_without_wire_secrets() {
    let codec = codec(ReasoningEffort::Max);
    let request = request(ReasoningEffort::Max);
    let prompt = codec
        .attempt_request(
            AttemptOperation::Execute,
            &request,
            "Inspect the repository",
            Some("kimi-session-1"),
            None,
        )
        .unwrap();
    assert_eq!(prompt.method, "session/prompt");
    assert_eq!(
        prompt.params,
        json!({
            "sessionId": "kimi-session-1",
            "prompt": [{"type": "text", "text": "Inspect the repository"}]
        })
    );
    assert_no_credentials(&prompt.params);

    let cancel = codec
        .cancel_notification("kimi-session-1")
        .expect("non-empty session id");
    assert_eq!(cancel.method, "session/cancel");
    assert_eq!(cancel.params, json!({"sessionId": "kimi-session-1"}));
    let wire = serde_json::to_value(&cancel).unwrap();
    assert_eq!(
        wire,
        json!({
            "method": "session/cancel",
            "params": {"sessionId": "kimi-session-1"}
        })
    );
    assert!(wire.get("id").is_none());

    assert!(matches!(
        codec
        .attempt_request(
            AttemptOperation::Cancel,
            &request,
            "",
            Some("kimi-session-1"),
            None,
        ),
        Err(HarnessAdapterError::InvalidRequest(message))
            if message.contains("notification")
    ));
    assert_eq!(
        codec
            .attempt_instruction(
                AttemptOperation::Cancel,
                &request,
                "",
                Some("kimi-session-1"),
                None,
            )
            .unwrap(),
        ProtocolAttemptInstruction::Notification(cancel)
    );

    assert!(codec.shutdown_request().is_none());
    assert!(
        codec
            .attempt_request(
                AttemptOperation::Execute,
                &request,
                " ",
                Some("kimi-session-1"),
                None,
            )
            .is_err()
    );
}

#[tokio::test]
async fn typed_cancellation_instruction_writes_notification_without_request_id() {
    let runtime = JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell(
        r#"
read cancellation
if [ "$cancellation" != '{"jsonrpc":"2.0","method":"session/cancel","params":{"sessionId":"kimi-session-1"}}' ]; then
  exit 41
fi
printf '%s\n' '{"jsonrpc":"2.0","method":"verified","params":{}}'
sleep 1
"#,
    ))
    .await
    .unwrap();
    let mut notifications = runtime.subscribe_notifications();
    let instruction = codec(ReasoningEffort::Max)
        .attempt_instruction(
            AttemptOperation::Cancel,
            &request(ReasoningEffort::Max),
            "",
            Some("kimi-session-1"),
            None,
        )
        .unwrap();

    assert_eq!(instruction.send(&runtime).await.unwrap(), None);
    let verified = tokio::time::timeout(std::time::Duration::from_secs(2), notifications.recv())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(verified.method, "verified");
    runtime.shutdown().await.unwrap();
}

#[test]
fn session_updates_map_streamed_text_reasoning_and_preserve_native_payloads() {
    let codec = codec(ReasoningEffort::Max);
    let mut state = NativeTurnState::new("kimi-session-1");
    let text_params = json!({
        "sessionId": "kimi-session-1",
        "update": {
            "sessionUpdate": "agent_message_chunk",
            "content": {"type": "text", "text": "hello"},
            "_meta": {"modelId": "gpt-5.6-luna"}
        }
    });
    let text = codec
        .decode_notification(&notification(text_params.clone()), &mut state)
        .unwrap();
    assert_eq!(text.event.event_type, "assistant.text_chunk");
    assert_eq!(text.event.payload["content"], "hello");
    assert_eq!(
        text.event.payload["native"],
        json!({"method": "session/update", "params": text_params})
    );
    assert_eq!(text.native_cursor.as_deref(), Some("kimi-session-1:1"));

    let reasoning = codec
        .decode_notification(
            &notification(json!({
                "sessionId": "kimi-session-1",
                "update": {
                    "sessionUpdate": "agent_thought_chunk",
                    "content": {"type": "text", "text": "thinking"}
                }
            })),
            &mut state,
        )
        .unwrap();
    assert_eq!(reasoning.event.event_type, "assistant.reasoning");
    assert_eq!(reasoning.event.payload["content"], "thinking");
    assert_eq!(reasoning.native_cursor.as_deref(), Some("kimi-session-1:2"));

    assert!(
        codec
            .decode_notification(
                &notification(json!({
                    "sessionId": "different-session",
                    "update": {
                        "sessionUpdate": "agent_message_chunk",
                        "content": {"type": "text", "text": "wrong"}
                    }
                })),
                &mut state,
            )
            .is_err()
    );
}

#[test]
fn session_updates_map_tools_terminal_attachments_and_usage() {
    let codec = codec(ReasoningEffort::Max);
    let mut state = NativeTurnState::new("kimi-session-1");

    let tool = codec
        .decode_notification(
            &notification(json!({
                "sessionId": "kimi-session-1",
                "update": {
                    "sessionUpdate": "tool_call",
                    "toolCallId": "turn-1:tool-1",
                    "title": "Run tests",
                    "kind": "execute",
                    "status": "in_progress",
                    "content": [{
                        "type": "content",
                        "content": {"type": "text", "text": "cargo test"}
                    }]
                }
            })),
            &mut state,
        )
        .unwrap();
    assert_eq!(tool.event.event_type, "tool.started");
    assert_eq!(tool.event.payload["tool_call_id"], "turn-1:tool-1");

    let terminal = codec
        .decode_notification(
            &notification(json!({
                "sessionId": "kimi-session-1",
                "update": {
                    "sessionUpdate": "tool_call_update",
                    "toolCallId": "turn-1:tool-1",
                    "status": "in_progress",
                    "content": [{"type": "terminal", "terminalId": "terminal-1"}]
                }
            })),
            &mut state,
        )
        .unwrap();
    assert_eq!(terminal.event.event_type, "terminal.attached");
    assert_eq!(
        terminal.event.payload["terminal_ids"],
        json!(["terminal-1"])
    );

    let completed = codec
        .decode_notification(
            &notification(json!({
                "sessionId": "kimi-session-1",
                "update": {
                    "sessionUpdate": "tool_call_update",
                    "toolCallId": "turn-1:tool-1",
                    "status": "completed",
                    "rawOutput": {"exitCode": 0}
                }
            })),
            &mut state,
        )
        .unwrap();
    assert_eq!(completed.event.event_type, "tool.completed");

    let usage_params = json!({
        "sessionId": "kimi-session-1",
        "update": {
            "sessionUpdate": "usage_update",
            "used": 1234,
            "size": 262144,
            "cost": {"amount": 0.12, "currency": "USD"}
        }
    });
    let usage = codec
        .decode_notification(&notification(usage_params), &mut state)
        .unwrap();
    assert_eq!(usage.event.event_type, "usage.recorded");
    assert_eq!(usage.usage.as_ref().unwrap()["used"], 1234);
    assert_eq!(usage.usage.as_ref().unwrap()["size"], 262144);
}

#[test]
fn malformed_session_updates_are_typed_errors() {
    let codec = codec(ReasoningEffort::Max);
    for params in [
        json!({}),
        json!({"sessionId": "kimi-session-1"}),
        json!({"sessionId": "kimi-session-1", "update": {}}),
        json!({
            "sessionId": "kimi-session-1",
            "update": {
                "sessionUpdate": "agent_message_chunk",
                "content": {"type": "text", "text": 3}
            }
        }),
        json!({
            "sessionId": "kimi-session-1",
            "update": {"sessionUpdate": "tool_call", "title": "missing id"}
        }),
        json!({
            "sessionId": "kimi-session-1",
            "update": {"sessionUpdate": "usage_update", "used": -1, "size": 100}
        }),
        json!({
            "sessionId": "kimi-session-1",
            "update": {
                "sessionUpdate": "tool_call_update",
                "toolCallId": "tool-1",
                "content": [{"type": "terminal"}]
            }
        }),
    ] {
        let mut state = NativeTurnState::new("kimi-session-1");
        assert!(
            matches!(
                codec.decode_notification(&notification(params), &mut state),
                Err(HarnessAdapterError::InvalidResponse(_))
            ),
            "malformed ACP shape was accepted"
        );
    }

    let mut state = NativeTurnState::new("kimi-session-1");
    assert!(
        codec
            .decode_notification(
                &JsonRpcNotification {
                    method: "wrong/update".to_owned(),
                    params: json!({}),
                },
                &mut state,
            )
            .is_err()
    );
}

#[test]
fn permission_requests_become_interaction_events_and_responses_are_validated() {
    let codec = codec(ReasoningEffort::Max);
    let native_params = json!({
        "sessionId": "kimi-session-1",
        "toolCall": {
            "toolCallId": "turn-1:tool-1",
            "title": "Modify source",
            "kind": "edit"
        },
        "options": [
            {"optionId": "approve", "name": "Approve once", "kind": "allow_once"},
            {"optionId": "reject", "name": "Reject", "kind": "reject_once"}
        ]
    });
    let request = JsonRpcServerRequest {
        id: JsonRpcId::from_number(42),
        method: "session/request_permission".to_owned(),
        params: native_params.clone(),
    };
    let event = codec.server_request_event(&request).unwrap();
    assert_eq!(event.event_type, "interaction.required");
    assert!(event.durable);
    assert_eq!(event.payload["native_request_id"], 42);
    assert_eq!(event.payload["native_method"], "session/request_permission");
    assert_eq!(event.payload["native_params"], native_params);

    let selected = codec
        .encode_server_response(
            "session/request_permission",
            &json!({
                "native_request_id": 42,
                "result": {"outcome": {"outcome": "selected", "optionId": "approve"}}
            }),
        )
        .unwrap();
    assert_eq!(
        selected,
        ServerResponse::Result(json!({"outcome": {"outcome": "selected", "optionId": "approve"}}))
    );
    let cancelled = codec
        .encode_server_response(
            "session/request_permission",
            &json!({"result": {"outcome": {"outcome": "cancelled"}}}),
        )
        .unwrap();
    assert_eq!(
        cancelled,
        ServerResponse::Result(json!({"outcome": {"outcome": "cancelled"}}))
    );

    for malformed in [
        json!({"result": {"outcome": {"outcome": "selected"}}}),
        json!({"result": {"outcome": {"outcome": "maybe"}}}),
        json!({"result": {}}),
    ] {
        assert!(
            codec
                .encode_server_response("session/request_permission", &malformed)
                .is_err()
        );
    }
    assert!(
        codec
            .server_request_event(&JsonRpcServerRequest {
                id: JsonRpcId::from_number(43),
                method: "session/request_permission".to_owned(),
                params: json!({"sessionId": "kimi-session-1", "toolCall": {}, "options": []}),
            })
            .is_err()
    );
}

#[test]
fn prompt_results_map_terminal_completion_and_usage() {
    let codec = codec(ReasoningEffort::Max);
    let completed = codec
        .decode_prompt_result(
            "kimi-session-1",
            json!({
                "stopReason": "end_turn",
                "usage": {"inputTokens": 10, "outputTokens": 4}
            }),
        )
        .unwrap();
    assert!(completed.terminal);
    assert_eq!(completed.event.event_type, "turn.completed");
    assert_eq!(
        completed.usage,
        Some(json!({"inputTokens": 10, "outputTokens": 4}))
    );
    assert_eq!(completed.event.payload["native"]["stopReason"], "end_turn");

    let cancelled = codec
        .decode_prompt_result("kimi-session-1", json!({"stopReason": "cancelled"}))
        .unwrap();
    assert_eq!(cancelled.event.event_type, "turn.cancelled");

    for malformed in [
        json!({}),
        json!({"stopReason": "unknown"}),
        json!({"stopReason": 1}),
        json!({"stopReason": "end_turn", "usage": "invalid"}),
    ] {
        assert!(
            codec
                .decode_prompt_result("kimi-session-1", malformed)
                .is_err()
        );
    }
}
