use std::collections::{BTreeMap, BTreeSet};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use uuid::Uuid;

use super::lifecycle::{InteractionState, ProcessState, ToolCallState};
use super::local_services::LocalServiceBundle;

pub type JsonMap = BTreeMap<String, Value>;
pub type StringMap = BTreeMap<String, String>;

fn identifier_tokens(identifier: &str) -> Vec<String> {
    let characters = identifier.chars().collect::<Vec<_>>();
    let mut tokens = Vec::new();
    let mut token = String::new();

    for (index, character) in characters.iter().copied().enumerate() {
        if !character.is_ascii_alphanumeric() {
            if !token.is_empty() {
                tokens.push(std::mem::take(&mut token));
            }
            continue;
        }

        let previous = index.checked_sub(1).and_then(|index| characters.get(index));
        let next = characters.get(index + 1);
        let camel_boundary = !token.is_empty()
            && character.is_ascii_uppercase()
            && (previous.is_some_and(|value| value.is_ascii_lowercase() || value.is_ascii_digit())
                || (previous.is_some_and(|value| value.is_ascii_uppercase())
                    && next.is_some_and(|value| value.is_ascii_lowercase())));
        if camel_boundary {
            tokens.push(std::mem::take(&mut token));
        }
        token.push(character.to_ascii_lowercase());
    }

    if !token.is_empty() {
        tokens.push(token);
    }
    tokens
}

fn contains_adjacent_tokens(tokens: &[String], first: &str, second: &str) -> bool {
    tokens
        .windows(2)
        .any(|pair| pair[0] == first && pair[1] == second)
}

pub(crate) fn sensitive_json_key(key: &str) -> bool {
    let tokens = identifier_tokens(key);
    (tokens.len() == 1
        && matches!(
            tokens[0].as_str(),
            "apikey"
                | "token"
                | "secret"
                | "credential"
                | "credentials"
                | "authorization"
                | "password"
                | "cookie"
        ))
        || tokens.iter().any(|token| token == "apikey")
        || contains_adjacent_tokens(&tokens, "api", "key")
        || contains_adjacent_tokens(&tokens, "access", "token")
        || contains_adjacent_tokens(&tokens, "refresh", "token")
        || contains_adjacent_tokens(&tokens, "client", "secret")
        || contains_adjacent_tokens(&tokens, "credential", "ref")
        || contains_adjacent_tokens(&tokens, "authenticated", "principal")
        || contains_adjacent_tokens(&tokens, "authorization", "header")
        || contains_adjacent_tokens(&tokens, "authorization", "token")
        || contains_adjacent_tokens(&tokens, "authorization", "value")
        || contains_adjacent_tokens(&tokens, "authorization", "credential")
        || contains_adjacent_tokens(&tokens, "database", "password")
        || contains_adjacent_tokens(&tokens, "db", "password")
        || contains_adjacent_tokens(&tokens, "user", "password")
        || contains_adjacent_tokens(&tokens, "account", "password")
        || contains_adjacent_tokens(&tokens, "admin", "password")
        || contains_adjacent_tokens(&tokens, "password", "hash")
        || contains_adjacent_tokens(&tokens, "password", "value")
        || contains_adjacent_tokens(&tokens, "session", "cookie")
        || contains_adjacent_tokens(&tokens, "auth", "cookie")
        || contains_adjacent_tokens(&tokens, "cookie", "header")
        || contains_adjacent_tokens(&tokens, "cookie", "value")
}

fn contains_sensitive_assignment(text: &str) -> bool {
    text.split(|character: char| {
        character.is_ascii_whitespace()
            || matches!(character, '?' | '&' | ';' | ',' | '#' | '\n' | '\r')
    })
    .filter(|part| !part.is_empty())
    .any(|part| {
        part.find(['=', ':'])
            .map(|separator| sensitive_json_key(part[..separator].trim()))
            .unwrap_or(false)
    })
}

fn url_has_password_userinfo(text: &str) -> bool {
    let Some(scheme_end) = text.find("://") else {
        return false;
    };
    let authority = &text[scheme_end + 3..];
    let authority_end = authority.find(['/', '?', '#']).unwrap_or(authority.len());
    let authority = &authority[..authority_end];
    let Some((userinfo, _)) = authority.rsplit_once('@') else {
        return false;
    };
    userinfo
        .split_once(':')
        .is_some_and(|(_, password)| !password.is_empty())
}

pub(crate) fn sensitive_json_text(text: &str) -> bool {
    let trimmed = text.trim_start();
    let normalized = trimmed.to_ascii_lowercase();
    if normalized.starts_with("bearer ")
        || (normalized.contains("-----begin") && normalized.contains("private key-----"))
    {
        return true;
    }

    url_has_password_userinfo(trimmed) || contains_sensitive_assignment(trimmed)
}

pub(crate) fn sanitize_credential_value(value: &Value) -> Value {
    match value {
        Value::String(text) if sensitive_json_text(text) => Value::String("[REDACTED]".to_owned()),
        Value::Array(values) => {
            Value::Array(values.iter().map(sanitize_credential_value).collect())
        }
        Value::Object(values) => Value::Object(
            values
                .iter()
                .filter(|(key, _)| !sensitive_json_key(key))
                .map(|(key, value)| (key.clone(), sanitize_credential_value(value)))
                .collect(),
        ),
        _ => value.clone(),
    }
}

pub(crate) fn sanitize_credential_map(metadata: &JsonMap) -> JsonMap {
    metadata
        .iter()
        .filter(|(key, _)| !sensitive_json_key(key))
        .map(|(key, value)| (key.clone(), sanitize_credential_value(value)))
        .collect()
}

fn serialize_resolved_model_metadata<S>(
    metadata: &JsonMap,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    sanitize_credential_map(metadata).serialize(serializer)
}

