use std::collections::BTreeSet;
use std::fs;
use std::time::Duration;

use chrono::Utc;
use serde_json::{Value, json};
use server_harness::middleware::capsule::{
    CanonicalRecord, CapsuleCompileInput, CapsuleCompiler, PortableRecord,
};
use server_harness::middleware::harness::{
    ExternalHarnessPreset, HarnessAdapter, HarnessCapability, HarnessDescriptor,
    HarnessProtocolKind, HarnessRegistry, HarnessSessionRequest, OmniSoloHarnessAdapterBridge,
    ProcessHarnessAdapter, ProcessHarnessSpec, RuntimeFamily,
};
use server_harness::middleware::local_services::{
    LocalServiceKind, LocalServiceRegistry, LocalServiceScopeContext,
};
use server_harness::middleware::protocol::{AttemptOperation, HarnessExecutionItem};
use server_harness::middleware::router::{HarnessRouter, RouterError};
use server_harness::middleware::types::{
    BindingAccessMode, BindingScope, BindingState, ModelApiDialect, ReasoningEffort,
    ResolvedModelSelection, RuntimeConfigSnapshot, Session, SessionBinding, SessionState,
};
use tokio_stream::StreamExt;
use uuid::Uuid;

fn resolved_model_selection() -> ResolvedModelSelection {
    ResolvedModelSelection {
        provider_route: "openai-compatible".to_owned(),
        model_id: "gpt-5.6-luna".to_owned(),
        reasoning_effort: Some(ReasoningEffort::Max),
        api_dialect: ModelApiDialect::OpenAiResponses,
        context_window: Some(400_000),
        max_output_tokens: Some(128_000),
        capabilities: BTreeSet::from(["tools".to_owned(), "reasoning".to_owned()]),
        binding_revision: "binding-v1".to_owned(),
        binding_digest: "sha256:test".to_owned(),
        metadata: [("routing_tier".to_owned(), json!("balanced"))]
            .into_iter()
            .collect(),
    }
}

fn contains_credential_key(value: &Value) -> bool {
    match value {
        Value::Array(values) => values.iter().any(contains_credential_key),
        Value::Object(values) => values.iter().any(|(key, value)| {
            matches!(
                key.as_str(),
                "api_key"
                    | "access_token"
                    | "token"
                    | "credential"
                    | "credential_ref"
                    | "secret"
                    | "password"
                    | "authorization"
            ) || contains_credential_key(value)
        }),
        _ => false,
    }
}

#[test]
fn registry_exposes_native_and_external_harnesses_with_capability_snapshots() {
    let registry = HarnessRegistry::with_defaults();

    for harness_id in [
        "omnisolo",
        "codex",
        "opencode",
        "deepseek",
        "pi",
        "kimi",
        "openhands",
        "openharness",
    ] {
        let descriptor = registry
            .descriptor(harness_id)
            .expect("built-in descriptor");
        assert_eq!(descriptor.harness_id, harness_id);
        assert!(!descriptor.adapter_id.is_empty());
        assert!(!descriptor.protocol_version.is_empty());
        assert!(descriptor.capabilities.contains(&HarnessCapability::Prompt));
        assert!(
            descriptor
                .capabilities
                .contains(&HarnessCapability::Streaming)
        );
    }

    assert_eq!(
        registry
            .preset("codex")
            .expect("codex preset")
            .protocol_kind,
        ExternalHarnessPreset::Codex.protocol_kind()
    );
    assert_eq!(
        registry
            .preset("opencode")
            .expect("opencode preset")
            .protocol_kind,
        ExternalHarnessPreset::OpenCode.protocol_kind()
    );
    assert_eq!(
        registry.preset("kimi").expect("kimi preset").executable,
        "omnisolo-kimi-acp"
    );
    assert_eq!(
        registry
            .preset("openhands")
            .expect("openhands preset")
            .executable,
        "openhands-agent-server"
    );
    assert_eq!(
        registry
            .build_process_adapter("codex")
            .unwrap()
            .descriptor()
            .protocol_kind,
        ExternalHarnessPreset::Codex.protocol_kind()
    );
    assert!(registry.build_process_adapter("missing").is_err());
}

#[test]
fn registry_advertises_only_native_operations_and_pinned_worker_images() {
    let registry = HarnessRegistry::with_defaults();
    let cases = [
        (
            "omnisolo",
            "omnisolo/harness-worker:0.1.0",
            BTreeSet::from([
                HarnessCapability::Prompt,
                HarnessCapability::Streaming,
                HarnessCapability::Cancellation,
                HarnessCapability::Steering,
                HarnessCapability::Branching,
                HarnessCapability::ExactResume,
                HarnessCapability::ImportCapsule,
                HarnessCapability::ExportNativeState,
            ]),
        ),
        (
            "codex",
            "omnisolo/harness-worker-codex:0.149.0",
            BTreeSet::from([
                HarnessCapability::Prompt,
                HarnessCapability::Streaming,
                HarnessCapability::Cancellation,
                HarnessCapability::Steering,
                HarnessCapability::Approvals,
                HarnessCapability::Questions,
                HarnessCapability::Workspace,
                HarnessCapability::Branching,
                HarnessCapability::ExactResume,
                HarnessCapability::ImportCapsule,
                HarnessCapability::ExportNativeState,
            ]),
        ),
        (
            "opencode",
            "omnisolo/harness-worker-opencode:1.18.15",
            BTreeSet::from([
                HarnessCapability::Prompt,
                HarnessCapability::Streaming,
                HarnessCapability::Cancellation,
                HarnessCapability::Workspace,
                HarnessCapability::ExactResume,
                HarnessCapability::ImportCapsule,
            ]),
        ),
        (
            "deepseek",
            "omnisolo/harness-worker-deepseek:0.1.1-rc.2",
            BTreeSet::from([
                HarnessCapability::Prompt,
                HarnessCapability::Streaming,
                HarnessCapability::Workspace,
                HarnessCapability::ImportCapsule,
            ]),
        ),
        (
            "pi",
            "omnisolo/harness-worker-pi:0.73.1",
            BTreeSet::from([
                HarnessCapability::Prompt,
                HarnessCapability::Streaming,
                HarnessCapability::Cancellation,
                HarnessCapability::Steering,
                HarnessCapability::Workspace,
                HarnessCapability::ImportCapsule,
            ]),
        ),
        (
            "kimi",
            "omnisolo/harness-worker-kimi:1.49.0",
            BTreeSet::from([
                HarnessCapability::Prompt,
                HarnessCapability::Streaming,
                HarnessCapability::Cancellation,
                HarnessCapability::Approvals,
                HarnessCapability::Questions,
                HarnessCapability::Workspace,
                HarnessCapability::ExactResume,
                HarnessCapability::ImportCapsule,
            ]),
        ),
        (
            "openhands",
            "omnisolo/harness-worker-openhands:1.43.1",
            BTreeSet::from([
                HarnessCapability::Prompt,
                HarnessCapability::Streaming,
                HarnessCapability::Cancellation,
                HarnessCapability::Approvals,
                HarnessCapability::Workspace,
                HarnessCapability::ExactResume,
                HarnessCapability::ImportCapsule,
            ]),
        ),
        (
            "openharness",
            "omnisolo/harness-worker-openharness:0.6.0",
            BTreeSet::from([
                HarnessCapability::Prompt,
                HarnessCapability::Streaming,
                HarnessCapability::Cancellation,
                HarnessCapability::Steering,
                HarnessCapability::Workspace,
                HarnessCapability::ExactResume,
                HarnessCapability::ImportCapsule,
            ]),
        ),
    ];

    for (harness_id, image, capabilities) in cases {
        let descriptor = registry.descriptor(harness_id).expect("descriptor");
        assert_eq!(descriptor.worker_image.as_deref(), Some(image));
        assert_eq!(descriptor.capabilities, capabilities, "{harness_id}");
        assert!(!image.ends_with(":latest"));
    }
}

#[test]
fn protocol_kinds_use_canonical_names_and_accept_legacy_aliases() {
    let cases = [
        (
            HarnessProtocolKind::OpenCodeHttp,
            "opencode_http",
            "opencode_json_rpc",
        ),
        (
            HarnessProtocolKind::DeepSeekJsonRpc,
            "deepseek_json_rpc",
            "deep_seek_json_rpc",
        ),
        (HarnessProtocolKind::PiRpc, "pi_rpc", "pi_jsonl"),
        (HarnessProtocolKind::KimiAcp, "kimi_acp", "kimi_json_rpc"),
        (HarnessProtocolKind::KimiAcp, "kimi_acp", "acp_v1"),
        (
            HarnessProtocolKind::OpenHandsHttp,
            "openhands_http",
            "open_hands_http",
        ),
        (
            HarnessProtocolKind::OpenHarnessSdk,
            "openharness_sdk",
            "open_harness_acp",
        ),
    ];

    for (kind, canonical, legacy) in cases {
        assert_eq!(serde_json::to_value(&kind).unwrap(), json!(canonical));
        assert_eq!(
            serde_json::from_value::<HarnessProtocolKind>(json!(legacy)).unwrap(),
            kind
        );
    }
}

#[test]
fn every_protocol_has_an_explicit_runtime_family_and_only_custom_is_legacy() {
    let cases = [
        (HarnessProtocolKind::OmniSolo, RuntimeFamily::InProcess),
        (HarnessProtocolKind::CodexAppServer, RuntimeFamily::JsonRpc),
        (HarnessProtocolKind::DeepSeekJsonRpc, RuntimeFamily::JsonRpc),
        (HarnessProtocolKind::KimiAcp, RuntimeFamily::JsonRpc),
        (HarnessProtocolKind::PiRpc, RuntimeFamily::JsonLines),
        (
            HarnessProtocolKind::OpenHarnessSdk,
            RuntimeFamily::JsonLines,
        ),
        (HarnessProtocolKind::OpenCodeHttp, RuntimeFamily::Http),
        (HarnessProtocolKind::OpenHandsHttp, RuntimeFamily::Http),
        (HarnessProtocolKind::Custom, RuntimeFamily::LegacyJsonLines),
    ];

    for (kind, family) in cases {
        assert_eq!(kind.runtime_family(), family);
        assert_eq!(
            kind.uses_legacy_json_lines(),
            matches!(kind, HarnessProtocolKind::Custom)
        );
    }
}

