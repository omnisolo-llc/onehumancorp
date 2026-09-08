use std::sync::Arc;

use serde_json::{Value, json};
use server_harness::middleware::capsule::SessionCapsule;
use server_harness::middleware::harness::{
    HarnessAdapterError, HarnessEvent, HarnessSessionRequest, NativeSession,
};
use server_harness::middleware::json_rpc::{
    JsonRpcErrorObject, JsonRpcNotification, JsonRpcProcessConfig, JsonRpcProcessRuntime,
    JsonRpcServerRequest,
};
use server_harness::middleware::protocol::{
    AttemptOperation, HarnessProtocolCodec, JsonRpcRequestSpec, NativeTurnState, ProtocolEvent,
    ProtocolProcessAdapter, ServerResponse, SessionOperation,
};
use tokio::sync::Mutex;

#[derive(Clone, Debug, Default)]
struct TestCodec;

impl HarnessProtocolCodec for TestCodec {
    fn initialize_request(&self) -> JsonRpcRequestSpec {
        JsonRpcRequestSpec {
            method: "initialize".to_owned(),
            params: Value::Null,
        }
    }

    fn session_request(
        &self,
        _operation: SessionOperation,
        _request: &HarnessSessionRequest,
    ) -> Result<JsonRpcRequestSpec, HarnessAdapterError> {
        Ok(JsonRpcRequestSpec {
            method: "session".to_owned(),
            params: Value::Null,
        })
    }

    fn attempt_request(
        &self,
        _operation: AttemptOperation,
        _request: &HarnessSessionRequest,
        _prompt: &str,
        _native_session_id: Option<&str>,
        _native_turn_id: Option<&str>,
    ) -> Result<JsonRpcRequestSpec, HarnessAdapterError> {
        Ok(JsonRpcRequestSpec {
            method: "attempt".to_owned(),
            params: Value::Null,
        })
    }

    fn decode_session_result(
        &self,
        _operation: SessionOperation,
        _result: Value,
    ) -> Result<NativeSession, HarnessAdapterError> {
        Ok(NativeSession {
            native_session_id: "native".to_owned(),
            native_cursor: None,
        })
    }

    fn decode_notification(
        &self,
        _notification: &JsonRpcNotification,
        _state: &mut NativeTurnState,
    ) -> Result<ProtocolEvent, HarnessAdapterError> {
        Ok(ProtocolEvent {
            event: HarnessEvent {
                event_type: "test".to_owned(),
                durable: false,
                payload: Value::Null,
                native_cursor: None,
            },
            native_cursor: None,
            final_text: None,
            usage: None,
            terminal: false,
        })
    }

    fn server_request_event(
        &self,
        _request: &JsonRpcServerRequest,
    ) -> Result<HarnessEvent, HarnessAdapterError> {
        Ok(HarnessEvent {
            event_type: "interaction.required".to_owned(),
            durable: true,
            payload: Value::Null,
            native_cursor: None,
        })
    }

    fn encode_server_response(
        &self,
        _method: &str,
        response: &Value,
    ) -> Result<ServerResponse, HarnessAdapterError> {
        if response.get("error").is_some() {
            return Ok(ServerResponse::Error(JsonRpcErrorObject {
                code: -32000,
                message: "test error".to_owned(),
                data: response.get("error").cloned(),
            }));
        }
        Ok(ServerResponse::Result(
            response.get("result").cloned().unwrap_or(Value::Null),
        ))
    }
}

fn adapter(runtime: JsonRpcProcessRuntime) -> ProtocolProcessAdapter {
    ProtocolProcessAdapter {
        runtime,
        codec: Arc::new(TestCodec),
        state: Arc::new(Mutex::new(NativeTurnState::new("thread-1"))),
        states: Arc::new(Mutex::new(Default::default())),
    }
}

#[test]
fn protocol_codec_default_import_and_state_helpers_are_explicit() {
    let codec = TestCodec;
    assert_eq!(codec.initialize_request().method, "initialize");
    assert!(matches!(
        codec.import_items(&dummy_capsule()),
        Err(HarnessAdapterError::InvalidRequest(message)) if message.contains("not supported")
    ));
    assert_eq!(NativeTurnState::new("thread-1").thread_id, "thread-1");
}

#[tokio::test]
async fn protocol_exchange_validates_payloads_and_responds_with_result_or_error() {
    let runtime = JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell(
        r#"
printf '%s\n' '{"id":7,"method":"approval","params":{}}'
read first
printf '%s\n' '{"id":8,"method":"approval","params":{}}'
read second
sleep 1
"#,
    ))
    .await
    .unwrap();
    let mut requests = runtime.subscribe_server_requests();
    let adapter = adapter(runtime.clone());

    let first = tokio::time::timeout(std::time::Duration::from_secs(2), requests.recv())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(first.id.as_value(), json!(7));

    let accepted = adapter
        .exchange(
            "approval",
            &json!({"native_request_id":7,"result":{"decision":"accept"}}),
        )
        .await
        .unwrap();
    assert_eq!(accepted["accepted"], true);

    let second = tokio::time::timeout(std::time::Duration::from_secs(2), requests.recv())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(second.id.as_value(), json!(8));
    let rejected = adapter
        .exchange(
            "approval",
            &json!({"native_request_id":8,"error":{"reason":"deny"}}),
        )
        .await
        .unwrap();
    assert_eq!(rejected["native_request_id"], 8);

    for payload in [
        json!({"result":{}}),
        json!({"native_request_id":7}),
        json!({"native_request_id":null,"result":{}}),
        json!({"native_request_id":7,"result":{},"error":{}}),
    ] {
        assert!(adapter.exchange("approval", &payload).await.is_err());
    }
    assert!(format!("{adapter:?}").contains("ProtocolProcessAdapter"));
    runtime.shutdown().await.unwrap();
}

fn dummy_capsule() -> SessionCapsule {
    serde_json::from_value(json!({
        "manifest": {
            "capsule_id": "00000000-0000-0000-0000-000000000001",
            "schema_version": 2,
            "minimum_reader_version": 2,
            "producer": "test",
            "compiler_version": 2,
            "created_at": "2026-01-01T00:00:00Z",
            "tenant_id": "tenant",
            "session_id": "00000000-0000-0000-0000-000000000002",
            "project_id": null,
            "workspace_id": null,
            "title": null,
            "labels": [],
            "tags": {},
            "session_state": "open",
            "session_state_version": 0,
            "session_created_at": "2026-01-01T00:00:00Z",
            "session_updated_at": "2026-01-01T00:00:00Z",
            "last_active_at": null,
            "parent_session_id": null,
            "root_session_id": "00000000-0000-0000-0000-000000000002",
            "fork_source_event_id": null,
            "active_task_id": null,
            "retention_class": null,
            "data_classification": null,
            "source_binding_id": null,
            "target_harness_id": "test",
            "handoff_id": "00000000-0000-0000-0000-000000000003",
            "from_durable_sequence": 0,
            "to_durable_sequence": 0,
            "branch_id": null,
            "head_event_id": null,
            "event_ancestor_ids": [],
            "workspace_snapshot_digests": [],
            "artifact_digests": [],
            "redaction_policy_id": "test",
            "redaction_policy_version": 1
        },
        "records": [],
        "loss_report": {"entries": []},
        "manifest_digest": "",
        "record_digest": "",
        "loss_report_digest": "",
        "head_event_id": null,
        "event_ancestor_ids": []
    }))
    .unwrap()
}
