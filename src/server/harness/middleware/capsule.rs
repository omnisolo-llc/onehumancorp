use std::collections::{BTreeMap, BTreeSet, HashMap};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use super::events::EventStore;
use super::lifecycle::{HandoffState, LifecycleError, LifecycleState, VersionedState};
use super::local_services::{
    LOCAL_SERVICE_BUNDLE_SCHEMA, LocalServiceBinding, LocalServiceBundle,
};
use super::types::{
    ArtifactKind, ArtifactRef, Attempt, AttemptKind, AttemptState, BindingAccessMode, BindingScope,
    BindingState, CapabilitySnapshot, Compaction, ContentAnnotation, ContentPart, ErrorRecord,
    Goal, HistoricalActor, Interaction, JsonMap, Message, MessageRole, MessageStatus,
    ModelDescriptor, NativeRecord, NativeRecordSet, Plan, Process, ProcessChunk,
    RuntimeConfigSnapshot, Session, SessionBinding, SessionState, Task, TaskKind, TaskResult,
    TaskState, Todo, ToolCall, ToolDefinitionSnapshot, ToolProgress, ToolResultRecord, Turn,
    UsageRecord, WorkspaceDescriptor, WorkspaceSnapshot, sensitive_json_key, sensitive_json_text,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Ord, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LossSeverity {
    Info,
    Warning,
    Required,
    Unsafe,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct LossEntry {
    pub source_path: String,
    pub reason: String,
    pub severity: LossSeverity,
    pub target_representation: Option<String>,
    pub acknowledgement_required: bool,
    pub capability: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LossReport {
    pub entries: Vec<LossEntry>,
}

impl LossReport {
    pub fn new(mut entries: Vec<LossEntry>) -> Self {
        entries.sort_by(|left, right| {
            (
                &left.source_path,
                &left.reason,
                &left.severity,
                &left.target_representation,
            )
                .cmp(&(
                    &right.source_path,
                    &right.reason,
                    &right.severity,
                    &right.target_representation,
                ))
        });
        Self { entries }
    }

    pub fn digest(&self) -> String {
        sha256_json(self)
    }

    pub fn requires_ack(&self) -> bool {
        self.entries.iter().any(|entry| match entry.severity {
            LossSeverity::Required | LossSeverity::Unsafe => true,
            LossSeverity::Info | LossSeverity::Warning => entry.acknowledgement_required,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PortableContextCheckpoint {
    pub summary: String,
    pub selected_message_ids: Vec<Uuid>,
    pub source_durable_ranges: Vec<(i64, i64)>,
    pub retained_ancestor_event_id: Option<Uuid>,
    pub compaction_provenance: Option<String>,
    pub usage: JsonMap,
    pub integrity_digest: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PortableSession {
    pub session_id: Uuid,
    pub tenant_id: String,
    pub project_id: Option<String>,
    pub workspace_id: Option<String>,
    pub title: Option<String>,
    pub labels: Vec<String>,
    pub tags: BTreeMap<String, String>,
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
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PortableTask {
    pub task_id: Uuid,
    pub session_id: Uuid,
    pub parent_task_id: Option<Uuid>,
    pub parent_attempt_id: Option<Uuid>,
    pub kind: TaskKind,
    pub objective: String,
    pub state: TaskState,
    pub state_version: i64,
    pub dependency_task_ids: Vec<Uuid>,
    pub owner_actor: Option<HistoricalActor>,
    pub input_message_ids: Vec<Uuid>,
    pub output_message_ids: Vec<Uuid>,
    pub artifact_ids: Vec<Uuid>,
    pub terminal_result: Option<PortableTaskResult>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PortableTaskResult {
    pub status: String,
    pub summary: Option<String>,
    pub output_message_ids: Vec<Uuid>,
    pub artifact_ids: Vec<Uuid>,
    pub error_code: Option<String>,
    pub completed_at: DateTime<Utc>,
}

impl From<&TaskResult> for PortableTaskResult {
    fn from(result: &TaskResult) -> Self {
        Self {
            status: result.status.clone(),
            summary: result.summary.clone(),
            output_message_ids: result.output_message_ids.clone(),
            artifact_ids: result.artifact_ids.clone(),
            error_code: result.error_code.clone(),
            completed_at: result.completed_at,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PortableTurn {
    pub turn_id: Uuid,
    pub task_id: Uuid,
    pub session_id: Uuid,
    pub sequence: i64,
    pub state: super::types::TurnState,
    pub state_version: i64,
    pub input_message_id: Option<Uuid>,
    pub output_message_ids: Vec<Uuid>,
    pub stop_reason: Option<String>,
    pub error_code: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PortableAttempt {
    pub attempt_id: Uuid,
    pub session_id: Uuid,
    pub task_id: Uuid,
    pub turn_id: Option<Uuid>,
    pub parent_attempt_id: Option<Uuid>,
    pub kind: AttemptKind,
    pub state: AttemptState,
    pub state_version: i64,
    pub actor: Option<HistoricalActor>,
    pub harness_id: String,
    pub model_runtime_id: Option<String>,
    pub worker_id: Option<String>,
    pub runtime_config_snapshot_id: Option<Uuid>,
    pub metadata: JsonMap,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PortableMessage {
    pub message_id: Uuid,
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub turn_id: Option<Uuid>,
    pub role: MessageRole,
    pub actor: Option<super::types::HistoricalActor>,
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
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PortableArtifact {
    pub artifact_id: Uuid,
    pub kind: ArtifactKind,
    pub name: String,
    pub media_type: Option<String>,
    pub byte_length: u64,
    pub sha256: String,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PortableInstruction {
    pub authority: String,
    pub text: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RuntimeConfigInput {
    pub snapshot_id: Uuid,
    pub system_instructions: Vec<String>,
    pub developer_instructions: Vec<String>,
    pub user_instructions: Vec<String>,
    pub requested_model: ModelDescriptor,
    pub response_schema: Option<Value>,
    pub tool_definition_digests: Vec<String>,
    pub extensions: JsonMap,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PortableRuntimeConfig {
    pub snapshot_id: Uuid,
    pub instruction_layers: Vec<PortableInstruction>,
    pub requested_model: ModelDescriptor,
    pub response_schema: Option<Value>,
    pub tool_definition_digests: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PortableToolResult {
    pub tool_call_id: Uuid,
    pub content: Vec<ContentPart>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HistoricalData {
    pub source_kind: String,
    pub summary: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PortableStructuredRecord {
    pub record_id: Uuid,
    pub kind: String,
    pub data: JsonMap,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum PortableRecord {
    Session(PortableSession),
    Task(PortableTask),
    Turn(PortableTurn),
    Attempt(PortableAttempt),
    Message(PortableMessage),
    Artifact(PortableArtifact),
    RuntimeConfig(PortableRuntimeConfig),
    ToolResult(PortableToolResult),
    ToolDefinition(ToolDefinitionSnapshot),
    ToolCall(ToolCall),
    ToolProgress(ToolProgress),
    ToolResultRecord(ToolResultRecord),
    Interaction(PortableStructuredRecord),
    Plan(PortableStructuredRecord),
    Todo(PortableStructuredRecord),
    Goal(PortableStructuredRecord),
    Process(PortableStructuredRecord),
    WorkspaceSnapshot(PortableStructuredRecord),
    TypedInteraction(Interaction),
    TypedPlan(Plan),
    TypedTodo(Todo),
    TypedGoal(Goal),
    TypedProcess(Process),
    TypedProcessChunk(ProcessChunk),
    TypedWorkspaceDescriptor(WorkspaceDescriptor),
    TypedWorkspaceSnapshot(WorkspaceSnapshot),
    RuntimeConfigSnapshot(RuntimeConfigSnapshot),
    Usage(UsageRecord),
    Error(ErrorRecord),
    Compaction(Compaction),
    CapabilitySnapshot(CapabilitySnapshot),
    ContextCheckpoint(PortableContextCheckpoint),
    HistoricalData(HistoricalData),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum CanonicalRecord {
    Session(Session),
    Task(Task),
    Turn(Turn),
    Attempt(Attempt),
    Message(Message),
    Artifact(ArtifactRef),
    RuntimeConfig(RuntimeConfigInput),
    ToolResult {
        tool_call_id: Uuid,
        content: Vec<ContentPart>,
        metadata: BTreeMap<String, String>,
    },
    ToolDefinition(ToolDefinitionSnapshot),
    ToolCall(ToolCall),
    ToolProgress(ToolProgress),
    ToolResultRecord(ToolResultRecord),
    Interaction(Interaction),
    Plan(Plan),
    Todo(Todo),
    Goal(Goal),
    Process(Process),
    ProcessChunk(ProcessChunk),
    WorkspaceDescriptor(WorkspaceDescriptor),
    WorkspaceSnapshot(WorkspaceSnapshot),
    RuntimeConfigSnapshot(RuntimeConfigSnapshot),
    Usage(UsageRecord),
    Error(ErrorRecord),
    Compaction(Compaction),
    CapabilitySnapshot(CapabilitySnapshot),
    NativeRecordSet(NativeRecordSet),
    NativeRecordTyped(NativeRecord),
    ContextCheckpoint(PortableContextCheckpoint),
    NativeRecord {
        kind: String,
        payload: Value,
    },
    Unsupported {
        kind: String,
        summary: String,
    },
    RawReasoning {
        path: String,
        text: String,
    },
    Extension {
        path: String,
        value: Value,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SessionManifest {
    pub capsule_id: Uuid,
    pub schema_version: u32,
    pub minimum_reader_version: u32,
    pub producer: String,
    pub compiler_version: u32,
    pub created_at: DateTime<Utc>,
    pub tenant_id: String,
    pub session_id: Uuid,
    pub project_id: Option<String>,
    pub workspace_id: Option<String>,
    pub title: Option<String>,
    pub labels: Vec<String>,
    pub tags: BTreeMap<String, String>,
    pub session_state: SessionState,
    pub session_state_version: i64,
    pub session_created_at: DateTime<Utc>,
    pub session_updated_at: DateTime<Utc>,
    pub last_active_at: Option<DateTime<Utc>>,
    pub parent_session_id: Option<Uuid>,
    pub root_session_id: Uuid,
    pub fork_source_event_id: Option<Uuid>,
    pub active_task_id: Option<Uuid>,
    pub retention_class: Option<String>,
    pub data_classification: Option<String>,
    pub source_binding_id: Option<Uuid>,
    pub target_harness_id: String,
    pub handoff_id: Uuid,
    pub from_durable_sequence: i64,
    pub to_durable_sequence: i64,
    pub branch_id: Option<Uuid>,
    pub head_event_id: Option<Uuid>,
    pub event_ancestor_ids: Vec<Uuid>,
    pub workspace_snapshot_digests: Vec<String>,
    pub artifact_digests: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub local_service_bindings: Vec<LocalServiceBinding>,
    pub redaction_policy_id: String,
    pub redaction_policy_version: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SessionCapsule {
    pub manifest: SessionManifest,
    pub records: Vec<PortableRecord>,
    pub loss_report: LossReport,
    pub manifest_digest: String,
    pub record_digest: String,
    pub loss_report_digest: String,
    pub head_event_id: Option<Uuid>,
    pub event_ancestor_ids: Vec<Uuid>,
}

impl SessionCapsule {
    pub fn verify_integrity(&self) -> Result<(), CapsuleError> {
        if sha256_json(&self.manifest) != self.manifest_digest {
            return Err(CapsuleError::IntegrityMismatch("manifest".to_owned()));
        }
        if sha256_json(&self.records) != self.record_digest {
            return Err(CapsuleError::IntegrityMismatch("records".to_owned()));
        }
        if sha256_json(&self.loss_report) != self.loss_report_digest {
            return Err(CapsuleError::IntegrityMismatch("loss_report".to_owned()));
        }
        if self.manifest.head_event_id != self.head_event_id
            || self.manifest.event_ancestor_ids != self.event_ancestor_ids
        {
            return Err(CapsuleError::IntegrityMismatch("event_pointers".to_owned()));
        }
        let ancestor_ids = self
            .event_ancestor_ids
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        if ancestor_ids.len() != self.event_ancestor_ids.len()
            || self.event_ancestor_ids.iter().any(Uuid::is_nil)
            || self
                .head_event_id
                .is_some_and(|head_event_id| !ancestor_ids.contains(&head_event_id))
            || self.head_event_id.is_none() && !self.event_ancestor_ids.is_empty()
        {
            return Err(CapsuleError::IntegrityMismatch("event_pointers".to_owned()));
        }
        if self.manifest.tenant_id.trim().is_empty()
            || self.manifest.capsule_id.is_nil()
            || self.manifest.session_id.is_nil()
            || self.manifest.handoff_id.is_nil()
            || self.manifest.root_session_id.is_nil()
            || self.manifest.target_harness_id.trim().is_empty()
            || self.manifest.producer.trim().is_empty()
            || self.manifest.redaction_policy_id.trim().is_empty()
            || self.manifest.minimum_reader_version == 0
            || self.manifest.compiler_version == 0
            || self.manifest.schema_version == 0
            || self.manifest.redaction_policy_version == 0
            || self
                .manifest
                .source_binding_id
                .is_some_and(|binding_id| binding_id.is_nil())
            || self
                .manifest
                .branch_id
                .is_some_and(|branch_id| branch_id.is_nil())
            || self.manifest.from_durable_sequence < 0
            || self.manifest.to_durable_sequence < self.manifest.from_durable_sequence
        {
            return Err(CapsuleError::IntegrityMismatch(
                "manifest_identity".to_owned(),
            ));
        }
        let local_services = LocalServiceBundle {
            schema: LOCAL_SERVICE_BUNDLE_SCHEMA.to_owned(),
            bindings: self.manifest.local_service_bindings.clone(),
        };
        if local_services.validate().is_err()
            || local_services.bindings.iter().any(|binding| {
                binding.tenant_id != self.manifest.tenant_id
                    || binding.session_id != self.manifest.session_id
            })
        {
            return Err(CapsuleError::IntegrityMismatch(
                "local_service_bindings".to_owned(),
            ));
        }

        let mut session_record_seen = false;
        let mut task_ids = BTreeSet::new();
        let mut turn_ids = BTreeSet::new();
        let mut attempt_ids = BTreeSet::new();
        for record in &self.records {
            match record {
                PortableRecord::Session(session) => {
                    if session_record_seen
                        || session.session_id.is_nil()
                        || session.session_id != self.manifest.session_id
                        || session.tenant_id != self.manifest.tenant_id
                        || session.project_id != self.manifest.project_id
                        || session.workspace_id != self.manifest.workspace_id
                        || session.title != self.manifest.title
                        || session.labels != self.manifest.labels
                        || session.tags != self.manifest.tags
                        || session.state != self.manifest.session_state
                        || session.state_version != self.manifest.session_state_version
                        || session.parent_session_id != self.manifest.parent_session_id
                        || session.root_session_id != self.manifest.root_session_id
                        || session.fork_source_event_id != self.manifest.fork_source_event_id
                        || session.active_task_id != self.manifest.active_task_id
                        || session.created_at != self.manifest.session_created_at
                        || session.updated_at != self.manifest.session_updated_at
                        || session.last_active_at != self.manifest.last_active_at
                        || session.retention_class != self.manifest.retention_class
                        || session.data_classification != self.manifest.data_classification
                    {
                        return Err(CapsuleError::IntegrityMismatch(
                            "record_identity".to_owned(),
                        ));
                    }
                    session_record_seen = true;
                }
                PortableRecord::Task(task) => {
                    if task.task_id.is_nil()
                        || task.session_id != self.manifest.session_id
                        || !task_ids.insert(task.task_id)
                    {
                        return Err(CapsuleError::IntegrityMismatch(
                            "record_identity".to_owned(),
                        ));
                    }
                }
                PortableRecord::Turn(turn) => {
                    if turn.turn_id.is_nil()
                        || turn.session_id != self.manifest.session_id
                        || !turn_ids.insert(turn.turn_id)
                    {
                        return Err(CapsuleError::IntegrityMismatch(
                            "record_identity".to_owned(),
                        ));
                    }
                }
                PortableRecord::Attempt(attempt) => {
                    if attempt.attempt_id.is_nil()
                        || attempt.harness_id.trim().is_empty()
                        || attempt.session_id != self.manifest.session_id
                        || !attempt_ids.insert(attempt.attempt_id)
                    {
                        return Err(CapsuleError::IntegrityMismatch(
                            "record_identity".to_owned(),
                        ));
                    }
                }
                PortableRecord::Message(message) => {
                    if message.message_id.is_nil() || message.session_id != self.manifest.session_id
                    {
                        return Err(CapsuleError::IntegrityMismatch(
                            "record_identity".to_owned(),
                        ));
                    }
                }
                record if !typed_record_identity_is_valid(record, self.manifest.session_id) => {
                    return Err(CapsuleError::IntegrityMismatch(
                        "record_identity".to_owned(),
                    ));
                }
                _ => {}
            }
        }
        for record in &self.records {
            match record {
                PortableRecord::Turn(turn)
                    if (!task_ids.is_empty() && !task_ids.contains(&turn.task_id)) =>
                {
                    return Err(CapsuleError::IntegrityMismatch(
                        "record_identity".to_owned(),
                    ));
                }
                PortableRecord::Task(task)
                    if task.parent_task_id.is_some_and(|parent_task_id| {
                        !task_ids.is_empty() && !task_ids.contains(&parent_task_id)
                    }) || task.parent_attempt_id.is_some_and(|attempt_id| {
                        !attempt_ids.is_empty() && !attempt_ids.contains(&attempt_id)
                    }) || task.dependency_task_ids.iter().any(|dependency_task_id| {
                        !task_ids.is_empty() && !task_ids.contains(dependency_task_id)
                    }) =>
                {
                    return Err(CapsuleError::IntegrityMismatch(
                        "record_identity".to_owned(),
                    ));
                }
                PortableRecord::Attempt(attempt)
                    if (!task_ids.is_empty() && !task_ids.contains(&attempt.task_id))
                        || attempt.turn_id.is_some_and(|turn_id| {
                            !turn_ids.is_empty() && !turn_ids.contains(&turn_id)
                        })
                        || attempt.parent_attempt_id.is_some_and(|parent_id| {
                            !attempt_ids.is_empty() && !attempt_ids.contains(&parent_id)
                        }) =>
                {
                    return Err(CapsuleError::IntegrityMismatch(
                        "record_identity".to_owned(),
                    ));
                }
                PortableRecord::Message(message)
                    if message.task_id.is_some_and(|task_id| {
                        !task_ids.is_empty() && !task_ids.contains(&task_id)
                    }) || message.turn_id.is_some_and(|turn_id| {
                        !turn_ids.is_empty() && !turn_ids.contains(&turn_id)
                    }) =>
                {
                    return Err(CapsuleError::IntegrityMismatch(
                        "record_identity".to_owned(),
                    ));
                }
                _ => {}
            }
        }
        Ok(())
    }
}

fn typed_record_identity_is_valid(record: &PortableRecord, session_id: Uuid) -> bool {
    match record {
        PortableRecord::ToolDefinition(value) => !value.snapshot_id.is_nil(),
        PortableRecord::ToolCall(value) => {
            !value.tool_call_id.is_nil()
                && value.session_id == session_id
                && !value.task_id.is_nil()
                && !value.attempt_id.is_nil()
        }
        PortableRecord::ToolProgress(value) => {
            !value.progress_id.is_nil() && !value.tool_call_id.is_nil()
        }
        PortableRecord::ToolResultRecord(value) => {
            !value.tool_call_id.is_nil()
                && value.session_id == session_id
                && !value.task_id.is_nil()
                && !value.attempt_id.is_nil()
        }
        PortableRecord::TypedInteraction(value) => {
            !value.interaction_id.is_nil() && value.session_id == session_id
        }
        PortableRecord::TypedPlan(value) => {
            !value.plan_id.is_nil() && value.session_id == session_id
        }
        PortableRecord::TypedTodo(value) => {
            !value.todo_id.is_nil() && value.session_id == session_id
        }
        PortableRecord::TypedGoal(value) => {
            !value.goal_id.is_nil() && value.session_id == session_id
        }
        PortableRecord::TypedProcess(value) => {
            !value.process_id.is_nil()
                && value.session_id == session_id
                && !value.task_id.is_nil()
                && !value.attempt_id.is_nil()
        }
        PortableRecord::TypedProcessChunk(value) => {
            !value.chunk_id.is_nil() && !value.process_id.is_nil()
        }
        PortableRecord::TypedWorkspaceDescriptor(value) => {
            !value.descriptor_id.is_nil() && value.session_id == session_id
        }
        PortableRecord::TypedWorkspaceSnapshot(value) => {
            !value.snapshot_id.is_nil() && value.session_id == session_id
        }
        PortableRecord::RuntimeConfigSnapshot(value) => !value.snapshot_id.is_nil(),
        PortableRecord::Usage(value) => !value.usage_id.is_nil() && value.session_id == session_id,
        PortableRecord::Error(value) => !value.error_id.is_nil() && value.session_id == session_id,
        PortableRecord::Compaction(value) => {
            !value.compaction_id.is_nil() && value.session_id == session_id
        }
        PortableRecord::CapabilitySnapshot(value) => !value.snapshot_id.is_nil(),
        _ => true,
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum CapsuleError {
    UnsafeLoss(Vec<LossEntry>),
    RequiredLoss(Vec<LossEntry>),
    EventClosure(String),
    IntegrityMismatch(String),
}

#[derive(Debug)]
pub struct CapsuleCompileInput {
    pub capsule_id: Uuid,
    pub handoff_id: Uuid,
    pub session: Session,
    pub source_binding_id: Option<Uuid>,
    pub target_harness_id: String,
    pub from_durable_sequence: i64,
    pub to_durable_sequence: i64,
    pub branch_id: Option<Uuid>,
    pub head_event_id: Option<Uuid>,
    pub event_ancestor_ids: Vec<Uuid>,
    pub records: Vec<CanonicalRecord>,
    pub workspace_snapshot_digests: Vec<String>,
    pub artifact_digests: Vec<String>,
    pub local_service_bindings: Vec<LocalServiceBinding>,
    pub created_at: DateTime<Utc>,
}

impl Clone for CapsuleCompileInput {
    fn clone(&self) -> Self {
        Self {
            capsule_id: self.capsule_id,
            handoff_id: self.handoff_id,
            session: self.session.clone(),
            source_binding_id: self.source_binding_id,
            target_harness_id: self.target_harness_id.clone(),
            from_durable_sequence: self.from_durable_sequence,
            to_durable_sequence: self.to_durable_sequence,
            branch_id: self.branch_id,
            head_event_id: self.head_event_id,
            event_ancestor_ids: self.event_ancestor_ids.clone(),
            records: self.records.clone(),
            workspace_snapshot_digests: self.workspace_snapshot_digests.clone(),
            artifact_digests: self.artifact_digests.clone(),
            local_service_bindings: self.local_service_bindings.clone(),
            created_at: self.created_at,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CapsuleCompiler {
    producer: String,
    redaction_policy_id: String,
    redaction_policy_version: u32,
}

impl CapsuleCompiler {
    pub fn new(
        producer: impl Into<String>,
        redaction_policy_id: impl Into<String>,
        redaction_policy_version: u32,
    ) -> Self {
        Self {
            producer: producer.into(),
            redaction_policy_id: redaction_policy_id.into(),
            redaction_policy_version,
        }
    }

    pub fn compile(&self, input: CapsuleCompileInput) -> Result<SessionCapsule, CapsuleError> {
        let mut losses = Vec::new();
        if !input.session.extensions.is_empty() {
            let _ = redact_json(
                "session.extensions",
                &Value::Object(input.session.extensions.clone().into_iter().collect()),
                &mut losses,
            );
            losses.push(LossEntry {
                source_path: "session.extensions".to_owned(),
                reason: "unreviewed extensions are excluded from portable capsules".to_owned(),
                severity: LossSeverity::Warning,
                target_representation: None,
                acknowledgement_required: false,
                capability: Some("portable_extensions".to_owned()),
            });
        }

        let portable_session = project_session(&input.session, "session", &mut losses);
        let mut records = Vec::new();
        for (index, record) in input.records.iter().enumerate() {
            let path = format!("records[{index}]");
            match project_record(record, &path, &mut losses) {
                Ok(Some(record)) => records.push(record),
                Ok(None) => {}
                Err(RecordProjectionError::Unsafe(loss)) => losses.push(loss),
            }
        }

        let report = LossReport::new(losses);
        let unsafe_entries = report
            .entries
            .iter()
            .filter(|entry| matches!(entry.severity, LossSeverity::Unsafe))
            .cloned()
            .collect::<Vec<_>>();
        if !unsafe_entries.is_empty() {
            return Err(CapsuleError::UnsafeLoss(unsafe_entries));
        }
        let required_entries = report
            .entries
            .iter()
            .filter(|entry| matches!(entry.severity, LossSeverity::Required))
            .cloned()
            .collect::<Vec<_>>();
        if !required_entries.is_empty() {
            return Err(CapsuleError::RequiredLoss(required_entries));
        }

        let manifest = SessionManifest {
            capsule_id: input.capsule_id,
            schema_version: 2,
            minimum_reader_version: 2,
            producer: self.producer.clone(),
            compiler_version: 2,
            created_at: input.created_at,
            tenant_id: portable_session.tenant_id.clone(),
            session_id: portable_session.session_id,
            project_id: portable_session.project_id.clone(),
            workspace_id: portable_session.workspace_id.clone(),
            title: portable_session.title.clone(),
            labels: portable_session.labels.clone(),
            tags: portable_session.tags.clone(),
            session_state: portable_session.state.clone(),
            session_state_version: portable_session.state_version,
            session_created_at: portable_session.created_at,
            session_updated_at: portable_session.updated_at,
            last_active_at: portable_session.last_active_at,
            parent_session_id: portable_session.parent_session_id,
            root_session_id: portable_session.root_session_id,
            fork_source_event_id: portable_session.fork_source_event_id,
            active_task_id: portable_session.active_task_id,
            retention_class: portable_session.retention_class.clone(),
            data_classification: portable_session.data_classification.clone(),
            source_binding_id: input.source_binding_id,
            target_harness_id: input.target_harness_id,
            handoff_id: input.handoff_id,
            from_durable_sequence: input.from_durable_sequence,
            to_durable_sequence: input.to_durable_sequence,
            branch_id: input.branch_id,
            head_event_id: input.head_event_id,
            event_ancestor_ids: input.event_ancestor_ids.clone(),
            workspace_snapshot_digests: input.workspace_snapshot_digests,
            artifact_digests: input.artifact_digests,
            local_service_bindings: input.local_service_bindings,
            redaction_policy_id: self.redaction_policy_id.clone(),
            redaction_policy_version: self.redaction_policy_version,
        };
        let capsule = SessionCapsule {
            manifest_digest: sha256_json(&manifest),
            record_digest: sha256_json(&records),
            loss_report_digest: sha256_json(&report),
            head_event_id: input.head_event_id,
            event_ancestor_ids: input.event_ancestor_ids,
            manifest,
            records,
            loss_report: report,
        };
        capsule.verify_integrity()?;
        Ok(capsule)
    }

    pub fn compile_from_event_head(
        &self,
        mut input: CapsuleCompileInput,
        store: &EventStore,
        head_event_id: Uuid,
    ) -> Result<SessionCapsule, CapsuleError> {
        let closure = store
            .ancestor_closure(input.session.session_id, head_event_id)
            .map_err(|error| CapsuleError::EventClosure(format!("{error:?}")))?;
        input.head_event_id = Some(head_event_id);
        input.event_ancestor_ids = closure.into_iter().map(|event| event.event_id).collect();
        self.compile(input)
    }
}

#[derive(Clone, Debug)]
enum RecordProjectionError {
    Unsafe(LossEntry),
}

fn project_session(session: &Session, path: &str, losses: &mut Vec<LossEntry>) -> PortableSession {
    PortableSession {
        session_id: session.session_id,
        tenant_id: session.tenant_id.clone(),
        project_id: session.project_id.clone(),
        workspace_id: session.workspace_id.clone(),
        title: session
            .title
            .as_ref()
            .map(|title| redact_text(&format!("{path}.title"), title, losses)),
        labels: session
            .labels
            .iter()
            .enumerate()
            .map(|(index, label)| redact_text(&format!("{path}.labels[{index}]"), label, losses))
            .collect(),
        tags: redact_string_map(&format!("{path}.tags"), &session.tags, losses),
        state: session.state.clone(),
        state_version: session.state_version,
        parent_session_id: session.parent_session_id,
        root_session_id: session.root_session_id,
        fork_source_event_id: session.fork_source_event_id,
        active_task_id: session.active_task_id,
        created_at: session.created_at,
        updated_at: session.updated_at,
        last_active_at: session.last_active_at,
        retention_class: session.retention_class.clone(),
        data_classification: session.data_classification.clone(),
    }
}

fn project_record(
    record: &CanonicalRecord,
    path: &str,
    losses: &mut Vec<LossEntry>,
) -> Result<Option<PortableRecord>, RecordProjectionError> {
    match record {
        CanonicalRecord::Session(session) => Ok(Some(PortableRecord::Session(project_session(
            session, path, losses,
        )))),
        CanonicalRecord::Task(task) => Ok(Some(PortableRecord::Task(PortableTask {
            task_id: task.task_id,
            session_id: task.session_id,
            parent_task_id: task.parent_task_id,
            parent_attempt_id: task.parent_attempt_id,
            kind: task.kind.clone(),
            objective: redact_text(&format!("{path}.objective"), &task.objective, losses),
            state: task.state.clone(),
            state_version: task.state_version,
            dependency_task_ids: task.dependency_task_ids.clone(),
            owner_actor: task
                .owner_actor
                .as_ref()
                .map(super::types::HistoricalActor::from),
            input_message_ids: task.input_message_ids.clone(),
            output_message_ids: task.output_message_ids.clone(),
            artifact_ids: task.artifact_ids.clone(),
            terminal_result: task.terminal_result.as_ref().map(PortableTaskResult::from),
            created_at: task.created_at,
            updated_at: task.updated_at,
            started_at: task.started_at,
            finished_at: task.finished_at,
        }))),
        CanonicalRecord::Turn(turn) => Ok(Some(PortableRecord::Turn(project_turn(turn)))),
        CanonicalRecord::Attempt(attempt) => Ok(Some(PortableRecord::Attempt(project_attempt(
            attempt, path, losses,
        )))),
        CanonicalRecord::Message(message) => Ok(Some(PortableRecord::Message(project_message(
            message, path, losses,
        )))),
        CanonicalRecord::Artifact(artifact) => Ok(Some(PortableRecord::Artifact(
            project_artifact(artifact, path, losses),
        ))),
        CanonicalRecord::RuntimeConfig(config) => Ok(Some(PortableRecord::RuntimeConfig(
            project_runtime_config(config, path, losses),
        ))),
        CanonicalRecord::ToolResult {
            tool_call_id,
            content,
            metadata,
        } => Ok(Some(PortableRecord::ToolResult(PortableToolResult {
            tool_call_id: *tool_call_id,
            content: project_content(content, path, losses),
            metadata: redact_string_map(&format!("{path}.metadata"), metadata, losses),
        }))),
        CanonicalRecord::ToolDefinition(value) => Ok(project_typed_record(
            value,
            path,
            losses,
            PortableRecord::ToolDefinition,
        )),
        CanonicalRecord::ToolCall(value) => Ok(project_typed_record(
            value,
            path,
            losses,
            PortableRecord::ToolCall,
        )),
        CanonicalRecord::ToolProgress(value) => Ok(project_typed_record(
            value,
            path,
            losses,
            PortableRecord::ToolProgress,
        )),
        CanonicalRecord::ToolResultRecord(value) => Ok(project_typed_record(
            value,
            path,
            losses,
            PortableRecord::ToolResultRecord,
        )),
        CanonicalRecord::Interaction(value) => Ok(project_typed_record(
            value,
            path,
            losses,
            PortableRecord::TypedInteraction,
        )),
        CanonicalRecord::Plan(value) => Ok(project_typed_record(
            value,
            path,
            losses,
            PortableRecord::TypedPlan,
        )),
        CanonicalRecord::Todo(value) => Ok(project_typed_record(
            value,
            path,
            losses,
            PortableRecord::TypedTodo,
        )),
        CanonicalRecord::Goal(value) => Ok(project_typed_record(
            value,
            path,
            losses,
            PortableRecord::TypedGoal,
        )),
        CanonicalRecord::Process(value) => Ok(project_typed_record(
            value,
            path,
            losses,
            PortableRecord::TypedProcess,
        )),
        CanonicalRecord::ProcessChunk(value) => Ok(project_typed_record(
            value,
            path,
            losses,
            PortableRecord::TypedProcessChunk,
        )),
        CanonicalRecord::WorkspaceDescriptor(value) => Ok(project_typed_record(
            value,
            path,
            losses,
            PortableRecord::TypedWorkspaceDescriptor,
        )),
        CanonicalRecord::WorkspaceSnapshot(value) => Ok(project_typed_record(
            value,
            path,
            losses,
            PortableRecord::TypedWorkspaceSnapshot,
        )),
        CanonicalRecord::RuntimeConfigSnapshot(value) => Ok(project_typed_record(
            value,
            path,
            losses,
            PortableRecord::RuntimeConfigSnapshot,
        )),
        CanonicalRecord::Usage(value) => Ok(project_typed_record(
            value,
            path,
            losses,
            PortableRecord::Usage,
        )),
        CanonicalRecord::Error(value) => Ok(project_typed_record(
            value,
            path,
            losses,
            PortableRecord::Error,
        )),
        CanonicalRecord::Compaction(value) => Ok(project_typed_record(
            value,
            path,
            losses,
            PortableRecord::Compaction,
        )),
        CanonicalRecord::CapabilitySnapshot(value) => Ok(project_typed_record(
            value,
            path,
            losses,
            PortableRecord::CapabilitySnapshot,
        )),
        CanonicalRecord::NativeRecordSet(_) | CanonicalRecord::NativeRecordTyped(_) => {
            losses.push(LossEntry {
                source_path: path.to_owned(),
                reason: "native record sets are excluded from cross-harness capsules".to_owned(),
                severity: LossSeverity::Info,
                target_representation: None,
                acknowledgement_required: false,
                capability: Some("native_rehydration".to_owned()),
            });
            Ok(None)
        }
        CanonicalRecord::ContextCheckpoint(checkpoint) => Ok(Some(
            PortableRecord::ContextCheckpoint(project_checkpoint(checkpoint, path, losses)),
        )),
        CanonicalRecord::NativeRecord { .. } => {
            losses.push(LossEntry {
                source_path: path.to_owned(),
                reason: "native records are excluded from cross-harness capsules".to_owned(),
                severity: LossSeverity::Info,
                target_representation: None,
                acknowledgement_required: false,
                capability: Some("native_rehydration".to_owned()),
            });
            Ok(None)
        }
        CanonicalRecord::Unsupported { kind, summary } => {
            losses.push(LossEntry {
                source_path: path.to_owned(),
                reason: format!("unsupported record rendered as historical data: {kind}"),
                severity: LossSeverity::Warning,
                target_representation: Some("historical_data".to_owned()),
                acknowledgement_required: false,
                capability: Some(kind.clone()),
            });
            Ok(Some(PortableRecord::HistoricalData(HistoricalData {
                source_kind: kind.clone(),
                summary: redact_text(&format!("{path}.summary"), summary, losses),
            })))
        }
        CanonicalRecord::RawReasoning { path, .. } => {
            Err(RecordProjectionError::Unsafe(LossEntry {
                source_path: path.clone(),
                reason: "raw chain-of-thought is never portable".to_owned(),
                severity: LossSeverity::Unsafe,
                target_representation: None,
                acknowledgement_required: true,
                capability: Some("reasoning_summary".to_owned()),
            }))
        }
        CanonicalRecord::Extension { path, value } => {
            let _ = redact_json(path, value, losses);
            losses.push(LossEntry {
                source_path: path.clone(),
                reason: "unreviewed extensions are excluded from portable capsules".to_owned(),
                severity: LossSeverity::Warning,
                target_representation: None,
                acknowledgement_required: false,
                capability: Some("portable_extensions".to_owned()),
            });
            Ok(None)
        }
    }
}

fn project_typed_record<T, P, F>(
    record: &T,
    path: &str,
    losses: &mut Vec<LossEntry>,
    constructor: F,
) -> Option<PortableRecord>
where
    T: Serialize,
    P: DeserializeOwned,
    F: FnOnce(P) -> PortableRecord,
{
    // All canonical typed records are serde-derived; redaction must preserve
    // their schema before the target-specific deserialization below. A
    // redaction that removes a required field is a required handoff loss.
    let value = serde_json::to_value(record).expect("canonical typed record must serialize");
    let redacted = redact_json(path, &value, losses);
    let value = match serde_json::from_value(redacted) {
        Ok(value) => value,
        Err(error) => {
            losses.push(LossEntry {
                source_path: path.to_owned(),
                reason: format!(
                    "typed record cannot be projected after portable redaction: {error}"
                ),
                severity: LossSeverity::Required,
                target_representation: Some("historical_data".to_owned()),
                acknowledgement_required: true,
                capability: Some("typed_record_projection".to_owned()),
            });
            return None;
        }
    };
    Some(constructor(value))
}

fn project_attempt(attempt: &Attempt, path: &str, losses: &mut Vec<LossEntry>) -> PortableAttempt {
    PortableAttempt {
        attempt_id: attempt.attempt_id,
        session_id: attempt.session_id,
        task_id: attempt.task_id,
        turn_id: attempt.turn_id,
        parent_attempt_id: attempt.parent_attempt_id,
        kind: attempt.kind.clone(),
        state: attempt.state.clone(),
        state_version: attempt.state_version,
        actor: attempt
            .actor
            .as_ref()
            .map(super::types::HistoricalActor::from),
        harness_id: attempt.harness_id.clone(),
        model_runtime_id: attempt.model_runtime_id.clone(),
        worker_id: attempt.worker_id.clone(),
        runtime_config_snapshot_id: attempt.runtime_config_snapshot_id,
        metadata: redact_json_map(&format!("{path}.metadata"), &attempt.metadata, losses),
        started_at: attempt.started_at,
        finished_at: attempt.finished_at,
    }
}

fn project_turn(turn: &Turn) -> PortableTurn {
    PortableTurn {
        turn_id: turn.turn_id,
        task_id: turn.task_id,
        session_id: turn.session_id,
        sequence: turn.sequence,
        state: turn.state.clone(),
        state_version: turn.state_version,
        input_message_id: turn.input_message_id,
        output_message_ids: turn.output_message_ids.clone(),
        stop_reason: turn.stop_reason.clone(),
        error_code: turn.error_code.clone(),
        created_at: turn.created_at,
        updated_at: turn.updated_at,
        completed_at: turn.completed_at,
    }
}

fn project_message(message: &Message, path: &str, losses: &mut Vec<LossEntry>) -> PortableMessage {
    PortableMessage {
        message_id: message.message_id,
        session_id: message.session_id,
        task_id: message.task_id,
        turn_id: message.turn_id,
        role: message.role.clone(),
        actor: message
            .author
            .as_ref()
            .map(super::types::HistoricalActor::from),
        origin: message.origin.clone(),
        phase: message.phase.clone(),
        parent_message_id: message.parent_message_id,
        correlation_id: message.correlation_id,
        status: message.status.clone(),
        content: project_content(&message.content, path, losses),
        visible_to_user: message.visible_to_user,
        redaction_state: message.redaction_state.clone(),
        created_at: message.created_at,
        settled_at: message.settled_at,
    }
}

fn project_artifact(
    artifact: &ArtifactRef,
    path: &str,
    losses: &mut Vec<LossEntry>,
) -> PortableArtifact {
    let artifact = project_artifact_ref(artifact, path, losses);
    PortableArtifact {
        artifact_id: artifact.artifact_id,
        kind: artifact.kind.clone(),
        name: artifact.name,
        media_type: artifact.media_type.clone(),
        byte_length: artifact.byte_length,
        sha256: artifact.sha256.clone(),
        metadata: artifact.metadata,
    }
}

fn project_artifact_ref(
    artifact: &ArtifactRef,
    path: &str,
    losses: &mut Vec<LossEntry>,
) -> ArtifactRef {
    let mut projected = artifact.clone();
    projected.name = redact_text(&format!("{path}.name"), &artifact.name, losses);
    projected.metadata = redact_string_map(&format!("{path}.metadata"), &artifact.metadata, losses);
    projected.uri = artifact.uri.as_ref().map(|uri| {
        let sanitized = redact_text(&format!("{path}.uri"), uri, losses);
        if sanitized == "[REDACTED]" {
            String::new()
        } else {
            sanitized
        }
    });
    if projected.uri.as_deref() == Some("") {
        projected.uri = None;
    }
    projected
}

fn project_runtime_config(
    config: &RuntimeConfigInput,
    path: &str,
    losses: &mut Vec<LossEntry>,
) -> PortableRuntimeConfig {
    let mut instruction_layers = Vec::new();
    for (authority, instructions) in [
        ("system", &config.system_instructions),
        ("developer", &config.developer_instructions),
        ("user", &config.user_instructions),
    ] {
        for (index, instruction) in instructions.iter().enumerate() {
            instruction_layers.push(PortableInstruction {
                authority: authority.to_owned(),
                text: redact_text(
                    &format!("{path}.{authority}_instructions[{index}]"),
                    instruction,
                    losses,
                ),
            });
        }
    }
    if !config.extensions.is_empty() {
        let _ = redact_json(
            &format!("{path}.extensions"),
            &Value::Object(config.extensions.clone().into_iter().collect()),
            losses,
        );
        losses.push(LossEntry {
            source_path: format!("{path}.extensions"),
            reason: "runtime extensions require target policy review".to_owned(),
            severity: LossSeverity::Warning,
            target_representation: None,
            acknowledgement_required: false,
            capability: Some("portable_runtime_extensions".to_owned()),
        });
    }
    PortableRuntimeConfig {
        snapshot_id: config.snapshot_id,
        instruction_layers,
        requested_model: project_model_descriptor(
            &config.requested_model,
            &format!("{path}.requested_model"),
            losses,
        ),
        response_schema: config
            .response_schema
            .as_ref()
            .map(|value| redact_json(&format!("{path}.response_schema"), value, losses)),
        tool_definition_digests: config.tool_definition_digests.clone(),
    }
}

fn project_content(
    content: &[ContentPart],
    path: &str,
    losses: &mut Vec<LossEntry>,
) -> Vec<ContentPart> {
    content
        .iter()
        .enumerate()
        .filter_map(|(index, part)| {
            let part_path = format!("{path}.content[{index}]");
            match part {
                ContentPart::Text { text, annotations } => Some(ContentPart::Text {
                    text: redact_text(&part_path, text, losses),
                    annotations: project_annotations(annotations, &part_path, losses),
                }),
                ContentPart::ReasoningSummary { text, annotations } => {
                    Some(ContentPart::ReasoningSummary {
                        text: redact_text(&part_path, text, losses),
                        annotations: project_annotations(annotations, &part_path, losses),
                    })
                }
                ContentPart::EmbeddedResource { media_type, data } => {
                    Some(ContentPart::EmbeddedResource {
                        media_type: media_type.clone(),
                        data: redact_text(&part_path, data, losses),
                    })
                }
                ContentPart::StructuredJson { value, schema_ref } => {
                    Some(ContentPart::StructuredJson {
                        value: redact_json(&part_path, value, losses),
                        schema_ref: schema_ref.clone(),
                    })
                }
                ContentPart::Native { .. } => {
                    losses.push(LossEntry {
                        source_path: part_path,
                        reason: "native content parts are excluded".to_owned(),
                        severity: LossSeverity::Info,
                        target_representation: None,
                        acknowledgement_required: false,
                        capability: Some("native_content".to_owned()),
                    });
                    None
                }
                ContentPart::File { artifact } => Some(ContentPart::File {
                    artifact: project_artifact_ref(artifact, &part_path, losses),
                }),
                ContentPart::Image { artifact } => Some(ContentPart::Image {
                    artifact: project_artifact_ref(artifact, &part_path, losses),
                }),
                ContentPart::Audio { artifact } => Some(ContentPart::Audio {
                    artifact: project_artifact_ref(artifact, &part_path, losses),
                }),
                ContentPart::Video { artifact } => Some(ContentPart::Video {
                    artifact: project_artifact_ref(artifact, &part_path, losses),
                }),
                ContentPart::Artifact { artifact } => Some(ContentPart::Artifact {
                    artifact: project_artifact_ref(artifact, &part_path, losses),
                }),
                ContentPart::ResourceLink {
                    uri,
                    name,
                    media_type,
                } => Some(ContentPart::ResourceLink {
                    uri: redact_text(&format!("{part_path}.uri"), uri, losses),
                    name: name.clone(),
                    media_type: media_type.clone(),
                }),
                ContentPart::Citation {
                    uri,
                    title,
                    locator,
                } => Some(ContentPart::Citation {
                    uri: redact_text(&format!("{part_path}.uri"), uri, losses),
                    title: title.clone(),
                    locator: locator.clone(),
                }),
                ContentPart::WorkspacePath { snapshot_id, path } if path.starts_with('/') => {
                    losses.push(LossEntry {
                        source_path: format!("{part_path}.path"),
                        reason: "host absolute paths are not portable identities".to_owned(),
                        severity: LossSeverity::Warning,
                        target_representation: Some("workspace_relative_path".to_owned()),
                        acknowledgement_required: false,
                        capability: Some("workspace_snapshots".to_owned()),
                    });
                    Some(ContentPart::WorkspacePath {
                        snapshot_id: *snapshot_id,
                        path: "[WORKSPACE_PATH]".to_owned(),
                    })
                }
                other => Some(other.clone()),
            }
        })
        .collect()
}

fn redact_string_map(
    path: &str,
    values: &BTreeMap<String, String>,
    losses: &mut Vec<LossEntry>,
) -> BTreeMap<String, String> {
    values
        .iter()
        .filter_map(|(key, value)| {
            if non_portable_json_key(key) {
                record_nonportable_key(&format!("{path}.{key}"), losses);
                None
            } else {
                Some((
                    key.clone(),
                    redact_text(&format!("{path}.{key}"), value, losses),
                ))
            }
        })
        .collect()
}

fn redact_json_map(path: &str, values: &JsonMap, losses: &mut Vec<LossEntry>) -> JsonMap {
    values
        .iter()
        .filter_map(|(key, value)| {
            if non_portable_json_key(key) {
                record_nonportable_key(&format!("{path}.{key}"), losses);
                None
            } else {
                Some((
                    key.clone(),
                    redact_json(&format!("{path}.{key}"), value, losses),
                ))
            }
        })
        .collect()
}

fn project_annotations(
    annotations: &[ContentAnnotation],
    path: &str,
    losses: &mut Vec<LossEntry>,
) -> Vec<ContentAnnotation> {
    annotations
        .iter()
        .enumerate()
        .map(|(index, annotation)| {
            let annotation_path = format!("{path}.annotations[{index}]");
            let mut projected = annotation.clone();
            projected.annotation_type = redact_text(
                &format!("{annotation_path}.annotation_type"),
                &annotation.annotation_type,
                losses,
            );
            projected.data =
                redact_json_map(&format!("{annotation_path}.data"), &annotation.data, losses);
            projected
        })
        .collect()
}

fn project_model_descriptor(
    model: &ModelDescriptor,
    path: &str,
    losses: &mut Vec<LossEntry>,
) -> ModelDescriptor {
    let mut projected = model.clone();
    projected.family = model
        .family
        .as_ref()
        .map(|family| redact_text(&format!("{path}.family"), family, losses));
    projected.quantization = model
        .quantization
        .as_ref()
        .map(|value| redact_text(&format!("{path}.quantization"), value, losses));
    projected.license = model
        .license
        .as_ref()
        .map(|value| redact_text(&format!("{path}.license"), value, losses));
    projected.provenance = model
        .provenance
        .as_ref()
        .map(|value| redact_text(&format!("{path}.provenance"), value, losses));
    projected.metadata = redact_json_map(&format!("{path}.metadata"), &model.metadata, losses);
    projected
}

fn project_checkpoint(
    checkpoint: &PortableContextCheckpoint,
    path: &str,
    losses: &mut Vec<LossEntry>,
) -> PortableContextCheckpoint {
    let mut projected = checkpoint.clone();
    projected.summary = redact_text(&format!("{path}.summary"), &checkpoint.summary, losses);
    projected.usage = redact_json_map(&format!("{path}.usage"), &checkpoint.usage, losses);
    projected.integrity_digest = sha256_json(&(
        &projected.summary,
        &projected.selected_message_ids,
        &projected.source_durable_ranges,
        projected.retained_ancestor_event_id,
        &projected.compaction_provenance,
        &projected.usage,
    ));
    projected
}

fn redact_text(path: &str, text: &str, losses: &mut Vec<LossEntry>) -> String {
    if !sensitive_json_text(text) {
        return text.to_owned();
    }
    losses.push(LossEntry {
        source_path: path.to_owned(),
        reason: "secret-like content redacted".to_owned(),
        severity: LossSeverity::Warning,
        target_representation: Some("redacted".to_owned()),
        acknowledgement_required: false,
        capability: Some("secret_redaction".to_owned()),
    });
    "[REDACTED]".to_owned()
}

fn redact_json(path: &str, value: &Value, losses: &mut Vec<LossEntry>) -> Value {
    match value {
        Value::String(text) => Value::String(redact_text(path, text, losses)),
        Value::Array(values) => Value::Array(
            values
                .iter()
                .enumerate()
                .map(|(index, value)| redact_json(&format!("{path}[{index}]"), value, losses))
                .collect(),
        ),
        Value::Object(values) => Value::Object(
            values
                .iter()
                .filter_map(|(key, value)| {
                    if non_portable_json_key(key) {
                        record_nonportable_key(&format!("{path}.{key}"), losses);
                        None
                    } else {
                        Some((
                            key.clone(),
                            redact_json(&format!("{path}.{key}"), value, losses),
                        ))
                    }
                })
                .collect::<Map<String, Value>>(),
        ),
        other => other.clone(),
    }
}

fn record_nonportable_key(path: &str, losses: &mut Vec<LossEntry>) {
    losses.push(LossEntry {
        source_path: path.to_owned(),
        reason: "authority, credential, or live runtime state is not portable".to_owned(),
        severity: LossSeverity::Info,
        target_representation: None,
        acknowledgement_required: false,
        capability: Some("portable_redaction".to_owned()),
    });
}

fn non_portable_json_key(key: &str) -> bool {
    sensitive_json_key(key)
        || matches!(
            key,
            "native_aliases"
                | "native_provenance"
                | "native_session_id"
                | "native_cursor"
                | "resume_token"
                | "fencing_token"
                | "lease_id"
                | "process_handle"
                | "pid"
                | "pty"
        )
}

fn sha256_json<T: Serialize>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("portable values must serialize");
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffScope {
    Session,
    Task { task_id: Uuid },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HandoffOperation {
    pub operation_id: Uuid,
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub source_binding_id: Uuid,
    pub target_harness_id: String,
    pub scope: HandoffScope,
    pub state: HandoffState,
    pub state_version: i64,
    pub loss_report: LossReport,
    pub loss_ack_digest: Option<String>,
    pub target_binding_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

impl HandoffOperation {
    pub fn new(
        operation_id: Uuid,
        session_id: Uuid,
        task_id: Option<Uuid>,
        source_binding_id: Uuid,
        target_harness_id: String,
        scope: HandoffScope,
        loss_report: LossReport,
    ) -> Self {
        Self {
            operation_id,
            session_id,
            task_id,
            source_binding_id,
            target_harness_id,
            scope,
            state: HandoffState::Requested,
            state_version: 0,
            loss_report,
            loss_ack_digest: None,
            target_binding_id: None,
            created_at: Utc::now(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum HandoffError {
    OperationNotFound,
    OperationAlreadyExists,
    BindingNotFound,
    BindingAlreadyExists,
    InvalidBinding,
    InvalidScope,
    SourceNotActive,
    SourceStillWritable,
    DuplicateWritableBinding,
    BindingMismatch,
    TargetMissing,
    LossAcknowledgementRequired,
    LossDigestMismatch,
    InvalidPhase,
    Terminal,
    Lifecycle(LifecycleError),
}

#[derive(Clone, Debug, Default)]
pub struct HandoffCoordinator {
    bindings: HashMap<Uuid, SessionBinding>,
    operations: HashMap<Uuid, HandoffOperation>,
}

impl HandoffCoordinator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_binding(&mut self, binding: SessionBinding) -> Result<(), HandoffError> {
        if binding.binding_id.is_nil()
            || binding.session_id.is_nil()
            || binding.harness_id.trim().is_empty()
            || binding.workspace_mutation_scope_id.trim().is_empty()
            || binding.generation <= 0
            || matches!(
                (&binding.scope, binding.task_id),
                (BindingScope::Session, Some(_)) | (BindingScope::Task, None)
            )
        {
            return Err(HandoffError::InvalidBinding);
        }
        if self.bindings.contains_key(&binding.binding_id) {
            return Err(HandoffError::BindingAlreadyExists);
        }
        if binding.state == BindingState::Active
            && binding.access_mode == BindingAccessMode::ReadWrite
            && self.bindings.values().any(|existing| {
                existing.binding_id != binding.binding_id
                    && existing.session_id == binding.session_id
                    && existing.workspace_mutation_scope_id == binding.workspace_mutation_scope_id
                    && existing.state == BindingState::Active
                    && existing.access_mode == BindingAccessMode::ReadWrite
            })
        {
            return Err(HandoffError::DuplicateWritableBinding);
        }
        self.bindings.insert(binding.binding_id, binding);
        Ok(())
    }

    pub fn begin(&mut self, operation: HandoffOperation) -> Result<Uuid, HandoffError> {
        if operation.operation_id.is_nil() || operation.target_harness_id.trim().is_empty() {
            return Err(HandoffError::InvalidScope);
        }
        if self.operations.contains_key(&operation.operation_id) {
            return Err(HandoffError::OperationAlreadyExists);
        }
        let source = self
            .bindings
            .get(&operation.source_binding_id)
            .ok_or(HandoffError::BindingNotFound)?;
        if source.session_id != operation.session_id {
            return Err(HandoffError::BindingMismatch);
        }
        if source.state != BindingState::Active {
            return Err(HandoffError::SourceNotActive);
        }
        let scoped_task_id = match &operation.scope {
            HandoffScope::Session => None,
            HandoffScope::Task { task_id } => Some(*task_id),
        };
        if operation.task_id != scoped_task_id
            || source.task_id != operation.task_id
            || (matches!(&operation.scope, HandoffScope::Session)
                && source.scope != BindingScope::Session)
            || (matches!(&operation.scope, HandoffScope::Task { .. })
                && source.scope != BindingScope::Task)
        {
            return Err(HandoffError::InvalidScope);
        }
        if operation.state != HandoffState::Requested || operation.state_version != 0 {
            return Err(HandoffError::InvalidPhase);
        }
        let operation_id = operation.operation_id;
        self.operations.insert(operation_id, operation);
        Ok(operation_id)
    }

    pub fn advance(
        &mut self,
        operation_id: Uuid,
        expected_version: i64,
        next: HandoffState,
    ) -> Result<(), HandoffError> {
        let operation = self
            .operations
            .get(&operation_id)
            .ok_or(HandoffError::OperationNotFound)?
            .clone();
        if next == HandoffState::TargetCreating
            && operation.loss_report.requires_ack()
            && operation.loss_ack_digest.as_deref() != Some(operation.loss_report.digest().as_str())
        {
            return Err(HandoffError::LossAcknowledgementRequired);
        }
        if next == HandoffState::Activating && operation.target_binding_id.is_none() {
            return Err(HandoffError::TargetMissing);
        }
        if next == HandoffState::Completed {
            let target_id = operation
                .target_binding_id
                .ok_or(HandoffError::TargetMissing)?;
            let target = self
                .bindings
                .get(&target_id)
                .ok_or(HandoffError::BindingNotFound)?;
            if target.state != BindingState::Active {
                return Err(HandoffError::TargetMissing);
            }
        }
        if next == HandoffState::Snapshotting {
            let source = self
                .bindings
                .get(&operation.source_binding_id)
                .ok_or(HandoffError::BindingNotFound)?;
            if source.state != BindingState::Quiescing {
                return Err(HandoffError::SourceStillWritable);
            }
        }
        let next_state = VersionedState {
            state: operation.state.clone(),
            state_version: operation.state_version,
        }
        .transition(expected_version, next)
        .map_err(|error| {
            if matches!(error, LifecycleError::TerminalState) {
                HandoffError::Terminal
            } else {
                HandoffError::Lifecycle(error)
            }
        })?;

        if next_state.state == HandoffState::Quiescing {
            let source = self
                .bindings
                .get_mut(&operation.source_binding_id)
                .ok_or(HandoffError::BindingNotFound)?;
            source.state = BindingState::Quiescing;
        }
        if next_state.state == HandoffState::Snapshotting {
            let source = self
                .bindings
                .get_mut(&operation.source_binding_id)
                .ok_or(HandoffError::BindingNotFound)?;
            source.state = BindingState::Fenced;
        }
        let stored = self
            .operations
            .get_mut(&operation_id)
            .ok_or(HandoffError::OperationNotFound)?;
        stored.state = next_state.state;
        stored.state_version = next_state.state_version;
        Ok(())
    }

    pub fn acknowledge_loss(
        &mut self,
        operation_id: Uuid,
        report_digest: String,
    ) -> Result<(), HandoffError> {
        let operation = self
            .operations
            .get_mut(&operation_id)
            .ok_or(HandoffError::OperationNotFound)?;
        if operation.state != HandoffState::AwaitingLossAck {
            return Err(HandoffError::InvalidPhase);
        }
        if operation.loss_report.digest() != report_digest {
            return Err(HandoffError::LossDigestMismatch);
        }
        operation.loss_ack_digest = Some(report_digest);
        Ok(())
    }

    pub fn replace_loss_report(
        &mut self,
        operation_id: Uuid,
        loss_report: LossReport,
    ) -> Result<(), HandoffError> {
        let operation = self
            .operations
            .get_mut(&operation_id)
            .ok_or(HandoffError::OperationNotFound)?;
        if operation.state.is_terminal() {
            return Err(HandoffError::Terminal);
        }
        operation.loss_report = loss_report;
        operation.loss_ack_digest = None;
        Ok(())
    }

    pub fn install_target(
        &mut self,
        operation_id: Uuid,
        target: SessionBinding,
    ) -> Result<(), HandoffError> {
        let operation = self
            .operations
            .get(&operation_id)
            .ok_or(HandoffError::OperationNotFound)?
            .clone();
        if operation.state != HandoffState::TargetCreating {
            return Err(HandoffError::InvalidPhase);
        }
        let source = self
            .bindings
            .get(&operation.source_binding_id)
            .ok_or(HandoffError::BindingNotFound)?;
        if source.state != BindingState::Fenced {
            return Err(HandoffError::SourceStillWritable);
        }
        if target.session_id != operation.session_id
            || target.harness_id != operation.target_harness_id
            || target.scope
                != match &operation.scope {
                    HandoffScope::Session => BindingScope::Session,
                    HandoffScope::Task { .. } => BindingScope::Task,
                }
            || target.task_id != operation.task_id
        {
            return Err(HandoffError::BindingMismatch);
        }
        if target.state != BindingState::Inactive {
            return Err(HandoffError::BindingMismatch);
        }
        let target_id = target.binding_id;
        self.register_binding(target)?;
        self.operations
            .get_mut(&operation_id)
            .ok_or(HandoffError::OperationNotFound)?
            .target_binding_id = Some(target_id);
        Ok(())
    }

    pub fn activate_target(&mut self, operation_id: Uuid) -> Result<(), HandoffError> {
        let operation = self
            .operations
            .get(&operation_id)
            .ok_or(HandoffError::OperationNotFound)?
            .clone();
        if operation.state != HandoffState::Activating {
            return Err(HandoffError::InvalidPhase);
        }
        let target_id = operation
            .target_binding_id
            .ok_or(HandoffError::TargetMissing)?;
        let target = self
            .bindings
            .get_mut(&target_id)
            .ok_or(HandoffError::BindingNotFound)?;
        if target.state != BindingState::Inactive {
            return Err(HandoffError::BindingMismatch);
        }
        target.state = BindingState::Active;
        Ok(())
    }

    pub fn binding(&self, binding_id: Uuid) -> Option<&SessionBinding> {
        self.bindings.get(&binding_id)
    }

    pub fn operation(&self, operation_id: Uuid) -> Option<&HandoffOperation> {
        self.operations.get(&operation_id)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use chrono::{TimeZone, Utc};
    use serde::de::DeserializeOwned;
    use serde_json::json;
    use uuid::Uuid;

    use super::super::lifecycle::HandoffState;
    use super::super::types::*;
    use super::*;

    fn session() -> Session {
        let session_id = Uuid::new_v4();
        Session {
            session_id,
            tenant_id: "tenant-1".to_owned(),
            project_id: Some("project-1".to_owned()),
            workspace_id: Some("workspace-1".to_owned()),
            title: Some("Portable session".to_owned()),
            labels: BTreeSet::from(["coding".to_owned()]),
            tags: [("team".to_owned(), "platform".to_owned())]
                .into_iter()
                .collect(),
            state: SessionState::Open,
            state_version: 3,
            parent_session_id: None,
            root_session_id: session_id,
            fork_source_event_id: None,
            active_task_id: None,
            created_at: Utc.timestamp_opt(1_700_000_000, 0).single().unwrap(),
            updated_at: Utc.timestamp_opt(1_700_000_010, 0).single().unwrap(),
            last_active_at: Some(Utc.timestamp_opt(1_700_000_010, 0).single().unwrap()),
            retention_class: Some("standard".to_owned()),
            data_classification: Some("internal".to_owned()),
            extensions: Default::default(),
        }
    }

    fn actor() -> ActorDescriptor {
        ActorDescriptor {
            actor_id: "user-1".to_owned(),
            actor_type: super::super::types::ActorType::Human,
            display_name: Some("Ada".to_owned()),
            authenticated_principal_ref: Some("principal-secret".to_owned()),
            external_subject: Some("external-secret".to_owned()),
            native_aliases: BTreeSet::from(["native-author".to_owned()]),
            ..Default::default()
        }
    }

    fn message(session_id: Uuid) -> Message {
        Message {
            message_id: Uuid::new_v4(),
            session_id,
            task_id: None,
            turn_id: None,
            role: MessageRole::User,
            author: Some(actor()),
            origin: Some("client".to_owned()),
            phase: Some("input".to_owned()),
            parent_message_id: None,
            correlation_id: None,
            status: MessageStatus::Settled,
            content: vec![ContentPart::Text {
                text: "please continue".to_owned(),
                annotations: Vec::new(),
            }],
            visible_to_user: true,
            redaction_state: None,
            created_at: Utc.timestamp_opt(1_700_000_020, 0).single().unwrap(),
            settled_at: Some(Utc.timestamp_opt(1_700_000_021, 0).single().unwrap()),
            native_provenance: Default::default(),
            extensions: Default::default(),
        }
    }

    fn input(session: Session, records: Vec<CanonicalRecord>) -> CapsuleCompileInput {
        CapsuleCompileInput {
            capsule_id: Uuid::new_v4(),
            handoff_id: Uuid::new_v4(),
            session,
            source_binding_id: Some(Uuid::new_v4()),
            target_harness_id: "opencode".to_owned(),
            from_durable_sequence: 1,
            to_durable_sequence: 20,
            branch_id: Some(Uuid::new_v4()),
            head_event_id: None,
            event_ancestor_ids: Vec::new(),
            records,
            workspace_snapshot_digests: vec!["tree-sha".to_owned()],
            artifact_digests: vec!["artifact-sha".to_owned()],
            local_service_bindings: Vec::new(),
            created_at: Utc.timestamp_opt(1_700_000_100, 0).single().unwrap(),
        }
    }

    fn compiler() -> CapsuleCompiler {
        CapsuleCompiler::new("omnisolo", "redaction-v1", 1)
    }

    fn binding(
        session_id: Uuid,
        binding_id: Uuid,
        harness_id: &str,
        state: BindingState,
        access_mode: BindingAccessMode,
    ) -> SessionBinding {
        SessionBinding {
            binding_id,
            session_id,
            task_id: None,
            harness_id: harness_id.to_owned(),
            scope: BindingScope::Session,
            owner_id: session_id,
            workspace_mutation_scope_id: "workspace-scope-1".to_owned(),
            access_mode,
            native_session_id: Some(format!("native-{binding_id}")),
            state,
            generation: 1,
            created_at: Utc.timestamp_opt(1_700_000_000, 0).single().unwrap(),
            last_used_at: None,
            capability_snapshot_id: None,
            adapter_config_digest: None,
            last_imported_native_cursor: None,
            last_exported_durable_sequence: None,
            native_checkpoint_ref: None,
            worker_pool: Some("harness-workers".to_owned()),
            state_locality: Some("remote".to_owned()),
            exact_resume_eligible: false,
            invalidation_reason: None,
            native_record_digest: None,
            extensions: Default::default(),
        }
    }

    #[test]
    fn compiler_allowlists_records_and_strips_authority_and_native_payloads() {
        let session = session();
        let message = message(session.session_id);
        let capsule = compiler()
            .compile(input(
                session,
                vec![
                    CanonicalRecord::Message(message),
                    CanonicalRecord::NativeRecord {
                        kind: "codex.rollout".to_owned(),
                        payload: json!({"native-canary": "must-not-transfer"}),
                    },
                ],
            ))
            .unwrap();

        let encoded = serde_json::to_string(&capsule).unwrap();
        assert!(!encoded.contains("principal-secret"));
        assert!(!encoded.contains("external-secret"));
        assert!(!encoded.contains("native-canary"));
        assert!(capsule.records.iter().any(|record| {
            matches!(
                record,
                PortableRecord::Message(message)
                    if message.actor.as_ref().is_some_and(|actor| {
                        actor.actor_id == "user-1" && !actor.has_authority_context()
                    })
            )
        }));
        assert!(
            capsule
                .loss_report
                .entries
                .iter()
                .any(|entry| entry.source_path == "records[1]")
        );
        capsule.verify_integrity().unwrap();
        let mut corrupt = capsule.clone();
        corrupt.loss_report_digest = "tampered".to_owned();
        assert_eq!(
            corrupt.verify_integrity(),
            Err(CapsuleError::IntegrityMismatch("loss_report".to_owned()))
        );
    }

    #[test]
    fn capsule_integrity_rejects_cross_session_portable_records() {
        let session = session();
        let capsule = compiler()
            .compile(input(
                session.clone(),
                vec![
                    CanonicalRecord::Session(session.clone()),
                    CanonicalRecord::Message(message(session.session_id)),
                ],
            ))
            .unwrap();
        let mut corrupt = capsule.clone();
        let message = corrupt
            .records
            .iter_mut()
            .find_map(|record| match record {
                PortableRecord::Message(message) => Some(message),
                _ => None,
            })
            .expect("compiled capsule contains a message");
        message.session_id = Uuid::new_v4();
        corrupt.record_digest = sha256_json(&corrupt.records);

        assert_eq!(
            corrupt.verify_integrity(),
            Err(CapsuleError::IntegrityMismatch(
                "record_identity".to_owned()
            ))
        );
    }

    #[test]
    fn capsule_integrity_rejects_each_legacy_record_identity_guard() {
        let session = session();
        let session_id = session.session_id;
        let timestamp = session.created_at;
        let source = compiler()
            .compile(input(
                session.clone(),
                vec![
                    CanonicalRecord::Unsupported {
                        kind: "future.unknown".to_owned(),
                        summary: "historical record".to_owned(),
                    },
                    CanonicalRecord::Session(session.clone()),
                ],
            ))
            .unwrap();
        let valid_session = source
            .records
            .into_iter()
            .find_map(|record| match record {
                PortableRecord::Session(value) => Some(value),
                _ => None,
            })
            .expect("compiled session record is present");
        let with_records = |records: Vec<PortableRecord>| {
            let mut capsule = compiler()
                .compile(input(session.clone(), Vec::new()))
                .unwrap();
            capsule.records = records;
            capsule.record_digest = sha256_json(&capsule.records);
            capsule
        };
        let assert_invalid = |record: PortableRecord| {
            assert_eq!(
                with_records(vec![record]).verify_integrity(),
                Err(CapsuleError::IntegrityMismatch(
                    "record_identity".to_owned()
                ))
            );
        };

        let mut bad_session = valid_session;
        bad_session.title = Some("different title".to_owned());
        assert_invalid(PortableRecord::Session(bad_session));

        let task_id = Uuid::from_u128(901);
        let task = PortableTask {
            task_id,
            session_id,
            parent_task_id: None,
            parent_attempt_id: None,
            kind: TaskKind::UserObjective,
            objective: "objective".to_owned(),
            state: TaskState::Queued,
            state_version: 0,
            dependency_task_ids: Vec::new(),
            owner_actor: None,
            input_message_ids: Vec::new(),
            output_message_ids: Vec::new(),
            artifact_ids: Vec::new(),
            terminal_result: None,
            created_at: timestamp,
            updated_at: timestamp,
            started_at: None,
            finished_at: None,
        };
        let mut bad_task = task.clone();
        bad_task.session_id = Uuid::new_v4();
        assert_invalid(PortableRecord::Task(bad_task));

        let turn_id = Uuid::from_u128(902);
        let turn = PortableTurn {
            turn_id,
            task_id,
            session_id,
            sequence: 1,
            state: TurnState::Queued,
            state_version: 0,
            input_message_id: None,
            output_message_ids: Vec::new(),
            stop_reason: None,
            error_code: None,
            created_at: timestamp,
            updated_at: timestamp,
            completed_at: None,
        };
        let mut bad_turn = turn.clone();
        bad_turn.session_id = Uuid::new_v4();
        assert_invalid(PortableRecord::Turn(bad_turn));

        let attempt_id = Uuid::from_u128(903);
        let attempt = PortableAttempt {
            attempt_id,
            session_id,
            task_id,
            turn_id: Some(turn_id),
            parent_attempt_id: None,
            kind: AttemptKind::Interactive,
            state: AttemptState::Pending,
            state_version: 0,
            actor: None,
            harness_id: "omnisolo".to_owned(),
            model_runtime_id: None,
            worker_id: None,
            runtime_config_snapshot_id: None,
            metadata: JsonMap::new(),
            started_at: timestamp,
            finished_at: None,
        };
        let mut bad_attempt = attempt.clone();
        bad_attempt.harness_id.clear();
        assert_invalid(PortableRecord::Attempt(bad_attempt));

        let message = PortableMessage {
            message_id: Uuid::from_u128(904),
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
            content: vec![ContentPart::Text {
                text: "message".to_owned(),
                annotations: Vec::new(),
            }],
            visible_to_user: true,
            redaction_state: None,
            created_at: timestamp,
            settled_at: None,
        };
        let mut bad_message = message.clone();
        bad_message.session_id = Uuid::new_v4();
        assert_invalid(PortableRecord::Message(bad_message));

        let mut bad_turn_relation = turn.clone();
        bad_turn_relation.task_id = Uuid::from_u128(905);
        assert_eq!(
            with_records(vec![
                PortableRecord::Task(task.clone()),
                PortableRecord::Turn(bad_turn_relation),
            ])
            .verify_integrity(),
            Err(CapsuleError::IntegrityMismatch(
                "record_identity".to_owned()
            ))
        );

        let mut bad_task_relation = task.clone();
        bad_task_relation.task_id = Uuid::from_u128(906);
        bad_task_relation.parent_task_id = Some(Uuid::from_u128(907));
        assert_eq!(
            with_records(vec![
                PortableRecord::Task(task.clone()),
                PortableRecord::Task(bad_task_relation),
            ])
            .verify_integrity(),
            Err(CapsuleError::IntegrityMismatch(
                "record_identity".to_owned()
            ))
        );

        let mut bad_parent_attempt = task.clone();
        bad_parent_attempt.task_id = Uuid::from_u128(910);
        bad_parent_attempt.parent_attempt_id = Some(Uuid::from_u128(911));
        assert_eq!(
            with_records(vec![
                PortableRecord::Task(task.clone()),
                PortableRecord::Attempt(attempt.clone()),
                PortableRecord::Task(bad_parent_attempt),
            ])
            .verify_integrity(),
            Err(CapsuleError::IntegrityMismatch(
                "record_identity".to_owned()
            ))
        );

        let mut bad_dependency = task.clone();
        bad_dependency.task_id = Uuid::from_u128(912);
        bad_dependency.dependency_task_ids = vec![Uuid::from_u128(913)];
        assert_eq!(
            with_records(vec![
                PortableRecord::Task(task.clone()),
                PortableRecord::Task(bad_dependency),
            ])
            .verify_integrity(),
            Err(CapsuleError::IntegrityMismatch(
                "record_identity".to_owned()
            ))
        );

        let mut bad_attempt_relation = attempt.clone();
        bad_attempt_relation.task_id = Uuid::from_u128(908);
        assert_eq!(
            with_records(vec![
                PortableRecord::Task(task.clone()),
                PortableRecord::Attempt(bad_attempt_relation),
            ])
            .verify_integrity(),
            Err(CapsuleError::IntegrityMismatch(
                "record_identity".to_owned()
            ))
        );

        let mut bad_message_relation = message;
        bad_message_relation.task_id = Some(Uuid::from_u128(909));
        assert_eq!(
            with_records(vec![
                PortableRecord::Task(task),
                PortableRecord::Message(bad_message_relation),
            ])
            .verify_integrity(),
            Err(CapsuleError::IntegrityMismatch(
                "record_identity".to_owned()
            ))
        );
    }

    #[test]
    fn capsule_manifest_carries_session_lineage_and_policy_metadata() {
        let mut session = session();
        let parent_session_id = Uuid::new_v4();
        let fork_source_event_id = Uuid::new_v4();
        let active_task_id = Uuid::new_v4();
        session.parent_session_id = Some(parent_session_id);
        session.root_session_id = parent_session_id;
        session.fork_source_event_id = Some(fork_source_event_id);
        session.active_task_id = Some(active_task_id);
        session.retention_class = Some("long-term".to_owned());
        session.data_classification = Some("confidential".to_owned());

        let capsule = compiler()
            .compile(input(session.clone(), Vec::new()))
            .unwrap();

        assert_eq!(capsule.manifest.project_id, session.project_id);
        assert_eq!(capsule.manifest.workspace_id, session.workspace_id);
        assert_eq!(capsule.manifest.title, session.title);
        assert_eq!(capsule.manifest.labels, vec!["coding"]);
        assert_eq!(capsule.manifest.tags, session.tags);
        assert_eq!(capsule.manifest.session_state, session.state);
        assert_eq!(
            capsule.manifest.parent_session_id,
            session.parent_session_id
        );
        assert_eq!(capsule.manifest.root_session_id, session.root_session_id);
        assert_eq!(
            capsule.manifest.fork_source_event_id,
            session.fork_source_event_id
        );
        assert_eq!(capsule.manifest.active_task_id, session.active_task_id);
        assert_eq!(capsule.manifest.retention_class, session.retention_class);
        assert_eq!(
            capsule.manifest.data_classification,
            session.data_classification
        );
    }

    #[test]
    fn compiler_redacts_secrets_in_instructions_tools_artifacts_and_extensions() {
        let session = session();
        let artifact = ArtifactRef {
            artifact_id: Uuid::new_v4(),
            kind: ArtifactKind::File,
            name: "output.txt".to_owned(),
            media_type: Some("text/plain".to_owned()),
            byte_length: 20,
            sha256: "b".repeat(64),
            uri: Some("artifact://safe".to_owned()),
            metadata: [("password".to_owned(), "password=secret-canary".to_owned())]
                .into_iter()
                .collect(),
        };
        let capsule = compiler()
            .compile(input(
                session,
                vec![
                    CanonicalRecord::RuntimeConfig(RuntimeConfigInput {
                        snapshot_id: Uuid::new_v4(),
                        system_instructions: vec!["api_key=secret-canary".to_owned()],
                        developer_instructions: vec!["be concise".to_owned()],
                        user_instructions: Vec::new(),
                        requested_model: ModelDescriptor {
                            model_id: "gpt-5".to_owned(),
                            provider: ModelProvider::Managed,
                            ..Default::default()
                        },
                        response_schema: None,
                        tool_definition_digests: Vec::new(),
                        extensions: [("future".to_owned(), json!({"enabled": true}))]
                            .into_iter()
                            .collect(),
                    }),
                    CanonicalRecord::ToolResult {
                        tool_call_id: Uuid::new_v4(),
                        content: vec![ContentPart::Text {
                            text: "password=secret-canary".to_owned(),
                            annotations: Vec::new(),
                        }],
                        metadata: [("token".to_owned(), "token=secret-canary".to_owned())]
                            .into_iter()
                            .collect(),
                    },
                    CanonicalRecord::Artifact(artifact),
                    CanonicalRecord::Extension {
                        path: "extensions.secret".to_owned(),
                        value: json!("Bearer secret-canary"),
                    },
                ],
            ))
            .unwrap();

        let encoded = serde_json::to_string(&capsule).unwrap();
        assert!(!encoded.contains("secret-canary"));
        assert!(encoded.contains("[REDACTED]"));
        assert!(capsule.loss_report.entries.len() >= 4);
    }

    #[test]
    fn portable_maps_and_annotations_drop_opaque_credential_keys() {
        let mut losses = Vec::new();
        let string_map = [
            ("api_key".to_owned(), "opaque-canary".to_owned()),
            ("safe".to_owned(), "value".to_owned()),
        ]
        .into_iter()
        .collect::<BTreeMap<_, _>>();
        let projected_strings = redact_string_map("session.tags", &string_map, &mut losses);
        assert!(!projected_strings.contains_key("api_key"));
        assert_eq!(projected_strings.get("safe"), Some(&"value".to_owned()));

        let json_map = [
            ("accessToken".to_owned(), json!("opaque-canary")),
            ("safe".to_owned(), json!({"nested": true})),
        ]
        .into_iter()
        .collect::<JsonMap>();
        let projected_json = redact_json_map("tool.metadata", &json_map, &mut losses);
        assert!(!projected_json.contains_key("accessToken"));
        assert_eq!(projected_json["safe"]["nested"], true);

        let annotations = project_annotations(
            &[ContentAnnotation {
                annotation_type: "safe".to_owned(),
                start: Some(0),
                end: Some(1),
                data: [
                    ("clientSecret".to_owned(), json!("opaque-canary")),
                    ("label".to_owned(), json!("kept")),
                ]
                .into_iter()
                .collect(),
            }],
            "message.content[0]",
            &mut losses,
        );
        assert!(!annotations[0].data.contains_key("clientSecret"));
        assert_eq!(annotations[0].data["label"], "kept");
        assert!(losses.iter().any(|entry| {
            entry.source_path == "message.content[0].annotations[0].data.clientSecret"
        }));
    }

    #[test]
    fn nested_content_models_and_checkpoints_are_portable_without_secret_leaks() {
        let session = session();
        let nested_artifact = ArtifactRef {
            artifact_id: Uuid::new_v4(),
            kind: ArtifactKind::Image,
            name: "diagram".to_owned(),
            media_type: Some("image/png".to_owned()),
            byte_length: 32,
            sha256: "c".repeat(64),
            uri: Some("https://example.test/download?token=secret-canary".to_owned()),
            metadata: [("note".to_owned(), "token=secret-canary".to_owned())]
                .into_iter()
                .collect(),
        };
        let mut nested_message = message(session.session_id);
        nested_message.content = vec![
            ContentPart::Image {
                artifact: nested_artifact,
            },
            ContentPart::StructuredJson {
                value: json!({"credential": "Bearer secret-canary"}),
                schema_ref: None,
            },
            ContentPart::Native {
                namespace: "native".to_owned(),
                kind: "private".to_owned(),
                payload: json!({"nested-canary": true}),
            },
        ];
        let checkpoint = PortableContextCheckpoint {
            summary: "summary token=secret-canary".to_owned(),
            selected_message_ids: vec![nested_message.message_id],
            source_durable_ranges: vec![(1, 4)],
            retained_ancestor_event_id: Some(Uuid::new_v4()),
            compaction_provenance: Some("compactor-v1".to_owned()),
            usage: [("note".to_owned(), json!("password=secret-canary"))]
                .into_iter()
                .collect(),
            integrity_digest: "source-digest".to_owned(),
        };
        let capsule = compiler()
            .compile(input(
                session,
                vec![
                    CanonicalRecord::Message(nested_message),
                    CanonicalRecord::RuntimeConfig(RuntimeConfigInput {
                        snapshot_id: Uuid::new_v4(),
                        system_instructions: Vec::new(),
                        developer_instructions: Vec::new(),
                        user_instructions: Vec::new(),
                        requested_model: ModelDescriptor {
                            model_id: "local-placeholder".to_owned(),
                            metadata: [("token".to_owned(), json!("token=secret-canary"))]
                                .into_iter()
                                .collect(),
                            ..Default::default()
                        },
                        response_schema: None,
                        tool_definition_digests: Vec::new(),
                        extensions: Default::default(),
                    }),
                    CanonicalRecord::ContextCheckpoint(checkpoint),
                ],
            ))
            .unwrap();
        let encoded = serde_json::to_string(&capsule).unwrap();
        assert!(!encoded.contains("secret-canary"));
        assert!(!encoded.contains("nested-canary"));
        assert!(capsule.records.iter().any(|record| {
            matches!(
                record,
                PortableRecord::ContextCheckpoint(checkpoint)
                    if checkpoint.integrity_digest != "source-digest"
            )
        }));
    }

    #[test]
    fn every_content_variant_is_projected_with_paths_and_effect_references() {
        let mut session = session();
        session.extensions.insert(
            "future_extension".to_owned(),
            json!({"access_token": "secret-canary"}),
        );
        let mut nested_message = message(session.session_id);
        let artifact = ArtifactRef {
            artifact_id: Uuid::new_v4(),
            kind: ArtifactKind::File,
            name: "artifact.txt".to_owned(),
            media_type: Some("text/plain".to_owned()),
            byte_length: 1,
            sha256: "d".repeat(64),
            uri: Some("artifact://safe".to_owned()),
            metadata: Default::default(),
        };
        let tool_call_id = Uuid::new_v4();
        let external_task_id = Uuid::new_v4();
        nested_message.content = vec![
            ContentPart::Text {
                text: "text".to_owned(),
                annotations: Vec::new(),
            },
            ContentPart::ReasoningSummary {
                text: "visible summary".to_owned(),
                annotations: Vec::new(),
            },
            ContentPart::EmbeddedResource {
                media_type: Some("text/plain".to_owned()),
                data: "embedded".to_owned(),
            },
            ContentPart::StructuredJson {
                value: json!({"ok": true}),
                schema_ref: Some("schema://v1".to_owned()),
            },
            ContentPart::File {
                artifact: artifact.clone(),
            },
            ContentPart::Image {
                artifact: artifact.clone(),
            },
            ContentPart::Audio {
                artifact: artifact.clone(),
            },
            ContentPart::Video {
                artifact: artifact.clone(),
            },
            ContentPart::Artifact {
                artifact: artifact.clone(),
            },
            ContentPart::ResourceLink {
                uri: "https://example.test/resource".to_owned(),
                name: Some("resource".to_owned()),
                media_type: Some("text/plain".to_owned()),
            },
            ContentPart::Citation {
                uri: "https://example.test/citation".to_owned(),
                title: Some("citation".to_owned()),
                locator: Some("line 1".to_owned()),
            },
            ContentPart::WorkspacePath {
                snapshot_id: Some(Uuid::new_v4()),
                path: "/host/private.txt".to_owned(),
            },
            ContentPart::WorkspacePath {
                snapshot_id: None,
                path: "relative.txt".to_owned(),
            },
            ContentPart::Symbol {
                path: "src/lib.rs".to_owned(),
                name: "main".to_owned(),
                kind: Some("function".to_owned()),
            },
            ContentPart::Range {
                path: "src/lib.rs".to_owned(),
                start_line: 1,
                start_column: Some(1),
                end_line: 2,
                end_column: Some(3),
            },
            ContentPart::ExternalTaskRef {
                task_id: external_task_id,
            },
            ContentPart::ToolCallRef { tool_call_id },
            ContentPart::ToolResultRef { tool_call_id },
            ContentPart::Native {
                namespace: "vendor".to_owned(),
                kind: "private".to_owned(),
                payload: json!({"secret": true}),
            },
        ];
        let capsule = compiler()
            .compile(input(
                session,
                vec![
                    CanonicalRecord::Unsupported {
                        kind: "future.unknown".to_owned(),
                        summary: "historical record".to_owned(),
                    },
                    CanonicalRecord::Message(nested_message),
                ],
            ))
            .unwrap();
        let projected = capsule
            .records
            .iter()
            .find_map(|record| match record {
                PortableRecord::Message(message) => Some(message),
                _ => None,
            })
            .unwrap();
        assert_eq!(projected.content.len(), 18);
        assert!(matches!(
            projected.content[11],
            ContentPart::WorkspacePath { ref path, .. } if path == "[WORKSPACE_PATH]"
        ));
        assert!(
            !serde_json::to_string(&capsule)
                .unwrap()
                .contains("secret-canary")
        );
        assert!(
            capsule
                .loss_report
                .entries
                .iter()
                .any(|entry| entry.source_path.contains("session.extensions"))
        );
    }

    #[test]
    fn loss_reports_are_sorted_and_acknowledgement_sensitive() {
        let report = LossReport::new(vec![
            LossEntry {
                source_path: "b".to_owned(),
                reason: "warning".to_owned(),
                severity: LossSeverity::Warning,
                target_representation: None,
                acknowledgement_required: false,
                capability: None,
            },
            LossEntry {
                source_path: "a".to_owned(),
                reason: "required".to_owned(),
                severity: LossSeverity::Required,
                target_representation: Some("historical_data".to_owned()),
                acknowledgement_required: true,
                capability: None,
            },
            LossEntry {
                source_path: "c".to_owned(),
                reason: "unsafe without explicit acknowledgement flag".to_owned(),
                severity: LossSeverity::Unsafe,
                target_representation: None,
                acknowledgement_required: false,
                capability: None,
            },
        ]);
        assert_eq!(report.entries[0].source_path, "a");
        assert!(report.requires_ack());
        assert_ne!(report.digest(), LossReport::default().digest());
        assert!(
            LossReport::new(vec![LossEntry {
                source_path: "unsafe".to_owned(),
                reason: "unsafe".to_owned(),
                severity: LossSeverity::Unsafe,
                target_representation: None,
                acknowledgement_required: false,
                capability: None,
            }])
            .requires_ack()
        );
        assert!(
            !LossReport::new(vec![LossEntry {
                source_path: "warning".to_owned(),
                reason: "warning".to_owned(),
                severity: LossSeverity::Warning,
                target_representation: None,
                acknowledgement_required: false,
                capability: None,
            }])
            .requires_ack()
        );
    }

    #[test]
    fn typed_projection_preserves_redaction_and_target_shape() {
        let mut losses = Vec::new();
        let result = project_typed_record::<Value, Value, _>(
            &json!({"value": true}),
            "records[0]",
            &mut losses,
            |_| {
                PortableRecord::HistoricalData(HistoricalData {
                    source_kind: "value".to_owned(),
                    summary: "value".to_owned(),
                })
            },
        );
        assert!(matches!(result, Some(PortableRecord::HistoricalData(_))));
        assert!(losses.is_empty());
    }

    #[test]
    fn typed_projection_records_required_loss_when_redaction_removes_required_field() {
        #[derive(serde::Deserialize)]
        struct RequiredNativeProvenance {
            native_provenance: String,
        }

        fn historical_record(value: RequiredNativeProvenance) -> PortableRecord {
            assert_eq!(value.native_provenance, "native-only");
            PortableRecord::HistoricalData(HistoricalData {
                source_kind: "value".to_owned(),
                summary: "value".to_owned(),
            })
        }

        let mut losses = Vec::new();
        let _ = historical_record(RequiredNativeProvenance {
            native_provenance: "native-only".to_owned(),
        });
        let result = project_typed_record::<Value, RequiredNativeProvenance, _>(
            &json!({"native_provenance": "native-only"}),
            "records[0]",
            &mut losses,
            historical_record,
        );

        assert!(result.is_none());
        assert!(losses.iter().any(|entry| {
            entry.severity == LossSeverity::Required
                && entry.acknowledgement_required
                && entry.capability.as_deref() == Some("typed_record_projection")
        }));
    }

    #[test]
    fn raw_reasoning_is_a_blocking_loss_and_unsupported_records_render_historically() {
        let session = session();
        assert!(matches!(
            compiler()
                .compile(input(
                    session.clone(),
                    vec![CanonicalRecord::RawReasoning {
                        path: "messages[0].reasoning".to_owned(),
                        text: "private chain-of-thought".to_owned(),
                    }],
                ))
                .unwrap_err(),
            CapsuleError::UnsafeLoss(_)
        ));

        let compile_input = input(
            session,
            vec![CanonicalRecord::Unsupported {
                kind: "harness.plan.v9".to_owned(),
                summary: "plan was represented in a native UI".to_owned(),
            }],
        );
        let first = compiler().compile(compile_input.clone()).unwrap();
        let second = compiler().compile(compile_input).unwrap();
        assert_eq!(first.manifest_digest, second.manifest_digest);
        assert_eq!(first.loss_report.digest(), second.loss_report.digest());
        assert!(first.records.iter().any(|record| {
            matches!(record, PortableRecord::HistoricalData(data)
                if data.source_kind == "harness.plan.v9")
        }));
    }

    #[test]
    fn event_head_compilation_carries_ancestor_closure() {
        let session = session();
        let mut store = super::super::events::EventStore::new();
        let root = test_event(session.session_id, Uuid::new_v4(), Vec::new());
        let root_id = root.event_id;
        store.append(root, None, None).unwrap();
        let child = test_event(session.session_id, Uuid::new_v4(), vec![root_id]);
        let child_id = child.event_id;
        store.append(child, None, Some(root_id)).unwrap();

        let capsule = compiler()
            .compile_from_event_head(input(session, Vec::new()), &store, child_id)
            .unwrap();
        assert_eq!(capsule.head_event_id, Some(child_id));
        assert_eq!(capsule.event_ancestor_ids, vec![root_id, child_id]);
    }

    #[test]
    fn handoff_requires_loss_ack_and_never_activates_two_writable_bindings() {
        let session_id = Uuid::new_v4();
        let source_id = Uuid::new_v4();
        let target_id = Uuid::new_v4();
        let mut coordinator = HandoffCoordinator::new();
        coordinator
            .register_binding(binding(
                session_id,
                source_id,
                "omnisolo",
                BindingState::Active,
                BindingAccessMode::ReadWrite,
            ))
            .unwrap();

        let loss_report = LossReport::new(vec![LossEntry {
            source_path: "records[2].native".to_owned(),
            reason: "target lacks native plan support".to_owned(),
            severity: LossSeverity::Required,
            target_representation: Some("historical_data".to_owned()),
            acknowledgement_required: true,
            capability: Some("native_plans".to_owned()),
        }]);
        let operation = HandoffOperation::new(
            Uuid::new_v4(),
            session_id,
            None,
            source_id,
            "opencode".to_owned(),
            HandoffScope::Session,
            loss_report.clone(),
        );
        let operation_id = coordinator.begin(operation).unwrap();
        coordinator
            .advance(operation_id, 0, HandoffState::Fencing)
            .unwrap();
        coordinator
            .advance(operation_id, 1, HandoffState::Quiescing)
            .unwrap();
        coordinator
            .advance(operation_id, 2, HandoffState::Snapshotting)
            .unwrap();
        assert_eq!(
            coordinator.binding(source_id).unwrap().state,
            BindingState::Fenced
        );
        coordinator
            .advance(operation_id, 3, HandoffState::Compiling)
            .unwrap();
        coordinator
            .advance(operation_id, 4, HandoffState::AwaitingLossAck)
            .unwrap();
        assert_eq!(
            coordinator.advance(operation_id, 5, HandoffState::TargetCreating),
            Err(HandoffError::LossAcknowledgementRequired)
        );
        coordinator
            .acknowledge_loss(operation_id, loss_report.digest())
            .unwrap();
        let changed_report = LossReport::new(vec![LossEntry {
            source_path: "records[3]".to_owned(),
            reason: "changed".to_owned(),
            severity: LossSeverity::Required,
            target_representation: None,
            acknowledgement_required: true,
            capability: None,
        }]);
        coordinator
            .replace_loss_report(operation_id, changed_report.clone())
            .unwrap();
        assert_eq!(
            coordinator.advance(operation_id, 5, HandoffState::TargetCreating),
            Err(HandoffError::LossAcknowledgementRequired)
        );
        assert_eq!(
            coordinator.acknowledge_loss(operation_id, loss_report.digest()),
            Err(HandoffError::LossDigestMismatch)
        );
        coordinator
            .acknowledge_loss(operation_id, changed_report.digest())
            .unwrap();
        coordinator
            .advance(operation_id, 5, HandoffState::TargetCreating)
            .unwrap();

        let target = binding(
            session_id,
            target_id,
            "opencode",
            BindingState::Inactive,
            BindingAccessMode::ReadWrite,
        );
        coordinator.install_target(operation_id, target).unwrap();
        coordinator
            .advance(operation_id, 6, HandoffState::Activating)
            .unwrap();
        coordinator.activate_target(operation_id).unwrap();
        assert_eq!(
            coordinator.binding(target_id).unwrap().state,
            BindingState::Active
        );
        coordinator
            .advance(operation_id, 7, HandoffState::Completed)
            .unwrap();
        assert_eq!(
            coordinator.replace_loss_report(operation_id, changed_report),
            Err(HandoffError::Terminal)
        );
    }

    #[test]
    fn handoff_rejects_invalid_scope_phase_and_writer_combinations() {
        let session_id = Uuid::new_v4();
        let source_id = Uuid::new_v4();
        let mut coordinator = HandoffCoordinator::new();
        let mut invalid = binding(
            session_id,
            Uuid::new_v4(),
            "omnisolo",
            BindingState::Active,
            BindingAccessMode::ReadWrite,
        );
        invalid.binding_id = Uuid::nil();
        assert_eq!(
            coordinator.register_binding(invalid),
            Err(HandoffError::InvalidBinding)
        );
        let source = binding(
            session_id,
            source_id,
            "omnisolo",
            BindingState::Active,
            BindingAccessMode::ReadWrite,
        );
        coordinator.register_binding(source.clone()).unwrap();
        assert_eq!(
            coordinator.register_binding(source.clone()),
            Err(HandoffError::BindingAlreadyExists)
        );
        let duplicate_writable = binding(
            session_id,
            Uuid::new_v4(),
            "opencode",
            BindingState::Active,
            BindingAccessMode::ReadWrite,
        );
        assert_eq!(
            coordinator.register_binding(duplicate_writable),
            Err(HandoffError::DuplicateWritableBinding)
        );

        let invalid_operation = HandoffOperation::new(
            Uuid::nil(),
            session_id,
            None,
            source_id,
            String::new(),
            HandoffScope::Session,
            LossReport::default(),
        );
        assert_eq!(
            coordinator.begin(invalid_operation),
            Err(HandoffError::InvalidScope)
        );
        let unknown_source = HandoffOperation::new(
            Uuid::new_v4(),
            session_id,
            None,
            Uuid::new_v4(),
            "opencode".to_owned(),
            HandoffScope::Session,
            LossReport::default(),
        );
        assert_eq!(
            coordinator.begin(unknown_source),
            Err(HandoffError::BindingNotFound)
        );
        let mismatch = HandoffOperation::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            None,
            source_id,
            "opencode".to_owned(),
            HandoffScope::Session,
            LossReport::default(),
        );
        assert_eq!(
            coordinator.begin(mismatch),
            Err(HandoffError::BindingMismatch)
        );

        let task_id = Uuid::from_u128(920);
        let task_source_id = Uuid::from_u128(921);
        let mut task_source = binding(
            session_id,
            task_source_id,
            "omnisolo",
            BindingState::Active,
            BindingAccessMode::ReadOnly,
        );
        task_source.task_id = Some(task_id);
        task_source.scope = BindingScope::Task;
        task_source.owner_id = task_id;
        coordinator.register_binding(task_source).unwrap();
        let task_operation = HandoffOperation::new(
            Uuid::from_u128(922),
            session_id,
            Some(task_id),
            task_source_id,
            "opencode".to_owned(),
            HandoffScope::Task { task_id },
            LossReport::default(),
        );
        coordinator.begin(task_operation).unwrap();
        let wrong_task_source = HandoffOperation::new(
            Uuid::from_u128(923),
            session_id,
            Some(task_id),
            source_id,
            "opencode".to_owned(),
            HandoffScope::Task { task_id },
            LossReport::default(),
        );
        assert_eq!(
            coordinator.begin(wrong_task_source),
            Err(HandoffError::InvalidScope)
        );
        coordinator
            .operations
            .get_mut(&Uuid::from_u128(922))
            .unwrap()
            .state = HandoffState::TargetCreating;
        coordinator.bindings.get_mut(&task_source_id).unwrap().state = BindingState::Fenced;
        let mut task_target = binding(
            session_id,
            Uuid::from_u128(929),
            "opencode",
            BindingState::Inactive,
            BindingAccessMode::ReadOnly,
        );
        task_target.task_id = Some(task_id);
        task_target.scope = BindingScope::Task;
        task_target.owner_id = task_id;
        coordinator
            .install_target(Uuid::from_u128(922), task_target)
            .unwrap();

        let stale_operation_id = Uuid::from_u128(924);
        coordinator
            .begin(HandoffOperation::new(
                stale_operation_id,
                session_id,
                None,
                source_id,
                "opencode".to_owned(),
                HandoffScope::Session,
                LossReport::default(),
            ))
            .unwrap();
        assert!(matches!(
            coordinator.advance(stale_operation_id, 99, HandoffState::Fencing),
            Err(HandoffError::Lifecycle(LifecycleError::StaleVersion { .. }))
        ));

        let mut inactive = binding(
            session_id,
            Uuid::new_v4(),
            "omnisolo",
            BindingState::Inactive,
            BindingAccessMode::ReadOnly,
        );
        inactive.access_mode = BindingAccessMode::ReadWrite;
        coordinator.register_binding(inactive.clone()).unwrap();
        let inactive_operation = HandoffOperation::new(
            Uuid::new_v4(),
            session_id,
            None,
            inactive.binding_id,
            "opencode".to_owned(),
            HandoffScope::Session,
            LossReport::default(),
        );
        assert_eq!(
            coordinator.begin(inactive_operation),
            Err(HandoffError::SourceNotActive)
        );

        let mut wrong_scope = HandoffOperation::new(
            Uuid::new_v4(),
            session_id,
            Some(Uuid::new_v4()),
            source_id,
            "opencode".to_owned(),
            HandoffScope::Session,
            LossReport::default(),
        );
        assert_eq!(
            coordinator.begin(wrong_scope.clone()),
            Err(HandoffError::InvalidScope)
        );
        wrong_scope.task_id = None;
        wrong_scope.state = HandoffState::Fencing;
        assert_eq!(
            coordinator.begin(wrong_scope),
            Err(HandoffError::InvalidPhase)
        );

        let operation_id = Uuid::new_v4();
        let operation = HandoffOperation::new(
            operation_id,
            session_id,
            None,
            source_id,
            "opencode".to_owned(),
            HandoffScope::Session,
            LossReport::default(),
        );
        coordinator.begin(operation.clone()).unwrap();
        assert_eq!(
            coordinator.begin(operation),
            Err(HandoffError::OperationAlreadyExists)
        );
        assert_eq!(
            coordinator.advance(Uuid::new_v4(), 0, HandoffState::Fencing),
            Err(HandoffError::OperationNotFound)
        );
        assert_eq!(
            coordinator.acknowledge_loss(operation_id, "digest".to_owned()),
            Err(HandoffError::InvalidPhase)
        );
        assert_eq!(
            coordinator.replace_loss_report(Uuid::new_v4(), LossReport::default()),
            Err(HandoffError::OperationNotFound)
        );
        assert_eq!(
            coordinator.advance(operation_id, 0, HandoffState::Snapshotting),
            Err(HandoffError::SourceStillWritable)
        );
        coordinator
            .advance(operation_id, 0, HandoffState::Fencing)
            .unwrap();
        coordinator
            .advance(operation_id, 1, HandoffState::Quiescing)
            .unwrap();
        coordinator
            .advance(operation_id, 2, HandoffState::Snapshotting)
            .unwrap();
        coordinator
            .advance(operation_id, 3, HandoffState::Compiling)
            .unwrap();
        assert_eq!(
            coordinator.install_target(
                operation_id,
                binding(
                    session_id,
                    Uuid::from_u128(925),
                    "opencode",
                    BindingState::Inactive,
                    BindingAccessMode::ReadOnly,
                ),
            ),
            Err(HandoffError::InvalidPhase)
        );
        coordinator
            .advance(operation_id, 4, HandoffState::TargetCreating)
            .unwrap();
        assert_eq!(
            coordinator.advance(operation_id, 5, HandoffState::Activating),
            Err(HandoffError::TargetMissing)
        );
        coordinator.bindings.get_mut(&source_id).unwrap().state = BindingState::Active;
        assert_eq!(
            coordinator.install_target(
                operation_id,
                binding(
                    session_id,
                    Uuid::new_v4(),
                    "opencode",
                    BindingState::Inactive,
                    BindingAccessMode::ReadOnly,
                ),
            ),
            Err(HandoffError::SourceStillWritable)
        );
        coordinator.bindings.get_mut(&source_id).unwrap().state = BindingState::Fenced;
        let mut wrong_scope_target = binding(
            session_id,
            Uuid::from_u128(926),
            "opencode",
            BindingState::Inactive,
            BindingAccessMode::ReadOnly,
        );
        wrong_scope_target.scope = BindingScope::Task;
        wrong_scope_target.task_id = Some(Uuid::from_u128(927));
        assert_eq!(
            coordinator.install_target(operation_id, wrong_scope_target),
            Err(HandoffError::BindingMismatch)
        );
        assert_eq!(
            coordinator.install_target(
                operation_id,
                binding(
                    session_id,
                    Uuid::new_v4(),
                    "wrong-harness",
                    BindingState::Inactive,
                    BindingAccessMode::ReadOnly,
                ),
            ),
            Err(HandoffError::BindingMismatch)
        );

        let target_id = Uuid::new_v4();
        let target = binding(
            session_id,
            target_id,
            "opencode",
            BindingState::Inactive,
            BindingAccessMode::ReadOnly,
        );
        assert_eq!(
            coordinator.install_target(
                operation_id,
                binding(
                    session_id,
                    Uuid::from_u128(928),
                    "opencode",
                    BindingState::Active,
                    BindingAccessMode::ReadOnly,
                ),
            ),
            Err(HandoffError::BindingMismatch)
        );
        coordinator.install_target(operation_id, target).unwrap();
        assert_eq!(
            coordinator.activate_target(operation_id),
            Err(HandoffError::InvalidPhase)
        );
        assert_eq!(
            coordinator.advance(operation_id, 5, HandoffState::Activating),
            Ok(())
        );
        assert_eq!(
            coordinator.advance(operation_id, 6, HandoffState::Completed),
            Err(HandoffError::TargetMissing)
        );
        coordinator.bindings.get_mut(&target_id).unwrap().state = BindingState::Active;
        assert_eq!(
            coordinator.activate_target(operation_id),
            Err(HandoffError::BindingMismatch)
        );
        coordinator.bindings.get_mut(&target_id).unwrap().state = BindingState::Inactive;
        coordinator.activate_target(operation_id).unwrap();
        coordinator
            .advance(operation_id, 6, HandoffState::Completed)
            .unwrap();
        assert_eq!(
            coordinator.advance(operation_id, 7, HandoffState::Failed),
            Err(HandoffError::Terminal)
        );
        assert!(coordinator.operation(operation_id).is_some());
        assert!(coordinator.binding(Uuid::new_v4()).is_none());
    }

    fn decode<T: DeserializeOwned>(value: serde_json::Value) -> T {
        serde_json::from_value(value).unwrap()
    }

    fn typed_canonical_records(
        session_id: Uuid,
        task_id: Uuid,
        turn_id: Uuid,
        attempt_id: Uuid,
    ) -> Vec<CanonicalRecord> {
        let timestamp = "2023-11-14T22:13:20Z";
        let definition_id = Uuid::from_u128(1001);
        let tool_call_id = Uuid::from_u128(1002);
        let process_id = Uuid::from_u128(1003);
        let plan_id = Uuid::from_u128(1004);
        let snapshot_id = Uuid::from_u128(1005);
        let records = vec![
            CanonicalRecord::ToolDefinition(decode(json!({
                "snapshot_id": definition_id,
                "qualified_name": "shell.run",
                "description": "run a command",
                "source": "built_in",
                "input_schema": {},
                "output_schema": null,
                "annotations": {},
                "version": "1",
                "digest": "tool-digest",
                "captured_at": timestamp,
            }))),
            CanonicalRecord::ToolCall(decode(json!({
                "tool_call_id": tool_call_id,
                "session_id": session_id,
                "task_id": task_id,
                "turn_id": turn_id,
                "attempt_id": attempt_id,
                "definition_snapshot_id": definition_id,
                "raw_input": {"command": "printf ok"},
                "parsed_input": {"command": "printf ok"},
                "state": "completed",
                "started_at": timestamp,
                "finished_at": timestamp,
                "retry_of": null,
                "native_provenance": {},
                "observed_effect_ids": [],
                "metadata": {},
            }))),
            CanonicalRecord::ToolProgress(decode(json!({
                "progress_id": Uuid::from_u128(1006),
                "tool_call_id": tool_call_id,
                "sequence": 1,
                "chunk_type": "stdout",
                "content": [],
                "percent": 50,
                "artifact_ids": [],
                "locations": ["stdout"],
                "native_display_data": {"line": "ok"},
                "occurred_at": timestamp,
            }))),
            CanonicalRecord::ToolResultRecord(decode(json!({
                "tool_call_id": tool_call_id,
                "session_id": session_id,
                "task_id": task_id,
                "turn_id": turn_id,
                "attempt_id": attempt_id,
                "state": "completed",
                "content": [],
                "structured_output": {"ok": true},
                "error_code": null,
                "error_message": null,
                "started_at": timestamp,
                "finished_at": timestamp,
                "artifact_ids": [],
                "observed_effect_ids": [],
                "provider_metadata": {},
            }))),
            CanonicalRecord::Interaction(decode(json!({
                "interaction_id": Uuid::from_u128(1007),
                "request_id": "approval-1",
                "session_id": session_id,
                "task_id": task_id,
                "turn_id": turn_id,
                "attempt_id": attempt_id,
                "kind": "approval",
                "subject": "write file",
                "action": "write",
                "description": "write a file",
                "risk": "medium",
                "input_schema": null,
                "options": [],
                "requested_scope": "workspace",
                "expires_at": null,
                "state": "pending",
                "response": null,
                "policy_evaluation": null,
                "created_at": timestamp,
                "updated_at": timestamp,
                "native_provenance": {},
            }))),
            CanonicalRecord::Plan(decode(json!({
                "plan_id": plan_id,
                "session_id": session_id,
                "task_id": task_id,
                "turn_id": turn_id,
                "version": 1,
                "objective": "finish",
                "state": "draft",
                "steps": [],
                "created_at": timestamp,
                "updated_at": timestamp,
                "source": "client",
            }))),
            CanonicalRecord::Todo(decode(json!({
                "todo_id": Uuid::from_u128(1008),
                "session_id": session_id,
                "task_id": task_id,
                "plan_id": plan_id,
                "title": "finish",
                "details": null,
                "state": "pending",
                "ordinal": 1,
                "owner_actor": null,
                "created_at": timestamp,
                "updated_at": timestamp,
                "completed_at": null,
            }))),
            CanonicalRecord::Goal(decode(json!({
                "goal_id": Uuid::from_u128(1009),
                "session_id": session_id,
                "task_id": task_id,
                "objective": "finish",
                "completion_criteria": "done",
                "state": "active",
                "budget": {
                    "max_input_tokens": null,
                    "max_output_tokens": null,
                    "max_turns": null,
                    "max_cost_micros": null,
                    "max_wall_time_ms": null
                },
                "usage": {
                    "input_tokens": 0,
                    "output_tokens": 0,
                    "turns": 0,
                    "cost_micros": 0,
                    "wall_time_ms": 0
                },
                "created_at": timestamp,
                "updated_at": timestamp,
                "completed_at": null,
            }))),
            CanonicalRecord::Process(decode(json!({
                "process_id": process_id,
                "session_id": session_id,
                "task_id": task_id,
                "turn_id": turn_id,
                "attempt_id": attempt_id,
                "tool_call_id": tool_call_id,
                "command": "printf ok",
                "argv": ["printf", "ok"],
                "logical_cwd": "/workspace",
                "environment_keys": ["PATH"],
                "state": "exited",
                "started_at": timestamp,
                "finished_at": timestamp,
                "exit_code": 0,
                "signal": null,
                "timed_out": false,
                "out_of_memory": false,
                "artifact_ids": [],
                "metadata": {},
            }))),
            CanonicalRecord::ProcessChunk(decode(json!({
                "chunk_id": Uuid::from_u128(1010),
                "process_id": process_id,
                "stream": "stdout",
                "sequence": 1,
                "content": "ok",
                "occurred_at": timestamp,
                "artifact_id": null,
            }))),
            CanonicalRecord::WorkspaceDescriptor(decode(json!({
                "descriptor_id": Uuid::from_u128(1011),
                "session_id": session_id,
                "logical_roots": ["/workspace"],
                "mount_policy": "isolated",
                "operating_system": "linux",
                "architecture": "amd64",
                "shell": "bash",
                "toolchain_hints": ["rust"],
                "environment_allowlist": ["PATH"],
                "repository_identity": {},
                "data_locality": "cluster",
                "metadata": {},
            }))),
            CanonicalRecord::WorkspaceSnapshot(decode(json!({
                "snapshot_id": Uuid::from_u128(1012),
                "session_id": session_id,
                "parent_snapshot_id": null,
                "durable_sequence": 5,
                "tree_digest": "tree",
                "archive_artifact_id": null,
                "git_repository_identity": {},
                "base_commit": "abc",
                "branch": "main",
                "remote": null,
                "working_tree_patch_digest": null,
                "index_patch_digest": null,
                "untracked_files": [],
                "submodule_state": {},
                "file_metadata": {},
                "completeness": "complete",
                "creator_attempt_id": attempt_id,
                "reason": "handoff",
                "created_at": timestamp,
                "integrity_digest": "snapshot-digest",
            }))),
            CanonicalRecord::RuntimeConfigSnapshot(decode(json!({
                "snapshot_id": snapshot_id,
                "tenant_id": "tenant-1",
                "snapshot_digest": "config-digest",
                "instruction_layers": [],
                "agent_profile": "default",
                "collaboration_settings": {},
                "requested_model": null,
                "response_schema": null,
                "resource_limits": {},
                "tool_definition_snapshot_ids": [definition_id],
                "mcp_descriptors": [],
                "skill_descriptors": [],
                "plugin_descriptors": [],
                "hook_descriptors": [],
                "sandbox_policy": {},
                "compaction_settings": {},
                "retry_settings": {},
                "budget_settings": {},
                "telemetry_settings": {},
                "environment_allowlist": ["PATH"],
                "workspace_snapshot_id": Uuid::from_u128(1012),
                "created_at": timestamp,
            }))),
            CanonicalRecord::Usage(decode(json!({
                "usage_id": Uuid::from_u128(1013),
                "tenant_id": "tenant-1",
                "session_id": session_id,
                "task_id": task_id,
                "turn_id": turn_id,
                "attempt_id": attempt_id,
                "model_binding_id": null,
                "provider": "managed",
                "model_id": "model-1",
                "input_tokens": 2,
                "output_tokens": 3,
                "cached_tokens": 0,
                "reasoning_tokens": 0,
                "cost_micros": 4,
                "latency_ms": 5,
                "finish_reason": "stop",
                "recorded_at": timestamp,
                "metadata": {},
            }))),
            CanonicalRecord::Error(decode(json!({
                "error_id": Uuid::from_u128(1014),
                "tenant_id": "tenant-1",
                "session_id": session_id,
                "task_id": task_id,
                "turn_id": turn_id,
                "attempt_id": attempt_id,
                "source": "harness",
                "code": "E_TEST",
                "message": "test",
                "retriable": true,
                "uncertain": false,
                "failure_class": "transport",
                "occurred_at": timestamp,
                "metadata": {},
            }))),
            CanonicalRecord::Compaction(decode(json!({
                "compaction_id": Uuid::from_u128(1015),
                "session_id": session_id,
                "task_id": task_id,
                "turn_id": turn_id,
                "source_durable_ranges": [[1, 4]],
                "retained_ancestor_event_id": null,
                "summary": "summary",
                "reason": "context limit",
                "created_by_attempt_id": attempt_id,
                "input_tokens": 4,
                "output_tokens": 2,
                "created_at": timestamp,
                "integrity_digest": "compaction-digest",
            }))),
            CanonicalRecord::CapabilitySnapshot(decode(json!({
                "snapshot_id": Uuid::from_u128(1016),
                "subject_kind": "harness",
                "subject_id": "codex",
                "capability_version": 1,
                "capabilities": ["prompt"],
                "source_revision": "revision",
                "captured_at": timestamp,
                "expires_at": null,
                "digest": "capability-digest",
                "metadata": {},
            }))),
            CanonicalRecord::ContextCheckpoint(decode(json!({
                "summary": "visible summary",
                "selected_message_ids": [],
                "source_durable_ranges": [[1, 2]],
                "retained_ancestor_event_id": null,
                "compaction_provenance": "compaction-1",
                "usage": {},
                "integrity_digest": "checkpoint-digest",
            }))),
            CanonicalRecord::NativeRecordSet(decode(json!({
                "record_set_id": Uuid::from_u128(1017),
                "session_id": session_id,
                "attempt_id": attempt_id,
                "harness_id": "codex",
                "adapter_version": "1",
                "native_schema": "rollout",
                "first_cursor": "1",
                "last_cursor": "2",
                "record_count": 2,
                "payload_digest": "native-set",
                "object_storage_ref": "objects/native-set",
                "captured_at": timestamp,
            }))),
            CanonicalRecord::NativeRecordTyped(decode(json!({
                "native_record_id": Uuid::from_u128(1018),
                "record_set_id": Uuid::from_u128(1017),
                "harness_id": "codex",
                "adapter_version": "1",
                "native_schema": "rollout",
                "record_kind": "line",
                "native_identity": "line-1",
                "native_cursor": "1",
                "ordinal": 1,
                "payload_digest": "native-record",
                "object_storage_ref": "objects/native-record",
                "captured_at": timestamp,
                "data_classification": "internal",
                "encrypted": true,
            }))),
            CanonicalRecord::Unsupported {
                kind: "future.plan".to_owned(),
                summary: "future data".to_owned(),
            },
            CanonicalRecord::Extension {
                path: "extensions.future".to_owned(),
                value: json!({"secret":"must redact"}),
            },
            CanonicalRecord::RawReasoning {
                path: "records[raw_reasoning]".to_owned(),
                text: "private reasoning".to_owned(),
            },
        ];
        records
    }

    #[test]
    fn typed_capsule_projection_and_integrity_guards_cover_all_transfer_families() {
        let session_id = session().session_id;
        let task_id = Uuid::from_u128(1101);
        let turn_id = Uuid::from_u128(1102);
        let attempt_id = Uuid::from_u128(1103);
        let canonical = typed_canonical_records(session_id, task_id, turn_id, attempt_id);
        let mut required_interaction = canonical
            .iter()
            .find_map(|record| match record {
                CanonicalRecord::Interaction(value) => Some(value.clone()),
                _ => None,
            })
            .unwrap();
        required_interaction
            .native_provenance
            .insert("provider_secret".to_owned(), json!("must not transfer"));
        let mut required_session = session();
        required_session.session_id = session_id;
        required_session.root_session_id = session_id;
        assert!(matches!(
            compiler().compile(input(
                required_session,
                vec![CanonicalRecord::Interaction(required_interaction)],
            )),
            Err(CapsuleError::RequiredLoss(_))
        ));
        let mut losses = Vec::new();
        let mut portable = Vec::new();
        for (index, record) in canonical.iter().enumerate() {
            let result = project_record(record, &format!("records[{index}]"), &mut losses);
            match result {
                Ok(Some(record)) => portable.push(record),
                Ok(None) => {}
                Err(RecordProjectionError::Unsafe(loss)) => {
                    assert!(matches!(loss.severity, LossSeverity::Unsafe));
                    losses.push(loss);
                }
            }
        }
        assert!(
            portable
                .iter()
                .any(|record| matches!(record, PortableRecord::HistoricalData(_)))
        );
        assert!(
            portable
                .iter()
                .any(|record| matches!(record, PortableRecord::ContextCheckpoint(_)))
        );
        assert!(
            losses
                .iter()
                .any(|loss| loss.source_path.contains("extensions.future"))
        );

        for record in canonical {
            match record {
                CanonicalRecord::ToolCall(value) => {
                    assert!(typed_record_identity_is_valid(
                        &PortableRecord::ToolCall(value),
                        session_id
                    ));
                }
                CanonicalRecord::Interaction(value) => {
                    assert!(typed_record_identity_is_valid(
                        &PortableRecord::TypedInteraction(value),
                        session_id
                    ));
                }
                _ => {}
            }
        }
        for record in &portable {
            assert!(typed_record_identity_is_valid(record, session_id));
        }

        let capsule = compiler().compile(input(session(), Vec::new())).unwrap();
        let mut manifest_corrupt = capsule.clone();
        manifest_corrupt.manifest_digest = "bad".to_owned();
        assert_eq!(
            manifest_corrupt.verify_integrity(),
            Err(CapsuleError::IntegrityMismatch("manifest".to_owned()))
        );
        let mut records_corrupt = capsule.clone();
        records_corrupt.record_digest = "bad".to_owned();
        assert_eq!(
            records_corrupt.verify_integrity(),
            Err(CapsuleError::IntegrityMismatch("records".to_owned()))
        );
        let mut pointers_corrupt = capsule.clone();
        pointers_corrupt.head_event_id = Some(Uuid::from_u128(1201));
        assert_eq!(
            pointers_corrupt.verify_integrity(),
            Err(CapsuleError::IntegrityMismatch("event_pointers".to_owned()))
        );

        let mut duplicate_ancestors = capsule.clone();
        let ancestor = Uuid::from_u128(1202);
        duplicate_ancestors.head_event_id = Some(ancestor);
        duplicate_ancestors.event_ancestor_ids = vec![ancestor, ancestor];
        duplicate_ancestors.manifest.head_event_id = Some(ancestor);
        duplicate_ancestors.manifest.event_ancestor_ids =
            duplicate_ancestors.event_ancestor_ids.clone();
        duplicate_ancestors.manifest_digest = sha256_json(&duplicate_ancestors.manifest);
        assert_eq!(
            duplicate_ancestors.verify_integrity(),
            Err(CapsuleError::IntegrityMismatch("event_pointers".to_owned()))
        );

        let mut identity_corrupt = capsule.clone();
        identity_corrupt.manifest.tenant_id.clear();
        identity_corrupt.manifest_digest = sha256_json(&identity_corrupt.manifest);
        assert_eq!(
            identity_corrupt.verify_integrity(),
            Err(CapsuleError::IntegrityMismatch(
                "manifest_identity".to_owned()
            ))
        );

        let wrong_goal: Goal = decode(json!({
            "goal_id": Uuid::from_u128(1203),
            "session_id": Uuid::from_u128(1204),
            "task_id": null,
            "objective": "wrong session",
            "completion_criteria": null,
            "state": "active",
            "budget": {
                "max_input_tokens": null,
                "max_output_tokens": null,
                "max_turns": null,
                "max_cost_micros": null,
                "max_wall_time_ms": null
            },
            "usage": {
                "input_tokens": 0,
                "output_tokens": 0,
                "turns": 0,
                "cost_micros": 0,
                "wall_time_ms": 0
            },
            "created_at": "2023-11-14T22:13:20Z",
            "updated_at": "2023-11-14T22:13:20Z",
            "completed_at": null,
        }));
        let mut typed_identity_corrupt = capsule.clone();
        typed_identity_corrupt.records = vec![PortableRecord::TypedGoal(wrong_goal)];
        typed_identity_corrupt.record_digest = sha256_json(&typed_identity_corrupt.records);
        assert_eq!(
            typed_identity_corrupt.verify_integrity(),
            Err(CapsuleError::IntegrityMismatch(
                "record_identity".to_owned()
            ))
        );
    }

    fn test_event(
        session_id: Uuid,
        event_id: Uuid,
        parents: Vec<Uuid>,
    ) -> super::super::types::EventEnvelope {
        super::super::types::EventEnvelope {
            event_id,
            tenant_id: "tenant-1".to_owned(),
            session_id,
            task_id: None,
            turn_id: None,
            source_attempt_id: None,
            ingest_attempt_id: None,
            actor_id: None,
            worker_id: None,
            harness_id: Some("omnisolo".to_owned()),
            binding_id: None,
            binding_generation: None,
            lease_id: None,
            lease_generation: None,
            fencing_token: None,
            durable_sequence: None,
            delivery_stream_id: None,
            delivery_sequence: None,
            aggregate_id: None,
            aggregate_sequence: None,
            branch_id: None,
            parent_event_ids: parents,
            event_type: "session.snapshot".to_owned(),
            payload_schema: "omnisolo.session.v1".to_owned(),
            payload_version: 1,
            occurred_at: Utc.timestamp_opt(1_700_000_000, 0).single().unwrap(),
            ingested_at: Utc.timestamp_opt(1_700_000_001, 0).single().unwrap(),
            correlation_id: None,
            causation_id: None,
            idempotency_key: None,
            durability: super::super::types::EventDurability::Durable,
            replay_requirement: super::super::types::ReplayRequirement::Required,
            visibility: None,
            data_classification: None,
            native_provenance: Default::default(),
            payload: json!({}),
            extensions: Default::default(),
        }
    }
}