#[test]
fn process_adapter_dispatches_every_builtin_to_its_native_runtime_family() {
    for preset in [
        ExternalHarnessPreset::Codex,
        ExternalHarnessPreset::OpenCode,
        ExternalHarnessPreset::DeepSeek,
        ExternalHarnessPreset::Pi,
        ExternalHarnessPreset::Kimi,
        ExternalHarnessPreset::OpenHands,
        ExternalHarnessPreset::OpenHarness,
    ] {
        let adapter = ProcessHarnessAdapter::new(ProcessHarnessSpec::command(
            "/bin/sh",
            ["-c", "read line"],
            preset.harness_id(),
        ));
        assert_eq!(
            adapter.runtime_family(),
            preset.protocol_kind().runtime_family()
        );
        assert!(!adapter.protocol_kind().uses_legacy_json_lines());
    }

    let custom = ProcessHarnessAdapter::new(
        ProcessHarnessSpec::command("/bin/sh", ["-c", "read line"], "future")
            .with_protocol(HarnessProtocolKind::Custom),
    );
    assert_eq!(custom.runtime_family(), RuntimeFamily::LegacyJsonLines);
}

#[tokio::test]
async fn deepseek_session_creation_is_lazy_and_does_not_spawn_or_send_session_new() {
    let request = HarnessSessionRequest::new("tenant", Uuid::new_v4(), Uuid::new_v4());
    let mut adapter = ProcessHarnessAdapter::new(ProcessHarnessSpec::command(
        "/definitely/missing/deepseek-harness",
        std::iter::empty::<String>(),
        "deepseek",
    ));

    let native = adapter.create_session(request.clone()).await.unwrap();
    assert_eq!(native.native_session_id, request.session_id.to_string());
    assert_eq!(native.native_cursor, None);
}

#[tokio::test]
async fn deepseek_virtual_session_close_and_delete_are_emulated_by_the_adapter() {
    let request = HarnessSessionRequest::new("tenant", Uuid::new_v4(), Uuid::new_v4());
    let mut adapter = ProcessHarnessAdapter::new(ProcessHarnessSpec::command(
        "/definitely/missing/deepseek-harness",
        std::iter::empty::<String>(),
        "deepseek",
    ));

    let native = adapter.create_session(request.clone()).await.unwrap();
    let mut delete_request = request;
    delete_request.extensions.insert(
        "native_session_id".to_owned(),
        serde_json::Value::String(native.native_session_id),
    );

    adapter.close_session(delete_request.clone()).await.unwrap();
    adapter.delete_session(delete_request).await.unwrap();
}

#[tokio::test]
async fn builtins_never_fall_back_to_the_custom_json_lines_protocol() {
    let request = HarnessSessionRequest::new("tenant", Uuid::new_v4(), Uuid::new_v4());
    for harness_id in ["opencode", "pi", "openhands", "openharness"] {
        let mut adapter = ProcessHarnessAdapter::new(ProcessHarnessSpec::command(
            "/definitely/missing/native-harness",
            std::iter::empty::<String>(),
            harness_id,
        ));
        let error = adapter.create_session(request.clone()).await.unwrap_err();
        let message = error.to_string();
        match harness_id {
            "opencode" => assert!(
                !message.contains("generic JSON-lines dispatch"),
                "OpenCode unexpectedly reached generic JSONL: {error}"
            ),
            "openhands" => assert!(
                !message.contains("generic JSON-lines dispatch"),
                "OpenHands unexpectedly reached generic JSONL: {error}"
            ),
            "pi" => assert!(
                message.contains("Pi RPC requires a resolved model selection"),
                "unexpected Pi error: {error}"
            ),
            "openharness" => assert!(
                !message.contains("generic JSON-lines dispatch"),
                "OpenHarness unexpectedly reached generic JSONL: {error}"
            ),
            _ => unreachable!(),
        }
    }
}

#[tokio::test]
async fn opencode_start_attempt_uses_the_native_http_runtime() {
    let request = HarnessSessionRequest::new("tenant", Uuid::new_v4(), Uuid::new_v4())
        .with_resolved_model(resolved_model_selection());
    let mut adapter = ProcessHarnessAdapter::new(ProcessHarnessSpec::command(
        "/definitely/missing/opencode",
        std::iter::empty::<String>(),
        "opencode",
    ));

    let error = adapter
        .start_attempt(request, "attempt", "prompt", Some("native-session"))
        .await
        .unwrap_err();
    assert!(
        !error.to_string().contains("generic JSON-lines dispatch"),
        "OpenCode start unexpectedly reached generic JSONL: {error}"
    );
}

#[test]
fn default_registry_contains_eight_native_and_four_openai_shim_harnesses() {
    let registry = HarnessRegistry::with_defaults();
    let ids = registry
        .descriptors()
        .map(|descriptor| descriptor.harness_id.as_str())
        .collect::<Vec<_>>();

    assert_eq!(ids.len(), 12);
    for id in [
        "omnisolo",
        "codex",
        "opencode",
        "deepseek",
        "pi",
        "kimi",
        "openhands",
        "openharness",
        "aider",
        "goose",
        "open-interpreter",
        "plandex",
    ] {
        assert!(ids.contains(&id), "missing harness {id}");
    }
    assert_eq!(
        registry.descriptor("aider").unwrap().metadata["integration_mode"],
        "openai_compatible"
    );
    assert_eq!(
        registry.descriptor("codex").unwrap().metadata["integration_mode"],
        "native"
    );
}

#[test]
fn shim_presets_have_exact_pins_and_explicit_protocol_kind() {
    let registry = HarnessRegistry::with_defaults();
    let expected = [
        ("aider", "0.86.0", "omnisolo/harness-worker-aider:0.86.0"),
        ("goose", "1.33.1", "omnisolo/harness-worker-goose:1.33.1"),
        (
            "open-interpreter",
            "0.4.2",
            "omnisolo/harness-worker-open-interpreter:0.4.2",
        ),
        (
            "plandex",
            "cli/v2.2.1",
            "omnisolo/harness-worker-plandex:2.2.1",
        ),
    ];

    for (harness_id, version, image) in expected {
        let descriptor = registry.descriptor(harness_id).unwrap();
        assert_eq!(descriptor.implementation_version, version);
        assert_eq!(descriptor.worker_image.as_deref(), Some(image));
        assert_eq!(
            serde_json::to_value(&descriptor.protocol_kind).unwrap(),
            json!("openai_compatible_shim")
        );
        assert_eq!(descriptor.metadata["integration_mode"], "openai_compatible");
    }
}

fn native_opencode_http_script() -> &'static str {
    r#"
import json
import sys
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

host = sys.argv[sys.argv.index("--hostname") + 1]
port = int(sys.argv[sys.argv.index("--port") + 1])
session_id = "ses_opencode_import_1"
state = {"prompt_count": 0, "message_id": None}

def session():
    return {
        "id": session_id,
        "slug": "portable-import",
        "projectID": "global",
        "directory": "/workspace",
        "title": "portable import",
        "version": "1.18.15",
        "time": {"created": 1, "updated": 1},
        "model": {"providerID": "omnisolo-openai-compatible", "id": "gpt-5.6-luna", "variant": "max"},
    }

class Handler(BaseHTTPRequestHandler):
    def log_message(self, *_):
        pass

    def read_json(self):
        size = int(self.headers.get("content-length", "0"))
        return json.loads(self.rfile.read(size)) if size else None

    def reply(self, status, body):
        encoded = json.dumps(body, separators=(",", ":")).encode()
        self.send_response(status)
        self.send_header("content-type", "application/json")
        self.send_header("content-length", str(len(encoded)))
        self.end_headers()
        self.wfile.write(encoded)

    def do_GET(self):
        if self.path == "/global/health":
            return self.reply(200, {"healthy": True, "version": "1.18.15"})
        if self.path == "/event":
            baseline = state["prompt_count"]
            self.send_response(200)
            self.send_header("content-type", "text/event-stream")
            self.send_header("connection", "close")
            self.end_headers()
            deadline = time.time() + 3
            while state["prompt_count"] == baseline and time.time() < deadline:
                time.sleep(0.01)
            message_id = state["message_id"]
            count = state["prompt_count"]
            event = {
                "id": f"evt_final_{count}",
                "type": "message.updated",
                "properties": {
                    "sessionID": session_id,
                    "info": {
                        "id": f"msg_assistant_{count}",
                        "sessionID": session_id,
                        "role": "assistant",
                        "parentID": message_id,
                        "time": {"created": 1, "completed": 2},
                        "cost": 0,
                        "tokens": {"total": 2, "input": 1, "output": 1, "reasoning": 0, "cache": {"read": 0, "write": 0}},
                        "finish": "stop",
                    },
                },
            }
            self.wfile.write(("data: " + json.dumps(event, separators=(",", ":")) + "\n\n").encode())
            self.wfile.flush()
            return
        if self.path == f"/session/{session_id}":
            return self.reply(200, session())
        return self.reply(404, {"name": "NotFoundError"})

    def do_POST(self):
        body = self.read_json()
        if self.path == "/session":
            return self.reply(200, session())
        if self.path == f"/session/{session_id}/prompt_async":
            text = body["parts"][0]["text"]
            if state["prompt_count"] == 0:
                assert "<omnisolo_portable_session_context>" in text
                assert '"schema":"omnisolo.portable_session_context.v1"' in text
                assert text.endswith("<current_user_request>\ncontinue OpenCode work\n</current_user_request>")
            else:
                assert text == "second prompt"
            state["message_id"] = body["messageID"]
            state["prompt_count"] += 1
            self.send_response(204)
            self.send_header("content-length", "0")
            self.end_headers()
            return
        if self.path == f"/session/{session_id}/abort":
            return self.reply(200, True)
        return self.reply(404, {"name": "NotFoundError"})

ThreadingHTTPServer((host, port), Handler).serve_forever()
"#
}

#[tokio::test]
async fn opencode_process_adapter_imports_portable_context_on_the_first_native_prompt_only() {
    let request = HarnessSessionRequest::new("tenant-opencode", Uuid::new_v4(), Uuid::new_v4())
        .with_task(Uuid::new_v4(), "portable OpenCode task")
        .with_resolved_model(resolved_model_selection());
    let mut adapter = ProcessHarnessAdapter::new(
        ProcessHarnessSpec::command(
            "python3",
            ["-u", "-c", native_opencode_http_script()],
            "opencode",
        )
        .with_model_routing(
            resolved_model_selection(),
            Some("https://llmapi.omnisolo.co/v1".to_owned()),
        )
        .with_timeout(Duration::from_secs(3)),
    );

    let native = adapter
        .import_session(request.clone(), test_capsule("opencode", &request))
        .await
        .unwrap();
    assert_eq!(native.native_session_id, "ses_opencode_import_1");

    for (attempt_id, prompt) in [
        ("opencode-import-1", "continue OpenCode work"),
        ("opencode-import-2", "second prompt"),
    ] {
        let execution = adapter
            .execute(
                request.clone(),
                attempt_id,
                prompt,
                Some(&native.native_session_id),
            )
            .await
            .unwrap();
        assert!(
            execution
                .events
                .iter()
                .any(|event| event.event_type == "assistant.final")
        );
    }
}

fn native_openhands_http_script() -> &'static str {
    r#"
import json
import os
import sys
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

