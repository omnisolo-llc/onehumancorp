use serde_json::{Value, json};

use super::capsule::{PortableRecord, SessionCapsule};
use super::harness::{HarnessAdapterError, HarnessEvent, HarnessSessionRequest, NativeSession};
use super::json_rpc::{JsonRpcNotification, JsonRpcServerRequest};
use super::protocol::{
    AttemptOperation, HarnessProtocolCodec, JsonRpcRequestSpec, NativeTurnState, ProtocolEvent,
    ServerResponse, SessionOperation,
};
use super::types::{ContentPart, ReasoningEffort, sanitize_credential_value};

#[derive(Clone, Debug, Default)]
pub struct CodexAppServerV2Codec;

impl CodexAppServerV2Codec {
    pub fn new() -> Self {
        Self
    }

    pub fn initialize_request(&self) -> JsonRpcRequestSpec {
        JsonRpcRequestSpec {
            method: "initialize".to_owned(),
            params: json!({
                "clientInfo": {
                    "name": "omnisolo",
                    "title": "OmniSolo Harness Middleware",
                    "version": env!("CARGO_PKG_VERSION"),
                },
                "capabilities": {
                    "experimentalApi": true,
                    "extensions": {},
                    "requestAttestation": false,
                },
            }),
        }
    }

    pub fn parse_thread_result(&self, result: Value) -> Result<NativeSession, HarnessAdapterError> {
        let native_session_id = result
            .get("thread")
            .and_then(|thread| thread.get("id"))
            .and_then(Value::as_str)
            .or_else(|| result.get("id").and_then(Value::as_str))
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| {
                HarnessAdapterError::InvalidResponse(
                    "Codex thread response did not contain a native thread id".to_owned(),
                )
            })?;
        Ok(NativeSession {
            native_session_id: native_session_id.to_owned(),
            native_cursor: result
                .get("thread")
                .and_then(|thread| thread.get("id"))
                .and_then(Value::as_str)
                .map(str::to_owned),
        })
    }

    pub fn capsule_items(
        &self,
        capsule: &SessionCapsule,
    ) -> Result<Vec<Value>, HarnessAdapterError> {
        capsule
            .verify_integrity()
            .map_err(|error| HarnessAdapterError::Capsule(format!("{error:?}")))?;
        let mut items = Vec::new();
        for record in &capsule.records {
            let (record_type, text) = match record {
                PortableRecord::Task(task) => ("task", task.objective.clone()),
                PortableRecord::Message(message) if message.visible_to_user => {
                    ("message", content_text(&message.content))
                }
                PortableRecord::ToolResult(result) => {
                    ("tool_result", content_text(&result.content))
                }
                PortableRecord::Plan(plan) | PortableRecord::Todo(plan) => {
                    ("plan", structured_summary(&plan.data))
                }
                PortableRecord::Goal(goal) => ("goal", structured_summary(&goal.data)),
                PortableRecord::HistoricalData(data) => ("historical", data.summary.clone()),
                PortableRecord::ContextCheckpoint(checkpoint) => {
                    ("context_checkpoint", checkpoint.summary.clone())
                }
                _ => continue,
            };
            if text.trim().is_empty() {
                continue;
            }
            items.push(json!({
                "type": "message",
                "role": "user",
                "content": [{"type": "input_text", "text": text}],
                "metadata": {
                    "source": "omnisolo.historical",
                    "record_type": record_type,
                },
            }));
        }
        if items.is_empty() {
            items.push(json!({
                "type": "message",
                "role": "user",
                "content": [{
                    "type": "input_text",
                    "text": "Historical OmniSolo session context was transferred without visible messages."
                }],
                "metadata": {"source": "omnisolo.historical", "record_type": "empty"},
            }));
        }
        Ok(items)
    }

    fn thread_options(
        &self,
        request: &HarnessSessionRequest,
    ) -> Result<serde_json::Map<String, Value>, HarnessAdapterError> {
        let mut options = serde_json::Map::new();
        options.insert("ephemeral".to_owned(), Value::Bool(false));
        options.insert(
            "approvalPolicy".to_owned(),
            Value::String("never".to_owned()),
        );
        options.insert("sandbox".to_owned(), Value::String("read-only".to_owned()));
        if let Some(Value::String(cwd)) = request.extensions.get("codex.cwd") {
            options.insert("cwd".to_owned(), Value::String(cwd.clone()));
        }
        self.insert_model_options(request, &mut options, true)?;
        if let Some(Value::Bool(ephemeral)) = request.extensions.get("codex.ephemeral") {
            options.insert("ephemeral".to_owned(), Value::Bool(*ephemeral));
        }
        if let Some(Value::String(personality)) = request.extensions.get("codex.personality") {
            options.insert("personality".to_owned(), Value::String(personality.clone()));
        }
        if let Some(Value::String(service_name)) = request.extensions.get("codex.service_name") {
            options.insert(
                "serviceName".to_owned(),
                Value::String(service_name.clone()),
            );
        }
        Ok(options)
    }

    fn turn_options(
        &self,
        request: &HarnessSessionRequest,
    ) -> Result<serde_json::Map<String, Value>, HarnessAdapterError> {
        let mut options = serde_json::Map::new();
        options.insert(
            "approvalPolicy".to_owned(),
            Value::String("never".to_owned()),
        );
        options.insert("sandboxPolicy".to_owned(), json!({"type": "readOnly"}));
        for (source, target) in [("codex.cwd", "cwd"), ("codex.personality", "personality")] {
            if let Some(Value::String(value)) = request.extensions.get(source) {
                options.insert(target.to_owned(), Value::String(value.clone()));
            }
        }
        self.insert_model_options(request, &mut options, false)?;
        if let Some(effort) = self.reasoning_effort(request)? {
            options.insert("effort".to_owned(), Value::String(effort));
        }
        Ok(options)
    }

    fn insert_model_options(
        &self,
        request: &HarnessSessionRequest,
        options: &mut serde_json::Map<String, Value>,
        include_provider: bool,
    ) -> Result<(), HarnessAdapterError> {
        if let Some(selection) = &request.resolved_model {
            let model = validated_routing_value(&selection.model_id, "Codex model")?;
            options.insert("model".to_owned(), Value::String(model));
            if include_provider {
                let provider =
                    validated_routing_value(&selection.provider_route, "Codex model provider")?;
                let provider = match provider.as_str() {
                    "openai-compatible" => "omnisolo",
                    provider => provider,
                };
                options.insert(
                    "modelProvider".to_owned(),
                    Value::String(provider.to_owned()),
                );
            }
            return Ok(());
        }

        if let Some(Value::String(model)) = request.extensions.get("codex.model") {
            options.insert(
                "model".to_owned(),
                Value::String(validated_routing_value(model, "Codex model")?),
            );
        }
        if include_provider
            && let Some(Value::String(provider)) = request.extensions.get("codex.model_provider")
        {
            options.insert(
                "modelProvider".to_owned(),
                Value::String(validated_routing_value(provider, "Codex model provider")?),
            );
        }
        Ok(())
    }

    fn reasoning_effort(
        &self,
        request: &HarnessSessionRequest,
    ) -> Result<Option<String>, HarnessAdapterError> {
        let Some(selection) = &request.resolved_model else {
            return request
                .extensions
                .get("codex.reasoning_effort")
                .and_then(Value::as_str)
                .map(validated_reasoning_effort)
                .transpose();
        };
        let effort = match selection.reasoning_effort.as_ref() {
            None => return Ok(None),
            Some(ReasoningEffort::None) => "none",
            Some(ReasoningEffort::Minimal) => "minimal",
            Some(ReasoningEffort::Low) => "low",
            Some(ReasoningEffort::Medium) => "medium",
            Some(ReasoningEffort::High) => "high",
            Some(ReasoningEffort::Max) => "max",
            Some(ReasoningEffort::Custom) => {
                return Err(HarnessAdapterError::InvalidRequest(
                    "Codex app-server v2 cannot translate a custom reasoning effort".to_owned(),
                ));
            }
        };
        Ok(Some(effort.to_owned()))
    }

    fn request_thread_id(
        &self,
        request: &HarnessSessionRequest,
    ) -> Result<String, HarnessAdapterError> {
        self.required_thread_id(
            request
                .extensions
                .get("native_session_id")
                .and_then(Value::as_str),
        )
    }

    fn required_thread_id(
        &self,
        native_session_id: Option<&str>,
    ) -> Result<String, HarnessAdapterError> {
        native_session_id
            .filter(|value| !value.trim().is_empty())
            .map(str::to_owned)
            .ok_or_else(|| {
                HarnessAdapterError::InvalidRequest("native thread id is required".to_owned())
            })
    }

    fn required_turn_id(
        &self,
        native_turn_id: Option<&str>,
    ) -> Result<String, HarnessAdapterError> {
        native_turn_id
            .filter(|value| !value.trim().is_empty())
            .map(str::to_owned)
            .ok_or_else(|| {
                HarnessAdapterError::InvalidRequest("native turn id is required".to_owned())
            })
    }
}

