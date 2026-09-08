use std::collections::BTreeSet;
use std::fs;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

use serde_json::json;
use server_harness::middleware::harness::HarnessAdapterError;
use server_harness::middleware::pi_rpc::{
    PiEventCorrelation, PiEventDecoder, PiIsolatedHome, PiRpcCommand, PiRpcProcessConfig,
    PiRpcRuntime, PiThinkingLevel, build_models_json, thinking_level,
};
use server_harness::middleware::types::{ModelApiDialect, ReasoningEffort, ResolvedModelSelection};
use uuid::Uuid;

fn selection(effort: ReasoningEffort) -> ResolvedModelSelection {
    ResolvedModelSelection {
        provider_route: "omnisolo-openai-compatible".to_owned(),
        model_id: "gpt-5.6-luna".to_owned(),
        reasoning_effort: Some(effort),
        api_dialect: ModelApiDialect::OpenAiResponses,
        context_window: Some(400_000),
        max_output_tokens: Some(128_000),
        capabilities: BTreeSet::from(["reasoning".to_owned(), "tools".to_owned()]),
        binding_revision: "binding-v1".to_owned(),
        binding_digest: "sha256:test".to_owned(),
        metadata: [("api_key".to_owned(), json!("credential-canary"))]
            .into_iter()
            .collect(),
    }
}

fn correlation() -> PiEventCorrelation {
    PiEventCorrelation {
        session_id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
        task_id: Some(Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap()),
        turn_id: Some(Uuid::parse_str("00000000-0000-0000-0000-000000000003").unwrap()),
        attempt_id: "attempt-1".to_owned(),
        native_session_id: "pi-session-1".to_owned(),
    }
}

fn environment_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

#[test]
fn commands_match_pi_0731_rpc_shapes_without_jsonrpc_fields() {
    let cases = [
        (
            PiRpcCommand::prompt("cmd-1", "hello"),
            json!({"id":"cmd-1","type":"prompt","message":"hello"}),
        ),
        (
            PiRpcCommand::steer("cmd-2", "focus"),
            json!({"id":"cmd-2","type":"steer","message":"focus"}),
        ),
        (
            PiRpcCommand::follow_up("cmd-3", "continue"),
            json!({"id":"cmd-3","type":"follow_up","message":"continue"}),
        ),
        (
            PiRpcCommand::abort("cmd-4"),
            json!({"id":"cmd-4","type":"abort"}),
        ),
        (
            PiRpcCommand::set_model("cmd-5", "omnisolo-openai-compatible", "gpt-5.6-luna"),
            json!({
                "id":"cmd-5",
                "type":"set_model",
                "provider":"omnisolo-openai-compatible",
                "modelId":"gpt-5.6-luna"
            }),
        ),
        (
            PiRpcCommand::set_thinking_level("cmd-6", PiThinkingLevel::XHigh),
            json!({"id":"cmd-6","type":"set_thinking_level","level":"xhigh"}),
        ),
        (
            PiRpcCommand::get_state("cmd-7"),
            json!({"id":"cmd-7","type":"get_state"}),
        ),
        (
            PiRpcCommand::compact("cmd-8", Some("retain decisions")),
            json!({
                "id":"cmd-8",
                "type":"compact",
                "customInstructions":"retain decisions"
            }),
        ),
    ];

    for (command, expected) in cases {
        let wire = serde_json::to_value(command).unwrap();
        assert_eq!(wire, expected);
        assert!(wire.get("jsonrpc").is_none());
        assert!(!wire.to_string().contains("credential-canary"));
    }
}

#[tokio::test]
async fn ambient_parent_environment_is_not_inherited() {
    let _guard = environment_lock().lock().unwrap();
    unsafe {
        std::env::set_var("UNRELATED_DEPLOYMENT_SECRET", "CANARY-AMBIENT-7KQ9");
    }

    let mut config = PiRpcProcessConfig::shell(
        r#"
read line
ambient=false
[ -n "${UNRELATED_DEPLOYMENT_SECRET:-}" ] && ambient=true
explicit=false
[ -n "${OPENAI_API_KEY:-}" ] && explicit=true
path=false
[ -n "${PATH:-}" ] && path=true
printf '{"type":"response","id":"environment-check","command":"get_state","success":true,"data":{"ambient":%s,"explicit":%s,"path":%s}}\n' "$ambient" "$explicit" "$path"
"#,
    );
    config
        .environment
        .insert("OPENAI_API_KEY".to_owned(), "explicit-key".to_owned());
    let runtime = PiRpcRuntime::spawn(config).await.unwrap();
    let response = runtime
        .request(PiRpcCommand::get_state("environment-check"))
        .await
        .unwrap();

    unsafe {
        std::env::remove_var("UNRELATED_DEPLOYMENT_SECRET");
    }
    assert_eq!(
        response.data.unwrap(),
        json!({"ambient": false, "explicit": true, "path": true})
    );
    runtime.shutdown().await.unwrap();
}