host = sys.argv[sys.argv.index("--host") + 1]
port = int(sys.argv[sys.argv.index("--port") + 1])
capture = os.environ["OMNISOLO_OPENHANDS_SHARED_CAPTURE"]
conversation_id = "shared-openhands-conversation"
state = {"prompted": False, "approved": False}

def record(method, path, body=None):
    with open(capture, "a", encoding="utf-8") as stream:
        stream.write(json.dumps({
            "method": method,
            "path": path,
            "body": body,
            "has_api_key": bool(os.environ.get("OPENAI_API_KEY")),
        }, separators=(",", ":")) + "\n")

class Handler(BaseHTTPRequestHandler):
    def log_message(self, *_):
        pass

    def read_json(self):
        size = int(self.headers.get("content-length", "0"))
        return json.loads(self.rfile.read(size)) if size else None

    def reply(self, status, body):
        encoded = json.dumps(body, separators=(",", ":")).encode()
        self.send_response(status)
        self.send_header("content-type", "application/json")
        self.send_header("content-length", str(len(encoded)))
        self.end_headers()
        self.wfile.write(encoded)

    def do_GET(self):
        record("GET", self.path)
        if self.path == "/ready":
            return self.reply(200, {"status": "ready"})
        if self.path == "/server_info":
            return self.reply(200, {
                "title": "OpenHands Agent Server",
                "version": "1.43.1",
                "uptime": 1,
                "idle_time": 0,
            })
        if self.path.startswith(f"/api/conversations/{conversation_id}/events/search"):
            events = []
            if state["prompted"]:
                events.append({
                    "id": "shared-action",
                    "kind": "ActionEvent",
                    "source": "agent",
                    "action": {"kind": "ExecuteBashAction", "command": "pwd"},
                })
            if state["approved"]:
                events.extend([
                    {
                        "id": "shared-observation",
                        "kind": "ObservationEvent",
                        "source": "environment",
                        "observation": {"stdout": "/workspace/project"},
                    },
                    {
                        "id": "shared-message",
                        "kind": "MessageEvent",
                        "source": "agent",
                        "message": {"text": "native OpenHands answer"},
                    },
                ])
            return self.reply(200, {"items": events, "next_page_id": None})
        if self.path == f"/api/conversations/{conversation_id}":
            return self.reply(200, {
                "id": conversation_id,
                "execution_status": "finished" if state["approved"] else "waiting_for_confirmation",
                "metrics": None,
                "stats": {
                    "usage_to_metrics": {
                        "agent": {
                            "accumulated_token_usage": {
                                "prompt_tokens": 13,
                                "completion_tokens": 8,
                            }
                        }
                    }
                },
            })
        return self.reply(404, {"detail": "not found"})

    def do_POST(self):
        body = self.read_json()
        record("POST", self.path, body)
        if self.path == "/api/conversations":
            llm = body["agent"]["llm"]
            assert llm["model"] == "openai/gpt-5.6-luna"
            assert llm["base_url"] == "https://llmapi.omnisolo.co/v1"
            assert llm["api_mode"] == "responses"
            assert llm["reasoning_effort"] == "max"
            assert body["workspace"]["working_dir"] == os.environ["OMNISOLO_OPENHANDS_EXPECTED_WORKSPACE"]
            assert body["confirmation_policy"] == {"kind": "AlwaysConfirm"}
            selected = body.get("conversation_id", conversation_id)
            return self.reply(201, {"id": selected, "execution_status": "idle"})
        if self.path == f"/api/conversations/{conversation_id}/events":
            assert body["role"] == "user"
            transferred = body["content"][0]["text"]
            assert "<omnisolo_portable_session_context>" in transferred
            assert '"schema":"omnisolo.portable_session_context.v1"' in transferred
            assert transferred.endswith("<current_user_request>\nuse native OpenHands\n</current_user_request>")
            state["prompted"] = True
            return self.reply(200, {"success": True})
        if self.path == f"/api/conversations/{conversation_id}/events/respond_to_confirmation":
            assert body == {"accept": True, "reason": "approved in shared adapter"}
            state["approved"] = True
            return self.reply(200, {"success": True})
        if self.path == f"/api/conversations/{conversation_id}/interrupt":
            return self.reply(200, {"success": True})
        return self.reply(404, {"detail": "not found"})

    def do_DELETE(self):
        record("DELETE", self.path)
        if self.path == f"/api/conversations/{conversation_id}":
            return self.reply(200, {"success": True})
        return self.reply(404, {"detail": "not found"})

ThreadingHTTPServer((host, port), Handler).serve_forever()
"#
}

#[tokio::test]
async fn openhands_process_adapter_uses_native_http_for_session_stream_approval_and_cancel() {
    let capture = std::env::temp_dir().join(format!(
        "omnisolo-openhands-shared-{}.jsonl",
        Uuid::new_v4()
    ));
    let request = HarnessSessionRequest::new("tenant-openhands", Uuid::new_v4(), Uuid::new_v4())
        .with_task(Uuid::new_v4(), "OpenHands native task")
        .with_resolved_model(resolved_model_selection());
    let workspace_root =
        std::env::temp_dir().join(format!("omnisolo-openhands-workspace-{}", Uuid::new_v4()));
    let expected_workspace = workspace_root
        .join(".omnisolo")
        .join("sessions")
        .join(request.session_id.to_string());
    let mut adapter = ProcessHarnessAdapter::new(
        ProcessHarnessSpec::command(
            "python3",
            ["-u", "-c", native_openhands_http_script()],
            "openhands",
        )
        .with_environment(
            "OMNISOLO_OPENHANDS_SHARED_CAPTURE",
            capture.to_string_lossy(),
        )
        .with_environment("OPENAI_API_KEY", "shared-child-secret-canary")
        .with_environment(
            "OMNISOLO_OPENHANDS_EXPECTED_WORKSPACE",
            expected_workspace.to_string_lossy(),
        )
        .with_model_routing(
            resolved_model_selection(),
            Some("https://llmapi.omnisolo.co/v1".to_owned()),
        )
        .with_working_directory(&workspace_root)
        .with_timeout(Duration::from_secs(3)),
    );

    let created = adapter
        .import_session(request.clone(), test_capsule("openhands", &request))
        .await
        .unwrap();
    assert_eq!(created.native_session_id, "shared-openhands-conversation");
    let resumed = adapter
        .resume_session(request.clone(), &created.native_session_id)
        .await
        .unwrap();
    assert_eq!(resumed.native_session_id, created.native_session_id);

    let mut stream = adapter
        .attempt_stream(
            AttemptOperation::Execute,
            request.clone(),
            "openhands-attempt",
            "use native OpenHands",
            Some(&created.native_session_id),
        )
        .await
        .unwrap();
    let mut downgrade_count = 0;
    let mut saw_action = false;
    let mut saw_approval = false;
    while !saw_approval {
        let item = tokio::time::timeout(Duration::from_secs(2), stream.next())
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        if let HarnessExecutionItem::Event(event) = item {
            downgrade_count += usize::from(event.event_type == "capability.downgraded");
            saw_action |= event.event_type == "tool.started"
                && event.payload["native"]["action"]["command"] == "pwd";
            saw_approval |= event.event_type == "interaction.required"
                && event.payload["native_session_id"] == created.native_session_id;
        }
    }
    assert_eq!(downgrade_count, 0);
    assert!(saw_action && saw_approval);

    let approval = adapter
        .exchange(
            request.clone(),
            "approval.response",
            json!({
                "native_session_id": created.native_session_id,
                "accept": true,
                "reason": "approved in shared adapter"
            }),
        )
        .await
        .unwrap();
    assert_eq!(approval, json!({"accepted": true}));

    let mut saw_observation = false;
    let mut completed = None;
    while let Some(item) = tokio::time::timeout(Duration::from_secs(2), stream.next())
        .await
        .unwrap()
    {
        match item.unwrap() {
            HarnessExecutionItem::Event(event) => {
                saw_observation |= event.event_type == "tool.completed"
                    && event.payload["native"]["observation"]["stdout"] == "/workspace/project";
            }
            HarnessExecutionItem::Completed { final_text, usage } => {
                completed = Some((final_text, usage));
                break;
            }
        }
    }
    assert!(saw_observation);
    let (final_text, usage) = completed.unwrap();
    assert_eq!(final_text.as_deref(), Some("native OpenHands answer"));
    assert_eq!(
        usage.unwrap()["agent"]["accumulated_token_usage"]["prompt_tokens"],
        13
    );

    adapter
        .control_attempt(
            request.clone(),
            "openhands-cancel",
            "cancel",
            "",
            Some(&created.native_session_id),
        )
        .await
        .unwrap();
    let mut delete_request = request;
    delete_request.extensions.insert(
        "native_session_id".to_owned(),
        Value::String(created.native_session_id.clone()),
    );
    adapter.delete_session(delete_request).await.unwrap();

    let records = fs::read_to_string(&capture).unwrap();
    assert!(records.lines().all(|line| !line.contains("\"operation\"")));
    assert!(records.contains("/events/respond_to_confirmation"));
    assert!(records.contains("/interrupt"));
    assert!(records.contains("\"method\":\"DELETE\""));
    assert!(
        records
            .lines()
            .all(|line| line.contains("\"has_api_key\":true"))
    );
    assert!(!records.contains("shared-child-secret-canary"));
    fs::remove_file(capture).unwrap();
    fs::remove_dir_all(workspace_root).unwrap();
}

