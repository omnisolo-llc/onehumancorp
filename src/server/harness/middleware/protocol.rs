use std::collections::BTreeMap;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::Value;
use tokio::sync::Mutex;

use super::capsule::SessionCapsule;
use super::harness::{HarnessAdapterError, HarnessEvent, HarnessSessionRequest, NativeSession};
use super::json_rpc::{
    JsonRpcErrorObject, JsonRpcId, JsonRpcNotification, JsonRpcProcessRuntime, JsonRpcResult,
    JsonRpcServerRequest,
};

#[derive(Clone, Debug)]
pub enum SessionOperation {
    Create,
    Import(SessionCapsule),
    Resume { native_session_id: String },
    Fork { native_session_id: Option<String> },
    Snapshot,
    Quiesce,
    Cancel,
    Close,
    Delete,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttemptOperation {
    Start,
    Execute,
    Resume,
    Steer,
    Cancel,
    Quiesce,
    Reconcile,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JsonRpcRequestSpec {
    pub method: String,
    pub params: Value,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JsonRpcRequestWithoutParamsSpec {
    pub method: String,
}

impl JsonRpcRequestWithoutParamsSpec {
    pub fn new(method: impl Into<String>) -> Self {
        Self {
            method: method.into(),
        }
    }

    pub async fn send(&self, runtime: &JsonRpcProcessRuntime) -> JsonRpcResult {
        runtime.request_without_params(&self.method).await
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum HarnessExecutionItem {
    Event(HarnessEvent),
    Completed {
        final_text: Option<String>,
        usage: Option<Value>,
    },
}

pub type HarnessExecutionStream = Pin<
    Box<dyn tokio_stream::Stream<Item = Result<HarnessExecutionItem, HarnessAdapterError>> + Send>,
>;

#[derive(Clone, Debug, PartialEq)]
pub struct NativeTurnState {
    pub thread_id: String,
    pub active_turn_id: Option<String>,
    pub native_cursor: Option<String>,
    pub notification_sequence: u64,
    pub pending_import: Option<Vec<Value>>,
    pub terminal_state: Option<NativeTurnTerminalState>,
    pub pending_events: Vec<HarnessEvent>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeTurnTerminalOutcome {
    Completed,
    Failed,
    Cancelled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeTurnTerminalState {
    Pending(NativeTurnTerminalOutcome),
    Emitted(NativeTurnTerminalOutcome),
}

impl NativeTurnState {
    pub fn new(thread_id: impl Into<String>) -> Self {
        Self {
            thread_id: thread_id.into(),
            active_turn_id: None,
            native_cursor: None,
            notification_sequence: 0,
            pending_import: None,
            terminal_state: None,
            pending_events: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProtocolSessionConfiguration {
    pub requests: Vec<JsonRpcRequestSpec>,
    pub events: Vec<HarnessEvent>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProtocolEvent {
    pub event: HarnessEvent,
    pub native_cursor: Option<String>,
    pub final_text: Option<String>,
    pub usage: Option<Value>,
    pub terminal: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ServerResponse {
    Result(Value),
    Error(JsonRpcErrorObject),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProtocolAttemptInstruction {
    Request(JsonRpcRequestSpec),
    Notification(JsonRpcNotification),
}

impl ProtocolAttemptInstruction {
    pub async fn send(
        &self,
        runtime: &JsonRpcProcessRuntime,
    ) -> Result<Option<Value>, super::json_rpc::JsonRpcError> {
        match self {
            Self::Request(request) => runtime
                .request(&request.method, request.params.clone())
                .await
                .map(Some),
            Self::Notification(notification) => {
                runtime
                    .notify(&notification.method, notification.params.clone())
                    .await?;
                Ok(None)
            }
        }
    }
}

pub trait HarnessProtocolCodec: Send + Sync {
    fn initialize_request(&self) -> JsonRpcRequestSpec;
    fn import_items(&self, _capsule: &SessionCapsule) -> Result<Vec<Value>, HarnessAdapterError> {
        Err(HarnessAdapterError::InvalidRequest(
            "session capsule import is not supported by this protocol codec".to_owned(),
        ))
    }
    fn session_request(
        &self,
        operation: SessionOperation,
        request: &HarnessSessionRequest,
    ) -> Result<JsonRpcRequestSpec, HarnessAdapterError>;
    fn attempt_request(
        &self,
        operation: AttemptOperation,
        request: &HarnessSessionRequest,
        prompt: &str,
        native_session_id: Option<&str>,
        native_turn_id: Option<&str>,
    ) -> Result<JsonRpcRequestSpec, HarnessAdapterError>;
    fn attempt_instruction(
        &self,
        operation: AttemptOperation,
        request: &HarnessSessionRequest,
        prompt: &str,
        native_session_id: Option<&str>,
        native_turn_id: Option<&str>,
    ) -> Result<ProtocolAttemptInstruction, HarnessAdapterError> {
        self.attempt_request(
            operation,
            request,
            prompt,
            native_session_id,
            native_turn_id,
        )
        .map(ProtocolAttemptInstruction::Request)
    }
    fn decode_session_result(
        &self,
        operation: SessionOperation,
        result: Value,
    ) -> Result<NativeSession, HarnessAdapterError>;
    fn session_configuration(
        &self,
        _native_session_id: &str,
    ) -> Result<ProtocolSessionConfiguration, HarnessAdapterError> {
        Ok(ProtocolSessionConfiguration::default())
    }
    fn decode_attempt_result(
        &self,
        _operation: AttemptOperation,
        _native_session_id: &str,
        _result: Value,
    ) -> Result<Option<ProtocolEvent>, HarnessAdapterError> {
        Ok(None)
    }
    fn decode_notification(
        &self,
        notification: &JsonRpcNotification,
        state: &mut NativeTurnState,
    ) -> Result<ProtocolEvent, HarnessAdapterError>;
    fn server_request_event(
        &self,
        request: &JsonRpcServerRequest,
    ) -> Result<HarnessEvent, HarnessAdapterError>;
    fn encode_server_response(
        &self,
        method: &str,
        response: &Value,
    ) -> Result<ServerResponse, HarnessAdapterError>;
}

pub struct ProtocolProcessAdapter {
    pub runtime: JsonRpcProcessRuntime,
    pub codec: Arc<dyn HarnessProtocolCodec>,
    /// The last selected session is retained for legacy callers that omit the
    /// native thread id. Per-thread state lives in `states` so concurrent
    /// sessions cannot overwrite each other's turn cursor.
    pub state: Arc<Mutex<NativeTurnState>>,
    pub states: Arc<Mutex<BTreeMap<String, NativeTurnState>>>,
}

impl ProtocolProcessAdapter {
    pub async fn exchange(
        &self,
        kind: &str,
        payload: &Value,
    ) -> Result<Value, HarnessAdapterError> {
        let _ = kind;
        let native_request_id = payload.get("native_request_id").cloned().ok_or_else(|| {
            HarnessAdapterError::InvalidRequest(
                "native exchange requires native_request_id".to_owned(),
            )
        })?;
        let has_result = payload.get("result").is_some();
        let has_error = payload.get("error").is_some();
        if has_result == has_error {
            return Err(HarnessAdapterError::InvalidRequest(
                "native exchange requires exactly one result or error".to_owned(),
            ));
        }
        let id =
            JsonRpcId::from_value(native_request_id.clone()).map_err(HarnessAdapterError::from)?;
        let response = self.codec.encode_server_response(kind, payload)?;
        match response {
            ServerResponse::Result(result) => self
                .runtime
                .respond_server_request(id, result)
                .await
                .map_err(HarnessAdapterError::from)?,
            ServerResponse::Error(error) => self
                .runtime
                .respond_server_error(id, error)
                .await
                .map_err(HarnessAdapterError::from)?,
        }
        Ok(serde_json::json!({
            "accepted": true,
            "native_request_id": native_request_id,
        }))
    }
}

impl std::fmt::Debug for ProtocolProcessAdapter {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProtocolProcessAdapter")
            .field("state", &self.state)
            .field("states", &self.states)
            .finish()
    }
}
