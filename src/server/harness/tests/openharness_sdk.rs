use std::collections::{BTreeMap, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

use serde_json::json;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use uuid::Uuid;

use server_harness::middleware::openharness::{
    OPENHARNESS_MAX_FRAME_BYTES, OpenHarnessCodec, OpenHarnessCommand, OpenHarnessEvent,
    OpenHarnessEventCorrelation, OpenHarnessEventDecoder, OpenHarnessProcessConfig,
    OpenHarnessResolvedModel, OpenHarnessRuntime,
};

fn resolved_model() -> OpenHarnessResolvedModel {
    OpenHarnessResolvedModel {
        provider: "openai".to_owned(),
        model_id: "gpt-5.6-luna".to_owned(),
        base_url: "https://llmapi.omnisolo.co/v1".to_owned(),
        reasoning_effort: "max".to_owned(),
        api_dialect: "openai_responses".to_owned(),
        context_window_tokens: 400_000,
        max_output_tokens: 65_536,
        capabilities: vec![
            "text".to_owned(),
            "tools".to_owned(),
            "reasoning".to_owned(),
        ],
        binding_revision: "binding-r17".to_owned(),
        binding_digest: "sha256:0123456789abcdef".to_owned(),
    }
}

#[test]
fn resolved_prompt_codec_carries_the_complete_immutable_model_contract() {
    let command = OpenHarnessCommand::prompt_with_resolved_model(
        "prompt-contract",
        "native-session",
        "inspect",
        resolved_model(),
        "bypass",
        "/workspace/project",
    );
    let frame = serde_json::to_value(&command).unwrap();

    assert_eq!(frame["resolved_model"]["provider"], "openai");
    assert_eq!(frame["resolved_model"]["model_id"], "gpt-5.6-luna");
    assert_eq!(frame["resolved_model"]["reasoning_effort"], "max");
    assert_eq!(frame["resolved_model"]["api_dialect"], "openai_responses");
    assert_eq!(frame["resolved_model"]["context_window_tokens"], 400_000);
    assert_eq!(frame["resolved_model"]["max_output_tokens"], 65_536);
    assert_eq!(
        frame["resolved_model"]["capabilities"],
        json!(["text", "tools", "reasoning"])
    );
    assert_eq!(frame["resolved_model"]["binding_revision"], "binding-r17");
    assert_eq!(
        frame["resolved_model"]["binding_digest"],
        "sha256:0123456789abcdef"
    );
    assert!(frame.get("model").is_none());
    assert!(frame.get("base_url").is_none());
}

#[test]
fn command_codec_encodes_session_prompt_steer_cancel_and_shutdown_frames() {
    let commands = [
        (
            OpenHarnessCommand::create_session("create-1"),
            json!({"id":"create-1","type":"create_session"}),
        ),
        (
            OpenHarnessCommand::resume_session("resume-1", "native-session-1"),
            json!({
                "id":"resume-1",
                "type":"resume_session",
                "session_id":"native-session-1"
            }),
        ),
        (
            OpenHarnessCommand::prompt(
                "prompt-1",
                "native-session-1",
                "Inspect the repository",
                "gpt-5.6-luna",
                "https://llmapi.omnisolo.co/v1",
                "bypass",
                "/workspace/project",
            ),
            json!({
                "id":"prompt-1",
                "type":"prompt",
                "session_id":"native-session-1",
                "prompt":"Inspect the repository",
                "resolved_model":{
                    "provider":"openai",
                    "model_id":"gpt-5.6-luna",
                    "base_url":"https://llmapi.omnisolo.co/v1",
                    "reasoning_effort":"max",
                    "api_dialect":"openai_chat_completions",
                    "context_window_tokens":200000,
                    "max_output_tokens":16384,
                    "capabilities":["text","tools","streaming"],
                    "binding_revision":"legacy-v1",
                    "binding_digest":"unresolved"
                },
                "permission_mode":"bypass",
                "cwd":"/workspace/project"
            }),
        ),
        (
            OpenHarnessCommand::steer("steer-1", "native-session-1", "Focus on tests"),
            json!({
                "id":"steer-1",
                "type":"steer",
                "session_id":"native-session-1",
                "message":"Focus on tests"
            }),
        ),
        (
            OpenHarnessCommand::cancel("cancel-1", "native-session-1"),
            json!({
                "id":"cancel-1",
                "type":"cancel",
                "session_id":"native-session-1"
            }),
        ),
        (
            OpenHarnessCommand::shutdown("shutdown-1"),
            json!({"id":"shutdown-1","type":"shutdown"}),
        ),
    ];

    for (command, expected) in commands {
        assert_eq!(serde_json::to_value(command).unwrap(), expected);
    }
}

#[test]
fn command_codec_writes_one_jsonl_frame_and_rejects_credential_bearing_urls() {
    let codec = OpenHarnessCodec;
    let command = OpenHarnessCommand::prompt(
        "prompt-1",
        "session-1",
        "Inspect the repository",
        "gpt-5.6-luna",
        "https://llmapi.omnisolo.co/v1",
        "bypass",
        "/workspace/project",
    );
    let wire = codec.encode_command_line(&command).unwrap();
    assert_eq!(wire.last(), Some(&b'\n'));
    assert_eq!(
        serde_json::from_slice::<OpenHarnessCommand>(&wire).unwrap(),
        command
    );

    for base_url in [
        "https://openai-key-canary@llmapi.omnisolo.co/v1",
        "https://llmapi.omnisolo.co/v1?api_key=openai-key-canary",
        "https://llmapi.omnisolo.co/v1#openai-key-canary",
    ] {
        let command = OpenHarnessCommand::prompt(
            "prompt-1",
            "session-1",
            "Inspect the repository",
            "gpt-5.6-luna",
            base_url,
            "bypass",
            "/workspace/project",
        );
        let error = codec.encode_command_line(&command).unwrap_err();
        assert!(!error.to_string().contains("openai-key-canary"));
    }
}

#[test]
fn event_codec_decodes_streamed_text_tools_results_and_terminal_outcomes() {
    let codec = OpenHarnessCodec;
    let frames = [
        json!({
            "type":"text",
            "request_id":"prompt-1",
            "session_id":"native-session-1",
            "text":"hello",
            "is_partial":true
        }),
        json!({
            "type":"tool_use",
            "request_id":"prompt-1",
            "session_id":"native-session-1",
            "tool_use_id":"tool-1",
            "name":"Read",
            "args":{"path":"README.md"}
        }),
        json!({
            "type":"tool_result",
            "request_id":"prompt-1",
            "session_id":"native-session-1",
            "tool_use_id":"tool-1",
            "content":"done",
            "is_error":false,
            "display":"README.md"
        }),
        json!({
            "type":"result",
            "request_id":"prompt-1",
            "session_id":"native-session-1",
            "text":"finished",
            "turns":2,
            "tool_calls":1,
            "total_tokens":17,
            "total_cost":0.25,
            "stop_reason":"end_turn"
        }),
        json!({
            "type":"cancelled",
            "request_id":"prompt-2",
            "session_id":"native-session-1"
        }),
    ];

    let decoded = frames
        .iter()
        .map(|frame| codec.decode_event_line(&frame.to_string()).unwrap())
        .collect::<Vec<_>>();

    assert!(matches!(decoded[0], OpenHarnessEvent::Text { .. }));
    assert!(matches!(decoded[1], OpenHarnessEvent::ToolUse { .. }));
    assert!(matches!(decoded[2], OpenHarnessEvent::ToolResult { .. }));
    assert!(!decoded[2].terminal());
    assert!(matches!(decoded[3], OpenHarnessEvent::Result { .. }));
    assert!(decoded[3].terminal());
    assert!(matches!(decoded[4], OpenHarnessEvent::Cancelled { .. }));
    assert!(decoded[4].terminal());
}

#[test]
fn canonical_decoder_maps_every_stream_variant_durability_terminal_and_cursor() {
    let session_id = Uuid::new_v4();
    let task_id = Uuid::new_v4();
    let turn_id = Uuid::new_v4();
    let mut decoder = OpenHarnessEventDecoder::new(OpenHarnessEventCorrelation {
        session_id,
        task_id: Some(task_id),
        turn_id: Some(turn_id),
        attempt_id: "attempt-all-events".to_owned(),
        native_session_id: "native-session-all-events".to_owned(),
    });
    let base = || {
        (
            "request-1".to_owned(),
            "native-session-all-events".to_owned(),
        )
    };
    let events = vec![
        OpenHarnessEvent::Text {
            request_id: base().0,
            session_id: base().1,
            text: "partial".to_owned(),
            is_partial: true,
        },
        OpenHarnessEvent::Text {
            request_id: base().0,
            session_id: base().1,
            text: "snapshot".to_owned(),
            is_partial: false,
        },
        OpenHarnessEvent::System {
            request_id: base().0,
            session_id: base().1,
            event: "session_start".to_owned(),
            data: json!({"phase":"running"}),
        },
        OpenHarnessEvent::System {
            request_id: base().0,
            session_id: base().1,
            event: "checkpoint".to_owned(),
            data: json!({"checkpoint":"cp-1"}),
        },
        OpenHarnessEvent::Downgrade {
            request_id: base().0,
            session_id: base().1,
            field: "reasoning_effort".to_owned(),
            requested: json!("max"),
            applied: json!("high"),
            reason: "native maximum".to_owned(),
            binding_revision: "binding-v1".to_owned(),
            binding_digest: "sha256:binding".to_owned(),
        },
        OpenHarnessEvent::Compaction {
            request_id: base().0,
            session_id: base().1,
            tokens_before: 100,
            tokens_after: 40,
            summary: "compacted".to_owned(),
        },
        OpenHarnessEvent::ToolUse {
            request_id: base().0,
            session_id: base().1,
            tool_use_id: "tool-1".to_owned(),
            name: "Read".to_owned(),
            args: json!({"path":"README.md"}),
        },
        OpenHarnessEvent::ToolResult {
            request_id: base().0,
            session_id: base().1,
            tool_use_id: "tool-1".to_owned(),
            content: "ok".to_owned(),
            is_error: false,
            display: Some("README.md".to_owned()),
        },
        OpenHarnessEvent::ToolResult {
            request_id: base().0,
            session_id: base().1,
            tool_use_id: "tool-2".to_owned(),
            content: "failed".to_owned(),
            is_error: true,
            display: None,
        },
        OpenHarnessEvent::Error {
            request_id: base().0,
            session_id: Some(base().1),
            code: "warning".to_owned(),
            message: "retrying".to_owned(),
            terminal: false,
        },
        OpenHarnessEvent::Error {
            request_id: base().0,
            session_id: Some(base().1),
            code: "provider".to_owned(),
            message: "rejected".to_owned(),
            terminal: true,
        },
        OpenHarnessEvent::Result {
            request_id: base().0,
            session_id: base().1,
            text: "finished".to_owned(),
            turns: 3,
            tool_calls: 2,
            total_tokens: 55,
            total_cost: 0.5,
            stop_reason: "end_turn".to_owned(),
        },
        OpenHarnessEvent::Cancelled {
            request_id: base().0,
            session_id: base().1,
        },
    ];
    let expected = [
        ("assistant.text_chunk", false, false),
        ("assistant.text_chunk", true, false),
        ("turn.started", true, false),
        ("native.openharness.system", false, false),
        ("model.contract_downgraded", true, false),
        ("session.compacted", true, false),
        ("tool.started", true, false),
        ("tool.completed", true, false),
        ("tool.failed", true, false),
        ("native.openharness.error", false, false),
        ("turn.failed", true, true),
        ("turn.completed", true, true),
        ("turn.cancelled", true, true),
    ];

    for (index, (native, (event_type, durable, terminal))) in
        events.into_iter().zip(expected).enumerate()
    {
        let decoded = decoder.decode(native).unwrap();
        assert_eq!(decoded.event.event_type, event_type);
        assert_eq!(decoded.event.durable, durable);
        assert_eq!(decoded.terminal, terminal);
        assert_eq!(decoded.event.payload["session_id"], session_id.to_string());
        assert_eq!(decoded.event.payload["task_id"], task_id.to_string());
        assert_eq!(decoded.event.payload["turn_id"], turn_id.to_string());
        assert_eq!(
            decoded.event.native_cursor.as_deref(),
            Some(format!("native-session-all-events:{}", index + 1).as_str())
        );
        if event_type == "turn.completed" {
            assert_eq!(decoded.final_text.as_deref(), Some("finished"));
            assert_eq!(decoded.usage.as_ref().unwrap()["total_tokens"], 55);
        }
    }

    for command_event in [
        OpenHarnessEvent::Ready {
            protocol_version: 1,
            sdk_version: "0.6.0".to_owned(),
        },
        OpenHarnessEvent::Ack {
            id: "ack-1".to_owned(),
            command: "prompt".to_owned(),
        },
        OpenHarnessEvent::Session {
            id: "session-1".to_owned(),
            command: "create_session".to_owned(),
            session_id: "native-session-all-events".to_owned(),
        },
        OpenHarnessEvent::SessionDeleted {
            id: "delete-1".to_owned(),
            command: "delete_session".to_owned(),
            session_id: "native-session-all-events".to_owned(),
        },
    ] {
        assert!(decoder.decode(command_event).is_err());
    }
}

#[test]
fn event_codec_rejects_malformed_or_unknown_sidecar_frames() {
    let codec = OpenHarnessCodec;

    for malformed in [
        "{not-json}",
        r#"{"type":"text","request_id":"prompt-1"}"#,
        r#"{"type":"text","request_id":"","session_id":"session-1","text":"x","is_partial":true}"#,
        r#"{"type":"future_sdk_event","request_id":"prompt-1"}"#,
    ] {
        assert!(codec.decode_event_line(malformed).is_err(), "{malformed}");
    }
}

fn raw_runtime_config(script: &str, timeout: Duration) -> OpenHarnessProcessConfig {
    OpenHarnessProcessConfig {
        executable: "python3".to_owned(),
        args: vec!["-c".to_owned(), script.to_owned()],
        environment: BTreeMap::from([(
            "OPENAI_API_KEY".to_owned(),
            "runtime-debug-secret-canary".to_owned(),
        )]),
        request_timeout: timeout,
    }
}

fn fake_runtime_config(script: &str, timeout: Duration) -> OpenHarnessProcessConfig {
    raw_runtime_config(
        &format!(
            "import json\nprint(json.dumps({{\"type\":\"ready\",\"protocol_version\":1,\"sdk_version\":\"0.6.0\"}}), flush=True)\n{script}"
        ),
        timeout,
    )
}

#[tokio::test]
async fn native_runtime_requires_a_bounded_valid_ready_handshake() {
    let no_ready = OpenHarnessRuntime::spawn(raw_runtime_config(
        "import time; time.sleep(10)",
        Duration::from_millis(50),
    ))
    .await
    .unwrap_err();
    assert!(no_ready.to_string().contains("timed out"));

    let malformed = OpenHarnessRuntime::spawn(raw_runtime_config(
        "print('{not-ready}', flush=True)",
        Duration::from_secs(1),
    ))
    .await
    .unwrap_err();
    assert!(malformed.to_string().contains("ready"));

    let runtime = OpenHarnessRuntime::spawn(fake_runtime_config(
        r#"
import sys
for line in sys.stdin:
    command = json.loads(line)
    if command["type"] == "shutdown":
        print(json.dumps({"type":"ack","id":command["id"],"command":"shutdown"}), flush=True)
        break
"#,
        Duration::from_secs(1),
    ))
    .await
    .unwrap();
    runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn native_runtime_owns_a_private_temporary_home_and_cleans_it_up() {
    let record = std::env::temp_dir().join(format!("openharness-home-{}.json", Uuid::new_v4()));
    let script = r#"
import os
import stat
import sys
with open(os.environ["RECORD_PATH"], "w", encoding="utf-8") as stream:
    json.dump({
        "home": os.environ.get("HOME"),
        "session_home": os.environ.get("OPENHARNESS_SESSION_HOME"),
        "mode": stat.S_IMODE(os.stat(os.environ["HOME"]).st_mode),
    }, stream)
for line in sys.stdin:
    command = json.loads(line)
    if command["type"] == "shutdown":
        print(json.dumps({"type":"ack","id":command["id"],"command":"shutdown"}), flush=True)
        break
"#;
    let mut config = fake_runtime_config(script, Duration::from_secs(2));
    config.environment.insert(
        "RECORD_PATH".to_owned(),
        record.to_string_lossy().into_owned(),
    );
    config
        .environment
        .insert("HOME".to_owned(), "/must/not/be-used".to_owned());
    let runtime = OpenHarnessRuntime::spawn(config).await.unwrap();
    let snapshot: serde_json::Value = loop {
        if let Ok(contents) = fs::read_to_string(&record)
            && let Ok(snapshot) = serde_json::from_str(&contents)
        {
            break snapshot;
        }
        tokio::time::sleep(Duration::from_millis(5)).await;
    };
    let home = PathBuf::from(snapshot["home"].as_str().unwrap());
    assert!(home.starts_with("/tmp/"));
    assert_eq!(snapshot["session_home"], snapshot["home"]);
    assert_eq!(snapshot["mode"], 0o700);
    runtime.shutdown().await.unwrap();
    assert!(
        !home.exists(),
        "runtime-owned HOME must be cleaned on shutdown"
    );
    fs::remove_file(record).unwrap();
}

#[tokio::test]
async fn prompt_event_timeout_sends_cancel_instead_of_hanging_forever() {
    let record = std::env::temp_dir().join(format!("openharness-cancel-{}", Uuid::new_v4()));
    let script = r#"
import os
import sys
active = None
for line in sys.stdin:
    command = json.loads(line)
    if command["type"] == "prompt":
        active = (command["id"], command["session_id"])
        print(json.dumps({"type":"ack","id":command["id"],"command":"prompt"}), flush=True)
    elif command["type"] == "cancel":
        with open(os.environ["RECORD_PATH"], "w", encoding="utf-8") as stream:
            stream.write(command["session_id"])
        print(json.dumps({"type":"ack","id":command["id"],"command":"cancel"}), flush=True)
        print(json.dumps({"type":"cancelled","request_id":active[0],"session_id":active[1]}), flush=True)
        active = None
    elif command["type"] == "shutdown":
        print(json.dumps({"type":"ack","id":command["id"],"command":"shutdown"}), flush=True)
        break
"#;
    // Leave enough headroom for the child process to acknowledge the prompt
    // under a parallel test load while keeping the runtime's own event wait
    // shorter than the outer assertion timeout.
    let mut config = fake_runtime_config(script, Duration::from_millis(250));
    config.environment.insert(
        "RECORD_PATH".to_owned(),
        record.to_string_lossy().into_owned(),
    );
    let runtime = OpenHarnessRuntime::spawn(config).await.unwrap();
    runtime
        .prompt(OpenHarnessCommand::prompt_with_resolved_model(
            "timeout-prompt",
            "timeout-session",
            "wait",
            resolved_model(),
            "plan",
            "/workspace/project",
        ))
        .await
        .unwrap();
    let outcome = tokio::time::timeout(Duration::from_secs(1), runtime.next_event()).await;
    let error = outcome
        .expect("next_event must have its own timeout")
        .unwrap_err();
    assert!(error.to_string().contains("timed out"));
    for _ in 0..200 {
        if record.exists() {
            break;
        }
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
    assert_eq!(fs::read_to_string(&record).unwrap(), "timeout-session");
    assert!(matches!(
        runtime.next_event().await.unwrap(),
        OpenHarnessEvent::Cancelled { request_id, .. } if request_id == "timeout-prompt"
    ));
    runtime.shutdown().await.unwrap();
    fs::remove_file(record).unwrap();
}

#[tokio::test]
async fn dropping_a_prompt_stream_handle_cancels_the_active_sidecar_turn() {
    let record = std::env::temp_dir().join(format!("openharness-drop-{}", Uuid::new_v4()));
    let script = r#"
import os
import sys
active = None
for line in sys.stdin:
    command = json.loads(line)
    if command["type"] == "prompt":
        active = (command["id"], command["session_id"])
        print(json.dumps({"type":"ack","id":command["id"],"command":"prompt"}), flush=True)
    elif command["type"] == "cancel":
        with open(os.environ["RECORD_PATH"], "w", encoding="utf-8") as stream:
            stream.write(command["session_id"])
        print(json.dumps({"type":"ack","id":command["id"],"command":"cancel"}), flush=True)
        print(json.dumps({"type":"cancelled","request_id":active[0],"session_id":active[1]}), flush=True)
        active = None
    elif command["type"] == "shutdown":
        print(json.dumps({"type":"ack","id":command["id"],"command":"shutdown"}), flush=True)
        break
"#;
    let mut config = fake_runtime_config(script, Duration::from_secs(1));
    config.environment.insert(
        "RECORD_PATH".to_owned(),
        record.to_string_lossy().into_owned(),
    );
    let runtime = OpenHarnessRuntime::spawn(config).await.unwrap();
    let stream = runtime
        .prompt_stream(OpenHarnessCommand::prompt_with_resolved_model(
            "drop-prompt",
            "drop-session",
            "wait",
            resolved_model(),
            "plan",
            "/workspace/project",
        ))
        .await
        .unwrap();
    drop(stream);
    for _ in 0..100 {
        if record.exists() {
            break;
        }
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
    assert_eq!(fs::read_to_string(&record).unwrap(), "drop-session");
    assert!(matches!(
        runtime.next_event().await.unwrap(),
        OpenHarnessEvent::Cancelled { request_id, .. } if request_id == "drop-prompt"
    ));
    runtime.shutdown().await.unwrap();
    fs::remove_file(record).unwrap();
}

#[tokio::test]
async fn native_runtime_correlates_sessions_stream_controls_and_clean_shutdown() {
    let script = r#"
import json
import sys

active = None
session_id = "runtime-session-1"

def emit(frame):
    print(json.dumps(frame, separators=(",", ":")), flush=True)

for line in sys.stdin:
    command = json.loads(line)
    command_type = command["type"]
    command_id = command["id"]
    if command_type == "create_session":
        emit({"type":"session","id":command_id,"command":command_type,"session_id":session_id})
    elif command_type == "resume_session":
        session_id = command["session_id"]
        emit({"type":"session","id":command_id,"command":command_type,"session_id":session_id})
    elif command_type == "prompt":
        active = (command_id, command["session_id"])
        emit({"type":"ack","id":command_id,"command":command_type})
        emit({"type":"text","request_id":command_id,"session_id":command["session_id"],"text":"waiting","is_partial":True})
    elif command_type == "steer":
        emit({"type":"ack","id":command_id,"command":command_type})
        request_id, native_session_id = active
        emit({"type":"result","request_id":request_id,"session_id":native_session_id,"text":"steered result","turns":1,"tool_calls":0,"total_tokens":9,"total_cost":0.1,"stop_reason":"end_turn"})
        active = None
    elif command_type == "cancel":
        emit({"type":"ack","id":command_id,"command":command_type})
        request_id, native_session_id = active
        emit({"type":"cancelled","request_id":request_id,"session_id":native_session_id})
        active = None
    elif command_type == "shutdown":
        emit({"type":"ack","id":command_id,"command":command_type})
        break
"#;
    let config = fake_runtime_config(script, Duration::from_secs(2));
    let debug = format!("{config:?}");
    assert!(!debug.contains("runtime-debug-secret-canary"));
    assert!(!debug.contains(script));

    let runtime = OpenHarnessRuntime::spawn(config).await.unwrap();
    let created = runtime
        .request(OpenHarnessCommand::create_session("create-runtime"))
        .await
        .unwrap();
    assert!(matches!(
        created,
        OpenHarnessEvent::Session { session_id, .. } if session_id == "runtime-session-1"
    ));
    let resumed = runtime
        .request(OpenHarnessCommand::resume_session(
            "resume-runtime",
            "retained-runtime-session",
        ))
        .await
        .unwrap();
    assert!(matches!(
        resumed,
        OpenHarnessEvent::Session { session_id, .. } if session_id == "retained-runtime-session"
    ));

    runtime
        .prompt(OpenHarnessCommand::prompt(
            "prompt-runtime-1",
            "retained-runtime-session",
            "wait for steering",
            "gpt-5.6-luna",
            "https://llmapi.omnisolo.co/v1",
            "bypass",
            "/workspace/project",
        ))
        .await
        .unwrap();
    assert!(matches!(
        runtime.next_event().await.unwrap(),
        OpenHarnessEvent::Text { request_id, .. } if request_id == "prompt-runtime-1"
    ));
    assert!(matches!(
        runtime
            .request(OpenHarnessCommand::steer(
                "steer-runtime",
                "retained-runtime-session",
                "finish now",
            ))
            .await
            .unwrap(),
        OpenHarnessEvent::Ack { id, command } if id == "steer-runtime" && command == "steer"
    ));
    assert!(matches!(
        runtime.next_event().await.unwrap(),
        OpenHarnessEvent::Result { request_id, .. } if request_id == "prompt-runtime-1"
    ));

    runtime
        .prompt(OpenHarnessCommand::prompt(
            "prompt-runtime-2",
            "retained-runtime-session",
            "wait for cancellation",
            "gpt-5.6-luna",
            "https://llmapi.omnisolo.co/v1",
            "accept_edits",
            "/workspace/project",
        ))
        .await
        .unwrap();
    assert!(matches!(
        runtime.next_event().await.unwrap(),
        OpenHarnessEvent::Text { request_id, .. } if request_id == "prompt-runtime-2"
    ));
    runtime
        .request(OpenHarnessCommand::cancel(
            "cancel-runtime",
            "retained-runtime-session",
        ))
        .await
        .unwrap();
    assert!(matches!(
        runtime.next_event().await.unwrap(),
        OpenHarnessEvent::Cancelled { request_id, .. } if request_id == "prompt-runtime-2"
    ));
    runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn native_runtime_reports_timeout_malformed_output_and_process_exit() {
    let timeout_runtime = OpenHarnessRuntime::spawn(fake_runtime_config(
        "import sys, time; sys.stdin.readline(); time.sleep(10)",
        Duration::from_secs(1),
    ))
    .await
    .unwrap();
    let timeout_error = timeout_runtime
        .request(OpenHarnessCommand::create_session("timeout-command"))
        .await
        .unwrap_err();
    assert!(timeout_error.to_string().contains("timed out"));

    let malformed_runtime = OpenHarnessRuntime::spawn(fake_runtime_config(
        "import sys; sys.stdin.readline(); print('{not-json}', flush=True)",
        Duration::from_secs(1),
    ))
    .await
    .unwrap();
    let malformed_error = malformed_runtime
        .request(OpenHarnessCommand::create_session("malformed-command"))
        .await
        .unwrap_err();
    assert!(
        malformed_error
            .to_string()
            .contains("malformed OpenHarness JSONL frame")
    );

    let exit_runtime = OpenHarnessRuntime::spawn(fake_runtime_config(
        "import sys; sys.stdin.readline(); raise SystemExit(23)",
        Duration::from_secs(1),
    ))
    .await
    .unwrap();
    let exit_error = exit_runtime
        .request(OpenHarnessCommand::create_session("exit-command"))
        .await
        .unwrap_err();
    assert!(exit_error.to_string().contains("process exited"));
}

#[tokio::test]
async fn native_runtime_rejects_mismatched_command_correlation() {
    let script = r#"
import json
import sys
command = json.loads(sys.stdin.readline())
print(json.dumps({"type":"ack","id":command["id"],"command":"steer"}), flush=True)
"#;
    let runtime = OpenHarnessRuntime::spawn(fake_runtime_config(script, Duration::from_secs(1)))
        .await
        .unwrap();
    let error = runtime
        .request(OpenHarnessCommand::create_session("wrong-command"))
        .await
        .unwrap_err();
    assert!(error.to_string().contains("did not match"));
}

struct FakeSdk {
    root: PathBuf,
    record_path: PathBuf,
}

impl FakeSdk {
    fn create() -> Self {
        let root = std::env::temp_dir().join(format!("omnisolo-openharness-{}", Uuid::new_v4()));
        let package = root.join("harness");
        fs::create_dir_all(package.join("core")).unwrap();
        fs::create_dir_all(package.join("providers")).unwrap();
        fs::create_dir_all(package.join("types")).unwrap();
        fs::write(package.join("core/__init__.py"), "").unwrap();
        fs::write(
            package.join("core/engine.py"),
            r#"
from pathlib import Path

def load_toml_config(cwd=None):
    path = Path(cwd or ".") / ".harness" / "config.toml"
    if path.exists():
        return {"provider": "workspace-override", "model": "workspace-override", "router": {"strategy": "cost"}}
    return {}
"#,
        )
        .unwrap();
        fs::write(package.join("providers/__init__.py"), "").unwrap();
        fs::write(package.join("types/__init__.py"), "").unwrap();
        fs::write(package.join("providers/registry.py"), "MODELS = {}\n").unwrap();
        fs::write(
            package.join("types/providers.py"),
            r#"
from dataclasses import dataclass

@dataclass
class ModelInfo:
    id: str
    provider: str
    display_name: str
    context_window: int
    max_output_tokens: int
    supports_tools: bool = True
    supports_streaming: bool = True
    supports_vision: bool = False
"#,
        )
        .unwrap();
        fs::write(
            package.join("providers/openai.py"),
            r#"
class OpenAIProvider:
    def __init__(self, api_key=None, model="gpt-4o", base_url=None):
        self.api_key = api_key
        self.model_id = model
        self.base_url = base_url
"#,
        )
        .unwrap();
        fs::write(
            package.join("core/steering.py"),
            r#"
import json
import os

class SteeringChannel:
    def __init__(self):
        self.messages = []

    async def send(self, message):
        self.messages.append(message)
        with open(os.environ["FAKE_HARNESS_RECORD"], "a", encoding="utf-8") as stream:
            stream.write(json.dumps({"kind": "steer", "message": message}) + "\n")

    async def close(self):
        return None
"#,
        )
        .unwrap();
        fs::write(
            package.join("__init__.py"),
            r#"
import asyncio
import json
import os
import subprocess
import sys

__version__ = "0.6.0"

class TextMessage:
    def __init__(self, text, is_partial=True):
        self.text = text
        self.is_partial = is_partial

class SystemEvent:
    def __init__(self, type, data):
        self.type = type
        self.data = data

class CompactionEvent:
    def __init__(self, tokens_before, tokens_after, summary):
        self.tokens_before = tokens_before
        self.tokens_after = tokens_after
        self.summary = summary

class ToolUse:
    def __init__(self, id, name, args):
        self.id = id
        self.name = name
        self.args = args

class ToolResult:
    def __init__(self, tool_use_id, content, is_error=False, display=None):
        self.tool_use_id = tool_use_id
        self.content = content
        self.is_error = is_error
        self.display = display

class Result:
    def __init__(self, text, session_id, turns=0, tool_calls=0, total_tokens=0,
                 total_cost=0.0, stop_reason="end_turn"):
        self.text = text
        self.session_id = session_id
        self.turns = turns
        self.tool_calls = tool_calls
        self.total_tokens = total_tokens
        self.total_cost = total_cost
        self.stop_reason = stop_reason

class FakeProviderError(Exception):
    def __init__(self, message, status_code=None):
        super().__init__(message)
        self.status_code = status_code

class UnsupportedReasoningError(Exception):
    pass

async def run(
        prompt,
        *,
        provider="anthropic",
        model=None,
        tools=None,
        mcp_servers=None,
        permission_mode="default",
        permission_rules=None,
        session_id=None,
        max_turns=100,
        max_tokens=16384,
        cwd=None,
        api_key=None,
        base_url=None,
        system_prompt=None,
        hooks=None,
        interactive=False,
        approval_callback=None,
        steering=None,
        sandbox_mode=None,
        _provider=None,
        **kwargs):
    if model == "gpt-5.6-luna" and _provider is None:
        raise KeyError("Unknown model 'gpt-5.6-luna'; static registry rejected it")
    from harness.core.engine import load_toml_config
    from harness.providers.registry import MODELS
    tool_key = subprocess.check_output(
        [sys.executable, "-c", "import os; print('OPENAI_API_KEY' in os.environ)"],
        text=True,
    ).strip()
    record = {
        "kind": "run",
        "prompt": prompt,
        "provider": provider,
        "model": model,
        "base_url": base_url,
        "permission_mode": permission_mode,
        "max_tokens": max_tokens,
        "cwd": cwd,
        "session_id": session_id,
        "api_key_present": api_key is not None,
        "injected_provider": _provider is not None,
        "injected_provider_model": getattr(_provider, "model_id", None),
        "injected_provider_base_url": getattr(_provider, "base_url", None),
        "injected_provider_key_present": bool(getattr(_provider, "api_key", None)),
        "openai_key_in_env": "OPENAI_API_KEY" in os.environ,
        "tool_subprocess_key_in_env": tool_key == "True",
        "immutable_env_present": "HARNESS_PROVIDER" in os.environ or "HARNESS_MODEL" in os.environ,
        "loaded_toml_config": load_toml_config(cwd),
        "home": os.environ.get("HOME"),
        "registered_model": {
            "provider": getattr(MODELS.get(model), "provider", None),
            "context_window": getattr(MODELS.get(model), "context_window", None),
            "max_output_tokens": getattr(MODELS.get(model), "max_output_tokens", None),
        },
        "steering_present": steering is not None,
        "argv": sys.argv,
        "version": __version__,
    }
    with open(os.environ["FAKE_HARNESS_RECORD"], "a", encoding="utf-8") as stream:
        stream.write(json.dumps(record) + "\n")

    secret = getattr(_provider, "api_key", "")
    provider_errors = {
        "provider auth error": FakeProviderError(f"unauthorized {secret}", 401),
        "provider rate error": FakeProviderError("rate limited", 429),
        "provider context error": FakeProviderError("maximum context length exceeded", 400),
        "provider unknown model": KeyError("unknown model gpt-5.6-luna"),
        "provider reasoning error": UnsupportedReasoningError("reasoning effort unsupported"),
        "provider timeout": asyncio.TimeoutError("provider timed out"),
        "provider rejection": FakeProviderError("request rejected", 400),
        "provider uncertain": RuntimeError("opaque upstream failure"),
    }
    if prompt in provider_errors:
        raise provider_errors[prompt]
    if prompt == "emit secret final":
        yield Result(f"final {secret}", session_id, 1, 0, 1, 0.0, "end_turn")
        return
    if prompt == "emit oversized event":
        yield TextMessage("x" * (1024 * 1024 + 1), True)
        return
    if prompt == "emit unsafe result session":
        yield Result("bad session", "../escape", 1, 0, 1, 0.0, "end_turn")
        return
    if prompt == "wait for cancellation":
        yield TextMessage("waiting", True)
        await asyncio.Event().wait()
        return
    if prompt == "emit malformed event":
        yield object()
        return
    if prompt == "emit nan result":
        yield Result("bad metrics", session_id, 1, 0, 1, float("nan"), "end_turn")
        return
    if prompt == "emit positive infinity result":
        yield Result("bad metrics", session_id, 1, 0, 1, float("inf"), "end_turn")
        return
    if prompt == "emit negative infinity result":
        yield Result("bad metrics", session_id, 1, 0, 1, float("-inf"), "end_turn")
        return
    if prompt == "emit lifecycle events":
        yield SystemEvent(
            "session_start",
            {
                "session_id": session_id,
                "progress": {"phase": "starting", "api_key": secret},
            },
        )
        yield SystemEvent("progress", {"message": f"working with {secret}"})
        yield CompactionEvent(40, 12, f"summary without {secret}")
        yield TextMessage("after lifecycle", False)
        yield Result("lifecycle finished", session_id, 1, 0, 12, 0.0, "end_turn")
        return

    yield TextMessage("hel", True)
    steering = steering
    for _ in range(100):
        if steering.messages:
            break
        await asyncio.sleep(0.005)
    yield ToolUse(
        "tool-1",
        "Read",
        {
            "path": "README.md",
            "nested": {
                "api_key": secret,
                "OPENAI_API_KEY": "different-secret",
                "values": ["safe", secret],
            },
        },
    )
    yield ToolResult("tool-1", f"tool output {secret}", False, secret)
    yield TextMessage("hello", False)
    yield Result("finished", session_id, 2, 1, 17, 0.25, "end_turn")
"#,
        )
        .unwrap();
        Self {
            record_path: root.join("sdk-record.jsonl"),
            root,
        }
    }
}

impl Drop for FakeSdk {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

struct BridgeProcess {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    raw_frames: Vec<String>,
    home: PathBuf,
    startup_events: VecDeque<OpenHarnessEvent>,
}

impl BridgeProcess {
    async fn spawn(fake: &FakeSdk, secret: &str) -> Self {
        let bridge = Path::new(env!("CARGO_MANIFEST_DIR")).join("sidecars/openharness_bridge.py");
        let home = fake.root.join(format!("bridge-home-{}", Uuid::new_v4()));
        fs::create_dir_all(&home).unwrap();
        let mut command = Command::new("python3");
        command
            .arg(bridge)
            .env("PYTHONPATH", &fake.root)
            .env("FAKE_HARNESS_RECORD", &fake.record_path)
            .env("OPENAI_API_KEY", secret)
            .env("HARNESS_PROVIDER", "workspace-override")
            .env("HARNESS_MODEL", "workspace-override")
            .env("HOME", &home)
            .env("OPENHARNESS_SESSION_HOME", &home)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);
        let mut child = command.spawn().unwrap();
        let stdin = child.stdin.take().unwrap();
        let mut stdout = BufReader::new(child.stdout.take().unwrap());
        let mut startup_line = String::new();
        let read =
            tokio::time::timeout(Duration::from_secs(3), stdout.read_line(&mut startup_line))
                .await
                .unwrap()
                .unwrap();
        assert_ne!(read, 0, "bridge exited without a startup frame");
        let startup = OpenHarnessCodec
            .decode_event_line(startup_line.trim())
            .unwrap();
        let mut startup_events = VecDeque::new();
        if !matches!(startup, OpenHarnessEvent::Ready { .. }) {
            startup_events.push_back(startup);
        }
        Self {
            child,
            stdin,
            stdout,
            raw_frames: vec![startup_line],
            home,
            startup_events,
        }
    }

    async fn send(&mut self, command: OpenHarnessCommand) {
        let mut wire = serde_json::to_vec(&command).unwrap();
        wire.push(b'\n');
        self.stdin.write_all(&wire).await.unwrap();
        self.stdin.flush().await.unwrap();
    }

    async fn event(&mut self) -> OpenHarnessEvent {
        if let Some(event) = self.startup_events.pop_front() {
            return event;
        }
        let line = self.raw_frame().await;
        OpenHarnessCodec.decode_event_line(line.trim()).unwrap()
    }

    async fn raw_frame(&mut self) -> String {
        let mut line = String::new();
        let read = tokio::time::timeout(Duration::from_secs(3), self.stdout.read_line(&mut line))
            .await
            .unwrap()
            .unwrap();
        assert_ne!(read, 0, "bridge exited before emitting a frame");
        self.raw_frames.push(line.clone());
        line
    }
}

#[tokio::test]
async fn native_session_ids_are_safe_persisted_resumable_and_truthfully_deleted() {
    let fake = FakeSdk::create();
    let mut bridge = BridgeProcess::spawn(&fake, "session-secret-canary").await;
    bridge
        .send(OpenHarnessCommand::create_session("create-safe-session"))
        .await;
    let session_id = match bridge.event().await {
        OpenHarnessEvent::Session { session_id, .. } => session_id,
        event => panic!("unexpected create response: {event:?}"),
    };
    let state = bridge
        .home
        .join(".harness/sessions")
        .join(format!("{session_id}.jsonl"));
    assert!(
        state.is_file(),
        "create_session must establish resumable state"
    );

    bridge
        .send(OpenHarnessCommand::resume_session(
            "resume-safe",
            &session_id,
        ))
        .await;
    assert!(matches!(
        bridge.event().await,
        OpenHarnessEvent::Session { id, session_id: resumed, .. }
            if id == "resume-safe" && resumed == session_id
    ));

    for (index, unsafe_id) in [
        "../escape",
        "/tmp/absolute",
        "nested/id",
        "nested\\id",
        ".",
        "..",
    ]
    .into_iter()
    .enumerate()
    {
        bridge
            .send(OpenHarnessCommand::resume_session(
                format!("unsafe-{index}"),
                unsafe_id,
            ))
            .await;
        assert!(matches!(
            bridge.event().await,
            OpenHarnessEvent::Error { code, terminal: false, .. } if code == "invalid_command"
        ));
    }
    bridge
        .send(OpenHarnessCommand::resume_session(
            "resume-missing",
            "safe-but-missing",
        ))
        .await;
    assert!(matches!(
        bridge.event().await,
        OpenHarnessEvent::Error { code, terminal: false, .. } if code == "session_not_found"
    ));

    bridge
        .send(OpenHarnessCommand::delete_session(
            "delete-safe",
            &session_id,
        ))
        .await;
    assert!(matches!(
        bridge.event().await,
        OpenHarnessEvent::SessionDeleted { id, session_id: deleted, .. }
            if id == "delete-safe" && deleted == session_id
    ));
    assert!(!state.exists(), "delete_session must remove native state");
    bridge
        .send(OpenHarnessCommand::resume_session(
            "resume-after-delete",
            &session_id,
        ))
        .await;
    assert!(matches!(
        bridge.event().await,
        OpenHarnessEvent::Error { code, terminal: false, .. } if code == "session_not_found"
    ));

    bridge
        .send(OpenHarnessCommand::shutdown("shutdown-sessions"))
        .await;
    assert!(matches!(bridge.event().await, OpenHarnessEvent::Ack { .. }));
    drop(bridge.stdin);
    assert!(bridge.child.wait().await.unwrap().success());
}

#[tokio::test]
async fn sdk_boundary_isolates_config_scrubs_tool_environment_and_redacts_final_text() {
    let fake = FakeSdk::create();
    let workspace = fake.root.join("requested-workspace");
    fs::create_dir_all(workspace.join(".harness")).unwrap();
    fs::write(
        workspace.join(".harness/config.toml"),
        "provider = 'workspace-override'\nmodel = 'workspace-override'\n[router]\nstrategy = 'cost'\n",
    )
    .unwrap();
    let secret = "environment-scrub-secret-canary";
    let mut bridge = BridgeProcess::spawn(&fake, secret).await;
    bridge
        .send(OpenHarnessCommand::create_session("create-isolation"))
        .await;
    let session_id = match bridge.event().await {
        OpenHarnessEvent::Session { session_id, .. } => session_id,
        event => panic!("unexpected create event: {event:?}"),
    };
    bridge
        .send(OpenHarnessCommand::prompt_with_resolved_model(
            "prompt-isolation",
            &session_id,
            "emit secret final",
            resolved_model(),
            "plan",
            workspace.to_string_lossy(),
        ))
        .await;
    let result_text = loop {
        match bridge.event().await {
            OpenHarnessEvent::Result { text, .. } => break text,
            OpenHarnessEvent::Error { message, .. } => panic!("prompt failed: {message}"),
            _ => {}
        }
    };
    assert_eq!(result_text, "final [REDACTED]");
    bridge
        .send(OpenHarnessCommand::shutdown("shutdown-isolation"))
        .await;
    assert!(matches!(bridge.event().await, OpenHarnessEvent::Ack { .. }));
    drop(bridge.stdin);
    assert!(bridge.child.wait().await.unwrap().success());

    let run = fs::read_to_string(&fake.record_path)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap())
        .find(|value| value["kind"] == "run")
        .unwrap();
    assert_eq!(run["provider"], "openai");
    assert_eq!(run["model"], "gpt-5.6-luna");
    assert_eq!(run["injected_provider_key_present"], true);
    assert_eq!(run["openai_key_in_env"], false);
    assert_eq!(run["tool_subprocess_key_in_env"], false);
    assert_eq!(run["immutable_env_present"], false);
    assert_eq!(run["loaded_toml_config"], json!({}));
    assert_eq!(run["cwd"], workspace.to_string_lossy().as_ref());
    assert_eq!(run["home"], bridge.home.to_string_lossy().as_ref());
    assert!(
        !fs::read_to_string(&fake.record_path)
            .unwrap()
            .contains(secret)
    );
}

#[tokio::test]
async fn provider_failures_use_stable_secret_free_error_classes() {
    let fake = FakeSdk::create();
    let secret = "provider-error-secret-canary";
    let mut bridge = BridgeProcess::spawn(&fake, secret).await;
    bridge
        .send(OpenHarnessCommand::create_session("create-provider-errors"))
        .await;
    let session_id = match bridge.event().await {
        OpenHarnessEvent::Session { session_id, .. } => session_id,
        event => panic!("unexpected create event: {event:?}"),
    };
    let cases = [
        ("provider auth error", "auth"),
        ("provider rate error", "rate_limit"),
        ("provider context error", "context"),
        ("provider unknown model", "unknown_model"),
        ("provider reasoning error", "unsupported_reasoning"),
        ("provider timeout", "timeout"),
        ("provider rejection", "provider_rejection"),
        ("provider uncertain", "uncertain"),
    ];
    for (index, (prompt, expected_code)) in cases.into_iter().enumerate() {
        let request_id = format!("provider-error-{index}");
        bridge
            .send(OpenHarnessCommand::prompt_with_resolved_model(
                &request_id,
                &session_id,
                prompt,
                resolved_model(),
                "plan",
                "/workspace/project",
            ))
            .await;
        loop {
            match bridge.event().await {
                OpenHarnessEvent::Error {
                    request_id: error_request,
                    code,
                    message,
                    terminal,
                    ..
                } if error_request == request_id => {
                    assert_eq!(code, expected_code, "{prompt}");
                    assert!(!message.contains(secret));
                    assert!(terminal);
                    break;
                }
                _ => {}
            }
        }
    }
    bridge
        .send(OpenHarnessCommand::shutdown("shutdown-provider-errors"))
        .await;
    assert!(matches!(bridge.event().await, OpenHarnessEvent::Ack { .. }));
    drop(bridge.stdin);
    assert!(bridge.child.wait().await.unwrap().success());
}

#[test]
fn rust_codec_rejects_oversized_jsonl_frames() {
    let codec = OpenHarnessCodec;
    let oversized_prompt = "x".repeat(OPENHARNESS_MAX_FRAME_BYTES);
    let command = OpenHarnessCommand::prompt_with_resolved_model(
        "oversized-command",
        "safe-session",
        oversized_prompt,
        resolved_model(),
        "plan",
        "/workspace/project",
    );
    assert!(codec.encode_command_line(&command).is_err());

    let oversized_event = format!(
        "{{\"type\":\"text\",\"request_id\":\"p\",\"session_id\":\"s\",\"text\":\"{}\",\"is_partial\":true}}",
        "x".repeat(OPENHARNESS_MAX_FRAME_BYTES)
    );
    assert!(codec.decode_event_line(&oversized_event).is_err());
}

#[tokio::test]
async fn python_sidecar_rejects_oversized_input_and_output_frames() {
    let fake = FakeSdk::create();
    let mut bridge = BridgeProcess::spawn(&fake, "frame-secret-canary").await;
    let oversized_command = format!(
        "{{\"type\":\"create_session\",\"id\":\"oversized-input\",\"padding\":\"{}\"}}\n",
        "x".repeat(OPENHARNESS_MAX_FRAME_BYTES)
    );
    bridge
        .stdin
        .write_all(oversized_command.as_bytes())
        .await
        .unwrap();
    bridge.stdin.flush().await.unwrap();
    assert!(matches!(
        bridge.event().await,
        OpenHarnessEvent::Error { code, terminal: false, .. } if code == "frame_too_large"
    ));

    bridge
        .send(OpenHarnessCommand::create_session("create-frame-output"))
        .await;
    let session_id = match bridge.event().await {
        OpenHarnessEvent::Session { session_id, .. } => session_id,
        event => panic!("unexpected create event: {event:?}"),
    };
    bridge
        .send(OpenHarnessCommand::prompt_with_resolved_model(
            "prompt-frame-output",
            &session_id,
            "emit oversized event",
            resolved_model(),
            "plan",
            "/workspace/project",
        ))
        .await;
    loop {
        match bridge.event().await {
            OpenHarnessEvent::Error { code, terminal, .. } => {
                assert_eq!(code, "frame_too_large");
                assert!(terminal);
                break;
            }
            OpenHarnessEvent::Ack { .. } | OpenHarnessEvent::Downgrade { .. } => {}
            event => panic!("unexpected oversized-output event: {event:?}"),
        }
    }
    assert!(
        bridge
            .raw_frames
            .iter()
            .all(|frame| frame.len() <= OPENHARNESS_MAX_FRAME_BYTES)
    );
    bridge
        .send(OpenHarnessCommand::shutdown("shutdown-frame-bounds"))
        .await;
    assert!(matches!(bridge.event().await, OpenHarnessEvent::Ack { .. }));
    drop(bridge.stdin);
    assert!(bridge.child.wait().await.unwrap().success());
}

#[tokio::test]
async fn sidecar_rejects_base_url_and_permission_before_prompt_acknowledgment() {
    let fake = FakeSdk::create();
    let mut bridge = BridgeProcess::spawn(&fake, "validation-secret-canary").await;
    bridge
        .send(OpenHarnessCommand::create_session("create-validation"))
        .await;
    let session_id = match bridge.event().await {
        OpenHarnessEvent::Session { session_id, .. } => session_id,
        event => panic!("unexpected create event: {event:?}"),
    };

    let mut invalid_url = resolved_model();
    invalid_url.base_url = "file:///tmp/provider".to_owned();
    bridge
        .send(OpenHarnessCommand::prompt_with_resolved_model(
            "invalid-url",
            &session_id,
            "must not start",
            invalid_url,
            "plan",
            "/workspace/project",
        ))
        .await;
    assert!(matches!(
        bridge.event().await,
        OpenHarnessEvent::Error { request_id, code, terminal: false, .. }
            if request_id == "invalid-url" && code == "invalid_command"
    ));

    bridge
        .send(OpenHarnessCommand::prompt_with_resolved_model(
            "invalid-permission",
            &session_id,
            "must not start",
            resolved_model(),
            "root",
            "/workspace/project",
        ))
        .await;
    assert!(matches!(
        bridge.event().await,
        OpenHarnessEvent::Error { request_id, code, terminal: false, .. }
            if request_id == "invalid-permission" && code == "invalid_command"
    ));

    bridge
        .send(OpenHarnessCommand::shutdown("shutdown-validation"))
        .await;
    assert!(matches!(bridge.event().await, OpenHarnessEvent::Ack { .. }));
    drop(bridge.stdin);
    assert!(bridge.child.wait().await.unwrap().success());
    assert!(
        !fake.record_path.exists(),
        "invalid prompts must not reach harness.run"
    );
}

#[tokio::test]
async fn sdk_result_cannot_replace_the_active_native_session_id() {
    let fake = FakeSdk::create();
    let mut bridge = BridgeProcess::spawn(&fake, "result-session-secret").await;
    bridge
        .send(OpenHarnessCommand::create_session("create-result-session"))
        .await;
    let session_id = match bridge.event().await {
        OpenHarnessEvent::Session { session_id, .. } => session_id,
        event => panic!("unexpected create event: {event:?}"),
    };
    bridge
        .send(OpenHarnessCommand::prompt_with_resolved_model(
            "prompt-result-session",
            &session_id,
            "emit unsafe result session",
            resolved_model(),
            "plan",
            "/workspace/project",
        ))
        .await;
    loop {
        match bridge.event().await {
            OpenHarnessEvent::Error { code, terminal, .. } => {
                assert_eq!(code, "malformed_sdk_event");
                assert!(terminal);
                break;
            }
            OpenHarnessEvent::Ack { .. } | OpenHarnessEvent::Downgrade { .. } => {}
            event => panic!("unsafe session escaped as event: {event:?}"),
        }
    }
    bridge
        .send(OpenHarnessCommand::shutdown("shutdown-result-session"))
        .await;
    assert!(matches!(bridge.event().await, OpenHarnessEvent::Ack { .. }));
    drop(bridge.stdin);
    assert!(bridge.child.wait().await.unwrap().success());
}

async fn assert_nonfinite_result_is_terminal_error(prompt: &str, suffix: &str) {
    let fake = FakeSdk::create();
    let secret = format!("nonfinite-secret-{suffix}");
    let mut bridge = BridgeProcess::spawn(&fake, &secret).await;
    bridge
        .send(OpenHarnessCommand::create_session(format!(
            "create-nonfinite-{suffix}"
        )))
        .await;
    let session_id = match bridge.event().await {
        OpenHarnessEvent::Session { session_id, .. } => session_id,
        event => panic!("unexpected create event: {event:?}"),
    };
    let prompt_id = format!("prompt-nonfinite-{suffix}");
    bridge
        .send(OpenHarnessCommand::prompt(
            &prompt_id,
            &session_id,
            prompt,
            "gpt-5.6-luna",
            "https://llmapi.omnisolo.co/v1",
            "bypass",
            "/workspace/project",
        ))
        .await;
    loop {
        match bridge.event().await {
            OpenHarnessEvent::Ack { id, .. } if id == prompt_id => break,
            OpenHarnessEvent::Downgrade { .. } => {}
            event => panic!("unexpected pre-prompt event: {event:?}"),
        }
    }
    let raw = bridge.raw_frame().await;
    assert!(!raw.contains("NaN"));
    assert!(!raw.contains("Infinity"));
    serde_json::from_str::<serde_json::Value>(&raw).expect("sidecar must emit strict JSON");
    match OpenHarnessCodec.decode_event_line(raw.trim()).unwrap() {
        OpenHarnessEvent::Error {
            request_id,
            code,
            message,
            terminal,
            ..
        } => {
            assert_eq!(request_id, prompt_id);
            assert_eq!(code, "malformed_sdk_event");
            assert!(message.contains("finite non-negative number"));
            assert!(!message.contains(&secret));
            assert!(terminal);
        }
        event => panic!("unexpected non-finite result event: {event:?}"),
    }
    bridge
        .send(OpenHarnessCommand::shutdown(format!(
            "shutdown-nonfinite-{suffix}"
        )))
        .await;
    assert!(matches!(bridge.event().await, OpenHarnessEvent::Ack { .. }));
    drop(bridge.stdin);
    assert!(bridge.child.wait().await.unwrap().success());
}

#[tokio::test]
async fn nan_result_metric_becomes_strict_json_terminal_error() {
    assert_nonfinite_result_is_terminal_error("emit nan result", "nan").await;
}

#[tokio::test]
async fn positive_infinity_result_metric_becomes_strict_json_terminal_error() {
    assert_nonfinite_result_is_terminal_error("emit positive infinity result", "positive").await;
}

#[tokio::test]
async fn negative_infinity_result_metric_becomes_strict_json_terminal_error() {
    assert_nonfinite_result_is_terminal_error("emit negative infinity result", "negative").await;
}

#[tokio::test]
async fn arbitrary_openai_model_uses_direct_provider_instead_of_static_registry() {
    let fake = FakeSdk::create();
    let secret = "custom-model-secret-canary";
    let mut bridge = BridgeProcess::spawn(&fake, secret).await;

    bridge
        .send(OpenHarnessCommand::create_session("create-custom-model"))
        .await;
    let session_id = match bridge.event().await {
        OpenHarnessEvent::Session { session_id, .. } => session_id,
        event => panic!("unexpected create event: {event:?}"),
    };
    bridge
        .send(OpenHarnessCommand::prompt(
            "prompt-custom-model",
            &session_id,
            "custom model prompt",
            "gpt-5.6-luna",
            "https://llmapi.omnisolo.co/v1",
            "bypass",
            "/workspace/project",
        ))
        .await;

    loop {
        match bridge.event().await {
            OpenHarnessEvent::Result { text, .. } => {
                assert_eq!(text, "finished");
                break;
            }
            OpenHarnessEvent::Error { message, .. } => {
                panic!("custom model reached static registry: {message}")
            }
            _ => {}
        }
    }

    bridge
        .send(OpenHarnessCommand::shutdown("shutdown-custom-model"))
        .await;
    assert!(matches!(
        bridge.event().await,
        OpenHarnessEvent::Ack { id, .. } if id == "shutdown-custom-model"
    ));
    drop(bridge.stdin);
    assert!(bridge.child.wait().await.unwrap().success());

    let records = fs::read_to_string(&fake.record_path).unwrap();
    assert!(!records.contains(secret));
    let run = records
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap())
        .find(|record| record["kind"] == "run")
        .unwrap();
    assert_eq!(run["provider"], "openai");
    assert_eq!(run["model"], "gpt-5.6-luna");
    assert_eq!(run["base_url"], "https://llmapi.omnisolo.co/v1");
    assert_eq!(run["permission_mode"], "bypass");
    assert_eq!(run["registered_model"]["provider"], "openai");
    assert_eq!(run["registered_model"]["context_window"], 200_000);
    assert_eq!(run["registered_model"]["max_output_tokens"], 16_384);
    assert_eq!(run["api_key_present"], false);
    assert_eq!(run["injected_provider"], true);
    assert_eq!(run["injected_provider_model"], "gpt-5.6-luna");
    assert_eq!(
        run["injected_provider_base_url"],
        "https://llmapi.omnisolo.co/v1"
    );
    assert_eq!(run["injected_provider_key_present"], true);
    assert_eq!(run["openai_key_in_env"], false);
}

#[tokio::test]
async fn max_reasoning_is_explicitly_downgraded_before_prompt_ack_and_token_limit_is_applied() {
    let fake = FakeSdk::create();
    let mut bridge = BridgeProcess::spawn(&fake, "reasoning-secret-canary").await;
    bridge
        .send(OpenHarnessCommand::create_session("create-reasoning"))
        .await;
    let session_id = match bridge.event().await {
        OpenHarnessEvent::Session { session_id, .. } => session_id,
        event => panic!("unexpected create event: {event:?}"),
    };
    bridge
        .send(OpenHarnessCommand::prompt_with_resolved_model(
            "prompt-reasoning",
            &session_id,
            "reason deeply",
            resolved_model(),
            "bypass",
            "/workspace/project",
        ))
        .await;

    match bridge.event().await {
        OpenHarnessEvent::Downgrade {
            request_id,
            field,
            requested,
            applied,
            ..
        } => {
            assert_eq!(request_id, "prompt-reasoning");
            assert_eq!(field, "reasoning_effort");
            assert_eq!(requested, json!("max"));
            assert_eq!(applied, json!("provider_default"));
        }
        event => panic!("expected downgrade before acknowledgment, got {event:?}"),
    }
    assert!(matches!(
        bridge.event().await,
        OpenHarnessEvent::Downgrade { field, .. } if field == "api_dialect"
    ));
    assert!(matches!(
        bridge.event().await,
        OpenHarnessEvent::Ack { id, .. } if id == "prompt-reasoning"
    ));
    loop {
        if matches!(bridge.event().await, OpenHarnessEvent::Result { .. }) {
            break;
        }
    }
    bridge
        .send(OpenHarnessCommand::shutdown("shutdown-reasoning"))
        .await;
    assert!(matches!(bridge.event().await, OpenHarnessEvent::Ack { .. }));
    drop(bridge.stdin);
    assert!(bridge.child.wait().await.unwrap().success());

    let record = fs::read_to_string(&fake.record_path).unwrap();
    let run = record
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap())
        .find(|value| value["kind"] == "run")
        .unwrap();
    assert_eq!(run["max_tokens"], 65_536);
}

#[tokio::test]
async fn bridge_pins_private_provider_injection_signature_for_sdk_0_6_0() {
    let fake = FakeSdk::create();
    let module_path = fake.root.join("harness/__init__.py");
    let module = fs::read_to_string(&module_path).unwrap().replace(
        "        _provider=None,\n",
        "        removed_provider_hook=None,\n",
    );
    assert!(!module.contains("        _provider=None,\n"));
    fs::write(module_path, module).unwrap();
    let mut bridge = BridgeProcess::spawn(&fake, "signature-secret-canary").await;

    match bridge.event().await {
        OpenHarnessEvent::Error {
            request_id,
            code,
            message,
            terminal,
            ..
        } => {
            assert_eq!(request_id, "__startup__");
            assert_eq!(code, "sdk_contract_mismatch");
            assert!(message.contains("_provider"));
            assert!(terminal);
        }
        event => panic!("unexpected startup event: {event:?}"),
    }
    drop(bridge.stdin);
    assert!(!bridge.child.wait().await.unwrap().success());
}

#[tokio::test]
async fn system_and_progress_events_do_not_kill_the_prompt_stream() {
    let fake = FakeSdk::create();
    let secret = "lifecycle-secret-canary";
    let mut bridge = BridgeProcess::spawn(&fake, secret).await;

    bridge
        .send(OpenHarnessCommand::create_session("create-lifecycle"))
        .await;
    let session_id = match bridge.event().await {
        OpenHarnessEvent::Session { session_id, .. } => session_id,
        event => panic!("unexpected create event: {event:?}"),
    };
    bridge
        .send(OpenHarnessCommand::prompt(
            "prompt-lifecycle",
            &session_id,
            "emit lifecycle events",
            "gpt-5.6-luna",
            "https://llmapi.omnisolo.co/v1",
            "bypass",
            "/workspace/project",
        ))
        .await;

    let mut saw_session_start = false;
    let mut saw_progress = false;
    let mut saw_compaction = false;
    let mut saw_text = false;
    loop {
        match bridge.event().await {
            OpenHarnessEvent::System {
                event,
                data,
                request_id,
                session_id: event_session_id,
            } => {
                assert_eq!(request_id, "prompt-lifecycle");
                assert_eq!(event_session_id, session_id);
                assert!(!data.to_string().contains(secret));
                match event.as_str() {
                    "session_start" => {
                        assert_eq!(data["progress"]["api_key"], "[REDACTED]");
                        saw_session_start = true;
                    }
                    "progress" => saw_progress = true,
                    other => panic!("unexpected system event: {other}"),
                }
            }
            OpenHarnessEvent::Compaction {
                tokens_before,
                tokens_after,
                summary,
                ..
            } => {
                assert_eq!(tokens_before, 40);
                assert_eq!(tokens_after, 12);
                assert!(!summary.contains(secret));
                saw_compaction = true;
            }
            OpenHarnessEvent::Text { text, .. } => {
                assert_eq!(text, "after lifecycle");
                saw_text = true;
            }
            OpenHarnessEvent::Result { text, .. } => {
                assert_eq!(text, "lifecycle finished");
                break;
            }
            OpenHarnessEvent::Error { message, .. } => {
                panic!("lifecycle event killed stream: {message}")
            }
            _ => {}
        }
    }
    assert!(saw_session_start && saw_progress && saw_compaction && saw_text);

    bridge
        .send(OpenHarnessCommand::shutdown("shutdown-lifecycle"))
        .await;
    assert!(matches!(
        bridge.event().await,
        OpenHarnessEvent::Ack { id, .. } if id == "shutdown-lifecycle"
    ));
    drop(bridge.stdin);
    assert!(bridge.child.wait().await.unwrap().success());
}

#[tokio::test]
async fn fake_sdk_sidecar_streams_controls_redacts_and_exits_cleanly() {
    let fake = FakeSdk::create();
    let secret = "openai-key-recursive-canary";
    let mut bridge = BridgeProcess::spawn(&fake, secret).await;

    bridge
        .send(OpenHarnessCommand::create_session("create-1"))
        .await;
    let native_session_id = match bridge.event().await {
        OpenHarnessEvent::Session {
            id,
            command,
            session_id,
        } => {
            assert_eq!(id, "create-1");
            assert_eq!(command, "create_session");
            session_id
        }
        event => panic!("unexpected create event: {event:?}"),
    };
    assert!(!native_session_id.is_empty());

    bridge
        .send(OpenHarnessCommand::resume_session(
            "resume-1",
            &native_session_id,
        ))
        .await;
    assert!(matches!(
        bridge.event().await,
        OpenHarnessEvent::Session { session_id, .. } if session_id == native_session_id
    ));

    bridge
        .send(OpenHarnessCommand::prompt(
            "prompt-1",
            &native_session_id,
            "normal prompt",
            "gpt-5.6-luna",
            "https://llmapi.omnisolo.co/v1",
            "bypass",
            "/workspace/project",
        ))
        .await;

    let mut prompt_ack = false;
    let mut steer_sent = false;
    let mut steer_ack = false;
    let mut saw_redacted_tool = false;
    loop {
        let event = bridge.event().await;
        match event {
            OpenHarnessEvent::Ack { id, command } if id == "prompt-1" => {
                assert_eq!(command, "prompt");
                prompt_ack = true;
            }
            OpenHarnessEvent::Text { text, .. } if text == "hel" && !steer_sent => {
                bridge
                    .send(OpenHarnessCommand::steer(
                        "steer-1",
                        &native_session_id,
                        "Focus on tests",
                    ))
                    .await;
                steer_sent = true;
            }
            OpenHarnessEvent::Ack { id, command } if id == "steer-1" => {
                assert_eq!(command, "steer");
                steer_ack = true;
            }
            OpenHarnessEvent::ToolUse { args, .. } => {
                assert_eq!(args["nested"]["api_key"], "[REDACTED]");
                assert_eq!(args["nested"]["OPENAI_API_KEY"], "[REDACTED]");
                assert_eq!(args["nested"]["values"][1], "[REDACTED]");
                saw_redacted_tool = true;
            }
            OpenHarnessEvent::ToolResult {
                content, display, ..
            } => {
                assert!(!content.contains(secret));
                assert_eq!(display.as_deref(), Some("[REDACTED]"));
            }
            OpenHarnessEvent::Result {
                session_id,
                text,
                total_tokens,
                ..
            } => {
                assert_eq!(session_id, native_session_id);
                assert_eq!(text, "finished");
                assert_eq!(total_tokens, 17);
                break;
            }
            _ => {}
        }
    }
    assert!(prompt_ack && steer_sent && steer_ack && saw_redacted_tool);

    bridge
        .send(OpenHarnessCommand::prompt(
            "prompt-2",
            &native_session_id,
            "wait for cancellation",
            "gpt-5.6-luna",
            "https://llmapi.omnisolo.co/v1",
            "accept_edits",
            "/workspace/project",
        ))
        .await;
    loop {
        if matches!(bridge.event().await, OpenHarnessEvent::Text { text, .. } if text == "waiting")
        {
            break;
        }
    }
    bridge
        .send(OpenHarnessCommand::cancel("cancel-1", &native_session_id))
        .await;
    let mut cancelled = false;
    let mut cancel_ack = false;
    while !(cancelled && cancel_ack) {
        match bridge.event().await {
            OpenHarnessEvent::Ack { id, command } if id == "cancel-1" => {
                assert_eq!(command, "cancel");
                cancel_ack = true;
            }
            OpenHarnessEvent::Cancelled { request_id, .. } if request_id == "prompt-2" => {
                cancelled = true;
            }
            _ => {}
        }
    }

    bridge
        .send(OpenHarnessCommand::prompt(
            "prompt-3",
            &native_session_id,
            "emit malformed event",
            "gpt-5.6-luna",
            "https://llmapi.omnisolo.co/v1",
            "plan",
            "/workspace/project",
        ))
        .await;
    loop {
        match bridge.event().await {
            OpenHarnessEvent::Error {
                request_id,
                code,
                message,
                terminal,
                ..
            } if request_id == "prompt-3" => {
                assert_eq!(code, "malformed_sdk_event");
                assert!(!message.contains(secret));
                assert!(terminal);
                break;
            }
            _ => {}
        }
    }

    bridge
        .send(OpenHarnessCommand::shutdown("shutdown-1"))
        .await;
    assert!(matches!(
        bridge.event().await,
        OpenHarnessEvent::Ack { id, command } if id == "shutdown-1" && command == "shutdown"
    ));
    drop(bridge.stdin);
    let output = tokio::time::timeout(Duration::from_secs(3), bridge.child.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    assert!(
        output.status.success(),
        "bridge stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        bridge
            .raw_frames
            .iter()
            .all(|frame| !frame.contains(secret))
    );

    let records = fs::read_to_string(&fake.record_path).unwrap();
    let records = records
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap())
        .collect::<Vec<_>>();
    let first_run = records
        .iter()
        .find(|record| record["kind"] == "run" && record["prompt"] == "normal prompt")
        .unwrap();
    assert_eq!(first_run["provider"], "openai");
    assert_eq!(first_run["model"], "gpt-5.6-luna");
    assert_eq!(first_run["base_url"], "https://llmapi.omnisolo.co/v1");
    assert_eq!(first_run["permission_mode"], "bypass");
    assert_eq!(first_run["cwd"], "/workspace/project");
    assert_eq!(first_run["session_id"], native_session_id);
    assert_eq!(first_run["api_key_present"], false);
    assert_eq!(first_run["injected_provider"], true);
    assert_eq!(first_run["injected_provider_key_present"], true);
    assert_eq!(first_run["openai_key_in_env"], false);
    assert_eq!(first_run["steering_present"], true);
    assert_eq!(first_run["version"], "0.6.0");
    assert!(
        first_run["argv"]
            .as_array()
            .unwrap()
            .iter()
            .all(|argument| !argument.as_str().unwrap().contains(secret))
    );
    assert!(
        records
            .iter()
            .any(|record| record["kind"] == "steer" && record["message"] == "Focus on tests")
    );
    assert!(
        !fs::read_to_string(&fake.record_path)
            .unwrap()
            .contains(secret)
    );
}

#[tokio::test]
async fn bridge_refuses_any_sdk_version_other_than_harness_agent_0_6_0() {
    let fake = FakeSdk::create();
    let module_path = fake.root.join("harness/__init__.py");
    let module = fs::read_to_string(&module_path)
        .unwrap()
        .replace("__version__ = \"0.6.0\"", "__version__ = \"0.6.1\"");
    fs::write(module_path, module).unwrap();
    let secret = "version-pin-secret-canary";
    let mut bridge = BridgeProcess::spawn(&fake, secret).await;

    match bridge.event().await {
        OpenHarnessEvent::Error {
            request_id,
            session_id,
            code,
            message,
            terminal,
        } => {
            assert_eq!(request_id, "__startup__");
            assert_eq!(session_id, None);
            assert_eq!(code, "sdk_version_mismatch");
            assert!(message.contains("0.6.0"));
            assert!(!message.contains(secret));
            assert!(terminal);
        }
        event => panic!("unexpected startup event: {event:?}"),
    }
    drop(bridge.stdin);
    let status = bridge.child.wait().await.unwrap();
    assert!(!status.success());
    assert!(
        bridge
            .raw_frames
            .iter()
            .all(|frame| !frame.contains(secret))
    );
}

#[tokio::test]
#[ignore = "requires OPENHARNESS_PINNED_SDK_SOURCE at the pinned AgentBoardTT revision"]
async fn real_pinned_sdk_prompts_with_private_tmp_home_and_read_only_workspace() {
    let source = std::env::var_os("OPENHARNESS_PINNED_SDK_SOURCE")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp/openharness-source-ozUT5J"));
    assert!(
        source.join("src/harness/__init__.py").is_file(),
        "set OPENHARNESS_PINNED_SDK_SOURCE to the AgentBoardTT OpenHarness 0.6.0 source tree"
    );
    let revision = std::process::Command::new("git")
        .args(["-C", source.to_str().unwrap(), "rev-parse", "HEAD"])
        .output()
        .unwrap();
    assert!(revision.status.success());
    assert_eq!(
        String::from_utf8(revision.stdout).unwrap().trim(),
        "85c54682a209ca7c3fc8b1ab2e820b6724dc3028"
    );

    let root = std::env::temp_dir().join(format!("real-openharness-{}", Uuid::new_v4()));
    fs::create_dir_all(&root).unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(source.join("src/harness"), root.join("harness")).unwrap();
    fs::write(
        root.join("dotenv.py"),
        "def load_dotenv(*args, **kwargs):\n    return False\n",
    )
    .unwrap();
    fs::create_dir_all(root.join("anyio")).unwrap();
    fs::write(
        root.join("anyio/abc.py"),
        "class ObjectReceiveStream: pass\nclass ObjectSendStream: pass\n",
    )
    .unwrap();
    fs::write(
        root.join("anyio/__init__.py"),
        r#"
from types import SimpleNamespace

class EndOfStream(Exception): pass
class WouldBlock(Exception): pass

class _Send:
    async def send(self, message): pass
    def send_nowait(self, message): pass
    async def aclose(self): pass

class _Receive:
    async def receive(self): raise WouldBlock()
    def statistics(self): return SimpleNamespace(current_buffer_used=0)
    async def aclose(self): pass

class _Factory:
    def __getitem__(self, item): return self
    def __call__(self, max_buffer_size=16): return _Send(), _Receive()

create_memory_object_stream = _Factory()

class _FailAfter:
    def __enter__(self): return self
    def __exit__(self, *args): return False

def fail_after(seconds): return _FailAfter()
"#,
    )
    .unwrap();
    fs::write(
        root.join("openai.py"),
        r#"
import json
import os
from types import SimpleNamespace

NOT_GIVEN = object()

class _Stream:
    def __aiter__(self):
        self.done = False
        return self
    async def __anext__(self):
        if self.done:
            raise StopAsyncIteration
        self.done = True
        delta = SimpleNamespace(content="real pinned sdk", tool_calls=None)
        choice = SimpleNamespace(delta=delta, finish_reason="stop")
        usage = SimpleNamespace(prompt_tokens=3, completion_tokens=4)
        return SimpleNamespace(choices=[choice], usage=usage)

class _Completions:
    async def create(self, **kwargs):
        with open(os.environ["FAKE_HARNESS_RECORD"], "w", encoding="utf-8") as stream:
            json.dump({
                "model": kwargs.get("model"),
                "max_completion_tokens": kwargs.get("max_completion_tokens"),
                "openai_key_in_env": "OPENAI_API_KEY" in os.environ,
                "home": os.environ.get("HOME"),
            }, stream)
        return _Stream()

class AsyncOpenAI:
    def __init__(self, api_key=None, base_url=None):
        assert api_key
        self.chat = SimpleNamespace(completions=_Completions())
"#,
    )
    .unwrap();
    let real = FakeSdk {
        record_path: root.join("real-record.json"),
        root,
    };
    let workspace = real.root.join("read-only-workspace");
    fs::create_dir_all(&workspace).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&workspace, fs::Permissions::from_mode(0o555)).unwrap();
    }
    let mut bridge = BridgeProcess::spawn(&real, "real-sdk-secret-canary").await;
    bridge
        .send(OpenHarnessCommand::create_session("create-real-sdk"))
        .await;
    let session_id = match bridge.event().await {
        OpenHarnessEvent::Session { session_id, .. } => session_id,
        event => panic!("unexpected create event: {event:?}"),
    };
    let mut contract = resolved_model();
    contract.max_output_tokens = 12_345;
    bridge
        .send(OpenHarnessCommand::prompt_with_resolved_model(
            "prompt-real-sdk",
            &session_id,
            "reply once",
            contract,
            "plan",
            workspace.to_string_lossy(),
        ))
        .await;
    loop {
        match bridge.event().await {
            OpenHarnessEvent::Result { text, .. } => {
                assert_eq!(text, "real pinned sdk");
                break;
            }
            OpenHarnessEvent::Error { code, message, .. } => {
                panic!("real pinned SDK failed ({code}): {message}")
            }
            _ => {}
        }
    }
    bridge
        .send(OpenHarnessCommand::shutdown("shutdown-real-sdk"))
        .await;
    assert!(matches!(bridge.event().await, OpenHarnessEvent::Ack { .. }));
    drop(bridge.stdin);
    assert!(bridge.child.wait().await.unwrap().success());
    let record: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&real.record_path).unwrap()).unwrap();
    assert_eq!(record["model"], "gpt-5.6-luna");
    assert_eq!(record["max_completion_tokens"], 12_345);
    assert_eq!(record["openai_key_in_env"], false);
    assert!(record["home"].as_str().unwrap().starts_with("/tmp/"));
}