impl HarnessProtocolCodec for CodexAppServerV2Codec {
    fn initialize_request(&self) -> JsonRpcRequestSpec {
        Self::initialize_request(self)
    }

    fn import_items(&self, capsule: &SessionCapsule) -> Result<Vec<Value>, HarnessAdapterError> {
        self.capsule_items(capsule)
    }

    fn session_request(
        &self,
        operation: SessionOperation,
        request: &HarnessSessionRequest,
    ) -> Result<JsonRpcRequestSpec, HarnessAdapterError> {
        let (method, params) = match operation {
            SessionOperation::Create | SessionOperation::Import(_) => {
                ("thread/start", self.thread_options(request)?)
            }
            SessionOperation::Resume { native_session_id } => {
                let mut params = self.thread_options(request)?;
                params.insert(
                    "threadId".to_owned(),
                    Value::String(self.required_thread_id(Some(&native_session_id))?),
                );
                ("thread/resume", params)
            }
            SessionOperation::Fork { native_session_id } => {
                let mut params = serde_json::Map::new();
                let thread_id = match native_session_id {
                    Some(native_session_id) => self.required_thread_id(Some(&native_session_id))?,
                    None => self.request_thread_id(request)?,
                };
                params.insert("threadId".to_owned(), Value::String(thread_id));
                if let Some(Value::String(last_turn_id)) =
                    request.extensions.get("codex.last_turn_id")
                {
                    params.insert("lastTurnId".to_owned(), Value::String(last_turn_id.clone()));
                }
                if let Some(Value::Bool(ephemeral)) = request.extensions.get("codex.ephemeral") {
                    params.insert("ephemeral".to_owned(), Value::Bool(*ephemeral));
                }
                ("thread/fork", params)
            }
            SessionOperation::Snapshot | SessionOperation::Quiesce | SessionOperation::Cancel => {
                let mut params = serde_json::Map::new();
                params.insert(
                    "threadId".to_owned(),
                    Value::String(self.request_thread_id(request)?),
                );
                params.insert("includeTurns".to_owned(), Value::Bool(true));
                ("thread/read", params)
            }
            SessionOperation::Close => {
                let mut params = serde_json::Map::new();
                params.insert(
                    "threadId".to_owned(),
                    Value::String(self.request_thread_id(request)?),
                );
                ("thread/archive", params)
            }
            SessionOperation::Delete => {
                let mut params = serde_json::Map::new();
                params.insert(
                    "threadId".to_owned(),
                    Value::String(self.request_thread_id(request)?),
                );
                ("thread/delete", params)
            }
        };
        Ok(JsonRpcRequestSpec {
            method: method.to_owned(),
            params: Value::Object(params),
        })
    }

