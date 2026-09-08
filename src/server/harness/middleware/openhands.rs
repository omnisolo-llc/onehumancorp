use std::collections::{BTreeMap, BTreeSet};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::task::{Context, Poll};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json::{Map, Value, json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, mpsc, oneshot};
use tokio::time::{Instant, sleep, timeout};
use tokio_stream::Stream;
use tokio_stream::wrappers::ReceiverStream;

use super::http_runtime::{HttpProcessConfig, HttpProcessError, HttpProcessRuntime};
use super::types::{ModelApiDialect, ReasoningEffort, ResolvedModelSelection};

const MAX_HTTP_RESPONSE_BYTES: usize = 16 * 1024 * 1024;
const EVENT_PAGE_LIMIT: usize = 100;
pub const OPENHANDS_AGENT_SERVER_VERSION: &str = "1.43.1";
static ISOLATED_HOME_SEQUENCE: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpenHandsReasoningSupport {
    MaxAccepted,
    MaxUnsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityDowngradePolicy {
    Accept,
    Reject,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenHandsCapabilityDowngrade {
    pub capability: String,
    pub requested: String,
    pub applied: Option<String>,
    pub reason: String,
    pub requires_acceptance: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpenHandsPreparedConversation {
    pub body: Value,
    pub downgrades: Vec<OpenHandsCapabilityDowngrade>,
}

impl OpenHandsPreparedConversation {
    pub fn authorize(&self, policy: CapabilityDowngradePolicy) -> Result<(), OpenHandsError> {
        if policy == CapabilityDowngradePolicy::Reject && !self.downgrades.is_empty() {
            return Err(OpenHandsError::CapabilityDowngradeRequired(
                self.downgrades.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
pub struct OpenHandsAdapterConfig {
    pub workspace: PathBuf,
    pub resolved_model: ResolvedModelSelection,
    pub provider_base_url: String,
    pub reasoning_support: OpenHandsReasoningSupport,
    pub readiness_timeout: Duration,
    pub request_timeout: Duration,
    pub poll_interval: Duration,
    provider_api_key: Option<String>,
}

impl std::fmt::Debug for OpenHandsAdapterConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OpenHandsAdapterConfig")
            .field("workspace", &self.workspace)
            .field("model_id", &self.resolved_model.model_id)
            .field("provider_base_url", &"<configured>")
            .field("reasoning_support", &self.reasoning_support)
            .field("readiness_timeout", &self.readiness_timeout)
            .field("request_timeout", &self.request_timeout)
            .field("poll_interval", &self.poll_interval)
            .finish()
    }
}

impl OpenHandsAdapterConfig {
    pub fn new(
        workspace: impl Into<PathBuf>,
        resolved_model: ResolvedModelSelection,
        provider_base_url: impl Into<String>,
    ) -> Self {
        Self {
            workspace: workspace.into(),
            resolved_model,
            provider_base_url: provider_base_url.into(),
            reasoning_support: OpenHandsReasoningSupport::MaxUnsupported,
            readiness_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(30),
            poll_interval: Duration::from_millis(50),
            provider_api_key: None,
        }
    }

    pub fn with_reasoning_support(mut self, support: OpenHandsReasoningSupport) -> Self {
        self.reasoning_support = support;
        self
    }

    pub fn with_provider_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.provider_api_key = Some(api_key.into());
        self
    }

    fn redaction_secrets(&self) -> Vec<String> {
        self.provider_api_key
            .iter()
            .filter(|secret| !secret.is_empty())
            .cloned()
            .collect()
    }

    pub fn with_timeouts(
        mut self,
        readiness_timeout: Duration,
        request_timeout: Duration,
        poll_interval: Duration,
    ) -> Self {
        self.readiness_timeout = readiness_timeout;
        self.request_timeout = request_timeout;
        self.poll_interval = poll_interval;
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpenHandsConversationConfig {
    pub workspace: PathBuf,
    pub resolved_model: ResolvedModelSelection,
    pub provider_base_url: String,
}

impl OpenHandsConversationConfig {
    pub fn new(
        workspace: impl Into<PathBuf>,
        resolved_model: ResolvedModelSelection,
        provider_base_url: impl Into<String>,
    ) -> Self {
        Self {
            workspace: workspace.into(),
            resolved_model,
            provider_base_url: provider_base_url.into(),
        }
    }
}

impl From<&OpenHandsAdapterConfig> for OpenHandsConversationConfig {
    fn from(config: &OpenHandsAdapterConfig) -> Self {
        Self::new(
            config.workspace.clone(),
            config.resolved_model.clone(),
            config.provider_base_url.clone(),
        )
    }
}

pub struct OpenHandsLaunchConfig {
    process: HttpProcessConfig,
    adapter: OpenHandsAdapterConfig,
    api_key: Option<String>,
}

impl std::fmt::Debug for OpenHandsLaunchConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OpenHandsLaunchConfig")
            .field("process", &self.process)
            .field("adapter", &self.adapter)
            .field("api_key", &self.api_key.as_ref().map(|_| "<configured>"))
            .finish()
    }
}

impl OpenHandsLaunchConfig {
    pub fn new(process: HttpProcessConfig, adapter: OpenHandsAdapterConfig) -> Self {
        Self {
            process,
            adapter,
            api_key: None,
        }
    }

    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        let api_key = api_key.into();
        self.adapter.provider_api_key = Some(api_key.clone());
        self.api_key = Some(api_key);
        self
    }

    pub fn agent_server(adapter: OpenHandsAdapterConfig) -> Self {
        let process = HttpProcessConfig::new(
            "openhands-agent-server",
            ["--host", "{host}", "--port", "{port}"],
        )
        .with_readiness(adapter.readiness_timeout, adapter.poll_interval);
        Self::new(process, adapter)
    }

    pub fn process_config(&self) -> &HttpProcessConfig {
        &self.process
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OpenHandsProviderErrorKind {
    Authentication,
    Quota,
    RateLimited,
    ContextLengthExceeded,
    ModelUnavailable,
    Configuration,
    Transient,
    AgentAction,
    Internal,
    Timeout,
    Unavailable,
    InvalidRequest,
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpenHandsProviderError {
    pub kind: OpenHandsProviderErrorKind,
    pub status: Option<u16>,
    pub retryable: bool,
    pub native: Value,
}

impl std::fmt::Display for OpenHandsProviderError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.status {
            Some(status) => write!(
                formatter,
                "OpenHands provider request failed ({:?}, HTTP {status})",
                self.kind
            ),
            None => write!(
                formatter,
                "OpenHands provider request failed ({:?})",
                self.kind
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpenHandsRouterError {
    pub status: u16,
    pub native: Value,
}

impl std::fmt::Display for OpenHandsRouterError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "OpenHands API request failed (HTTP {})",
            self.status
        )
    }
}

#[derive(Debug)]
pub enum OpenHandsError {
    InvalidRequest(String),
    InvalidResponse(String),
    NonLoopbackAddress(SocketAddr),
    Io(std::io::Error),
    Json(serde_json::Error),
    Timeout,
    ReadinessTimeout {
        address: SocketAddr,
    },
    UnsupportedServerVersion {
        expected: &'static str,
        actual: String,
    },
    ProcessExited {
        status: Option<i32>,
    },
    Router(OpenHandsRouterError),
    Provider(OpenHandsProviderError),
    CapabilityDowngradeRequired(Vec<OpenHandsCapabilityDowngrade>),
}

impl std::fmt::Display for OpenHandsError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRequest(message) => {
                write!(formatter, "invalid OpenHands request: {message}")
            }
            Self::InvalidResponse(message) => {
                write!(formatter, "invalid OpenHands response: {message}")
            }
            Self::NonLoopbackAddress(address) => {
                write!(formatter, "OpenHands address is not loopback: {address}")
            }
            Self::Io(error) => write!(formatter, "OpenHands HTTP transport failed: {error}"),
            Self::Json(error) => write!(formatter, "invalid OpenHands JSON: {error}"),
            Self::Timeout => formatter.write_str("OpenHands request timed out"),
            Self::ReadinessTimeout { address } => {
                write!(formatter, "OpenHands readiness timed out at {address}")
            }
            Self::UnsupportedServerVersion { expected, actual } => write!(
                formatter,
                "unsupported OpenHands Agent Server version {actual}; expected {expected}"
            ),
            Self::ProcessExited { status } => {
                write!(formatter, "OpenHands process exited: {status:?}")
            }
            Self::Router(error) => error.fmt(formatter),
            Self::Provider(error) => error.fmt(formatter),
            Self::CapabilityDowngradeRequired(downgrades) => write!(
                formatter,
                "OpenHands capability downgrade requires acceptance ({} downgrade(s))",
                downgrades.len()
            ),
        }
    }
}

impl std::error::Error for OpenHandsError {}

impl From<serde_json::Error> for OpenHandsError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<HttpProcessError> for OpenHandsError {
    fn from(error: HttpProcessError) -> Self {
        match error {
            HttpProcessError::NonLoopbackAddress(address) => Self::NonLoopbackAddress(address),
            HttpProcessError::Bind(error)
            | HttpProcessError::Spawn(error)
            | HttpProcessError::Poll(error) => Self::Io(error),
            HttpProcessError::ReadinessTimeout { address } => Self::ReadinessTimeout { address },
            HttpProcessError::EarlyExit { status } => Self::ProcessExited { status },
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OpenHandsEventCategory {
    Message,
    Action,
    Observation,
    Approval,
    Usage,
    Error,
    Native,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpenHandsEvent {
    pub kind: String,
    pub category: OpenHandsEventCategory,
    pub native_cursor: Option<String>,
    pub native: Value,
    pub final_text: Option<String>,
    pub usage: Option<Value>,
    pub provider_error: Option<OpenHandsProviderError>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpenHandsConversation {
    pub id: String,
    pub status: String,
    pub native: Value,
    pub downgrades: Vec<OpenHandsCapabilityDowngrade>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OpenHandsStreamItem {
    Event(OpenHandsEvent),
    ApprovalRequired {
        action: OpenHandsEvent,
    },
    Completed {
        final_text: Option<String>,
        usage: Option<Value>,
    },
}

pub struct OpenHandsEventStream {
    receiver: ReceiverStream<Result<OpenHandsStreamItem, OpenHandsError>>,
    cancel: Option<oneshot::Sender<()>>,
    completed: Arc<AtomicBool>,
}

impl Stream for OpenHandsEventStream {
    type Item = Result<OpenHandsStreamItem, OpenHandsError>;

    fn poll_next(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.get_mut().receiver).poll_next(context)
    }
}

impl Drop for OpenHandsEventStream {
    fn drop(&mut self) {
        if !self.completed.load(Ordering::Acquire)
            && let Some(cancel) = self.cancel.take()
        {
            let _ = cancel.send(());
        }
    }
}

pub fn prepare_conversation_request(
    config: &OpenHandsAdapterConfig,
    conversation_id: Option<&str>,
) -> Result<OpenHandsPreparedConversation, OpenHandsError> {
    prepare_conversation_request_with_config(&config.into(), conversation_id)
}

pub fn prepare_conversation_request_with_config(
    config: &OpenHandsConversationConfig,
    conversation_id: Option<&str>,
) -> Result<OpenHandsPreparedConversation, OpenHandsError> {
    validate_provider_base_url(&config.provider_base_url)?;
    if config.workspace.as_os_str().is_empty() {
        return Err(OpenHandsError::InvalidRequest(
            "workspace must not be empty".to_owned(),
        ));
    }
    let model = openhands_model_id(&config.resolved_model.model_id)?;
    let mut llm = Map::from_iter([
        ("model".to_owned(), Value::String(model)),
        (
            "base_url".to_owned(),
            Value::String(config.provider_base_url.clone()),
        ),
        ("usage_id".to_owned(), Value::String("agent".to_owned())),
    ]);
    match config.resolved_model.api_dialect {
        ModelApiDialect::OpenAiResponses => {
            llm.insert("api_mode".to_owned(), Value::String("responses".to_owned()));
        }
        ModelApiDialect::OpenAiChatCompletions => {
            llm.insert("api_mode".to_owned(), Value::String("chat".to_owned()));
        }
        _ => {
            return Err(OpenHandsError::InvalidRequest(
                "OpenHands requires an OpenAI Responses or Chat Completions dialect".to_owned(),
            ));
        }
    }
    let downgrades = Vec::new();
    if let Some(effort) = &config.resolved_model.reasoning_effort {
        match effort {
            ReasoningEffort::Max => {
                llm.insert(
                    "reasoning_effort".to_owned(),
                    Value::String("max".to_owned()),
                );
            }
            ReasoningEffort::None => insert_reasoning(&mut llm, "none"),
            ReasoningEffort::Minimal => insert_reasoning(&mut llm, "minimal"),
            ReasoningEffort::Low => insert_reasoning(&mut llm, "low"),
            ReasoningEffort::Medium => insert_reasoning(&mut llm, "medium"),
            ReasoningEffort::High => insert_reasoning(&mut llm, "high"),
            ReasoningEffort::Custom => {
                return Err(OpenHandsError::InvalidRequest(
                    "custom reasoning effort cannot be translated to OpenHands".to_owned(),
                ));
            }
        }
    }

    if let Some(max_output_tokens) = config.resolved_model.max_output_tokens {
        llm.insert(
            "max_output_tokens".to_owned(),
            Value::Number(max_output_tokens.into()),
        );
    }

    let mut body = json!({
        "agent": {
            "kind": "Agent",
            "llm": Value::Object(llm),
            "tools": [
                {"name": "terminal"},
                {"name": "file_editor"},
                {"name": "task_tracker"}
            ]
        },
        "workspace": {
            "kind": "LocalWorkspace",
            "working_dir": config.workspace.to_string_lossy()
        },
        "confirmation_policy": {"kind": "AlwaysConfirm"},
        "worktree": false,
        "autotitle": false
    });
    if let Some(conversation_id) = conversation_id {
        if conversation_id.is_empty() {
            return Err(OpenHandsError::InvalidRequest(
                "conversation id must not be empty".to_owned(),
            ));
        }
        body.as_object_mut()
            .expect("conversation body is an object")
            .insert(
                "conversation_id".to_owned(),
                Value::String(conversation_id.to_owned()),
            );
    }
    Ok(OpenHandsPreparedConversation { body, downgrades })
}

fn insert_reasoning(llm: &mut Map<String, Value>, effort: &str) {
    llm.insert(
        "reasoning_effort".to_owned(),
        Value::String(effort.to_owned()),
    );
}

fn openhands_model_id(model_id: &str) -> Result<String, OpenHandsError> {
    if model_id.is_empty() || model_id != model_id.trim() || model_id.len() > 256 {
        return Err(OpenHandsError::InvalidRequest(
            "model id must be non-empty, bounded, and have no surrounding whitespace".to_owned(),
        ));
    }
    let portable = model_id.strip_prefix("openai/").unwrap_or(model_id);
    if portable.is_empty()
        || portable.contains('/')
        || portable.contains("..")
        || !portable
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':'))
        || !portable
            .as_bytes()
            .first()
            .is_some_and(u8::is_ascii_alphanumeric)
        || !portable
            .as_bytes()
            .last()
            .is_some_and(u8::is_ascii_alphanumeric)
    {
        return Err(OpenHandsError::InvalidRequest(
            "portable OpenHands model contains invalid characters or path syntax".to_owned(),
        ));
    }
    if model_id.contains('/') && !model_id.starts_with("openai/") {
        return Err(OpenHandsError::InvalidRequest(
            "portable OpenHands model must be unprefixed or use openai/".to_owned(),
        ));
    }
    Ok(format!("openai/{portable}"))
}

fn validate_provider_base_url(base_url: &str) -> Result<(), OpenHandsError> {
    if base_url.is_empty()
        || base_url != base_url.trim()
        || base_url.len() > 2_048
        || base_url
            .bytes()
            .any(|byte| byte.is_ascii_whitespace() || byte.is_ascii_control() || byte == b'\\')
    {
        return Err(OpenHandsError::InvalidRequest(
            "provider base URL contains invalid whitespace or characters".to_owned(),
        ));
    }
    let Some((scheme, remainder)) = base_url.split_once("://") else {
        return Err(OpenHandsError::InvalidRequest(
            "provider base URL must be absolute".to_owned(),
        ));
    };
    if !matches!(scheme, "http" | "https") || remainder.is_empty() {
        return Err(OpenHandsError::InvalidRequest(
            "provider base URL must use http or https".to_owned(),
        ));
    }
    let (authority, path) = remainder
        .split_once('/')
        .map_or((remainder, ""), |(authority, path)| (authority, path));
    if authority.is_empty()
        || authority.contains('@')
        || remainder.contains(['?', '#', '%'])
        || path.split('/').any(|segment| matches!(segment, "." | ".."))
    {
        return Err(OpenHandsError::InvalidRequest(
            "provider base URL must not contain credentials, traversal, query, or fragment"
                .to_owned(),
        ));
    }
    validate_url_authority(authority)?;
    Ok(())
}

fn validate_url_authority(authority: &str) -> Result<(), OpenHandsError> {
    let (host, port) = if authority.starts_with('[') {
        let close = authority.find(']').ok_or_else(|| {
            OpenHandsError::InvalidRequest("provider base URL has invalid IPv6 host".to_owned())
        })?;
        let host = &authority[1..close];
        host.parse::<std::net::Ipv6Addr>().map_err(|_| {
            OpenHandsError::InvalidRequest("provider base URL has invalid IPv6 host".to_owned())
        })?;
        let suffix = &authority[close + 1..];
        let port = if suffix.is_empty() {
            None
        } else {
            Some(suffix.strip_prefix(':').ok_or_else(|| {
                OpenHandsError::InvalidRequest("provider base URL has invalid authority".to_owned())
            })?)
        };
        (host, port)
    } else {
        if authority.matches(':').count() > 1 {
            return Err(OpenHandsError::InvalidRequest(
                "provider base URL IPv6 hosts must use brackets".to_owned(),
            ));
        }
        authority
            .split_once(':')
            .map_or((authority, None), |(host, port)| (host, Some(port)))
    };
    if host.is_empty() {
        return Err(OpenHandsError::InvalidRequest(
            "provider base URL host must not be empty".to_owned(),
        ));
    }
    if !authority.starts_with('[') {
        let numeric_ipv4 = host
            .bytes()
            .all(|byte| byte.is_ascii_digit() || byte == b'.');
        if numeric_ipv4 {
            host.parse::<std::net::Ipv4Addr>().map_err(|_| {
                OpenHandsError::InvalidRequest("provider base URL has invalid IPv4 host".to_owned())
            })?;
        } else if !host.split('.').all(|label| {
            !label.is_empty()
                && label.len() <= 63
                && label
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
                && label
                    .as_bytes()
                    .first()
                    .is_some_and(u8::is_ascii_alphanumeric)
                && label
                    .as_bytes()
                    .last()
                    .is_some_and(u8::is_ascii_alphanumeric)
        }) {
            return Err(OpenHandsError::InvalidRequest(
                "provider base URL has invalid host".to_owned(),
            ));
        }
    }
    if let Some(port) = port {
        let port = port.parse::<u16>().map_err(|_| {
            OpenHandsError::InvalidRequest("provider base URL has invalid port".to_owned())
        })?;
        if port == 0 {
            return Err(OpenHandsError::InvalidRequest(
                "provider base URL port must be non-zero".to_owned(),
            ));
        }
    }
    Ok(())
}

pub fn sanitize_native_json(value: &Value) -> Value {
    sanitize_native_json_with_secrets(value, &[])
}

fn sanitize_native_json_with_secrets(value: &Value, secrets: &[String]) -> Value {
    match value {
        Value::Object(object) => Value::Object(
            object
                .iter()
                .map(|(key, value)| {
                    let value = if sensitive_key(key) {
                        Value::String("[REDACTED]".to_owned())
                    } else {
                        sanitize_native_json_with_secrets(value, secrets)
                    };
                    (key.clone(), value)
                })
                .collect(),
        ),
        Value::Array(values) => Value::Array(
            values
                .iter()
                .map(|value| sanitize_native_json_with_secrets(value, secrets))
                .collect(),
        ),
        Value::String(text) => Value::String(redact_native_text(text, secrets)),
        other => other.clone(),
    }
}

fn redact_native_text(text: &str, secrets: &[String]) -> String {
    let mut redacted = text.to_owned();
    for secret in secrets.iter().filter(|secret| !secret.is_empty()) {
        redacted = redacted.replace(secret, "[REDACTED]");
    }
    if sensitive_text(&redacted) {
        "[REDACTED]".to_owned()
    } else {
        redacted
    }
}

fn sensitive_key(key: &str) -> bool {
    let normalized = key
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect::<String>();
    [
        "apikey",
        "accesstoken",
        "refreshtoken",
        "authorization",
        "password",
        "secret",
        "credential",
        "cookie",
    ]
    .iter()
    .any(|marker| normalized.contains(marker))
}

fn sensitive_text(text: &str) -> bool {
    let lower = text.trim().to_ascii_lowercase();
    lower.starts_with("bearer ")
        || (lower.contains("-----begin") && lower.contains("private key-----"))
        || [
            "api_key=",
            "api-key=",
            "access_token=",
            "refresh_token=",
            "authorization:",
            "password=",
        ]
        .iter()
        .any(|marker| lower.contains(marker))
}

pub fn decode_native_event(value: &Value) -> Result<OpenHandsEvent, OpenHandsError> {
    decode_native_event_with_secrets(value, &[])
}

pub fn decode_native_event_with_config(
    value: &Value,
    config: &OpenHandsAdapterConfig,
) -> Result<OpenHandsEvent, OpenHandsError> {
    decode_native_event_with_secrets(value, &config.redaction_secrets())
}

fn decode_native_event_with_secrets(
    value: &Value,
    secrets: &[String],
) -> Result<OpenHandsEvent, OpenHandsError> {
    let sanitized = sanitize_native_json_with_secrets(value, secrets);
    let value = &sanitized;
    let object = value.as_object().ok_or_else(|| {
        OpenHandsError::InvalidResponse("OpenHands event must be an object".to_owned())
    })?;
    let kind = ["kind", "type", "event_type"]
        .iter()
        .find_map(|key| object.get(*key).and_then(Value::as_str))
        .unwrap_or("UnknownEvent")
        .to_owned();
    let normalized = kind.to_ascii_lowercase();
    let category = if normalized.contains("observation") || normalized.contains("reject") {
        OpenHandsEventCategory::Observation
    } else if normalized.contains("action") {
        OpenHandsEventCategory::Action
    } else if normalized.contains("message") {
        OpenHandsEventCategory::Message
    } else if normalized.contains("usage") || normalized.contains("token") {
        OpenHandsEventCategory::Usage
    } else if normalized.contains("error") {
        OpenHandsEventCategory::Error
    } else {
        OpenHandsEventCategory::Native
    };
    let native_cursor = ["id", "event_id"]
        .iter()
        .find_map(|key| object.get(*key).and_then(Value::as_str))
        .map(ToOwned::to_owned);
    let final_text = if category == OpenHandsEventCategory::Message
        && object
            .get("source")
            .and_then(Value::as_str)
            .is_none_or(|source| source == "agent" || source == "assistant")
    {
        event_message_text(value)
    } else {
        None
    };
    let usage = object
        .get("usage")
        .filter(|value| !value.is_null())
        .or_else(|| object.get("metrics"))
        .filter(|value| !value.is_null())
        .map(|value| sanitize_native_json_with_secrets(value, secrets));
    let provider_error = (kind == "ConversationErrorEvent")
        .then(|| classify_conversation_error_event_with_secrets(value, secrets));
    Ok(OpenHandsEvent {
        kind,
        category,
        native_cursor,
        native: sanitized,
        final_text,
        usage,
        provider_error,
    })
}

fn classify_conversation_error_event_with_secrets(
    value: &Value,
    secrets: &[String],
) -> OpenHandsProviderError {
    let classification = value.get("classification");
    let kind = match classification
        .and_then(|classification| classification.get("kind"))
        .and_then(Value::as_str)
    {
        Some("auth") => OpenHandsProviderErrorKind::Authentication,
        Some("quota") => OpenHandsProviderErrorKind::Quota,
        Some("rate_limit") => OpenHandsProviderErrorKind::RateLimited,
        Some("config") => OpenHandsProviderErrorKind::Configuration,
        Some("transient") => OpenHandsProviderErrorKind::Transient,
        Some("agent_action") => OpenHandsProviderErrorKind::AgentAction,
        Some("internal") => OpenHandsProviderErrorKind::Internal,
        _ => OpenHandsProviderErrorKind::Unknown,
    };
    let retryable = classification
        .and_then(|classification| classification.get("retryable"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    OpenHandsProviderError {
        kind,
        status: None,
        retryable,
        native: sanitize_native_json_with_secrets(value, secrets),
    }
}

fn event_message_text(value: &Value) -> Option<String> {
    let message = value
        .get("llm_message")
        .or_else(|| value.get("message"))
        .unwrap_or(value);
    if let Some(text) = message.get("text").and_then(Value::as_str) {
        return Some(text.to_owned());
    }
    let content = message.get("content")?.as_array()?;
    let text = content
        .iter()
        .filter_map(|part| {
            part.as_str().map(ToOwned::to_owned).or_else(|| {
                part.get("text")
                    .and_then(Value::as_str)
                    .map(ToOwned::to_owned)
            })
        })
        .collect::<Vec<_>>()
        .join("\n");
    (!text.is_empty()).then_some(text)
}

pub fn classify_litellm_error(status: u16, body: &Value) -> OpenHandsProviderError {
    let text = body.to_string().to_ascii_lowercase();
    let kind = if status == 401
        || status == 403
        || text.contains("authenticationerror")
        || text.contains("unauthorized")
    {
        OpenHandsProviderErrorKind::Authentication
    } else if status == 429 || text.contains("ratelimiterror") || text.contains("rate limit") {
        OpenHandsProviderErrorKind::RateLimited
    } else if text.contains("context_length")
        || text.contains("context window")
        || text.contains("maximum context")
    {
        OpenHandsProviderErrorKind::ContextLengthExceeded
    } else if text.contains("model_not_found") || text.contains("model not found") || status == 404
    {
        OpenHandsProviderErrorKind::ModelUnavailable
    } else if status == 408
        || status == 504
        || text.contains("apitimeouterror")
        || text.contains("timed out")
    {
        OpenHandsProviderErrorKind::Timeout
    } else if status >= 500 || text.contains("serviceunavailable") {
        OpenHandsProviderErrorKind::Unavailable
    } else if status == 400 || status == 409 || status == 422 {
        OpenHandsProviderErrorKind::InvalidRequest
    } else {
        OpenHandsProviderErrorKind::Unknown
    };
    let retryable = matches!(
        kind,
        OpenHandsProviderErrorKind::RateLimited
            | OpenHandsProviderErrorKind::Timeout
            | OpenHandsProviderErrorKind::Unavailable
    );
    OpenHandsProviderError {
        kind,
        status: Some(status),
        retryable,
        native: sanitize_native_json(body),
    }
}

struct OpenHandsIsolatedHome {
    root: PathBuf,
}

impl OpenHandsIsolatedHome {
    fn create() -> Result<Self, OpenHandsError> {
        let sequence = ISOLATED_HOME_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "omnisolo-openhands-{}-{sequence}-{timestamp}",
            std::process::id()
        ));
        for directory in [
            root.clone(),
            root.join("home"),
            root.join("config"),
            root.join("data"),
            root.join("data/conversations"),
            root.join("data/bash-events"),
            root.join("data/worktrees"),
            root.join("data/tmux"),
            root.join("server-workspace"),
        ] {
            std::fs::create_dir_all(&directory).map_err(OpenHandsError::Io)?;
            set_owner_only_directory(&directory)?;
        }
        Ok(Self { root })
    }

    fn root(&self) -> &Path {
        &self.root
    }

    fn environment(&self) -> BTreeMap<String, String> {
        [
            ("HOME", self.root.join("home")),
            ("XDG_CONFIG_HOME", self.root.join("config")),
            ("XDG_DATA_HOME", self.root.join("data")),
            (
                "OPENHANDS_AGENT_SERVER_CONFIG_PATH",
                self.root.join("config/openhands_agent_server_config.json"),
            ),
            (
                "OH_CONVERSATIONS_PATH",
                self.root.join("data/conversations"),
            ),
            ("OH_BASH_EVENTS_DIR", self.root.join("data/bash-events")),
            (
                "OH_CONVERSATION_WORKTREE_ROOT",
                self.root.join("data/worktrees"),
            ),
            ("TMUX_TMPDIR", self.root.join("data/tmux")),
        ]
        .into_iter()
        .map(|(key, value)| (key.to_owned(), value.to_string_lossy().into_owned()))
        .collect()
    }
}

#[cfg(unix)]
fn set_owner_only_directory(path: &Path) -> Result<(), OpenHandsError> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700))
        .map_err(OpenHandsError::Io)
}

#[cfg(not(unix))]
fn set_owner_only_directory(_path: &Path) -> Result<(), OpenHandsError> {
    Ok(())
}

impl Drop for OpenHandsIsolatedHome {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

#[derive(Clone)]
struct OpenHandsHttpClient {
    address: SocketAddr,
    request_timeout: Duration,
    launched_process: bool,
    redaction_secrets: Arc<Vec<String>>,
}

impl OpenHandsHttpClient {
    async fn request(
        &self,
        method: &str,
        path: &str,
        body: Option<&Value>,
    ) -> Result<Value, OpenHandsError> {
        if !path.starts_with('/') || path.contains(['\r', '\n', ' ']) {
            return Err(OpenHandsError::InvalidRequest(
                "HTTP path is invalid".to_owned(),
            ));
        }
        let operation = self.request_inner(method, path, body);
        timeout(self.request_timeout, operation)
            .await
            .map_err(|_| OpenHandsError::Timeout)?
    }

    async fn request_inner(
        &self,
        method: &str,
        path: &str,
        body: Option<&Value>,
    ) -> Result<Value, OpenHandsError> {
        let mut stream = TcpStream::connect(self.address)
            .await
            .map_err(|error| self.transport_error(error))?;
        let body = body
            .map(serde_json::to_vec)
            .transpose()
            .map_err(OpenHandsError::Json)?
            .unwrap_or_default();
        let mut request = format!(
            "{method} {path} HTTP/1.1\r\nHost: {}\r\nAccept: application/json\r\nConnection: close\r\n",
            self.address
        );
        if !body.is_empty() {
            request.push_str("Content-Type: application/json\r\n");
        }
        request.push_str(&format!("Content-Length: {}\r\n\r\n", body.len()));
        stream
            .write_all(request.as_bytes())
            .await
            .map_err(|error| self.transport_error(error))?;
        if !body.is_empty() {
            stream
                .write_all(&body)
                .await
                .map_err(|error| self.transport_error(error))?;
        }
        let mut response = Vec::new();
        stream
            .take((MAX_HTTP_RESPONSE_BYTES + 1) as u64)
            .read_to_end(&mut response)
            .await
            .map_err(|error| self.transport_error(error))?;
        if response.len() > MAX_HTTP_RESPONSE_BYTES {
            return Err(OpenHandsError::InvalidResponse(
                "HTTP response exceeded size limit".to_owned(),
            ));
        }
        parse_http_response(&response, &self.redaction_secrets)
    }

    fn transport_error(&self, error: std::io::Error) -> OpenHandsError {
        if self.launched_process {
            OpenHandsError::ProcessExited { status: None }
        } else {
            OpenHandsError::Io(error)
        }
    }
}

fn parse_http_response(response: &[u8], secrets: &[String]) -> Result<Value, OpenHandsError> {
    let header_index = response
        .windows(4)
        .position(|part| part == b"\r\n\r\n")
        .ok_or_else(|| OpenHandsError::InvalidResponse("missing HTTP headers".to_owned()))?;
    let headers = std::str::from_utf8(&response[..header_index])
        .map_err(|_| OpenHandsError::InvalidResponse("HTTP headers are not UTF-8".to_owned()))?;
    let mut lines = headers.lines();
    let status_line = lines
        .next()
        .ok_or_else(|| OpenHandsError::InvalidResponse("missing HTTP status line".to_owned()))?;
    let status = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|value| value.parse::<u16>().ok())
        .ok_or_else(|| OpenHandsError::InvalidResponse("invalid HTTP status".to_owned()))?;
    let chunked = lines.any(|line| {
        line.split_once(':').is_some_and(|(name, value)| {
            name.eq_ignore_ascii_case("transfer-encoding")
                && value.to_ascii_lowercase().contains("chunked")
        })
    });
    let encoded_body = &response[header_index + 4..];
    let decoded_body = if chunked {
        decode_chunked_body(encoded_body)?
    } else {
        encoded_body.to_vec()
    };
    let body = if decoded_body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice::<Value>(&decoded_body).map_err(OpenHandsError::Json)?
    };
    if matches!(status, 404 | 422) && !has_provider_error_shape(&body) {
        return Err(OpenHandsError::Router(OpenHandsRouterError {
            status,
            native: sanitize_native_json_with_secrets(&body, secrets),
        }));
    }
    if !(200..300).contains(&status) {
        let mut error = classify_litellm_error(status, &body);
        error.native = sanitize_native_json_with_secrets(&body, secrets);
        return Err(OpenHandsError::Provider(error));
    }
    Ok(body)
}

fn has_provider_error_shape(body: &Value) -> bool {
    fn visit(value: &Value, key: Option<&str>) -> bool {
        match value {
            Value::Object(object) => object
                .iter()
                .any(|(key, value)| visit(value, Some(key.as_str()))),
            Value::Array(values) => values.iter().any(|value| visit(value, key)),
            Value::String(text) if matches!(key, Some("type" | "code")) => {
                let marker = text.to_ascii_lowercase();
                marker.contains("litellm")
                    || marker.contains("model_not_found")
                    || marker.contains("invalid_request_error")
                    || marker.contains("authenticationerror")
                    || marker.contains("ratelimiterror")
                    || marker.contains("badrequesterror")
                    || marker.contains("notfounderror")
                    || marker.contains("serviceunavailableerror")
                    || marker.contains("apitimeouterror")
            }
            _ => false,
        }
    }
    visit(body, None)
}

fn decode_chunked_body(mut bytes: &[u8]) -> Result<Vec<u8>, OpenHandsError> {
    let mut decoded = Vec::new();
    loop {
        let line_end = bytes
            .windows(2)
            .position(|part| part == b"\r\n")
            .ok_or_else(|| {
                OpenHandsError::InvalidResponse("invalid chunked response".to_owned())
            })?;
        let size_text = std::str::from_utf8(&bytes[..line_end])
            .map_err(|_| OpenHandsError::InvalidResponse("invalid chunk size".to_owned()))?
            .split(';')
            .next()
            .unwrap_or_default();
        let size = usize::from_str_radix(size_text.trim(), 16)
            .map_err(|_| OpenHandsError::InvalidResponse("invalid chunk size".to_owned()))?;
        bytes = &bytes[line_end + 2..];
        if size == 0 {
            return Ok(decoded);
        }
        if bytes.len() < size + 2 || &bytes[size..size + 2] != b"\r\n" {
            return Err(OpenHandsError::InvalidResponse(
                "truncated chunked response".to_owned(),
            ));
        }
        decoded.extend_from_slice(&bytes[..size]);
        bytes = &bytes[size + 2..];
    }
}

pub struct OpenHandsHttpAdapter {
    config: OpenHandsAdapterConfig,
    client: OpenHandsHttpClient,
    runtime: Option<HttpProcessRuntime>,
    isolated_home: Option<OpenHandsIsolatedHome>,
    event_identities: Arc<Mutex<BTreeMap<String, BTreeSet<String>>>>,
    ready: bool,
}

impl std::fmt::Debug for OpenHandsHttpAdapter {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OpenHandsHttpAdapter")
            .field("address", &self.client.address)
            .field("config", &self.config)
            .field("process_id", &self.process_id())
            .field(
                "isolated_home",
                &self.isolated_home.as_ref().map(|home| home.root()),
            )
            .field("ready", &self.ready)
            .finish()
    }
}

impl OpenHandsHttpAdapter {
    pub async fn launch(launch: OpenHandsLaunchConfig) -> Result<Self, OpenHandsError> {
        let home = OpenHandsIsolatedHome::create()?;
        let mut process = launch.process;
        process.working_directory = Some(home.root().join("server-workspace"));
        for (key, value) in home.environment() {
            process.environment.insert(key, value);
        }
        if let Some(api_key) = launch.api_key {
            process
                .environment
                .insert("OPENAI_API_KEY".to_owned(), api_key.clone());
            process
                .environment
                .insert("LLM_API_KEY".to_owned(), api_key);
        }
        let runtime = HttpProcessRuntime::spawn(process).await?;
        let client = OpenHandsHttpClient {
            address: runtime.address(),
            request_timeout: launch.adapter.request_timeout,
            launched_process: true,
            redaction_secrets: Arc::new(launch.adapter.redaction_secrets()),
        };
        let mut adapter = Self {
            config: launch.adapter,
            client,
            runtime: Some(runtime),
            isolated_home: Some(home),
            event_identities: Arc::new(Mutex::new(BTreeMap::new())),
            ready: false,
        };
        adapter.wait_until_ready().await?;
        Ok(adapter)
    }

    pub async fn connect(
        address: SocketAddr,
        config: OpenHandsAdapterConfig,
    ) -> Result<Self, OpenHandsError> {
        if !address.ip().is_loopback() {
            return Err(OpenHandsError::NonLoopbackAddress(address));
        }
        let client = OpenHandsHttpClient {
            address,
            request_timeout: config.request_timeout,
            launched_process: false,
            redaction_secrets: Arc::new(config.redaction_secrets()),
        };
        let mut adapter = Self {
            config,
            client,
            runtime: None,
            isolated_home: None,
            event_identities: Arc::new(Mutex::new(BTreeMap::new())),
            ready: false,
        };
        adapter.wait_until_ready().await?;
        Ok(adapter)
    }

    async fn wait_until_ready(&mut self) -> Result<(), OpenHandsError> {
        let deadline = Instant::now() + self.config.readiness_timeout;
        loop {
            match self.client.request("GET", "/ready", None).await {
                Ok(readiness)
                    if readiness.get("status").and_then(Value::as_str) == Some("ready") =>
                {
                    let details = self.client.request("GET", "/server_info", None).await?;
                    let actual =
                        details
                            .get("version")
                            .and_then(Value::as_str)
                            .ok_or_else(|| {
                                OpenHandsError::InvalidResponse(
                                    "OpenHands /server_info response is missing version".to_owned(),
                                )
                            })?;
                    if actual != OPENHANDS_AGENT_SERVER_VERSION {
                        return Err(OpenHandsError::UnsupportedServerVersion {
                            expected: OPENHANDS_AGENT_SERVER_VERSION,
                            actual: actual.to_owned(),
                        });
                    }
                    self.ready = true;
                    return Ok(());
                }
                Ok(_) => {}
                Err(error @ OpenHandsError::ProcessExited { .. }) => return Err(error),
                Err(_) => {}
            }
            if Instant::now() >= deadline {
                return Err(OpenHandsError::ReadinessTimeout {
                    address: self.client.address,
                });
            }
            sleep(self.config.poll_interval).await;
        }
    }

    pub fn address(&self) -> SocketAddr {
        self.client.address
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }

    pub fn process_id(&self) -> Option<u32> {
        self.runtime
            .as_ref()
            .and_then(HttpProcessRuntime::process_id)
    }

    pub fn isolated_home(&self) -> Option<&Path> {
        self.isolated_home.as_ref().map(OpenHandsIsolatedHome::root)
    }

    pub async fn create_conversation(
        &mut self,
        policy: CapabilityDowngradePolicy,
    ) -> Result<OpenHandsConversation, OpenHandsError> {
        let config = OpenHandsConversationConfig::from(&self.config);
        self.start_conversation(None, &config, policy).await
    }

    pub async fn create_conversation_with_config(
        &mut self,
        config: &OpenHandsConversationConfig,
        policy: CapabilityDowngradePolicy,
    ) -> Result<OpenHandsConversation, OpenHandsError> {
        self.start_conversation(None, config, policy).await
    }

    pub async fn resume_conversation(
        &mut self,
        conversation_id: &str,
        policy: CapabilityDowngradePolicy,
    ) -> Result<OpenHandsConversation, OpenHandsError> {
        let config = OpenHandsConversationConfig::from(&self.config);
        self.start_conversation(Some(conversation_id), &config, policy)
            .await
    }

    pub async fn resume_conversation_with_config(
        &mut self,
        conversation_id: &str,
        config: &OpenHandsConversationConfig,
        policy: CapabilityDowngradePolicy,
    ) -> Result<OpenHandsConversation, OpenHandsError> {
        self.start_conversation(Some(conversation_id), config, policy)
            .await
    }

    async fn start_conversation(
        &self,
        conversation_id: Option<&str>,
        config: &OpenHandsConversationConfig,
        policy: CapabilityDowngradePolicy,
    ) -> Result<OpenHandsConversation, OpenHandsError> {
        let prepared = prepare_conversation_request_with_config(config, conversation_id)?;
        prepared.authorize(policy)?;
        let downgrades = prepared.downgrades.clone();
        let response = self
            .client
            .request("POST", "/api/conversations", Some(&prepared.body))
            .await?;
        decode_conversation(&response, downgrades, &self.client.redaction_secrets)
    }

    pub async fn delete_conversation(&self, conversation_id: &str) -> Result<(), OpenHandsError> {
        let path = format!(
            "/api/conversations/{}",
            encode_path_segment(conversation_id)
        );
        self.client.request("DELETE", &path, None).await?;
        Ok(())
    }

    pub async fn respond_to_approval(
        &self,
        conversation_id: &str,
        accept: bool,
        reason: &str,
    ) -> Result<(), OpenHandsError> {
        let path = format!(
            "/api/conversations/{}/events/respond_to_confirmation",
            encode_path_segment(conversation_id)
        );
        self.client
            .request(
                "POST",
                &path,
                Some(&json!({"accept": accept, "reason": reason})),
            )
            .await?;
        Ok(())
    }

    pub async fn cancel(&self, conversation_id: &str) -> Result<(), OpenHandsError> {
        let path = format!(
            "/api/conversations/{}/interrupt",
            encode_path_segment(conversation_id)
        );
        self.client.request("POST", &path, Some(&json!({}))).await?;
        Ok(())
    }

    pub async fn prompt_stream(
        &self,
        conversation_id: &str,
        prompt: &str,
    ) -> Result<OpenHandsEventStream, OpenHandsError> {
        if prompt.is_empty() {
            return Err(OpenHandsError::InvalidRequest(
                "prompt must not be empty".to_owned(),
            ));
        }
        let encoded_id = encode_path_segment(conversation_id);
        let historical_events = fetch_event_search(&self.client, &encoded_id).await?;
        {
            let mut identities = self.event_identities.lock().await;
            let conversation_identities = identities.entry(encoded_id.clone()).or_default();
            conversation_identities.extend(historical_events.iter().map(event_identity));
        }
        let event_path = format!("/api/conversations/{encoded_id}/events");
        let message = json!({
            "role": "user",
            "content": [{"type": "text", "text": prompt, "cache_prompt": false}],
            "run": true
        });
        self.client
            .request("POST", &event_path, Some(&message))
            .await?;

        let (sender, receiver) = mpsc::channel(EVENT_PAGE_LIMIT + 2);
        let (cancel, cancel_receiver) = oneshot::channel();
        let completed = Arc::new(AtomicBool::new(false));
        let task_completed = Arc::clone(&completed);
        let client = self.client.clone();
        let poll_interval = self.config.poll_interval;
        let stream_timeout = self.config.request_timeout;
        let event_identities = Arc::clone(&self.event_identities);
        tokio::spawn(async move {
            tokio::select! {
                biased;
                _ = cancel_receiver => {
                    let cancel_path = format!("/api/conversations/{encoded_id}/interrupt");
                    let _ = client.request("POST", &cancel_path, Some(&json!({}))).await;
                    task_completed.store(true, Ordering::Release);
                }
                result = poll_prompt_events(
                    client.clone(),
                    encoded_id.clone(),
                    poll_interval,
                    stream_timeout,
                    event_identities,
                    sender.clone(),
                ) => {
                    task_completed.store(true, Ordering::Release);
                    if let Err(error) = result {
                        let _ = sender.send(Err(error)).await;
                    }
                }
            }
        });
        Ok(OpenHandsEventStream {
            receiver: ReceiverStream::new(receiver),
            cancel: Some(cancel),
            completed,
        })
    }

    pub async fn shutdown(mut self) -> Result<(), OpenHandsError> {
        self.ready = false;
        if let Some(runtime) = self.runtime.take()
            && let Err(error) = runtime.shutdown().await
        {
            // Preserve the isolated home when process exit could not be confirmed.
            // Deleting it while a child may still be using it is unsafe.
            if let Some(home) = self.isolated_home.take() {
                std::mem::forget(home);
            }
            return Err(OpenHandsError::Io(error));
        }
        self.isolated_home.take();
        Ok(())
    }
}

async fn poll_prompt_events(
    client: OpenHandsHttpClient,
    conversation_id: String,
    poll_interval: Duration,
    stream_timeout: Duration,
    event_identities: Arc<Mutex<BTreeMap<String, BTreeSet<String>>>>,
    sender: mpsc::Sender<Result<OpenHandsStreamItem, OpenHandsError>>,
) -> Result<(), OpenHandsError> {
    let deadline = Instant::now() + stream_timeout;
    let mut final_text = None;
    let mut usage = None;
    let mut pending_actions = Vec::new();
    let mut emitted_approvals = BTreeSet::new();
    let mut terminal_provider_error = None;
    loop {
        if Instant::now() >= deadline {
            return Err(OpenHandsError::Timeout);
        }
        let items = fetch_event_search(&client, &conversation_id).await?;
        for native in items {
            let identity = event_identity(&native);
            let is_new = {
                let mut identities = event_identities.lock().await;
                identities
                    .entry(conversation_id.clone())
                    .or_default()
                    .insert(identity)
            };
            if !is_new {
                continue;
            }
            let event = decode_native_event_with_secrets(&native, &client.redaction_secrets)?;
            if event.final_text.is_some() {
                final_text = event.final_text.clone();
            }
            if event.usage.is_some() {
                usage = event.usage.clone();
            }
            if event.provider_error.is_some() {
                terminal_provider_error = event.provider_error.clone();
            }
            if event.category == OpenHandsEventCategory::Action {
                pending_actions.push(event.clone());
            }
            if sender
                .send(Ok(OpenHandsStreamItem::Event(event)))
                .await
                .is_err()
            {
                return Ok(());
            }
        }
        let conversation_path = format!("/api/conversations/{conversation_id}");
        let conversation = client.request("GET", &conversation_path, None).await?;
        // Conversation statistics are cumulative and can start at zero while
        // the model runs. Refresh them on every poll through terminal state.
        if let Some(current_usage) = conversation
            .get("metrics")
            .filter(|value| !value.is_null())
            .or_else(|| {
                conversation
                    .get("stats")
                    .and_then(|stats| stats.get("usage_to_metrics"))
                    .filter(|value| !value.is_null())
            })
            .or_else(|| conversation.get("usage"))
            .filter(|value| !value.is_null())
            .map(|value| sanitize_native_json_with_secrets(value, &client.redaction_secrets))
        {
            usage = Some(current_usage);
        }
        let status = conversation
            .get("execution_status")
            .or_else(|| conversation.get("status"))
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_ascii_lowercase();
        if status == "waiting_for_confirmation" {
            for action in &pending_actions {
                let identity = action
                    .native_cursor
                    .clone()
                    .unwrap_or_else(|| event_identity(&action.native));
                if emitted_approvals.insert(identity)
                    && sender
                        .send(Ok(OpenHandsStreamItem::ApprovalRequired {
                            action: action.clone(),
                        }))
                        .await
                        .is_err()
                {
                    return Ok(());
                }
            }
        }
        if status == "error" || status == "failed" {
            let provider_error =
                terminal_provider_error.unwrap_or_else(|| OpenHandsProviderError {
                    kind: OpenHandsProviderErrorKind::Unknown,
                    status: None,
                    retryable: false,
                    native: sanitize_native_json_with_secrets(
                        conversation
                            .get("last_error")
                            .or_else(|| conversation.get("error"))
                            .unwrap_or(&conversation),
                        &client.redaction_secrets,
                    ),
                });
            return Err(OpenHandsError::Provider(provider_error));
        }
        let terminal = matches!(
            status.as_str(),
            "finished" | "paused" | "stopped" | "cancelled"
        );
        if terminal {
            if final_text.is_none() {
                let final_path =
                    format!("/api/conversations/{conversation_id}/agent_final_response");
                let response = client.request("GET", &final_path, None).await?;
                final_text = response
                    .get("response")
                    .and_then(Value::as_str)
                    .filter(|text| !text.is_empty())
                    .map(|text| redact_native_text(text, &client.redaction_secrets));
            }
            let _ = sender
                .send(Ok(OpenHandsStreamItem::Completed { final_text, usage }))
                .await;
            return Ok(());
        }
        sleep(poll_interval).await;
    }
}

async fn fetch_event_search(
    client: &OpenHandsHttpClient,
    conversation_id: &str,
) -> Result<Vec<Value>, OpenHandsError> {
    let mut items = Vec::new();
    let mut page_id = None::<String>;
    let mut page_ids = BTreeSet::new();
    loop {
        let mut path = format!(
            "/api/conversations/{conversation_id}/events/search?limit={EVENT_PAGE_LIMIT}&sort_order=TIMESTAMP"
        );
        if let Some(cursor) = &page_id {
            path.push_str("&page_id=");
            path.push_str(&encode_query_component(cursor));
        }
        let page = client.request("GET", &path, None).await?;
        let page_items = page.get("items").and_then(Value::as_array).ok_or_else(|| {
            OpenHandsError::InvalidResponse("event search response is missing items".to_owned())
        })?;
        items.extend(page_items.iter().cloned());
        page_id = page
            .get("next_page_id")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned);
        let Some(cursor) = &page_id else {
            return Ok(items);
        };
        if !page_ids.insert(cursor.clone()) {
            return Err(OpenHandsError::InvalidResponse(
                "event search returned a repeated next_page_id".to_owned(),
            ));
        }
    }
}

