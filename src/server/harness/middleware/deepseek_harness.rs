use serde_json::{Map, Value, json};

use super::harness::{HarnessAdapterError, HarnessEvent, HarnessSessionRequest, NativeSession};
use super::json_rpc::{JsonRpcNotification, JsonRpcServerRequest};
use super::protocol::{
    AttemptOperation, HarnessProtocolCodec, JsonRpcRequestSpec, JsonRpcRequestWithoutParamsSpec,
    NativeTurnState, NativeTurnTerminalOutcome, NativeTurnTerminalState, ProtocolEvent,
    ServerResponse, SessionOperation,
};

const DEFAULT_PROVIDER: &str = "deepseek-official";
const DEFAULT_MODEL: &str = "deepseek-official";
const SERVER_NAME: &str = "deepseek-harness-sdk-runtime";
const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeepSeekServerInfo {
    pub name: String,
    pub version: String,
}

#[derive(Clone, Debug)]
pub struct DeepSeekHarnessCodec {
    cwd: String,
    provider: String,
    model: String,
    max_tokens: Option<u64>,
}

impl DeepSeekHarnessCodec {
    pub fn for_request(
        cwd: impl Into<String>,
        request: &HarnessSessionRequest,
    ) -> Result<Self, HarnessAdapterError> {
        let cwd = cwd.into();
        if cwd.trim().is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(
                "DeepSeek Harness initialize cwd is required".to_owned(),
            ));
        }
        let (provider, model, max_tokens) = request
            .resolved_model
            .as_ref()
            .map(|selection| {
                (
                    selection.provider_route.clone(),
                    selection.model_id.clone(),
                    selection.max_output_tokens,
                )
            })
            .unwrap_or_else(|| (DEFAULT_PROVIDER.to_owned(), DEFAULT_MODEL.to_owned(), None));
        if provider.trim().is_empty() || model.trim().is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(
                "DeepSeek Harness provider and model are required".to_owned(),
            ));
        }
        if max_tokens.is_some_and(|value| value == 0 || value > MAX_SAFE_INTEGER) {
            return Err(HarnessAdapterError::InvalidRequest(
                "DeepSeek Harness maxTokens must be a positive safe integer".to_owned(),
            ));
        }
        Ok(Self {
            cwd,
            provider,
            model,
            max_tokens,
        })
    }

    pub fn initialize_request(&self) -> JsonRpcRequestSpec {
        let mut params = Map::new();
        params.insert("cwd".to_owned(), Value::String(self.cwd.clone()));
        params.insert("provider".to_owned(), Value::String(self.provider.clone()));
        params.insert("model".to_owned(), Value::String(self.model.clone()));
        if let Some(max_tokens) = self.max_tokens {
            params.insert("maxTokens".to_owned(), Value::from(max_tokens));
        }
        JsonRpcRequestSpec {
            method: "initialize".to_owned(),
            params: Value::Object(params),
        }
    }

    pub fn shutdown_request(&self) -> JsonRpcRequestWithoutParamsSpec {
        JsonRpcRequestWithoutParamsSpec::new("shutdown")
    }

    pub fn decode_initialize_result(
        &self,
        result: Value,
    ) -> Result<DeepSeekServerInfo, HarnessAdapterError> {
        let server_info = result
            .get("serverInfo")
            .and_then(Value::as_object)
            .ok_or_else(|| invalid_response("initialize result did not contain serverInfo"))?;
        let name = required_string(server_info.get("name"), "initialize serverInfo.name")?;
        if name != SERVER_NAME {
            return Err(invalid_response(
                "initialize serverInfo.name did not match the DeepSeek SDK runtime",
            ));
        }
        let version = required_string(server_info.get("version"), "initialize serverInfo.version")?;
        Ok(DeepSeekServerInfo {
            name: name.to_owned(),
            version: version.to_owned(),
        })
    }

    pub fn decode_prompt_result(&self, result: Value) -> Result<String, HarnessAdapterError> {
        required_string(result.get("messageId"), "session/prompt messageId").map(str::to_owned)
    }

    pub fn decode_shutdown_result(&self, result: Value) -> Result<(), HarnessAdapterError> {
        match result.as_object() {
            Some(result) if result.is_empty() => Ok(()),
            _ => Err(invalid_response("shutdown result must be an empty object")),
        }
    }

    fn unsupported(&self, operation: &str) -> HarnessAdapterError {
        HarnessAdapterError::InvalidRequest(format!(
            "DeepSeek Harness SDK protocol operation is not supported: {operation}"
        ))
    }
}