    fn attempt_request(
        &self,
        operation: AttemptOperation,
        request: &HarnessSessionRequest,
        prompt: &str,
        native_session_id: Option<&str>,
        native_turn_id: Option<&str>,
    ) -> Result<JsonRpcRequestSpec, HarnessAdapterError> {
        let thread_id = self.required_thread_id(native_session_id)?;
        match operation {
            AttemptOperation::Start | AttemptOperation::Execute | AttemptOperation::Resume => {
                if prompt.trim().is_empty() {
                    return Err(HarnessAdapterError::InvalidRequest(
                        "prompt is required for Codex turn/start".to_owned(),
                    ));
                }
                let mut params = self.turn_options(request)?;
                params.insert("threadId".to_owned(), Value::String(thread_id));
                params.insert("input".to_owned(), json!([{"type":"text","text":prompt}]));
                Ok(JsonRpcRequestSpec {
                    method: "turn/start".to_owned(),
                    params: Value::Object(params),
                })
            }
            AttemptOperation::Steer => {
                let turn_id = self.required_turn_id(native_turn_id)?;
                Ok(JsonRpcRequestSpec {
                    method: "turn/steer".to_owned(),
                    params: json!({
                        "threadId": thread_id,
                        "expectedTurnId": turn_id,
                        "input": [{"type":"text","text":prompt}],
                    }),
                })
            }
            AttemptOperation::Cancel | AttemptOperation::Quiesce => {
                let turn_id = self.required_turn_id(native_turn_id)?;
                Ok(JsonRpcRequestSpec {
                    method: "turn/interrupt".to_owned(),
                    params: json!({"threadId": thread_id, "turnId": turn_id}),
                })
            }
            AttemptOperation::Reconcile => Ok(JsonRpcRequestSpec {
                method: "thread/read".to_owned(),
                params: json!({"threadId": thread_id, "includeTurns": true}),
            }),
        }
    }