#[test]
fn all_reasoning_efforts_map_explicitly_and_max_uses_xhigh() {
    assert_eq!(thinking_level(&ReasoningEffort::None).unwrap(), None);
    assert_eq!(
        thinking_level(&ReasoningEffort::Minimal).unwrap(),
        Some(PiThinkingLevel::Minimal)
    );
    assert_eq!(
        thinking_level(&ReasoningEffort::Low).unwrap(),
        Some(PiThinkingLevel::Low)
    );
    assert_eq!(
        thinking_level(&ReasoningEffort::Medium).unwrap(),
        Some(PiThinkingLevel::Medium)
    );
    assert_eq!(
        thinking_level(&ReasoningEffort::High).unwrap(),
        Some(PiThinkingLevel::High)
    );
    assert_eq!(
        thinking_level(&ReasoningEffort::Max).unwrap(),
        Some(PiThinkingLevel::XHigh)
    );
    assert!(thinking_level(&ReasoningEffort::Custom).is_err());
}

#[test]
fn models_json_is_openai_responses_and_references_environment_credential() {
    let config = build_models_json(
        &selection(ReasoningEffort::Max),
        "https://llmapi.omnisolo.co/v1",
    )
    .unwrap();
    let provider = &config["providers"]["omnisolo-openai-compatible"];

    assert_eq!(provider["baseUrl"], "https://llmapi.omnisolo.co/v1");
    assert_eq!(provider["api"], "openai-responses");
    assert_eq!(provider["apiKey"], "OPENAI_API_KEY");
    assert_eq!(provider["models"][0]["id"], "gpt-5.6-luna");
    assert_eq!(provider["models"][0]["reasoning"], true);
    assert_eq!(provider["models"][0]["contextWindow"], 400_000);
    assert_eq!(provider["models"][0]["maxTokens"], 128_000);
    assert_eq!(provider["models"][0]["thinkingLevelMap"]["xhigh"], "max");
    let wire = serde_json::to_string(&config).unwrap();
    assert!(!wire.contains("credential-canary"));
}

#[test]
fn models_json_rejects_credential_bearing_base_urls() {
    for base_url in [
        "https://credential-canary@llmapi.omnisolo.co/v1",
        "https://llmapi.omnisolo.co/v1?api_key=credential-canary",
        "https://llmapi.omnisolo.co/v1#credential-canary",
    ] {
        let error = build_models_json(&selection(ReasoningEffort::Max), base_url).unwrap_err();
        assert!(matches!(error, HarnessAdapterError::InvalidRequest(_)));
        assert!(!error.to_string().contains("credential-canary"));
    }
}

#[test]
fn isolated_home_writes_mode_0600_config_and_removes_it_on_drop() {
    let parent = std::env::temp_dir().join(format!("omnisolo-pi-test-{}", Uuid::new_v4()));
    fs::create_dir_all(&parent).unwrap();
    let agent_dir;
    {
        let home = PiIsolatedHome::create_in(
            &parent,
            &selection(ReasoningEffort::Max),
            "https://llmapi.omnisolo.co/v1",
        )
        .unwrap();
        agent_dir = home.agent_dir().to_owned();
        assert_eq!(home.environment()["PI_CODING_AGENT_DIR"], agent_dir);
        let models_path = agent_dir.join("models.json");
        let text = fs::read_to_string(&models_path).unwrap();
        assert!(!text.contains("credential-canary"));
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                fs::metadata(models_path).unwrap().permissions().mode() & 0o777,
                0o600
            );
        }
    }
    assert!(!agent_dir.exists());
    fs::remove_dir_all(parent).unwrap();
}