#[tokio::test]
async fn openharness_process_adapter_uses_native_jsonl_runtime_never_generic_jsonl() {
    let script = r#"
import json
import os
import sys

active = None
session_id = "adapter-openharness-session"
prompt_count = 0

def emit(frame):
    print(json.dumps(frame, separators=(",", ":")), flush=True)

emit({"type":"ready","protocol_version":1,"sdk_version":"0.6.0"})

for line in sys.stdin:
    command = json.loads(line)
    if "operation" in command:
        raise SystemExit("generic JSONL reached")
    command_type = command["type"]
    command_id = command["id"]
    if command_type == "create_session":
        emit({"type":"session","id":command_id,"command":command_type,"session_id":session_id})
    elif command_type == "resume_session":
        session_id = command["session_id"]
        emit({"type":"session","id":command_id,"command":command_type,"session_id":session_id})
    elif command_type == "prompt":
        prompt_count += 1
        resolved = command["resolved_model"]
        assert resolved["provider"] == "openai"
        assert resolved["model_id"] == "gpt-5.6-luna"
        assert resolved["base_url"] == "https://llmapi.omnisolo.co/v1"
        assert resolved["reasoning_effort"] == "max"
        assert resolved["api_dialect"] == "openai_responses"
        assert resolved["context_window_tokens"] == 400000
        assert resolved["max_output_tokens"] == 128000
        assert resolved["binding_revision"] == "binding-v1"
        assert resolved["binding_digest"] == "sha256:test"
        assert command["permission_mode"] == "accept_edits"
        assert command["cwd"] == os.environ["OMNISOLO_OPENHARNESS_EXPECTED_WORKSPACE"]
        if prompt_count == 1:
            assert "<omnisolo_portable_session_context>" in command["prompt"]
            assert '"schema":"omnisolo.portable_session_context.v1"' in command["prompt"]
            assert command["prompt"].endswith("<current_user_request>\nwait for steer\n</current_user_request>")
        else:
            assert command["prompt"] == "wait for cancel"
        active = (command_id, command["session_id"])
        emit({"type":"ack","id":command_id,"command":command_type})
        emit({"type":"system","request_id":command_id,"session_id":command["session_id"],"event":"session_start","data":{"phase":"running"}})
        emit({"type":"text","request_id":command_id,"session_id":command["session_id"],"text":"native text","is_partial":True})
    elif command_type == "steer":
        emit({"type":"ack","id":command_id,"command":command_type})
        request_id, native_session_id = active
        emit({"type":"result","request_id":request_id,"session_id":native_session_id,"text":"native final","turns":2,"tool_calls":0,"total_tokens":21,"total_cost":0.2,"stop_reason":"end_turn"})
        active = None
    elif command_type == "cancel":
        emit({"type":"ack","id":command_id,"command":command_type})
        request_id, native_session_id = active
        emit({"type":"cancelled","request_id":request_id,"session_id":native_session_id})
        active = None
    elif command_type == "delete_session":
        emit({"type":"session_deleted","id":command_id,"command":command_type,"session_id":command["session_id"]})
    elif command_type == "shutdown":
        emit({"type":"ack","id":command_id,"command":command_type})
        break
"#;
    let mut request = HarnessSessionRequest::new("tenant", Uuid::new_v4(), Uuid::new_v4())
        .with_task(Uuid::new_v4(), "exercise native OpenHarness runtime")
        .with_resolved_model(resolved_model_selection());
    request
        .extensions
        .insert("permission_mode".to_owned(), json!("accept_edits"));
    let workspace_root =
        std::env::temp_dir().join(format!("omnisolo-openharness-workspace-{}", Uuid::new_v4()));
    let expected_workspace = workspace_root
        .join(".omnisolo")
        .join("sessions")
        .join(request.session_id.to_string());
    let spec = ProcessHarnessSpec::command(
        "python3",
        ["-c".to_owned(), script.to_owned()],
        "openharness",
    )
    .with_model_routing(
        resolved_model_selection(),
        Some("https://llmapi.omnisolo.co/v1".to_owned()),
    )
    .with_environment("OPENAI_API_KEY", "adapter-runtime-secret-canary")
    .with_environment(
        "OMNISOLO_OPENHARNESS_EXPECTED_WORKSPACE",
        expected_workspace.to_string_lossy(),
    )
    .with_working_directory(&workspace_root)
    .with_timeout(Duration::from_secs(2));
    assert!(!format!("{spec:?}").contains("adapter-runtime-secret-canary"));
    let mut adapter = ProcessHarnessAdapter::new(spec);

    let created = adapter
        .import_session(request.clone(), test_capsule("openharness", &request))
        .await
        .unwrap();
    assert_eq!(created.native_session_id, "adapter-openharness-session");
    let resumed = adapter
        .resume_session(request.clone(), &created.native_session_id)
        .await
        .unwrap();
    assert_eq!(resumed.native_session_id, created.native_session_id);

    let mut stream = adapter
        .attempt_stream(
            AttemptOperation::Execute,
            request.clone(),
            "attempt-native-1",
            "wait for steer",
            Some(&created.native_session_id),
        )
        .await
        .unwrap();
    let mut event_types = Vec::new();
    loop {
        match stream.next().await.unwrap().unwrap() {
            HarnessExecutionItem::Event(event) => {
                event_types.push(event.event_type.clone());
                if event.event_type == "assistant.text_chunk" {
                    break;
                }
            }
            HarnessExecutionItem::Completed { .. } => panic!("stream ended before steering"),
        }
    }
    adapter
        .control_attempt(
            request.clone(),
            "attempt-native-1",
            "steer",
            "finish now",
            Some(&created.native_session_id),
        )
        .await
        .unwrap();
    let mut completed = None;
    while let Some(item) = stream.next().await {
        match item.unwrap() {
            HarnessExecutionItem::Event(event) => event_types.push(event.event_type),
            HarnessExecutionItem::Completed { final_text, usage } => {
                completed = Some((final_text, usage));
            }
        }
    }
    assert_eq!(
        event_types,
        [
            "inference.model_binding",
            "turn.started",
            "assistant.text_chunk",
            "turn.completed"
        ]
    );
    let (final_text, usage) = completed.unwrap();
    assert_eq!(final_text.as_deref(), Some("native final"));
    assert_eq!(usage.unwrap()["total_tokens"], json!(21));

    let mut cancelled_stream = adapter
        .attempt_stream(
            AttemptOperation::Execute,
            request.clone(),
            "attempt-native-2",
            "wait for cancel",
            Some(&created.native_session_id),
        )
        .await
        .unwrap();
    while let HarnessExecutionItem::Event(event) = cancelled_stream.next().await.unwrap().unwrap() {
        if event.event_type == "assistant.text_chunk" {
            break;
        }
    }
    adapter
        .control_attempt(
            request.clone(),
            "attempt-native-2",
            "cancel",
            "",
            Some(&created.native_session_id),
        )
        .await
        .unwrap();
    let mut saw_cancelled = false;
    while let Some(item) = cancelled_stream.next().await {
        if let HarnessExecutionItem::Event(event) = item.unwrap() {
            saw_cancelled |= event.event_type == "turn.cancelled";
        }
    }
    assert!(saw_cancelled);
    request.extensions.insert(
        "native_session_id".to_owned(),
        json!(created.native_session_id),
    );
    adapter.delete_session(request).await.unwrap();
    fs::remove_dir_all(workspace_root).unwrap();
}

#[tokio::test]
async fn kimi_adapter_sends_acp_cancellation_as_a_notification() {
    let script = r#"
read initialize
initialize_id=$(printf '%s' "$initialize" | sed -n 's/.*"id":\([0-9][0-9]*\).*/\1/p')
printf '{"jsonrpc":"2.0","id":%s,"result":{"protocolVersion":1,"agentCapabilities":{"loadSession":true},"agentInfo":{"name":"kimi-cli","version":"test"}}}\n' "$initialize_id"

read create
create_id=$(printf '%s' "$create" | sed -n 's/.*"id":\([0-9][0-9]*\).*/\1/p')
printf '{"jsonrpc":"2.0","id":%s,"result":{"sessionId":"kimi-session-1"}}\n' "$create_id"

read cancel
if printf '%s' "$cancel" | grep -q '"method":"session/cancel"' \
  && printf '%s' "$cancel" | grep -q '"sessionId":"kimi-session-1"' \
  && ! printf '%s' "$cancel" | grep -q '"id":'; then
  sleep 10
else
  exit 42
fi
"#;
    let mut adapter = ProcessHarnessAdapter::new(
        ProcessHarnessSpec::command("/bin/sh", ["-c", script], "kimi")
            .with_working_directory("/workspace"),
    );
    let request = HarnessSessionRequest::new("tenant", Uuid::new_v4(), Uuid::new_v4());
    let native = adapter.create_session(request.clone()).await.unwrap();

    let execution = adapter
        .control_attempt(
            request,
            "attempt-cancel",
            "cancel",
            "",
            Some(&native.native_session_id),
        )
        .await
        .unwrap();
    assert!(execution.events.is_empty());
}

#[tokio::test]
async fn kimi_session_close_and_delete_are_portably_emulated_without_non_acp_methods() {
    let script = r#"
read initialize
initialize_id=$(printf '%s' "$initialize" | sed -n 's/.*"id":\([0-9][0-9]*\).*/\1/p')
printf '{"jsonrpc":"2.0","id":%s,"result":{"protocolVersion":1,"agentCapabilities":{"loadSession":true},"agentInfo":{"name":"kimi-cli","version":"test"}}}\n' "$initialize_id"

read create
create_id=$(printf '%s' "$create" | sed -n 's/.*"id":\([0-9][0-9]*\).*/\1/p')
if ! printf '%s' "$create" | grep -q '"method":"session/new"'; then exit 61; fi
printf '{"jsonrpc":"2.0","id":%s,"result":{"sessionId":"kimi-session-lifecycle"}}\n' "$create_id"

# ACP has no session close/delete request. Any additional line is a contract failure.
if read unexpected; then exit 62; fi
"#;
    let mut adapter = ProcessHarnessAdapter::new(
        ProcessHarnessSpec::command("/bin/sh", ["-c", script], "kimi")
            .with_working_directory("/workspace"),
    );
    let request = HarnessSessionRequest::new("tenant", Uuid::new_v4(), Uuid::new_v4());
    let native = adapter.create_session(request.clone()).await.unwrap();
    assert_eq!(native.native_session_id, "kimi-session-lifecycle");

    let mut lifecycle_request = request;
    lifecycle_request.extensions.insert(
        "native_session_id".to_owned(),
        Value::String(native.native_session_id),
    );
    adapter
        .close_session(lifecycle_request.clone())
        .await
        .unwrap();
    adapter.delete_session(lifecycle_request).await.unwrap();
}