fn event_identity(value: &Value) -> String {
    value
        .get("id")
        .or_else(|| value.get("event_id"))
        .and_then(Value::as_str)
        .map(|id| format!("id:{id}"))
        .unwrap_or_else(|| format!("native:{}", sanitize_native_json(value)))
}

fn decode_conversation(
    value: &Value,
    downgrades: Vec<OpenHandsCapabilityDowngrade>,
    secrets: &[String],
) -> Result<OpenHandsConversation, OpenHandsError> {
    let id = value
        .get("id")
        .or_else(|| value.get("conversation_id"))
        .and_then(Value::as_str)
        .filter(|id| !id.is_empty())
        .ok_or_else(|| {
            OpenHandsError::InvalidResponse("conversation response is missing id".to_owned())
        })?
        .to_owned();
    let status = value
        .get("execution_status")
        .or_else(|| value.get("status"))
        .and_then(Value::as_str)
        .unwrap_or("unknown")
        .to_owned();
    Ok(OpenHandsConversation {
        id,
        status,
        native: sanitize_native_json_with_secrets(value, secrets),
        downgrades,
    })
}

fn encode_path_segment(value: &str) -> String {
    percent_encode(value)
}

fn encode_query_component(value: &str) -> String {
    percent_encode(value)
}

fn percent_encode(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(byte as char);
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    encoded
}