#[test]
fn decoder_maps_streams_tools_usage_terminal_and_preserves_native_context() {
    let mut decoder = PiEventDecoder::new(correlation());
    let events = [
        json!({"type":"agent_start"}),
        json!({
            "type":"message_update",
            "message":{"role":"assistant","content":[]},
            "assistantMessageEvent":{"type":"text_delta","contentIndex":0,"delta":"hello","partial":{}}
        }),
        json!({
            "type":"message_update",
            "message":{"role":"assistant","content":[]},
            "assistantMessageEvent":{"type":"thinking_delta","contentIndex":1,"delta":"reason","partial":{}}
        }),
        json!({"type":"tool_execution_start","toolCallId":"tool-1","toolName":"read","args":{"path":"README.md"}}),
        json!({"type":"tool_execution_update","toolCallId":"tool-1","toolName":"read","args":{"path":"README.md"},"partialResult":{"content":"partial"}}),
        json!({"type":"tool_execution_end","toolCallId":"tool-1","toolName":"read","result":{"content":"done"},"isError":false}),
        json!({
            "type":"message_end",
            "message":{
                "role":"assistant",
                "content":[{"type":"text","text":"hello world"}],
                "api":"openai-responses",
                "provider":"omnisolo-openai-compatible",
                "model":"gpt-5.6-luna",
                "usage":{"input":10,"output":2,"cacheRead":0,"cacheWrite":0,"totalTokens":12,"cost":{"input":0,"output":0,"cacheRead":0,"cacheWrite":0,"total":0}},
                "stopReason":"stop",
                "timestamp":1
            }
        }),
        json!({
            "type":"agent_end",
            "messages":[{"role":"assistant","content":[],"usage":{"totalTokens":12},"stopReason":"stop"}]
        }),
    ];
    let decoded = events
        .into_iter()
        .map(|event| decoder.decode(event).unwrap())
        .collect::<Vec<_>>();

    assert_eq!(
        decoded
            .iter()
            .map(|event| event.event.event_type.as_str())
            .collect::<Vec<_>>(),
        vec![
            "turn.started",
            "assistant.text_chunk",
            "assistant.reasoning",
            "tool.started",
            "tool.output",
            "tool.completed",
            "assistant.final",
            "turn.completed",
        ]
    );
    assert_eq!(decoded[1].event.payload["content"], "hello");
    assert_eq!(decoded[6].final_text.as_deref(), Some("hello world"));
    assert_eq!(decoded[6].usage.as_ref().unwrap()["totalTokens"], 12);
    assert!(decoded[7].terminal);
    for event in decoded {
        assert_eq!(
            event.event.payload["session_id"],
            correlation().session_id.to_string()
        );
        assert_eq!(
            event.event.payload["task_id"],
            correlation().task_id.unwrap().to_string()
        );
        assert_eq!(
            event.event.payload["turn_id"],
            correlation().turn_id.unwrap().to_string()
        );
        assert_eq!(event.event.payload["attempt_id"], "attempt-1");
        assert!(event.event.payload.get("native").is_some());
    }
}

#[test]
fn decoder_maps_provider_error_and_abort_terminals_fail_closed() {
    for (reason, expected) in [
        ("error", "turn.failed"),
        ("aborted", "turn.cancelled"),
        ("future_reason", "turn.failed"),
    ] {
        let mut decoder = PiEventDecoder::new(correlation());
        let event = decoder
            .decode(json!({
                "type":"agent_end",
                "messages":[{"role":"assistant","content":[],"stopReason":reason,"errorMessage":"failed"}]
            }))
            .unwrap();
        assert_eq!(event.event.event_type, expected);
        assert!(event.terminal);
    }
}

#[test]
fn decoder_exposes_session_state_and_compaction_events() {
    let mut decoder = PiEventDecoder::new(correlation());
    let changed = decoder
        .decode(json!({"type":"session_info_changed","name":"portable task"}))
        .unwrap();
    let compacted = decoder
        .decode(json!({
            "type":"compaction_end",
            "reason":"manual",
            "result":{"summary":"kept"},
            "aborted":false,
            "willRetry":false
        }))
        .unwrap();
    assert_eq!(changed.event.event_type, "session.state_changed");
    assert_eq!(compacted.event.event_type, "session.compacted");
}

#[test]
fn decoder_recursively_sanitizes_native_provider_metadata() {
    let mut decoder = PiEventDecoder::new(correlation());
    let event = decoder
        .decode(json!({
            "type": "session_info_changed",
            "_meta": {
                "api_key": "credential-canary",
                "nested": ["authorization=credential-canary"]
            }
        }))
        .unwrap();
    let encoded = serde_json::to_string(&event.event.payload).unwrap();
    assert!(!encoded.contains("credential-canary"));
    assert!(!encoded.contains("api_key"));
}

#[test]
fn decoder_does_not_treat_user_or_tool_result_messages_as_assistant_finals() {
    let mut decoder = PiEventDecoder::new(correlation());
    let user = decoder
        .decode(json!({
            "type":"message_end",
            "message":{"role":"user","content":"question","timestamp":1}
        }))
        .unwrap();
    let tool_result = decoder
        .decode(json!({
            "type":"message_end",
            "message":{
                "role":"toolResult",
                "toolCallId":"tool-1",
                "toolName":"read",
                "content":[{"type":"text","text":"result"}],
                "isError":false,
                "timestamp":2
            }
        }))
        .unwrap();

    assert_eq!(user.event.event_type, "conversation.user_message");
    assert_eq!(tool_result.event.event_type, "tool.result_message");
    assert_eq!(user.final_text, None);
    assert_eq!(tool_result.final_text, None);
}