#[tokio::test]
async fn kimi_shared_runtime_configures_the_session_before_prompt_and_uses_jsonrpc_v2() {
    let script = r#"
read initialize
initialize_id=$(printf '%s' "$initialize" | sed -n 's/.*"id":\([0-9][0-9]*\).*/\1/p')
if ! printf '%s' "$initialize" | grep -q '"jsonrpc":"2.0"'; then exit 40; fi
printf '{"jsonrpc":"2.0","id":%s,"result":{"protocolVersion":1,"agentCapabilities":{"loadSession":true},"agentInfo":{"name":"kimi-cli","version":"test"}}}\n' "$initialize_id"

read create
create_id=$(printf '%s' "$create" | sed -n 's/.*"id":\([0-9][0-9]*\).*/\1/p')
if ! printf '%s' "$create" | grep -q '"jsonrpc":"2.0"'; then exit 41; fi
printf '{"jsonrpc":"2.0","id":%s,"result":{"sessionId":"kimi-session-configured","configOptions":[{"id":"model","category":"model","type":"select","options":[{"value":"gpt-5.6-luna"}]},{"id":"thinking","category":"thought_level","type":"select","options":[{"value":"high"}]}]}}\n' "$create_id"

read model_config
model_id=$(printf '%s' "$model_config" | sed -n 's/.*"id":\([0-9][0-9]*\).*/\1/p')
if ! printf '%s' "$model_config" | grep -q '"jsonrpc":"2.0"' \
  || ! printf '%s' "$model_config" | grep -q '"method":"session/set_config_option"' \
  || ! printf '%s' "$model_config" | grep -q '"configId":"model"' \
  || ! printf '%s' "$model_config" | grep -q '"value":"gpt-5.6-luna"'; then exit 42; fi
printf '{"jsonrpc":"2.0","id":%s,"result":{}}\n' "$model_id"

read thought_config
thought_id=$(printf '%s' "$thought_config" | sed -n 's/.*"id":\([0-9][0-9]*\).*/\1/p')
if ! printf '%s' "$thought_config" | grep -q '"configId":"thinking"' \
  || ! printf '%s' "$thought_config" | grep -q '"value":"high"'; then exit 43; fi
printf '{"jsonrpc":"2.0","id":%s,"result":{}}\n' "$thought_id"

read prompt
prompt_id=$(printf '%s' "$prompt" | sed -n 's/.*"id":\([0-9][0-9]*\).*/\1/p')
if ! printf '%s' "$prompt" | grep -q '"method":"session/prompt"' \
  || ! printf '%s' "$prompt" | grep -q 'omnisolo_portable_session_context' \
  || ! printf '%s' "$prompt" | grep -q 'Answer after configuration'; then exit 44; fi
printf '%s\n' '{"jsonrpc":"2.0","method":"session/update","params":{"sessionId":"kimi-session-configured","update":{"sessionUpdate":"agent_message_chunk","content":{"type":"text","text":"configured answer"}}}}'
printf '{"jsonrpc":"2.0","id":%s,"result":{"stopReason":"end_turn","usage":{"inputTokens":2,"outputTokens":3}}}\n' "$prompt_id"
sleep 10
"#;
    let request = HarnessSessionRequest::new("tenant", Uuid::new_v4(), Uuid::new_v4())
        .with_resolved_model(resolved_model_selection());
    let mut adapter = ProcessHarnessAdapter::new(
        ProcessHarnessSpec::command("/bin/sh", ["-c", script], "kimi")
            .with_working_directory("/workspace"),
    );

    let native = adapter
        .import_session(request.clone(), test_capsule("kimi", &request))
        .await
        .unwrap();
    let execution = adapter
        .execute(
            request,
            "attempt-configured",
            "Answer after configuration",
            Some(&native.native_session_id),
        )
        .await
        .unwrap();
    assert!(execution.events.iter().any(|event| {
        event.event_type == "capability.downgraded"
            && event.payload["requested"] == "max"
            && event.payload["effective"] == "high"
    }));
    assert!(execution.events.iter().any(|event| {
        event.event_type == "assistant.text_chunk"
            && event.payload["content"] == "configured answer"
    }));
    assert!(
        execution
            .events
            .iter()
            .any(|event| event.event_type == "turn.completed")
    );
    assert_eq!(execution.usage.as_ref().unwrap()["outputTokens"], 3);
}

#[tokio::test]
async fn deepseek_adapter_initializes_without_codex_initialized_notification() {
    let session_id = Uuid::new_v4();
    let script = format!(
        r#"
read initialize
initialize_id=$(printf '%s' "$initialize" | sed -n 's/.*"id":\([0-9][0-9]*\).*/\1/p')
if ! printf '%s' "$initialize" | grep -q '"method":"initialize"' \
  || ! printf '%s' "$initialize" | grep -q '"cwd":"/workspace"' \
  || ! printf '%s' "$initialize" | grep -q '"provider":"openai-compatible"' \
  || ! printf '%s' "$initialize" | grep -q '"model":"gpt-5.6-luna"'; then
  exit 41
fi
printf '{{"jsonrpc":"2.0","id":%s,"result":{{"serverInfo":{{"name":"deepseek-harness-sdk-runtime","version":"test"}}}}}}\n' "$initialize_id"

read prompt
if printf '%s' "$prompt" | grep -q '"method":"initialized"' \
  || ! printf '%s' "$prompt" | grep -q '"method":"session/prompt"' \
  || ! printf '%s' "$prompt" | grep -q '"sessionId":"{session_id}"' \
  || ! printf '%s' "$prompt" | grep -q 'omnisolo_portable_session_context' \
  || ! printf '%s' "$prompt" | grep -q 'Inspect the repository'; then
  exit 42
fi
prompt_id=$(printf '%s' "$prompt" | sed -n 's/.*"id":\([0-9][0-9]*\).*/\1/p')
printf '{{"jsonrpc":"2.0","id":%s,"result":{{"messageId":"message-1"}}}}\n' "$prompt_id"
printf '%s\n' '{{"jsonrpc":"2.0","method":"session.event","params":{{"sessionId":"{session_id}","event":{{"type":"assistant/message","seq":1,"time":1,"data":{{"turn":1,"step":1,"message":{{"role":"assistant","content":[{{"type":"text","text":"done"}}]}}}}}}}}}}'
printf '%s\n' '{{"jsonrpc":"2.0","method":"session.status","params":{{"sessionId":"{session_id}","status":"idle"}}}}'
sleep 10
"#
    );
    let request = HarnessSessionRequest::new("tenant", session_id, Uuid::new_v4())
        .with_resolved_model(resolved_model_selection());
    let mut adapter = ProcessHarnessAdapter::new(
        ProcessHarnessSpec::command("/bin/sh", ["-c".to_owned(), script], "deepseek")
            .with_working_directory("/workspace"),
    );
    let native = adapter
        .import_session(request.clone(), test_capsule("deepseek", &request))
        .await
        .unwrap();
    let execution = adapter
        .execute(
            request,
            "attempt-1",
            "Inspect the repository",
            Some(&native.native_session_id),
        )
        .await
        .unwrap();
    assert_eq!(execution.final_text.as_deref(), Some("done"));
}

#[tokio::test]
async fn pi_adapter_configures_model_and_xhigh_before_streaming_a_prompt() {
    let script = r#"
while IFS= read -r line; do
  id=$(printf '%s' "$line" | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')
  type=$(printf '%s' "$line" | sed -n 's/.*"type":"\([^"]*\)".*/\1/p')
  case "$type" in
    set_model)
      if ! printf '%s' "$line" | grep -q '"provider":"omnisolo-openai-compatible"' \
        || ! printf '%s' "$line" | grep -q '"modelId":"gpt-5.6-luna"'; then exit 51; fi
      printf '%s\n' "{\"id\":\"$id\",\"type\":\"response\",\"command\":\"set_model\",\"success\":true}" ;;
    set_thinking_level)
      if ! printf '%s' "$line" | grep -q '"level":"xhigh"'; then exit 52; fi
      printf '%s\n' "{\"id\":\"$id\",\"type\":\"response\",\"command\":\"set_thinking_level\",\"success\":true}" ;;
    prompt)
      if ! printf '%s' "$line" | grep -q 'omnisolo_portable_session_context' \
        || ! printf '%s' "$line" | grep -q 'omnisolo.portable_session_context.v1' \
        || ! printf '%s' "$line" | grep -q 'Answer through Pi'; then exit 54; fi
      printf '%s\n' "{\"id\":\"$id\",\"type\":\"response\",\"command\":\"prompt\",\"success\":true}"
      printf '%s\n' '{"type":"agent_start"}'
      printf '%s\n' '{"type":"message_update","message":{"role":"assistant","content":[]},"assistantMessageEvent":{"type":"text_delta","contentIndex":0,"delta":"pi answer","partial":{}}}'
      printf '%s\n' '{"type":"message_end","message":{"role":"assistant","content":[{"type":"text","text":"pi answer"}],"usage":{"input":2,"output":3,"totalTokens":5},"stopReason":"stop"}}'
      printf '%s\n' '{"type":"agent_end","messages":[{"role":"assistant","content":[],"stopReason":"stop"}]}' ;;
    *) exit 53 ;;
  esac
done
"#;
    let request = HarnessSessionRequest::new("tenant", Uuid::new_v4(), Uuid::new_v4())
        .with_task(Uuid::new_v4(), "Pi native task")
        .with_resolved_model(resolved_model_selection());
    let mut adapter = ProcessHarnessAdapter::new(
        ProcessHarnessSpec::command("/bin/sh", ["-c", script], "pi").with_model_routing(
            resolved_model_selection(),
            Some("https://llmapi.omnisolo.co/v1".to_owned()),
        ),
    );
    let native = adapter
        .import_session(request.clone(), test_capsule("pi", &request))
        .await
        .unwrap();
    let execution = adapter
        .execute(
            request,
            "pi-attempt-1",
            "Answer through Pi",
            Some(&native.native_session_id),
        )
        .await
        .unwrap();
    assert_eq!(execution.final_text.as_deref(), Some("pi answer"));
    assert_eq!(execution.usage.as_ref().unwrap()["totalTokens"], 5);
    let binding = execution
        .events
        .first()
        .expect("model binding provenance precedes native Pi events");
    assert_eq!(binding.event_type, "inference.model_binding");
    assert!(binding.durable);
    assert_eq!(binding.payload["harness_id"], "pi");
    assert_eq!(binding.payload["provider_route"], "openai-compatible");
    assert_eq!(binding.payload["model_id"], "gpt-5.6-luna");
    assert_eq!(binding.payload["reasoning_effort"], "max");
    assert_eq!(binding.payload["api_dialect"], "open_ai_responses");
    assert_eq!(binding.payload["binding_revision"], "binding-v1");
    assert_eq!(binding.payload["binding_digest"], "sha256:test");
    assert!(execution.events.iter().any(|event| {
        event.event_type == "assistant.text_chunk" && event.payload["content"] == "pi answer"
    }));
    assert!(
        execution
            .events
            .iter()
            .any(|event| event.event_type == "turn.completed")
    );
}

#[tokio::test]
async fn native_codex_adapter_streams_and_resolves_server_requests() {
    let script = r#"
while IFS= read -r line; do
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
      printf '%s\n' '{"jsonrpc":"2.0","method":"item/agentMessage/delta","params":{"threadId":"thread-1","turnId":"turn-1","delta":"native answer"}}'
      printf '%s\n' '{"jsonrpc":"2.0","method":"item/completed","params":{"threadId":"thread-1","turnId":"turn-1","item":{"type":"agentMessage","text":"native answer"}}}'
      printf '%s\n' '{"jsonrpc":"2.0","method":"thread/tokenUsage/updated","params":{"threadId":"thread-1","turnId":"turn-1","tokenUsage":{"inputTokens":2,"outputTokens":3}}}'
      printf '%s\n' '{"jsonrpc":"2.0","method":"turn/completed","params":{"threadId":"thread-1","turnId":"turn-1"}}' ;;
    thread/archive|thread/delete|turn/interrupt)
      printf '{"jsonrpc":"2.0","id":%s,"result":{"thread":{"id":"thread-1"}}}\n' "$id" ;;
    *)
      printf '{"jsonrpc":"2.0","id":%s,"result":{}}\n' "$id" ;;
  esac
