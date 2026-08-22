use std::collections::{BTreeMap, BTreeSet};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

pub type JsonMap = BTreeMap<String, Value>;
pub type StringMap = BTreeMap<String, String>;

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
}