    fn decode_session_result(
        &self,
        _operation: SessionOperation,
        result: Value,
    ) -> Result<NativeSession, HarnessAdapterError> {
        self.parse_thread_result(result)
    }

    fn decode_notification(
        &self,
        notification: &JsonRpcNotification,
        state: &mut NativeTurnState,
    ) -> Result<ProtocolEvent, HarnessAdapterError> {
        let params = &notification.params;
        let thread_id = notification_thread_id(params, state).to_owned();
        if thread_id != state.thread_id {
            return Err(HarnessAdapterError::InvalidResponse(
                "Codex notification thread id does not match the active session".to_owned(),
            ));
        }
        if let Some(turn_id) = notification_turn_id(params) {
            state.active_turn_id = Some(turn_id.to_owned());
        }
        state.notification_sequence = state.notification_sequence.saturating_add(1);
        let cursor_root = state.active_turn_id.as_deref().unwrap_or(&state.thread_id);
        state.native_cursor = Some(format!("{cursor_root}:{}", state.notification_sequence));
        let cursor = state.native_cursor.clone();
        let native = sanitize_credential_value(&json!({
            "method": notification.method,
            "params": params,
        }));
        let mut event = HarnessEvent {
            event_type: format!("native.{}", notification.method.replace('/', ".")),
            durable: false,
            payload: json!({"native": native}),
            native_cursor: cursor.clone(),
        };
        let mut final_text = None;
        let mut usage = None;
        let mut terminal = false;
        match notification.method.as_str() {
            "turn/started" => {
                event.event_type = "turn.started".to_owned();
                event.durable = true;
                event.payload = json!({"turn": params.get("turn"), "native": native});
            }
            "item/agentMessage/delta" => {
                event.event_type = "assistant.text_chunk".to_owned();
                event.payload = json!({
                    "content": params.get("delta").and_then(Value::as_str).unwrap_or_default(),
                    "native": native,
                });
            }
            "item/reasoning/summaryTextDelta"
            | "item/reasoning/textDelta"
            | "item/reasoning/summaryPartAdded" => {
                event.event_type = "assistant.reasoning".to_owned();
                event.payload = json!({
                    "content": notification_delta(params),
                    "native": native,
                });
            }
            "item/plan/delta" => {
                event.event_type = "plan.updated".to_owned();
                event.payload = json!({
                    "content": notification_delta(params),
                    "native": native,
                });
            }
            "item/commandExecution/outputDelta" => {
                event.event_type = "tool.output".to_owned();
                event.payload = json!({
                    "content": notification_delta(params),
                    "native": native,
                });
            }
            "item/fileChange/outputDelta" => {
                event.event_type = "file.change".to_owned();
                event.payload = json!({
                    "content": notification_delta(params),
                    "native": native,
                });
            }
            "item/completed" => {
                let item = params.get("item").cloned().unwrap_or(Value::Null);
                match item.get("type").and_then(Value::as_str) {
                    Some("agentMessage") => {
                        final_text = item.get("text").and_then(Value::as_str).map(str::to_owned);
                        event.event_type = "assistant.final".to_owned();
                        event.durable = true;
                        event.payload = json!({"text": final_text, "native": native});
                    }
                    Some("reasoning") => {
                        event.event_type = "assistant.reasoning".to_owned();
                        event.durable = true;
                        event.payload = json!({"item": item, "native": native});
                    }
                    Some("plan") => {
                        event.event_type = "plan.updated".to_owned();
                        event.durable = true;
                        event.payload = json!({"item": item, "native": native});
                    }
                    Some("commandExecution") => {
                        event.event_type = "tool.completed".to_owned();
                        event.durable = true;
                        event.payload = json!({"item": item, "native": native});
                    }
                    Some("fileChange") => {
                        event.event_type = "file.change".to_owned();
                        event.durable = true;
                        event.payload = json!({"item": item, "native": native});
                    }
                    Some("mcpToolCall") | Some("dynamicToolCall") | Some("collabToolCall") => {
                        event.event_type = "tool.completed".to_owned();
                        event.durable = true;
                        event.payload = json!({"item": item, "native": native});
                    }
                    _ => {}
                }
            }
            "item/started" => {
                let item_type = params
                    .get("item")
                    .and_then(|item| item.get("type"))
                    .and_then(Value::as_str)
                    .unwrap_or("unknown");
                event.event_type = match item_type {
                    "commandExecution" => "tool.started",
                    "fileChange" => "file.change",
                    "agentMessage" => "assistant.started",
                    "reasoning" => "assistant.reasoning.started",
                    _ => "item.started",
                }
                .to_owned();
                event.durable = true;
                event.payload = json!({"item": params.get("item"), "native": native});
            }
            "thread/tokenUsage/updated" => {
                event.event_type = "usage.recorded".to_owned();
                event.durable = true;
                usage = params.get("tokenUsage").cloned();
                event.payload = json!({"usage": usage, "native": native});
            }
            "turn/diff/updated" => {
                event.event_type = "turn.diff.updated".to_owned();
                event.durable = true;
                event.payload = json!({"diff": params.get("diff"), "native": native});
            }
            "turn/plan/updated" => {
                event.event_type = "plan.updated".to_owned();
                event.durable = true;
                event.payload = json!({
                    "explanation": params.get("explanation"),
                    "plan": params.get("plan"),
                    "native": native,
                });
            }
            "turn/completed" => {
                let status = params
                    .get("turn")
                    .and_then(|turn| turn.get("status"))
                    .and_then(Value::as_str)
                    .or_else(|| params.get("status").and_then(Value::as_str));
                event.event_type = if status == Some("failed") {
                    "turn.failed"
                } else {
                    "turn.completed"
                }
                .to_owned();
                event.durable = true;
                event.payload = json!({"turn": params.get("turn"), "native": native});
                terminal = true;
            }
            "error" | "warning" | "configWarning" => {
                event.event_type = if notification.method == "error" {
                    "error"
                } else {
                    "warning"
                }
                .to_owned();
                event.durable = true;
                event.payload = json!({"details": params, "native": native});
            }
            "contextCompaction" => {
                event.event_type = "context.compacted".to_owned();
                event.durable = true;
            }
            "serverRequest/resolved" => {
                event.event_type = "interaction.resolved".to_owned();
                event.durable = true;
            }
            method if method.starts_with("thread/") => {
                event.event_type = format!("native.{}", method.replace('/', "."));
                event.durable = true;
            }
            method if method.starts_with("model/") => {
                event.event_type = format!("native.{}", method.replace('/', "."));
                event.durable = true;
            }
            _ => {}
        }
        event.payload = sanitize_credential_value(&event.payload);
        Ok(ProtocolEvent {
            event,
            native_cursor: cursor,
            final_text,
            usage,
            terminal,
        })
    }