done
"#;
    let mut adapter = ProcessHarnessAdapter::new(
        ProcessHarnessSpec::command("/bin/sh", ["-c", script], "codex")
            .with_protocol(HarnessProtocolKind::CodexAppServer),
    );
    let request = HarnessSessionRequest::new("tenant-native", Uuid::new_v4(), Uuid::new_v4())
        .with_task(Uuid::new_v4(), "native objective");
    let native = adapter.create_session(request.clone()).await.unwrap();

    let mut stream = adapter
        .attempt_stream(
            AttemptOperation::Execute,
            request.clone(),
            "attempt-native",
            "continue",
            Some(&native.native_session_id),
        )
        .await
        .unwrap();
    let first = tokio::time::timeout(std::time::Duration::from_secs(2), stream.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    let interaction = match first {
        HarnessExecutionItem::Event(event) => event,
        other => panic!("expected approval event, got {other:?}"),
    };
    assert_eq!(interaction.event_type, "interaction.required");

    let response = adapter
        .exchange(
            request.clone(),
            "approval.requested",
            serde_json::json!({
                "native_request_id": 9001,
                "result": {"decision": "accept"}
            }),
        )
        .await
        .unwrap();
    assert_eq!(response["accepted"], true);

    let mut saw_delta = false;
    let mut saw_completion = false;
    let mut saw_completed = false;
    while let Some(item) = tokio::time::timeout(std::time::Duration::from_secs(2), stream.next())
        .await
        .unwrap()
    {
        match item.unwrap() {
            HarnessExecutionItem::Event(event) if event.event_type == "assistant.text_chunk" => {
                saw_delta = true;
            }
            HarnessExecutionItem::Event(event) if event.event_type == "assistant.final" => {
                saw_completion = true;
            }
            HarnessExecutionItem::Completed { final_text, usage } => {
                assert_eq!(final_text.as_deref(), Some("native answer"));
                assert_eq!(usage.unwrap()["outputTokens"], 3);
                saw_completed = true;
                break;
            }
            _ => {}
        }
    }
    assert!(saw_delta && saw_completion && saw_completed);
    adapter.close_session(request).await.unwrap();
}

#[tokio::test]
async fn native_codex_adapter_runs_the_complete_session_task_lifecycle() {
    let mut adapter = ProcessHarnessAdapter::new(
        ProcessHarnessSpec::command("/bin/sh", ["-c", native_lifecycle_script()], "codex")
            .with_protocol(HarnessProtocolKind::CodexAppServer),
    );
    let request = HarnessSessionRequest::new("tenant-lifecycle", Uuid::new_v4(), Uuid::new_v4())
        .with_task(Uuid::new_v4(), "run the complete native lifecycle");

    let created = adapter.create_session(request.clone()).await.unwrap();
    assert_eq!(created.native_session_id, "thread-1");

    let mut imported_request = request.clone();
    imported_request.extensions.insert(
        "native_session_id".to_owned(),
        serde_json::Value::String("thread-2".to_owned()),
    );
    let imported = adapter
        .import_session(
            imported_request.clone(),
            test_capsule("codex", &imported_request),
        )
        .await
        .unwrap();
    assert_eq!(imported.native_session_id, "thread-2");

    let execution = adapter
        .execute(
            imported_request.clone(),
            "attempt-execute",
            "continue imported work",
            Some(&imported.native_session_id),
        )
        .await
        .unwrap();
    assert_eq!(
        execution.final_text.as_deref(),
        Some("native lifecycle answer")
    );

    let steered = adapter
        .control_attempt(
            imported_request.clone(),
            "attempt-steer",
            "steer",
            "use the updated direction",
            Some(&imported.native_session_id),
        )
        .await
        .unwrap();
    assert_eq!(
        steered.final_text.as_deref(),
        Some("native lifecycle answer")
    );

    let reconciled = adapter
        .control_attempt(
            imported_request.clone(),
            "attempt-reconcile",
            "reconcile",
            "",
            Some(&imported.native_session_id),
        )
        .await
        .unwrap();
    assert!(reconciled.events.is_empty());

    let snapshot = adapter
        .control_session(
            imported_request.clone(),
            "snapshot",
            serde_json::json!({"native_session_id": imported.native_session_id}),
        )
        .await
        .unwrap();
    assert_eq!(snapshot.native_session_id, "thread-2");

    let resumed = adapter
        .resume_session(request.clone(), &created.native_session_id)
        .await
        .unwrap();
    assert_eq!(resumed.native_session_id, "thread-1");
    let forked = adapter
        .fork_session(request.clone(), Some(&created.native_session_id))
        .await
        .unwrap();
    assert_eq!(forked.native_session_id, "thread-fork");

    adapter
        .control_attempt(
            imported_request.clone(),
            "attempt-cancel",
            "cancel",
            "",
            Some("thread-2"),
        )
        .await
        .unwrap();
    adapter
        .control_attempt(
            imported_request.clone(),
            "attempt-quiesce",
            "quiesce",
            "",
            Some("thread-2"),
        )
        .await
        .unwrap();

    adapter
        .close_session(imported_request.clone())
        .await
        .unwrap();
    adapter.delete_session(imported_request).await.unwrap();

    let recreated = adapter.create_session(request.clone()).await.unwrap();
    assert_eq!(recreated.native_session_id, "thread-3");
    let mut recreated_request = request;
    recreated_request.extensions.insert(
        "native_session_id".to_owned(),
        serde_json::Value::String(recreated.native_session_id.clone()),
    );
    adapter.delete_session(recreated_request).await.unwrap();
}

#[tokio::test]
async fn process_adapter_runs_session_task_and_resume_operations() {
    let script = r#"
while IFS= read -r line; do
  request_id=$(printf '%s' "$line" | sed -n 's/.*"request_id":"\([^"]*\)".*/\1/p')
  operation=$(printf '%s' "$line" | sed -n 's/.*"operation":"\([^"]*\)".*/\1/p')
  case "$operation" in
    create_session) payload='{"native_session_id":"native-1","native_cursor":"cursor-1"}' ;;
    fork_session) payload='{"native_session_id":"native-fork","native_cursor":"cursor-fork"}' ;;
    import_session) payload='{"native_session_id":"native-imported","native_cursor":"cursor-imported"}' ;;
    resume_session) payload='{"native_session_id":"native-1","native_cursor":"cursor-2"}' ;;
    start|execute|resume) payload='{"events":[{"event_type":"assistant.text","durable":true,"payload":{"text":"done"}}],"final_text":"done"}' ;;
    checkpoint) payload='{"checkpoint_ref":"checkpoint-1","native_cursor":"cursor-3"}' ;;
    close_session|delete_session) payload='{}' ;;
    *) payload='{}' ;;
  esac
  printf '{"request_id":"%s","ok":true,"payload":%s}\n' "$request_id" "$payload"
done
"#;
    let mut adapter = ProcessHarnessAdapter::new(
        ProcessHarnessSpec::command("/bin/sh", ["-c", script], "codex")
            .with_protocol(HarnessProtocolKind::Custom),
    );
    let request = HarnessSessionRequest::new("tenant-1", Uuid::new_v4(), Uuid::new_v4())
        .with_task(Uuid::new_v4(), "finish the task");

    let created = adapter.create_session(request.clone()).await.unwrap();
    assert_eq!(created.native_session_id, "native-1");

    let forked = adapter
        .fork_session(request.clone(), Some(&created.native_session_id))
        .await
        .unwrap();
    assert_eq!(forked.native_session_id, "native-fork");

    let imported = adapter
        .import_session(request.clone(), test_capsule("codex", &request))
        .await
        .unwrap();
    assert_eq!(imported.native_session_id, "native-imported");

    let resumed = adapter
        .resume_session(request.clone(), &created.native_session_id)
        .await
        .unwrap();
    assert_eq!(resumed.native_cursor.as_deref(), Some("cursor-2"));

    let execution = adapter
        .execute(request.clone(), "attempt-1", "do it", None)
        .await
        .unwrap();
    assert_eq!(execution.final_text.as_deref(), Some("done"));
    assert_eq!(execution.events.len(), 1);

    let started = adapter
        .start_attempt(request.clone(), "attempt-1", "start it", None)
        .await
        .unwrap();
    assert_eq!(started.final_text.as_deref(), Some("done"));
    let resumed_attempt = adapter
        .resume_attempt(request.clone(), "attempt-1", "resume it", None)
        .await
        .unwrap();
    assert_eq!(resumed_attempt.final_text.as_deref(), Some("done"));

    let checkpoint = adapter
        .checkpoint(request.clone(), "attempt-1")
        .await
        .unwrap();
    assert_eq!(checkpoint.checkpoint_ref, "checkpoint-1");
    adapter.delete_session(request.clone()).await.unwrap();
    adapter.close_session(request).await.unwrap();
}

