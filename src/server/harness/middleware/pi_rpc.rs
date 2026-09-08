use std::collections::{BTreeMap, HashMap};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use serde::{Serialize, Serializer};
use serde_json::{Map, Value, json};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, Command};
use tokio::sync::{Mutex, mpsc, oneshot};
use tokio::time::timeout;
use uuid::Uuid;

use super::harness::{HarnessAdapterError, HarnessEvent};
use super::process_env::apply_isolated_environment;
use super::types::{
    ModelApiDialect, ReasoningEffort, ResolvedModelSelection, sanitize_credential_value,
};

pub const PI_PROVIDER_ID: &str = "omnisolo-openai-compatible";
pub const PI_API_KEY_REFERENCE: &str = "OPENAI_API_KEY";
pub const PI_AGENT_DIR_ENV: &str = "PI_CODING_AGENT_DIR";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PiThinkingLevel {
    Minimal,
    Low,
    Medium,
    High,
    XHigh,
}

pub fn thinking_level(
    effort: &ReasoningEffort,
) -> Result<Option<PiThinkingLevel>, HarnessAdapterError> {
    match effort {
        ReasoningEffort::None => Ok(None),
        ReasoningEffort::Minimal => Ok(Some(PiThinkingLevel::Minimal)),
        ReasoningEffort::Low => Ok(Some(PiThinkingLevel::Low)),
        ReasoningEffort::Medium => Ok(Some(PiThinkingLevel::Medium)),
        ReasoningEffort::High => Ok(Some(PiThinkingLevel::High)),
        ReasoningEffort::Max => Ok(Some(PiThinkingLevel::XHigh)),
        ReasoningEffort::Custom => Err(HarnessAdapterError::InvalidRequest(
            "Pi RPC cannot translate a custom reasoning effort".to_owned(),
        )),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PiRpcCommand {
    id: String,
    command_type: String,
    wire: Value,
}

impl PiRpcCommand {
    pub fn prompt(id: impl Into<String>, message: impl Into<String>) -> Self {
        Self::with_fields(id, "prompt", [("message", Value::String(message.into()))])
    }

    pub fn steer(id: impl Into<String>, message: impl Into<String>) -> Self {
        Self::with_fields(id, "steer", [("message", Value::String(message.into()))])
    }

    pub fn follow_up(id: impl Into<String>, message: impl Into<String>) -> Self {
        Self::with_fields(
            id,
            "follow_up",
            [("message", Value::String(message.into()))],
        )
    }

    pub fn abort(id: impl Into<String>) -> Self {
        Self::with_fields(id, "abort", [])
    }

    pub fn set_model(
        id: impl Into<String>,
        provider: impl Into<String>,
        model_id: impl Into<String>,
    ) -> Self {
        Self::with_fields(
            id,
            "set_model",
            [
                ("provider", Value::String(provider.into())),
                ("modelId", Value::String(model_id.into())),
            ],
        )
    }

    pub fn set_thinking_level(id: impl Into<String>, level: PiThinkingLevel) -> Self {
        Self::with_fields(
            id,
            "set_thinking_level",
            [(
                "level",
                serde_json::to_value(level).expect("Pi thinking levels always serialize"),
            )],
        )
    }

    pub fn get_state(id: impl Into<String>) -> Self {
        Self::with_fields(id, "get_state", [])
    }

    pub fn compact(id: impl Into<String>, custom_instructions: Option<&str>) -> Self {
        let fields = custom_instructions
            .map(|instructions| {
                vec![("customInstructions", Value::String(instructions.to_owned()))]
            })
            .unwrap_or_default();
        Self::with_fields(id, "compact", fields)
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn command_type(&self) -> &str {
        &self.command_type
    }

    fn with_fields<I>(id: impl Into<String>, command_type: &str, fields: I) -> Self
    where
        I: IntoIterator<Item = (&'static str, Value)>,
    {
        let id = id.into();
        let mut object = Map::new();
        object.insert("id".to_owned(), Value::String(id.clone()));
        object.insert("type".to_owned(), Value::String(command_type.to_owned()));
        for (key, value) in fields {
            object.insert(key.to_owned(), value);
        }
        Self {
            id,
            command_type: command_type.to_owned(),
            wire: Value::Object(object),
        }
    }
}

impl Serialize for PiRpcCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.wire.serialize(serializer)
    }
}

pub fn build_models_json(
    selection: &ResolvedModelSelection,
    base_url: &str,
) -> Result<Value, HarnessAdapterError> {
    let provider = selection.provider_route.trim();
    let model_id = selection.model_id.trim();
    let base_url = base_url.trim().trim_end_matches('/');
    if provider.is_empty() || model_id.is_empty() {
        return Err(HarnessAdapterError::InvalidRequest(
            "Pi models.json requires a provider route and model id".to_owned(),
        ));
    }
    if selection.api_dialect != ModelApiDialect::OpenAiResponses {
        return Err(HarnessAdapterError::InvalidRequest(
            "Pi integration requires the OpenAI Responses API dialect".to_owned(),
        ));
    }
    if !(base_url.starts_with("https://") || base_url.starts_with("http://")) {
        return Err(HarnessAdapterError::InvalidRequest(
            "Pi models.json requires an HTTP(S) base URL".to_owned(),
        ));
    }
    let authority_start = base_url
        .find("://")
        .expect("validated HTTP(S) URLs always contain a scheme separator")
        + 3;
    let authority_end = base_url[authority_start..]
        .find('/')
        .map(|offset| authority_start + offset)
        .unwrap_or(base_url.len());
    if base_url[authority_start..authority_end].contains('@')
        || base_url.contains('?')
        || base_url.contains('#')
        || base_url.chars().any(char::is_whitespace)
    {
        return Err(HarnessAdapterError::InvalidRequest(
            "Pi models.json base URL must not contain credentials, query parameters, fragments, or whitespace"
                .to_owned(),
        ));
    }
    if let Some(effort) = selection.reasoning_effort.as_ref() {
        thinking_level(effort)?;
    }

    let mut model = Map::new();
    model.insert("id".to_owned(), Value::String(model_id.to_owned()));
    model.insert("name".to_owned(), Value::String(model_id.to_owned()));
    model.insert(
        "reasoning".to_owned(),
        Value::Bool(selection.reasoning_effort.as_ref() != Some(&ReasoningEffort::None)),
    );
    model.insert("input".to_owned(), json!(["text"]));
    if let Some(context_window) = selection.context_window {
        model.insert("contextWindow".to_owned(), Value::from(context_window));
    }
    if let Some(max_tokens) = selection.max_output_tokens {
        model.insert("maxTokens".to_owned(), Value::from(max_tokens));
    }
    model.insert(
        "thinkingLevelMap".to_owned(),
        json!({
            "minimal": "minimal",
            "low": "low",
            "medium": "medium",
            "high": "high",
            "xhigh": "max"
        }),
    );

    Ok(json!({
        "providers": {
            provider: {
                "baseUrl": base_url,
                "api": "openai-responses",
                "apiKey": PI_API_KEY_REFERENCE,
                "authHeader": true,
                "models": [Value::Object(model)]
            }
        }
    }))
}

#[derive(Debug)]
pub struct PiIsolatedHome {
    agent_dir: PathBuf,
    environment: BTreeMap<String, PathBuf>,
}

impl PiIsolatedHome {
    pub fn create_in(
        parent: &Path,
        selection: &ResolvedModelSelection,
        base_url: &str,
    ) -> Result<Self, HarnessAdapterError> {
        let descriptor = build_models_json(selection, base_url)?;
        let agent_dir = parent.join(format!("omnisolo-pi-{}", Uuid::new_v4()));
        fs::create_dir(&agent_dir).map_err(HarnessAdapterError::Io)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&agent_dir, fs::Permissions::from_mode(0o700))
                .map_err(HarnessAdapterError::Io)?;
        }
        let models_path = agent_dir.join("models.json");
        let write_result = write_private_json(&models_path, &descriptor);
        if let Err(error) = write_result {
            let _ = fs::remove_dir_all(&agent_dir);
            return Err(error);
        }
        Ok(Self {
            environment: BTreeMap::from([(PI_AGENT_DIR_ENV.to_owned(), agent_dir.clone())]),
            agent_dir,
        })
    }

    pub fn agent_dir(&self) -> &Path {
        &self.agent_dir
    }

    pub fn environment(&self) -> &BTreeMap<String, PathBuf> {
        &self.environment
    }
}