#[tokio::test]
async fn fake_process_jsonl_e2e_correlates_commands_and_streamed_events() {
    let runtime = PiRpcRuntime::spawn(PiRpcProcessConfig::shell(
        r#"
while IFS= read -r line; do
  id=$(printf '%s' "$line" | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')
  type=$(printf '%s' "$line" | sed -n 's/.*"type":"\([^"]*\)".*/\1/p')
  case "$type" in
    set_model) printf '%s\n' "{\"id\":\"$id\",\"type\":\"response\",\"command\":\"set_model\",\"success\":true,\"data\":{\"provider\":\"omnisolo-openai-compatible\",\"id\":\"gpt-5.6-luna\"}}" ;;
    set_thinking_level) printf '%s\n' "{\"id\":\"$id\",\"type\":\"response\",\"command\":\"set_thinking_level\",\"success\":true}" ;;
    prompt)
      printf '%s\n' "{\"id\":\"$id\",\"type\":\"response\",\"command\":\"prompt\",\"success\":true}"
      printf '%s\n' '{"type":"agent_start"}'
      printf '%s\n' '{"type":"message_update","message":{"role":"assistant","content":[]},"assistantMessageEvent":{"type":"text_delta","contentIndex":0,"delta":"real","partial":{}}}'
      printf '%s\n' '{"type":"message_end","message":{"role":"assistant","content":[{"type":"text","text":"real answer"}],"usage":{"input":1,"output":2,"totalTokens":3},"stopReason":"stop"}}'
      printf '%s\n' '{"type":"agent_end","messages":[{"role":"assistant","content":[],"usage":{"totalTokens":3},"stopReason":"stop"}]}'
      ;;
    get_state) printf '%s\n' "{\"id\":\"$id\",\"type\":\"response\",\"command\":\"get_state\",\"success\":true,\"data\":{\"sessionId\":\"pi-session-1\",\"thinkingLevel\":\"xhigh\",\"isStreaming\":false}}" ;;
    steer|follow_up|abort|compact) printf '%s\n' "{\"id\":\"$id\",\"type\":\"response\",\"command\":\"$type\",\"success\":true}" ;;
  esac