#[tokio::test]
async fn process_adapter_forwards_protocol_and_transfer_context_to_the_worker() {
    let model_binding_id = Uuid::new_v4();
    let runtime_config_snapshot_id = Uuid::new_v4();
    let workspace_snapshot_id = Uuid::new_v4();
    let capability_snapshot_id = Uuid::new_v4();
    let artifact_id = Uuid::new_v4();
    let script = format!(
        r#"
while IFS= read -r line; do
  request_id=$(printf '%s' "$line" | sed -n 's/.*"request_id":"\([^"]*\)".*/\1/p')
  operation=$(printf '%s' "$line" | sed -n 's/.*"operation":"\([^"]*\)".*/\1/p')
  if [ "$operation" = "create_session" ] \
    && printf '%s' "$line" | grep -q '"protocol_kind":"custom"' \
    && printf '%s' "$line" | grep -q '"model_binding_id":"{model_binding_id}"' \
    && printf '%s' "$line" | grep -q '"runtime_config_snapshot_id":"{runtime_config_snapshot_id}"' \
    && printf '%s' "$line" | grep -q '"workspace_snapshot_id":"{workspace_snapshot_id}"' \
    && printf '%s' "$line" | grep -q '"capability_snapshot_id":"{capability_snapshot_id}"' \
    && printf '%s' "$line" | grep -q '"durable_sequence":42' \
    && printf '%s' "$line" | grep -q '"artifact_ids":\["{artifact_id}"\]' \
    && printf '%s' "$line" | grep -q '"resolved_model":{{' \
    && printf '%s' "$line" | grep -q '"provider_route":"openai-compatible"' \
    && printf '%s' "$line" | grep -q '"model_id":"gpt-5.6-luna"' \
    && printf '%s' "$line" | grep -q '"reasoning_effort":"max"' \
    && printf '%s' "$line" | grep -q '"api_dialect":"open_ai_responses"' \
    && printf '%s' "$line" | grep -q '"capabilities":\["reasoning","tools"\]' \
    && printf '%s' "$line" | grep -q '"binding_revision":"binding-v1"' \
    && printf '%s' "$line" | grep -q '"binding_digest":"sha256:test"' \
    && ! printf '%s' "$line" | grep -Eq '"(api_key|access_token|credential|credential_ref|secret)"[[:space:]]*:' \
    && printf '%s' "$line" | grep -q '"region":"portable"' \
    && printf '%s' "$line" | grep -q '"trace":"adapter-test"'; then
    printf '{{"request_id":"%s","ok":true,"payload":{{"native_session_id":"native-context","native_cursor":"cursor-context"}}}}\n' "$request_id"
  else
    printf '{{"request_id":"%s","ok":false,"error":"context was not forwarded"}}\n' "$request_id"
  fi
done
"#,
        model_binding_id = model_binding_id,
        runtime_config_snapshot_id = runtime_config_snapshot_id,
        workspace_snapshot_id = workspace_snapshot_id,
        capability_snapshot_id = capability_snapshot_id,
        artifact_id = artifact_id,
    );
    let mut adapter = ProcessHarnessAdapter::new(
        ProcessHarnessSpec::command("/bin/sh", ["-c".to_owned(), script], "codex")
            .with_protocol(HarnessProtocolKind::Custom),
    );
    let mut request = HarnessSessionRequest::new("tenant-context", Uuid::new_v4(), Uuid::new_v4())
        .with_task(Uuid::new_v4(), "transfer context")
        .with_resolved_model(resolved_model_selection())
        .with_transfer_pointers(
            Some(model_binding_id),
            Some(runtime_config_snapshot_id),
            Some(workspace_snapshot_id),
            vec![artifact_id],
            Some(42),
        );
    request.capability_snapshot_id = Some(capability_snapshot_id);
    request
        .metadata
        .insert("region".to_owned(), "portable".to_owned());
    request
        .extensions
        .insert("trace".to_owned(), serde_json::json!("adapter-test"));

    let request_json = serde_json::to_value(&request).unwrap();
    assert_eq!(
        request_json["resolved_model"]["model_id"],
        json!("gpt-5.6-luna")
    );
    assert!(!contains_credential_key(&request_json));

    let mut legacy_json = request_json;
    legacy_json
        .as_object_mut()
        .unwrap()
        .remove("resolved_model");
    let legacy_request: HarnessSessionRequest = serde_json::from_value(legacy_json).unwrap();
    assert!(legacy_request.resolved_model.is_none());

    let native = adapter.create_session(request).await.unwrap();
    assert_eq!(native.native_session_id, "native-context");
}

#[tokio::test]
async fn resolved_model_metadata_is_redacted_before_request_and_process_serialization() {
    const CANARY: &str = "resolved-model-secret-canary";
    let script = format!(
        r#"
while IFS= read -r line; do
  request_id=$(printf '%s' "$line" | sed -n 's/.*"request_id":"\([^"]*\)".*/\1/p')
  operation=$(printf '%s' "$line" | sed -n 's/.*"operation":"\([^"]*\)".*/\1/p')
  if [ "$operation" = "create_session" ] \
    && printf '%s' "$line" | grep -q '"routing_tier":"balanced"' \
    && printf '%s' "$line" | grep -q '"max_output_token":"max_output_token=128000"' \
    && ! printf '%s' "$line" | grep -q '{CANARY}' \
    && ! printf '%s' "$line" | grep -Eq '"(api_key|token|secret|password|authorization|credential)"[[:space:]]*:'; then
    printf '{{"request_id":"%s","ok":true,"payload":{{"native_session_id":"native-redacted","native_cursor":null}}}}\n' "$request_id"
  else
    printf '{{"request_id":"%s","ok":false,"error":"resolved model metadata leaked"}}\n' "$request_id"
  fi
done
"#,
    );
    let mut adapter = ProcessHarnessAdapter::new(
        ProcessHarnessSpec::command("/bin/sh", ["-c".to_owned(), script], "codex")
            .with_protocol(HarnessProtocolKind::Custom),
    );
    let mut selection = resolved_model_selection();
    selection.metadata.insert(
        "nested".to_owned(),
        json!({
            "api_key": CANARY,
            "openaiApiKey": CANARY,
            "accessToken": CANARY,
            "credentialRef": CANARY,
            "clientSecret": CANARY,
            "children": [
                {"token": CANARY},
                {"secret": CANARY},
                {"password": CANARY},
                {"authorization": format!("Bearer {CANARY}")},
                {"credential": CANARY},
                {"ordinary": "preserved", "note": format!("token={CANARY}")}
            ]
        }),
    );
    selection.metadata.insert(
        "max_output_token".to_owned(),
        json!("max_output_token=128000"),
    );
    let request = HarnessSessionRequest::new("tenant-redaction", Uuid::new_v4(), Uuid::new_v4())
        .with_resolved_model(selection);

    let request_json = serde_json::to_value(&request).unwrap();
    assert!(!contains_credential_key(&request_json));
    let encoded = serde_json::to_string(&request_json).unwrap();
    assert!(!encoded.contains(CANARY));
    assert_eq!(
        request_json["resolved_model"]["metadata"]["routing_tier"],
        json!("balanced")
    );
    assert_eq!(
        request_json["resolved_model"]["metadata"]["nested"]["children"][5]["ordinary"],
        json!("preserved")
    );
    assert_eq!(
        request_json["resolved_model"]["metadata"]["max_output_token"],
        json!("max_output_token=128000")
    );

    let mut inbound_selection = serde_json::to_value(resolved_model_selection()).unwrap();
    inbound_selection["metadata"] = json!({
        "ordinary": "preserved",
        "nested": [{"credential": CANARY}, {"note": format!("password={CANARY}")}]
    });
    let decoded: ResolvedModelSelection = serde_json::from_value(inbound_selection).unwrap();
    let decoded_metadata = Value::Object(decoded.metadata.into_iter().collect());
    assert!(!contains_credential_key(&decoded_metadata));
    assert!(
        !serde_json::to_string(&decoded_metadata)
            .unwrap()
            .contains(CANARY)
    );
    assert_eq!(decoded_metadata["ordinary"], json!("preserved"));

    let native = adapter.create_session(request).await.unwrap();
    assert_eq!(native.native_session_id, "native-redacted");
}

#[test]
fn portable_capsule_retains_resolved_model_without_credentials() {
    let session_id = Uuid::new_v4();
    let now = Utc::now();
    let session = Session {
        session_id,
        tenant_id: "tenant-portable-model".to_owned(),
        project_id: None,
        workspace_id: None,
        title: None,
        labels: BTreeSet::new(),
        tags: Default::default(),
        state: SessionState::Open,
        state_version: 1,
        parent_session_id: None,
        root_session_id: session_id,
        fork_source_event_id: None,
        active_task_id: None,
        created_at: now,
        updated_at: now,
        last_active_at: Some(now),
        retention_class: None,
        data_classification: None,
        extensions: Default::default(),
    };
    let mut selection = resolved_model_selection();
    selection
        .metadata
        .insert("api_key".to_owned(), json!("secret-canary"));
    let runtime_config = RuntimeConfigSnapshot {
        snapshot_id: Uuid::new_v4(),
        tenant_id: session.tenant_id.clone(),
        snapshot_digest: "sha256:runtime-config".to_owned(),
        resolved_model: Some(selection),
        created_at: now,
        ..Default::default()
    };

    let capsule = CapsuleCompiler::new("omnisolo", "redaction-v1", 1)
        .compile(CapsuleCompileInput {
            capsule_id: Uuid::new_v4(),
            handoff_id: Uuid::new_v4(),
            session,
            source_binding_id: None,
            target_harness_id: "codex".to_owned(),
            from_durable_sequence: 0,
            to_durable_sequence: 0,
            branch_id: None,
            head_event_id: None,
            event_ancestor_ids: Vec::new(),
            records: vec![CanonicalRecord::RuntimeConfigSnapshot(runtime_config)],
            workspace_snapshot_digests: Vec::new(),
            artifact_digests: Vec::new(),
            local_service_bindings: Vec::new(),
            created_at: now,
        })
        .unwrap();

    let portable_selection = capsule
        .records
        .iter()
        .find_map(|record| match record {
            PortableRecord::RuntimeConfigSnapshot(config) => config.resolved_model.as_ref(),
            _ => None,
        })
        .expect("portable resolved model selection");
    assert_eq!(portable_selection.model_id, "gpt-5.6-luna");
    assert_eq!(
        portable_selection.reasoning_effort,
        Some(ReasoningEffort::Max)
    );
    assert_eq!(
        portable_selection.api_dialect,
        ModelApiDialect::OpenAiResponses
    );
    assert!(!portable_selection.metadata.contains_key("api_key"));

    let portable_records = serde_json::to_value(&capsule.records).unwrap();
    assert!(!contains_credential_key(&portable_records));
    let encoded = serde_json::to_string(&capsule).unwrap();
    assert!(!encoded.contains("secret-canary"));
    assert!(!encoded.contains("credential_ref"));
}

#[test]
fn portable_capsule_preserves_scoped_local_service_references_without_authority() {
    let session_id = Uuid::from_u128(200);
    let task_id = Uuid::from_u128(201);
    let attempt_id = Uuid::from_u128(202);
    let now = Utc::now();
    let session = Session {
        session_id,
        tenant_id: "tenant-shared-services".to_owned(),
        project_id: Some("project-shared".to_owned()),
        workspace_id: Some("workspace-shared".to_owned()),
        title: None,
        labels: BTreeSet::new(),
        tags: Default::default(),
        state: SessionState::Open,
        state_version: 1,
        parent_session_id: None,
        root_session_id: session_id,
        fork_source_event_id: None,
        active_task_id: Some(task_id),
        created_at: now,
        updated_at: now,
        last_active_at: Some(now),
        retention_class: None,
        data_classification: None,
        extensions: Default::default(),
    };
    let service_context = LocalServiceScopeContext::for_attempt(
        &session.tenant_id,
        session.project_id.as_deref(),
        session.workspace_id.as_deref(),
        session_id,
        Some(task_id),
        Some(attempt_id),
    );
    let bundle = LocalServiceRegistry::with_defaults()
        .resolve(service_context)
        .unwrap();

    let capsule = CapsuleCompiler::new("omnisolo", "redaction-v1", 1)
        .compile(CapsuleCompileInput {
            capsule_id: Uuid::from_u128(203),
            handoff_id: Uuid::from_u128(204),
            session,
            source_binding_id: None,
            target_harness_id: "aider".to_owned(),
            from_durable_sequence: 0,
            to_durable_sequence: 0,
            branch_id: None,
            head_event_id: None,
            event_ancestor_ids: Vec::new(),
            records: Vec::new(),
            workspace_snapshot_digests: Vec::new(),
            artifact_digests: Vec::new(),
            local_service_bindings: bundle.bindings.clone(),
            created_at: now,
        })
        .unwrap();

    assert_eq!(capsule.manifest.local_service_bindings, bundle.bindings);
    assert_eq!(
        capsule
            .manifest
            .local_service_bindings
            .iter()
            .find(|binding| binding.kind == LocalServiceKind::Memory)
            .unwrap()
            .generation,
        1
    );
    let encoded = serde_json::to_string(&capsule)
        .unwrap()
        .to_ascii_lowercase();
    for forbidden in [
        "authorization",
        "facade_token",
        "browser_cookie",
        "upstream_key",
    ] {
        assert!(
            !encoded.contains(forbidden),
            "portable capsule contains {forbidden}"
        );
    }
}