impl HarnessProtocolCodec for DeepSeekHarnessCodec {
    fn initialize_request(&self) -> JsonRpcRequestSpec {
        Self::initialize_request(self)
    }

    fn session_request(
        &self,
        operation: SessionOperation,
        _request: &HarnessSessionRequest,
    ) -> Result<JsonRpcRequestSpec, HarnessAdapterError> {
        let operation = match operation {
            SessionOperation::Create => "session creation (sessions are created by session/prompt)",
            SessionOperation::Import(_) => "session import",
            SessionOperation::Resume { .. } => "session resume",
            SessionOperation::Fork { .. } => "session fork",
            SessionOperation::Snapshot => "session snapshot",
            SessionOperation::Quiesce => "session quiesce",
            SessionOperation::Cancel => "session cancel",
            SessionOperation::Close => "session close",
            SessionOperation::Delete => "session delete",
        };
        Err(self.unsupported(operation))
    }

    fn attempt_request(
        &self,
        operation: AttemptOperation,
        request: &HarnessSessionRequest,
        prompt: &str,
        native_session_id: Option<&str>,
        _native_turn_id: Option<&str>,
    ) -> Result<JsonRpcRequestSpec, HarnessAdapterError> {
        match operation {
            AttemptOperation::Start | AttemptOperation::Execute | AttemptOperation::Resume => {}
            AttemptOperation::Steer => return Err(self.unsupported("prompt steering")),
            AttemptOperation::Cancel => return Err(self.unsupported("prompt cancellation")),
            AttemptOperation::Quiesce => return Err(self.unsupported("attempt quiesce")),
            AttemptOperation::Reconcile => return Err(self.unsupported("attempt reconcile")),
        }
        if prompt.trim().is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(
                "DeepSeek Harness session/prompt content is required".to_owned(),
            ));
        }
        let session_id = native_session_id.unwrap_or_default().trim().to_owned();
        if session_id.is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(format!(
                "DeepSeek Harness native session id is required for request {}",
                request.request_id
            )));
        }
        Ok(JsonRpcRequestSpec {
            method: "session/prompt".to_owned(),
            params: json!({
                "sessionId": session_id,
                "contentBlocks": [{"type": "text", "text": prompt}],
            }),
        })
    }

    fn decode_session_result(
        &self,
        _operation: SessionOperation,
        _result: Value,
    ) -> Result<NativeSession, HarnessAdapterError> {
        Err(invalid_response(
            "DeepSeek Harness has no session lifecycle result; session/prompt returns messageId",
        ))
    }

    fn decode_notification(
        &self,
        notification: &JsonRpcNotification,
        state: &mut NativeTurnState,
    ) -> Result<ProtocolEvent, HarnessAdapterError> {
        match notification.method.as_str() {
            "session.event" => decode_session_event(&notification.params, state),
            "session.status" => decode_session_status(&notification.params, state),
            "subagent.started" => decode_subagent_started(&notification.params, state),
            "subagent.finished" => decode_subagent_finished(&notification.params, state),
            method => Err(invalid_response(format!(
                "unsupported DeepSeek Harness notification method: {method}"
            ))),
        }
    }

    fn server_request_event(
        &self,
        _request: &JsonRpcServerRequest,
    ) -> Result<HarnessEvent, HarnessAdapterError> {
        Err(self.unsupported("server-to-client requests"))
    }

    fn encode_server_response(
        &self,
        _method: &str,
        _response: &Value,
    ) -> Result<ServerResponse, HarnessAdapterError> {
        Err(self.unsupported("server-to-client request responses"))
    }
}