fn deserialize_resolved_model_metadata<'de, D>(deserializer: D) -> Result<JsonMap, D::Error>
where
    D: Deserializer<'de>,
{
    JsonMap::deserialize(deserializer).map(|metadata| sanitize_credential_map(&metadata))
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorType {
    Human,
    Agent,
    Subagent,
    Service,
    Harness,
    System,
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ActorDescriptor {
    pub actor_id: String,
    pub tenant_id: Option<String>,
    pub actor_type: ActorType,
    pub display_name: Option<String>,
    pub authenticated_principal_ref: Option<String>,
    pub delegation_parent_id: Option<String>,
    pub agent_profile: Option<String>,
    pub agent_version: Option<String>,
    pub native_aliases: BTreeSet<String>,
    pub provider: Option<String>,
    pub external_subject: Option<String>,
    pub metadata: StringMap,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HistoricalActor {
    pub actor_id: String,
    pub historical_role: String,
    pub display_name: Option<String>,
    pub delegation_lineage: Vec<String>,
}

impl From<&ActorDescriptor> for HistoricalActor {
    fn from(actor: &ActorDescriptor) -> Self {
        let mut delegation_lineage = Vec::new();
        if let Some(parent_id) = &actor.delegation_parent_id {
            delegation_lineage.push(parent_id.clone());
        }

        Self {
            actor_id: actor.actor_id.clone(),
            historical_role: actor.actor_type.to_string(),
            display_name: actor.display_name.clone(),
            delegation_lineage,
        }
    }
}

impl HistoricalActor {
    pub fn has_authority_context(&self) -> bool {
        false
    }
}

impl std::fmt::Display for ActorType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Human => "human",
            Self::Agent => "agent",
            Self::Subagent => "subagent",
            Self::Service => "service",
            Self::Harness => "harness",
            Self::System => "system",
            Self::Unknown => "unknown",
        };
        formatter.write_str(value)
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionState {
    #[default]
    Open,
    HandingOff,
    Archived,
    Deleting,
    Deleted,
    Error,
    Unknown,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskState {
    #[default]
    Queued,
    Running,
    HandingOff,
    WaitingInput,
    Paused,
    Completed,
    Failed,
    Cancelled,
    Lost,
    Unknown,
}

impl TaskState {
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Failed | Self::Cancelled | Self::Lost
        )
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TurnState {
    Admitted,
    #[default]
    Queued,
    Running,
    WaitingInput,
    Completed,
    Interrupted,
    Failed,
    Cancelled,
    Unknown,
}

impl TurnState {
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Interrupted | Self::Failed | Self::Cancelled
        )
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AttemptKind {
    #[default]
    Interactive,
    Background,
    Recovery,
    Handoff,
    Subagent,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AttemptState {
    #[default]
    Pending,
    Leased,
    Starting,
    Running,
    Quiescing,
    Checkpointing,
    Succeeded,
    Failed,
    Cancelled,
    Lost,
    Fenced,
    Unknown,
}

impl AttemptState {
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Succeeded | Self::Failed | Self::Cancelled | Self::Lost | Self::Fenced
        )
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Session {
    pub session_id: Uuid,
    pub tenant_id: String,
    pub project_id: Option<String>,
    pub workspace_id: Option<String>,
    pub title: Option<String>,
    pub labels: BTreeSet<String>,
    pub tags: StringMap,
    pub state: SessionState,
    pub state_version: i64,
    pub parent_session_id: Option<Uuid>,
    pub root_session_id: Uuid,
    pub fork_source_event_id: Option<Uuid>,
    pub active_task_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_active_at: Option<DateTime<Utc>>,
    pub retention_class: Option<String>,
    pub data_classification: Option<String>,
    pub extensions: JsonMap,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskKind {
    #[default]
    UserObjective,
    Background,
    Subagent,
    Monitor,
    DetachedCommand,
    Handoff,
    Other,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Task {
    pub task_id: Uuid,
    pub session_id: Uuid,
    pub parent_task_id: Option<Uuid>,
    pub parent_attempt_id: Option<Uuid>,
    pub kind: TaskKind,
    pub objective: String,
    pub state: TaskState,
    pub state_version: i64,
    pub dependency_task_ids: Vec<Uuid>,
    pub owner_actor: Option<ActorDescriptor>,
    pub input_message_ids: Vec<Uuid>,
    pub output_message_ids: Vec<Uuid>,
    pub artifact_ids: Vec<Uuid>,
    pub terminal_result: Option<TaskResult>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub extensions: JsonMap,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Turn {
    pub turn_id: Uuid,
    pub task_id: Uuid,
    pub session_id: Uuid,
    pub sequence: i64,
    pub state: TurnState,
    pub state_version: i64,
    pub input_message_id: Option<Uuid>,
    pub output_message_ids: Vec<Uuid>,
    pub stop_reason: Option<String>,
    pub error_code: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub extensions: JsonMap,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Attempt {
    pub attempt_id: Uuid,
    pub session_id: Uuid,
    pub task_id: Uuid,
    pub turn_id: Option<Uuid>,
    pub parent_attempt_id: Option<Uuid>,
    pub kind: AttemptKind,
    pub state: AttemptState,
    pub state_version: i64,
    pub actor: Option<ActorDescriptor>,
    pub harness_id: String,
    pub model_runtime_id: Option<String>,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub runtime_config_snapshot_id: Option<Uuid>,
    pub model_binding_id: Option<Uuid>,
    pub worker_id: Option<String>,
    pub extensions: JsonMap,
    pub metadata: JsonMap,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TaskResult {
    pub status: String,
    pub summary: Option<String>,
    pub output_message_ids: Vec<Uuid>,
    pub artifact_ids: Vec<Uuid>,
    pub error_code: Option<String>,
    pub completed_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactKind {
    #[default]
    File,
    Image,
    Audio,
    Video,
    Archive,
    Dataset,
    Patch,
    Log,
    Other,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ArtifactRef {
    pub artifact_id: Uuid,
    pub kind: ArtifactKind,
    pub name: String,
    pub media_type: Option<String>,
    pub byte_length: u64,
    pub sha256: String,
    pub uri: Option<String>,
    pub metadata: StringMap,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
pub struct ContentAnnotation {
    pub annotation_type: String,
    pub start: Option<u64>,
    pub end: Option<u64>,
    pub data: JsonMap,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    Text {
        text: String,
        annotations: Vec<ContentAnnotation>,
    },
    ReasoningSummary {
        text: String,
        annotations: Vec<ContentAnnotation>,
    },
    Image {
        artifact: ArtifactRef,
    },
    Audio {
        artifact: ArtifactRef,
    },
    Video {
        artifact: ArtifactRef,
    },
    File {
        artifact: ArtifactRef,
    },
    ResourceLink {
        uri: String,
        name: Option<String>,
        media_type: Option<String>,
    },
    EmbeddedResource {
        media_type: Option<String>,
        data: String,
    },
    StructuredJson {
        value: Value,
        schema_ref: Option<String>,
    },
    ToolCallRef {
        tool_call_id: Uuid,
    },
    ToolResultRef {
        tool_call_id: Uuid,
    },
    Artifact {
        artifact: ArtifactRef,
    },
    WorkspacePath {
        snapshot_id: Option<Uuid>,
        path: String,
    },
    Symbol {
        path: String,
        name: String,
        kind: Option<String>,
    },
    Range {
        path: String,
        start_line: u64,
        start_column: Option<u64>,
        end_line: u64,
        end_column: Option<u64>,
    },
    ExternalTaskRef {
        task_id: Uuid,
    },
    Citation {
        uri: String,
        title: Option<String>,
        locator: Option<String>,
    },
    Native {
        namespace: String,
        kind: String,
        payload: Value,
    },
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    System,
    Developer,
    User,
    Assistant,
    Tool,
    #[default]
    Other,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageStatus {
    #[default]
    Settled,
    Streaming,
    Redacted,
    Invalid,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Message {
    pub message_id: Uuid,
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub turn_id: Option<Uuid>,
    pub role: MessageRole,
    pub author: Option<ActorDescriptor>,
    pub origin: Option<String>,
    pub phase: Option<String>,
    pub parent_message_id: Option<Uuid>,
    pub correlation_id: Option<Uuid>,
    pub status: MessageStatus,
    pub content: Vec<ContentPart>,
    pub visible_to_user: bool,
    pub redaction_state: Option<String>,
    pub created_at: DateTime<Utc>,
    pub settled_at: Option<DateTime<Utc>>,
    pub native_provenance: JsonMap,
    pub extensions: JsonMap,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventDurability {
    #[default]
    Durable,
    Transient,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayRequirement {
    #[default]
    Required,
    Ignorable,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EventEnvelope {
    pub event_id: Uuid,
    pub tenant_id: String,
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub turn_id: Option<Uuid>,
    pub source_attempt_id: Option<Uuid>,
    pub ingest_attempt_id: Option<Uuid>,
    pub actor_id: Option<String>,
    pub worker_id: Option<String>,
    pub harness_id: Option<String>,
    pub binding_id: Option<Uuid>,
    pub binding_generation: Option<i64>,
    pub lease_id: Option<Uuid>,
    pub lease_generation: Option<i64>,
    pub fencing_token: Option<String>,
    pub durable_sequence: Option<i64>,
    pub delivery_stream_id: Option<String>,
    pub delivery_sequence: Option<i64>,
    pub aggregate_id: Option<Uuid>,
    pub aggregate_sequence: Option<i64>,
    pub branch_id: Option<Uuid>,
    pub parent_event_ids: Vec<Uuid>,
    pub event_type: String,
    pub payload_schema: String,
    pub payload_version: u32,
    pub occurred_at: DateTime<Utc>,
    pub ingested_at: DateTime<Utc>,
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
    pub idempotency_key: Option<String>,
    pub durability: EventDurability,
    pub replay_requirement: ReplayRequirement,
    pub visibility: Option<String>,
    pub data_classification: Option<String>,
    pub native_provenance: JsonMap,
    pub payload: Value,
    pub extensions: JsonMap,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelProvider {
    Managed,
    OpenAiCompatible,
    #[default]
    SelfHosted,
    Custom,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningEffort {
    None,
    Minimal,
    Low,
    Medium,
    High,
    Max,
    #[default]
    #[serde(other)]
    Custom,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelApiDialect {
    OpenAiResponses,
    OpenAiChatCompletions,
    AnthropicMessages,
    GoogleGenerateContent,
    Ollama,
    #[default]
    #[serde(other)]
    Custom,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ResolvedModelSelection {
    pub provider_route: String,
    pub model_id: String,
    pub reasoning_effort: Option<ReasoningEffort>,
    pub api_dialect: ModelApiDialect,
    pub context_window: Option<u64>,
    pub max_output_tokens: Option<u64>,
    pub capabilities: BTreeSet<String>,
    pub binding_revision: String,
    pub binding_digest: String,
    #[serde(
        default,
        serialize_with = "serialize_resolved_model_metadata",
        deserialize_with = "deserialize_resolved_model_metadata"
    )]
    pub metadata: JsonMap,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ModelDescriptor {
    pub model_id: String,
    pub family: Option<String>,
    pub provider: ModelProvider,
    pub revision: Option<String>,
    pub weight_digest: Option<String>,
    pub tokenizer_digest: Option<String>,
    pub context_window: Option<u64>,
    pub max_output_tokens: Option<u64>,
    pub modalities: BTreeSet<String>,
    pub capabilities: BTreeSet<String>,
    pub quantization: Option<String>,
    pub license: Option<String>,
    pub provenance: Option<String>,
    pub metadata: JsonMap,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelRuntimeKind {
    #[default]
    ManagedApi,
    OpenAiCompatibleEndpoint,
    LocalProcess,
    KubernetesService,
    RemoteCluster,
    Custom,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ServingEngine {
    Vllm,
    Tgi,
    Sglang,
    Ollama,
    LlamaCpp,
    #[default]
    Custom,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RuntimePlacement {
    pub cluster: Option<String>,
    pub namespace: Option<String>,
    pub region: Option<String>,
    pub tenant: Option<String>,
    pub data_locality: Option<String>,
    pub network_policy: Option<String>,
    pub node_selector: StringMap,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
pub struct RuntimeCapacity {
    pub max_concurrent_requests: u32,
    pub max_batch_tokens: Option<u64>,
    pub accelerator_family: Option<String>,
    pub accelerator_count: Option<u32>,
    pub accelerator_memory_mb: Option<u64>,
    pub tensor_parallelism: Option<u32>,
    pub pipeline_parallelism: Option<u32>,
    pub quantization: Option<String>,
    pub batching: bool,
    pub context_cache: bool,
    pub speculative_decoding: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
pub struct RuntimeAutoscaling {
    pub min_replicas: u32,
    pub max_replicas: u32,
    pub target_utilization_percent: u8,
    pub scale_to_zero: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeReadiness {
    #[default]
    Unknown,
    Ready,
    NotReady,
    Draining,
    Unhealthy,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ModelRuntimeDescriptor {
    pub runtime_id: String,
    pub model: ModelDescriptor,
    pub kind: ModelRuntimeKind,
    pub engine: Option<ServingEngine>,
    pub api_dialect: Option<String>,
    pub api_version: Option<String>,
    pub endpoint: Option<String>,
    pub capability_discovery_endpoint: Option<String>,
    pub credential_ref: Option<String>,
    pub image: Option<String>,
    pub supported_revisions: BTreeSet<String>,
    pub adapter_refs: BTreeSet<String>,
    pub gpu_profile: Option<String>,
    pub lora_adapter: Option<String>,
    pub placement: Option<RuntimePlacement>,
    pub capacity: RuntimeCapacity,
    pub autoscaling: Option<RuntimeAutoscaling>,
    pub readiness: RuntimeReadiness,
    pub health_endpoint: Option<String>,
    pub draining: bool,
    pub metadata: JsonMap,
}

impl Default for ModelRuntimeDescriptor {
    fn default() -> Self {
        Self {
            runtime_id: String::new(),
            model: ModelDescriptor::default(),
            kind: ModelRuntimeKind::default(),
            engine: None,
            api_dialect: None,
            api_version: None,
            endpoint: None,
            capability_discovery_endpoint: None,
            credential_ref: None,
            image: None,
            supported_revisions: BTreeSet::new(),
            adapter_refs: BTreeSet::new(),
            gpu_profile: None,
            lora_adapter: None,
            placement: None,
            capacity: RuntimeCapacity::default(),
            autoscaling: None,
            readiness: RuntimeReadiness::default(),
            health_endpoint: None,
            draining: false,
            metadata: JsonMap::new(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelBindingStatus {
    #[default]
    Selected,
    Active,
    Draining,
    Released,
    Failed,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ModelBinding {
    pub model_binding_id: Uuid,
    pub attempt_id: Uuid,
    pub model: ModelDescriptor,
    pub runtime: ModelRuntimeDescriptor,
    pub realized_capabilities: BTreeSet<String>,
    pub serving_revision: Option<String>,
    pub routing_reason: Option<String>,
    pub usage_accounting_source: Option<String>,
    pub status: ModelBindingStatus,
    pub created_at: DateTime<Utc>,
    pub released_at: Option<DateTime<Utc>>,
    pub extensions: JsonMap,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BindingScope {
    #[default]
    Session,
    Task,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BindingAccessMode {
    ReadOnly,
    #[default]
    ReadWrite,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BindingState {
    #[default]
    Creating,
    Inactive,
    Active,
    Quiescing,
    Fenced,
    Closed,
    Error,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SessionBinding {
    pub binding_id: Uuid,
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub harness_id: String,
    pub scope: BindingScope,
    pub owner_id: Uuid,
    pub workspace_mutation_scope_id: String,
    pub access_mode: BindingAccessMode,
    pub native_session_id: Option<String>,
    pub state: BindingState,
    pub generation: i64,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub capability_snapshot_id: Option<Uuid>,
    pub adapter_config_digest: Option<String>,
    pub last_imported_native_cursor: Option<String>,
    pub last_exported_durable_sequence: Option<i64>,
    pub native_checkpoint_ref: Option<String>,
    pub worker_pool: Option<String>,
    pub state_locality: Option<String>,
    pub exact_resume_eligible: bool,
    pub invalidation_reason: Option<String>,
    pub native_record_digest: Option<String>,
    pub extensions: JsonMap,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ActorBinding {
    pub actor_binding_id: Uuid,
    pub session_id: Uuid,
    pub binding_id: Uuid,
    pub generation: i64,
    pub canonical_actor_id: String,
    pub native_actor_id: String,
    pub actor_type: ActorType,
    pub captured_at: DateTime<Utc>,
    pub metadata: JsonMap,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolSource {
    #[default]
    BuiltIn,
    Mcp,
    Skill,
    Plugin,
    ClientDynamic,
    HarnessNative,
    RemoteAgent,
    Custom,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ToolDefinitionSnapshot {
    pub snapshot_id: Uuid,
    pub qualified_name: String,
    pub description: Option<String>,
    pub source: ToolSource,
    pub input_schema: Value,
    pub output_schema: Option<Value>,
    pub annotations: JsonMap,
    pub version: String,
    pub digest: String,
    pub captured_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolResultState {
    #[default]
    Completed,
    Failed,
    Cancelled,
    Uncertain,
    Unknown,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ToolCall {
    pub tool_call_id: Uuid,
    pub session_id: Uuid,
    pub task_id: Uuid,
    pub turn_id: Option<Uuid>,
    pub attempt_id: Uuid,
    pub definition_snapshot_id: Uuid,
    pub raw_input: Value,
    pub parsed_input: Option<Value>,
    pub state: ToolCallState,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub retry_of: Option<Uuid>,
    pub native_provenance: JsonMap,
    pub observed_effect_ids: Vec<Uuid>,
    pub metadata: JsonMap,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ToolProgress {
    pub progress_id: Uuid,
    pub tool_call_id: Uuid,
    pub sequence: u64,
    pub chunk_type: String,
    pub content: Vec<ContentPart>,
    pub percent: Option<u8>,
    pub artifact_ids: Vec<Uuid>,
    pub locations: Vec<String>,
    pub native_display_data: Option<Value>,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ToolResultRecord {
    pub tool_call_id: Uuid,
    pub session_id: Uuid,
    pub task_id: Uuid,
    pub turn_id: Option<Uuid>,
    pub attempt_id: Uuid,
    pub state: ToolResultState,
    pub content: Vec<ContentPart>,
    pub structured_output: Option<Value>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: DateTime<Utc>,
    pub artifact_ids: Vec<Uuid>,
    pub observed_effect_ids: Vec<Uuid>,
    pub provider_metadata: JsonMap,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionKind {
    #[default]
    Approval,
    Question,
    Elicitation,
    AuthenticationChallenge,
    AdditionalInput,
    Custom,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionRisk {
    #[default]
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyDecision {
    #[default]
    Allowed,
    Denied,
    RequiresUser,
    NotEvaluated,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PolicyEvaluation {
    pub policy_id: String,
    pub policy_version: u32,
    pub decision: PolicyDecision,
    pub reason: Option<String>,
    pub evaluated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InteractionResponse {
    pub response_id: Uuid,
    pub state: InteractionState,
    pub value: Option<Value>,
    pub response_actor: Option<HistoricalActor>,
    pub responded_at: DateTime<Utc>,
    pub native_provenance: JsonMap,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Interaction {
    pub interaction_id: Uuid,
    pub request_id: String,
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub turn_id: Option<Uuid>,
    pub attempt_id: Option<Uuid>,
    pub kind: InteractionKind,
    pub subject: String,
    pub action: Option<String>,
    pub description: Option<String>,
    pub risk: InteractionRisk,
    pub input_schema: Option<Value>,
    pub options: Vec<Value>,
    pub requested_scope: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub state: InteractionState,
    pub response: Option<InteractionResponse>,
    pub policy_evaluation: Option<PolicyEvaluation>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub native_provenance: JsonMap,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanState {
    #[default]
    Draft,
    Proposed,
    Active,
    Completed,
    Failed,
    Cancelled,
    Unknown,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PlanStep {
    pub step_id: Uuid,
    pub plan_id: Uuid,
    pub ordinal: u32,
    pub title: String,
    pub description: Option<String>,
    pub state: String,
    pub task_id: Option<Uuid>,
    pub dependency_step_ids: Vec<Uuid>,
    pub completed_at: Option<DateTime<Utc>>,
    pub metadata: JsonMap,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Plan {
    pub plan_id: Uuid,
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub turn_id: Option<Uuid>,
    pub version: i64,
    pub objective: String,
    pub state: PlanState,
    pub steps: Vec<PlanStep>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub source: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TodoState {
    #[default]
    Pending,
    Running,
    Completed,
    Cancelled,
    Blocked,
    Unknown,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Todo {
    pub todo_id: Uuid,
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub plan_id: Option<Uuid>,
    pub title: String,
    pub details: Option<String>,
    pub state: TodoState,
    pub ordinal: u32,
    pub owner_actor: Option<HistoricalActor>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GoalState {
    #[default]
    Active,
    Completed,
    Failed,
    Cancelled,
    Unknown,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
pub struct GoalBudget {
    pub max_input_tokens: Option<u64>,
    pub max_output_tokens: Option<u64>,
    pub max_turns: Option<u32>,
    pub max_cost_micros: Option<u64>,
    pub max_wall_time_ms: Option<u64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
pub struct GoalUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub turns: u32,
    pub cost_micros: u64,
    pub wall_time_ms: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Goal {
    pub goal_id: Uuid,
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub objective: String,
    pub completion_criteria: Option<String>,
    pub state: GoalState,
    pub budget: GoalBudget,
    pub usage: GoalUsage,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcessStream {
    #[default]
    Stdout,
    Stderr,
    Combined,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Process {
    pub process_id: Uuid,
    pub session_id: Uuid,
    pub task_id: Uuid,
    pub turn_id: Option<Uuid>,
    pub attempt_id: Uuid,
    pub tool_call_id: Option<Uuid>,
    pub command: String,
    pub argv: Vec<String>,
    pub logical_cwd: Option<String>,
    pub environment_keys: Vec<String>,
    pub state: ProcessState,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub exit_code: Option<i32>,
    pub signal: Option<i32>,
    pub timed_out: bool,
    pub out_of_memory: bool,
    pub artifact_ids: Vec<Uuid>,
    pub metadata: JsonMap,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProcessChunk {
    pub chunk_id: Uuid,
    pub process_id: Uuid,
    pub stream: ProcessStream,
    pub sequence: u64,
    pub content: String,
    pub occurred_at: DateTime<Utc>,
    pub artifact_id: Option<Uuid>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct WorkspaceDescriptor {
    pub descriptor_id: Uuid,
    pub session_id: Uuid,
    pub logical_roots: Vec<String>,
    pub mount_policy: Option<String>,
    pub operating_system: Option<String>,
    pub architecture: Option<String>,
    pub shell: Option<String>,
    pub toolchain_hints: Vec<String>,
    pub environment_allowlist: BTreeSet<String>,
    pub repository_identity: JsonMap,
    pub data_locality: Option<String>,
    pub metadata: JsonMap,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct WorkspaceSnapshot {
    pub snapshot_id: Uuid,
    pub session_id: Uuid,
    pub parent_snapshot_id: Option<Uuid>,
    pub durable_sequence: Option<i64>,
    pub tree_digest: String,
    pub archive_artifact_id: Option<Uuid>,
    pub git_repository_identity: JsonMap,
    pub base_commit: Option<String>,
    pub branch: Option<String>,
    pub remote: Option<String>,
    pub working_tree_patch_digest: Option<String>,
    pub index_patch_digest: Option<String>,
    pub untracked_files: Vec<String>,
    pub submodule_state: JsonMap,
    pub file_metadata: JsonMap,
    pub completeness: String,
    pub creator_attempt_id: Option<Uuid>,
    pub reason: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub integrity_digest: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InstructionAuthority {
    System,
    Developer,
    User,
    #[default]
    Historical,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PortableInstructionLayer {
    pub layer_id: Uuid,
    pub authority: InstructionAuthority,
    pub provenance: String,
    pub text: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
pub struct ResourceLimits {
    pub max_context_tokens: Option<u64>,
    pub max_output_tokens: Option<u64>,
    pub reasoning_budget_tokens: Option<u64>,
    pub max_wall_time_ms: Option<u64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RuntimeConfigSnapshot {
    pub snapshot_id: Uuid,
    pub tenant_id: String,
    pub snapshot_digest: String,
    pub instruction_layers: Vec<PortableInstructionLayer>,
    pub agent_profile: Option<String>,
    pub collaboration_settings: JsonMap,
    pub requested_model: Option<ModelDescriptor>,
    #[serde(default)]
    pub resolved_model: Option<ResolvedModelSelection>,
    #[serde(default)]
    pub local_service_bundle: Option<LocalServiceBundle>,
    pub response_schema: Option<Value>,
    pub resource_limits: ResourceLimits,
    pub tool_definition_snapshot_ids: Vec<Uuid>,
    pub mcp_descriptors: Vec<String>,
    pub skill_descriptors: Vec<String>,
    pub plugin_descriptors: Vec<String>,
    pub hook_descriptors: Vec<String>,
    pub sandbox_policy: JsonMap,
    pub compaction_settings: JsonMap,
    pub retry_settings: JsonMap,
    pub budget_settings: JsonMap,
    pub telemetry_settings: JsonMap,
    pub environment_allowlist: BTreeSet<String>,
    pub workspace_snapshot_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
pub struct UsageRecord {
    pub usage_id: Uuid,
    pub tenant_id: String,
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub turn_id: Option<Uuid>,
    pub attempt_id: Option<Uuid>,
    pub model_binding_id: Option<Uuid>,
    pub provider: Option<String>,
    pub model_id: Option<String>,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cached_tokens: u64,
    pub reasoning_tokens: u64,
    pub cost_micros: u64,
    pub latency_ms: Option<u64>,
    pub finish_reason: Option<String>,
    pub recorded_at: Option<DateTime<Utc>>,
    pub metadata: StringMap,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
pub struct ErrorRecord {
    pub error_id: Uuid,
    pub tenant_id: String,
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub turn_id: Option<Uuid>,
    pub attempt_id: Option<Uuid>,
    pub source: String,
    pub code: String,
    pub message: String,
    pub retriable: bool,
    pub uncertain: bool,
    pub failure_class: Option<String>,
    pub occurred_at: Option<DateTime<Utc>>,
    pub metadata: StringMap,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Compaction {
    pub compaction_id: Uuid,
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub turn_id: Option<Uuid>,
    pub source_durable_ranges: Vec<(i64, i64)>,
    pub retained_ancestor_event_id: Option<Uuid>,
    pub summary: String,
    pub reason: String,
    pub created_by_attempt_id: Option<Uuid>,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub created_at: DateTime<Utc>,
    pub integrity_digest: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum PatchValue<T> {
    Unset,
    Null,
    Value(T),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ResumeCheckpoint {
    pub checkpoint_id: Uuid,
    pub binding_id: Uuid,
    pub binding_generation: i64,
    pub task_id: Uuid,
    pub turn_id: Option<Uuid>,
    pub source_attempt_id: Uuid,
    pub created_by_attempt_id: Uuid,
    pub created_by_lease_generation: i64,
    pub native_cursor: Option<String>,
    pub native_record_set_digest: String,
    pub durable_sequence: i64,
    pub runtime_config_digest: String,
    pub model_binding_id: Option<Uuid>,
    pub workspace_snapshot_digest: Option<String>,
    pub effect_watermark: i64,
    pub command_inbox_watermark: i64,
    pub command_outbox_watermark: i64,
    pub pending_interaction_ids: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub integrity_digest: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NativeRecordSet {
    pub record_set_id: Uuid,
    pub session_id: Uuid,
    pub attempt_id: Option<Uuid>,
    pub harness_id: String,
    pub adapter_version: String,
    pub native_schema: String,
    pub first_cursor: Option<String>,
    pub last_cursor: Option<String>,
    pub record_count: u64,
    pub payload_digest: String,
    pub object_storage_ref: Option<String>,
    pub captured_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NativeRecord {
    pub native_record_id: Uuid,
    pub record_set_id: Uuid,
    pub harness_id: String,
    pub adapter_version: String,
    pub native_schema: String,
    pub record_kind: String,
    pub native_identity: Option<String>,
    pub native_cursor: Option<String>,
    pub ordinal: Option<i64>,
    pub payload_digest: String,
    pub object_storage_ref: Option<String>,
    pub captured_at: DateTime<Utc>,
    pub data_classification: Option<String>,
    pub encrypted: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CommandInboxEntry {
    pub command_id: Uuid,
    pub tenant_id: String,
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub turn_id: Option<Uuid>,
    pub attempt_id: Option<Uuid>,
    pub lease_id: Option<Uuid>,
    pub lease_generation: Option<i64>,
    pub fencing_token: Option<String>,
    pub idempotency_key: String,
    pub command_type: String,
    pub payload_schema: String,
    pub payload_version: u32,
    pub payload: Value,
    pub status: String,
    pub outcome_event_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub settled_at: Option<DateTime<Utc>>,
    pub unresolved_effect: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CommandOutboxEntry {
    pub outbox_id: Uuid,
    pub tenant_id: String,
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub turn_id: Option<Uuid>,
    pub attempt_id: Option<Uuid>,
    pub command_id: Option<Uuid>,
    pub event_id: Option<Uuid>,
    pub idempotency_key: String,
    pub payload_schema: String,
    pub payload_version: u32,
    pub payload: Value,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CapabilitySnapshot {
    pub snapshot_id: Uuid,
    pub subject_kind: String,
    pub subject_id: String,
    pub capability_version: u64,
    pub capabilities: BTreeSet<String>,
    pub source_revision: Option<String>,
    pub captured_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub digest: String,
    pub metadata: JsonMap,
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use serde_json::json;
    use uuid::Uuid;

    use super::*;

    #[test]
    fn historical_actor_is_authority_free() {
        let actor = ActorDescriptor {
            actor_id: "user-42".to_owned(),
            actor_type: ActorType::Human,
            display_name: Some("Ada".to_owned()),
            provider: Some("omnisolo".to_owned()),
            external_subject: Some("subject-42".to_owned()),
            metadata: [("role".to_owned(), "developer".to_owned())]
                .into_iter()
                .collect(),
            ..Default::default()
        };

        let historical = HistoricalActor::from(&actor);
        assert_eq!(historical.actor_id, actor.actor_id);
        assert_eq!(historical.historical_role, "human");
        assert!(historical.delegation_lineage.is_empty());
        assert!(!historical.has_authority_context());

        let encoded = serde_json::to_value(&historical).unwrap();
        assert_eq!(encoded.get("display_name"), Some(&json!("Ada")));
        assert!(encoded.get("provider").is_none());
        assert!(encoded.get("external_subject").is_none());
        assert!(encoded.get("authenticated_principal_ref").is_none());
    }

    #[test]
    fn actor_types_and_delegation_lineage_are_deterministic() {
        for actor_type in [
            ActorType::Human,
            ActorType::Agent,
            ActorType::Subagent,
            ActorType::Service,
            ActorType::Harness,
            ActorType::System,
            ActorType::Unknown,
        ] {
            assert!(!actor_type.to_string().is_empty());
        }

        let actor = ActorDescriptor {
            actor_id: "child".to_owned(),
            delegation_parent_id: Some("parent".to_owned()),
            ..Default::default()
        };
        let historical = HistoricalActor::from(&actor);
        assert_eq!(historical.delegation_lineage, ["parent"]);
        assert!(!historical.has_authority_context());
    }

    #[test]
    fn canonical_content_and_artifact_round_trip() {
        let artifact = ArtifactRef {
            artifact_id: Uuid::new_v4(),
            kind: ArtifactKind::File,
            name: "report.md".to_owned(),
            media_type: Some("text/markdown".to_owned()),
            byte_length: 12,
            sha256: "a".repeat(64),
            uri: Some("artifact://report".to_owned()),
            metadata: [("path".to_owned(), "report.md".to_owned())]
                .into_iter()
                .collect(),
        };
        let content = ContentPart::File { artifact };
        let encoded = serde_json::to_string(&content).unwrap();
        let decoded: ContentPart = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, content);

        let text = ContentPart::Text {
            text: "finished".to_owned(),
            annotations: Vec::new(),
        };
        assert_eq!(
            serde_json::from_value::<ContentPart>(json!({
                "type": "text",
                "text": "finished",
                "annotations": []
            }))
            .unwrap(),
            text
        );
    }

    #[test]
    fn model_runtime_descriptor_preserves_future_runtime_fields() {
        let descriptor = ModelRuntimeDescriptor {
            runtime_id: "runtime-local-1".to_owned(),
            model: ModelDescriptor {
                model_id: "qwen3-coder".to_owned(),
                provider: ModelProvider::SelfHosted,
                revision: Some("sha256:model".to_owned()),
                context_window: Some(131_072),
                capabilities: ["tool_use".to_owned(), "vision".to_owned()]
                    .into_iter()
                    .collect(),
                metadata: Default::default(),
                ..Default::default()
            },
            kind: ModelRuntimeKind::KubernetesService,
            engine: Some(ServingEngine::Vllm),
            endpoint: Some("http://qwen3.default.svc".to_owned()),
            image: Some("registry.example/qwen:v1".to_owned()),
            gpu_profile: Some("1xH100".to_owned()),
            lora_adapter: Some("repo/adapter".to_owned()),
            placement: Some(RuntimePlacement {
                cluster: Some("prod".to_owned()),
                namespace: Some("models".to_owned()),
                node_selector: [("accelerator".to_owned(), "h100".to_owned())]
                    .into_iter()
                    .collect(),
                ..Default::default()
            }),
            capacity: RuntimeCapacity {
                max_concurrent_requests: 8,
                max_batch_tokens: Some(16_384),
                ..Default::default()
            },
            autoscaling: Some(RuntimeAutoscaling {
                min_replicas: 1,
                max_replicas: 4,
                target_utilization_percent: 70,
                ..Default::default()
            }),
            metadata: [("future_field".to_owned(), json!({"enabled": true}))]
                .into_iter()
                .collect(),
            ..Default::default()
        };

        let encoded = serde_json::to_value(&descriptor).unwrap();
        assert_eq!(encoded["model"]["provider"], json!("self_hosted"));
        assert_eq!(encoded["engine"], json!("vllm"));
        assert_eq!(encoded["metadata"]["future_field"]["enabled"], json!(true));
        assert_eq!(
            serde_json::from_value::<ModelRuntimeDescriptor>(encoded).unwrap(),
            descriptor
        );
    }

    fn resolved_model_with_metadata(metadata: JsonMap) -> ResolvedModelSelection {
        ResolvedModelSelection {
            provider_route: "openai-compatible".to_owned(),
            model_id: "gpt-5.6-luna".to_owned(),
            reasoning_effort: Some(ReasoningEffort::Max),
            api_dialect: ModelApiDialect::OpenAiResponses,
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            capabilities: ["tools".to_owned(), "reasoning".to_owned()]
                .into_iter()
                .collect(),
            binding_revision: "binding-v1".to_owned(),
            binding_digest: "sha256:test".to_owned(),
            metadata,
        }
    }

    #[test]
    fn credential_sanitizer_value_handles_nested_keys_and_secret_like_scalars() {
        const CANARY: &str = "CANARY-OPAQUE-7KQ9";

        for sensitive_key in [
            "api_key",
            "accessToken",
            "refresh-token",
            "clientSecret",
            "authorization",
            "cookie",
            "password",
        ] {
            let mut nested = serde_json::Map::new();
            nested.insert(sensitive_key.to_owned(), json!(CANARY));
            nested.insert("bearer_note".to_owned(), json!(format!("Bearer {CANARY}")));
            nested.insert(
                "assignment_note".to_owned(),
                json!(format!("api_key={CANARY}")),
            );

            let input = json!({
                "name": "portable-name",
                "payload": "portable-payload",
                "model": "synthetic-model",
                "usage": {"input_tokens": 12, "output_tokens": 4},
                "ordinary": [{"nested": Value::Object(nested)}],
            });
            let original = input.clone();

            let sanitized = sanitize_credential_value(&input);

            assert!(
                sanitized["ordinary"][0]["nested"]
                    .get(sensitive_key)
                    .is_none(),
                "sensitive nested key survived: {sensitive_key}"
            );
            assert_eq!(
                sanitized["ordinary"][0]["nested"]["bearer_note"],
                json!("[REDACTED]")
            );
            assert_eq!(
                sanitized["ordinary"][0]["nested"]["assignment_note"],
                json!("[REDACTED]")
            );
            assert_eq!(sanitized["name"], input["name"]);
            assert_eq!(sanitized["payload"], input["payload"]);
            assert_eq!(sanitized["model"], input["model"]);
            assert_eq!(sanitized["usage"], input["usage"]);
            assert_eq!(input, original, "sanitizer mutated its input");
        }
    }

    #[test]
    fn credential_sanitizer_map_drops_sensitive_top_level_keys() {
        const CANARY: &str = "CANARY-OPAQUE-7KQ9";

        for sensitive_key in [
            "api_key",
            "accessToken",
            "refresh-token",
            "clientSecret",
            "authorization",
            "cookie",
            "password",
        ] {
            let metadata = [
                (sensitive_key.to_owned(), json!(CANARY)),
                ("name".to_owned(), json!("portable-name")),
                ("payload".to_owned(), json!({"safe": true})),
                ("model".to_owned(), json!("synthetic-model")),
                (
                    "usage".to_owned(),
                    json!({"input_tokens": 12, "output_tokens": 4}),
                ),
            ]
            .into_iter()
            .collect::<JsonMap>();
            let original = metadata.clone();

            let sanitized = sanitize_credential_map(&metadata);

            assert!(
                !sanitized.contains_key(sensitive_key),
                "sensitive top-level key survived: {sensitive_key}"
            );
            for safe_key in ["name", "payload", "model", "usage"] {
                assert_eq!(sanitized.get(safe_key), metadata.get(safe_key));
            }
            assert_eq!(metadata, original, "sanitizer mutated its input");
        }
    }

    #[test]
    fn resolved_model_metadata_redacts_explicit_credential_key_forms() {
        for key in [
            "api_key",
            "api-key",
            "apiKey",
            "apikey",
            "openaiApikey",
            "provider_apikey",
            "openaiApiKey",
            "anthropicApiKey",
            "providerApiKey",
            "access_token",
            "accessToken",
            "refreshToken",
            "credentialRef",
            "clientSecret",
            "authorization",
            "authorizationHeader",
            "password",
            "databasePassword",
            "cookie",
            "sessionCookie",
        ] {
            let mut nested = serde_json::Map::new();
            nested.insert(key.to_owned(), json!("credential-canary"));
            nested.insert("ordinary".to_owned(), json!("preserved"));
            let selection = resolved_model_with_metadata(
                [(
                    "nested".to_owned(),
                    Value::Array(vec![Value::Object(nested)]),
                )]
                .into_iter()
                .collect(),
            );

            let encoded = serde_json::to_value(selection).unwrap();
            let nested = &encoded["metadata"]["nested"][0];
            assert!(nested.get(key).is_none(), "credential key leaked: {key}");
            assert_eq!(nested["ordinary"], json!("preserved"));
            assert!(
                !serde_json::to_string(&encoded)
                    .unwrap()
                    .contains("credential-canary"),
                "credential value leaked for key: {key}"
            );
        }
    }

    #[test]
    fn resolved_model_metadata_redacts_only_explicit_credential_values() {
        for value in [
            "Bearer credential-canary",
            "api_key=credential-canary",
            "accessToken=credential-canary",
            "client_secret=credential-canary",
            "secret=credential-canary",
            "password=credential-canary",
            "authorization: credential-canary",
            "credential=credential-canary",
            "-----BEGIN PRIVATE KEY----- credential-canary",
            "https://host.test/v1?api_key=credential-canary",
            "https://host.test/v1?accessToken=credential-canary",
            "https://host.test/v1?region=us&api_key=credential-canary&mode=fast",
            "https://user:credential-canary@host.test/v1",
        ] {
            let selection = resolved_model_with_metadata(
                [("note".to_owned(), json!(value))].into_iter().collect(),
            );
            let encoded = serde_json::to_value(selection).unwrap();
            assert_eq!(encoded["metadata"]["note"], json!("[REDACTED]"));
        }

        for (key, value) in [
            ("max_output_token", "128000"),
            ("max_output_tokens", "128000"),
            ("input_tokens", "1200"),
            ("cached_tokens", "800"),
            ("tokenizer", "o200k_base"),
            ("token_count", "2000"),
            ("endpoint", "https://api.example.test/v1"),
            ("base_url", "https://api.example.test"),
            ("model_id", "gpt-5.6-luna"),
            ("note", "max_output_token=128000"),
            ("authorization_scheme", "Bearer"),
            ("authorization_type", "oauth2"),
            ("password_policy", "minimum-16-characters"),
            ("password_required", "true"),
            ("cookie_support", "enabled"),
            ("cookie_policy", "same-site-strict"),
            ("endpoint", "https://host.test/v1?region=us&mode=fast"),
            ("endpoint", "https://user@host.test/v1"),
        ] {
            let selection = resolved_model_with_metadata(
                [(key.to_owned(), json!(value))].into_iter().collect(),
            );
            let encoded = serde_json::to_value(selection).unwrap();
            assert_eq!(
                encoded["metadata"][key],
                json!(value),
                "metadata changed: {key}"
            );
        }
    }

    #[test]
    fn resolved_model_selection_is_portable_and_backward_compatible() {
        let selection = ResolvedModelSelection {
            provider_route: "openai-compatible".to_owned(),
            model_id: "gpt-5.6-luna".to_owned(),
            reasoning_effort: Some(ReasoningEffort::Max),
            api_dialect: ModelApiDialect::OpenAiResponses,
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            capabilities: ["tools".to_owned(), "reasoning".to_owned()]
                .into_iter()
                .collect(),
            binding_revision: "binding-v1".to_owned(),
            binding_digest: "sha256:test".to_owned(),
            metadata: [("routing_tier".to_owned(), json!("balanced"))]
                .into_iter()
                .collect(),
        };

        let encoded = serde_json::to_value(&selection).unwrap();
        assert_eq!(encoded["reasoning_effort"], json!("max"));
        assert_eq!(encoded["api_dialect"], json!("open_ai_responses"));
        assert!(encoded.get("api_key").is_none());
        assert!(encoded.get("credential_ref").is_none());
        assert_eq!(
            serde_json::from_value::<ResolvedModelSelection>(encoded).unwrap(),
            selection
        );
        assert_eq!(
            serde_json::from_value::<ReasoningEffort>(json!("future_effort")).unwrap(),
            ReasoningEffort::Custom
        );
        assert_eq!(
            serde_json::from_value::<ModelApiDialect>(json!("future_dialect")).unwrap(),
            ModelApiDialect::Custom
        );

        let config = RuntimeConfigSnapshot {
            resolved_model: Some(selection),
            ..Default::default()
        };
        let mut legacy_json = serde_json::to_value(config).unwrap();
        legacy_json
            .as_object_mut()
            .unwrap()
            .remove("resolved_model");
        let decoded: RuntimeConfigSnapshot = serde_json::from_value(legacy_json).unwrap();
        assert!(decoded.resolved_model.is_none());
    }

    #[test]
    fn attempt_can_be_task_scoped_without_turn() {
        let task_id = Uuid::new_v4();
        let attempt = Attempt {
            attempt_id: Uuid::new_v4(),
            session_id: Uuid::new_v4(),
            task_id,
            turn_id: None,
            parent_attempt_id: None,
            kind: AttemptKind::Background,
            state: AttemptState::Running,
            state_version: 3,
            actor: None,
            harness_id: "omnisolo".to_owned(),
            model_runtime_id: Some("runtime-local-1".to_owned()),
            started_at: Utc.timestamp_opt(1_700_000_000, 0).single().unwrap(),
            finished_at: None,
            runtime_config_snapshot_id: None,
            model_binding_id: None,
            worker_id: None,
            extensions: Default::default(),
            metadata: Default::default(),
        };

        assert_eq!(attempt.task_id, task_id);
        assert!(attempt.turn_id.is_none());
        let decoded: Attempt =
            serde_json::from_value(serde_json::to_value(&attempt).unwrap()).unwrap();
        assert_eq!(decoded, attempt);
    }

    #[test]
    fn lifecycle_state_helpers_only_mark_terminal_states() {
        assert!(!TaskState::Running.is_terminal());
        assert!(TaskState::Completed.is_terminal());
        assert!(TaskState::Failed.is_terminal());
        assert!(TaskState::Cancelled.is_terminal());
        assert!(TaskState::Lost.is_terminal());

        assert!(!TurnState::WaitingInput.is_terminal());
        assert!(TurnState::Completed.is_terminal());
        assert!(TurnState::Interrupted.is_terminal());
        assert!(TurnState::Failed.is_terminal());
        assert!(TurnState::Cancelled.is_terminal());

        assert!(!AttemptState::Checkpointing.is_terminal());
        assert!(AttemptState::Succeeded.is_terminal());
        assert!(AttemptState::Failed.is_terminal());
        assert!(AttemptState::Cancelled.is_terminal());
        assert!(AttemptState::Lost.is_terminal());
        assert!(AttemptState::Fenced.is_terminal());
    }

    #[test]
    fn event_envelope_round_trip_preserves_replay_identity() {
        let event = EventEnvelope {
            event_id: Uuid::new_v4(),
            tenant_id: "tenant-1".to_owned(),
            session_id: Uuid::new_v4(),
            task_id: Some(Uuid::new_v4()),
            turn_id: None,
            source_attempt_id: Some(Uuid::new_v4()),
            ingest_attempt_id: Some(Uuid::new_v4()),
            actor_id: Some("agent-1".to_owned()),
            worker_id: Some("worker-1".to_owned()),
            harness_id: Some("omnisolo".to_owned()),
            binding_id: Some(Uuid::new_v4()),
            binding_generation: Some(4),
            lease_id: Some(Uuid::new_v4()),
            lease_generation: Some(2),
            fencing_token: Some("fence-2".to_owned()),
            durable_sequence: Some(19),
            delivery_stream_id: Some("stream-1".to_owned()),
            delivery_sequence: Some(23),
            aggregate_id: Some(Uuid::new_v4()),
            aggregate_sequence: Some(7),
            branch_id: Some(Uuid::new_v4()),
            parent_event_ids: vec![Uuid::new_v4()],
            event_type: "turn.completed".to_owned(),
            payload_schema: "omnisolo.turn.v1".to_owned(),
            payload_version: 1,
            occurred_at: Utc.timestamp_opt(1_700_000_000, 0).single().unwrap(),
            ingested_at: Utc.timestamp_opt(1_700_000_001, 0).single().unwrap(),
            correlation_id: Some(Uuid::new_v4()),
            causation_id: Some(Uuid::new_v4()),
            idempotency_key: Some("turn-1-completed".to_owned()),
            durability: EventDurability::Durable,
            replay_requirement: ReplayRequirement::Required,
            visibility: Some("user".to_owned()),
            data_classification: Some("internal".to_owned()),
            native_provenance: [("cursor".to_owned(), json!(42))].into_iter().collect(),
            payload: json!({"stop_reason": "completed"}),
            extensions: [("adapter".to_owned(), json!({"version": 1}))]
                .into_iter()
                .collect(),
        };

        let decoded: EventEnvelope =
            serde_json::from_value(serde_json::to_value(&event).unwrap()).unwrap();
        assert_eq!(decoded, event);
        assert_eq!(decoded.durable_sequence, Some(19));
        assert_eq!(decoded.delivery_sequence, Some(23));
        assert_eq!(decoded.source_attempt_id, event.source_attempt_id);
        assert_eq!(decoded.ingest_attempt_id, event.ingest_attempt_id);
    }

    #[test]
    fn typed_effect_records_round_trip_without_collapsing_lifecycle_fields() {
        let now = Utc.timestamp_opt(1_700_000_100, 0).single().unwrap();
        let session_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();
        let attempt_id = Uuid::new_v4();
        let definition = ToolDefinitionSnapshot {
            snapshot_id: Uuid::new_v4(),
            qualified_name: "mcp.files.read".to_owned(),
            description: Some("Read a file".to_owned()),
            source: ToolSource::Mcp,
            input_schema: json!({"type": "object"}),
            output_schema: Some(json!({"type": "string"})),
            annotations: [("read_only".to_owned(), json!(true))]
                .into_iter()
                .collect(),
            version: "2026-08".to_owned(),
            digest: "tool-digest".to_owned(),
            captured_at: now,
        };
        let call = ToolCall {
            tool_call_id: Uuid::new_v4(),
            session_id,
            task_id,
            turn_id: None,
            attempt_id,
            definition_snapshot_id: definition.snapshot_id,
            raw_input: json!({"path": "README.md"}),
            parsed_input: Some(json!({"path": "README.md"})),
            state: ToolCallState::Completed,
            started_at: now,
            finished_at: Some(now),
            retry_of: None,
            native_provenance: [("native_cursor".to_owned(), json!(42))]
                .into_iter()
                .collect(),
            observed_effect_ids: vec![Uuid::new_v4()],
            metadata: Default::default(),
        };
        let interaction = Interaction {
            interaction_id: Uuid::new_v4(),
            request_id: "approval-1".to_owned(),
            session_id,
            task_id: Some(task_id),
            turn_id: None,
            attempt_id: Some(attempt_id),
            kind: InteractionKind::Approval,
            subject: "write_file".to_owned(),
            action: Some("write".to_owned()),
            description: Some("Approve the patch".to_owned()),
            risk: InteractionRisk::Medium,
            input_schema: None,
            options: vec![json!("approve"), json!("decline")],
            requested_scope: Some("workspace-1".to_owned()),
            expires_at: None,
            state: InteractionState::Pending,
            response: None,
            policy_evaluation: Some(PolicyEvaluation {
                policy_id: "policy-1".to_owned(),
                policy_version: 1,
                decision: PolicyDecision::RequiresUser,
                reason: None,
                evaluated_at: now,
            }),
            created_at: now,
            updated_at: now,
            native_provenance: Default::default(),
        };
        let config = RuntimeConfigSnapshot {
            snapshot_id: Uuid::new_v4(),
            tenant_id: "tenant-1".to_owned(),
            snapshot_digest: "config-digest".to_owned(),
            instruction_layers: vec![PortableInstructionLayer {
                layer_id: Uuid::new_v4(),
                authority: InstructionAuthority::Developer,
                provenance: "project-policy".to_owned(),
                text: "Keep changes reviewable".to_owned(),
            }],
            requested_model: Some(ModelDescriptor {
                model_id: "future-self-hosted-model".to_owned(),
                provider: ModelProvider::SelfHosted,
                ..Default::default()
            }),
            created_at: now,
            ..Default::default()
        };

        for value in [
            serde_json::to_value(&definition).unwrap(),
            serde_json::to_value(&call).unwrap(),
            serde_json::to_value(&interaction).unwrap(),
            serde_json::to_value(&config).unwrap(),
        ] {
            assert!(value.is_object());
        }
        let decoded_call: ToolCall =
            serde_json::from_value(serde_json::to_value(&call).unwrap()).unwrap();
        assert_eq!(decoded_call.state, ToolCallState::Completed);
        assert_eq!(decoded_call.session_id, session_id);
        assert!(matches!(PatchValue::<String>::Unset, PatchValue::Unset));
        assert!(matches!(PatchValue::<String>::Null, PatchValue::Null));
        assert!(matches!(
            PatchValue::Value("configured".to_owned()),
            PatchValue::Value(_)
        ));
    }
}