done
"#,
    ))
    .await
    .unwrap();

    runtime
        .request(PiRpcCommand::set_model(
            "model-1",
            "omnisolo-openai-compatible",
            "gpt-5.6-luna",
        ))
        .await
        .unwrap();
    runtime
        .request(PiRpcCommand::set_thinking_level(
            "thinking-1",
            PiThinkingLevel::XHigh,
        ))
        .await
        .unwrap();
    runtime
        .prompt(correlation(), PiRpcCommand::prompt("prompt-1", "answer"))
        .await
        .unwrap();

    let mut received = Vec::new();
    loop {
        let event = tokio::time::timeout(Duration::from_secs(2), runtime.next_event())
            .await
            .unwrap()
            .unwrap();
        let terminal = event.terminal;
        received.push(event);
        if terminal {
            break;
        }
    }
    assert_eq!(received[1].event.payload["content"], "real");
    assert_eq!(received[2].final_text.as_deref(), Some("real answer"));
    assert_eq!(received.last().unwrap().event.event_type, "turn.completed");

    let state = runtime
        .request(PiRpcCommand::get_state("state-1"))
        .await
        .unwrap();
    assert_eq!(state.data.unwrap()["sessionId"], "pi-session-1");
    runtime
        .request(PiRpcCommand::steer("steer-1", "focus"))
        .await
        .unwrap();
    runtime
        .request(PiRpcCommand::follow_up("follow-1", "continue"))
        .await
        .unwrap();
    runtime
        .request(PiRpcCommand::abort("abort-1"))
        .await
        .unwrap();
    runtime
        .request(PiRpcCommand::compact("compact-1", None))
        .await
        .unwrap();
    runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn runtime_accepts_pi_0731_thinking_change_before_setup_response() {
    let runtime = PiRpcRuntime::spawn(PiRpcProcessConfig::shell(
        r#"
while IFS= read -r line; do
  id=$(printf '%s' "$line" | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')
  type=$(printf '%s' "$line" | sed -n 's/.*"type":"\([^"]*\)".*/\1/p')
  case "$type" in
    set_model)
      printf '%s\n' '{"type":"thinking_level_changed","level":"off"}'
      printf '%s\n' "{\"id\":\"$id\",\"type\":\"response\",\"command\":\"set_model\",\"success\":true}"
      ;;
    prompt)
      printf '%s\n' "{\"id\":\"$id\",\"type\":\"response\",\"command\":\"prompt\",\"success\":true}"
      printf '%s\n' '{"type":"agent_start"}'
      printf '%s\n' '{"type":"agent_end","messages":[{"role":"assistant","content":[],"stopReason":"stop"}]}'
      ;;
  esac
done
"#,
    ))
    .await
    .unwrap();

    runtime
        .request(PiRpcCommand::set_model(
            "model-1",
            "omnisolo-openai-compatible",
            "gpt-5.6-luna",
        ))
        .await
        .unwrap();
    runtime
        .prompt(correlation(), PiRpcCommand::prompt("prompt-1", "answer"))
        .await
        .unwrap();

    let started = runtime.next_event().await.unwrap();
    assert_eq!(started.event.event_type, "turn.started");
    assert_eq!(started.event.payload["attempt_id"], "attempt-1");
    assert_eq!(
        runtime.next_event().await.unwrap().event.event_type,
        "turn.completed"
    );
    runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn runtime_reports_response_mismatch_remote_error_malformed_frame_and_exit() {
    let mismatch = PiRpcRuntime::spawn(PiRpcProcessConfig::shell(
        r#"read line; printf '%s\n' '{"id":"request-1","type":"response","command":"abort","success":true}'"#,
    ))
    .await
    .unwrap();
    let error = mismatch
        .request(PiRpcCommand::get_state("request-1"))
        .await
        .unwrap_err();
    assert!(error.to_string().contains("command"));

    let remote = PiRpcRuntime::spawn(PiRpcProcessConfig::shell(
        r#"read line; printf '%s\n' '{"id":"request-2","type":"response","command":"set_model","success":false,"error":"Model not found"}'"#,
    ))
    .await
    .unwrap();
    assert!(
        remote
            .request(PiRpcCommand::set_model("request-2", "provider", "missing"))
            .await
            .unwrap_err()
            .to_string()
            .contains("Model not found")
    );

    let malformed = PiRpcRuntime::spawn(PiRpcProcessConfig::shell("printf '%s\\n' '{bad-json}'"))
        .await
        .unwrap();
    assert!(
        malformed
            .next_event()
            .await
            .unwrap_err()
            .to_string()
            .contains("malformed")
    );

    let exited = PiRpcRuntime::spawn(PiRpcProcessConfig::shell("exit 0"))
        .await
        .unwrap();
    assert!(
        exited
            .next_event()
            .await
            .unwrap_err()
            .to_string()
            .contains("exited")
    );

    let pending_exit = PiRpcRuntime::spawn(PiRpcProcessConfig::shell("read line; exit 0"))
        .await
        .unwrap();
    assert!(matches!(
        pending_exit
            .request(PiRpcCommand::get_state("pending-exit"))
            .await,
        Err(HarnessAdapterError::ProcessExited)
    ));
}

#[tokio::test]
async fn runtime_rejects_overlapping_prompts_to_protect_event_correlation() {
    let runtime = PiRpcRuntime::spawn(PiRpcProcessConfig::shell(
        r#"
while IFS= read -r line; do
  id=$(printf '%s' "$line" | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')
  printf '%s\n' "{\"id\":\"$id\",\"type\":\"response\",\"command\":\"prompt\",\"success\":true}"
done
"#,
    ))
    .await
    .unwrap();
    runtime
        .prompt(correlation(), PiRpcCommand::prompt("prompt-1", "first"))
        .await
        .unwrap();
    assert!(matches!(
        runtime
            .prompt(correlation(), PiRpcCommand::prompt("prompt-2", "second"))
            .await,
        Err(HarnessAdapterError::InvalidRequest(_))
    ));
    runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn shutdown_closes_stdin_so_pi_can_run_its_cleanup_path() {
    let parent = std::env::temp_dir().join(format!("omnisolo-pi-shutdown-{}", Uuid::new_v4()));
    fs::create_dir_all(&parent).unwrap();
    let marker = parent.join("clean-exit");
    let script = format!(
        "while IFS= read -r line; do :; done\nprintf '%s' clean > '{}'",
        marker.display()
    );
    let runtime = PiRpcRuntime::spawn(PiRpcProcessConfig::shell(script))
        .await
        .unwrap();

    runtime.shutdown().await.unwrap();

    assert_eq!(fs::read_to_string(&marker).unwrap(), "clean");
    fs::remove_dir_all(parent).unwrap();
}