impl Drop for PiIsolatedHome {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.agent_dir);
    }
}

fn write_private_json(path: &Path, value: &Value) -> Result<(), HarnessAdapterError> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(path).map_err(HarnessAdapterError::Io)?;
    serde_json::to_writer_pretty(&mut file, value).map_err(HarnessAdapterError::Json)?;
    file.write_all(b"\n").map_err(HarnessAdapterError::Io)?;
    file.sync_all().map_err(HarnessAdapterError::Io)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PiEventCorrelation {
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub turn_id: Option<Uuid>,
    pub attempt_id: String,
    pub native_session_id: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PiDecodedEvent {
    pub event: HarnessEvent,
    pub final_text: Option<String>,
    pub usage: Option<Value>,
    pub terminal: bool,
}

#[derive(Clone, Debug)]
pub struct PiEventDecoder {
    correlation: PiEventCorrelation,
    sequence: u64,
    terminal_emitted: bool,
}

impl PiEventDecoder {
    pub fn new(correlation: PiEventCorrelation) -> Self {
        Self {
            correlation,
            sequence: 0,
            terminal_emitted: false,
        }
    }

    pub fn decode(&mut self, native: Value) -> Result<PiDecodedEvent, HarnessAdapterError> {
        let object = native
            .as_object()
            .ok_or_else(|| invalid_response("Pi RPC event must be a JSON object"))?;
        let event_type = required_string(object.get("type"), "Pi RPC event type")?;
        if event_type == "response" {
            return Err(invalid_response(
                "Pi RPC response cannot be decoded as a streamed event",
            ));
        }
        self.sequence = self.sequence.saturating_add(1);
        let cursor = format!("{}:{}", self.correlation.native_session_id, self.sequence);

        let mut final_text = None;
        let mut usage = None;
        let mut terminal = false;
        let (canonical_type, durable, data) = match event_type {
            "agent_start" => ("turn.started", true, Map::new()),
            "agent_end" => {
                if self.terminal_emitted {
                    ("native.pi.agent_end", true, Map::new())
                } else {
                    self.terminal_emitted = true;
                    terminal = true;
                    let stop_reason = last_assistant_stop_reason(object.get("messages"));
                    let canonical = match stop_reason {
                        Some("stop" | "toolUse") => "turn.completed",
                        Some("aborted") => "turn.cancelled",
                        Some("length" | "error") | None => "turn.failed",
                        Some(_) => "turn.failed",
                    };
                    (canonical, true, Map::new())
                }
            }
            "message_update" => decode_message_update(object)?,
            "message_end" => {
                let message = required_object(object.get("message"), "Pi message_end message")?;
                let role = required_string(message.get("role"), "Pi message_end message role")?;
                let mut data = Map::new();
                data.insert("message".to_owned(), Value::Object(message.clone()));
                match role {
                    "assistant" => {
                        final_text = assistant_text(message.get("content"))?;
                        usage = optional_object(message.get("usage"), "Pi assistant usage")?;
                        if let Some(value) = usage.clone() {
                            data.insert("usage".to_owned(), value);
                        }
                        ("assistant.final", true, data)
                    }
                    "user" => ("conversation.user_message", true, data),
                    "toolResult" => ("tool.result_message", true, data),
                    _ => {
                        return Err(invalid_response(
                            "Pi message_end role must be assistant, user, or toolResult",
                        ));
                    }
                }
            }
            "tool_execution_start" => (
                "tool.started",
                true,
                selected_fields(
                    object,
                    &["toolCallId", "toolName", "args"],
                    "Pi tool_execution_start",
                )?,
            ),
            "tool_execution_update" => (
                "tool.output",
                false,
                selected_fields(
                    object,
                    &["toolCallId", "toolName", "args", "partialResult"],
                    "Pi tool_execution_update",
                )?,
            ),
            "tool_execution_end" => {
                let is_error = object
                    .get("isError")
                    .and_then(Value::as_bool)
                    .ok_or_else(|| {
                        invalid_response("Pi tool_execution_end isError must be a boolean")
                    })?;
                (
                    if is_error {
                        "tool.failed"
                    } else {
                        "tool.completed"
                    },
                    true,
                    selected_fields(
                        object,
                        &["toolCallId", "toolName", "result", "isError"],
                        "Pi tool_execution_end",
                    )?,
                )
            }
            "session_info_changed" | "thinking_level_changed" | "queue_update" => {
                ("session.state_changed", true, object.clone())
            }
            "compaction_start" => ("session.compaction_started", true, object.clone()),
            "compaction_end" => {
                let aborted = object
                    .get("aborted")
                    .and_then(Value::as_bool)
                    .ok_or_else(|| {
                        invalid_response("Pi compaction_end aborted must be a boolean")
                    })?;
                (
                    if aborted {
                        "session.compaction_failed"
                    } else {
                        "session.compacted"
                    },
                    true,
                    object.clone(),
                )
            }
            "extension_error" => ("harness.extension_failed", true, object.clone()),
            _ => ("native.pi.event", false, object.clone()),
        };

        let mut payload = data;
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
        payload.insert("native".to_owned(), native);
        let payload = sanitize_credential_value(&Value::Object(payload))
            .as_object()
            .cloned()
            .expect("sanitizing an object preserves its JSON shape");
        usage = usage.map(|value| sanitize_credential_value(&value));

        Ok(PiDecodedEvent {
            event: HarnessEvent {
                event_type: canonical_type.to_owned(),
                durable,
                payload: Value::Object(payload),
                native_cursor: Some(cursor),
            },
            final_text,
            usage,
            terminal,
        })
    }
}

fn decode_message_update(
    object: &Map<String, Value>,
) -> Result<(&'static str, bool, Map<String, Value>), HarnessAdapterError> {
    let update = required_object(
        object.get("assistantMessageEvent"),
        "Pi message_update assistantMessageEvent",
    )?;
    let update_type = required_string(update.get("type"), "Pi assistant message event type")?;
    let mut data = Map::new();
    data.insert("update".to_owned(), Value::Object(update.clone()));
    match update_type {
        "text_delta" => {
            data.insert(
                "content".to_owned(),
                Value::String(required_string(update.get("delta"), "Pi text delta")?.to_owned()),
            );
            Ok(("assistant.text_chunk", false, data))
        }
        "thinking_delta" => {
            data.insert(
                "content".to_owned(),
                Value::String(
                    required_string(update.get("delta"), "Pi thinking delta")?.to_owned(),
                ),
            );
            Ok(("assistant.reasoning", false, data))
        }
        "toolcall_start" => Ok(("assistant.tool_call_started", false, data)),
        "toolcall_delta" => Ok(("assistant.tool_call_chunk", false, data)),
        "toolcall_end" => Ok(("assistant.tool_call_completed", true, data)),
        "error" => Ok(("assistant.provider_error", true, data)),
        "start" | "text_start" | "text_end" | "thinking_start" | "thinking_end" | "done" => {
            Ok(("native.pi.message_update", false, data))
        }
        _ => Ok(("native.pi.message_update", false, data)),
    }
}

fn selected_fields(
    source: &Map<String, Value>,
    fields: &[&str],
    context: &str,
) -> Result<Map<String, Value>, HarnessAdapterError> {
    let mut selected = Map::new();
    for field in fields {
        let value = source
            .get(*field)
            .cloned()
            .ok_or_else(|| invalid_response(format!("{context} {field} is required")))?;
        selected.insert((*field).to_owned(), value);
    }
    Ok(selected)
}

fn last_assistant_stop_reason(messages: Option<&Value>) -> Option<&str> {
    messages?
        .as_array()?
        .iter()
        .rev()
        .find(|message| message.get("role").and_then(Value::as_str) == Some("assistant"))?
        .get("stopReason")?
        .as_str()
}

fn assistant_text(content: Option<&Value>) -> Result<Option<String>, HarnessAdapterError> {
    let blocks = content
        .and_then(Value::as_array)
        .ok_or_else(|| invalid_response("Pi assistant content must be an array"))?;
    let mut text = String::new();
    for block in blocks {
        let block = block
            .as_object()
            .ok_or_else(|| invalid_response("Pi assistant content block must be an object"))?;
        if block.get("type").and_then(Value::as_str) == Some("text") {
            text.push_str(required_string(block.get("text"), "Pi assistant text")?);
        }
    }
    Ok((!text.is_empty()).then_some(text))
}

fn required_object<'a>(
    value: Option<&'a Value>,
    field: &str,
) -> Result<&'a Map<String, Value>, HarnessAdapterError> {
    value
        .and_then(Value::as_object)
        .ok_or_else(|| invalid_response(format!("{field} must be an object")))
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

