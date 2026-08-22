use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use super::events::EventStore;
use super::lifecycle::{HandoffState, LifecycleError, LifecycleState, VersionedState};
use super::types::{
    ArtifactKind, ArtifactRef, BindingAccessMode, BindingState, ContentPart, JsonMap, Message,
    MessageRole, MessageStatus, ModelDescriptor, Session, SessionBinding, SessionState, Task,
    TaskKind, TaskState, Turn,
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
        self.entries.iter().any(|entry| {
            entry.acknowledgement_required
                || matches!(
                    entry.severity,
                    LossSeverity::Required | LossSeverity::Unsafe
                )
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PortableTask {
    pub task_id: Uuid,
    pub session_id: Uuid,
    pub parent_task_id: Option<Uuid>,
    pub kind: TaskKind,
    pub objective: String,
    pub state: TaskState,
    pub dependency_task_ids: Vec<Uuid>,
    pub input_message_ids: Vec<Uuid>,
    pub output_message_ids: Vec<Uuid>,
    pub artifact_ids: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PortableTurn {
    pub turn_id: Uuid,
    pub task_id: Uuid,
    pub session_id: Uuid,
    pub sequence: i64,
    pub state: super::types::TurnState,
    pub input_message_id: Option<Uuid>,
    pub output_message_ids: Vec<Uuid>,
    pub stop_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
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
    Message(PortableMessage),
    Artifact(PortableArtifact),
    RuntimeConfig(PortableRuntimeConfig),
    ToolResult(PortableToolResult),
    Interaction(PortableStructuredRecord),
    Plan(PortableStructuredRecord),
    Todo(PortableStructuredRecord),
    Goal(PortableStructuredRecord),
    Process(PortableStructuredRecord),
    WorkspaceSnapshot(PortableStructuredRecord),
    ContextCheckpoint(PortableContextCheckpoint),
    HistoricalData(HistoricalData),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum CanonicalRecord {
    Session(Session),
    Task(Task),
    Turn(Turn),
    Message(Message),
    Artifact(ArtifactRef),
    RuntimeConfig(RuntimeConfigInput),
    ToolResult {
        tool_call_id: Uuid,
        content: Vec<ContentPart>,
        metadata: BTreeMap<String, String>,
    },
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
        Ok(())
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
            schema_version: 1,
            minimum_reader_version: 1,
            producer: self.producer.clone(),
            compiler_version: 1,
            created_at: input.created_at,
            tenant_id: input.session.tenant_id.clone(),
            session_id: input.session.session_id,
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
            redaction_policy_id: self.redaction_policy_id.clone(),
            redaction_policy_version: self.redaction_policy_version,
        };
        Ok(SessionCapsule {
            manifest_digest: sha256_json(&manifest),
            record_digest: sha256_json(&records),
            loss_report_digest: sha256_json(&report),
            head_event_id: input.head_event_id,
            event_ancestor_ids: input.event_ancestor_ids,
            manifest,
            records,
            loss_report: report,
        })
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

fn project_record(
    record: &CanonicalRecord,
    path: &str,
    losses: &mut Vec<LossEntry>,
) -> Result<Option<PortableRecord>, RecordProjectionError> {
    match record {
        CanonicalRecord::Session(session) => {
            let title = session
                .title
                .as_ref()
                .map(|title| redact_text(&format!("{path}.title"), title, losses));
            let tags = redact_string_map(&format!("{path}.tags"), &session.tags, losses);
            Ok(Some(PortableRecord::Session(PortableSession {
                session_id: session.session_id,
                tenant_id: session.tenant_id.clone(),
                project_id: session.project_id.clone(),
                workspace_id: session.workspace_id.clone(),
                title,
                labels: session.labels.iter().cloned().collect(),
                tags,
                state: session.state.clone(),
                created_at: session.created_at,
                updated_at: session.updated_at,
            })))
        }
        CanonicalRecord::Task(task) => Ok(Some(PortableRecord::Task(PortableTask {
            task_id: task.task_id,
            session_id: task.session_id,
            parent_task_id: task.parent_task_id,
            kind: task.kind.clone(),
            objective: redact_text(&format!("{path}.objective"), &task.objective, losses),
            state: task.state.clone(),
            dependency_task_ids: task.dependency_task_ids.clone(),
            input_message_ids: task.input_message_ids.clone(),
            output_message_ids: task.output_message_ids.clone(),
            artifact_ids: task.artifact_ids.clone(),
            created_at: task.created_at,
            updated_at: task.updated_at,
            finished_at: task.finished_at,
        }))),
        CanonicalRecord::Turn(turn) => Ok(Some(PortableRecord::Turn(project_turn(turn)))),
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

fn project_turn(turn: &Turn) -> PortableTurn {
    PortableTurn {
        turn_id: turn.turn_id,
        task_id: turn.task_id,
        session_id: turn.session_id,
        sequence: turn.sequence,
        state: turn.state.clone(),
        input_message_id: turn.input_message_id,
        output_message_ids: turn.output_message_ids.clone(),
        stop_reason: turn.stop_reason.clone(),
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
                    annotations: annotations.clone(),
                }),
                ContentPart::ReasoningSummary { text, annotations } => {
                    Some(ContentPart::ReasoningSummary {
                        text: redact_text(&part_path, text, losses),
                        annotations: annotations.clone(),
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
        .map(|(key, value)| {
            (
                key.clone(),
                redact_text(&format!("{path}.{key}"), value, losses),
            )
        })
        .collect()
}

fn redact_json_map(path: &str, values: &JsonMap, losses: &mut Vec<LossEntry>) -> JsonMap {
    match redact_json(
        path,
        &Value::Object(values.clone().into_iter().collect()),
        losses,
    ) {
        Value::Object(values) => values.into_iter().collect(),
        _ => JsonMap::new(),
    }
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
    let lower = text.to_ascii_lowercase();
    let secret = ["api_key=", "password=", "token=", "bearer ", "-----begin"]
        .iter()
        .any(|marker| lower.contains(marker));
    if !secret {
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
                .map(|(key, value)| {
                    (
                        key.clone(),
                        redact_json(&format!("{path}.{key}"), value, losses),
                    )
                })
                .collect::<Map<String, Value>>(),
        ),
        other => other.clone(),
    }
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
    BindingNotFound,
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
    use serde_json::json;
    use uuid::Uuid;

    use super::super::lifecycle::HandoffState;
    use super::super::types::{
        ActorDescriptor, ArtifactKind, ArtifactRef, BindingAccessMode, BindingScope, BindingState,
        ContentPart, Message, MessageRole, MessageStatus, ModelDescriptor, ModelProvider, Session,
        SessionBinding, SessionState,
    };
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
                        extensions: Default::default(),
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
        ]);
        assert_eq!(report.entries[0].source_path, "a");
        assert!(report.requires_ack());
        assert_ne!(report.digest(), LossReport::default().digest());
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
