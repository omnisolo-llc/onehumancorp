use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, Command};
use tokio::sync::{Mutex, mpsc, oneshot};
use tokio::time::timeout;
use uuid::Uuid;

use super::harness::{HarnessAdapterError, HarnessEvent};
use super::types::sanitize_credential_value;

pub const OPENHARNESS_MAX_FRAME_BYTES: usize = 1024 * 1024;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpenHarnessResolvedModel {
    pub provider: String,
    pub model_id: String,
    pub base_url: String,
    pub reasoning_effort: String,
    pub api_dialect: String,
    pub context_window_tokens: u64,
    pub max_output_tokens: u64,
    pub capabilities: Vec<String>,
    pub binding_revision: String,
    pub binding_digest: String,
}

impl OpenHarnessResolvedModel {
    fn legacy(model_id: String, base_url: String) -> Self {
        Self {
            provider: "openai".to_owned(),
            model_id,
            base_url,
            reasoning_effort: "max".to_owned(),
            api_dialect: "openai_chat_completions".to_owned(),
            context_window_tokens: 200_000,
            max_output_tokens: 16_384,
            capabilities: vec![
                "text".to_owned(),
                "tools".to_owned(),
                "streaming".to_owned(),
            ],
            binding_revision: "legacy-v1".to_owned(),
            binding_digest: "unresolved".to_owned(),
        }
    }