fn required_string<'a>(
    value: Option<&'a Value>,
    field: &str,
) -> Result<&'a str, HarnessAdapterError> {
    value
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| invalid_response(format!("{field} must be a non-empty string")))
}

fn invalid_response(message: impl Into<String>) -> HarnessAdapterError {
    HarnessAdapterError::InvalidResponse(message.into())
}

#[derive(Clone)]
pub struct PiRpcProcessConfig {
    pub executable: String,
    pub args: Vec<String>,
    pub environment: HashMap<String, String>,
    pub request_timeout: Duration,
}

impl std::fmt::Debug for PiRpcProcessConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PiRpcProcessConfig")
            .field("executable", &self.executable)
            .field("args", &self.args)
            .field(
                "environment_keys",
                &self.environment.keys().collect::<Vec<_>>(),
            )
            .field("request_timeout", &self.request_timeout)
            .finish()
    }
}

impl PiRpcProcessConfig {
    pub fn shell(script: impl Into<String>) -> Self {
        Self {
            executable: "/bin/sh".to_owned(),
            args: vec!["-c".to_owned(), script.into()],
            environment: HashMap::new(),
            request_timeout: Duration::from_secs(30),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PiRpcResponse {
    pub id: String,
    pub command: String,
    pub data: Option<Value>,
    pub native: Value,
}

struct PendingCommand {
    expected_command: String,
    sender: oneshot::Sender<Result<PiRpcResponse, HarnessAdapterError>>,
}

struct PiRuntimeState {
    writer: Mutex<Option<ChildStdin>>,
    process: Mutex<Child>,
    pending: Mutex<HashMap<String, PendingCommand>>,
    event_tx: mpsc::Sender<Result<PiDecodedEvent, HarnessAdapterError>>,
    event_rx: Mutex<mpsc::Receiver<Result<PiDecodedEvent, HarnessAdapterError>>>,
    decoder: Mutex<Option<PiEventDecoder>>,
    request_timeout: Duration,
    active_prompt: AtomicBool,
    closed: AtomicBool,
}

#[derive(Clone)]
pub struct PiRpcRuntime {
    inner: Arc<PiRuntimeState>,
}

impl std::fmt::Debug for PiRpcRuntime {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PiRpcRuntime")
            .field("closed", &self.inner.closed.load(Ordering::Acquire))
            .finish()
    }
}

impl PiRpcRuntime {
    pub async fn spawn(config: PiRpcProcessConfig) -> Result<Self, HarnessAdapterError> {
        let mut command = Command::new(&config.executable);
        command.args(&config.args);
        apply_isolated_environment(&mut command, &config.environment);
        let mut process = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(HarnessAdapterError::Spawn)?;
        let writer = process
            .stdin
            .take()
            .ok_or_else(|| invalid_response("Pi RPC process did not expose stdin"))?;
        let stdout = process
            .stdout
            .take()
            .ok_or_else(|| invalid_response("Pi RPC process did not expose stdout"))?;
        let (event_tx, event_rx) = mpsc::channel(256);
        let inner = Arc::new(PiRuntimeState {
            writer: Mutex::new(Some(writer)),
            process: Mutex::new(process),
            pending: Mutex::new(HashMap::new()),
            event_tx,
            event_rx: Mutex::new(event_rx),
            decoder: Mutex::new(None),
            request_timeout: config.request_timeout,
            active_prompt: AtomicBool::new(false),
            closed: AtomicBool::new(false),
        });
        tokio::spawn(read_pi_stdout(stdout, inner.clone()));
        Ok(Self { inner })
    }

    pub async fn prompt(
        &self,
        correlation: PiEventCorrelation,
        command: PiRpcCommand,
    ) -> Result<PiRpcResponse, HarnessAdapterError> {
        if command.command_type() != "prompt" {
            return Err(HarnessAdapterError::InvalidRequest(
                "Pi RPC prompt requires a prompt command".to_owned(),
            ));
        }
        if self.inner.active_prompt.swap(true, Ordering::AcqRel) {
            return Err(HarnessAdapterError::InvalidRequest(
                "Pi RPC already has an active prompt; overlapping prompts cannot be correlated safely"
                    .to_owned(),
            ));
        }
        *self.inner.decoder.lock().await = Some(PiEventDecoder::new(correlation));
        let result = self.request(command).await;
        if result.is_err() {
            self.inner.active_prompt.store(false, Ordering::Release);
        }
        result
    }

    pub async fn request(
        &self,
        command: PiRpcCommand,
    ) -> Result<PiRpcResponse, HarnessAdapterError> {
        if self.inner.closed.load(Ordering::Acquire) {
            return Err(HarnessAdapterError::ProcessExited);
        }
        if command.id().trim().is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(
                "Pi RPC command id is required".to_owned(),
            ));
        }
        let id = command.id().to_owned();
        let expected_command = command.command_type().to_owned();
        let (sender, receiver) = oneshot::channel();
        {
            let mut pending = self.inner.pending.lock().await;
            if pending.contains_key(&id) {
                return Err(HarnessAdapterError::InvalidRequest(format!(
                    "duplicate Pi RPC command id: {id}"
                )));
            }
            pending.insert(
                id.clone(),
                PendingCommand {
                    expected_command,
                    sender,
                },
            );
        }
        let mut wire = serde_json::to_vec(&command).map_err(HarnessAdapterError::Json)?;
        wire.push(b'\n');
        let write_result = async {
            let mut writer = self.inner.writer.lock().await;
            let writer = writer.as_mut().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::BrokenPipe, "Pi RPC stdin is closed")
            })?;
            writer.write_all(&wire).await?;
            writer.flush().await
        }
        .await;
        if let Err(error) = write_result {
            self.inner.pending.lock().await.remove(&id);
            return Err(HarnessAdapterError::Io(error));
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

    pub async fn next_event(&self) -> Result<PiDecodedEvent, HarnessAdapterError> {
        self.inner
            .event_rx
            .lock()
            .await
            .recv()
            .await
            .unwrap_or(Err(HarnessAdapterError::ProcessExited))
    }

    pub async fn shutdown(&self) -> Result<(), HarnessAdapterError> {
        if self.inner.closed.swap(true, Ordering::AcqRel) {
            return Ok(());
        }
        drop(self.inner.writer.lock().await.take());
        let mut process = self.inner.process.lock().await;
        match timeout(Duration::from_secs(5), process.wait()).await {
            Ok(result) => {
                result.map_err(HarnessAdapterError::Io)?;
            }
            Err(_) => {
                process.start_kill().map_err(HarnessAdapterError::Io)?;
                process.wait().await.map_err(HarnessAdapterError::Io)?;
            }
        }
        self.inner.active_prompt.store(false, Ordering::Release);
        fail_pending(&self.inner, PendingFailure::ProcessExited).await;
        Ok(())
    }
}

