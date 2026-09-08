use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::Digest;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tokio::time::timeout;
use tokio_stream::StreamExt;
use uuid::Uuid;

use super::acp::AcpV1Codec;
use super::adapter::{OmniSoloEvent, OmniSoloHarnessAdapter, OmniSoloRunConfig};
use super::capsule::SessionCapsule;
use super::codex_app_server::CodexAppServerV2Codec;
use super::deepseek_harness::DeepSeekHarnessCodec;
use super::http_runtime::HttpProcessConfig;
use super::inference::OpenAiResponsesClient;
use super::local_services::{LOCAL_SERVICE_BUNDLE_SCHEMA, LocalServiceBundle};
use super::json_rpc::{JsonRpcError, JsonRpcProcessConfig, JsonRpcProcessRuntime};
use super::opencode::{OpenCodeEventCorrelation, OpenCodeHttpAdapter, OpenCodeProcessConfig};
use super::openhands::{
    CapabilityDowngradePolicy, OpenHandsAdapterConfig, OpenHandsCapabilityDowngrade,
    OpenHandsConversationConfig, OpenHandsError, OpenHandsEvent, OpenHandsEventCategory,
    OpenHandsHttpAdapter, OpenHandsLaunchConfig, OpenHandsProviderError, OpenHandsReasoningSupport,
    OpenHandsRouterError, OpenHandsStreamItem,
};
use super::openharness::{
    OpenHarnessCommand, OpenHarnessEvent, OpenHarnessEventCorrelation, OpenHarnessEventDecoder,
    OpenHarnessProcessConfig, OpenHarnessResolvedModel, OpenHarnessRuntime,
};
use super::pi_rpc::{
    PI_PROVIDER_ID, PiEventCorrelation, PiIsolatedHome, PiRpcCommand, PiRpcProcessConfig,
    PiRpcRuntime, thinking_level,
};
use super::process_env::apply_isolated_environment;
use super::protocol::{
    AttemptOperation, HarnessExecutionItem, HarnessExecutionStream, HarnessProtocolCodec,
    NativeTurnState, ProtocolProcessAdapter, SessionOperation,
};
use super::types::{
    CapabilitySnapshot, JsonMap, ModelApiDialect, ResolvedModelSelection, sensitive_json_key,
};

const PROCESS_PROTOCOL_VERSION: u32 = 1;
const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HarnessProtocolKind {
    OmniSolo,
    CodexAppServer,
    #[serde(
        rename = "opencode_http",
        alias = "opencode_json_rpc",
        alias = "open_code_json_rpc"
    )]
    OpenCodeHttp,
    #[serde(rename = "deepseek_json_rpc", alias = "deep_seek_json_rpc")]
    DeepSeekJsonRpc,
    #[serde(alias = "pi_jsonl")]
    PiRpc,
    #[serde(alias = "kimi_json_rpc", alias = "kimi_jsonrpc", alias = "acp_v1")]
    KimiAcp,
    #[serde(rename = "openhands_http", alias = "open_hands_http")]
    OpenHandsHttp,
    #[serde(
        rename = "openharness_sdk",
        alias = "openharness_acp",
        alias = "open_harness_acp"
    )]
    OpenHarnessSdk,
    #[serde(rename = "openai_compatible_shim")]
    OpenAiCompatibleShim,
    Custom,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeFamily {
    InProcess,
    JsonRpc,
    JsonLines,
    Http,
    LegacyJsonLines,
}

impl HarnessProtocolKind {
    pub fn runtime_family(&self) -> RuntimeFamily {
        match self {
            Self::OmniSolo => RuntimeFamily::InProcess,
            Self::CodexAppServer | Self::DeepSeekJsonRpc | Self::KimiAcp => RuntimeFamily::JsonRpc,
            Self::PiRpc | Self::OpenHarnessSdk => RuntimeFamily::JsonLines,
            Self::OpenCodeHttp | Self::OpenHandsHttp => RuntimeFamily::Http,
            Self::OpenAiCompatibleShim | Self::Custom => RuntimeFamily::LegacyJsonLines,
        }
    }