    fn validate(&self) -> Result<(), OpenHarnessCodecError> {
        if self.provider != "openai" {
            return Err(OpenHarnessCodecError(
                "OpenHarness resolved provider must be openai".to_owned(),
            ));
        }
        if [
            &self.model_id,
            &self.reasoning_effort,
            &self.api_dialect,
            &self.binding_revision,
            &self.binding_digest,
        ]
        .into_iter()
        .any(|value| value.trim().is_empty())
            || self.context_window_tokens == 0
            || self.max_output_tokens == 0
            || self.capabilities.is_empty()
            || self
                .capabilities
                .iter()
                .any(|value| value.trim().is_empty())
        {
            return Err(OpenHarnessCodecError(
                "OpenHarness resolved model contract is incomplete".to_owned(),
            ));
        }
        validate_base_url(&self.base_url)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
// Keep owned payloads in this established public protocol representation.
#[allow(clippy::large_enum_variant)]
pub enum OpenHarnessCommand {
    CreateSession {
        id: String,
    },
    ResumeSession {
        id: String,
        session_id: String,
    },
    DeleteSession {
        id: String,
        session_id: String,
    },
    Prompt {
        id: String,
        session_id: String,
        prompt: String,
        resolved_model: OpenHarnessResolvedModel,
        permission_mode: String,
        cwd: String,
    },
    Steer {
        id: String,
        session_id: String,
        message: String,
    },
    Cancel {
        id: String,
        session_id: String,
    },
    Shutdown {
        id: String,
    },
}

impl OpenHarnessCommand {
    pub fn create_session(id: impl Into<String>) -> Self {
        Self::CreateSession { id: id.into() }
    }

    pub fn resume_session(id: impl Into<String>, session_id: impl Into<String>) -> Self {
        Self::ResumeSession {
            id: id.into(),
            session_id: session_id.into(),
        }
    }

    pub fn delete_session(id: impl Into<String>, session_id: impl Into<String>) -> Self {
        Self::DeleteSession {
            id: id.into(),
            session_id: session_id.into(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn prompt(
        id: impl Into<String>,
        session_id: impl Into<String>,
        prompt: impl Into<String>,
        model: impl Into<String>,
        base_url: impl Into<String>,
        permission_mode: impl Into<String>,
        cwd: impl Into<String>,
    ) -> Self {
        let resolved_model = OpenHarnessResolvedModel::legacy(model.into(), base_url.into());
        Self::prompt_with_resolved_model(
            id,
            session_id,
            prompt,
            resolved_model,
            permission_mode,
            cwd,
        )
    }

    pub fn prompt_with_resolved_model(
        id: impl Into<String>,
        session_id: impl Into<String>,
        prompt: impl Into<String>,
        resolved_model: OpenHarnessResolvedModel,
        permission_mode: impl Into<String>,
        cwd: impl Into<String>,
    ) -> Self {
        Self::Prompt {
            id: id.into(),
            session_id: session_id.into(),
            prompt: prompt.into(),
            resolved_model,
            permission_mode: permission_mode.into(),
            cwd: cwd.into(),
        }
    }

    pub fn steer(
        id: impl Into<String>,
        session_id: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::Steer {
            id: id.into(),
            session_id: session_id.into(),
            message: message.into(),
        }
    }

    pub fn cancel(id: impl Into<String>, session_id: impl Into<String>) -> Self {
        Self::Cancel {
            id: id.into(),
            session_id: session_id.into(),
        }
    }

    pub fn shutdown(id: impl Into<String>) -> Self {
        Self::Shutdown { id: id.into() }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::CreateSession { id }
            | Self::ResumeSession { id, .. }
            | Self::DeleteSession { id, .. }
            | Self::Prompt { id, .. }
            | Self::Steer { id, .. }
            | Self::Cancel { id, .. }
            | Self::Shutdown { id } => id,
        }
    }

    pub fn command_type(&self) -> &'static str {
        match self {
            Self::CreateSession { .. } => "create_session",
            Self::ResumeSession { .. } => "resume_session",
            Self::DeleteSession { .. } => "delete_session",
            Self::Prompt { .. } => "prompt",
            Self::Steer { .. } => "steer",
            Self::Cancel { .. } => "cancel",
            Self::Shutdown { .. } => "shutdown",
        }
    }

    fn prompt_correlation(&self) -> Option<(&str, &str)> {
        match self {
            Self::Prompt { id, session_id, .. } => Some((id, session_id)),
            _ => None,
        }
    }

    fn expects_session(&self) -> bool {
        matches!(
            self,
            Self::CreateSession { .. } | Self::ResumeSession { .. } | Self::DeleteSession { .. }
        )
    }

    fn validate(&self) -> Result<(), OpenHarnessCodecError> {
        let id = match self {
            Self::CreateSession { id }
            | Self::ResumeSession { id, .. }
            | Self::DeleteSession { id, .. }
            | Self::Prompt { id, .. }
            | Self::Steer { id, .. }
            | Self::Cancel { id, .. }
            | Self::Shutdown { id } => id,
        };
        if id.trim().is_empty() {
            return Err(OpenHarnessCodecError(
                "OpenHarness command id must be non-empty".to_owned(),
            ));
        }
        match self {
            Self::ResumeSession { session_id, .. }
            | Self::DeleteSession { session_id, .. }
            | Self::Cancel { session_id, .. }
                if !safe_native_session_id(session_id) =>
            {
                Err(OpenHarnessCodecError(
                    "OpenHarness command session_id is unsafe".to_owned(),
                ))
            }
            Self::Steer {
                session_id,
                message,
                ..
            } if !safe_native_session_id(session_id) || message.trim().is_empty() => {
                Err(OpenHarnessCodecError(
                    "OpenHarness steer session_id and message must be non-empty".to_owned(),
                ))
            }
            Self::Prompt {
                session_id,
                prompt,
                resolved_model,
                permission_mode,
                cwd,
                ..
            } => {
                if !safe_native_session_id(session_id)
                    || [prompt, permission_mode, cwd]
                        .into_iter()
                        .any(|value| value.trim().is_empty())
                {
                    return Err(OpenHarnessCodecError(
                        "OpenHarness prompt fields must be non-empty".to_owned(),
                    ));
                }
                if !matches!(
                    permission_mode.as_str(),
                    "default" | "accept_edits" | "plan" | "bypass"
                ) {
                    return Err(OpenHarnessCodecError(
                        "OpenHarness permission_mode is unsupported".to_owned(),
                    ));
                }
                resolved_model.validate()
            }
            _ => Ok(()),
        }
    }
}

fn safe_native_session_id(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty()
        && bytes.len() <= 128
        && bytes[0].is_ascii_alphanumeric()
        && bytes
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.'))
}

fn validate_base_url(base_url: &str) -> Result<(), OpenHarnessCodecError> {
    let base_url = base_url.trim();
    let Some(authority_start) = base_url
        .strip_prefix("https://")
        .or_else(|| base_url.strip_prefix("http://"))
    else {
        return Err(OpenHarnessCodecError(
            "OpenHarness base_url must use HTTP(S)".to_owned(),
        ));
    };
    let authority = authority_start.split('/').next().unwrap_or_default();
    if authority.is_empty()
        || authority.contains('@')
        || base_url.contains('?')
        || base_url.contains('#')
        || base_url.chars().any(char::is_whitespace)
    {
        return Err(OpenHarnessCodecError(
            "OpenHarness base_url must not contain credentials, query parameters, fragments, or whitespace"
                .to_owned(),
        ));
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OpenHarnessEvent {
    Ready {
        protocol_version: u32,
        sdk_version: String,
    },
    Ack {
        id: String,
        command: String,
    },
    Session {
        id: String,
        command: String,
        session_id: String,
    },
    SessionDeleted {
        id: String,
        command: String,
        session_id: String,
    },
    Text {
        request_id: String,
        session_id: String,
        text: String,
        is_partial: bool,
    },
    System {
        request_id: String,
        session_id: String,
        event: String,
        data: Value,
    },
    Downgrade {
        request_id: String,
        session_id: String,
        field: String,
        requested: Value,
        applied: Value,
        reason: String,
        binding_revision: String,
        binding_digest: String,
    },
    Compaction {
        request_id: String,
        session_id: String,
        tokens_before: u64,
        tokens_after: u64,
        summary: String,
    },
    ToolUse {
        request_id: String,
        session_id: String,
        tool_use_id: String,
        name: String,
        args: Value,
    },
    ToolResult {
        request_id: String,
        session_id: String,
        tool_use_id: String,
        content: String,
        is_error: bool,
        display: Option<String>,
    },
    Result {
        request_id: String,
        session_id: String,
        text: String,
        turns: u64,
        tool_calls: u64,
        total_tokens: u64,
        total_cost: f64,
        stop_reason: String,
    },
    Cancelled {
        request_id: String,
        session_id: String,
    },
    Error {
        request_id: String,
        session_id: Option<String>,
        code: String,
        message: String,
        terminal: bool,
    },
}

impl OpenHarnessEvent {
    pub fn terminal(&self) -> bool {
        match self {
            Self::Result { .. } | Self::Cancelled { .. } => true,
            Self::Error { terminal, .. } => *terminal,
            _ => false,
        }
    }

    fn validate(&self) -> Result<(), OpenHarnessCodecError> {
        match self {
            Self::Ready {
                protocol_version,
                sdk_version,
            } if *protocol_version != 1 || sdk_version != "0.6.0" => {
                return Err(OpenHarnessCodecError(
                    "OpenHarness ready handshake is incompatible".to_owned(),
                ));
            }
            Self::Ack { id, command } if id.trim().is_empty() || command.trim().is_empty() => {
                return Err(OpenHarnessCodecError(
                    "OpenHarness acknowledgment id and command must be non-empty".to_owned(),
                ));
            }
            Self::Session {
                id,
                command,
                session_id,
            } if id.trim().is_empty()
                || command.trim().is_empty()
                || session_id.trim().is_empty() =>
            {
                return Err(OpenHarnessCodecError(
                    "OpenHarness session event fields must be non-empty".to_owned(),
                ));
            }
            Self::SessionDeleted {
                id,
                command,
                session_id,
            } if id.trim().is_empty()
                || command != "delete_session"
                || !safe_native_session_id(session_id) =>
            {
                return Err(OpenHarnessCodecError(
                    "OpenHarness deleted-session event fields are invalid".to_owned(),
                ));
            }
            Self::Error {
                request_id,
                session_id,
                code,
                message,
                ..
            } if request_id.trim().is_empty()
                || session_id
                    .as_ref()
                    .is_some_and(|value| value.trim().is_empty())
                || code.trim().is_empty()
                || message.trim().is_empty() =>
            {
                return Err(OpenHarnessCodecError(
                    "OpenHarness error event fields must be non-empty".to_owned(),
                ));
            }
            _ => {}
        }
        let (request_id, session_id) = match self {
            Self::Text {
                request_id,
                session_id,
                ..
            }
            | Self::System {
                request_id,
                session_id,
                ..
            }
            | Self::Downgrade {
                request_id,
                session_id,
                ..
            }
            | Self::Compaction {
                request_id,
                session_id,
                ..
            }
            | Self::ToolUse {
                request_id,
                session_id,
                ..
            }
            | Self::ToolResult {
                request_id,
                session_id,
                ..
            }
            | Self::Result {
                request_id,
                session_id,
                ..
            }
            | Self::Cancelled {
                request_id,
                session_id,
            } => (request_id, session_id),
            Self::Ready { .. }
            | Self::Ack { .. }
            | Self::Session { .. }
            | Self::SessionDeleted { .. }
            | Self::Error { .. } => return Ok(()),
        };
        if request_id.trim().is_empty() || session_id.trim().is_empty() {
            return Err(OpenHarnessCodecError(
                "OpenHarness event request_id and session_id must be non-empty".to_owned(),
            ));
        }
        match self {
            Self::System { event, data, .. } if event.trim().is_empty() || !data.is_object() => {
                Err(OpenHarnessCodecError(
                    "OpenHarness system event type must be non-empty and data must be an object"
                        .to_owned(),
                ))
            }
            Self::Downgrade {
                field,
                reason,
                binding_revision,
                binding_digest,
                ..
            } if field.trim().is_empty()
                || reason.trim().is_empty()
                || binding_revision.trim().is_empty()
                || binding_digest.trim().is_empty() =>
            {
                Err(OpenHarnessCodecError(
                    "OpenHarness downgrade metadata must be non-empty".to_owned(),
                ))
            }
            Self::ToolUse {
                tool_use_id, name, ..
            } if tool_use_id.trim().is_empty() || name.trim().is_empty() => {
                Err(OpenHarnessCodecError(
                    "OpenHarness tool event identifiers must be non-empty".to_owned(),
                ))
            }
            Self::ToolResult { tool_use_id, .. } if tool_use_id.trim().is_empty() => {
                Err(OpenHarnessCodecError(
                    "OpenHarness tool result identifier must be non-empty".to_owned(),
                ))
            }
            Self::Result { stop_reason, .. } if stop_reason.trim().is_empty() => {
                Err(OpenHarnessCodecError(
                    "OpenHarness result stop_reason must be non-empty".to_owned(),
                ))
            }
            _ => Ok(()),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct OpenHarnessCodec;

impl OpenHarnessCodec {
    pub fn encode_command_line(
        &self,
        command: &OpenHarnessCommand,
    ) -> Result<Vec<u8>, OpenHarnessCodecError> {
        command.validate()?;
        let mut wire = serde_json::to_vec(command).map_err(|_| {
            OpenHarnessCodecError("OpenHarness command could not be serialized".to_owned())
        })?;
        if wire.len() + 1 > OPENHARNESS_MAX_FRAME_BYTES {
            return Err(OpenHarnessCodecError(
                "OpenHarness command frame exceeds the JSONL limit".to_owned(),
            ));
        }
        wire.push(b'\n');
        Ok(wire)
    }

    pub fn decode_event_line(&self, line: &str) -> Result<OpenHarnessEvent, OpenHarnessCodecError> {
        if line.len() + 1 > OPENHARNESS_MAX_FRAME_BYTES {
            return Err(OpenHarnessCodecError(
                "OpenHarness event frame exceeds the JSONL limit".to_owned(),
            ));
        }
        let event: OpenHarnessEvent = serde_json::from_str(line).map_err(|error| {
            OpenHarnessCodecError(format!("malformed OpenHarness JSONL frame: {error}"))
        })?;
        event.validate()?;
        Ok(event)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenHarnessCodecError(String);

impl std::fmt::Display for OpenHarnessCodecError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for OpenHarnessCodecError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenHarnessEventCorrelation {
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub turn_id: Option<Uuid>,
    pub attempt_id: String,
    pub native_session_id: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpenHarnessDecodedEvent {
    pub event: HarnessEvent,
    pub final_text: Option<String>,
    pub usage: Option<Value>,
    pub terminal: bool,
}

#[derive(Clone, Debug)]
pub struct OpenHarnessEventDecoder {
    correlation: OpenHarnessEventCorrelation,
    sequence: u64,
}

impl OpenHarnessEventDecoder {
    pub fn new(correlation: OpenHarnessEventCorrelation) -> Self {
        Self {
            correlation,
            sequence: 0,
        }
    }

    pub fn decode(
        &mut self,
        native: OpenHarnessEvent,
    ) -> Result<OpenHarnessDecodedEvent, HarnessAdapterError> {
        if matches!(
            native,
            OpenHarnessEvent::Ready { .. }
                | OpenHarnessEvent::Ack { .. }
                | OpenHarnessEvent::Session { .. }
                | OpenHarnessEvent::SessionDeleted { .. }
        ) {
            return Err(invalid_response(
                "OpenHarness command response cannot be decoded as a streamed event",
            ));
        }
        self.sequence = self.sequence.saturating_add(1);
        let cursor = format!("{}:{}", self.correlation.native_session_id, self.sequence);
        let native_json = serde_json::to_value(&native).map_err(HarnessAdapterError::Json)?;
        let mut final_text = None;
        let mut usage = None;
        let terminal = native.terminal();
        let (event_type, durable, mut payload) = match &native {
            OpenHarnessEvent::Text {
                text, is_partial, ..
            } => (
                "assistant.text_chunk",
                !*is_partial,
                Map::from_iter([
                    ("content".to_owned(), Value::String(text.clone())),
                    ("is_partial".to_owned(), Value::Bool(*is_partial)),
                ]),
            ),
            OpenHarnessEvent::System { event, data, .. } => {
                let mut payload = data.as_object().cloned().unwrap_or_default();
                payload.insert("event".to_owned(), Value::String(event.clone()));
                (
                    if event == "session_start" {
                        "turn.started"
                    } else {
                        "native.openharness.system"
                    },
                    event == "session_start",
                    payload,
                )
            }
            OpenHarnessEvent::Downgrade {
                field,
                requested,
                applied,
                reason,
                binding_revision,
                binding_digest,
                ..
            } => (
                "model.contract_downgraded",
                true,
                Map::from_iter([
                    ("field".to_owned(), Value::String(field.clone())),
                    ("requested".to_owned(), requested.clone()),
                    ("applied".to_owned(), applied.clone()),
                    ("reason".to_owned(), Value::String(reason.clone())),
                    (
                        "binding_revision".to_owned(),
                        Value::String(binding_revision.clone()),
                    ),
                    (
                        "binding_digest".to_owned(),
                        Value::String(binding_digest.clone()),
                    ),
                ]),
            ),
            OpenHarnessEvent::Compaction {
                tokens_before,
                tokens_after,
                summary,
                ..
            } => (
                "session.compacted",
                true,
                Map::from_iter([
                    ("tokens_before".to_owned(), json!(tokens_before)),
                    ("tokens_after".to_owned(), json!(tokens_after)),
                    ("summary".to_owned(), Value::String(summary.clone())),
                ]),
            ),
            OpenHarnessEvent::ToolUse {
                tool_use_id,
                name,
                args,
                ..
            } => (
                "tool.started",
                true,
                Map::from_iter([
                    ("tool_use_id".to_owned(), Value::String(tool_use_id.clone())),
                    ("name".to_owned(), Value::String(name.clone())),
                    ("args".to_owned(), args.clone()),
                ]),
            ),
            OpenHarnessEvent::ToolResult {
                tool_use_id,
                content,
                is_error,
                display,
                ..
            } => (
                if *is_error {
                    "tool.failed"
                } else {
                    "tool.completed"
                },
                true,
                Map::from_iter([
                    ("tool_use_id".to_owned(), Value::String(tool_use_id.clone())),
                    ("content".to_owned(), Value::String(content.clone())),
                    ("is_error".to_owned(), Value::Bool(*is_error)),
                    (
                        "display".to_owned(),
                        display.clone().map(Value::String).unwrap_or(Value::Null),
                    ),
                ]),
            ),
            OpenHarnessEvent::Result {
                text,
                turns,
                tool_calls,
                total_tokens,
                total_cost,
                stop_reason,
                ..
            } => {
                final_text = Some(text.clone());
                let result_usage = json!({
                    "turns": turns,
                    "tool_calls": tool_calls,
                    "total_tokens": total_tokens,
                    "total_cost": total_cost,
                });
                usage = Some(result_usage.clone());
                (
                    "turn.completed",
                    true,
                    Map::from_iter([
                        ("text".to_owned(), Value::String(text.clone())),
                        ("usage".to_owned(), result_usage),
                        ("stop_reason".to_owned(), Value::String(stop_reason.clone())),
                    ]),
                )
            }
            OpenHarnessEvent::Cancelled { .. } => ("turn.cancelled", true, Map::new()),
            OpenHarnessEvent::Error {
                code,
                message,
                terminal,
                ..
            } => (
                if *terminal {
                    "turn.failed"
                } else {
                    "native.openharness.error"
                },
                *terminal,
                Map::from_iter([
                    ("code".to_owned(), Value::String(code.clone())),
                    ("message".to_owned(), Value::String(message.clone())),
                ]),
            ),
            OpenHarnessEvent::Ready { .. }
            | OpenHarnessEvent::Ack { .. }
            | OpenHarnessEvent::Session { .. }
            | OpenHarnessEvent::SessionDeleted { .. } => unreachable!(),
        };
        payload.insert(
            "session_id".to_owned(),
            Value::String(self.correlation.session_id.to_string()),
        );
        payload.insert(
            "task_id".to_owned(),
            self.correlation
                .task_id
                .map(|id| Value::String(id.to_string()))
                .unwrap_or(Value::Null),
        );
        payload.insert(
            "turn_id".to_owned(),
            self.correlation
                .turn_id
                .map(|id| Value::String(id.to_string()))
                .unwrap_or(Value::Null),
        );
        payload.insert(
            "attempt_id".to_owned(),
            Value::String(self.correlation.attempt_id.clone()),
        );
        payload.insert(
            "native_session_id".to_owned(),
            Value::String(self.correlation.native_session_id.clone()),
        );
        payload.insert("native".to_owned(), native_json);
        let payload = sanitize_credential_value(&Value::Object(payload));
        Ok(OpenHarnessDecodedEvent {
            event: HarnessEvent {
                event_type: event_type.to_owned(),
                durable,
                payload,
                native_cursor: Some(cursor),
            },
            final_text,
            usage: usage.map(|value| sanitize_credential_value(&value)),
            terminal,
        })
    }
}

#[derive(Clone)]
pub struct OpenHarnessProcessConfig {
    pub executable: String,
    pub args: Vec<String>,
    pub environment: BTreeMap<String, String>,
    pub request_timeout: Duration,
}

impl std::fmt::Debug for OpenHarnessProcessConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OpenHarnessProcessConfig")
            .field("executable", &self.executable)
            .field("arg_count", &self.args.len())
            .field(
                "environment_keys",
                &self.environment.keys().collect::<Vec<_>>(),
            )
            .field("request_timeout", &self.request_timeout)
            .finish()
    }
}

struct PendingCommand {
    expected_command: String,
    expects_session: bool,
    sender: oneshot::Sender<Result<OpenHarnessEvent, HarnessAdapterError>>,
}

#[derive(Clone)]
enum RuntimeFailure {
    ProcessExited,
    InvalidResponse(String),
    Remote(String),
}

impl RuntimeFailure {
    fn error(&self) -> HarnessAdapterError {
        match self {
            Self::ProcessExited => HarnessAdapterError::ProcessExited,
            Self::InvalidResponse(message) => invalid_response(message.clone()),
            Self::Remote(message) => HarnessAdapterError::Remote(message.clone()),
        }
    }
}

struct OpenHarnessRuntimeState {
    writer: Mutex<Option<ChildStdin>>,
    process: Mutex<Child>,
    pending: Mutex<HashMap<String, PendingCommand>>,
    events_tx: mpsc::Sender<Result<OpenHarnessEvent, HarnessAdapterError>>,
    events_rx: Mutex<mpsc::Receiver<Result<OpenHarnessEvent, HarnessAdapterError>>>,
    active_prompt: Mutex<Option<(String, String)>>,
    fatal: Mutex<Option<RuntimeFailure>>,
    request_timeout: Duration,
    shutting_down: AtomicBool,
    closed: AtomicBool,
    private_home: PrivateHome,
}

impl Drop for OpenHarnessRuntimeState {
    fn drop(&mut self) {
        let _ = self.process.get_mut().start_kill();
        self.private_home.cleanup();
    }
}

struct PrivateHome {
    root: PathBuf,
    home: PathBuf,
}

impl PrivateHome {
    fn create() -> Result<Self, HarnessAdapterError> {
        let root = PathBuf::from("/tmp").join(format!("omnisolo-openharness-{}", Uuid::new_v4()));
        let home = root.join("home");
        fs::create_dir_all(&home).map_err(HarnessAdapterError::Io)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&root, fs::Permissions::from_mode(0o700))
                .map_err(HarnessAdapterError::Io)?;
            fs::set_permissions(&home, fs::Permissions::from_mode(0o700))
                .map_err(HarnessAdapterError::Io)?;
        }
        Ok(Self { root, home })
    }

    fn cleanup(&self) {
        if self.root.parent() == Some(Path::new("/tmp"))
            && self
                .root
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("omnisolo-openharness-"))
        {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}

impl Drop for PrivateHome {
    fn drop(&mut self) {
        self.cleanup();
    }
}

#[derive(Clone)]
pub struct OpenHarnessRuntime {
    inner: Arc<OpenHarnessRuntimeState>,
}

pub struct OpenHarnessPromptStream {
    runtime: OpenHarnessRuntime,
    session_id: String,
    armed: bool,
}

impl OpenHarnessPromptStream {
    pub async fn next_event(&mut self) -> Result<OpenHarnessEvent, HarnessAdapterError> {
        let event = self.runtime.next_event().await?;
        if event.terminal() {
            self.armed = false;
        }
        Ok(event)
    }

    pub async fn cancel(mut self) -> Result<(), HarnessAdapterError> {
        self.armed = false;
        self.runtime
            .request(OpenHarnessCommand::cancel(
                format!("omnisolo-stream-cancel-{}", Uuid::new_v4()),
                self.session_id.clone(),
            ))
            .await
            .map(|_| ())
    }
}

impl Drop for OpenHarnessPromptStream {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        self.armed = false;
        let runtime = self.runtime.clone();
        let session_id = self.session_id.clone();
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            handle.spawn(async move {
                let _ = runtime
                    .request(OpenHarnessCommand::cancel(
                        format!("omnisolo-stream-drop-cancel-{}", Uuid::new_v4()),
                        session_id,
                    ))
                    .await;
            });
        }
    }
}

impl std::fmt::Debug for OpenHarnessRuntime {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OpenHarnessRuntime")
            .field("closed", &self.inner.closed.load(Ordering::Acquire))
            .finish()
    }
}

impl OpenHarnessRuntime {
    pub async fn spawn(config: OpenHarnessProcessConfig) -> Result<Self, HarnessAdapterError> {
        if config.executable.trim().is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(
                "OpenHarness executable is empty".to_owned(),
            ));
        }
        let private_home = PrivateHome::create()?;
        let mut command = Command::new(&config.executable);
        command
            .args(&config.args)
            .env_clear()
            .envs(&config.environment)
            .env("HOME", &private_home.home)
            .env("OPENHARNESS_SESSION_HOME", &private_home.home)
            .env("XDG_CONFIG_HOME", private_home.home.join(".config"))
            .env("XDG_CACHE_HOME", private_home.home.join(".cache"))
            .env("XDG_DATA_HOME", private_home.home.join(".local/share"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        if !config.environment.contains_key("PATH")
            && let Some(path) = std::env::var_os("PATH")
        {
            command.env("PATH", path);
        }
        let mut process = command.spawn().map_err(HarnessAdapterError::Spawn)?;
        let writer = process
            .stdin
            .take()
            .ok_or_else(|| invalid_response("OpenHarness process did not expose stdin"))?;
        let stdout = process
            .stdout
            .take()
            .ok_or_else(|| invalid_response("OpenHarness process did not expose stdout"))?;
        let mut stdout = BufReader::new(stdout);
        let ready_line = match timeout(config.request_timeout, read_bounded_line(&mut stdout)).await
        {
            Ok(Ok(Some(line))) => line,
            Ok(Ok(None)) => return Err(HarnessAdapterError::ProcessExited),
            Ok(Err(error)) => return Err(invalid_response(error.to_string())),
            Err(_) => return Err(HarnessAdapterError::Timeout),
        };
        let ready = OpenHarnessCodec
            .decode_event_line(&ready_line)
            .map_err(|error| {
                invalid_response(format!("invalid OpenHarness ready handshake: {error}"))
            })?;
        match ready {
            OpenHarnessEvent::Ready {
                protocol_version: 1,
                ref sdk_version,
            } if sdk_version == "0.6.0" => {}
            OpenHarnessEvent::Error { code, message, .. } => {
                return Err(HarnessAdapterError::Remote(format!("{code}: {message}")));
            }
            _ => {
                return Err(invalid_response(
                    "OpenHarness process did not emit ready handshake",
                ));
            }
        }
        let (events_tx, events_rx) = mpsc::channel(256);
        let inner = Arc::new(OpenHarnessRuntimeState {
            writer: Mutex::new(Some(writer)),
            process: Mutex::new(process),
            pending: Mutex::new(HashMap::new()),
            events_tx,
            events_rx: Mutex::new(events_rx),
            active_prompt: Mutex::new(None),
            fatal: Mutex::new(None),
            request_timeout: config.request_timeout,
            shutting_down: AtomicBool::new(false),
            closed: AtomicBool::new(false),
            private_home,
        });
        tokio::spawn(read_openharness_stdout(stdout, Arc::downgrade(&inner)));
        Ok(Self { inner })
    }

    pub async fn prompt(
        &self,
        command: OpenHarnessCommand,
    ) -> Result<OpenHarnessEvent, HarnessAdapterError> {
        let Some((request_id, session_id)) = command.prompt_correlation() else {
            return Err(HarnessAdapterError::InvalidRequest(
                "OpenHarness prompt requires a prompt command".to_owned(),
            ));
        };
        {
            let mut active = self.inner.active_prompt.lock().await;
            if active.is_some() {
                return Err(HarnessAdapterError::InvalidRequest(
                    "OpenHarness already has an active prompt".to_owned(),
                ));
            }
            *active = Some((request_id.to_owned(), session_id.to_owned()));
        }
        let response = self.request(command).await;
        if response.is_err() {
            self.inner.active_prompt.lock().await.take();
        }
        response
    }

    pub async fn prompt_stream(
        &self,
        command: OpenHarnessCommand,
    ) -> Result<OpenHarnessPromptStream, HarnessAdapterError> {
        let Some((_, session_id)) = command.prompt_correlation() else {
            return Err(HarnessAdapterError::InvalidRequest(
                "OpenHarness prompt stream requires a prompt command".to_owned(),
            ));
        };
        let session_id = session_id.to_owned();
        self.prompt(command).await?;
        Ok(OpenHarnessPromptStream {
            runtime: self.clone(),
            session_id,
            armed: true,
        })
    }

    pub async fn request(
        &self,
        command: OpenHarnessCommand,
    ) -> Result<OpenHarnessEvent, HarnessAdapterError> {
        if let Some(failure) = self.inner.fatal.lock().await.clone() {
            return Err(failure.error());
        }
        if self.inner.closed.load(Ordering::Acquire) {
            return Err(HarnessAdapterError::ProcessExited);
        }
        let codec = OpenHarnessCodec;
        let wire = codec
            .encode_command_line(&command)
            .map_err(|error| HarnessAdapterError::InvalidRequest(error.to_string()))?;
        let id = command.id().to_owned();
        let (sender, receiver) = oneshot::channel();
        {
            let mut pending = self.inner.pending.lock().await;
            if pending.contains_key(&id) {
                return Err(HarnessAdapterError::InvalidRequest(format!(
                    "duplicate OpenHarness command id: {id}"
                )));
            }
            pending.insert(
                id.clone(),
                PendingCommand {
                    expected_command: command.command_type().to_owned(),
                    expects_session: command.expects_session(),
                    sender,
                },
            );
        }
        let write_result = async {
            let mut writer = self.inner.writer.lock().await;
            let writer = writer.as_mut().ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::BrokenPipe,
                    "OpenHarness stdin is closed",
                )
            })?;
            writer.write_all(&wire).await?;
            writer.flush().await
        }
        .await;
        if let Err(error) = write_result {
            self.inner.pending.lock().await.remove(&id);
            return Err(if error.kind() == std::io::ErrorKind::BrokenPipe {
                HarnessAdapterError::ProcessExited
            } else {
                HarnessAdapterError::Io(error)
            });
        }
        match timeout(self.inner.request_timeout, receiver).await {
            Ok(Ok(result)) => result,
            Ok(Err(_)) => Err(HarnessAdapterError::ProcessExited),
            Err(_) => {
                self.inner.pending.lock().await.remove(&id);
                Err(HarnessAdapterError::Timeout)
            }
        }
    }

    pub async fn next_event(&self) -> Result<OpenHarnessEvent, HarnessAdapterError> {
        match timeout(self.inner.request_timeout, async {
            self.inner.events_rx.lock().await.recv().await
        })
        .await
        {
            Ok(Some(result)) => result,
            Ok(None) => Err(HarnessAdapterError::ProcessExited),
            Err(_) => {
                if let Some((_, session_id)) = self.inner.active_prompt.lock().await.clone() {
                    let _ = self
                        .request(OpenHarnessCommand::cancel(
                            format!("omnisolo-timeout-cancel-{}", Uuid::new_v4()),
                            session_id,
                        ))
                        .await;
                }
                Err(HarnessAdapterError::Timeout)
            }
        }
    }

    pub async fn shutdown(&self) -> Result<(), HarnessAdapterError> {
        if self.inner.closed.load(Ordering::Acquire) {
            return Ok(());
        }
        self.inner.shutting_down.store(true, Ordering::Release);
        let command_result = if self.inner.fatal.lock().await.is_none() {
            self.request(OpenHarnessCommand::shutdown(format!(
                "omnisolo-shutdown-{}",
                Uuid::new_v4()
            )))
            .await
            .map(|_| ())
        } else {
            Ok(())
        };
        self.inner.closed.store(true, Ordering::Release);
        drop(self.inner.writer.lock().await.take());
        let mut process = self.inner.process.lock().await;
        let status = match timeout(self.inner.request_timeout, process.wait()).await {
            Ok(result) => result.map_err(HarnessAdapterError::Io)?,
            Err(_) => {
                process.start_kill().map_err(HarnessAdapterError::Io)?;
                process.wait().await.map_err(HarnessAdapterError::Io)?
            }
        };
        fail_pending(&self.inner, RuntimeFailure::ProcessExited).await;
        self.inner.private_home.cleanup();
        command_result?;
        if !status.success() {
            return Err(HarnessAdapterError::Remote(format!(
                "OpenHarness process exited with status {status}"
            )));
        }
        Ok(())
    }
}

async fn read_openharness_stdout(
    mut lines: BufReader<tokio::process::ChildStdout>,
    weak: std::sync::Weak<OpenHarnessRuntimeState>,
) {
    loop {
        match read_bounded_line(&mut lines).await {
            Ok(Some(line)) if line.trim().is_empty() => continue,
            Ok(Some(line)) => {
                let Some(inner) = weak.upgrade() else {
                    return;
                };
                let event = match OpenHarnessCodec.decode_event_line(&line) {
                    Ok(event) => event,
                    Err(error) => {
                        let failure = RuntimeFailure::InvalidResponse(error.to_string());
                        mark_fatal(&inner, failure.clone()).await;
                        let _ = inner.events_tx.send(Err(failure.error())).await;
                        continue;
                    }
                };
                route_openharness_event(&inner, event).await;
            }
            Ok(None) => break,
            Err(error) => {
                let Some(inner) = weak.upgrade() else {
                    return;
                };
                let failure = RuntimeFailure::InvalidResponse(format!(
                    "OpenHarness stdout I/O error: {error}"
                ));
                mark_fatal(&inner, failure.clone()).await;
                let _ = inner.events_tx.send(Err(failure.error())).await;
                return;
            }
        }
    }
    let Some(inner) = weak.upgrade() else {
        return;
    };
    if !inner.shutting_down.load(Ordering::Acquire) && !inner.closed.load(Ordering::Acquire) {
        mark_fatal(&inner, RuntimeFailure::ProcessExited).await;
        let _ = inner
            .events_tx
            .send(Err(HarnessAdapterError::ProcessExited))
            .await;
    }
}

async fn read_bounded_line<R: AsyncBufRead + Unpin>(
    reader: &mut R,
) -> std::io::Result<Option<String>> {
    let mut bytes = Vec::new();
    loop {
        let available = reader.fill_buf().await?;
        if available.is_empty() {
            if bytes.is_empty() {
                return Ok(None);
            }
            break;
        }
        let newline = available.iter().position(|byte| *byte == b'\n');
        let consumed = newline.map_or(available.len(), |index| index + 1);
        if bytes.len().saturating_add(consumed) > OPENHARNESS_MAX_FRAME_BYTES {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "OpenHarness JSONL frame exceeds the limit",
            ));
        }
        bytes.extend_from_slice(&available[..consumed]);
        reader.consume(consumed);
        if newline.is_some() {
            break;
        }
    }
    if bytes.last() == Some(&b'\n') {
        bytes.pop();
    }
    if bytes.last() == Some(&b'\r') {
        bytes.pop();
    }
    String::from_utf8(bytes).map(Some).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "OpenHarness frame is not UTF-8",
        )
    })
}