async fn read_pi_stdout(stdout: tokio::process::ChildStdout, inner: Arc<PiRuntimeState>) {
    let mut lines = BufReader::new(stdout).lines();
    loop {
        match lines.next_line().await {
            Ok(Some(line)) if line.trim().is_empty() => continue,
            Ok(Some(line)) => {
                let native = match serde_json::from_str::<Value>(&line) {
                    Ok(value) => value,
                    Err(error) => {
                        let message = format!("malformed Pi RPC JSONL frame: {error}");
                        let _ = inner
                            .event_tx
                            .send(Err(invalid_response(message.clone())))
                            .await;
                        fail_pending(&inner, PendingFailure::InvalidResponse(message)).await;
                        continue;
                    }
                };
                if native.get("type").and_then(Value::as_str) == Some("response") {
                    route_response(&inner, native).await;
                } else {
                    let decoded = match inner.decoder.lock().await.as_mut() {
                        Some(decoder) => decoder.decode(native),
                        None if native.get("type").and_then(Value::as_str)
                            == Some("thinking_level_changed") =>
                        {
                            continue;
                        }
                        None => Err(invalid_response(
                            "Pi RPC emitted an event without an active session/task correlation",
                        )),
                    };
                    if decoded.as_ref().is_ok_and(|event| event.terminal) {
                        inner.active_prompt.store(false, Ordering::Release);
                    }
                    let _ = inner.event_tx.send(decoded).await;
                }
            }
            Ok(None) => break,
            Err(error) => {
                let message = format!("Pi RPC stdout I/O error: {error}");
                let _ = inner
                    .event_tx
                    .send(Err(invalid_response(message.clone())))
                    .await;
                fail_pending(&inner, PendingFailure::InvalidResponse(message)).await;
                return;
            }
        }
    }
    if !inner.closed.load(Ordering::Acquire) {
        let _ = inner
            .event_tx
            .send(Err(HarnessAdapterError::ProcessExited))
            .await;
        inner.active_prompt.store(false, Ordering::Release);
        fail_pending(&inner, PendingFailure::ProcessExited).await;
    }
}