    pub fn uses_legacy_json_lines(&self) -> bool {
        self.runtime_family() == RuntimeFamily::LegacyJsonLines
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HarnessIntegrationMode {
    #[default]
    Native,
    #[serde(rename = "openai_compatible")]
    OpenAiCompatible,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HarnessCapability {
    Prompt,
    Streaming,
    Cancellation,
    Steering,
    Approvals,
    Questions,
    Workspace,
    Artifacts,
    Compaction,
    Branching,
    Subagents,
    ExactResume,
    ImportCapsule,
    ExportNativeState,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HarnessDescriptor {
    pub harness_id: String,
    pub adapter_id: String,
    pub display_name: String,
    pub implementation_version: String,
    pub protocol_kind: HarnessProtocolKind,
    #[serde(default)]
    pub integration_mode: HarnessIntegrationMode,
    pub protocol_version: String,
    pub source_revision: Option<String>,
    pub schema_revision: Option<String>,
    pub capabilities: BTreeSet<HarnessCapability>,
    pub worker_image: Option<String>,
    pub worker_pool: Option<String>,
    pub state_locality: Option<String>,
    pub native_extension_namespace: String,
    pub metadata: BTreeMap<String, String>,
}

impl HarnessDescriptor {
    pub fn capability_snapshot(
        &self,
        snapshot_id: Uuid,
        capability_version: u64,
        captured_at: chrono::DateTime<chrono::Utc>,
    ) -> CapabilitySnapshot {
        let capabilities = self
            .capabilities
            .iter()
            .map(|capability| serde_json::to_string(capability).unwrap_or_default())
            .map(|capability| capability.trim_matches('"').to_owned())
            .collect::<BTreeSet<_>>();
        let digest = format!(
            "{:x}",
            sha2::Sha256::digest(serde_json::to_vec(&capabilities).unwrap_or_default())
        );
        CapabilitySnapshot {
            snapshot_id,
            subject_kind: "harness".to_owned(),
            subject_id: self.harness_id.clone(),
            capability_version,
            capabilities,
            source_revision: self.source_revision.clone(),
            captured_at,
            expires_at: None,
            digest,
            metadata: self
                .metadata
                .iter()
                .map(|(key, value)| (key.clone(), serde_json::Value::String(value.clone())))
                .collect(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ProcessHarnessSpec {
    pub executable: String,
    pub args: Vec<String>,
    pub harness_id: String,
    pub protocol_kind: HarnessProtocolKind,
    pub environment: BTreeMap<String, String>,
    pub resolved_model: Option<ResolvedModelSelection>,
    pub api_base_url: Option<String>,
    pub working_directory: PathBuf,
    pub request_timeout: Duration,
}

impl std::fmt::Debug for ProcessHarnessSpec {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProcessHarnessSpec")
            .field("executable", &self.executable)
            .field("args", &self.args)
            .field("harness_id", &self.harness_id)
            .field("protocol_kind", &self.protocol_kind)
            .field("environment", &redacted_environment(&self.environment))
            .field(
                "provider_route",
                &self
                    .resolved_model
                    .as_ref()
                    .map(|selection| &selection.provider_route),
            )
            .field(
                "model_id",
                &self
                    .resolved_model
                    .as_ref()
                    .map(|selection| &selection.model_id),
            )
            .field(
                "reasoning_effort",
                &self
                    .resolved_model
                    .as_ref()
                    .and_then(|selection| selection.reasoning_effort.as_ref()),
            )
            .field(
                "api_base_url",
                &self.api_base_url.as_ref().map(|_| "<configured>"),
            )
            .field("working_directory", &self.working_directory)
            .field("request_timeout", &self.request_timeout)
            .finish()
    }
}

fn redacted_environment(environment: &BTreeMap<String, String>) -> BTreeMap<String, String> {
    environment
        .iter()
        .map(|(key, value)| {
            let value = if debug_safe_environment_key(key) && !sensitive_json_key(key) {
                value.clone()
            } else {
                "<redacted>".to_owned()
            };
            (key.clone(), value)
        })
        .collect()
}

fn debug_safe_environment_key(key: &str) -> bool {
    matches!(key, "PATH" | "RUST_LOG" | "OPENAI_MODEL")
}

impl ProcessHarnessSpec {
    pub fn command<I, S>(
        executable: impl Into<String>,
        args: I,
        harness_id: impl Into<String>,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let harness_id = harness_id.into();
        Self {
            executable: executable.into(),
            args: args.into_iter().map(Into::into).collect(),
            protocol_kind: protocol_for_harness(&harness_id),
            harness_id,
            environment: BTreeMap::new(),
            resolved_model: None,
            api_base_url: None,
            working_directory: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
            request_timeout: DEFAULT_REQUEST_TIMEOUT,
        }
    }

    pub fn with_protocol(mut self, protocol_kind: HarnessProtocolKind) -> Self {
        self.protocol_kind = protocol_kind;
        self
    }

    pub fn with_environment(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.environment.insert(key.into(), value.into());
        self
    }

    pub fn with_model_routing(
        mut self,
        resolved_model: ResolvedModelSelection,
        api_base_url: Option<String>,
    ) -> Self {
        self.resolved_model = Some(resolved_model);
        self.api_base_url = api_base_url;
        self
    }

    pub fn with_timeout(mut self, request_timeout: Duration) -> Self {
        self.request_timeout = request_timeout;
        self
    }

    pub fn with_working_directory(mut self, working_directory: impl Into<PathBuf>) -> Self {
        self.working_directory = working_directory.into();
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExternalHarnessPreset {
    Codex,
    OpenCode,
    DeepSeek,
    Pi,
    Kimi,
    OpenHands,
    OpenHarness,
    Aider,
    Goose,
    OpenInterpreter,
    Plandex,
}

impl ExternalHarnessPreset {
    pub fn from_harness_id(harness_id: &str) -> Option<Self> {
        match harness_id {
            "codex" => Some(Self::Codex),
            "opencode" => Some(Self::OpenCode),
            "deepseek" => Some(Self::DeepSeek),
            "pi" => Some(Self::Pi),
            "kimi" => Some(Self::Kimi),
            "openhands" => Some(Self::OpenHands),
            "openharness" => Some(Self::OpenHarness),
            "aider" => Some(Self::Aider),
            "goose" => Some(Self::Goose),
            "open-interpreter" => Some(Self::OpenInterpreter),
            "plandex" => Some(Self::Plandex),
            _ => None,
        }
    }

    pub fn harness_id(self) -> &'static str {
        match self {
            Self::Codex => "codex",
            Self::OpenCode => "opencode",
            Self::DeepSeek => "deepseek",
            Self::Pi => "pi",
            Self::Kimi => "kimi",
            Self::OpenHands => "openhands",
            Self::OpenHarness => "openharness",
            Self::Aider => "aider",
            Self::Goose => "goose",
            Self::OpenInterpreter => "open-interpreter",
            Self::Plandex => "plandex",
        }
    }

    pub fn protocol_kind(self) -> HarnessProtocolKind {
        match self {
            Self::Codex => HarnessProtocolKind::CodexAppServer,
            Self::OpenCode => HarnessProtocolKind::OpenCodeHttp,
            Self::DeepSeek => HarnessProtocolKind::DeepSeekJsonRpc,
            Self::Pi => HarnessProtocolKind::PiRpc,
            Self::Kimi => HarnessProtocolKind::KimiAcp,
            Self::OpenHands => HarnessProtocolKind::OpenHandsHttp,
            Self::OpenHarness => HarnessProtocolKind::OpenHarnessSdk,
            Self::Aider | Self::Goose | Self::OpenInterpreter | Self::Plandex => {
                HarnessProtocolKind::OpenAiCompatibleShim
            }
        }
    }

    pub fn implementation_version(self) -> &'static str {
        match self {
            Self::Codex => "0.149.0",
            Self::OpenCode => "1.18.15",
            Self::DeepSeek => "0.1.1-rc.2",
            Self::Pi => "0.73.1",
            Self::Kimi => "1.49.0",
            Self::OpenHands => "1.43.1",
            Self::OpenHarness => "0.6.0",
            Self::Aider => "0.86.0",
            Self::Goose => "1.33.1",
            Self::OpenInterpreter => "0.4.2",
            Self::Plandex => "cli/v2.2.1",
        }
    }

    pub fn worker_image(self) -> String {
        if let Self::Plandex = self {
            return "omnisolo/harness-worker-plandex:2.2.1".to_owned();
        }
        format!(
            "omnisolo/harness-worker-{}:{}",
            self.harness_id(),
            self.implementation_version()
        )
    }

    pub fn capabilities(self) -> BTreeSet<HarnessCapability> {
        use HarnessCapability as Capability;

        let capabilities: &[Capability] = match self {
            Self::Codex => &[
                Capability::Prompt,
                Capability::Streaming,
                Capability::Cancellation,
                Capability::Steering,
                Capability::Approvals,
                Capability::Questions,
                Capability::Workspace,
                Capability::Branching,
                Capability::ExactResume,
                Capability::ImportCapsule,
                Capability::ExportNativeState,
            ],
            Self::OpenCode => &[
                Capability::Prompt,
                Capability::Streaming,
                Capability::Cancellation,
                Capability::Workspace,
                Capability::ExactResume,
                Capability::ImportCapsule,
            ],
            Self::DeepSeek => &[
                Capability::Prompt,
                Capability::Streaming,
                Capability::Workspace,
                Capability::ImportCapsule,
            ],
            Self::Pi => &[
                Capability::Prompt,
                Capability::Streaming,
                Capability::Cancellation,
                Capability::Steering,
                Capability::Workspace,
                Capability::ImportCapsule,
            ],
            Self::Kimi => &[
                Capability::Prompt,
                Capability::Streaming,
                Capability::Cancellation,
                Capability::Approvals,
                Capability::Questions,
                Capability::Workspace,
                Capability::ExactResume,
                Capability::ImportCapsule,
            ],
            Self::OpenHands => &[
                Capability::Prompt,
                Capability::Streaming,
                Capability::Cancellation,
                Capability::Approvals,
                Capability::Workspace,
                Capability::ExactResume,
                Capability::ImportCapsule,
            ],
            Self::OpenHarness => &[
                Capability::Prompt,
                Capability::Streaming,
                Capability::Cancellation,
                Capability::Steering,
                Capability::Workspace,
                Capability::ExactResume,
                Capability::ImportCapsule,
            ],
            Self::Aider | Self::Goose | Self::OpenInterpreter | Self::Plandex => &[
                Capability::Prompt,
                Capability::Streaming,
                Capability::Cancellation,
                Capability::Steering,
                Capability::Workspace,
                Capability::ImportCapsule,
            ],
        };
        capabilities.iter().cloned().collect()
    }

    pub fn integration_mode(self) -> HarnessIntegrationMode {
        match self {
            Self::Aider | Self::Goose | Self::OpenInterpreter | Self::Plandex => {
                HarnessIntegrationMode::OpenAiCompatible
            }
            _ => HarnessIntegrationMode::Native,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct HarnessRegistry {
    descriptors: BTreeMap<String, HarnessDescriptor>,
    presets: BTreeMap<String, ProcessHarnessSpec>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum HarnessRegistryError {
    UnknownHarness(String),
    MissingProcessPreset(String),
}

impl std::fmt::Display for HarnessRegistryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownHarness(harness_id) => write!(formatter, "unknown harness: {harness_id}"),
            Self::MissingProcessPreset(harness_id) => {
                write!(formatter, "harness has no process preset: {harness_id}")
            }
        }
    }
}

impl std::error::Error for HarnessRegistryError {}

impl HarnessRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        registry.register_descriptor(native_descriptor());
        registry.register_preset(
            ProcessHarnessSpec::command("omnisolo", std::iter::empty::<String>(), "omnisolo")
                .with_protocol(HarnessProtocolKind::OmniSolo),
        );
        for preset in [
            ExternalHarnessPreset::Codex,
            ExternalHarnessPreset::OpenCode,
            ExternalHarnessPreset::DeepSeek,
            ExternalHarnessPreset::Pi,
            ExternalHarnessPreset::Kimi,
            ExternalHarnessPreset::OpenHands,
            ExternalHarnessPreset::OpenHarness,
            ExternalHarnessPreset::Aider,
            ExternalHarnessPreset::Goose,
            ExternalHarnessPreset::OpenInterpreter,
            ExternalHarnessPreset::Plandex,
        ] {
            let harness_id = preset.harness_id();
            let executable = match preset {
                ExternalHarnessPreset::Kimi => "omnisolo-kimi-acp",
                ExternalHarnessPreset::OpenHands => "openhands-agent-server",
                ExternalHarnessPreset::Aider
                | ExternalHarnessPreset::Goose
                | ExternalHarnessPreset::OpenInterpreter
                | ExternalHarnessPreset::Plandex => "omnisolo-openai-shim",
                _ => harness_id,
            };
            registry.register_descriptor(external_descriptor(preset));
            registry.register_preset(
                ProcessHarnessSpec::command(executable, std::iter::empty::<String>(), harness_id)
                    .with_protocol(preset.protocol_kind()),
            );
        }
        registry
    }

    pub fn register_descriptor(&mut self, descriptor: HarnessDescriptor) {
        self.descriptors
            .insert(descriptor.harness_id.clone(), descriptor);
    }

    pub fn register_preset(&mut self, spec: ProcessHarnessSpec) {
        self.presets.insert(spec.harness_id.clone(), spec);
    }

    pub fn descriptor(&self, harness_id: &str) -> Option<&HarnessDescriptor> {
        self.descriptors.get(harness_id)
    }

    pub fn preset(&self, harness_id: &str) -> Option<&ProcessHarnessSpec> {
        self.presets.get(harness_id)
    }

    pub fn build_process_adapter(
        &self,
        harness_id: &str,
    ) -> Result<ProcessHarnessAdapter, HarnessRegistryError> {
        if self.descriptor(harness_id).is_none() {
            return Err(HarnessRegistryError::UnknownHarness(harness_id.to_owned()));
        }
        let spec = self
            .preset(harness_id)
            .cloned()
            .ok_or_else(|| HarnessRegistryError::MissingProcessPreset(harness_id.to_owned()))?;
        Ok(ProcessHarnessAdapter::new(spec))
    }

    pub fn build_adapter(
        &self,
        harness_id: &str,
    ) -> Result<Box<dyn HarnessAdapter>, HarnessRegistryError> {
        if self.descriptor(harness_id).is_none() {
            return Err(HarnessRegistryError::UnknownHarness(harness_id.to_owned()));
        }
        if harness_id == "omnisolo" {
            return Ok(Box::new(OmniSoloHarnessAdapterBridge::new(
                native_descriptor(),
            )));
        }
        Ok(Box::new(self.build_process_adapter(harness_id)?))
    }

    pub fn descriptors(&self) -> impl Iterator<Item = &HarnessDescriptor> {
        self.descriptors.values()
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HarnessSessionRequest {
    pub request_id: Uuid,
    pub tenant_id: String,
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub turn_id: Option<Uuid>,
    pub objective: String,
    pub capsule: Option<SessionCapsule>,
    pub metadata: BTreeMap<String, String>,
    pub model_binding_id: Option<Uuid>,
    #[serde(default)]
    pub resolved_model: Option<ResolvedModelSelection>,
    #[serde(default)]
    pub local_service_bundle: Option<LocalServiceBundle>,
    #[serde(default)]
    pub attempt_id: Option<Uuid>,
    pub runtime_config_snapshot_id: Option<Uuid>,
    pub workspace_snapshot_id: Option<Uuid>,
    pub artifact_ids: Vec<Uuid>,
    pub durable_sequence: Option<i64>,
    pub capability_snapshot_id: Option<Uuid>,
    pub extensions: JsonMap,
}

impl HarnessSessionRequest {
    pub fn new(tenant_id: impl Into<String>, session_id: Uuid, request_id: Uuid) -> Self {
        Self {
            request_id,
            tenant_id: tenant_id.into(),
            session_id,
            task_id: None,
            turn_id: None,
            objective: String::new(),
            capsule: None,
            metadata: BTreeMap::new(),
            model_binding_id: None,
            resolved_model: None,
            local_service_bundle: None,
            attempt_id: None,
            runtime_config_snapshot_id: None,
            workspace_snapshot_id: None,
            artifact_ids: Vec::new(),
            durable_sequence: None,
            capability_snapshot_id: None,
            extensions: JsonMap::new(),
        }
    }

    pub fn with_task(mut self, task_id: Uuid, objective: impl Into<String>) -> Self {
        self.task_id = Some(task_id);
        self.objective = objective.into();
        self
    }

    pub fn with_turn(mut self, turn_id: Uuid) -> Self {
        self.turn_id = Some(turn_id);
        self
    }

    pub fn with_capsule(mut self, capsule: SessionCapsule) -> Self {
        self.capsule = Some(capsule);
        self
    }

    pub fn with_resolved_model(mut self, resolved_model: ResolvedModelSelection) -> Self {
        self.resolved_model = Some(resolved_model);
        self
    }

    pub fn with_local_service_bundle(mut self, bundle: LocalServiceBundle) -> Self {
        self.local_service_bundle = Some(bundle);
        self
    }

    pub fn with_attempt_id(mut self, attempt_id: Uuid) -> Self {
        self.attempt_id = Some(attempt_id);
        self
    }

    pub fn with_transfer_pointers(
        mut self,
        model_binding_id: Option<Uuid>,
        runtime_config_snapshot_id: Option<Uuid>,
        workspace_snapshot_id: Option<Uuid>,
        artifact_ids: Vec<Uuid>,
        durable_sequence: Option<i64>,
    ) -> Self {
        self.model_binding_id = model_binding_id;
        self.runtime_config_snapshot_id = runtime_config_snapshot_id;
        self.workspace_snapshot_id = workspace_snapshot_id;
        self.artifact_ids = artifact_ids;
        self.durable_sequence = durable_sequence;
        self
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NativeSession {
    pub native_session_id: String,
    pub native_cursor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HarnessEvent {
    pub event_type: String,
    pub durable: bool,
    pub payload: Value,
    pub native_cursor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HarnessExecution {
    pub events: Vec<HarnessEvent>,
    pub final_text: Option<String>,
    pub usage: Option<Value>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NativeCheckpoint {
    pub checkpoint_ref: String,
    pub native_cursor: Option<String>,
}

#[derive(Clone, Debug)]
pub struct HarnessAttemptRequest {
    pub session: HarnessSessionRequest,
    pub attempt_id: String,
    pub prompt: String,
    pub native_session_id: Option<String>,
    pub checkpoint_ref: Option<String>,
}

#[derive(Debug)]
pub enum HarnessAdapterError {
    InvalidRequest(String),
    Spawn(std::io::Error),
    Io(std::io::Error),
    Json(serde_json::Error),
    Timeout,
    ProcessExited,
    RequestMismatch { expected: Uuid, actual: Uuid },
    Remote(String),
    Capsule(String),
    InvalidResponse(String),
    OpenHandsProvider(OpenHandsProviderError),
    OpenHandsRouter(OpenHandsRouterError),
}

impl std::fmt::Display for HarnessAdapterError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRequest(message) => write!(formatter, "invalid request: {message}"),
            Self::Spawn(error) | Self::Io(error) => error.fmt(formatter),
            Self::Json(error) => error.fmt(formatter),
            Self::Timeout => formatter.write_str("harness request timed out"),
            Self::ProcessExited => formatter.write_str("harness process exited"),
            Self::RequestMismatch { expected, actual } => {
                write!(
                    formatter,
                    "request id mismatch: expected {expected}, got {actual}"
                )
            }
            Self::Remote(error) => write!(formatter, "harness rejected request: {error}"),
            Self::Capsule(error) => write!(formatter, "invalid capsule: {error}"),
            Self::InvalidResponse(error) => write!(formatter, "invalid harness response: {error}"),
            Self::OpenHandsProvider(error) => error.fmt(formatter),
            Self::OpenHandsRouter(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for HarnessAdapterError {}

impl From<JsonRpcError> for HarnessAdapterError {
    fn from(error: JsonRpcError) -> Self {
        classify_protocol_error(error)
    }
}

#[async_trait]
pub trait HarnessAdapter: Send {
    fn descriptor(&self) -> &HarnessDescriptor;
    fn supports_concurrent_streaming(&self) -> bool {
        false
    }
    async fn preflight(
        &mut self,
        _request: HarnessSessionRequest,
    ) -> Result<(), HarnessAdapterError> {
        Ok(())
    }
    async fn create_session(
        &mut self,
        request: HarnessSessionRequest,
    ) -> Result<NativeSession, HarnessAdapterError>;
    async fn fork_session(
        &mut self,
        request: HarnessSessionRequest,
        native_session_id: Option<&str>,
    ) -> Result<NativeSession, HarnessAdapterError> {
        let _ = native_session_id;
        self.create_session(request).await
    }
    async fn import_session(
        &mut self,
        request: HarnessSessionRequest,
        capsule: SessionCapsule,
    ) -> Result<NativeSession, HarnessAdapterError>;
    async fn resume_session(
        &mut self,
        request: HarnessSessionRequest,
        native_session_id: &str,
    ) -> Result<NativeSession, HarnessAdapterError>;
    async fn execute(
        &mut self,
        request: HarnessSessionRequest,
        attempt_id: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecution, HarnessAdapterError>;
    async fn attempt_stream(
        &mut self,
        operation: AttemptOperation,
        request: HarnessSessionRequest,
        attempt_id: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecutionStream, HarnessAdapterError> {
        let execution = match operation {
            AttemptOperation::Start => {
                self.start_attempt(request, attempt_id, prompt, native_session_id)
                    .await?
            }
            AttemptOperation::Execute => {
                self.execute(request, attempt_id, prompt, native_session_id)
                    .await?
            }
            AttemptOperation::Resume => {
                self.resume_attempt(request, attempt_id, prompt, native_session_id)
                    .await?
            }
            AttemptOperation::Steer
            | AttemptOperation::Cancel
            | AttemptOperation::Quiesce
            | AttemptOperation::Reconcile => {
                let operation_name = match operation {
                    AttemptOperation::Steer => "steer",
                    AttemptOperation::Cancel => "cancel",
                    AttemptOperation::Quiesce => "quiesce",
                    AttemptOperation::Reconcile => "reconcile",
                    AttemptOperation::Start
                    | AttemptOperation::Execute
                    | AttemptOperation::Resume => unreachable!(),
                };
                self.control_attempt(
                    request,
                    attempt_id,
                    operation_name,
                    prompt,
                    native_session_id,
                )
                .await?
            }
        };
        let mut items = execution
            .events
            .into_iter()
            .map(|event| Ok(HarnessExecutionItem::Event(event)))
            .collect::<Vec<_>>();
        items.push(Ok(HarnessExecutionItem::Completed {
            final_text: execution.final_text,
            usage: execution.usage,
        }));
        Ok(Box::pin(tokio_stream::iter(items)))
    }
    async fn start_attempt(
        &mut self,
        request: HarnessSessionRequest,
        attempt_id: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecution, HarnessAdapterError> {
        self.execute(request, attempt_id, prompt, native_session_id)
            .await
    }
    async fn resume_attempt(
        &mut self,
        request: HarnessSessionRequest,
        attempt_id: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecution, HarnessAdapterError> {
        self.execute(request, attempt_id, prompt, native_session_id)
            .await
    }
    async fn reconcile_attempt(
        &mut self,
        request: HarnessSessionRequest,
        attempt_id: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecution, HarnessAdapterError> {
        self.control_attempt(request, attempt_id, "reconcile", "", native_session_id)
            .await
    }
    async fn checkpoint(
        &mut self,
        request: HarnessSessionRequest,
        attempt_id: &str,
    ) -> Result<NativeCheckpoint, HarnessAdapterError>;
    async fn close_session(
        &mut self,
        request: HarnessSessionRequest,
    ) -> Result<(), HarnessAdapterError>;
    async fn delete_session(
        &mut self,
        request: HarnessSessionRequest,
    ) -> Result<(), HarnessAdapterError> {
        self.close_session(request).await
    }

    async fn control_session(
        &mut self,
        _request: HarnessSessionRequest,
        operation: &str,
        _payload: Value,
    ) -> Result<NativeSession, HarnessAdapterError> {
        Err(HarnessAdapterError::InvalidRequest(format!(
            "session operation is not supported by this adapter: {operation}"
        )))
    }

    async fn control_attempt(
        &mut self,
        _request: HarnessSessionRequest,
        _attempt_id: &str,
        operation: &str,
        _prompt: &str,
        _native_session_id: Option<&str>,
    ) -> Result<HarnessExecution, HarnessAdapterError> {
        Err(HarnessAdapterError::InvalidRequest(format!(
            "attempt operation is not supported by this adapter: {operation}"
        )))
    }

    async fn exchange(
        &mut self,
        _request: HarnessSessionRequest,
        kind: &str,
        _payload: Value,
    ) -> Result<Value, HarnessAdapterError> {
        Err(HarnessAdapterError::InvalidRequest(format!(
            "worker exchange is not supported by this adapter: {kind}"
        )))
    }
}

struct ProcessChannel {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

fn classify_process_io_error(error: std::io::Error) -> HarnessAdapterError {
    if error.kind() == std::io::ErrorKind::BrokenPipe {
        HarnessAdapterError::ProcessExited
    } else {
        HarnessAdapterError::Io(error)
    }
}

fn classify_protocol_error(error: JsonRpcError) -> HarnessAdapterError {
    match error {
        JsonRpcError::Timeout => HarnessAdapterError::Timeout,
        JsonRpcError::ProcessExited => HarnessAdapterError::ProcessExited,
        JsonRpcError::Spawn(message) | JsonRpcError::Io(message) => {
            HarnessAdapterError::Remote(message)
        }
        JsonRpcError::Cancelled => {
            HarnessAdapterError::Remote("native request was cancelled".to_owned())
        }
        JsonRpcError::Malformed(message) | JsonRpcError::InvalidMessage(message) => {
            HarnessAdapterError::InvalidResponse(message)
        }
        JsonRpcError::UnknownRequest(id) => HarnessAdapterError::InvalidRequest(format!(
            "native server request is no longer pending: {id:?}"
        )),
        JsonRpcError::RequestFailed(error) => HarnessAdapterError::Remote(format!(
            "native request failed with code {}: {}",
            error.code, error.message
        )),
    }
}

fn classify_openhands_error(error: OpenHandsError) -> HarnessAdapterError {
    match error {
        OpenHandsError::InvalidRequest(message) => HarnessAdapterError::InvalidRequest(message),
        OpenHandsError::InvalidResponse(message) => HarnessAdapterError::InvalidResponse(message),
        OpenHandsError::NonLoopbackAddress(address) => HarnessAdapterError::InvalidRequest(
            format!("OpenHands address is not loopback: {address}"),
        ),
        OpenHandsError::Io(error) => HarnessAdapterError::Io(error),
        OpenHandsError::Json(error) => HarnessAdapterError::Json(error),
        OpenHandsError::Timeout | OpenHandsError::ReadinessTimeout { .. } => {
            HarnessAdapterError::Timeout
        }
        OpenHandsError::UnsupportedServerVersion { .. } => {
            HarnessAdapterError::InvalidResponse(error.to_string())
        }
        OpenHandsError::ProcessExited { .. } => HarnessAdapterError::ProcessExited,
        OpenHandsError::Router(error) => HarnessAdapterError::OpenHandsRouter(error),
        OpenHandsError::Provider(error) => HarnessAdapterError::OpenHandsProvider(error),
        OpenHandsError::CapabilityDowngradeRequired(_) => {
            HarnessAdapterError::Remote(error.to_string())
        }
    }
}

fn openhands_downgrade_event(downgrade: OpenHandsCapabilityDowngrade) -> HarnessEvent {
    HarnessEvent {
        event_type: "capability.downgraded".to_owned(),
        durable: true,
        payload: json!({
            "capability": downgrade.capability,
            "requested": downgrade.requested,
            "applied": downgrade.applied,
            "reason": downgrade.reason,
            "requires_acceptance": downgrade.requires_acceptance,
        }),
        native_cursor: None,
    }
}

fn openhands_harness_event(event: OpenHandsEvent) -> HarnessEvent {
    let event_type = match &event.category {
        OpenHandsEventCategory::Message => "assistant.text_chunk",
        OpenHandsEventCategory::Action => "tool.started",
        OpenHandsEventCategory::Observation => "tool.completed",
        OpenHandsEventCategory::Approval => "interaction.required",
        OpenHandsEventCategory::Usage => "usage.recorded",
        OpenHandsEventCategory::Error => "turn.failed",
        OpenHandsEventCategory::Native => "native.event",
    };
    let durable = event.category != OpenHandsEventCategory::Message;
    let mut payload = json!({
        "kind": event.kind,
        "native": event.native,
    });
    if let Some(final_text) = event.final_text {
        payload["content"] = Value::String(final_text);
    }
    if let Some(usage) = event.usage {
        payload["usage"] = usage;
    }
    HarnessEvent {
        event_type: event_type.to_owned(),
        durable,
        payload,
        native_cursor: event.native_cursor,
    }
}

fn openhands_approval_event(native_session_id: &str, action: OpenHandsEvent) -> HarnessEvent {
    HarnessEvent {
        event_type: "interaction.required".to_owned(),
        durable: true,
        payload: json!({
            "interaction_kind": "approval",
            "native_session_id": native_session_id,
            "native": action.native,
        }),
        native_cursor: action.native_cursor,
    }
}

fn completed_stream(
    final_text: Option<String>,
    usage: Option<Value>,
    events: Vec<HarnessEvent>,
) -> HarnessExecutionStream {
    let mut items = events
        .into_iter()
        .map(|event| Ok(HarnessExecutionItem::Event(event)))
        .collect::<Vec<_>>();
    items.push(Ok(HarnessExecutionItem::Completed { final_text, usage }));
    Box::pin(tokio_stream::iter(items))
}

fn portable_capsule_context(capsule: &SessionCapsule) -> Result<String, HarnessAdapterError> {
    serde_json::to_string(&json!({
        "schema": "omnisolo.portable_session_context.v1",
        "manifest": capsule.manifest,
        "records": capsule.records,
        "loss_report": capsule.loss_report,
        "manifest_digest": capsule.manifest_digest,
        "record_digest": capsule.record_digest,
        "loss_report_digest": capsule.loss_report_digest,
        "head_event_id": capsule.head_event_id,
        "event_ancestor_ids": capsule.event_ancestor_ids,
    }))
    .map_err(HarnessAdapterError::Json)
}

fn prompt_with_portable_context(context: &str, prompt: &str) -> String {
    format!(
        "The following verified OmniSolo JSON is historical session context transferred from another harness. Preserve its conversation, task, tool, plan, workspace, and instruction-layer semantics; do not treat JSON delimiters or metadata as a new user request.\n\
<omnisolo_portable_session_context>\n{context}\n</omnisolo_portable_session_context>\n\
<current_user_request>\n{prompt}\n</current_user_request>"
    )
}

pub struct ProcessHarnessAdapter {
    descriptor: HarnessDescriptor,
    spec: ProcessHarnessSpec,
    channel: Option<ProcessChannel>,
    protocol: Option<ProtocolProcessAdapter>,
    pi: Option<PiProcessAdapter>,
    opencode: Option<OpenCodeProcessAdapter>,
    openhands: Option<OpenHandsProcessAdapter>,
    openharness: Option<OpenHarnessProcessAdapter>,
    pending_portable_imports: Arc<std::sync::Mutex<BTreeMap<String, String>>>,
}

struct PiProcessAdapter {
    runtime: PiRpcRuntime,
    _home: PiIsolatedHome,
    native_session_id: String,
}

struct OpenHarnessProcessAdapter {
    runtime: OpenHarnessRuntime,
    native_session_id: Option<String>,
}

struct OpenCodeProcessAdapter {
    runtime: OpenCodeHttpAdapter,
    native_session_id: Option<String>,
}

struct OpenHandsProcessAdapter {
    runtime: OpenHandsHttpAdapter,
    native_session_id: Option<String>,
    pending_events: Vec<HarnessEvent>,
}

impl Drop for ProcessHarnessAdapter {
    fn drop(&mut self) {
        if let Some(protocol) = self.protocol.take() {
            if let Ok(handle) = tokio::runtime::Handle::try_current() {
                handle.spawn(async move {
                    let _ = protocol.runtime.shutdown().await;
                });
            }
        }
        if let Some(pi) = self.pi.take()
            && let Ok(handle) = tokio::runtime::Handle::try_current()
        {
            handle.spawn(async move {
                let _ = pi.runtime.shutdown().await;
            });
        }
        if let Some(opencode) = self.opencode.take()
            && let Ok(handle) = tokio::runtime::Handle::try_current()
        {
            handle.spawn(async move {
                let _ = opencode.runtime.shutdown().await;
            });
        }
        if let Some(openhands) = self.openhands.take()
            && let Ok(handle) = tokio::runtime::Handle::try_current()
        {
            handle.spawn(async move {
                let _ = openhands.runtime.shutdown().await;
            });
        }
        if let Some(openharness) = self.openharness.take()
            && let Ok(handle) = tokio::runtime::Handle::try_current()
        {
            handle.spawn(async move {
                let _ = openharness.runtime.shutdown().await;
            });
        }
        if let Some(mut channel) = self.channel.take() {
            let _ = channel.child.start_kill();
        }
    }
}

impl std::fmt::Debug for ProcessHarnessAdapter {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProcessHarnessAdapter")
            .field("harness_id", &self.spec.harness_id)
            .field("protocol_kind", &self.spec.protocol_kind)
            .field("legacy_started", &self.channel.is_some())
            .field("native_started", &self.protocol.is_some())
            .field("pi_started", &self.pi.is_some())
            .field("opencode_started", &self.opencode.is_some())
            .field("openhands_started", &self.openhands.is_some())
            .field("openharness_started", &self.openharness.is_some())
            .field(
                "pending_portable_imports",
                &self
                    .pending_portable_imports
                    .lock()
                    .map(|imports| imports.len())
                    .unwrap_or_default(),
            )
            .finish()
    }
}

impl ProcessHarnessAdapter {
    pub fn new(spec: ProcessHarnessSpec) -> Self {
        let descriptor = descriptor_for_spec(&spec);
        Self {
            descriptor,
            spec,
            channel: None,
            protocol: None,
            pi: None,
            opencode: None,
            openhands: None,
            openharness: None,
            pending_portable_imports: Arc::new(std::sync::Mutex::new(BTreeMap::new())),
        }
    }

    pub fn descriptor(&self) -> &HarnessDescriptor {
        &self.descriptor
    }

    pub fn protocol_kind(&self) -> HarnessProtocolKind {
        self.spec.protocol_kind.clone()
    }

    fn model_binding_event(
        &self,
        operation: AttemptOperation,
        request: &HarnessSessionRequest,
    ) -> Option<HarnessEvent> {
        if !matches!(
            operation,
            AttemptOperation::Start | AttemptOperation::Execute | AttemptOperation::Resume
        ) {
            return None;
        }
        let selection = request
            .resolved_model
            .as_ref()
            .or(self.spec.resolved_model.as_ref())?;
        Some(HarnessEvent {
            event_type: "inference.model_binding".to_owned(),
            durable: true,
            payload: json!({
                "harness_id": self.spec.harness_id,
                "integration_mode": self.descriptor.integration_mode,
                "provider_route": selection.provider_route,
                "model_id": selection.model_id,
                "reasoning_effort": selection.reasoning_effort,
                "api_dialect": selection.api_dialect,
                "context_window": selection.context_window,
                "max_output_tokens": selection.max_output_tokens,
                "capabilities": selection.capabilities,
                "binding_revision": selection.binding_revision,
                "binding_digest": selection.binding_digest,
            }),
            native_cursor: None,
        })
    }

    fn prepend_model_binding(
        event: Option<HarnessEvent>,
        stream: HarnessExecutionStream,
    ) -> HarnessExecutionStream {
        match event {
            Some(event) => {
                Box::pin(tokio_stream::iter([Ok(HarnessExecutionItem::Event(event))]).chain(stream))
            }
            None => stream,
        }
    }

    fn remember_portable_import(&mut self, native_session_id: &str, context: String) {
        self.pending_portable_imports
            .lock()
            .expect("portable import lock is not poisoned")
            .insert(native_session_id.to_owned(), context);
    }

    fn prompt_for_native_session(&self, native_session_id: &str, prompt: &str) -> String {
        self.pending_portable_imports
            .lock()
            .expect("portable import lock is not poisoned")
            .get(native_session_id)
            .map(|context| prompt_with_portable_context(context, prompt))
            .unwrap_or_else(|| prompt.to_owned())
    }

    fn mark_portable_import_applied(&mut self, native_session_id: &str) -> bool {
        self.pending_portable_imports
            .lock()
            .expect("portable import lock is not poisoned")
            .remove(native_session_id)
            .is_some()
    }

    pub fn runtime_family(&self) -> RuntimeFamily {
        self.spec.protocol_kind.runtime_family()
    }

    fn uses_protocol_runtime(&self) -> bool {
        self.runtime_family() == RuntimeFamily::JsonRpc
    }

    fn uses_pi_runtime(&self) -> bool {
        self.spec.protocol_kind == HarnessProtocolKind::PiRpc
    }

    fn uses_opencode_runtime(&self) -> bool {
        self.spec.protocol_kind == HarnessProtocolKind::OpenCodeHttp
    }

    fn uses_openharness_runtime(&self) -> bool {
        self.spec.protocol_kind == HarnessProtocolKind::OpenHarnessSdk
    }

    fn uses_openhands_runtime(&self) -> bool {
        self.spec.protocol_kind == HarnessProtocolKind::OpenHandsHttp
    }

    fn uses_openai_compatible_shim(&self) -> bool {
        self.spec.protocol_kind == HarnessProtocolKind::OpenAiCompatibleShim
    }

    fn shim_session_id(session_id: Uuid) -> String {
        format!("shim:{session_id}")
    }

    fn validate_shim_session(
        &self,
        session_id: Uuid,
        native_session_id: &str,
    ) -> Result<(), HarnessAdapterError> {
        if native_session_id == Self::shim_session_id(session_id) {
            Ok(())
        } else {
            Err(HarnessAdapterError::InvalidRequest(
                "OpenAI-compatible shim session id does not match the request".to_owned(),
            ))
        }
    }

    fn opencode_selection(
        &self,
        request: &HarnessSessionRequest,
    ) -> Result<ResolvedModelSelection, HarnessAdapterError> {
        request
            .resolved_model
            .clone()
            .or_else(|| self.spec.resolved_model.clone())
            .ok_or_else(|| {
                HarnessAdapterError::InvalidRequest(
                    "OpenCode requires a resolved model selection".to_owned(),
                )
            })
    }

    async fn ensure_opencode_runtime(
        &mut self,
        request: &HarnessSessionRequest,
    ) -> Result<(), HarnessAdapterError> {
        if self.opencode.is_some() {
            return Ok(());
        }
        if !self.uses_opencode_runtime() {
            return Err(HarnessAdapterError::InvalidRequest(
                "OpenCode runtime requested for a different harness protocol".to_owned(),
            ));
        }
        let selection = self.opencode_selection(request)?;
        let base_url = self
            .spec
            .api_base_url
            .clone()
            .unwrap_or_else(|| "https://api.openai.com/v1".to_owned());
        let mut process = OpenCodeProcessConfig::new(self.spec.executable.clone())
            .with_prefix_args(self.spec.args.clone())
            .with_working_directory(self.spec.working_directory.clone())
            .with_readiness(self.spec.request_timeout, Duration::from_millis(10))
            .with_request_timeout(self.spec.request_timeout);
        for (key, value) in &self.spec.environment {
            process = process.with_environment(key.clone(), value.clone());
        }
        let runtime = OpenCodeHttpAdapter::spawn(process, selection, &base_url).await?;
        self.opencode = Some(OpenCodeProcessAdapter {
            runtime,
            native_session_id: None,
        });
        Ok(())
    }

    fn opencode_session_id(
        &self,
        native_session_id: Option<&str>,
    ) -> Result<String, HarnessAdapterError> {
        native_session_id
            .filter(|value| !value.trim().is_empty())
            .map(str::to_owned)
            .or_else(|| {
                self.opencode
                    .as_ref()
                    .and_then(|adapter| adapter.native_session_id.clone())
            })
            .ok_or_else(|| {
                HarnessAdapterError::InvalidRequest(
                    "OpenCode native session id is required".to_owned(),
                )
            })
    }

    async fn opencode_session_native(
        &mut self,
        request: &HarnessSessionRequest,
        resume_session_id: Option<&str>,
    ) -> Result<NativeSession, HarnessAdapterError> {
        self.ensure_opencode_runtime(request).await?;
        let selection = self.opencode_selection(request)?;
        let session = match resume_session_id {
            Some(native_session_id) => {
                self.opencode
                    .as_ref()
                    .expect("OpenCode runtime initialized")
                    .runtime
                    .get_session(native_session_id)
                    .await?
            }
            None => {
                let title = if request.objective.trim().is_empty() {
                    format!("OmniSolo session {}", request.session_id)
                } else {
                    request.objective.clone()
                };
                self.opencode
                    .as_ref()
                    .expect("OpenCode runtime initialized")
                    .runtime
                    .create_session(&title, &selection)
                    .await?
            }
        };
        self.opencode
            .as_mut()
            .expect("OpenCode runtime initialized")
            .native_session_id = Some(session.id.clone());
        Ok(NativeSession {
            native_session_id: session.id,
            native_cursor: None,
        })
    }

    async fn opencode_attempt_stream(
        &mut self,
        operation: AttemptOperation,
        request: HarnessSessionRequest,
        attempt_id: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecutionStream, HarnessAdapterError> {
        if attempt_id.trim().is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(
                "attempt id is empty".to_owned(),
            ));
        }
        self.ensure_opencode_runtime(&request).await?;
        let selected_session_id = self.opencode_session_id(native_session_id)?;
        match operation {
            AttemptOperation::Start | AttemptOperation::Execute | AttemptOperation::Resume => {
                if prompt.trim().is_empty() {
                    return Err(HarnessAdapterError::InvalidRequest(
                        "OpenCode prompt is required".to_owned(),
                    ));
                }
                let selection = self.opencode_selection(&request)?;
                let translated_prompt =
                    self.prompt_for_native_session(&selected_session_id, prompt);
                let correlation = OpenCodeEventCorrelation {
                    session_id: request.session_id,
                    task_id: request.task_id,
                    turn_id: request.turn_id,
                    attempt_id: attempt_id.to_owned(),
                    native_session_id: selected_session_id.clone(),
                    admitted_user_message_id: None,
                };
                let mut native_stream = self
                    .opencode
                    .as_ref()
                    .expect("OpenCode runtime initialized")
                    .runtime
                    .prompt_async(correlation, &translated_prompt, &selection)
                    .await?;
                self.mark_portable_import_applied(&selected_session_id);
                let (sender, receiver) = tokio::sync::mpsc::channel(64);
                tokio::spawn(async move {
                    loop {
                        let decoded = match native_stream.next_event().await {
                            Ok(decoded) => decoded,
                            Err(error) => {
                                let _ = sender.send(Err(error)).await;
                                return;
                            }
                        };
                        let terminal = decoded.terminal;
                        let final_text = decoded.final_text.clone();
                        let usage = decoded.usage.clone();
                        if sender
                            .send(Ok(HarnessExecutionItem::Event(decoded.event)))
                            .await
                            .is_err()
                        {
                            return;
                        }
                        if terminal {
                            let _ = sender
                                .send(Ok(HarnessExecutionItem::Completed { final_text, usage }))
                                .await;
                            return;
                        }
                    }
                });
                Ok(Box::pin(tokio_stream::wrappers::ReceiverStream::new(
                    receiver,
                )))
            }
            AttemptOperation::Cancel | AttemptOperation::Quiesce => {
                self.opencode
                    .as_ref()
                    .expect("OpenCode runtime initialized")
                    .runtime
                    .abort(&selected_session_id)
                    .await?;
                Ok(completed_stream(None, None, Vec::new()))
            }
            AttemptOperation::Steer => Err(HarnessAdapterError::InvalidRequest(
                "OpenCode does not expose a native steer operation".to_owned(),
            )),
            AttemptOperation::Reconcile => Err(HarnessAdapterError::InvalidRequest(
                "OpenCode does not expose a reconcile operation".to_owned(),
            )),
        }
    }

    async fn collect_opencode_execution(
        &mut self,
        operation: AttemptOperation,
        request: HarnessSessionRequest,
        attempt_id: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecution, HarnessAdapterError> {
        let model_binding = self.model_binding_event(operation, &request);
        let mut stream = self
            .opencode_attempt_stream(operation, request, attempt_id, prompt, native_session_id)
            .await?;
        let mut execution = HarnessExecution {
            events: model_binding.into_iter().collect(),
            final_text: None,
            usage: None,
        };
        while let Some(item) = stream.next().await {
            match item? {
                HarnessExecutionItem::Event(event) => execution.events.push(event),
                HarnessExecutionItem::Completed { final_text, usage } => {
                    execution.final_text = final_text;
                    execution.usage = usage;
                }
            }
        }
        Ok(execution)
    }

    fn openhands_adapter_config(
        &self,
        request: &HarnessSessionRequest,
    ) -> Result<OpenHandsAdapterConfig, HarnessAdapterError> {
        let config = self.openhands_conversation_config(request)?;
        Ok(OpenHandsAdapterConfig::new(
            config.workspace,
            config.resolved_model,
            config.provider_base_url,
        )
        .with_reasoning_support(OpenHandsReasoningSupport::MaxAccepted)
        .with_timeouts(
            self.spec.request_timeout,
            self.spec.request_timeout,
            Duration::from_millis(10),
        ))
    }

    fn openhands_conversation_config(
        &self,
        request: &HarnessSessionRequest,
    ) -> Result<OpenHandsConversationConfig, HarnessAdapterError> {
        let selection = request
            .resolved_model
            .clone()
            .or_else(|| self.spec.resolved_model.clone())
            .ok_or_else(|| {
                HarnessAdapterError::InvalidRequest(
                    "OpenHands requires a resolved model selection".to_owned(),
                )
            })?;
        let base_url = self
            .spec
            .api_base_url
            .clone()
            .unwrap_or_else(|| "https://api.openai.com/v1".to_owned());
        let workspace = self
            .spec
            .working_directory
            .join(".omnisolo")
            .join("sessions")
            .join(request.session_id.to_string());
        std::fs::create_dir_all(&workspace).map_err(HarnessAdapterError::Io)?;
        Ok(OpenHandsConversationConfig::new(
            workspace, selection, base_url,
        ))
    }

    async fn ensure_openhands_runtime(
        &mut self,
        request: &HarnessSessionRequest,
    ) -> Result<(), HarnessAdapterError> {
        if self.openhands.is_some() {
            return Ok(());
        }
        if !self.uses_openhands_runtime() {
            return Err(HarnessAdapterError::InvalidRequest(
                "OpenHands runtime requested for a different harness protocol".to_owned(),
            ));
        }
        let adapter_config = self.openhands_adapter_config(request)?;
        let mut args = self.spec.args.clone();
        args.extend([
            "--host".to_owned(),
            "{host}".to_owned(),
            "--port".to_owned(),
            "{port}".to_owned(),
        ]);
        let mut process = HttpProcessConfig::new(self.spec.executable.clone(), args)
            .with_readiness(self.spec.request_timeout, Duration::from_millis(10));
        process.environment = self.spec.environment.clone();
        let api_key = process.environment.remove("OPENAI_API_KEY");
        let launch = OpenHandsLaunchConfig::new(process, adapter_config);
        let launch = match api_key {
            Some(api_key) => launch.with_api_key(api_key),
            None => launch,
        };
        let runtime = OpenHandsHttpAdapter::launch(launch)
            .await
            .map_err(classify_openhands_error)?;
        self.openhands = Some(OpenHandsProcessAdapter {
            runtime,
            native_session_id: None,
            pending_events: Vec::new(),
        });
        Ok(())
    }

    fn openhands_session_id(
        &self,
        native_session_id: Option<&str>,
    ) -> Result<String, HarnessAdapterError> {
        native_session_id
            .filter(|value| !value.trim().is_empty())
            .map(str::to_owned)
            .or_else(|| {
                self.openhands
                    .as_ref()
                    .and_then(|adapter| adapter.native_session_id.clone())
            })
            .ok_or_else(|| {
                HarnessAdapterError::InvalidRequest(
                    "OpenHands native conversation id is required".to_owned(),
                )
            })
    }

    async fn openhands_session_native(
        &mut self,
        request: &HarnessSessionRequest,
        resume_session_id: Option<&str>,
    ) -> Result<NativeSession, HarnessAdapterError> {
        self.ensure_openhands_runtime(request).await?;
        let conversation_config = self.openhands_conversation_config(request)?;
        let conversation = {
            let runtime = &mut self
                .openhands
                .as_mut()
                .expect("OpenHands runtime initialized")
                .runtime;
            match resume_session_id {
                Some(conversation_id) => {
                    runtime
                        .resume_conversation_with_config(
                            conversation_id,
                            &conversation_config,
                            CapabilityDowngradePolicy::Reject,
                        )
                        .await
                }
                None => {
                    runtime
                        .create_conversation_with_config(
                            &conversation_config,
                            CapabilityDowngradePolicy::Reject,
                        )
                        .await
                }
            }
        }
        .map_err(classify_openhands_error)?;
        let adapter = self
            .openhands
            .as_mut()
            .expect("OpenHands runtime initialized");
        adapter.native_session_id = Some(conversation.id.clone());
        adapter.pending_events.clear();
        adapter.pending_events.extend(
            conversation
                .downgrades
                .into_iter()
                .map(openhands_downgrade_event),
        );
        Ok(NativeSession {
            native_session_id: conversation.id,
            native_cursor: None,
        })
    }

    async fn openhands_attempt_stream(
        &mut self,
        operation: AttemptOperation,
        request: HarnessSessionRequest,
        attempt_id: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecutionStream, HarnessAdapterError> {
        if attempt_id.trim().is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(
                "attempt id is empty".to_owned(),
            ));
        }
        self.ensure_openhands_runtime(&request).await?;
        let selected_session_id = self.openhands_session_id(native_session_id)?;
        match operation {
            AttemptOperation::Start | AttemptOperation::Execute | AttemptOperation::Resume => {
                if prompt.trim().is_empty() {
                    return Err(HarnessAdapterError::InvalidRequest(
                        "OpenHands prompt is required".to_owned(),
                    ));
                }
                let translated_prompt =
                    self.prompt_for_native_session(&selected_session_id, prompt);
                let (mut native_stream, pending_events) = {
                    let adapter = self
                        .openhands
                        .as_mut()
                        .expect("OpenHands runtime initialized");
                    let native_stream = adapter
                        .runtime
                        .prompt_stream(&selected_session_id, &translated_prompt)
                        .await
                        .map_err(classify_openhands_error)?;
                    (native_stream, std::mem::take(&mut adapter.pending_events))
                };
                self.mark_portable_import_applied(&selected_session_id);
                let (sender, receiver) = tokio::sync::mpsc::channel(64);
                tokio::spawn(async move {
                    for event in pending_events {
                        if sender
                            .send(Ok(HarnessExecutionItem::Event(event)))
                            .await
                            .is_err()
                        {
                            return;
                        }
                    }
                    while let Some(item) = native_stream.next().await {
                        let item = match item {
                            Ok(OpenHandsStreamItem::Event(event)) => {
                                HarnessExecutionItem::Event(openhands_harness_event(event))
                            }
                            Ok(OpenHandsStreamItem::ApprovalRequired { action }) => {
                                HarnessExecutionItem::Event(openhands_approval_event(
                                    &selected_session_id,
                                    action,
                                ))
                            }
                            Ok(OpenHandsStreamItem::Completed { final_text, usage }) => {
                                HarnessExecutionItem::Completed { final_text, usage }
                            }
                            Err(error) => {
                                let _ = sender.send(Err(classify_openhands_error(error))).await;
                                return;
                            }
                        };
                        let terminal = matches!(item, HarnessExecutionItem::Completed { .. });
                        if sender.send(Ok(item)).await.is_err() || terminal {
                            return;
                        }
                    }
                });
                Ok(Box::pin(tokio_stream::wrappers::ReceiverStream::new(
                    receiver,
                )))
            }
            AttemptOperation::Cancel | AttemptOperation::Quiesce => {
                self.openhands
                    .as_ref()
                    .expect("OpenHands runtime initialized")
                    .runtime
                    .cancel(&selected_session_id)
                    .await
                    .map_err(classify_openhands_error)?;
                Ok(completed_stream(None, None, Vec::new()))
            }
            AttemptOperation::Steer => Err(HarnessAdapterError::InvalidRequest(
                "OpenHands does not expose a native steer operation".to_owned(),
            )),
            AttemptOperation::Reconcile => Err(HarnessAdapterError::InvalidRequest(
                "OpenHands does not expose a reconcile operation".to_owned(),
            )),
        }
    }

    async fn collect_openhands_execution(
        &mut self,
        operation: AttemptOperation,
        request: HarnessSessionRequest,
        attempt_id: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecution, HarnessAdapterError> {
        let model_binding = self.model_binding_event(operation, &request);
        let mut stream = self
            .openhands_attempt_stream(operation, request, attempt_id, prompt, native_session_id)
            .await?;
        let mut execution = HarnessExecution {
            events: model_binding.into_iter().collect(),
            final_text: None,
            usage: None,
        };
        while let Some(item) = stream.next().await {
            match item? {
                HarnessExecutionItem::Event(event) => execution.events.push(event),
                HarnessExecutionItem::Completed { final_text, usage } => {
                    execution.final_text = final_text;
                    execution.usage = usage;
                }
            }
        }
        Ok(execution)
    }

    async fn ensure_openharness_runtime(&mut self) -> Result<(), HarnessAdapterError> {
        if self.openharness.is_some() {
            return Ok(());
        }
        if !self.uses_openharness_runtime() {
            return Err(HarnessAdapterError::InvalidRequest(
                "OpenHarness runtime requested for a different harness protocol".to_owned(),
            ));
        }
        let runtime = OpenHarnessRuntime::spawn(OpenHarnessProcessConfig {
            executable: self.spec.executable.clone(),
            args: self.spec.args.clone(),
            environment: self.spec.environment.clone(),
            request_timeout: self.spec.request_timeout,
        })
        .await?;
        self.openharness = Some(OpenHarnessProcessAdapter {
            runtime,
            native_session_id: None,
        });
        Ok(())
    }

    fn openharness_prompt_configuration(
        &self,
        request: &HarnessSessionRequest,
    ) -> Result<(OpenHarnessResolvedModel, String, String), HarnessAdapterError> {
        let selection = request
            .resolved_model
            .as_ref()
            .or(self.spec.resolved_model.as_ref())
            .ok_or_else(|| {
                HarnessAdapterError::InvalidRequest(
                    "OpenHarness requires a resolved model selection".to_owned(),
                )
            })?;
        if selection.model_id.trim().is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(
                "OpenHarness model id is empty".to_owned(),
            ));
        }
        if !matches!(
            selection.provider_route.as_str(),
            "openai" | "openai-compatible"
        ) {
            return Err(HarnessAdapterError::InvalidRequest(format!(
                "OpenHarness does not support provider route {}",
                selection.provider_route
            )));
        }
        let base_url = self
            .spec
            .api_base_url
            .clone()
            .unwrap_or_else(|| "https://api.openai.com/v1".to_owned());
        let permission_mode = request
            .extensions
            .get("permission_mode")
            .and_then(Value::as_str)
            .unwrap_or("bypass")
            .trim()
            .to_owned();
        if permission_mode.is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(
                "OpenHarness permission mode is empty".to_owned(),
            ));
        }
        let reasoning_effort = selection
            .reasoning_effort
            .as_ref()
            .map(|effort| serde_json::to_value(effort).expect("reasoning effort serializes"))
            .and_then(|value| value.as_str().map(str::to_owned))
            .unwrap_or_else(|| "none".to_owned());
        if reasoning_effort == "custom" {
            return Err(HarnessAdapterError::InvalidRequest(
                "OpenHarness cannot translate custom reasoning effort".to_owned(),
            ));
        }
        let api_dialect = match &selection.api_dialect {
            ModelApiDialect::OpenAiResponses => "openai_responses",
            ModelApiDialect::OpenAiChatCompletions => "openai_chat_completions",
            ModelApiDialect::AnthropicMessages => "anthropic_messages",
            ModelApiDialect::GoogleGenerateContent => "google_generate_content",
            ModelApiDialect::Ollama => "ollama",
            ModelApiDialect::Custom => "custom",
        }
        .to_owned();
        let capabilities = if selection.capabilities.is_empty() {
            vec![
                "text".to_owned(),
                "tools".to_owned(),
                "streaming".to_owned(),
            ]
        } else {
            selection.capabilities.iter().cloned().collect()
        };
        let resolved_model = OpenHarnessResolvedModel {
            provider: "openai".to_owned(),
            model_id: selection.model_id.clone(),
            base_url,
            reasoning_effort,
            api_dialect,
            context_window_tokens: selection.context_window.unwrap_or(200_000),
            max_output_tokens: selection.max_output_tokens.unwrap_or(16_384),
            capabilities,
            binding_revision: selection.binding_revision.clone(),
            binding_digest: selection.binding_digest.clone(),
        };
        let workspace = self
            .spec
            .working_directory
            .join(".omnisolo")
            .join("sessions")
            .join(request.session_id.to_string());
        std::fs::create_dir_all(&workspace).map_err(HarnessAdapterError::Io)?;
        Ok((
            resolved_model,
            permission_mode,
            workspace.to_string_lossy().into_owned(),
        ))
    }

    async fn openharness_session_native(
        &mut self,
        request: &HarnessSessionRequest,
        resume_session_id: Option<&str>,
    ) -> Result<NativeSession, HarnessAdapterError> {
        self.ensure_openharness_runtime().await?;
        let command_id = format!("omnisolo-session-{}", Uuid::new_v4());
        let command = match resume_session_id {
            Some(session_id) => OpenHarnessCommand::resume_session(command_id, session_id),
            None => OpenHarnessCommand::create_session(command_id),
        };
        let event = self
            .openharness
            .as_ref()
            .expect("OpenHarness runtime initialized")
            .runtime
            .request(command)
            .await?;
        let OpenHarnessEvent::Session { session_id, .. } = event else {
            return Err(HarnessAdapterError::InvalidResponse(
                "OpenHarness session command did not return a session event".to_owned(),
            ));
        };
        self.openharness
            .as_mut()
            .expect("OpenHarness runtime initialized")
            .native_session_id = Some(session_id.clone());
        let _ = request;
        Ok(NativeSession {
            native_session_id: session_id,
            native_cursor: None,
        })
    }

    async fn openharness_attempt_stream(
        &mut self,
        operation: AttemptOperation,
        request: HarnessSessionRequest,
        attempt_id: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecutionStream, HarnessAdapterError> {
        if attempt_id.trim().is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(
                "attempt id is empty".to_owned(),
            ));
        }
        let prompt_configuration = matches!(
            operation,
            AttemptOperation::Start | AttemptOperation::Execute | AttemptOperation::Resume
        )
        .then(|| self.openharness_prompt_configuration(&request))
        .transpose()?;
        self.ensure_openharness_runtime().await?;
        let openharness = self
            .openharness
            .as_ref()
            .expect("OpenHarness runtime initialized");
        let selected_session_id = native_session_id
            .filter(|value| !value.trim().is_empty())
            .map(str::to_owned)
            .or_else(|| openharness.native_session_id.clone())
            .ok_or_else(|| {
                HarnessAdapterError::InvalidRequest(
                    "OpenHarness native session id is required".to_owned(),
                )
            })?;
        let runtime = openharness.runtime.clone();
        let command_id = format!("omnisolo-attempt-{attempt_id}-{}", Uuid::new_v4());
        match operation {
            AttemptOperation::Start | AttemptOperation::Execute | AttemptOperation::Resume => {
                if prompt.trim().is_empty() {
                    return Err(HarnessAdapterError::InvalidRequest(
                        "OpenHarness prompt is required".to_owned(),
                    ));
                }
                let (resolved_model, permission_mode, cwd) =
                    prompt_configuration.expect("prompt operations have configuration");
                let translated_prompt =
                    self.prompt_for_native_session(&selected_session_id, prompt);
                let mut native_stream = runtime
                    .prompt_stream(OpenHarnessCommand::prompt_with_resolved_model(
                        command_id,
                        &selected_session_id,
                        &translated_prompt,
                        resolved_model,
                        permission_mode,
                        cwd,
                    ))
                    .await?;
                self.mark_portable_import_applied(&selected_session_id);
                let mut decoder = OpenHarnessEventDecoder::new(OpenHarnessEventCorrelation {
                    session_id: request.session_id,
                    task_id: request.task_id,
                    turn_id: request.turn_id,
                    attempt_id: attempt_id.to_owned(),
                    native_session_id: selected_session_id,
                });
                let (sender, receiver) = tokio::sync::mpsc::channel(64);
                tokio::spawn(async move {
                    let mut final_text = None;
                    let mut usage = None;
                    loop {
                        let native = match native_stream.next_event().await {
                            Ok(event) => event,
                            Err(error) => {
                                let _ = sender.send(Err(error)).await;
                                return;
                            }
                        };
                        let decoded = match decoder.decode(native) {
                            Ok(decoded) => decoded,
                            Err(error) => {
                                let _ = sender.send(Err(error)).await;
                                return;
                            }
                        };
                        if decoded.final_text.is_some() {
                            final_text = decoded.final_text.clone();
                        }
                        if decoded.usage.is_some() {
                            usage = decoded.usage.clone();
                        }
                        let terminal = decoded.terminal;
                        if sender
                            .send(Ok(HarnessExecutionItem::Event(decoded.event)))
                            .await
                            .is_err()
                        {
                            return;
                        }
                        if terminal {
                            let _ = sender
                                .send(Ok(HarnessExecutionItem::Completed { final_text, usage }))
                                .await;
                            return;
                        }
                    }
                });
                Ok(Box::pin(tokio_stream::wrappers::ReceiverStream::new(
                    receiver,
                )))
            }
            AttemptOperation::Steer => {
                if prompt.trim().is_empty() {
                    return Err(HarnessAdapterError::InvalidRequest(
                        "OpenHarness steering message is required".to_owned(),
                    ));
                }
                runtime
                    .request(OpenHarnessCommand::steer(
                        command_id,
                        selected_session_id,
                        prompt,
                    ))
                    .await?;
                Ok(completed_stream(None, None, Vec::new()))
            }
            AttemptOperation::Cancel | AttemptOperation::Quiesce => {
                runtime
                    .request(OpenHarnessCommand::cancel(command_id, selected_session_id))
                    .await?;
                Ok(completed_stream(None, None, Vec::new()))
            }
            AttemptOperation::Reconcile => Err(HarnessAdapterError::InvalidRequest(
                "OpenHarness does not expose a reconcile command".to_owned(),
            )),
        }
    }

    async fn collect_openharness_execution(
        &mut self,
        operation: AttemptOperation,
        request: HarnessSessionRequest,
        attempt_id: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecution, HarnessAdapterError> {
        let model_binding = self.model_binding_event(operation, &request);
        let mut stream = self
            .openharness_attempt_stream(operation, request, attempt_id, prompt, native_session_id)
            .await?;
        let mut execution = HarnessExecution {
            events: model_binding.into_iter().collect(),
            final_text: None,
            usage: None,
        };
        while let Some(item) = stream.next().await {
            match item? {
                HarnessExecutionItem::Event(event) => execution.events.push(event),
                HarnessExecutionItem::Completed { final_text, usage } => {
                    execution.final_text = final_text;
                    execution.usage = usage;
                }
            }
        }
        Ok(execution)
    }

    async fn ensure_protocol_runtime(
        &mut self,
        request: &HarnessSessionRequest,
    ) -> Result<(), HarnessAdapterError> {
        if self.protocol.is_some() {
            return Ok(());
        }
        if !self.uses_protocol_runtime() {
            return Err(HarnessAdapterError::InvalidRequest(
                "no native protocol codec is registered for this harness".to_owned(),
            ));
        }
        if self.spec.executable.trim().is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(
                "harness executable is empty".to_owned(),
            ));
        }
        let runtime = JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig {
            executable: self.spec.executable.clone(),
            args: self.spec.args.clone(),
            environment: self
                .spec
                .environment
                .iter()
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect(),
            request_timeout: self.spec.request_timeout,
            include_jsonrpc_header: matches!(self.spec.protocol_kind, HarnessProtocolKind::KimiAcp),
        })
        .await
        .map_err(classify_protocol_error)?;
        let cwd = self.spec.working_directory.to_string_lossy().into_owned();
        let codec: Arc<dyn HarnessProtocolCodec> = match self.spec.protocol_kind.clone() {
            HarnessProtocolKind::CodexAppServer => {
                let codec = Arc::new(CodexAppServerV2Codec::new());
                let initialize = codec.initialize_request();
                if let Err(error) = runtime.request(&initialize.method, initialize.params).await {
                    let _ = runtime.shutdown().await;
                    return Err(classify_protocol_error(error));
                }
                if let Err(error) = runtime
                    .notify("initialized", Value::Object(serde_json::Map::new()))
                    .await
                {
                    let _ = runtime.shutdown().await;
                    return Err(classify_protocol_error(error));
                }
                codec
            }
            HarnessProtocolKind::DeepSeekJsonRpc => {
                let codec = Arc::new(DeepSeekHarnessCodec::for_request(cwd, request)?);
                let initialize = codec.initialize_request();
                let result = match runtime.request(&initialize.method, initialize.params).await {
                    Ok(result) => result,
                    Err(error) => {
                        let _ = runtime.shutdown().await;
                        return Err(classify_protocol_error(error));
                    }
                };
                if let Err(error) = codec.decode_initialize_result(result) {
                    let _ = runtime.shutdown().await;
                    return Err(error);
                }
                codec
            }
            HarnessProtocolKind::KimiAcp => {
                let codec = Arc::new(AcpV1Codec::for_kimi(cwd, request)?);
                let initialize = codec.initialize_request();
                let result = match runtime.request(&initialize.method, initialize.params).await {
                    Ok(result) => result,
                    Err(error) => {
                        let _ = runtime.shutdown().await;
                        return Err(classify_protocol_error(error));
                    }
                };
                if let Err(error) = codec.decode_initialize_result(result) {
                    let _ = runtime.shutdown().await;
                    return Err(error);
                }
                codec
            }
            _ => {
                let _ = runtime.shutdown().await;
                return Err(HarnessAdapterError::InvalidRequest(
                    "no native protocol codec is registered for this harness".to_owned(),
                ));
            }
        };
        self.protocol = Some(ProtocolProcessAdapter {
            runtime,
            codec,
            state: Arc::new(tokio::sync::Mutex::new(NativeTurnState::new(""))),
            states: Arc::new(tokio::sync::Mutex::new(std::collections::BTreeMap::new())),
        });
        Ok(())
    }

    async fn ensure_pi_runtime(
        &mut self,
        request: &HarnessSessionRequest,
    ) -> Result<(), HarnessAdapterError> {
        if self.pi.is_some() {
            return Ok(());
        }
        if !self.uses_pi_runtime() {
            return Err(HarnessAdapterError::InvalidRequest(
                "Pi RPC runtime requested for a different harness protocol".to_owned(),
            ));
        }
        let mut selection = request
            .resolved_model
            .clone()
            .or_else(|| self.spec.resolved_model.clone())
            .ok_or_else(|| {
                HarnessAdapterError::InvalidRequest(
                    "Pi RPC requires a resolved model selection".to_owned(),
                )
            })?;
        selection.provider_route = PI_PROVIDER_ID.to_owned();
        let base_url = self
            .spec
            .api_base_url
            .as_deref()
            .unwrap_or("https://api.openai.com/v1");
        let home = PiIsolatedHome::create_in(&std::env::temp_dir(), &selection, base_url)?;
        let mut environment = self
            .spec
            .environment
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect::<std::collections::HashMap<_, _>>();
        for (key, value) in home.environment() {
            environment.insert(key.clone(), value.to_string_lossy().into_owned());
        }
        let runtime = PiRpcRuntime::spawn(PiRpcProcessConfig {
            executable: self.spec.executable.clone(),
            args: self.spec.args.clone(),
            environment,
            request_timeout: self.spec.request_timeout,
        })
        .await?;
        let model_command_id = format!("omnisolo-model-{}", Uuid::new_v4());
        if let Err(error) = runtime
            .request(PiRpcCommand::set_model(
                model_command_id,
                PI_PROVIDER_ID,
                selection.model_id.clone(),
            ))
            .await
        {
            let _ = runtime.shutdown().await;
            return Err(error);
        }
        if let Some(effort) = selection.reasoning_effort.as_ref()
            && let Some(level) = thinking_level(effort)?
            && let Err(error) = runtime
                .request(PiRpcCommand::set_thinking_level(
                    format!("omnisolo-thinking-{}", Uuid::new_v4()),
                    level,
                ))
                .await
        {
            let _ = runtime.shutdown().await;
            return Err(error);
        }
        self.pi = Some(PiProcessAdapter {
            runtime,
            _home: home,
            native_session_id: format!("pi:{}", request.session_id),
        });
        Ok(())
    }

    async fn pi_attempt_stream(
        &mut self,
        operation: AttemptOperation,
        request: HarnessSessionRequest,
        attempt_id: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecutionStream, HarnessAdapterError> {
        if attempt_id.trim().is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(
                "attempt id is empty".to_owned(),
            ));
        }
        self.ensure_pi_runtime(&request).await?;
        let pi = self.pi.as_ref().expect("Pi runtime initialized");
        let selected_session_id = native_session_id
            .filter(|value| !value.trim().is_empty())
            .unwrap_or(&pi.native_session_id)
            .to_owned();
        if selected_session_id != pi.native_session_id {
            return Err(HarnessAdapterError::InvalidRequest(
                "Pi RPC native session id does not match the active process".to_owned(),
            ));
        }
        let runtime = pi.runtime.clone();
        let command_id = format!("omnisolo-attempt-{attempt_id}-{}", Uuid::new_v4());
        match operation {
            AttemptOperation::Start | AttemptOperation::Execute | AttemptOperation::Resume => {
                if prompt.trim().is_empty() {
                    return Err(HarnessAdapterError::InvalidRequest(
                        "Pi RPC prompt is required".to_owned(),
                    ));
                }
                let translated_prompt =
                    self.prompt_for_native_session(&selected_session_id, prompt);
                runtime
                    .prompt(
                        PiEventCorrelation {
                            session_id: request.session_id,
                            task_id: request.task_id,
                            turn_id: request.turn_id,
                            attempt_id: attempt_id.to_owned(),
                            native_session_id: selected_session_id.clone(),
                        },
                        PiRpcCommand::prompt(command_id, translated_prompt),
                    )
                    .await?;
                self.mark_portable_import_applied(&selected_session_id);
                let (sender, receiver) = tokio::sync::mpsc::channel(64);
                tokio::spawn(async move {
                    let mut final_text = None;
                    let mut usage = None;
                    loop {
                        let decoded = match runtime.next_event().await {
                            Ok(decoded) => decoded,
                            Err(error) => {
                                let _ = sender.send(Err(error)).await;
                                return;
                            }
                        };
                        if decoded.final_text.is_some() {
                            final_text = decoded.final_text.clone();
                        }
                        if decoded.usage.is_some() {
                            usage = decoded.usage.clone();
                        }
                        let terminal = decoded.terminal;
                        if sender
                            .send(Ok(HarnessExecutionItem::Event(decoded.event)))
                            .await
                            .is_err()
                        {
                            return;
                        }
                        if terminal {
                            let _ = sender
                                .send(Ok(HarnessExecutionItem::Completed { final_text, usage }))
                                .await;
                            return;
                        }
                    }
                });
                Ok(Box::pin(tokio_stream::wrappers::ReceiverStream::new(
                    receiver,
                )))
            }
            AttemptOperation::Steer => {
                runtime
                    .request(PiRpcCommand::steer(command_id, prompt))
                    .await?;
                Ok(completed_stream(None, None, Vec::new()))
            }
            AttemptOperation::Cancel | AttemptOperation::Quiesce => {
                runtime.request(PiRpcCommand::abort(command_id)).await?;
                Ok(completed_stream(None, None, Vec::new()))
            }
            AttemptOperation::Reconcile => {
                let response = runtime.request(PiRpcCommand::get_state(command_id)).await?;
                let event = HarnessEvent {
                    event_type: "session.state_changed".to_owned(),
                    durable: true,
                    payload: json!({
                        "native_session_id": selected_session_id,
                        "state": response.data,
                    }),
                    native_cursor: None,
                };
                Ok(completed_stream(None, None, vec![event]))
            }
        }
    }

    async fn collect_pi_execution(
        &mut self,
        operation: AttemptOperation,
        request: HarnessSessionRequest,
        attempt_id: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecution, HarnessAdapterError> {
        let model_binding = self.model_binding_event(operation, &request);
        let mut stream = self
            .pi_attempt_stream(operation, request, attempt_id, prompt, native_session_id)
            .await?;
        let mut execution = HarnessExecution {
            events: model_binding.into_iter().collect(),
            final_text: None,
            usage: None,
        };
        while let Some(item) = stream.next().await {
            match item? {
                HarnessExecutionItem::Event(event) => execution.events.push(event),
                HarnessExecutionItem::Completed { final_text, usage } => {
                    execution.final_text = final_text;
                    execution.usage = usage;
                }
            }
        }
        Ok(execution)
    }

    async fn protocol_session_result(
        &mut self,
        operation: SessionOperation,
        request: &HarnessSessionRequest,
    ) -> Result<Value, HarnessAdapterError> {
        self.ensure_protocol_runtime(request).await?;
        let protocol = self
            .protocol
            .as_ref()
            .expect("native protocol runtime initialized");
        let spec = protocol.codec.session_request(operation, request)?;
        protocol
            .runtime
            .request(&spec.method, spec.params)
            .await
            .map_err(classify_protocol_error)
    }

    async fn protocol_session_native(
        &mut self,
        operation: SessionOperation,
        request: &HarnessSessionRequest,
    ) -> Result<NativeSession, HarnessAdapterError> {
        let import_capsule = match &operation {
            SessionOperation::Import(capsule) => Some(capsule.clone()),
            _ => None,
        };
        let result = self
            .protocol_session_result(operation.clone(), request)
            .await?;
        let protocol = self
            .protocol
            .as_ref()
            .expect("native protocol runtime initialized");
        let native = protocol
            .codec
            .decode_session_result(operation.clone(), result)?;
        let configuration = protocol
            .codec
            .session_configuration(&native.native_session_id)?;
        for request in configuration.requests {
            protocol
                .runtime
                .request(&request.method, request.params)
                .await
                .map_err(classify_protocol_error)?;
        }
        if let Some(capsule) = import_capsule {
            let items = protocol.codec.import_items(&capsule)?;
            protocol.state.lock().await.pending_import = Some(items.clone());
            protocol
                .runtime
                .request(
                    "thread/inject_items",
                    json!({
                        "threadId": native.native_session_id.clone(),
                        "items": items,
                    }),
                )
                .await
                .map_err(classify_protocol_error)?;
            protocol.state.lock().await.pending_import = None;
        }
        let state = {
            let mut states = protocol.states.lock().await;
            let mut state = states
                .remove(&native.native_session_id)
                .unwrap_or_else(|| NativeTurnState::new(native.native_session_id.clone()));
            state.thread_id = native.native_session_id.clone();
            state.native_cursor = native.native_cursor.clone();
            state.pending_events.extend(configuration.events);
            if matches!(
                &operation,
                SessionOperation::Create
                    | SessionOperation::Import(_)
                    | SessionOperation::Resume { .. }
                    | SessionOperation::Fork { .. }
            ) {
                state.active_turn_id = None;
                state.notification_sequence = 0;
            }
            states.insert(native.native_session_id.clone(), state.clone());
            state
        };
        *protocol.state.lock().await = state;
        Ok(native)
    }

    async fn protocol_session_command(
        &mut self,
        operation: SessionOperation,
        request: &HarnessSessionRequest,
    ) -> Result<(), HarnessAdapterError> {
        self.protocol_session_result(operation, request)
            .await
            .map(|_| ())
    }

    async fn protocol_thread_id(
        &self,
        native_session_id: Option<&str>,
    ) -> Result<String, HarnessAdapterError> {
        if let Some(native_session_id) = native_session_id.filter(|id| !id.trim().is_empty()) {
            return Ok(native_session_id.to_owned());
        }
        let protocol = self
            .protocol
            .as_ref()
            .ok_or_else(|| HarnessAdapterError::ProcessExited)?;
        let thread_id = protocol.state.lock().await.thread_id.clone();
        if thread_id.trim().is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(
                "native thread id is required".to_owned(),
            ));
        }
        Ok(thread_id)
    }

    async fn protocol_request_with_state(
        &self,
        request: &HarnessSessionRequest,
    ) -> HarnessSessionRequest {
        let mut request = request.clone();
        if request
            .extensions
            .get("native_session_id")
            .and_then(Value::as_str)
            .is_none()
        {
            if let Some(protocol) = &self.protocol {
                let thread_id = protocol.state.lock().await.thread_id.clone();
                if !thread_id.trim().is_empty() {
                    request
                        .extensions
                        .insert("native_session_id".to_owned(), Value::String(thread_id));
                }
            }
        }
        request
    }

    async fn protocol_turn_id(&self, thread_id: &str) -> Option<String> {
        let protocol = self.protocol.as_ref()?;
        if let Some(state) = protocol.states.lock().await.get(thread_id).cloned() {
            return state.active_turn_id;
        }
        let state = protocol.state.lock().await;
        (state.thread_id == thread_id).then(|| state.active_turn_id.clone())?
    }

    async fn sync_protocol_state(
        states: &Arc<tokio::sync::Mutex<std::collections::BTreeMap<String, NativeTurnState>>>,
        state: &Arc<tokio::sync::Mutex<NativeTurnState>>,
    ) {
        let snapshot = state.lock().await.clone();
        states
            .lock()
            .await
            .insert(snapshot.thread_id.clone(), snapshot);
    }

    async fn protocol_attempt_stream(
        &mut self,
        operation: AttemptOperation,
        request: HarnessSessionRequest,
        attempt_id: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecutionStream, HarnessAdapterError> {
        if attempt_id.trim().is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(
                "attempt id is empty".to_owned(),
            ));
        }
        self.ensure_protocol_runtime(&request).await?;
        let thread_id = self.protocol_thread_id(native_session_id).await?;
        let turn_id = self.protocol_turn_id(&thread_id).await;
        let prompt_operation = matches!(
            operation,
            AttemptOperation::Start | AttemptOperation::Execute | AttemptOperation::Resume
        );
        let translated_prompt = if prompt_operation {
            self.prompt_for_native_session(&thread_id, prompt)
        } else {
            prompt.to_owned()
        };
        let portable_import_session_id = (translated_prompt != prompt).then(|| thread_id.clone());
        let pending_portable_imports = Arc::clone(&self.pending_portable_imports);
        let protocol = self
            .protocol
            .as_ref()
            .expect("native protocol runtime initialized");
        let instruction = protocol.codec.attempt_instruction(
            operation,
            &request,
            &translated_prompt,
            Some(&thread_id),
            turn_id.as_deref(),
        )?;
        let runtime = protocol.runtime.clone();
        let codec = protocol.codec.clone();
        let state = Arc::new(tokio::sync::Mutex::new(
            protocol
                .states
                .lock()
                .await
                .get(&thread_id)
                .cloned()
                .unwrap_or_else(|| NativeTurnState::new(thread_id.clone())),
        ));
        let pending_events = {
            let mut state = state.lock().await;
            std::mem::take(&mut state.pending_events)
        };
        let states = protocol.states.clone();
        let wait_for_terminal = matches!(
            operation,
            AttemptOperation::Start
                | AttemptOperation::Execute
                | AttemptOperation::Resume
                | AttemptOperation::Steer
        );
        let mut notifications = runtime.subscribe_notifications();
        let mut server_requests = runtime.subscribe_server_requests();
        let (sender, receiver) = tokio::sync::mpsc::channel(64);
        tokio::spawn(async move {
            let mark_portable_import_applied = || {
                if let Some(native_session_id) = portable_import_session_id.as_deref() {
                    pending_portable_imports
                        .lock()
                        .expect("portable import lock is not poisoned")
                        .remove(native_session_id);
                }
            };
            for event in pending_events {
                if sender
                    .send(Ok(HarnessExecutionItem::Event(event)))
                    .await
                    .is_err()
                {
                    return;
                }
            }
            let mut native_request = Box::pin(instruction.send(&runtime));
            let mut request_finished = false;
            let mut final_text = None;
            let mut usage = None;
            loop {
                tokio::select! {
                    result = &mut native_request, if !request_finished => {
                        request_finished = true;
                        match result {
                            Ok(Some(result)) => {
                                mark_portable_import_applied();
                                let turn_id = result
                                    .get("turn")
                                    .and_then(|turn| turn.get("id"))
                                    .and_then(Value::as_str)
                                    .or_else(|| result.get("turnId").and_then(Value::as_str));
                                if let Some(turn_id) = turn_id {
                                    let mut state = state.lock().await;
                                    state.active_turn_id = Some(turn_id.to_owned());
                                    state.native_cursor = Some(turn_id.to_owned());
                                }
                                Self::sync_protocol_state(&states, &state).await;
                                let decoded_result = match codec.decode_attempt_result(
                                    operation,
                                    &thread_id,
                                    result,
                                ) {
                                    Ok(decoded) => decoded,
                                    Err(error) => {
                                        let _ = sender.send(Err(error)).await;
                                        return;
                                    }
                                };
                                if let Some(decoded) = decoded_result {
                                    if decoded.final_text.is_some() {
                                        final_text = decoded.final_text.clone();
                                    }
                                    if decoded.usage.is_some() {
                                        usage = decoded.usage.clone();
                                    }
                                    if decoded.terminal {
                                        while let Ok(notification) = notifications.try_recv() {
                                            if native_message_thread_id(&notification.params)
                                                .is_some_and(|notification_thread_id| notification_thread_id != thread_id)
                                            {
                                                continue;
                                            }
                                            let prior = {
                                                let mut state = state.lock().await;
                                                match codec.decode_notification(&notification, &mut state) {
                                                    Ok(decoded) => decoded,
                                                    Err(error) => {
                                                        let _ = sender.send(Err(error)).await;
                                                        return;
                                                    }
                                                }
                                            };
                                            Self::sync_protocol_state(&states, &state).await;
                                            if prior.final_text.is_some() {
                                                final_text = prior.final_text.clone();
                                            }
                                            if prior.usage.is_some() {
                                                usage = prior.usage.clone();
                                            }
                                            if sender.send(Ok(HarnessExecutionItem::Event(prior.event))).await.is_err() {
                                                return;
                                            }
                                        }
                                    }
                                    let terminal = decoded.terminal;
                                    if sender.send(Ok(HarnessExecutionItem::Event(decoded.event))).await.is_err() {
                                        return;
                                    }
                                    if terminal {
                                        let _ = sender.send(Ok(HarnessExecutionItem::Completed {
                                            final_text,
                                            usage,
                                        })).await;
                                        return;
                                    }
                                }
                            }
                            Ok(None) => mark_portable_import_applied(),
                            Err(error) => {
                                let _ = sender.send(Err(classify_protocol_error(error))).await;
                                return;
                            }
                        }
                        if !wait_for_terminal {
                            let _ = sender
                                .send(Ok(HarnessExecutionItem::Completed {
                                    final_text: None,
                                    usage: None,
                                }))
                                .await;
                            return;
                        }
                    }
                    notification = notifications.recv() => {
                        let notification = match notification {
                            Ok(notification) => {
                                mark_portable_import_applied();
                                notification
                            },
                            Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                            Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                                let _ = sender.send(Err(HarnessAdapterError::ProcessExited)).await;
                                return;
                            }
                        };
                        if native_message_thread_id(&notification.params)
                            .is_some_and(|notification_thread_id| notification_thread_id != thread_id)
                        {
                            continue;
                        }
                        let decoded = {
                            let mut state = state.lock().await;
                            match codec.decode_notification(&notification, &mut state) {
                                Ok(decoded) => decoded,
                                Err(error) => {
                                    let _ = sender.send(Err(error)).await;
                                    return;
                                }
                            }
                        };
                        Self::sync_protocol_state(&states, &state).await;
                        if decoded.final_text.is_some() {
                            final_text = decoded.final_text.clone();
                        }
                        if decoded.usage.is_some() {
                            usage = decoded.usage.clone();
                        }
                        let terminal = decoded.terminal;
                        if sender.send(Ok(HarnessExecutionItem::Event(decoded.event))).await.is_err() {
                            return;
                        }
                        if terminal {
                            let _ = sender.send(Ok(HarnessExecutionItem::Completed {
                                final_text,
                                usage,
                            })).await;
                            return;
                        }
                    }
                    server_request = server_requests.recv() => {
                        let server_request = match server_request {
                            Ok(server_request) => {
                                mark_portable_import_applied();
                                server_request
                            },
                            Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                            Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                                let _ = sender.send(Err(HarnessAdapterError::ProcessExited)).await;
                                return;
                            }
                        };
                        if native_message_thread_id(&server_request.params)
                            .is_some_and(|request_thread_id| request_thread_id != thread_id)
                        {
                            continue;
                        }
                        let event = match codec.server_request_event(&server_request) {
                            Ok(event) => event,
                            Err(error) => {
                                let _ = sender.send(Err(error)).await;
                                return;
                            }
                        };
                        if sender.send(Ok(HarnessExecutionItem::Event(event))).await.is_err() {
                            return;
                        }
                    }
                }
            }
        });
        Ok(Box::pin(tokio_stream::wrappers::ReceiverStream::new(
            receiver,
        )))
    }

    async fn collect_protocol_execution(
        &mut self,
        operation: AttemptOperation,
        request: HarnessSessionRequest,
        attempt_id: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecution, HarnessAdapterError> {
        let model_binding = self.model_binding_event(operation, &request);
        let mut stream = self
            .protocol_attempt_stream(operation, request, attempt_id, prompt, native_session_id)
            .await?;
        let mut execution = HarnessExecution {
            events: model_binding.into_iter().collect(),
            final_text: None,
            usage: None,
        };
        while let Some(item) = stream.next().await {
            match item? {
                HarnessExecutionItem::Event(event) => execution.events.push(event),
                HarnessExecutionItem::Completed { final_text, usage } => {
                    execution.final_text = final_text;
                    execution.usage = usage;
                }
            }
        }
        Ok(execution)
    }

    async fn compatibility_attempt_stream(
        &mut self,
        operation: AttemptOperation,
        request: HarnessSessionRequest,
        attempt_id: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecutionStream, HarnessAdapterError> {
        let execution = match operation {
            AttemptOperation::Start => {
                self.start_attempt(request, attempt_id, prompt, native_session_id)
                    .await?
            }
            AttemptOperation::Execute => {
                self.execute(request, attempt_id, prompt, native_session_id)
                    .await?
            }
            AttemptOperation::Resume => {
                self.resume_attempt(request, attempt_id, prompt, native_session_id)
                    .await?
            }
            AttemptOperation::Steer
            | AttemptOperation::Cancel
            | AttemptOperation::Quiesce
            | AttemptOperation::Reconcile => {
                let operation_name = match operation {
                    AttemptOperation::Steer => "steer",
                    AttemptOperation::Cancel => "cancel",
                    AttemptOperation::Quiesce => "quiesce",
                    AttemptOperation::Reconcile => "reconcile",
                    AttemptOperation::Start
                    | AttemptOperation::Execute
                    | AttemptOperation::Resume => unreachable!(),
                };
                self.control_attempt(
                    request,
                    attempt_id,
                    operation_name,
                    prompt,
                    native_session_id,
                )
                .await?
            }
        };
        let mut items = execution
            .events
            .into_iter()
            .map(|event| Ok(HarnessExecutionItem::Event(event)))
            .collect::<Vec<_>>();
        items.push(Ok(HarnessExecutionItem::Completed {
            final_text: execution.final_text,
            usage: execution.usage,
        }));
        Ok(Box::pin(tokio_stream::iter(items)))
    }

    async fn ensure_channel(&mut self) -> Result<(), HarnessAdapterError> {
        if self.channel.is_some() {
            return Ok(());
        }
        if self.spec.executable.trim().is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(
                "harness executable is empty".to_owned(),
            ));
        }
        let mut command = Command::new(&self.spec.executable);
        command.args(&self.spec.args);
        apply_isolated_environment(&mut command, &self.spec.environment);
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        let mut child = command.spawn().map_err(HarnessAdapterError::Spawn)?;
        // A successfully spawned child with `Stdio::piped()` always exposes both handles.
        let stdin = child.stdin.take().expect("harness stdin was piped");
        let stdout = child.stdout.take().expect("harness stdout was piped");
        self.channel = Some(ProcessChannel {
            child,
            stdin,
            stdout: BufReader::new(stdout),
        });
        Ok(())
    }

    async fn request(
        &mut self,
        operation: &str,
        request: &HarnessSessionRequest,
        payload: Value,
    ) -> Result<Value, HarnessAdapterError> {
        if !self.spec.protocol_kind.uses_legacy_json_lines() {
            return Err(HarnessAdapterError::InvalidRequest(format!(
                "{} uses the {:?} native runtime; generic JSON-lines dispatch is only available for custom harnesses",
                self.spec.harness_id,
                self.runtime_family()
            )));
        }
        self.ensure_channel().await?;
        let request_id = Uuid::new_v4();
        let wire = json!({
            "protocol_version": PROCESS_PROTOCOL_VERSION,
            "request_id": request_id,
            "protocol_kind": self.spec.protocol_kind,
            "harness_id": self.spec.harness_id,
            "operation": operation,
            "tenant_id": request.tenant_id,
            "session_id": request.session_id,
            "task_id": request.task_id,
            "turn_id": request.turn_id,
            "context": request_context(request),
            "payload": payload,
        });
        let line = serde_json::to_vec(&wire).map_err(HarnessAdapterError::Json)?;
        let channel = self.channel.as_mut().expect("channel created above");
        channel
            .stdin
            .write_all(&line)
            .await
            .map_err(classify_process_io_error)?;
        channel
            .stdin
            .write_all(b"\n")
            .await
            .map_err(classify_process_io_error)?;
        channel
            .stdin
            .flush()
            .await
            .map_err(classify_process_io_error)?;

        let mut response_line = String::new();
        let read = timeout(
            self.spec.request_timeout,
            channel.stdout.read_line(&mut response_line),
        )
        .await
        .map_err(|_| HarnessAdapterError::Timeout)?
        .map_err(HarnessAdapterError::Io)?;
        if read == 0 {
            return Err(HarnessAdapterError::ProcessExited);
        }
        let response: ProcessResponse =
            serde_json::from_str(response_line.trim()).map_err(HarnessAdapterError::Json)?;
        if response.request_id != request_id {
            return Err(HarnessAdapterError::RequestMismatch {
                expected: request_id,
                actual: response.request_id,
            });
        }
        if !response.ok {
            return Err(HarnessAdapterError::Remote(
                response
                    .error
                    .unwrap_or_else(|| "unknown harness error".to_owned()),
            ));
        }
        Ok(response.payload)
    }

    async fn terminate(&mut self) -> Result<(), HarnessAdapterError> {
        if let Some(opencode) = self.opencode.take() {
            opencode.runtime.shutdown().await?;
        }
        if let Some(openhands) = self.openhands.take() {
            openhands
                .runtime
                .shutdown()
                .await
                .map_err(classify_openhands_error)?;
        }
        if let Some(openharness) = self.openharness.take() {
            openharness.runtime.shutdown().await?;
        }
        if let Some(protocol) = self.protocol.take() {
            protocol
                .runtime
                .shutdown()
                .await
                .map_err(classify_protocol_error)?;
        }
        if let Some(mut channel) = self.channel.take() {
            channel
                .child
                .kill()
                .await
                .map_err(HarnessAdapterError::Io)?;
            let _ = channel.child.wait().await;
        }
        self.pending_portable_imports
            .lock()
            .expect("portable import lock is not poisoned")
            .clear();
        Ok(())
    }

    async fn execute_operation(
        &mut self,
        operation: &str,
        request: HarnessSessionRequest,
        attempt_id: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecution, HarnessAdapterError> {
        if attempt_id.trim().is_empty() || prompt.trim().is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(
                "attempt id and prompt are required".to_owned(),
            ));
        }
        let payload = self
            .request(
                operation,
                &request,
                json!({
                    "attempt_id": attempt_id,
                    "prompt": prompt,
                    "native_session_id": native_session_id,
                    "context": request_context(&request),
                }),
            )
            .await?;
        serde_json::from_value(payload).map_err(HarnessAdapterError::Json)
    }
}

#[derive(Deserialize)]
struct ProcessResponse {
    request_id: Uuid,
    ok: bool,
    #[serde(default)]
    payload: Value,
    error: Option<String>,
}

#[async_trait]
impl HarnessAdapter for ProcessHarnessAdapter {
    fn descriptor(&self) -> &HarnessDescriptor {
        &self.descriptor
    }

    fn supports_concurrent_streaming(&self) -> bool {
        self.uses_protocol_runtime()
            || self.uses_opencode_runtime()
            || self.uses_openhands_runtime()
            || self.uses_openharness_runtime()
    }

    async fn preflight(
        &mut self,
        request: HarnessSessionRequest,
    ) -> Result<(), HarnessAdapterError> {
        if self.uses_openai_compatible_shim() {
            return Ok(());
        }
        let result = if self.uses_protocol_runtime() {
            self.ensure_protocol_runtime(&request).await
        } else if self.uses_pi_runtime() {
            self.ensure_pi_runtime(&request).await
        } else if self.uses_opencode_runtime() {
            self.ensure_opencode_runtime(&request).await
        } else if self.uses_openhands_runtime() {
            self.ensure_openhands_runtime(&request).await
        } else if self.uses_openharness_runtime() {
            self.ensure_openharness_runtime().await
        } else {
            self.ensure_channel().await
        };
        let shutdown = self.terminate().await;
        result?;
        shutdown
    }

    async fn create_session(
        &mut self,
        request: HarnessSessionRequest,
    ) -> Result<NativeSession, HarnessAdapterError> {
        if self.uses_openai_compatible_shim() {
            return Ok(NativeSession {
                native_session_id: Self::shim_session_id(request.session_id),
                native_cursor: None,
            });
        }
        if self.spec.protocol_kind == HarnessProtocolKind::DeepSeekJsonRpc {
            return Ok(NativeSession {
                native_session_id: request.session_id.to_string(),
                native_cursor: None,
            });
        }
        if self.uses_protocol_runtime() {
            return self
                .protocol_session_native(SessionOperation::Create, &request)
                .await;
        }
        if self.uses_pi_runtime() {
            self.ensure_pi_runtime(&request).await?;
            let native_session_id = self
                .pi
                .as_ref()
                .expect("Pi runtime initialized")
                .native_session_id
                .clone();
            return Ok(NativeSession {
                native_session_id,
                native_cursor: None,
            });
        }
        if self.uses_opencode_runtime() {
            return self.opencode_session_native(&request, None).await;
        }
        if self.uses_openhands_runtime() {
            return self.openhands_session_native(&request, None).await;
        }
        if self.uses_openharness_runtime() {
            return self.openharness_session_native(&request, None).await;
        }
        let payload = self
            .request(
                "create_session",
                &request,
                json!({
                    "objective": request.objective,
                    "metadata": request.metadata,
                    "context": request_context(&request),
                }),
            )
            .await?;
        parse_native_session(payload)
    }

    async fn fork_session(
        &mut self,
        request: HarnessSessionRequest,
        native_session_id: Option<&str>,
    ) -> Result<NativeSession, HarnessAdapterError> {
        if self.uses_openai_compatible_shim() {
            if let Some(native_session_id) = native_session_id {
                self.validate_shim_session(request.session_id, native_session_id)?;
            }
            return Ok(NativeSession {
                native_session_id: Self::shim_session_id(request.session_id),
                native_cursor: None,
            });
        }
        if self.uses_protocol_runtime() {
            return self
                .protocol_session_native(
                    SessionOperation::Fork {
                        native_session_id: native_session_id.map(str::to_owned),
                    },
                    &request,
                )
                .await;
        }
        if self.uses_pi_runtime() {
            return Err(HarnessAdapterError::InvalidRequest(
                "Pi RPC does not support native session forks".to_owned(),
            ));
        }
        if self.uses_opencode_runtime() {
            return Err(HarnessAdapterError::InvalidRequest(
                "OpenCode does not support native session forks".to_owned(),
            ));
        }
        if self.uses_openhands_runtime() {
            return Err(HarnessAdapterError::InvalidRequest(
                "OpenHands does not support native conversation forks".to_owned(),
            ));
        }
        if self.uses_openharness_runtime() {
            return Err(HarnessAdapterError::InvalidRequest(
                "OpenHarness does not support native session forks".to_owned(),
            ));
        }
        let payload = self
            .request(
                "fork_session",
                &request,
                json!({
                    "native_session_id": native_session_id,
                    "context": request_context(&request),
                }),
            )
            .await?;
        parse_native_session(payload)
    }

    async fn import_session(
        &mut self,
        mut request: HarnessSessionRequest,
        capsule: SessionCapsule,
    ) -> Result<NativeSession, HarnessAdapterError> {
        capsule
            .verify_integrity()
            .map_err(|error| HarnessAdapterError::Capsule(format!("{error:?}")))?;
        if capsule.manifest.tenant_id != request.tenant_id
            || capsule.manifest.session_id != request.session_id
            || capsule.manifest.target_harness_id != self.spec.harness_id
        {
            return Err(HarnessAdapterError::Capsule(
                "capsule identity does not match target request".to_owned(),
            ));
        }
        if request.local_service_bundle.is_none()
            && !capsule.manifest.local_service_bindings.is_empty()
        {
            request.local_service_bundle = Some(LocalServiceBundle {
                schema: LOCAL_SERVICE_BUNDLE_SCHEMA.to_owned(),
                bindings: capsule.manifest.local_service_bindings.clone(),
            });
        }
        if self.uses_openai_compatible_shim() {
            let native_session_id = Self::shim_session_id(request.session_id);
            self.remember_portable_import(&native_session_id, portable_capsule_context(&capsule)?);
            return Ok(NativeSession {
                native_session_id,
                native_cursor: None,
            });
        }
        if self.spec.protocol_kind == HarnessProtocolKind::DeepSeekJsonRpc {
            let native_session_id = request.session_id.to_string();
            self.remember_portable_import(&native_session_id, portable_capsule_context(&capsule)?);
            return Ok(NativeSession {
                native_session_id,
                native_cursor: None,
            });
        }
        if self.spec.protocol_kind == HarnessProtocolKind::KimiAcp {
            let native = self
                .protocol_session_native(SessionOperation::Create, &request)
                .await?;
            self.remember_portable_import(
                &native.native_session_id,
                portable_capsule_context(&capsule)?,
            );
            return Ok(native);
        }
        if self.uses_protocol_runtime() {
            return self
                .protocol_session_native(SessionOperation::Import(capsule), &request)
                .await;
        }
        if self.uses_pi_runtime() {
            self.ensure_pi_runtime(&request).await?;
            let native_session_id = self
                .pi
                .as_ref()
                .expect("Pi runtime initialized")
                .native_session_id
                .clone();
            self.remember_portable_import(&native_session_id, portable_capsule_context(&capsule)?);
            return Ok(NativeSession {
                native_session_id,
                native_cursor: None,
            });
        }
        if self.uses_opencode_runtime() {
            let native = self.opencode_session_native(&request, None).await?;
            self.remember_portable_import(
                &native.native_session_id,
                portable_capsule_context(&capsule)?,
            );
            return Ok(native);
        }
        if self.uses_openhands_runtime() {
            let native = self.openhands_session_native(&request, None).await?;
            self.remember_portable_import(
                &native.native_session_id,
                portable_capsule_context(&capsule)?,
            );
            return Ok(native);
        }
        if self.uses_openharness_runtime() {
            let native = self.openharness_session_native(&request, None).await?;
            self.remember_portable_import(
                &native.native_session_id,
                portable_capsule_context(&capsule)?,
            );
            return Ok(native);
        }
        let payload = self
            .request(
                "import_session",
                &request,
                json!({"capsule": capsule, "context": request_context(&request)}),
            )
            .await?;
        parse_native_session(payload)
    }

    async fn resume_session(
        &mut self,
        request: HarnessSessionRequest,
        native_session_id: &str,
    ) -> Result<NativeSession, HarnessAdapterError> {
        if native_session_id.trim().is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(
                "native session id is empty".to_owned(),
            ));
        }
        if self.uses_openai_compatible_shim() {
            self.validate_shim_session(request.session_id, native_session_id)?;
            return Ok(NativeSession {
                native_session_id: native_session_id.to_owned(),
                native_cursor: None,
            });
        }
        if self.uses_protocol_runtime() {
            return self
                .protocol_session_native(
                    SessionOperation::Resume {
                        native_session_id: native_session_id.to_owned(),
                    },
                    &request,
                )
                .await;
        }
        if self.uses_pi_runtime() {
            self.ensure_pi_runtime(&request).await?;
            let active = &self
                .pi
                .as_ref()
                .expect("Pi runtime initialized")
                .native_session_id;
            if native_session_id != active {
                return Err(HarnessAdapterError::InvalidRequest(
                    "Pi RPC native session id does not match the active process".to_owned(),
                ));
            }
            return Ok(NativeSession {
                native_session_id: active.clone(),
                native_cursor: None,
            });
        }
        if self.uses_opencode_runtime() {
            return self
                .opencode_session_native(&request, Some(native_session_id))
                .await;
        }
        if self.uses_openhands_runtime() {
            return self
                .openhands_session_native(&request, Some(native_session_id))
                .await;
        }
        if self.uses_openharness_runtime() {
            return self
                .openharness_session_native(&request, Some(native_session_id))
                .await;
        }
        let payload = self
            .request(
                "resume_session",
                &request,
                json!({"native_session_id": native_session_id}),
            )
            .await?;
        parse_native_session(payload)
    }

    async fn execute(
        &mut self,
        request: HarnessSessionRequest,
        attempt_id: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecution, HarnessAdapterError> {
        if self.uses_openai_compatible_shim() {
            if let Some(native_session_id) = native_session_id {
                self.validate_shim_session(request.session_id, native_session_id)?;
            }
            let prompt = native_session_id
                .map(|native_session_id| self.prompt_for_native_session(native_session_id, prompt))
                .unwrap_or_else(|| prompt.to_owned());
            if native_session_id.is_some() {
                self.mark_portable_import_applied(
                    native_session_id.expect("checked for shim prompt translation"),
                );
            }
            return self
                .execute_operation(
                    "execute",
                    request,
                    attempt_id,
                    &prompt,
                    native_session_id,
                )
                .await;
        }
        if self.uses_protocol_runtime() {
            return self
                .collect_protocol_execution(
                    AttemptOperation::Execute,
                    request,
                    attempt_id,
                    prompt,
                    native_session_id,
                )
                .await;
        }
        if self.uses_pi_runtime() {
            return self
                .collect_pi_execution(
                    AttemptOperation::Execute,
                    request,
                    attempt_id,
                    prompt,
                    native_session_id,
                )
                .await;
        }
        if self.uses_opencode_runtime() {
            return self
                .collect_opencode_execution(
                    AttemptOperation::Execute,
                    request,
                    attempt_id,
                    prompt,
                    native_session_id,
                )
                .await;
        }
        if self.uses_openhands_runtime() {
            return self
                .collect_openhands_execution(
                    AttemptOperation::Execute,
                    request,
                    attempt_id,
                    prompt,
                    native_session_id,
                )
                .await;
        }
        if self.uses_openharness_runtime() {
            return self
                .collect_openharness_execution(
                    AttemptOperation::Execute,
                    request,
                    attempt_id,
                    prompt,
                    native_session_id,
                )
                .await;
        }
        self.execute_operation("execute", request, attempt_id, prompt, native_session_id)
            .await
    }

    async fn attempt_stream(
        &mut self,
        operation: AttemptOperation,
        request: HarnessSessionRequest,
        attempt_id: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecutionStream, HarnessAdapterError> {
        let model_binding = self.model_binding_event(operation, &request);
        let stream = if self.uses_protocol_runtime() {
            self.protocol_attempt_stream(operation, request, attempt_id, prompt, native_session_id)
                .await?
        } else if self.uses_pi_runtime() {
            self.pi_attempt_stream(operation, request, attempt_id, prompt, native_session_id)
                .await?
        } else if self.uses_opencode_runtime() {
            self.opencode_attempt_stream(operation, request, attempt_id, prompt, native_session_id)
                .await?
        } else if self.uses_openhands_runtime() {
            self.openhands_attempt_stream(operation, request, attempt_id, prompt, native_session_id)
                .await?
        } else if self.uses_openharness_runtime() {
            self.openharness_attempt_stream(
                operation,
                request,
                attempt_id,
                prompt,
                native_session_id,
            )
            .await?
        } else {
            self.compatibility_attempt_stream(
                operation,
                request,
                attempt_id,
                prompt,
                native_session_id,
            )
            .await?
        };
        Ok(Self::prepend_model_binding(model_binding, stream))
    }

    async fn start_attempt(
        &mut self,
        request: HarnessSessionRequest,
        attempt_id: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecution, HarnessAdapterError> {
        if self.uses_protocol_runtime() {
            return self
                .collect_protocol_execution(
                    AttemptOperation::Start,
                    request,
                    attempt_id,
                    prompt,
                    native_session_id,
                )
                .await;
        }
        if self.uses_pi_runtime() {
            return self
                .collect_pi_execution(
                    AttemptOperation::Start,
                    request,
                    attempt_id,
                    prompt,
                    native_session_id,
                )
                .await;
        }
        if self.uses_opencode_runtime() {
            return self
                .collect_opencode_execution(
                    AttemptOperation::Start,
                    request,
                    attempt_id,
                    prompt,
                    native_session_id,
                )
                .await;
        }
        if self.uses_openhands_runtime() {
            return self
                .collect_openhands_execution(
                    AttemptOperation::Start,
                    request,
                    attempt_id,
                    prompt,
                    native_session_id,
                )
                .await;
        }
        if self.uses_openharness_runtime() {
            return self
                .collect_openharness_execution(
                    AttemptOperation::Start,
                    request,
                    attempt_id,
                    prompt,
                    native_session_id,
                )
                .await;
        }
        self.execute_operation("start", request, attempt_id, prompt, native_session_id)
            .await
    }

    async fn resume_attempt(
        &mut self,
        request: HarnessSessionRequest,
        attempt_id: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecution, HarnessAdapterError> {
        if self.uses_protocol_runtime() {
            return self
                .collect_protocol_execution(
                    AttemptOperation::Resume,
                    request,
                    attempt_id,
                    prompt,
                    native_session_id,
                )
                .await;
        }
        if self.uses_pi_runtime() {
            return self
                .collect_pi_execution(
                    AttemptOperation::Resume,
                    request,
                    attempt_id,
                    prompt,
                    native_session_id,
                )
                .await;
        }
        if self.uses_opencode_runtime() {
            return self
                .collect_opencode_execution(
                    AttemptOperation::Resume,
                    request,
                    attempt_id,
                    prompt,
                    native_session_id,
                )
                .await;
        }
        if self.uses_openhands_runtime() {
            return self
                .collect_openhands_execution(
                    AttemptOperation::Resume,
                    request,
                    attempt_id,
                    prompt,
                    native_session_id,
                )
                .await;
        }
        if self.uses_openharness_runtime() {
            return self
                .collect_openharness_execution(
                    AttemptOperation::Resume,
                    request,
                    attempt_id,
                    prompt,
                    native_session_id,
                )
                .await;
        }
        self.execute_operation("resume", request, attempt_id, prompt, native_session_id)
            .await
    }

    async fn checkpoint(
        &mut self,
        request: HarnessSessionRequest,
        attempt_id: &str,
    ) -> Result<NativeCheckpoint, HarnessAdapterError> {
        if attempt_id.trim().is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(
                "attempt id is empty".to_owned(),
            ));
        }
        if self.uses_openai_compatible_shim() {
            return Ok(NativeCheckpoint {
                checkpoint_ref: format!("shim:{attempt_id}"),
                native_cursor: None,
            });
        }
        if self.uses_protocol_runtime() {
            let request = self.protocol_request_with_state(&request).await;
            let native = self
                .protocol_session_native(SessionOperation::Snapshot, &request)
                .await?;
            return Ok(NativeCheckpoint {
                checkpoint_ref: format!("codex-thread:{}", native.native_session_id),
                native_cursor: native.native_cursor,
            });
        }
        if self.uses_pi_runtime() {
            let execution = self
                .collect_pi_execution(AttemptOperation::Reconcile, request, attempt_id, "", None)
                .await?;
            return Ok(NativeCheckpoint {
                checkpoint_ref: format!("pi-state:{attempt_id}"),
                native_cursor: execution
                    .events
                    .last()
                    .and_then(|event| event.native_cursor.clone()),
            });
        }
        if self.uses_opencode_runtime() {
            return Err(HarnessAdapterError::InvalidRequest(
                "OpenCode does not expose native checkpoints".to_owned(),
            ));
        }
        if self.uses_openhands_runtime() {
            return Err(HarnessAdapterError::InvalidRequest(
                "OpenHands does not expose native checkpoints".to_owned(),
            ));
        }
        if self.uses_openharness_runtime() {
            return Err(HarnessAdapterError::InvalidRequest(
                "OpenHarness does not expose native checkpoints".to_owned(),
            ));
        }
        let payload = self
            .request(
                "checkpoint",
                &request,
                json!({"attempt_id": attempt_id, "context": request_context(&request)}),
            )
            .await?;
        serde_json::from_value(payload).map_err(HarnessAdapterError::Json)
    }

    async fn close_session(
        &mut self,
        request: HarnessSessionRequest,
    ) -> Result<(), HarnessAdapterError> {
        if self.uses_openai_compatible_shim() {
            return self.terminate().await;
        }
        if matches!(
            self.spec.protocol_kind,
            HarnessProtocolKind::DeepSeekJsonRpc | HarnessProtocolKind::KimiAcp
        ) {
            return self.terminate().await;
        }
        if self.uses_protocol_runtime() {
            let request = self.protocol_request_with_state(&request).await;
            return self
                .protocol_session_command(SessionOperation::Close, &request)
                .await;
        }
        if self.uses_pi_runtime() {
            if let Some(pi) = self.pi.take() {
                pi.runtime.shutdown().await?;
            }
            self.pending_portable_imports
                .lock()
                .expect("portable import lock is not poisoned")
                .clear();
            return Ok(());
        }
        if self.uses_opencode_runtime() {
            return self.terminate().await;
        }
        if self.uses_openhands_runtime() {
            return self.terminate().await;
        }
        if self.uses_openharness_runtime() {
            return self.terminate().await;
        }
        self.request("close_session", &request, Value::Null).await?;
        self.terminate().await
    }

    async fn delete_session(
        &mut self,
        request: HarnessSessionRequest,
    ) -> Result<(), HarnessAdapterError> {
        if self.uses_openai_compatible_shim() {
            return self.terminate().await;
        }
        if matches!(
            self.spec.protocol_kind,
            HarnessProtocolKind::DeepSeekJsonRpc | HarnessProtocolKind::KimiAcp
        ) {
            return self.terminate().await;
        }
        if self.uses_protocol_runtime() {
            let request = self.protocol_request_with_state(&request).await;
            self.protocol_session_command(SessionOperation::Delete, &request)
                .await?;
            if let Some(protocol) = &self.protocol {
                if let Some(thread_id) = request
                    .extensions
                    .get("native_session_id")
                    .and_then(Value::as_str)
                {
                    protocol.states.lock().await.remove(thread_id);
                    let mut current = protocol.state.lock().await;
                    if current.thread_id == thread_id {
                        *current = NativeTurnState::new("");
                    }
                }
            }
            return Ok(());
        }
        if self.uses_pi_runtime() {
            if let Some(pi) = self.pi.take() {
                pi.runtime.shutdown().await?;
            }
            self.pending_portable_imports
                .lock()
                .expect("portable import lock is not poisoned")
                .clear();
            return Ok(());
        }
        if self.uses_opencode_runtime() {
            let native_session_id = request
                .extensions
                .get("native_session_id")
                .and_then(Value::as_str)
                .filter(|value| !value.trim().is_empty())
                .map(str::to_owned)
                .or_else(|| {
                    self.opencode
                        .as_ref()
                        .and_then(|adapter| adapter.native_session_id.clone())
                })
                .ok_or_else(|| {
                    HarnessAdapterError::InvalidRequest(
                        "OpenCode native session id is required for deletion".to_owned(),
                    )
                })?;
            self.opencode
                .as_ref()
                .ok_or(HarnessAdapterError::ProcessExited)?
                .runtime
                .delete_session(&native_session_id)
                .await?;
            return self.terminate().await;
        }
        if self.uses_openhands_runtime() {
            let native_session_id = request
                .extensions
                .get("native_session_id")
                .and_then(Value::as_str)
                .filter(|value| !value.trim().is_empty())
                .map(str::to_owned)
                .or_else(|| {
                    self.openhands
                        .as_ref()
                        .and_then(|adapter| adapter.native_session_id.clone())
                })
                .ok_or_else(|| {
                    HarnessAdapterError::InvalidRequest(
                        "OpenHands native conversation id is required for deletion".to_owned(),
                    )
                })?;
            self.openhands
                .as_ref()
                .ok_or(HarnessAdapterError::ProcessExited)?
                .runtime
                .delete_conversation(&native_session_id)
                .await
                .map_err(classify_openhands_error)?;
            return self.terminate().await;
        }
        if self.uses_openharness_runtime() {
            let native_session_id = request
                .extensions
                .get("native_session_id")
                .and_then(Value::as_str)
                .filter(|value| !value.trim().is_empty())
                .map(str::to_owned)
                .or_else(|| {
                    self.openharness
                        .as_ref()
                        .and_then(|adapter| adapter.native_session_id.clone())
                })
                .ok_or_else(|| {
                    HarnessAdapterError::InvalidRequest(
                        "OpenHarness native session id is required for deletion".to_owned(),
                    )
                })?;
            let event = self
                .openharness
                .as_ref()
                .ok_or(HarnessAdapterError::ProcessExited)?
                .runtime
                .request(OpenHarnessCommand::delete_session(
                    format!("omnisolo-session-delete-{}", Uuid::new_v4()),
                    native_session_id,
                ))
                .await?;
            if !matches!(event, OpenHarnessEvent::SessionDeleted { .. }) {
                return Err(HarnessAdapterError::InvalidResponse(
                    "OpenHarness delete did not return a session_deleted event".to_owned(),
                ));
            }
            return self.terminate().await;
        }
        self.request("delete_session", &request, Value::Null)
            .await?;
        self.terminate().await
    }

    async fn control_session(
        &mut self,
        request: HarnessSessionRequest,
        operation: &str,
        payload: Value,
    ) -> Result<NativeSession, HarnessAdapterError> {
        if self.uses_openai_compatible_shim() {
            let default_session_id = Self::shim_session_id(request.session_id);
            let native_session_id = payload
                .get("native_session_id")
                .and_then(Value::as_str)
                .map(str::to_owned)
                .unwrap_or(default_session_id);
            self.validate_shim_session(request.session_id, &native_session_id)?;
            if matches!(operation, "cancel" | "quiesce") {
                self.terminate().await?;
            } else if operation != "snapshot" {
                return Err(HarnessAdapterError::InvalidRequest(format!(
                    "unsupported OpenAI-compatible shim session operation: {operation}"
                )));
            }
            return Ok(NativeSession {
                native_session_id,
                native_cursor: None,
            });
        }
        if self.uses_protocol_runtime() {
            let mut request = request;
            if let Some(native_session_id) = payload
                .get("native_session_id")
                .and_then(Value::as_str)
                .filter(|value| !value.trim().is_empty())
            {
                request.extensions.insert(
                    "native_session_id".to_owned(),
                    Value::String(native_session_id.to_owned()),
                );
            }
            let request = self.protocol_request_with_state(&request).await;
            let operation = match operation {
                "snapshot" => SessionOperation::Snapshot,
                "quiesce" => SessionOperation::Quiesce,
                "cancel" => SessionOperation::Cancel,
                _ => {
                    return Err(HarnessAdapterError::InvalidRequest(format!(
                        "unsupported native session operation: {operation}"
                    )));
                }
            };
            return self.protocol_session_native(operation, &request).await;
        }
        if self.uses_pi_runtime() {
            self.ensure_pi_runtime(&request).await?;
            return match operation {
                "snapshot" => Ok(NativeSession {
                    native_session_id: self
                        .pi
                        .as_ref()
                        .expect("Pi runtime initialized")
                        .native_session_id
                        .clone(),
                    native_cursor: None,
                }),
                _ => Err(HarnessAdapterError::InvalidRequest(format!(
                    "unsupported Pi session operation: {operation}"
                ))),
            };
        }
        if self.uses_opencode_runtime() {
            self.ensure_opencode_runtime(&request).await?;
            let native_session_id = payload
                .get("native_session_id")
                .and_then(Value::as_str)
                .filter(|value| !value.trim().is_empty());
            let native_session_id = self.opencode_session_id(native_session_id)?;
            return match operation {
                "cancel" | "quiesce" => {
                    self.opencode
                        .as_ref()
                        .expect("OpenCode runtime initialized")
                        .runtime
                        .abort(&native_session_id)
                        .await?;
                    Ok(NativeSession {
                        native_session_id,
                        native_cursor: None,
                    })
                }
                _ => Err(HarnessAdapterError::InvalidRequest(format!(
                    "unsupported OpenCode session operation: {operation}"
                ))),
            };
        }
        if self.uses_openhands_runtime() {
            self.ensure_openhands_runtime(&request).await?;
            let native_session_id = payload
                .get("native_session_id")
                .and_then(Value::as_str)
                .filter(|value| !value.trim().is_empty());
            let native_session_id = self.openhands_session_id(native_session_id)?;
            return match operation {
                "cancel" | "quiesce" => {
                    self.openhands
                        .as_ref()
                        .expect("OpenHands runtime initialized")
                        .runtime
                        .cancel(&native_session_id)
                        .await
                        .map_err(classify_openhands_error)?;
                    Ok(NativeSession {
                        native_session_id,
                        native_cursor: None,
                    })
                }
                _ => Err(HarnessAdapterError::InvalidRequest(format!(
                    "unsupported OpenHands session operation: {operation}"
                ))),
            };
        }
        if self.uses_openharness_runtime() {
            self.ensure_openharness_runtime().await?;
            let native_session_id = payload
                .get("native_session_id")
                .and_then(Value::as_str)
                .filter(|value| !value.trim().is_empty())
                .map(str::to_owned)
                .or_else(|| {
                    self.openharness
                        .as_ref()
                        .and_then(|adapter| adapter.native_session_id.clone())
                })
                .ok_or_else(|| {
                    HarnessAdapterError::InvalidRequest(
                        "OpenHarness native session id is required".to_owned(),
                    )
                })?;
            match operation {
                "cancel" | "quiesce" => {
                    self.openharness
                        .as_ref()
                        .expect("OpenHarness runtime initialized")
                        .runtime
                        .request(OpenHarnessCommand::cancel(
                            format!("omnisolo-session-control-{}", Uuid::new_v4()),
                            &native_session_id,
                        ))
                        .await?;
                    return Ok(NativeSession {
                        native_session_id,
                        native_cursor: None,
                    });
                }
                _ => {
                    return Err(HarnessAdapterError::InvalidRequest(format!(
                        "unsupported OpenHarness session operation: {operation}"
                    )));
                }
            }
        }
        parse_native_session(self.request(operation, &request, payload).await?)
    }

    async fn control_attempt(
        &mut self,
        request: HarnessSessionRequest,
        attempt_id: &str,
        operation: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecution, HarnessAdapterError> {
        if self.uses_openai_compatible_shim() {
            if let Some(native_session_id) = native_session_id {
                self.validate_shim_session(request.session_id, native_session_id)?;
            }
            return match operation {
                "steer" => self.execute(request, attempt_id, prompt, native_session_id).await,
                "cancel" | "quiesce" => {
                    self.terminate().await?;
                    Ok(HarnessExecution {
                        events: Vec::new(),
                        final_text: None,
                        usage: None,
                    })
                }
                "reconcile" => Ok(HarnessExecution {
                    events: Vec::new(),
                    final_text: None,
                    usage: None,
                }),
                _ => Err(HarnessAdapterError::InvalidRequest(format!(
                    "unsupported OpenAI-compatible shim attempt operation: {operation}"
                ))),
            };
        }
        if self.uses_protocol_runtime() {
            let operation = match operation {
                "steer" => AttemptOperation::Steer,
                "cancel" => AttemptOperation::Cancel,
                "quiesce" => AttemptOperation::Quiesce,
                "reconcile" => AttemptOperation::Reconcile,
                _ => {
                    return Err(HarnessAdapterError::InvalidRequest(format!(
                        "unsupported native attempt operation: {operation}"
                    )));
                }
            };
            return self
                .collect_protocol_execution(
                    operation,
                    request,
                    attempt_id,
                    prompt,
                    native_session_id,
                )
                .await;
        }
        if self.uses_pi_runtime() {
            let operation = match operation {
                "steer" => AttemptOperation::Steer,
                "cancel" => AttemptOperation::Cancel,
                "quiesce" => AttemptOperation::Quiesce,
                "reconcile" => AttemptOperation::Reconcile,
                _ => {
                    return Err(HarnessAdapterError::InvalidRequest(format!(
                        "unsupported Pi attempt operation: {operation}"
                    )));
                }
            };
            return self
                .collect_pi_execution(operation, request, attempt_id, prompt, native_session_id)
                .await;
        }
        if self.uses_opencode_runtime() {
            let operation = match operation {
                "cancel" => AttemptOperation::Cancel,
                "quiesce" => AttemptOperation::Quiesce,
                "steer" => AttemptOperation::Steer,
                "reconcile" => AttemptOperation::Reconcile,
                _ => {
                    return Err(HarnessAdapterError::InvalidRequest(format!(
                        "unsupported OpenCode attempt operation: {operation}"
                    )));
                }
            };
            return self
                .collect_opencode_execution(
                    operation,
                    request,
                    attempt_id,
                    prompt,
                    native_session_id,
                )
                .await;
        }
        if self.uses_openhands_runtime() {
            let operation = match operation {
                "cancel" => AttemptOperation::Cancel,
                "quiesce" => AttemptOperation::Quiesce,
                "steer" => AttemptOperation::Steer,
                "reconcile" => AttemptOperation::Reconcile,
                _ => {
                    return Err(HarnessAdapterError::InvalidRequest(format!(
                        "unsupported OpenHands attempt operation: {operation}"
                    )));
                }
            };
            return self
                .collect_openhands_execution(
                    operation,
                    request,
                    attempt_id,
                    prompt,
                    native_session_id,
                )
                .await;
        }
        if self.uses_openharness_runtime() {
            let operation = match operation {
                "steer" => AttemptOperation::Steer,
                "cancel" => AttemptOperation::Cancel,
                "quiesce" => AttemptOperation::Quiesce,
                "reconcile" => AttemptOperation::Reconcile,
                _ => {
                    return Err(HarnessAdapterError::InvalidRequest(format!(
                        "unsupported OpenHarness attempt operation: {operation}"
                    )));
                }
            };
            return self
                .collect_openharness_execution(
                    operation,
                    request,
                    attempt_id,
                    prompt,
                    native_session_id,
                )
                .await;
        }
        if attempt_id.trim().is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(
                "attempt id is empty".to_owned(),
            ));
        }
        let payload = self
            .request(
                operation,
                &request,
                json!({
                    "attempt_id": attempt_id,
                    "prompt": prompt,
                    "native_session_id": native_session_id,
                    "context": request_context(&request),
                }),
            )
            .await?;
        serde_json::from_value(payload).map_err(HarnessAdapterError::Json)
    }

    async fn exchange(
        &mut self,
        request: HarnessSessionRequest,
        kind: &str,
        payload: Value,
    ) -> Result<Value, HarnessAdapterError> {
        if self.uses_protocol_runtime() {
            let protocol = self
                .protocol
                .as_ref()
                .ok_or(HarnessAdapterError::ProcessExited)?;
            return protocol.exchange(kind, &payload).await;
        }
        if self.uses_opencode_runtime() {
            return Err(HarnessAdapterError::InvalidRequest(
                "OpenCode does not expose a generic exchange command".to_owned(),
            ));
        }
        if self.uses_openhands_runtime() {
            self.ensure_openhands_runtime(&request).await?;
            if !matches!(kind, "approval.response" | "approval.requested") {
                return Err(HarnessAdapterError::InvalidRequest(format!(
                    "unsupported OpenHands exchange kind: {kind}"
                )));
            }
            let native_session_id = payload
                .get("native_session_id")
                .and_then(Value::as_str)
                .filter(|value| !value.trim().is_empty());
            let native_session_id = self.openhands_session_id(native_session_id)?;
            let accept = payload
                .get("accept")
                .and_then(Value::as_bool)
                .or_else(|| {
                    payload
                        .get("result")
                        .and_then(|result| result.get("decision"))
                        .and_then(Value::as_str)
                        .map(|decision| matches!(decision, "accept" | "approve" | "approved"))
                })
                .ok_or_else(|| {
                    HarnessAdapterError::InvalidRequest(
                        "OpenHands approval response requires an accept decision".to_owned(),
                    )
                })?;
            let reason = payload
                .get("reason")
                .and_then(Value::as_str)
                .unwrap_or(if accept { "approved" } else { "rejected" });
            self.openhands
                .as_ref()
                .expect("OpenHands runtime initialized")
                .runtime
                .respond_to_approval(&native_session_id, accept, reason)
                .await
                .map_err(classify_openhands_error)?;
            return Ok(json!({"accepted": accept}));
        }
        if self.uses_openharness_runtime() {
            return Err(HarnessAdapterError::InvalidRequest(
                "OpenHarness does not expose a generic exchange command".to_owned(),
            ));
        }
        self.request(
            "exchange",
            &request,
            json!({
                "kind": kind,
                "payload": payload,
                "context": request_context(&request),
            }),
        )
        .await
    }
}

/// Adapter boundary for the first-party OmniSolo harness. It deliberately
/// implements the same trait as external process adapters so routing and
/// handoff do not privilege the native implementation.
pub struct OmniSoloHarnessAdapterBridge {
    descriptor: HarnessDescriptor,
    run: Option<OmniSoloHarnessAdapter>,
    imported_capsule: Option<SessionCapsule>,
    provider_client: Option<OpenAiResponsesClient>,
}

impl std::fmt::Debug for OmniSoloHarnessAdapterBridge {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OmniSoloHarnessAdapterBridge")
            .field("started", &self.run.is_some())
            .field("imported_capsule", &self.imported_capsule.is_some())
            .field("provider_backed", &self.provider_client.is_some())
            .finish()
    }
}

impl OmniSoloHarnessAdapterBridge {
    pub fn new(descriptor: HarnessDescriptor) -> Self {
        Self {
            descriptor,
            run: None,
            imported_capsule: None,
            provider_client: None,
        }
    }

    pub fn with_provider_client(
        descriptor: HarnessDescriptor,
        provider_client: OpenAiResponsesClient,
    ) -> Self {
        Self {
            descriptor,
            run: None,
            imported_capsule: None,
            provider_client: Some(provider_client),
        }
    }

    pub fn imported_capsule(&self) -> Option<&SessionCapsule> {
        self.imported_capsule.as_ref()
    }

    fn native_session_id(session_id: Uuid) -> String {
        format!("omnisolo:{session_id}")
    }

    fn start_run(
        &mut self,
        request: &HarnessSessionRequest,
    ) -> Result<NativeSession, HarnessAdapterError> {
        let objective = if request.objective.trim().is_empty() {
            "Continue the portable OmniSolo session".to_owned()
        } else {
            request.objective.clone()
        };
        let mut config = OmniSoloRunConfig::new(request.tenant_id.clone(), objective)
            .with_session_id(request.session_id)
            .with_harness("omnisolo");
        config.attempt_id = request.attempt_id;
        if let Some(binding) = request.local_service_bundle.as_ref()
            .and_then(|bundle| bundle.bindings.first())
        {
            config.project_id = binding.project_id.clone();
            config.workspace_id = binding.workspace_id.clone();
        }
        if let Some(task_id) = request.task_id {
            config = config.with_task_id(task_id);
        }
        if request.turn_id.is_some() {
            config = config.with_turn();
        }
        let mut run = OmniSoloHarnessAdapter::start(config)
            .map_err(|error| HarnessAdapterError::Remote(error.to_string()))?;
        Self::retain_request_services(&mut run, request)?;
        self.run = Some(run);
        Ok(NativeSession {
            native_session_id: Self::native_session_id(request.session_id),
            native_cursor: self
                .run
                .as_ref()
                .and_then(OmniSoloHarnessAdapter::last_durable_sequence)
                .map(|sequence| sequence.to_string()),
        })
    }

    fn retain_request_services(
        run: &mut OmniSoloHarnessAdapter,
        request: &HarnessSessionRequest,
    ) -> Result<(), HarnessAdapterError> {
        use super::local_services::LocalServiceScopeContext;
        if let Some(bundle) = request.local_service_bundle.clone() {
            let first = bundle.bindings.first();
            let context = LocalServiceScopeContext::new(
                request.tenant_id.clone(),
                first.and_then(|binding| binding.project_id.clone()),
                first.and_then(|binding| binding.workspace_id.clone()),
                request.session_id,
                request.task_id,
                request.attempt_id.or_else(|| first.and_then(|binding| binding.attempt_id)),
            );
            run.retain_local_service_references(bundle, &context)
                .map_err(|error| HarnessAdapterError::InvalidRequest(error.to_string()))?;
        }
        Ok(())
    }

    fn execution_since(&self, start: usize) -> HarnessExecution {
        let events = self
            .run
            .as_ref()
            .map(|run| {
                run.events()[start..]
                    .iter()
                    .map(|event| HarnessEvent {
                        event_type: event.event_type.clone(),
                        durable: event.durability == super::types::EventDurability::Durable,
                        payload: event.payload.clone(),
                        native_cursor: event.durable_sequence.map(|sequence| sequence.to_string()),
                    })
                    .collect()
            })
            .unwrap_or_default();
        HarnessExecution {
            events,
            final_text: None,
            usage: None,
        }
    }
}

#[async_trait]
impl HarnessAdapter for OmniSoloHarnessAdapterBridge {
    fn descriptor(&self) -> &HarnessDescriptor {
        &self.descriptor
    }

    async fn create_session(
        &mut self,
        request: HarnessSessionRequest,
    ) -> Result<NativeSession, HarnessAdapterError> {
        if self.run.is_some() {
            return Err(HarnessAdapterError::Remote(
                "OmniSolo native session is already active".to_owned(),
            ));
        }
        self.start_run(&request)
    }

    async fn fork_session(
        &mut self,
        request: HarnessSessionRequest,
        _native_session_id: Option<&str>,
    ) -> Result<NativeSession, HarnessAdapterError> {
        self.run = None;
        self.start_run(&request)
    }

    async fn import_session(
        &mut self,
        request: HarnessSessionRequest,
        capsule: SessionCapsule,
    ) -> Result<NativeSession, HarnessAdapterError> {
        let imported =
            OmniSoloHarnessAdapter::import_capsule(&capsule, &request.tenant_id, "omnisolo")
                .map_err(|error| HarnessAdapterError::Capsule(error.to_string()))?;
        self.imported_capsule = Some(capsule.clone());
        let mut request = request.with_capsule(capsule.clone());
        if request.local_service_bundle.is_none() && !capsule.manifest.local_service_bindings.is_empty() {
            request.local_service_bundle = Some(LocalServiceBundle {
                schema: LOCAL_SERVICE_BUNDLE_SCHEMA.to_owned(),
                bindings: capsule.manifest.local_service_bindings.clone(),
            });
        }
        if request.objective.trim().is_empty() {
            request.objective = imported
                .records
                .iter()
                .find_map(|record| match record {
                    super::capsule::PortableRecord::Task(task) => Some(task.objective.clone()),
                    _ => None,
                })
                .unwrap_or_default();
        }
        self.start_run(&request)
    }

    async fn resume_session(
        &mut self,
        request: HarnessSessionRequest,
        native_session_id: &str,
    ) -> Result<NativeSession, HarnessAdapterError> {
        if native_session_id != Self::native_session_id(request.session_id) {
            return Err(HarnessAdapterError::InvalidRequest(
                "native session id does not belong to this OmniSolo session".to_owned(),
            ));
        }
        if self.run.is_none() {
            self.start_run(&request)
        } else {
            Ok(NativeSession {
                native_session_id: native_session_id.to_owned(),
                native_cursor: self
                    .run
                    .as_ref()
                    .and_then(OmniSoloHarnessAdapter::last_durable_sequence)
                    .map(|sequence| sequence.to_string()),
            })
        }
    }

    async fn execute(
        &mut self,
        request: HarnessSessionRequest,
        attempt_id: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecution, HarnessAdapterError> {
        if attempt_id.trim().is_empty() || prompt.trim().is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(
                "attempt id and prompt are required".to_owned(),
            ));
        }
        if let Some(native_session_id) = native_session_id {
            if native_session_id != Self::native_session_id(request.session_id) {
                return Err(HarnessAdapterError::InvalidRequest(
                    "native session id does not belong to this OmniSolo session".to_owned(),
                ));
            }
        }
        if self.run.is_none() {
            self.start_run(&request)?;
        }
        Self::retain_request_services(
            self.run.as_mut().expect("start_run initializes the OmniSolo run"),
            &request,
        )?;
        if let Some(provider_client) = self.provider_client.clone() {
            let selection = request.resolved_model.as_ref().ok_or_else(|| {
                HarnessAdapterError::InvalidRequest(
                    "provider-backed OmniSolo execution requires a resolved model selection"
                        .to_owned(),
                )
            })?;
            let result = provider_client
                .execute(selection, prompt)
                .await
                .map_err(|error| HarnessAdapterError::Remote(error.to_string()))?;
            let run = self
                .run
                .as_mut()
                .expect("start_run initializes the OmniSolo run");
            let start = run.events().len();
            run.record(OmniSoloEvent::TextChunk {
                content: result.text.clone(),
            })
            .map_err(|error| HarnessAdapterError::Remote(error.to_string()))?;
            let mut execution = self.execution_since(start);
            execution.events.insert(
                0,
                HarnessEvent {
                    event_type: "inference.model_binding".to_owned(),
                    durable: true,
                    payload: json!({
                        "integration_mode": "native",
                        "provider_route": selection.provider_route,
                        "model_id": result.model,
                        "reasoning_effort": selection.reasoning_effort,
                        "binding_revision": result.binding_revision,
                        "binding_digest": result.binding_digest,
                        "response_id": result.response_id,
                    }),
                    native_cursor: None,
                },
            );
            execution.final_text = Some(result.text);
            execution.usage = Some(json!({
                "input_tokens": result.usage.input_tokens,
                "output_tokens": result.usage.output_tokens,
                "cached_tokens": result.usage.cached_tokens,
            }));
            return Ok(execution);
        }
        let run = self
            .run
            .as_mut()
            .expect("start_run initializes the OmniSolo run");
        let start = run.events().len();
        run.record(OmniSoloEvent::TextChunk {
            content: prompt.to_owned(),
        })
        .map_err(|error| HarnessAdapterError::Remote(error.to_string()))?;
        Ok(self.execution_since(start))
    }

    async fn checkpoint(
        &mut self,
        _request: HarnessSessionRequest,
        attempt_id: &str,
    ) -> Result<NativeCheckpoint, HarnessAdapterError> {
        if attempt_id.trim().is_empty() {
            return Err(HarnessAdapterError::InvalidRequest(
                "attempt id is empty".to_owned(),
            ));
        }
        let sequence = self
            .run
            .as_ref()
            .and_then(OmniSoloHarnessAdapter::last_durable_sequence)
            .ok_or_else(|| {
                HarnessAdapterError::Remote("no durable OmniSolo checkpoint".to_owned())
            })?;
        Ok(NativeCheckpoint {
            checkpoint_ref: format!("omnisolo-durable:{sequence}"),
            native_cursor: Some(sequence.to_string()),
        })
    }

    async fn close_session(
        &mut self,
        _request: HarnessSessionRequest,
    ) -> Result<(), HarnessAdapterError> {
        self.run = None;
        Ok(())
    }

    async fn delete_session(
        &mut self,
        _request: HarnessSessionRequest,
    ) -> Result<(), HarnessAdapterError> {
        self.run = None;
        Ok(())
    }

    async fn control_session(
        &mut self,
        request: HarnessSessionRequest,
        operation: &str,
        _payload: Value,
    ) -> Result<NativeSession, HarnessAdapterError> {
        let native_session_id = Self::native_session_id(request.session_id);
        let native = NativeSession {
            native_session_id,
            native_cursor: self
                .run
                .as_ref()
                .and_then(OmniSoloHarnessAdapter::last_durable_sequence)
                .map(|sequence| sequence.to_string()),
        };
        match operation {
            "snapshot" => Ok(native),
            "quiesce" | "cancel" => {
                self.run = None;
                Ok(native)
            }
            _ => Err(HarnessAdapterError::InvalidRequest(format!(
                "unsupported OmniSolo session operation: {operation}"
            ))),
        }
    }

    async fn control_attempt(
        &mut self,
        request: HarnessSessionRequest,
        attempt_id: &str,
        operation: &str,
        prompt: &str,
        native_session_id: Option<&str>,
    ) -> Result<HarnessExecution, HarnessAdapterError> {
        match operation {
            "steer" => {
                self.execute(request, attempt_id, prompt, native_session_id)
                    .await
            }
            "wait" => Ok(HarnessExecution {
                events: Vec::new(),
                final_text: None,
                usage: None,
            }),
            "reconcile" => Ok(if self.run.is_some() {
                self.execution_since(0)
            } else {
                HarnessExecution {
                    events: Vec::new(),
                    final_text: None,
                    usage: None,
                }
            }),
            "quiesce" | "cancel" => {
                self.run = None;
                Ok(HarnessExecution {
                    events: Vec::new(),
                    final_text: None,
                    usage: None,
                })
            }
            _ => Err(HarnessAdapterError::InvalidRequest(format!(
                "unsupported OmniSolo attempt operation: {operation}"
            ))),
        }
    }

    async fn exchange(
        &mut self,
        _request: HarnessSessionRequest,
        _kind: &str,
        payload: Value,
    ) -> Result<Value, HarnessAdapterError> {
        Ok(payload)
    }
}

fn parse_native_session(payload: Value) -> Result<NativeSession, HarnessAdapterError> {
    let session: NativeSession =
        serde_json::from_value(payload).map_err(HarnessAdapterError::Json)?;
    if session.native_session_id.trim().is_empty() {
        return Err(HarnessAdapterError::InvalidResponse(
            "native session id is empty".to_owned(),
        ));
    }
    Ok(session)
}

fn request_context(request: &HarnessSessionRequest) -> Value {
    let mut context = json!({
        "model_binding_id": request.model_binding_id,
        "resolved_model": request.resolved_model,
        "runtime_config_snapshot_id": request.runtime_config_snapshot_id,
        "workspace_snapshot_id": request.workspace_snapshot_id,
        "artifact_ids": request.artifact_ids,
        "durable_sequence": request.durable_sequence,
        "capability_snapshot_id": request.capability_snapshot_id,
        "extensions": request.extensions,
    });
    if let Some(bundle) = &request.local_service_bundle {
        context["local_services"] =
            serde_json::to_value(bundle).expect("local service bundles are serializable");
    }
    context
}

fn native_message_thread_id(params: &Value) -> Option<&str> {
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
}

fn protocol_for_harness(harness_id: &str) -> HarnessProtocolKind {
    match harness_id {
        "omnisolo" => HarnessProtocolKind::OmniSolo,
        "codex" => HarnessProtocolKind::CodexAppServer,
        "opencode" => HarnessProtocolKind::OpenCodeHttp,
        "deepseek" => HarnessProtocolKind::DeepSeekJsonRpc,
        "pi" => HarnessProtocolKind::PiRpc,
        "kimi" => HarnessProtocolKind::KimiAcp,
        "openhands" => HarnessProtocolKind::OpenHandsHttp,
        "openharness" => HarnessProtocolKind::OpenHarnessSdk,
        "aider" | "goose" | "open-interpreter" | "plandex" => {
            HarnessProtocolKind::OpenAiCompatibleShim
        }
        _ => HarnessProtocolKind::Custom,
    }
}

fn default_capabilities() -> BTreeSet<HarnessCapability> {
    [
        HarnessCapability::Prompt,
        HarnessCapability::Streaming,
        HarnessCapability::Cancellation,
        HarnessCapability::Steering,
        HarnessCapability::Approvals,
        HarnessCapability::Questions,
        HarnessCapability::Workspace,
        HarnessCapability::Artifacts,
        HarnessCapability::Compaction,
        HarnessCapability::Branching,
        HarnessCapability::Subagents,
        HarnessCapability::ImportCapsule,
        HarnessCapability::ExportNativeState,
    ]
    .into_iter()
    .collect()
}

fn native_capabilities() -> BTreeSet<HarnessCapability> {
    [
        HarnessCapability::Prompt,
        HarnessCapability::Streaming,
        HarnessCapability::Cancellation,
        HarnessCapability::Steering,
        HarnessCapability::Branching,
        HarnessCapability::ExactResume,
        HarnessCapability::ImportCapsule,
        HarnessCapability::ExportNativeState,
    ]
    .into_iter()
    .collect()
}

fn native_descriptor() -> HarnessDescriptor {
    HarnessDescriptor {
        harness_id: "omnisolo".to_owned(),
        adapter_id: "omnisolo.native".to_owned(),
        display_name: "OmniSolo harness".to_owned(),
        implementation_version: env!("CARGO_PKG_VERSION").to_owned(),
        protocol_kind: HarnessProtocolKind::OmniSolo,
        integration_mode: HarnessIntegrationMode::Native,
        protocol_version: "omnisolo.worker.v1".to_owned(),
        source_revision: None,
        schema_revision: Some("omnisolo.session.v2".to_owned()),
        capabilities: native_capabilities(),
        worker_image: Some("omnisolo/harness-worker:0.1.0".to_owned()),
        worker_pool: Some("omnisolo".to_owned()),
        state_locality: Some("external_or_persistent_volume".to_owned()),
        native_extension_namespace: "omnisolo".to_owned(),
        metadata: [("integration_mode".to_owned(), "native".to_owned())]
            .into_iter()
            .collect(),
    }
}

fn external_descriptor(preset: ExternalHarnessPreset) -> HarnessDescriptor {
    let harness_id = preset.harness_id().to_owned();
    HarnessDescriptor {
        harness_id: harness_id.clone(),
        adapter_id: format!("{harness_id}.process"),
        display_name: harness_id.clone(),
        implementation_version: preset.implementation_version().to_owned(),
        protocol_kind: preset.protocol_kind(),
        integration_mode: preset.integration_mode(),
        protocol_version: "omnisolo.adapter.v1".to_owned(),
        source_revision: None,
        schema_revision: Some("omnisolo.session.v2".to_owned()),
        capabilities: preset.capabilities(),
        worker_image: Some(preset.worker_image()),
        worker_pool: Some(harness_id.clone()),
        state_locality: Some("adapter_defined".to_owned()),
        native_extension_namespace: harness_id,
        metadata: [(
            "integration_mode".to_owned(),
            serde_json::to_string(&preset.integration_mode())
                .unwrap_or_else(|_| "native".to_owned())
                .trim_matches('"')
                .to_owned(),
        )]
        .into_iter()
        .collect(),
    }
}

fn descriptor_for_spec(spec: &ProcessHarnessSpec) -> HarnessDescriptor {
    let mut descriptor = match spec.harness_id.as_str() {
        "codex" => external_descriptor(ExternalHarnessPreset::Codex),
        "opencode" => external_descriptor(ExternalHarnessPreset::OpenCode),
        "deepseek" => external_descriptor(ExternalHarnessPreset::DeepSeek),
        "pi" => external_descriptor(ExternalHarnessPreset::Pi),
        "kimi" => external_descriptor(ExternalHarnessPreset::Kimi),
        "openhands" => external_descriptor(ExternalHarnessPreset::OpenHands),
        "openharness" => external_descriptor(ExternalHarnessPreset::OpenHarness),
        "aider" => external_descriptor(ExternalHarnessPreset::Aider),
        "goose" => external_descriptor(ExternalHarnessPreset::Goose),
        "open-interpreter" => external_descriptor(ExternalHarnessPreset::OpenInterpreter),
        "plandex" => external_descriptor(ExternalHarnessPreset::Plandex),
        _ => HarnessDescriptor {
            harness_id: spec.harness_id.clone(),
            adapter_id: format!("{}.process", spec.harness_id),
            display_name: spec.harness_id.clone(),
            implementation_version: "1".to_owned(),
            protocol_kind: spec.protocol_kind.clone(),
            integration_mode: if spec.protocol_kind == HarnessProtocolKind::OpenAiCompatibleShim {
                HarnessIntegrationMode::OpenAiCompatible
            } else {
                HarnessIntegrationMode::Native
            },
            protocol_version: "omnisolo.adapter.v1".to_owned(),
            source_revision: None,
            schema_revision: Some("omnisolo.session.v2".to_owned()),
            capabilities: default_capabilities(),
            worker_image: None,
            worker_pool: Some(spec.harness_id.clone()),
            state_locality: Some("adapter_defined".to_owned()),
            native_extension_namespace: spec.harness_id.clone(),
            metadata: BTreeMap::new(),
        },
    };
    descriptor.protocol_kind = spec.protocol_kind.clone();
    descriptor
}

#[cfg(test)]
mod tests {
    use std::sync::{Mutex, OnceLock};
    use std::time::Duration;

    use chrono::Utc;
    use serde_json::json;
    use uuid::Uuid;

    use super::*;
    use crate::middleware::adapter::{OmniSoloHarnessAdapter, OmniSoloRunConfig};

    fn request() -> HarnessSessionRequest {
        HarnessSessionRequest::new("tenant-harness", Uuid::new_v4(), Uuid::new_v4())
            .with_task(Uuid::new_v4(), "exercise the harness")
            .with_turn(Uuid::new_v4())
    }

    #[tokio::test]
    async fn native_bridge_retains_request_service_references_for_export() {
        use crate::middleware::local_services::{LocalServiceRegistry, LocalServiceScopeContext};
        let mut request = request();
        request.attempt_id = Some(Uuid::new_v4());
        let bundle = LocalServiceRegistry::with_defaults().resolve(
            LocalServiceScopeContext::for_attempt(
                &request.tenant_id, Some("project-a"), Some("workspace-a"),
                request.session_id, request.task_id, request.attempt_id,
            ),
        ).unwrap();
        request.local_service_bundle = Some(bundle.clone());
        let mut bridge = OmniSoloHarnessAdapterBridge::new(native_descriptor());
        bridge.create_session(request).await.unwrap();
        let capsule = bridge.run.as_ref().unwrap()
            .export_capsule("codex", Uuid::new_v4()).unwrap();
        assert_eq!(capsule.manifest.local_service_bindings, bundle.bindings);
        capsule.verify_integrity().unwrap();
    }

    #[tokio::test]
    async fn native_bridge_retains_bindings_admitted_after_session_creation() {
        use crate::middleware::local_services::{LocalServiceRegistry, LocalServiceScopeContext};
        let mut request = request();
        let mut bridge = OmniSoloHarnessAdapterBridge::new(native_descriptor());
        bridge.create_session(request.clone()).await.unwrap();
        request.attempt_id = Some(Uuid::new_v4());
        let bundle = LocalServiceRegistry::with_defaults().resolve(
            LocalServiceScopeContext::for_attempt(
                &request.tenant_id, Some("project-a"), Some("workspace-a"),
                request.session_id, request.task_id, request.attempt_id,
            ),
        ).unwrap();
        request.local_service_bundle = Some(bundle.clone());
        bridge.execute(request.clone(), &request.attempt_id.unwrap().to_string(),
            "keep service references", None).await.unwrap();
        let capsule = bridge.run.as_ref().unwrap()
            .export_capsule("omnisolo", Uuid::new_v4()).unwrap();
        assert_eq!(capsule.manifest.local_service_bindings, bundle.bindings);
        let mut imported = OmniSoloHarnessAdapterBridge::new(native_descriptor());
        request.local_service_bundle = None;
        imported.import_session(request, capsule).await.unwrap();
        assert_eq!(imported.run.as_ref().unwrap().export_capsule("codex", Uuid::new_v4())
            .unwrap().manifest.local_service_bindings, bundle.bindings);
    }

    #[test]
    fn openhands_provider_and_router_errors_remain_typed_at_the_adapter_boundary() {
        let provider = classify_openhands_error(OpenHandsError::Provider(OpenHandsProviderError {
            kind: super::super::openhands::OpenHandsProviderErrorKind::RateLimited,
            status: Some(429),
            retryable: true,
            native: json!({"type":"rate_limit"}),
        }));
        assert!(matches!(
            provider,
            HarnessAdapterError::OpenHandsProvider(OpenHandsProviderError {
                status: Some(429),
                retryable: true,
                ..
            })
        ));

        let router = classify_openhands_error(OpenHandsError::Router(OpenHandsRouterError {
            status: 404,
            native: json!({"detail":"route missing"}),
        }));
        assert!(matches!(
            router,
            HarnessAdapterError::OpenHandsRouter(OpenHandsRouterError { status: 404, .. })
        ));
    }

    fn capsule(target_harness_id: &str, request: &HarnessSessionRequest) -> SessionCapsule {
        OmniSoloHarnessAdapter::start(
            OmniSoloRunConfig::new(&request.tenant_id, "portable objective")
                .with_session_id(request.session_id)
                .with_task_id(request.task_id.unwrap())
                .with_turn(),
        )
        .unwrap()
        .export_capsule(target_harness_id, Uuid::new_v4())
        .unwrap()
    }

    #[test]
    fn portable_import_projection_preserves_the_verified_capsule_and_current_prompt() {
        let request = request();
        let capsule = capsule("opencode", &request);
        let context = portable_capsule_context(&capsule).unwrap();
        let envelope: Value = serde_json::from_str(&context).unwrap();

        assert_eq!(envelope["schema"], "omnisolo.portable_session_context.v1");
        assert_eq!(
            envelope["manifest"],
            serde_json::to_value(&capsule.manifest).unwrap()
        );
        assert_eq!(
            envelope["records"],
            serde_json::to_value(&capsule.records).unwrap()
        );
        assert_eq!(
            envelope["loss_report"],
            serde_json::to_value(&capsule.loss_report).unwrap()
        );
        assert_eq!(envelope["record_digest"], capsule.record_digest);

        let translated = prompt_with_portable_context(&context, "continue from here");
        assert!(translated.contains("<omnisolo_portable_session_context>"));
        assert!(translated.contains(&context));
        assert!(
            translated
                .ends_with("<current_user_request>\ncontinue from here\n</current_user_request>")
        );
    }

    #[test]
    fn process_adapter_applies_pending_portable_context_once_per_native_session() {
        let request = request();
        let capsule = capsule("opencode", &request);
        let expected_context = portable_capsule_context(&capsule).unwrap();
        let mut adapter = ProcessHarnessAdapter::new(
            ProcessHarnessSpec::command("opencode", ["serve"], "opencode")
                .with_protocol(HarnessProtocolKind::OpenCodeHttp),
        );

        adapter.remember_portable_import("native-1", expected_context.clone());
        assert_eq!(
            adapter.prompt_for_native_session("native-2", "plain prompt"),
            "plain prompt"
        );
        let translated = adapter.prompt_for_native_session("native-1", "continue");
        assert_eq!(
            translated,
            prompt_with_portable_context(&expected_context, "continue")
        );
        assert!(adapter.mark_portable_import_applied("native-1"));
        assert!(!adapter.mark_portable_import_applied("native-1"));
        assert_eq!(
            adapter.prompt_for_native_session("native-1", "next"),
            "next"
        );
    }

    fn shell_adapter(script: &str) -> ProcessHarnessAdapter {
        ProcessHarnessAdapter::new(
            ProcessHarnessSpec::command("/bin/sh", ["-c", script], "codex")
                .with_protocol(HarnessProtocolKind::Custom),
        )
    }

    struct DefaultingAdapter {
        descriptor: HarnessDescriptor,
    }

    #[async_trait]
    impl HarnessAdapter for DefaultingAdapter {
        fn descriptor(&self) -> &HarnessDescriptor {
            &self.descriptor
        }

        async fn create_session(
            &mut self,
            request: HarnessSessionRequest,
        ) -> Result<NativeSession, HarnessAdapterError> {
            Ok(NativeSession {
                native_session_id: format!("native:{}", request.session_id),
                native_cursor: None,
            })
        }

        async fn import_session(
            &mut self,
            request: HarnessSessionRequest,
            _capsule: SessionCapsule,
        ) -> Result<NativeSession, HarnessAdapterError> {
            self.create_session(request).await
        }

        async fn resume_session(
            &mut self,
            request: HarnessSessionRequest,
            native_session_id: &str,
        ) -> Result<NativeSession, HarnessAdapterError> {
            Ok(NativeSession {
                native_session_id: native_session_id.to_owned(),
                native_cursor: Some(request.request_id.to_string()),
            })
        }

        async fn execute(
            &mut self,
            _request: HarnessSessionRequest,
            _attempt_id: &str,
            prompt: &str,
            _native_session_id: Option<&str>,
        ) -> Result<HarnessExecution, HarnessAdapterError> {
            Ok(HarnessExecution {
                events: Vec::new(),
                final_text: Some(prompt.to_owned()),
                usage: None,
            })
        }

        async fn checkpoint(
            &mut self,
            _request: HarnessSessionRequest,
            attempt_id: &str,
        ) -> Result<NativeCheckpoint, HarnessAdapterError> {
            Ok(NativeCheckpoint {
                checkpoint_ref: attempt_id.to_owned(),
                native_cursor: None,
            })
        }

        async fn close_session(
            &mut self,
            _request: HarnessSessionRequest,
        ) -> Result<(), HarnessAdapterError> {
            Ok(())
        }
    }

    #[test]
    fn registry_descriptors_preserve_capability_digests_and_all_preset_aliases() {
        let mut registry = HarnessRegistry::with_defaults();
        let descriptor = registry.descriptor("codex").unwrap().clone();
        let snapshot = descriptor.capability_snapshot(Uuid::new_v4(), 4, Utc::now());
        assert_eq!(snapshot.subject_id, "codex");
        assert!(!snapshot.digest.is_empty());
        assert!(snapshot.capabilities.contains("prompt"));

        for harness_id in [
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
            assert!(ExternalHarnessPreset::from_harness_id(harness_id).is_some());
            assert!(registry.preset(harness_id).is_some());
        }
        assert!(ExternalHarnessPreset::from_harness_id("unknown").is_none());
        assert_eq!(registry.descriptors().count(), 12);

        let custom = HarnessDescriptor {
            harness_id: "custom".to_owned(),
            adapter_id: "custom.adapter".to_owned(),
            display_name: "Custom".to_owned(),
            implementation_version: "1".to_owned(),
            protocol_kind: HarnessProtocolKind::Custom,
            integration_mode: HarnessIntegrationMode::Native,
            protocol_version: "custom.v1".to_owned(),
            source_revision: Some("revision".to_owned()),
            schema_revision: None,
            capabilities: BTreeSet::new(),
            worker_image: None,
            worker_pool: None,
            state_locality: None,
            native_extension_namespace: "custom".to_owned(),
            metadata: [("region".to_owned(), "test".to_owned())]
                .into_iter()
                .collect(),
        };
        registry.register_descriptor(custom);
        assert!(matches!(
            registry.build_process_adapter("custom"),
            Err(HarnessRegistryError::MissingProcessPreset(_))
        ));
        assert!(matches!(
            registry.build_adapter("missing"),
            Err(HarnessRegistryError::UnknownHarness(_))
        ));
        assert!(matches!(
            registry.build_process_adapter("missing"),
            Err(HarnessRegistryError::UnknownHarness(_))
        ));
        assert!(registry.build_adapter("omnisolo").is_ok());
        registry.register_preset(ProcessHarnessSpec::command(
            "/bin/sh",
            ["-c", "exit 0"],
            "custom",
        ));
        let custom_adapter = registry.build_adapter("custom").unwrap();
        assert_eq!(custom_adapter.descriptor().harness_id, "custom");
    }

    #[test]
    fn request_builders_and_protocol_mapping_cover_future_harness_ids() {
        let mut request = request();
        let capsule = capsule("codex", &request);
        request = request.with_capsule(capsule).with_transfer_pointers(
            Some(Uuid::new_v4()),
            Some(Uuid::new_v4()),
            Some(Uuid::new_v4()),
            vec![Uuid::new_v4()],
            Some(7),
        );
        request.capability_snapshot_id = Some(Uuid::new_v4());
        assert!(request.capsule.is_some());
        assert_eq!(request.artifact_ids.len(), 1);
        assert_eq!(request.durable_sequence, Some(7));

        assert_eq!(
            protocol_for_harness("omnisolo"),
            HarnessProtocolKind::OmniSolo
        );
        assert_eq!(
            protocol_for_harness("codex"),
            HarnessProtocolKind::CodexAppServer
        );
        assert_eq!(
            protocol_for_harness("opencode"),
            HarnessProtocolKind::OpenCodeHttp
        );
        assert_eq!(
            protocol_for_harness("deepseek"),
            HarnessProtocolKind::DeepSeekJsonRpc
        );
        assert_eq!(protocol_for_harness("pi"), HarnessProtocolKind::PiRpc);
        assert_eq!(protocol_for_harness("kimi"), HarnessProtocolKind::KimiAcp);
        assert_eq!(
            protocol_for_harness("openhands"),
            HarnessProtocolKind::OpenHandsHttp
        );
        assert_eq!(
            protocol_for_harness("openharness"),
            HarnessProtocolKind::OpenHarnessSdk
        );
        assert_eq!(protocol_for_harness("future"), HarnessProtocolKind::Custom);

        let spec = ProcessHarnessSpec::command("harness", ["--stdio"], "future")
            .with_environment("MODE", "test")
            .with_timeout(Duration::from_secs(2));
        assert_eq!(spec.environment.get("MODE"), Some(&"test".to_owned()));
        assert_eq!(spec.request_timeout, Duration::from_secs(2));
        assert_eq!(spec.protocol_kind, HarnessProtocolKind::Custom);
        let custom_adapter = ProcessHarnessAdapter::new(spec.clone());
        assert_eq!(custom_adapter.descriptor().harness_id, "future");
        assert!(format!("{custom_adapter:?}").contains("started: false"));

        let secret_spec = ProcessHarnessSpec::command("codex", ["app-server"], "codex")
            .with_environment("OPENAI_API_KEY", "secret-value");
        let secret_debug = format!("{secret_spec:?}");
        assert!(!secret_debug.contains("secret-value"));
        assert!(secret_debug.contains("<redacted>"));
    }

    #[test]
    fn process_spec_debug_redacts_secret_environment_and_preserves_safe_values() {
        for (key, canary) in [
            ("OPENAI_API_KEY", "api-key-canary"),
            ("SERVICE_ACCESS_TOKEN", "token-canary"),
            ("DATABASE_PASSWORD", "password-canary"),
            ("CLIENT_SECRET", "client-secret-canary"),
            ("AWS_CREDENTIALS", "credentials-canary"),
            ("SESSION_COOKIE", "cookie-canary"),
            ("PROXY_AUTH", "auth-canary"),
            ("SSH_PASSPHRASE", "passphrase-canary"),
        ] {
            let debug = format!(
                "{:?}",
                ProcessHarnessSpec::command("harness", std::iter::empty::<String>(), "future")
                    .with_environment(key, canary)
            );
            assert!(!debug.contains(canary), "{key} leaked through Debug");
            assert!(debug.contains("<redacted>"), "{key} was not redacted");
        }

        for (key, value) in [
            ("PATH", "/usr/local/bin:/usr/bin"),
            ("RUST_LOG", "server_harness=debug"),
            ("OPENAI_MODEL", "gpt-5.6-luna"),
        ] {
            let debug = format!(
                "{:?}",
                ProcessHarnessSpec::command("harness", std::iter::empty::<String>(), "future")
                    .with_environment(key, value)
            );
            assert!(
                debug.contains(value),
                "{key} should remain visible in Debug"
            );
        }
    }

    #[tokio::test]
    async fn default_adapter_hooks_preserve_the_common_lifecycle_contract() {
        let descriptor = external_descriptor(ExternalHarnessPreset::Codex);
        let mut adapter = DefaultingAdapter { descriptor };
        let session_request = request();
        assert_eq!(adapter.descriptor().harness_id, "codex");

        let imported = adapter
            .import_session(session_request.clone(), capsule("codex", &session_request))
            .await
            .unwrap();
        assert!(imported.native_session_id.starts_with("native:"));
        let resumed = adapter
            .resume_session(session_request.clone(), "native-resume")
            .await
            .unwrap();
        assert_eq!(resumed.native_session_id, "native-resume");

        let forked = adapter
            .fork_session(session_request.clone(), Some("source"))
            .await
            .unwrap();
        assert!(forked.native_session_id.starts_with("native:"));
        let started = adapter
            .start_attempt(session_request.clone(), "attempt", "start", None)
            .await
            .unwrap();
        assert_eq!(started.final_text.as_deref(), Some("start"));
        let resumed = adapter
            .resume_attempt(session_request.clone(), "attempt", "resume", None)
            .await
            .unwrap();
        assert_eq!(resumed.final_text.as_deref(), Some("resume"));
        assert!(matches!(
            adapter
                .reconcile_attempt(session_request.clone(), "attempt", None)
                .await,
            Err(HarnessAdapterError::InvalidRequest(_))
        ));
        adapter
            .delete_session(session_request.clone())
            .await
            .unwrap();
        assert!(matches!(
            adapter
                .control_session(session_request.clone(), "snapshot", Value::Null)
                .await,
            Err(HarnessAdapterError::InvalidRequest(_))
        ));
        assert!(matches!(
            adapter
                .control_attempt(session_request.clone(), "attempt", "wait", "", None)
                .await,
            Err(HarnessAdapterError::InvalidRequest(_))
        ));
        assert!(matches!(
            adapter
                .exchange(session_request, "artifact", json!({"ok": true}))
                .await,
            Err(HarnessAdapterError::InvalidRequest(_))
        ));

        adapter.checkpoint(request(), "checkpoint").await.unwrap();
        adapter.close_session(request()).await.unwrap();

        let mut no_channel = shell_adapter("");
        no_channel.terminate().await.unwrap();
    }

    #[tokio::test]
    async fn process_adapter_reuses_an_existing_channel() {
        let mut adapter = shell_adapter("cat");
        adapter.ensure_channel().await.unwrap();
        adapter.ensure_channel().await.unwrap();
        adapter.terminate().await.unwrap();
    }

    fn environment_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[tokio::test]
    async fn process_adapter_does_not_inherit_ambient_parent_environment() {
        let _guard = environment_lock().lock().unwrap();
        unsafe {
            std::env::set_var("UNRELATED_DEPLOYMENT_SECRET", "CANARY-AMBIENT-7KQ9");
        }
        let script = r#"
while IFS= read -r line; do
  id=$(printf '%s' "$line" | sed -n 's/.*"request_id":"\([^"]*\)".*/\1/p')
  ambient=false
  [ -n "${UNRELATED_DEPLOYMENT_SECRET:-}" ] && ambient=true
  explicit=false
  [ -n "${OPENAI_API_KEY:-}" ] && explicit=true
  path=false
  [ -n "${PATH:-}" ] && path=true
  printf '{"request_id":"%s","ok":true,"payload":{"events":[{"event_type":"assistant.text","durable":true,"payload":{"ambient":%s,"explicit":%s,"path":%s}}],"final_text":"ok"}}\n' "$id" "$ambient" "$explicit" "$path"
done
"#;
        let mut adapter = ProcessHarnessAdapter::new(
            ProcessHarnessSpec::command("/bin/sh", ["-c", script], "future")
                .with_protocol(HarnessProtocolKind::Custom)
                .with_environment("OPENAI_API_KEY", "explicit-key"),
        );
        let execution = adapter
            .execute(request(), "attempt-1", "prompt", None)
            .await
            .unwrap();

        unsafe {
            std::env::remove_var("UNRELATED_DEPLOYMENT_SECRET");
        }
        assert_eq!(
            execution.events[0].payload,
            json!({"ambient": false, "explicit": true, "path": true})
        );
        adapter.terminate().await.unwrap();
    }

    #[test]
    fn omnisolo_bridge_exposes_its_descriptor_directly() {
        let bridge = OmniSoloHarnessAdapterBridge::new(native_descriptor());
        assert_eq!(bridge.descriptor().harness_id, "omnisolo");
    }

    #[test]
    fn adapter_error_display_is_defined_for_each_transport_failure_family() {
        let json_error = serde_json::from_str::<Value>("{").unwrap_err();
        let io_error = std::io::Error::other("io");
        let expected = Uuid::new_v4();
        let actual = Uuid::new_v4();
        for error in [
            HarnessAdapterError::InvalidRequest("bad".to_owned()),
            HarnessAdapterError::Spawn(std::io::Error::other("spawn")),
            HarnessAdapterError::Io(io_error),
            HarnessAdapterError::Json(json_error),
            HarnessAdapterError::Timeout,
            HarnessAdapterError::ProcessExited,
            HarnessAdapterError::RequestMismatch { expected, actual },
            HarnessAdapterError::Remote("remote".to_owned()),
            HarnessAdapterError::Capsule("capsule".to_owned()),
            HarnessAdapterError::InvalidResponse("response".to_owned()),
        ] {
            assert!(!error.to_string().is_empty());
        }
        assert!(
            !HarnessRegistryError::UnknownHarness("missing".to_owned())
                .to_string()
                .is_empty()
        );
        assert!(
            !HarnessRegistryError::MissingProcessPreset("custom".to_owned())
                .to_string()
                .is_empty()
        );
    }

    #[test]
    fn process_pipe_errors_classify_exit_and_other_io_separately() {
        assert!(matches!(
            classify_process_io_error(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "closed harness stdin"
            )),
            HarnessAdapterError::ProcessExited
        ));
        assert!(matches!(
            classify_process_io_error(std::io::Error::other("unavailable harness pipe")),
            HarnessAdapterError::Io(_)
        ));
    }

    #[tokio::test]
    async fn process_adapter_rejects_invalid_process_and_response_behaviors() {
        {
            let request = request();

            let mut empty = shell_adapter("");
            empty.spec.executable.clear();
            assert!(matches!(
                empty.create_session(request.clone()).await,
                Err(HarnessAdapterError::InvalidRequest(_))
            ));

            let mut spawn = ProcessHarnessAdapter::new(
                ProcessHarnessSpec::command(
                    "/path/does/not/exist",
                    std::iter::empty::<String>(),
                    "codex",
                )
                .with_protocol(HarnessProtocolKind::Custom),
            );
            assert!(matches!(
                spawn.create_session(request.clone()).await,
                Err(HarnessAdapterError::Spawn(_))
            ));

            let mut exited = shell_adapter("exit 0");
            assert!(matches!(
                exited.create_session(request.clone()).await,
                Err(HarnessAdapterError::ProcessExited)
            ));

            let mut malformed = shell_adapter("read line; printf 'not-json\\n'");
            assert!(matches!(
                malformed.create_session(request.clone()).await,
                Err(HarnessAdapterError::Json(_))
            ));

            let mut mismatch = shell_adapter(
                "read line; printf '{\"request_id\":\"00000000-0000-0000-0000-000000000000\",\"ok\":true,\"payload\":{}}\\n'",
            );
            assert!(matches!(
                mismatch.create_session(request.clone()).await,
                Err(HarnessAdapterError::RequestMismatch { .. })
            ));

            let mut remote = shell_adapter(
                r#"read line; id=$(printf '%s' "$line" | sed -n 's/.*"request_id":"\([^"]*\)".*/\1/p'); printf '{"request_id":"%s","ok":false}\n' "$id""#,
            );
            assert!(matches!(
                remote.create_session(request.clone()).await,
                Err(HarnessAdapterError::Remote(message)) if message == "unknown harness error"
            ));

            let mut invalid_native = shell_adapter(
                r#"read line; id=$(printf '%s' "$line" | sed -n 's/.*"request_id":"\([^"]*\)".*/\1/p'); printf '{"request_id":"%s","ok":true,"payload":{"native_session_id":""}}\n' "$id""#,
            );
            assert!(matches!(
                invalid_native.create_session(request.clone()).await,
                Err(HarnessAdapterError::InvalidResponse(_))
            ));

            let mut invalid_execution = shell_adapter(
                r#"read line; id=$(printf '%s' "$line" | sed -n 's/.*"request_id":"\([^"]*\)".*/\1/p'); printf '{"request_id":"%s","ok":true,"payload":{}}\n' "$id""#,
            );
            assert!(matches!(
                invalid_execution
                    .execute(request.clone(), "attempt", "prompt", None)
                    .await,
                Err(HarnessAdapterError::Json(_))
            ));

            let mut timeout_adapter = ProcessHarnessAdapter::new(
                ProcessHarnessSpec::command("/bin/sh", ["-c", "sleep 1"], "codex")
                    .with_protocol(HarnessProtocolKind::Custom)
                    .with_timeout(Duration::from_millis(10)),
            );
            assert!(matches!(
                timeout_adapter.create_session(request.clone()).await,
                Err(HarnessAdapterError::Timeout)
            ));

            let mut valid = shell_adapter(
                r#"while IFS= read -r line; do id=$(printf '%s' "$line" | sed -n 's/.*"request_id":"\([^"]*\)".*/\1/p'); printf '{"request_id":"%s","ok":true,"payload":{"native_session_id":"native","native_cursor":"cursor"}}\n' "$id"; done"#,
            );
            assert!(matches!(
                valid.resume_session(request.clone(), "").await,
                Err(HarnessAdapterError::InvalidRequest(_))
            ));
            assert!(matches!(
                valid.execute(request.clone(), "", "prompt", None).await,
                Err(HarnessAdapterError::InvalidRequest(_))
            ));
            assert!(matches!(
                valid.execute(request.clone(), "attempt", "", None).await,
                Err(HarnessAdapterError::InvalidRequest(_))
            ));
            assert!(matches!(
                valid.checkpoint(request.clone(), "").await,
                Err(HarnessAdapterError::InvalidRequest(_))
            ));

            let wrong_target = capsule("opencode", &request);
            assert!(matches!(
                valid.import_session(request.clone(), wrong_target).await,
                Err(HarnessAdapterError::Capsule(_))
            ));

            let mut corrupt = capsule("codex", &request);
            corrupt.manifest_digest = "bad".to_owned();
            assert!(matches!(
                valid.import_session(request, corrupt).await,
                Err(HarnessAdapterError::Capsule(_))
            ));
        }

        let request = request();
        let mut all_operations = shell_adapter(
            r#"while IFS= read -r line; do id=$(printf '%s' "$line" | sed -n 's/.*"request_id":"\([^"]*\)".*/\1/p'); op=$(printf '%s' "$line" | sed -n 's/.*"operation":"\([^"]*\)".*/\1/p'); case "$op" in create_session|fork_session|import_session|resume_session|snapshot|quiesce|cancel|close_session|delete_session) payload='{"native_session_id":"native","native_cursor":"cursor"}' ;; checkpoint) payload='{"checkpoint_ref":"checkpoint","native_cursor":"cursor"}' ;; exchange) payload='{"ok":true}' ;; *) payload='{"events":[],"final_text":"done","usage":{"tokens":1}}' ;; esac; printf '{"request_id":"%s","ok":true,"payload":%s}\n' "$id" "$payload"; done"#,
        );
        let session = all_operations
            .create_session(request.clone())
            .await
            .unwrap();
        assert_eq!(session.native_session_id, "native");
        all_operations
            .fork_session(request.clone(), Some("native"))
            .await
            .unwrap();
        all_operations
            .import_session(request.clone(), capsule("codex", &request))
            .await
            .unwrap();
        all_operations
            .resume_session(request.clone(), "native")
            .await
            .unwrap();
        all_operations
            .execute(request.clone(), "attempt", "prompt", Some("native"))
            .await
            .unwrap();
        all_operations
            .start_attempt(request.clone(), "attempt", "prompt", Some("native"))
            .await
            .unwrap();
        all_operations
            .resume_attempt(request.clone(), "attempt", "prompt", Some("native"))
            .await
            .unwrap();
        all_operations
            .checkpoint(request.clone(), "attempt")
            .await
            .unwrap();
        all_operations
            .control_attempt(request.clone(), "attempt", "wait", "", Some("native"))
            .await
            .unwrap();
        all_operations
            .control_session(request.clone(), "snapshot", json!({"snapshot": true}))
            .await
            .unwrap();
        all_operations
            .exchange(request.clone(), "artifact", json!({"artifact": true}))
            .await
            .unwrap();
        all_operations.close_session(request.clone()).await.unwrap();

        let mut delete = shell_adapter(
            r#"while IFS= read -r line; do id=$(printf '%s' "$line" | sed -n 's/.*"request_id":"\([^"]*\)".*/\1/p'); printf '{"request_id":"%s","ok":true,"payload":{}}\n' "$id"; done"#,
        );
        delete.delete_session(request).await.unwrap();
    }

    #[tokio::test]
    async fn omnisolo_bridge_rejects_invalid_reuse_and_exposes_checkpoint_lifecycle() {
        let descriptor = native_descriptor();
        let request = request();
        let mut bridge = OmniSoloHarnessAdapterBridge::new(descriptor);
        assert!(bridge.imported_capsule().is_none());
        assert!(bridge.execution_since(0).events.is_empty());
        assert!(format!("{bridge:?}").contains("started: false"));

        let taskless_request =
            HarnessSessionRequest::new("tenant-taskless", Uuid::new_v4(), Uuid::new_v4());
        let mut taskless_bridge = OmniSoloHarnessAdapterBridge::new(native_descriptor());
        taskless_bridge
            .create_session(taskless_request.clone())
            .await
            .unwrap();
        taskless_bridge
            .close_session(taskless_request)
            .await
            .unwrap();

        assert!(matches!(
            bridge.checkpoint(request.clone(), "attempt").await,
            Err(HarnessAdapterError::Remote(_))
        ));
        assert!(matches!(
            bridge.execute(request.clone(), "", "prompt", None).await,
            Err(HarnessAdapterError::InvalidRequest(_))
        ));
        assert!(matches!(
            bridge.execute(request.clone(), "attempt", "", None).await,
            Err(HarnessAdapterError::InvalidRequest(_))
        ));

        let mut terminal_bridge = OmniSoloHarnessAdapterBridge::new(native_descriptor());
        terminal_bridge
            .create_session(request.clone())
            .await
            .unwrap();
        terminal_bridge
            .run
            .as_mut()
            .unwrap()
            .record(OmniSoloEvent::TaskComplete {
                content: "done".to_owned(),
            })
            .unwrap();
        assert!(matches!(
            terminal_bridge
                .execute(request.clone(), "attempt", "prompt", None)
                .await,
            Err(HarnessAdapterError::Remote(_))
        ));

        bridge.create_session(request.clone()).await.unwrap();
        assert!(matches!(
            bridge.create_session(request.clone()).await,
            Err(HarnessAdapterError::Remote(_))
        ));
        bridge
            .resume_session(request.clone(), &format!("omnisolo:{}", request.session_id))
            .await
            .unwrap();
        assert!(matches!(
            bridge
                .resume_session(request.clone(), "wrong-session")
                .await,
            Err(HarnessAdapterError::InvalidRequest(_))
        ));
        let checkpoint = bridge.checkpoint(request.clone(), "attempt").await.unwrap();
        assert!(checkpoint.checkpoint_ref.starts_with("omnisolo-durable:"));
        bridge
            .control_session(request.clone(), "snapshot", Value::Null)
            .await
            .unwrap();
        bridge
            .control_attempt(request.clone(), "attempt", "steer", "steer", None)
            .await
            .unwrap();
        bridge
            .control_attempt(request.clone(), "attempt", "wait", "", None)
            .await
            .unwrap();
        bridge
            .control_attempt(request.clone(), "attempt", "reconcile", "", None)
            .await
            .unwrap();
        assert!(matches!(
            bridge
                .control_session(request.clone(), "unknown", Value::Null)
                .await,
            Err(HarnessAdapterError::InvalidRequest(_))
        ));
        assert!(matches!(
            bridge
                .control_attempt(request.clone(), "attempt", "unknown", "", None)
                .await,
            Err(HarnessAdapterError::InvalidRequest(_))
        ));
        assert_eq!(
            bridge
                .exchange(request.clone(), "artifact", json!({"ok": true}))
                .await
                .unwrap(),
            json!({"ok": true})
        );
        bridge.close_session(request.clone()).await.unwrap();
        bridge
            .resume_session(request.clone(), &format!("omnisolo:{}", request.session_id))
            .await
            .unwrap();
        bridge.close_session(request.clone()).await.unwrap();

        let mut imported_request = request.clone();
        imported_request.objective.clear();
        let mut invalid_capsule = capsule("omnisolo", &imported_request);
        invalid_capsule.manifest_digest = "invalid".to_owned();
        assert!(matches!(
            bridge
                .import_session(imported_request.clone(), invalid_capsule)
                .await,
            Err(HarnessAdapterError::Capsule(_))
        ));
        let imported_capsule = capsule("omnisolo", &imported_request);
        bridge
            .import_session(imported_request.clone(), imported_capsule)
            .await
            .unwrap();
        assert!(bridge.imported_capsule().is_some());
        bridge
            .fork_session(imported_request.clone(), None)
            .await
            .unwrap();
        bridge
            .control_session(imported_request.clone(), "quiesce", Value::Null)
            .await
            .unwrap();
        bridge
            .control_session(imported_request.clone(), "cancel", Value::Null)
            .await
            .unwrap();
        bridge
            .resume_session(
                imported_request.clone(),
                &format!("omnisolo:{}", imported_request.session_id),
            )
            .await
            .unwrap();
        bridge.delete_session(imported_request).await.unwrap();

        let value = json!({"native_session_id":"native","native_cursor":null});
        assert_eq!(
            parse_native_session(value).unwrap().native_session_id,
            "native"
        );
        assert!(matches!(
            parse_native_session(json!({"native_session_id":""})),
            Err(HarnessAdapterError::InvalidResponse(_))
        ));
    }
}