fn decode_session_event(
    params: &Value,
    state: &mut NativeTurnState,
) -> Result<ProtocolEvent, HarnessAdapterError> {
    let params = required_object(params, "session.event params")?;
    let session_id = correlated_session_id(params, "sessionId", state)?;
    let native_event = params
        .get("event")
        .and_then(Value::as_object)
        .ok_or_else(|| invalid_response("session.event event must be an object"))?;
    let event_type = required_string(native_event.get("type"), "session.event event.type")?;
    let sequence = native_event
        .get("seq")
        .and_then(Value::as_u64)
        .ok_or_else(|| invalid_response("session.event event.seq must be an unsigned integer"))?;
    if !native_event.get("time").is_some_and(Value::is_number) {
        return Err(invalid_response(
            "session.event event.time must be a number",
        ));
    }
    let data = native_event
        .get("data")
        .and_then(Value::as_object)
        .ok_or_else(|| invalid_response("session.event event.data must be an object"))?;
    state.notification_sequence = state.notification_sequence.saturating_add(1);
    let cursor = format!("{session_id}:{sequence}");
    state.native_cursor = Some(cursor.clone());
    let native = Value::Object(native_event.clone());

    let (canonical_type, durable, payload, final_text, usage) = match event_type {
        "turn/start" => {
            state.terminal_state = None;
            (
                "turn.started",
                true,
                json!({"turn": data.get("turn"), "native": native}),
                None,
                None,
            )
        }
        "turn/end" => {
            let reason = data
                .get("reason")
                .and_then(Value::as_object)
                .ok_or_else(|| invalid_response("turn/end reason must be an object"))?;
            let kind = required_string(reason.get("kind"), "turn/end reason.kind")?;
            let outcome = match kind {
                "completed" => NativeTurnTerminalOutcome::Completed,
                "aborted" => NativeTurnTerminalOutcome::Cancelled,
                "error" | "blocked" | "max-tokens" | "interrupted" => {
                    NativeTurnTerminalOutcome::Failed
                }
                _ => NativeTurnTerminalOutcome::Failed,
            };
            state.terminal_state = Some(NativeTurnTerminalState::Pending(outcome));
            (
                "native.deepseek.turn_end",
                true,
                json!({"turn": data.get("turn"), "reason": reason, "native": native}),
                None,
                None,
            )
        }
        "assistant/chunk" => decode_assistant_chunk(data, &native)?,
        "assistant/message" => {
            let message = data
                .get("message")
                .and_then(Value::as_object)
                .ok_or_else(|| invalid_response("assistant/message message must be an object"))?;
            let final_text = content_text(message.get("content"))?;
            let usage = optional_object(data.get("usage"), "assistant/message usage")?;
            (
                "assistant.final",
                true,
                json!({"message": message, "usage": usage, "native": native}),
                final_text,
                usage,
            )
        }
        "tool/call" => {
            let call_id = required_string(data.get("callId"), "tool/call callId")?;
            let name = required_string(data.get("name"), "tool/call name")?;
            let arguments = required_string(data.get("arguments"), "tool/call arguments")?;
            (
                "tool.started",
                true,
                json!({"call_id": call_id, "name": name, "arguments": arguments, "native": native}),
                None,
                None,
            )
        }
        "tool/result" => {
            let message = data
                .get("message")
                .and_then(Value::as_object)
                .ok_or_else(|| invalid_response("tool/result message must be an object"))?;
            let call_id = required_string(message.get("toolCallId"), "tool/result toolCallId")?;
            (
                "tool.completed",
                true,
                json!({"call_id": call_id, "message": message, "error": data.get("error"), "native": native}),
                None,
                None,
            )
        }
        _ => (
            "native.deepseek.session_event",
            true,
            json!({"event": native}),
            None,
            None,
        ),
    };

    Ok(ProtocolEvent {
        event: HarnessEvent {
            event_type: canonical_type.to_owned(),
            durable,
            payload,
            native_cursor: Some(cursor.clone()),
        },
        native_cursor: Some(cursor),
        final_text,
        usage,
        terminal: false,
    })
}