async fn route_response(inner: &Arc<PiRuntimeState>, native: Value) {
    let id = native.get("id").and_then(Value::as_str).map(str::to_owned);
    let Some(id) = id else {
        let _ = inner
            .event_tx
            .send(Err(invalid_response(
                "Pi RPC response id must be a non-empty string",
            )))
            .await;
        return;
    };
    let Some(pending) = inner.pending.lock().await.remove(&id) else {
        let _ = inner
            .event_tx
            .send(Err(invalid_response(format!(
                "Pi RPC response used unknown command id: {id}"
            ))))
            .await;
        return;
    };
    let result = decode_response(native, &pending.expected_command);
    let _ = pending.sender.send(result);
}

fn decode_response(
    native: Value,
    expected_command: &str,
) -> Result<PiRpcResponse, HarnessAdapterError> {
    let object = native
        .as_object()
        .ok_or_else(|| invalid_response("Pi RPC response must be an object"))?;
    let id = required_string(object.get("id"), "Pi RPC response id")?.to_owned();
    let command = required_string(object.get("command"), "Pi RPC response command")?.to_owned();
    if command != expected_command {
        return Err(invalid_response(format!(
            "Pi RPC response command {command:?} did not match {expected_command:?}"
        )));
    }
    let success = object
        .get("success")
        .and_then(Value::as_bool)
        .ok_or_else(|| invalid_response("Pi RPC response success must be a boolean"))?;
    if !success {
        let error = required_string(object.get("error"), "Pi RPC response error")?;
        return Err(HarnessAdapterError::Remote(error.to_owned()));
    }
    Ok(PiRpcResponse {
        id,
        command,
        data: object.get("data").cloned(),
        native,
    })
}

enum PendingFailure {
    ProcessExited,
    InvalidResponse(String),
}

async fn fail_pending(inner: &Arc<PiRuntimeState>, failure: PendingFailure) {
    let pending = {
        let mut commands = inner.pending.lock().await;
        commands
            .drain()
            .map(|(_, pending)| pending)
            .collect::<Vec<_>>()
    };
    for pending in pending {
        let error = match &failure {
            PendingFailure::ProcessExited => HarnessAdapterError::ProcessExited,
            PendingFailure::InvalidResponse(message) => invalid_response(message.clone()),
        };
        let _ = pending.sender.send(Err(error));
    }
}