#[tokio::test]
async fn omnisolo_bridge_imports_capsule_context_and_executes_through_the_same_contract() {
    let descriptor: HarnessDescriptor = HarnessRegistry::with_defaults()
        .descriptor("omnisolo")
        .unwrap()
        .clone();
    let session_id = Uuid::new_v4();
    let task_id = Uuid::new_v4();
    let request = HarnessSessionRequest::new("tenant-bridge", session_id, Uuid::new_v4())
        .with_task(task_id, "");
    let capsule = test_capsule("omnisolo", &request);
    let expected_digest = capsule.record_digest.clone();
    let expected_record_count = capsule.records.len();
    let mut bridge = OmniSoloHarnessAdapterBridge::new(descriptor);

    let native = bridge
        .import_session(request.clone(), capsule)
        .await
        .unwrap();
    assert_eq!(native.native_session_id, format!("omnisolo:{session_id}"));
    let imported = bridge.imported_capsule().expect("capsule retained");
    assert_eq!(imported.record_digest, expected_digest);
    assert_eq!(imported.records.len(), expected_record_count);
    assert_eq!(imported.manifest.session_id, session_id);

    let execution = bridge
        .execute(request, "attempt-bridge", "continue imported work", None)
        .await
        .unwrap();
    assert_eq!(execution.events.len(), 1);
    assert_eq!(
        execution.events[0].payload["content"],
        "continue imported work"
    );
}

#[test]
fn router_supports_session_and_task_selection_without_two_writable_bindings() {
    let registry = HarnessRegistry::with_defaults();
    let session_id = Uuid::new_v4();
    let task_id = Uuid::new_v4();
    let workspace = "workspace-1";
    let now = chrono::Utc::now();
    let mut router = HarnessRouter::new(registry);

    router
        .register_binding(binding(
            session_id,
            None,
            "omnisolo",
            BindingScope::Session,
            workspace,
        ))
        .unwrap();
    router
        .register_binding(binding(
            session_id,
            Some(task_id),
            "codex",
            BindingScope::Task,
            "task-workspace",
        ))
        .unwrap();

    assert_eq!(
        router
            .select_for_task(session_id, task_id)
            .unwrap()
            .harness_id,
        "codex"
    );
    assert_eq!(
        router.select_for_session(session_id).unwrap().harness_id,
        "omnisolo"
    );

    let duplicate = binding(
        session_id,
        None,
        "opencode",
        BindingScope::Session,
        workspace,
    );
    assert_eq!(
        router.register_binding(duplicate),
        Err(RouterError::DuplicateWritableBinding)
    );

    let readonly = SessionBinding {
        access_mode: BindingAccessMode::ReadOnly,
        state: BindingState::Active,
        created_at: now,
        ..binding(
            session_id,
            Some(Uuid::new_v4()),
            "opencode",
            BindingScope::Task,
            workspace,
        )
    };
    router.register_binding(readonly).unwrap();
    assert_eq!(router.active_harnesses(session_id).len(), 3);
}

#[test]
fn router_rejects_duplicate_writable_bindings_across_owners() {
    let registry = HarnessRegistry::with_defaults();
    let session_id = Uuid::new_v4();
    let workspace = "shared-workspace";
    let mut router = HarnessRouter::new(registry);
    router
        .register_binding(binding(
            session_id,
            None,
            "omnisolo",
            BindingScope::Session,
            workspace,
        ))
        .unwrap();

    let mut duplicate = binding(session_id, None, "codex", BindingScope::Session, workspace);
    duplicate.owner_id = Uuid::new_v4();
    assert_eq!(
        router.register_binding(duplicate),
        Err(RouterError::DuplicateWritableBinding)
    );
}

#[test]
fn router_rejects_invalid_and_unknown_bindings_and_uses_session_fallbacks() {
    let session_id = Uuid::new_v4();
    let task_id = Uuid::new_v4();
    let mut router = HarnessRouter::new(HarnessRegistry::with_defaults());
    assert!(matches!(
        router.descriptor("missing"),
        Err(RouterError::UnknownHarness(_))
    ));
    assert!(matches!(
        router.binding(Uuid::new_v4()),
        Err(RouterError::BindingNotFound(_))
    ));
    assert_eq!(
        router.select_for_session(session_id),
        Err(RouterError::SessionBindingNotFound(session_id))
    );
    assert_eq!(
        router.select_for_task(session_id, task_id),
        Err(RouterError::TaskBindingNotFound {
            session_id,
            task_id
        })
    );

    let mut invalid = binding(
        session_id,
        None,
        "omnisolo",
        BindingScope::Session,
        "workspace",
    );
    invalid.binding_id = Uuid::nil();
    assert_eq!(
        router.register_binding(invalid),
        Err(RouterError::InvalidBinding)
    );

    let mut invalid_scope = binding(
        session_id,
        Some(task_id),
        "omnisolo",
        BindingScope::Session,
        "workspace",
    );
    assert_eq!(
        router.register_binding(invalid_scope.clone()),
        Err(RouterError::InvalidBinding)
    );
    invalid_scope.task_id = None;
    invalid_scope.scope = BindingScope::Task;
    assert_eq!(
        router.register_binding(invalid_scope),
        Err(RouterError::InvalidBinding)
    );

    let session_binding = binding(
        session_id,
        None,
        "omnisolo",
        BindingScope::Session,
        "workspace",
    );
    let duplicate_id = session_binding.binding_id;
    router.register_binding(session_binding).unwrap();
    let duplicate = SessionBinding {
        binding_id: duplicate_id,
        ..binding(
            session_id,
            None,
            "codex",
            BindingScope::Session,
            "other-workspace",
        )
    };
    assert_eq!(
        router.register_binding(duplicate),
        Err(RouterError::DuplicateBinding(duplicate_id))
    );
    assert_eq!(
        router
            .select_for_task(session_id, Uuid::new_v4())
            .unwrap()
            .harness_id,
        "omnisolo"
    );

    let mut readonly_router = HarnessRouter::new(HarnessRegistry::with_defaults());
    let readonly = SessionBinding {
        access_mode: BindingAccessMode::ReadOnly,
        ..binding(
            session_id,
            None,
            "codex",
            BindingScope::Session,
            "readonly-workspace",
        )
    };
    let readonly_id = readonly.binding_id;
    readonly_router.register_binding(readonly).unwrap();
    assert_eq!(
        readonly_router.binding(readonly_id).unwrap().access_mode,
        BindingAccessMode::ReadOnly
    );
    assert_eq!(
        readonly_router
            .select_for_session(session_id)
            .unwrap()
            .binding_id,
        readonly_id
    );
}

fn native_lifecycle_script() -> &'static str {
    r#"
thread_count=0
turn_count=0
while IFS= read -r line; do
  if printf '%s' "$line" | grep -q '"jsonrpc"'; then exit 24; fi
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
      printf '%s' "$line" | grep -q '"source":"omnisolo.historical"' || exit 25
      printf '{"id":%s,"result":{}}\n' "$id" ;;
    thread/resume|thread/read)
      printf '{"id":%s,"result":{"thread":{"id":"%s"}}}\n' "$id" "$thread_id" ;;
    thread/fork)
      printf '{"id":%s,"result":{"thread":{"id":"thread-fork"}}}\n' "$id" ;;
    turn/start|turn/steer)
      turn_count=$((turn_count + 1)); turn_id="turn-$turn_count"
      printf '{"id":%s,"result":{"turn":{"id":"%s","threadId":"%s"}}}\n' "$id" "$turn_id" "$thread_id"
      printf '{"method":"turn/started","params":{"turn":{"id":"%s","threadId":"%s"}}}\n' "$turn_id" "$thread_id"
      printf '{"method":"item/agentMessage/delta","params":{"threadId":"%s","turnId":"%s","delta":"native lifecycle answer"}}\n' "$thread_id" "$turn_id"
      printf '{"method":"item/completed","params":{"threadId":"%s","turnId":"%s","item":{"type":"agentMessage","text":"native lifecycle answer"}}}\n' "$thread_id" "$turn_id"
      printf '{"method":"turn/completed","params":{"turn":{"id":"%s","threadId":"%s","status":"completed"}}}\n' "$turn_id" "$thread_id" ;;
    turn/interrupt) printf '{"id":%s,"result":{}}\n' "$id" ;;
    thread/archive|thread/delete)
      if printf '%s' "$line" | grep -qE '"approvalPolicy"|"sandbox"|"sandboxPolicy"'; then exit 26; fi
      printf '{"id":%s,"result":{}}\n' "$id" ;;
    *) printf '{"id":%s,"result":{}}\n' "$id" ;;
  esac
done
"#
}

fn binding(
    session_id: Uuid,
    task_id: Option<Uuid>,
    harness_id: &str,
    scope: BindingScope,
    workspace: &str,
) -> SessionBinding {
    SessionBinding {
        binding_id: Uuid::new_v4(),
        session_id,
        task_id,
        harness_id: harness_id.to_owned(),
        scope: scope.clone(),
        owner_id: task_id.unwrap_or(session_id),
        workspace_mutation_scope_id: workspace.to_owned(),
        access_mode: BindingAccessMode::ReadWrite,
        native_session_id: None,
        state: BindingState::Active,
        generation: 1,
        created_at: chrono::Utc::now(),
        last_used_at: None,
        capability_snapshot_id: None,
        adapter_config_digest: None,
        last_imported_native_cursor: None,
        last_exported_durable_sequence: None,
        native_checkpoint_ref: None,
        worker_pool: None,
        state_locality: None,
        exact_resume_eligible: false,
        invalidation_reason: None,
        native_record_digest: None,
        extensions: Default::default(),
    }
}

fn test_capsule(
    target_harness_id: &str,
    request: &HarnessSessionRequest,
) -> server_harness::middleware::capsule::SessionCapsule {
    let mut config = server_harness::middleware::adapter::OmniSoloRunConfig::new(
        &request.tenant_id,
        "portable task",
    )
    .with_session_id(request.session_id);
    if let Some(task_id) = request.task_id {
        config = config.with_task_id(task_id);
    }
    let adapter =
        server_harness::middleware::adapter::OmniSoloHarnessAdapter::start(config.with_turn())
            .unwrap();
    adapter
        .export_capsule(target_harness_id, Uuid::new_v4())
        .unwrap()
}