    fn server_request_event(
        &self,
        request: &JsonRpcServerRequest,
    ) -> Result<HarnessEvent, HarnessAdapterError> {
        Ok(HarnessEvent {
            event_type: "interaction.required".to_owned(),
            durable: true,
            payload: json!({
                "native_request_id": request.id.as_value(),
                "native_method": request.method,
                "native_params": sanitize_credential_value(&request.params),
            }),
            native_cursor: None,
        })
    }

    fn encode_server_response(
        &self,
        _method: &str,
        response: &Value,
    ) -> Result<ServerResponse, HarnessAdapterError> {
        if response.get("error").is_some() {
            let error = response
                .get("error")
                .and_then(Value::as_object)
                .ok_or_else(|| {
                    HarnessAdapterError::InvalidRequest("native error must be an object".to_owned())
                })?;
            return Ok(ServerResponse::Error(super::json_rpc::JsonRpcErrorObject {
                code: error.get("code").and_then(Value::as_i64).unwrap_or(-32000),
                message: error
                    .get("message")
                    .and_then(Value::as_str)
                    .unwrap_or("OmniSolo rejected native request")
                    .to_owned(),
                data: error.get("data").cloned(),
            }));
        }
        Ok(ServerResponse::Result(
            response
                .get("result")
                .cloned()
                .unwrap_or_else(|| response.clone()),
        ))
    }
}