async fn route_openharness_event(inner: &Arc<OpenHarnessRuntimeState>, event: OpenHarnessEvent) {
    let response = match &event {
        OpenHarnessEvent::Ack { id, command } => Some((id.clone(), command.clone(), false, None)),
        OpenHarnessEvent::Session { id, command, .. } => {
            Some((id.clone(), command.clone(), true, None))
        }
        OpenHarnessEvent::SessionDeleted { id, command, .. } => {
            Some((id.clone(), command.clone(), true, None))
        }
        OpenHarnessEvent::Error {
            request_id,
            code,
            message,
            ..
        } if inner.pending.lock().await.contains_key(request_id) => Some((
            request_id.clone(),
            String::new(),
            false,
            Some(format!("{code}: {message}")),
        )),
        _ => None,
    };
    if let Some((id, command, is_session, remote_error)) = response {
        let Some(pending) = inner.pending.lock().await.remove(&id) else {
            let failure = RuntimeFailure::InvalidResponse(format!(
                "OpenHarness response used unknown command id: {id}"
            ));
            mark_fatal(inner, failure.clone()).await;
            let _ = inner.events_tx.send(Err(failure.error())).await;
            return;
        };
        let result = if let Some(message) = remote_error {
            Err(HarnessAdapterError::Remote(message))
        } else if command != pending.expected_command {
            Err(invalid_response(format!(
                "OpenHarness response command {command:?} did not match {:?}",
                pending.expected_command
            )))
        } else if is_session != pending.expects_session {
            Err(invalid_response(format!(
                "OpenHarness {} response used the wrong event shape",
                pending.expected_command
            )))
        } else {
            Ok(event)
        };
        let _ = pending.sender.send(result);
        return;
    }

    if let OpenHarnessEvent::Error {
        request_id,
        session_id: None,
        code,
        message,
        terminal: true,
    } = &event
        && request_id == "__startup__"
    {
        let failure = RuntimeFailure::Remote(format!("{code}: {message}"));
        mark_fatal(inner, failure.clone()).await;
        let _ = inner.events_tx.send(Err(failure.error())).await;
        return;
    }

    let correlation = stream_correlation(&event);
    let Some((request_id, session_id)) = correlation else {
        let failure = RuntimeFailure::InvalidResponse(
            "OpenHarness emitted an uncorrelated command event".to_owned(),
        );
        mark_fatal(inner, failure.clone()).await;
        let _ = inner.events_tx.send(Err(failure.error())).await;
        return;
    };
    let mut active = inner.active_prompt.lock().await;
    if active
        .as_ref()
        .is_none_or(|expected| expected.0 != request_id || expected.1 != session_id)
    {
        drop(active);
        let failure = RuntimeFailure::InvalidResponse(format!(
            "OpenHarness event correlation did not match active prompt {request_id}"
        ));
        mark_fatal(inner, failure.clone()).await;
        let _ = inner.events_tx.send(Err(failure.error())).await;
        return;
    }
    if event.terminal() {
        active.take();
    }
    drop(active);
    let _ = inner.events_tx.send(Ok(event)).await;
}