type AssistantChunk = (&'static str, bool, Value, Option<String>, Option<Value>);

fn decode_assistant_chunk(
    data: &Map<String, Value>,
    native: &Value,
) -> Result<AssistantChunk, HarnessAdapterError> {
    let chunk = data
        .get("chunk")
        .and_then(Value::as_object)
        .ok_or_else(|| invalid_response("assistant/chunk chunk must be an object"))?;
    let chunk_type = required_string(chunk.get("type"), "assistant/chunk chunk.type")?;
    match chunk_type {
        "text-delta" => Ok((
            "assistant.text_chunk",
            false,
            json!({
                "content": streamed_text(chunk.get("text"), "text-delta text")?,
                "native": native,
            }),
            None,
            None,
        )),
        "reasoning-delta" => Ok((
            "assistant.reasoning",
            false,
            json!({
                "content": streamed_text(chunk.get("text"), "reasoning-delta text")?,
                "native": native,
            }),
            None,
            None,
        )),
        "tool-call-delta" => Ok((
            "tool.output",
            false,
            json!({"chunk": chunk, "native": native}),
            None,
            None,
        )),
        "usage" => {
            let usage = chunk
                .get("usage")
                .and_then(Value::as_object)
                .cloned()
                .map(Value::Object)
                .ok_or_else(|| invalid_response("usage chunk usage must be an object"))?;
            Ok((
                "usage.recorded",
                true,
                json!({"usage": usage, "native": native}),
                None,
                Some(usage),
            ))
        }
        _ => Ok((
            "native.deepseek.assistant_chunk",
            false,
            json!({"chunk": chunk, "native": native}),
            None,
            None,
        )),
    }
}

fn decode_session_status(
    params: &Value,
    state: &mut NativeTurnState,
) -> Result<ProtocolEvent, HarnessAdapterError> {
    let params = required_object(params, "session.status params")?;
    let session_id = correlated_session_id(params, "sessionId", state)?;
    let status = required_string(params.get("status"), "session.status status")?;
    let (event_type, terminal) = match status {
        "running" => {
            state.terminal_state = None;
            ("turn.started", false)
        }
        "idle" => match state.terminal_state {
            Some(NativeTurnTerminalState::Emitted(_)) => ("native.deepseek.session_status", false),
            Some(NativeTurnTerminalState::Pending(outcome)) => {
                state.terminal_state = Some(NativeTurnTerminalState::Emitted(outcome));
                (terminal_event_type(outcome), true)
            }
            None => {
                let outcome = NativeTurnTerminalOutcome::Completed;
                state.terminal_state = Some(NativeTurnTerminalState::Emitted(outcome));
                (terminal_event_type(outcome), true)
            }
        },
        _ => {
            return Err(invalid_response(
                "session.status status must be running or idle",
            ));
        }
    };
    protocol_event(
        state,
        session_id,
        event_type,
        true,
        json!({"status": status, "native": params}),
        terminal,
    )
}

fn terminal_event_type(outcome: NativeTurnTerminalOutcome) -> &'static str {
    match outcome {
        NativeTurnTerminalOutcome::Completed => "turn.completed",
        NativeTurnTerminalOutcome::Failed => "turn.failed",
        NativeTurnTerminalOutcome::Cancelled => "turn.cancelled",
    }
}

fn decode_subagent_started(
    params: &Value,
    state: &mut NativeTurnState,
) -> Result<ProtocolEvent, HarnessAdapterError> {
    let params = required_object(params, "subagent.started params")?;
    let parent = correlated_session_id(params, "parentSessionId", state)?;
    let child = required_string(
        params.get("childSessionId"),
        "subagent.started childSessionId",
    )?;
    protocol_event(
        state,
        parent,
        "subagent.started",
        true,
        json!({"parent_session_id": parent, "child_session_id": child, "native": params}),
        false,
    )
}