fn validated_routing_value(value: &str, field: &str) -> Result<String, HarnessAdapterError> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.chars().any(char::is_control) {
        return Err(HarnessAdapterError::InvalidRequest(format!(
            "{field} must be a non-empty printable value"
        )));
    }
    Ok(trimmed.to_owned())
}

fn validated_reasoning_effort(value: &str) -> Result<String, HarnessAdapterError> {
    let effort = validated_routing_value(value, "Codex reasoning effort")?.to_ascii_lowercase();
    match effort.as_str() {
        "none" | "minimal" | "low" | "medium" | "high" | "max" => Ok(effort),
        _ => Err(HarnessAdapterError::InvalidRequest(
            "Codex reasoning effort must be one of none, minimal, low, medium, high, or max"
                .to_owned(),
        )),
    }
}

fn notification_thread_id<'a>(params: &'a Value, state: &'a NativeTurnState) -> &'a str {
    params
        .get("threadId")
        .and_then(Value::as_str)
        .or_else(|| {
            params
                .get("thread")
                .and_then(|thread| thread.get("id"))
                .and_then(Value::as_str)
        })
        .or_else(|| {
            params
                .get("turn")
                .and_then(|turn| turn.get("threadId"))
                .and_then(Value::as_str)
        })
        .unwrap_or(&state.thread_id)
}

fn notification_turn_id(params: &Value) -> Option<&str> {
    params.get("turnId").and_then(Value::as_str).or_else(|| {
        params
            .get("turn")
            .and_then(|turn| turn.get("id"))
            .and_then(Value::as_str)
    })
}