fn stream_correlation(event: &OpenHarnessEvent) -> Option<(&str, &str)> {
    match event {
        OpenHarnessEvent::Text {
            request_id,
            session_id,
            ..
        }
        | OpenHarnessEvent::System {
            request_id,
            session_id,
            ..
        }
        | OpenHarnessEvent::Downgrade {
            request_id,
            session_id,
            ..
        }
        | OpenHarnessEvent::Compaction {
            request_id,
            session_id,
            ..
        }
        | OpenHarnessEvent::ToolUse {
            request_id,
            session_id,
            ..
        }
        | OpenHarnessEvent::ToolResult {
            request_id,
            session_id,
            ..
        }
        | OpenHarnessEvent::Result {
            request_id,
            session_id,
            ..
        }
        | OpenHarnessEvent::Cancelled {
            request_id,
            session_id,
        } => Some((request_id, session_id)),
        OpenHarnessEvent::Error {
            request_id,
            session_id: Some(session_id),
            ..
        } => Some((request_id, session_id)),
        OpenHarnessEvent::Ready { .. }
        | OpenHarnessEvent::Ack { .. }
        | OpenHarnessEvent::Session { .. }
        | OpenHarnessEvent::SessionDeleted { .. }
        | OpenHarnessEvent::Error {
            session_id: None, ..
        } => None,
    }
}

async fn mark_fatal(inner: &Arc<OpenHarnessRuntimeState>, failure: RuntimeFailure) {
    let mut fatal = inner.fatal.lock().await;
    if fatal.is_none() {
        *fatal = Some(failure.clone());
    }
    drop(fatal);
    inner.active_prompt.lock().await.take();
    fail_pending(inner, failure).await;
}

async fn fail_pending(inner: &Arc<OpenHarnessRuntimeState>, failure: RuntimeFailure) {
    let pending = inner
        .pending
        .lock()
        .await
        .drain()
        .map(|(_, pending)| pending)
        .collect::<Vec<_>>();
    for pending in pending {
        let _ = pending.sender.send(Err(failure.error()));
    }
}

fn invalid_response(message: impl Into<String>) -> HarnessAdapterError {
    HarnessAdapterError::InvalidResponse(message.into())
}