fn decode_subagent_finished(
    params: &Value,
    state: &mut NativeTurnState,
) -> Result<ProtocolEvent, HarnessAdapterError> {
    let params = required_object(params, "subagent.finished params")?;
    let parent = correlated_session_id(params, "parentSessionId", state)?;
    let child = required_string(
        params.get("childSessionId"),
        "subagent.finished childSessionId",
    )?;
    let agent_id = required_string(params.get("agentId"), "subagent.finished agentId")?;
    let provider = required_string(params.get("provider"), "subagent.finished provider")?;
    let status = required_string(params.get("status"), "subagent.finished status")?;
    if !matches!(status, "ok" | "error") {
        return Err(invalid_response(
            "subagent.finished status must be ok or error",
        ));
    }
    let stop_reason = required_string(params.get("stopReason"), "subagent.finished stopReason")?;
    if let Some(message) = params.get("lastAssistantMessage")
        && !message.is_array()
    {
        return Err(invalid_response(
            "subagent.finished lastAssistantMessage must be an array",
        ));
    }
    protocol_event(
        state,
        parent,
        if status == "ok" {
            "subagent.finished"
        } else {
            "subagent.failed"
        },
        true,
        json!({
            "provider": provider,
            "agent_id": agent_id,
            "parent_session_id": parent,
            "child_session_id": child,
            "status": status,
            "stop_reason": stop_reason,
            "last_assistant_message": params.get("lastAssistantMessage"),
            "native": params,
        }),
        false,
    )
}

fn protocol_event(
    state: &mut NativeTurnState,
    session_id: &str,
    event_type: &str,
    durable: bool,
    payload: Value,
    terminal: bool,
) -> Result<ProtocolEvent, HarnessAdapterError> {
    state.notification_sequence = state.notification_sequence.saturating_add(1);
    let cursor = format!("{session_id}:notification:{}", state.notification_sequence);
    state.native_cursor = Some(cursor.clone());
    Ok(ProtocolEvent {
        event: HarnessEvent {
            event_type: event_type.to_owned(),
            durable,
            payload,
            native_cursor: Some(cursor.clone()),
        },
        native_cursor: Some(cursor),
        final_text: None,
        usage: None,
        terminal,
    })
}

fn correlated_session_id<'a>(
    params: &'a Map<String, Value>,
    field: &str,
    state: &NativeTurnState,
) -> Result<&'a str, HarnessAdapterError> {
    let session_id = required_string(params.get(field), field)?;
    if session_id != state.thread_id {
        return Err(invalid_response(format!(
            "DeepSeek Harness notification session id {session_id:?} did not match active session {:?}",
            state.thread_id
        )));
    }
    Ok(session_id)
}

fn content_text(content: Option<&Value>) -> Result<Option<String>, HarnessAdapterError> {
    let blocks = content
        .and_then(Value::as_array)
        .ok_or_else(|| invalid_response("assistant message content must be an array"))?;
    let mut text = String::new();
    for block in blocks {
        let block = block
            .as_object()
            .ok_or_else(|| invalid_response("assistant message content block must be an object"))?;
        if block.get("type").and_then(Value::as_str) == Some("text") {
            text.push_str(required_string(
                block.get("text"),
                "assistant text block text",
            )?);
        }
    }
    Ok((!text.is_empty()).then_some(text))
}

fn optional_object(
    value: Option<&Value>,
    field: &str,
) -> Result<Option<Value>, HarnessAdapterError> {
    match value {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Object(value)) => Ok(Some(Value::Object(value.clone()))),
        Some(_) => Err(invalid_response(format!("{field} must be an object"))),
    }
}

fn required_object<'a>(
    value: &'a Value,
    field: &str,
) -> Result<&'a Map<String, Value>, HarnessAdapterError> {
    value
        .as_object()
        .ok_or_else(|| invalid_response(format!("{field} must be an object")))
}

fn streamed_text<'a>(
    value: Option<&'a Value>,
    field: &str,
) -> Result<&'a str, HarnessAdapterError> {
    value
        .and_then(Value::as_str)
        .ok_or_else(|| invalid_response(format!("{field} must be a string")))
}

fn required_string<'a>(
    value: Option<&'a Value>,
    field: &str,
) -> Result<&'a str, HarnessAdapterError> {
    value
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| invalid_response(format!("{field} must be a non-empty string")))
}

fn invalid_response(message: impl Into<String>) -> HarnessAdapterError {
    HarnessAdapterError::InvalidResponse(message.into())
}