fn notification_delta(params: &Value) -> String {
    ["delta", "text", "summaryText"]
        .iter()
        .find_map(|key| params.get(*key).and_then(Value::as_str))
        .unwrap_or_default()
        .to_owned()
}

fn content_text(content: &[ContentPart]) -> String {
    content
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text, .. } | ContentPart::ReasoningSummary { text, .. } => {
                Some(text.as_str())
            }
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn structured_summary(data: &std::collections::BTreeMap<String, Value>) -> String {
    data.get("summary")
        .and_then(Value::as_str)
        .or_else(|| data.get("text").and_then(Value::as_str))
        .unwrap_or("Portable OmniSolo context record")
        .to_owned()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use chrono::Utc;
    use sha2::{Digest, Sha256};
    use uuid::Uuid;

    use super::*;
    use crate::middleware::adapter::{OmniSoloHarnessAdapter, OmniSoloRunConfig};
    use crate::middleware::capsule::{
        HistoricalData, PortableContextCheckpoint, PortableMessage, PortableRecord,
        PortableStructuredRecord, PortableToolResult,
    };
    use crate::middleware::types::{ContentPart, MessageRole, MessageStatus};

    fn records_digest(records: &[PortableRecord]) -> String {
        Sha256::digest(serde_json::to_vec(records).unwrap())
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect()
    }

    fn portable_capsule_with_records() -> SessionCapsule {
        let session_id = Uuid::new_v4();
        let mut capsule = OmniSoloHarnessAdapter::start(
            OmniSoloRunConfig::new("tenant", "portable objective")
                .with_session_id(session_id)
                .with_task_id(Uuid::new_v4()),
        )
        .unwrap()
        .export_capsule("codex", Uuid::new_v4())
        .unwrap();
        let now = Utc::now();
        let content = vec![
            ContentPart::Text {
                text: "message".to_owned(),
                annotations: Vec::new(),
            },
            ContentPart::ReasoningSummary {
                text: "reasoning".to_owned(),
                annotations: Vec::new(),
            },
            ContentPart::ResourceLink {
                uri: "omnisolo://context".to_owned(),
                name: None,
                media_type: None,
            },
        ];
        capsule.records.extend([
            PortableRecord::Message(PortableMessage {
                message_id: Uuid::new_v4(),
                session_id,
                task_id: None,
                turn_id: None,
                role: MessageRole::User,
                actor: None,
                origin: None,
                phase: None,
                parent_message_id: None,
                correlation_id: None,
                status: MessageStatus::Settled,
                content: content.clone(),
                visible_to_user: true,
                redaction_state: None,
                created_at: now,
                settled_at: Some(now),
            }),
            PortableRecord::Message(PortableMessage {
                message_id: Uuid::new_v4(),
                session_id,
                task_id: None,
                turn_id: None,
                role: MessageRole::System,
                actor: None,
                origin: None,
                phase: None,
                parent_message_id: None,
                correlation_id: None,
                status: MessageStatus::Settled,
                content,
                visible_to_user: false,
                redaction_state: None,
                created_at: now,
                settled_at: Some(now),
            }),
            PortableRecord::Message(PortableMessage {
                message_id: Uuid::new_v4(),
                session_id,
                task_id: None,
                turn_id: None,
                role: MessageRole::User,
                actor: None,
                origin: None,
                phase: None,
                parent_message_id: None,
                correlation_id: None,
                status: MessageStatus::Settled,
                content: Vec::new(),
                visible_to_user: true,
                redaction_state: None,
                created_at: now,
                settled_at: Some(now),
            }),
            PortableRecord::ToolResult(PortableToolResult {
                tool_call_id: Uuid::new_v4(),
                content: vec![ContentPart::Text {
                    text: "tool".to_owned(),
                    annotations: Vec::new(),
                }],
                metadata: BTreeMap::new(),
            }),
            PortableRecord::Plan(PortableStructuredRecord {
                record_id: Uuid::new_v4(),
                kind: "plan".to_owned(),
                data: [("summary".to_owned(), json!("plan summary"))]
                    .into_iter()
                    .collect(),
            }),
            PortableRecord::Todo(PortableStructuredRecord {
                record_id: Uuid::new_v4(),
                kind: "todo".to_owned(),
                data: [("text".to_owned(), json!("todo text"))]
                    .into_iter()
                    .collect(),
            }),
            PortableRecord::Goal(PortableStructuredRecord {
                record_id: Uuid::new_v4(),
                kind: "goal".to_owned(),
                data: BTreeMap::new(),
            }),
            PortableRecord::HistoricalData(HistoricalData {
                source_kind: "legacy".to_owned(),
                summary: "historical".to_owned(),
            }),
            PortableRecord::ContextCheckpoint(PortableContextCheckpoint {
                summary: "checkpoint".to_owned(),
                selected_message_ids: Vec::new(),
                source_durable_ranges: Vec::new(),
                retained_ancestor_event_id: None,
                compaction_provenance: None,
                usage: BTreeMap::new(),
                integrity_digest: "digest".to_owned(),
            }),
            PortableRecord::Interaction(PortableStructuredRecord {
                record_id: Uuid::new_v4(),
                kind: "interaction".to_owned(),
                data: BTreeMap::new(),
            }),
        ]);
        capsule.record_digest = records_digest(&capsule.records);
        capsule.verify_integrity().unwrap();
        capsule
    }

    #[test]
    fn capsule_projection_covers_portable_record_fallbacks_and_content() {
        let items = CodexAppServerV2Codec::new()
            .capsule_items(&portable_capsule_with_records())
            .unwrap();
        let record_types = items
            .iter()
            .filter_map(|item| item["metadata"]["record_type"].as_str())
            .collect::<Vec<_>>();
        assert!(record_types.contains(&"task"));
        assert!(record_types.contains(&"message"));
        assert!(record_types.contains(&"tool_result"));
        assert!(record_types.contains(&"plan"));
        assert!(record_types.contains(&"goal"));
        assert!(record_types.contains(&"historical"));
        assert!(record_types.contains(&"context_checkpoint"));
        assert!(items.iter().all(|item| item["role"] == "user"));

        let mut empty = portable_capsule_with_records();
        empty.records = vec![PortableRecord::Message(PortableMessage {
            message_id: Uuid::new_v4(),
            session_id: empty.manifest.session_id,
            task_id: None,
            turn_id: None,
            role: MessageRole::System,
            actor: None,
            origin: None,
            phase: None,
            parent_message_id: None,
            correlation_id: None,
            status: MessageStatus::Settled,
            content: Vec::new(),
            visible_to_user: false,
            redaction_state: None,
            created_at: Utc::now(),
            settled_at: None,
        })];
        empty.record_digest = records_digest(&empty.records);
        let fallback = CodexAppServerV2Codec::new().capsule_items(&empty).unwrap();
        assert_eq!(fallback.len(), 1);
        assert_eq!(fallback[0]["metadata"]["record_type"], "empty");
    }

    #[test]
    fn shared_credential_sanitizer_and_summary_defaults() {
        let value = sanitize_credential_value(&json!({
            "safe": [{"password": "secret", "nested": true}],
            "number": 1,
        }));
        assert!(value["safe"][0].get("password").is_none());
        assert_eq!(value["safe"][0]["nested"], true);
        assert_eq!(value["number"], 1);

        let content = vec![
            ContentPart::Text {
                text: "one".to_owned(),
                annotations: Vec::new(),
            },
            ContentPart::ReasoningSummary {
                text: "two".to_owned(),
                annotations: Vec::new(),
            },
        ];
        assert_eq!(content_text(&content), "one\ntwo");
        assert_eq!(
            structured_summary(&BTreeMap::new()),
            "Portable OmniSolo context record"
        );
    }
}
