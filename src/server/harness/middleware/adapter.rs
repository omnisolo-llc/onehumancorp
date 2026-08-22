use std::collections::{BTreeMap, BTreeSet};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use super::capsule::{
    CanonicalRecord, CapsuleCompileInput, CapsuleCompiler, CapsuleError, PortableRecord,
    SessionCapsule,
};
use super::events::{AppendResult, EventStore, EventStoreError};
use super::lease::{FenceToken, Lease, LeaseError};
use super::lifecycle::{LifecycleError, VersionedState};
use super::types::{
    ActorDescriptor, Attempt, AttemptKind, AttemptState, ContentPart, EventDurability,
    EventEnvelope, JsonMap, Message, MessageRole, MessageStatus, ReplayRequirement, Session,
    SessionState, Task, TaskKind, TaskState, Turn, TurnState,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OmniSoloRunConfig {
    pub tenant_id: String,
    pub objective: String,
    pub session_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub project_id: Option<String>,
    pub workspace_id: Option<String>,
    pub title: Option<String>,
    pub labels: BTreeSet<String>,
    pub tags: BTreeMap<String, String>,
    pub actor: Option<ActorDescriptor>,
    pub harness_id: String,
    pub worker_id: Option<String>,
    pub model_runtime_id: Option<String>,
    pub idempotency_key: Option<String>,
    pub create_turn: bool,
}

impl OmniSoloRunConfig {
    pub fn new(tenant_id: impl Into<String>, objective: impl Into<String>) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            objective: objective.into(),
            session_id: None,
            task_id: None,
            project_id: None,
            workspace_id: None,
            title: None,
            labels: BTreeSet::new(),
            tags: BTreeMap::new(),
            actor: None,
            harness_id: "omnisolo".to_owned(),
            worker_id: None,
            model_runtime_id: None,
            idempotency_key: None,
            create_turn: false,
        }
    }

    pub fn with_turn(mut self) -> Self {
        self.create_turn = true;
        self
    }

    pub fn without_turn(mut self) -> Self {
        self.create_turn = false;
        self
    }

    pub fn with_worker(mut self, worker_id: impl Into<String>) -> Self {
        self.worker_id = Some(worker_id.into());
        self
    }

    pub fn with_harness(mut self, harness_id: impl Into<String>) -> Self {
        self.harness_id = harness_id.into();
        self
    }

    pub fn with_session_id(mut self, session_id: Uuid) -> Self {
        self.session_id = Some(session_id);
        self
    }

    pub fn with_task_id(mut self, task_id: Uuid) -> Self {
        self.task_id = Some(task_id);
        self
    }

    pub fn with_workspace(
        mut self,
        project_id: impl Into<String>,
        workspace_id: impl Into<String>,
    ) -> Self {
        self.project_id = Some(project_id.into());
        self.workspace_id = Some(workspace_id.into());
        self
    }

    pub fn with_session_metadata(
        mut self,
        project_id: Option<String>,
        workspace_id: Option<String>,
        title: Option<String>,
        labels: BTreeSet<String>,
        tags: BTreeMap<String, String>,
    ) -> Self {
        self.project_id = project_id;
        self.workspace_id = workspace_id;
        self.title = title;
        self.labels = labels;
        self.tags = tags;
        self
    }

    pub fn with_idempotency_key(mut self, idempotency_key: impl Into<String>) -> Self {
        self.idempotency_key = Some(idempotency_key.into());
        self
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum OmniSoloEvent {
    RunStarted {
        iteration: i32,
    },
    TextChunk {
        content: String,
    },
    ToolCall {
        name: String,
        args_json: String,
        result: String,
        iteration: i32,
    },
    TaskComplete {
        content: String,
    },
    TaskError {
        error: String,
    },
    UserInterventionRequired {
        error: String,
    },
    IterationStarted {
        iteration: i32,
        message_count: usize,
    },
    CheckpointSaved {
        iteration: i32,
        path: String,
    },
    Handoff {
        target_agent: String,
    },
    RewindOccurred {
        iteration: i32,
        checkpoint_id: String,
        reason: String,
    },
    GuardrailTripped {
        reason: String,
    },
    CostUpdate {
        total_cost_usd: f64,
    },
}

impl OmniSoloEvent {
    fn wire(
        &self,
    ) -> (
        &'static str,
        serde_json::Value,
        EventDurability,
        ReplayRequirement,
    ) {
        match self {
            Self::RunStarted { iteration } => (
                "run.started",
                json!({ "iteration": iteration }),
                EventDurability::Durable,
                ReplayRequirement::Required,
            ),
            Self::TextChunk { content } => (
                "assistant.text_chunk",
                json!({ "content": content }),
                EventDurability::Transient,
                ReplayRequirement::Ignorable,
            ),
            Self::ToolCall {
                name,
                args_json,
                result,
                iteration,
            } => (
                "tool.call_settled",
                json!({
                    "name": name,
                    "args_json": args_json,
                    "result": result,
                    "iteration": iteration,
                }),
                EventDurability::Durable,
                ReplayRequirement::Required,
            ),
            Self::TaskComplete { content } => (
                "task.completed",
                json!({ "content": content }),
                EventDurability::Durable,
                ReplayRequirement::Required,
            ),
            Self::TaskError { error } => (
                "task.failed",
                json!({ "error": error }),
                EventDurability::Durable,
                ReplayRequirement::Required,
            ),
            Self::UserInterventionRequired { error } => (
                "interaction.required",
                json!({ "error": error }),
                EventDurability::Durable,
                ReplayRequirement::Required,
            ),
            Self::IterationStarted {
                iteration,
                message_count,
            } => (
                "turn.iteration_started",
                json!({ "iteration": iteration, "message_count": message_count }),
                EventDurability::Durable,
                ReplayRequirement::Required,
            ),
            Self::CheckpointSaved { iteration, path } => (
                "context.checkpoint_saved",
                json!({ "iteration": iteration, "path": path }),
                EventDurability::Durable,
                ReplayRequirement::Required,
            ),
            Self::Handoff { target_agent } => (
                "task.handoff_requested",
                json!({ "target_agent": target_agent }),
                EventDurability::Durable,
                ReplayRequirement::Required,
            ),
            Self::RewindOccurred {
                iteration,
                checkpoint_id,
                reason,
            } => (
                "context.rewound",
                json!({
                    "iteration": iteration,
                    "checkpoint_id": checkpoint_id,
                    "reason": reason,
                }),
                EventDurability::Durable,
                ReplayRequirement::Required,
            ),
            Self::GuardrailTripped { reason } => (
                "guardrail.tripped",
                json!({ "reason": reason }),
                EventDurability::Durable,
                ReplayRequirement::Required,
            ),
            Self::CostUpdate { total_cost_usd } => (
                "usage.cost_updated",
                json!({ "total_cost_usd": total_cost_usd }),
                EventDurability::Durable,
                ReplayRequirement::Required,
            ),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ImportedCapsule {
    pub session_id: Uuid,
    pub tenant_id: String,
    pub target_harness_id: String,
    pub records: Vec<PortableRecord>,
    pub loss_report: super::capsule::LossReport,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum AdapterError {
    EmptyTenant,
    EmptyObjective,
    EmptyHarnessId,
    EventStore(EventStoreError),
    Lifecycle(LifecycleError),
    Lease(LeaseError),
    Capsule(CapsuleError),
    TenantMismatch,
    TargetHarnessMismatch,
    NoDurableHead,
    TerminalAttempt,
    RecoveryNotAllowed,
}

impl From<EventStoreError> for AdapterError {
    fn from(error: EventStoreError) -> Self {
        Self::EventStore(error)
    }
}

impl From<LifecycleError> for AdapterError {
    fn from(error: LifecycleError) -> Self {
        Self::Lifecycle(error)
    }
}

impl From<LeaseError> for AdapterError {
    fn from(error: LeaseError) -> Self {
        Self::Lease(error)
    }
}

impl From<CapsuleError> for AdapterError {
    fn from(error: CapsuleError) -> Self {
        Self::Capsule(error)
    }
}

pub struct OmniSoloHarnessAdapter {
    session: Session,
    task: Task,
    turn: Option<Turn>,
    attempt: Attempt,
    lease: Lease,
    branch_id: Uuid,
    event_store: EventStore,
    recorded_events: Vec<EventEnvelope>,
    canonical_records: Vec<CanonicalRecord>,
    attempt_history: Vec<Attempt>,
    turn_history: Vec<Turn>,
    last_event_id: Option<Uuid>,
    last_durable_event_id: Option<Uuid>,
    last_durable_sequence: Option<i64>,
    idempotency_key: Option<String>,
}

impl std::fmt::Debug for OmniSoloHarnessAdapter {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OmniSoloHarnessAdapter")
            .field("session_id", &self.session.session_id)
            .field("task_id", &self.task.task_id)
            .field("turn_id", &self.turn.as_ref().map(|turn| turn.turn_id))
            .field("attempt_id", &self.attempt.attempt_id)
            .field("attempt_history_count", &self.attempt_history.len())
            .field("recorded_event_count", &self.recorded_events.len())
            .finish()
    }
}

impl OmniSoloHarnessAdapter {
    pub fn start(config: OmniSoloRunConfig) -> Result<Self, AdapterError> {
        if config.tenant_id.trim().is_empty() {
            return Err(AdapterError::EmptyTenant);
        }
        if config.objective.trim().is_empty() {
            return Err(AdapterError::EmptyObjective);
        }
        if config.harness_id.trim().is_empty() {
            return Err(AdapterError::EmptyHarnessId);
        }

        let now = Utc::now();
        let session_id = config.session_id.unwrap_or_else(Uuid::new_v4);
        let task_id = config.task_id.unwrap_or_else(Uuid::new_v4);
        let turn_id = config.create_turn.then(Uuid::new_v4);
        let attempt_id = Uuid::new_v4();
        let branch_id = Uuid::new_v4();

        let session = Session {
            session_id,
            tenant_id: config.tenant_id,
            project_id: config.project_id,
            workspace_id: config.workspace_id,
            title: config.title,
            labels: config.labels,
            tags: config.tags,
            state: SessionState::Open,
            state_version: 0,
            parent_session_id: None,
            root_session_id: session_id,
            fork_source_event_id: None,
            active_task_id: Some(task_id),
            created_at: now,
            updated_at: now,
            last_active_at: Some(now),
            retention_class: None,
            data_classification: None,
            extensions: JsonMap::new(),
        };
        let task = Task {
            task_id,
            session_id,
            parent_task_id: None,
            parent_attempt_id: None,
            kind: TaskKind::UserObjective,
            objective: config.objective.clone(),
            state: TaskState::Running,
            state_version: 1,
            dependency_task_ids: Vec::new(),
            owner_actor: config.actor.clone(),
            input_message_ids: Vec::new(),
            output_message_ids: Vec::new(),
            artifact_ids: Vec::new(),
            terminal_result: None,
            created_at: now,
            updated_at: now,
            started_at: Some(now),
            finished_at: None,
            extensions: JsonMap::new(),
        };
        let turn = turn_id.map(|turn_id| Turn {
            turn_id,
            task_id,
            session_id,
            sequence: 1,
            state: TurnState::Running,
            state_version: 1,
            input_message_id: None,
            output_message_ids: Vec::new(),
            stop_reason: None,
            error_code: None,
            created_at: now,
            updated_at: now,
            completed_at: None,
            extensions: JsonMap::new(),
        });
        let attempt = Attempt {
            attempt_id,
            session_id,
            task_id,
            turn_id,
            parent_attempt_id: None,
            kind: AttemptKind::Interactive,
            state: AttemptState::Running,
            state_version: 3,
            actor: config.actor,
            harness_id: config.harness_id,
            model_runtime_id: config.model_runtime_id,
            started_at: now,
            finished_at: None,
            runtime_config_snapshot_id: None,
            model_binding_id: None,
            worker_id: config.worker_id,
            extensions: JsonMap::new(),
            metadata: JsonMap::new(),
        };
        let lease = Lease::active(attempt_id.to_string(), 1);

        let mut adapter = Self {
            session,
            task,
            turn,
            attempt,
            lease,
            branch_id,
            event_store: EventStore::new(),
            recorded_events: Vec::new(),
            canonical_records: Vec::new(),
            attempt_history: Vec::new(),
            turn_history: Vec::new(),
            last_event_id: None,
            last_durable_event_id: None,
            last_durable_sequence: None,
            idempotency_key: config.idempotency_key,
        };
        adapter.event_store.register_lease(adapter.lease.clone());
        adapter.record(OmniSoloEvent::RunStarted { iteration: 0 })?;
        Ok(adapter)
    }

    pub fn session(&self) -> &Session {
        &self.session
    }

    pub fn task(&self) -> &Task {
        &self.task
    }

    pub fn turn(&self) -> Option<&Turn> {
        self.turn.as_ref()
    }

    pub fn attempt(&self) -> &Attempt {
        &self.attempt
    }

    pub fn lease(&self) -> &Lease {
        &self.lease
    }

    pub fn fence_token(&self) -> FenceToken {
        self.lease.token()
    }

    pub fn events(&self) -> &[EventEnvelope] {
        &self.recorded_events
    }

    pub fn replay(&self, from_sequence: i64, to_sequence: i64) -> Vec<EventEnvelope> {
        self.event_store
            .replay(self.session.session_id, from_sequence, to_sequence)
    }

    pub fn last_durable_sequence(&self) -> Option<i64> {
        self.last_durable_sequence
    }

    pub fn record(&mut self, event: OmniSoloEvent) -> Result<AppendResult, AdapterError> {
        self.record_with_fence(event, self.fence_token())
    }

    pub fn record_with_fence(
        &mut self,
        event: OmniSoloEvent,
        fence: FenceToken,
    ) -> Result<AppendResult, AdapterError> {
        self.record_from_source_with_fence(event, self.attempt.attempt_id, fence)
    }

    pub fn record_from_source_with_fence(
        &mut self,
        event: OmniSoloEvent,
        source_attempt_id: Uuid,
        fence: FenceToken,
    ) -> Result<AppendResult, AdapterError> {
        if self.attempt.state.is_terminal() {
            return Err(AdapterError::TerminalAttempt);
        }

        self.validate_lifecycle(&event)?;

        let (event_type, payload, durability, replay_requirement) = event.wire();
        let event_id = Uuid::new_v4();
        let mut envelope = EventEnvelope {
            event_id,
            tenant_id: self.session.tenant_id.clone(),
            session_id: self.session.session_id,
            task_id: Some(self.task.task_id),
            turn_id: self.turn.as_ref().map(|turn| turn.turn_id),
            source_attempt_id: Some(source_attempt_id),
            ingest_attempt_id: Some(self.attempt.attempt_id),
            actor_id: self
                .attempt
                .actor
                .as_ref()
                .map(|actor| actor.actor_id.clone()),
            worker_id: self.attempt.worker_id.clone(),
            harness_id: Some(self.attempt.harness_id.clone()),
            binding_id: self.attempt.model_binding_id,
            binding_generation: None,
            lease_id: Some(self.lease.lease_id),
            lease_generation: Some(fence.generation),
            fencing_token: Some(fence.value()),
            durable_sequence: None,
            delivery_stream_id: Some(format!("session:{}", self.session.session_id)),
            delivery_sequence: None,
            aggregate_id: Some(self.task.task_id),
            aggregate_sequence: None,
            branch_id: Some(self.branch_id),
            parent_event_ids: self.last_durable_event_id.into_iter().collect(),
            event_type: event_type.to_owned(),
            payload_schema: "omnisolo.agent_event.v1".to_owned(),
            payload_version: 1,
            occurred_at: Utc::now(),
            ingested_at: Utc::now(),
            correlation_id: Some(self.task.task_id),
            causation_id: self.last_event_id,
            idempotency_key: if matches!(event, OmniSoloEvent::RunStarted { .. }) {
                self.idempotency_key.clone()
            } else {
                None
            },
            durability,
            replay_requirement,
            visibility: Some("user".to_owned()),
            data_classification: Some("internal".to_owned()),
            native_provenance: JsonMap::new(),
            payload,
            extensions: JsonMap::new(),
        };

        let result = self
            .event_store
            .append(envelope.clone(), Some(fence), self.last_event_id)?;
        envelope = result.event.clone();
        if !result.duplicate {
            self.last_event_id = Some(envelope.event_id);
            if envelope.durability == EventDurability::Durable {
                self.last_durable_event_id = Some(envelope.event_id);
                self.last_durable_sequence = envelope.durable_sequence;
            }
            self.capture_canonical_record(&event, &envelope);
            let event_id = envelope.event_id;
            self.recorded_events.push(envelope);
            self.apply_lifecycle(&event, event_id)?;
        }
        Ok(result)
    }

    pub fn reassign_worker(&mut self, worker_id: impl Into<String>) -> Result<(), AdapterError> {
        self.lease.reassign()?;
        self.attempt.worker_id = Some(worker_id.into());
        self.event_store.register_lease(self.lease.clone());
        Ok(())
    }

    /// Starts an explicit recovery attempt after a terminal attempt. The old
    /// attempt remains historical and is linked through `parent_attempt_id`.
    pub fn begin_recovery_attempt(
        &mut self,
        worker_id: impl Into<String>,
    ) -> Result<(), AdapterError> {
        if !matches!(
            self.attempt.state,
            AttemptState::Failed
                | AttemptState::Cancelled
                | AttemptState::Lost
                | AttemptState::Fenced
        ) {
            return Err(AdapterError::RecoveryNotAllowed);
        }

        let now = Utc::now();
        let previous_attempt = self.attempt.clone();
        let parent_attempt_id = self.attempt.attempt_id;
        let attempt_id = Uuid::new_v4();
        self.task.state = TaskState::Running;
        self.task.state_version += 1;
        self.task.updated_at = now;
        self.task.finished_at = None;
        self.task.terminal_result = None;

        if let Some(previous_turn) = self.turn.take() {
            self.turn_history.push(previous_turn.clone());
            self.turn = Some(Turn {
                turn_id: Uuid::new_v4(),
                task_id: self.task.task_id,
                session_id: self.session.session_id,
                sequence: previous_turn.sequence + 1,
                state: TurnState::Running,
                state_version: 1,
                input_message_id: previous_turn.input_message_id,
                output_message_ids: Vec::new(),
                stop_reason: None,
                error_code: None,
                created_at: now,
                updated_at: now,
                completed_at: None,
                extensions: JsonMap::new(),
            });
        }

        self.attempt = Attempt {
            attempt_id,
            session_id: self.session.session_id,
            task_id: self.task.task_id,
            turn_id: self.turn.as_ref().map(|turn| turn.turn_id),
            parent_attempt_id: Some(parent_attempt_id),
            kind: AttemptKind::Recovery,
            state: AttemptState::Running,
            state_version: 3,
            actor: self.attempt.actor.clone(),
            harness_id: self.attempt.harness_id.clone(),
            model_runtime_id: self.attempt.model_runtime_id.clone(),
            started_at: now,
            finished_at: None,
            runtime_config_snapshot_id: self.attempt.runtime_config_snapshot_id,
            model_binding_id: self.attempt.model_binding_id,
            worker_id: Some(worker_id.into()),
            extensions: JsonMap::new(),
            metadata: [(
                "recovery_parent_attempt_id".to_owned(),
                json!(parent_attempt_id),
            )]
            .into_iter()
            .collect(),
        };
        self.attempt_history.push(previous_attempt);
        self.lease = Lease::active(attempt_id.to_string(), 1);
        self.event_store.register_lease(self.lease.clone());
        Ok(())
    }

    pub fn export_capsule(
        &self,
        target_harness_id: impl Into<String>,
        handoff_id: Uuid,
    ) -> Result<SessionCapsule, AdapterError> {
        let head_event_id = self
            .last_durable_event_id
            .ok_or(AdapterError::NoDurableHead)?;
        let to_durable_sequence = self.last_durable_sequence.unwrap_or(0);
        let mut records = vec![
            CanonicalRecord::Session(self.session.clone()),
            CanonicalRecord::Task(self.task.clone()),
        ];
        records.extend(
            self.attempt_history
                .iter()
                .cloned()
                .map(CanonicalRecord::Attempt),
        );
        records.push(CanonicalRecord::Attempt(self.attempt.clone()));
        records.extend(self.canonical_records.clone());
        records.extend(self.turn_history.iter().cloned().map(CanonicalRecord::Turn));
        if let Some(turn) = &self.turn {
            records.push(CanonicalRecord::Turn(turn.clone()));
        }
        let input = CapsuleCompileInput {
            capsule_id: Uuid::new_v4(),
            handoff_id,
            session: self.session.clone(),
            source_binding_id: None,
            target_harness_id: target_harness_id.into(),
            from_durable_sequence: 1,
            to_durable_sequence,
            branch_id: Some(self.branch_id),
            head_event_id: Some(head_event_id),
            event_ancestor_ids: Vec::new(),
            records,
            workspace_snapshot_digests: Vec::new(),
            artifact_digests: Vec::new(),
            created_at: Utc::now(),
        };
        CapsuleCompiler::new("omnisolo", "omnisolo.portable.v2", 1)
            .compile_from_event_head(input, &self.event_store, head_event_id)
            .map_err(AdapterError::Capsule)
    }

    pub fn import_capsule(
        capsule: &SessionCapsule,
        expected_tenant_id: &str,
        target_harness_id: &str,
    ) -> Result<ImportedCapsule, AdapterError> {
        capsule.verify_integrity().map_err(AdapterError::Capsule)?;
        if capsule.manifest.tenant_id != expected_tenant_id {
            return Err(AdapterError::TenantMismatch);
        }
        if capsule.manifest.target_harness_id != target_harness_id {
            return Err(AdapterError::TargetHarnessMismatch);
        }
        Ok(ImportedCapsule {
            session_id: capsule.manifest.session_id,
            tenant_id: capsule.manifest.tenant_id.clone(),
            target_harness_id: capsule.manifest.target_harness_id.clone(),
            records: capsule.records.clone(),
            loss_report: capsule.loss_report.clone(),
        })
    }

    fn capture_canonical_record(&mut self, event: &OmniSoloEvent, envelope: &EventEnvelope) {
        let text_message = |text: &str, status: MessageStatus| {
            CanonicalRecord::Message(Message {
                message_id: envelope.event_id,
                session_id: envelope.session_id,
                task_id: envelope.task_id,
                turn_id: envelope.turn_id,
                role: MessageRole::Assistant,
                author: self.attempt.actor.clone(),
                origin: Some("omnisolo".to_owned()),
                phase: Some(envelope.event_type.clone()),
                parent_message_id: None,
                correlation_id: envelope.correlation_id,
                status,
                content: vec![ContentPart::Text {
                    text: text.to_owned(),
                    annotations: Vec::new(),
                }],
                visible_to_user: true,
                redaction_state: None,
                created_at: envelope.occurred_at,
                settled_at: Some(envelope.ingested_at),
                native_provenance: JsonMap::new(),
                extensions: JsonMap::new(),
            })
        };

        let record = match event {
            OmniSoloEvent::ToolCall {
                name,
                args_json,
                result,
                iteration,
            } => Some(CanonicalRecord::ToolResult {
                tool_call_id: envelope.event_id,
                content: vec![ContentPart::Text {
                    text: result.clone(),
                    annotations: Vec::new(),
                }],
                metadata: [
                    ("name".to_owned(), name.clone()),
                    ("args_json".to_owned(), args_json.clone()),
                    ("iteration".to_owned(), iteration.to_string()),
                ]
                .into_iter()
                .collect(),
            }),
            OmniSoloEvent::TaskComplete { content } => {
                Some(text_message(content, MessageStatus::Settled))
            }
            OmniSoloEvent::TaskError { error } => Some(text_message(
                &format!("Task failed: {error}"),
                MessageStatus::Settled,
            )),
            OmniSoloEvent::UserInterventionRequired { error } => {
                Some(CanonicalRecord::Unsupported {
                    kind: "interaction.required".to_owned(),
                    summary: error.clone(),
                })
            }
            OmniSoloEvent::CheckpointSaved { path, iteration } => {
                Some(CanonicalRecord::Unsupported {
                    kind: "context.checkpoint_saved".to_owned(),
                    summary: format!("iteration {iteration}, checkpoint {path}"),
                })
            }
            OmniSoloEvent::Handoff { target_agent } => Some(CanonicalRecord::Unsupported {
                kind: "task.handoff_requested".to_owned(),
                summary: target_agent.clone(),
            }),
            OmniSoloEvent::RewindOccurred {
                iteration,
                checkpoint_id,
                reason,
            } => Some(CanonicalRecord::Unsupported {
                kind: "context.rewound".to_owned(),
                summary: format!("iteration {iteration}, checkpoint {checkpoint_id}: {reason}"),
            }),
            OmniSoloEvent::GuardrailTripped { reason } => Some(CanonicalRecord::Unsupported {
                kind: "guardrail.tripped".to_owned(),
                summary: reason.clone(),
            }),
            OmniSoloEvent::CostUpdate { total_cost_usd } => Some(CanonicalRecord::Unsupported {
                kind: "usage.cost_updated".to_owned(),
                summary: format!("total_cost_usd={total_cost_usd}"),
            }),
            OmniSoloEvent::RunStarted { .. }
            | OmniSoloEvent::TextChunk { .. }
            | OmniSoloEvent::IterationStarted { .. } => None,
        };
        if let Some(record) = record {
            self.canonical_records.push(record);
        }
    }

    fn validate_lifecycle(&self, event: &OmniSoloEvent) -> Result<(), AdapterError> {
        match event {
            OmniSoloEvent::TaskComplete { .. } => {
                transition_task(&self.task, TaskState::Completed)?;
                transition_attempt(&self.attempt, AttemptState::Succeeded)?;
                if let Some(turn) = &self.turn {
                    transition_turn(turn, TurnState::Completed)?;
                }
            }
            OmniSoloEvent::TaskError { .. } => {
                transition_task(&self.task, TaskState::Failed)?;
                transition_attempt(&self.attempt, AttemptState::Failed)?;
                if let Some(turn) = &self.turn {
                    transition_turn(turn, TurnState::Failed)?;
                }
            }
            OmniSoloEvent::UserInterventionRequired { .. } => {
                transition_task(&self.task, TaskState::WaitingInput)?;
                transition_attempt(&self.attempt, AttemptState::Quiescing)?;
                if let Some(turn) = &self.turn {
                    transition_turn(turn, TurnState::WaitingInput)?;
                }
            }
            OmniSoloEvent::Handoff { .. } => {
                transition_session(&self.session, SessionState::HandingOff)?;
                transition_task(&self.task, TaskState::HandingOff)?;
                transition_attempt(&self.attempt, AttemptState::Quiescing)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn apply_lifecycle(
        &mut self,
        event: &OmniSoloEvent,
        event_id: Uuid,
    ) -> Result<(), AdapterError> {
        match event {
            OmniSoloEvent::TaskComplete { content } => {
                self.task = transition_task(&self.task, TaskState::Completed)?;
                self.task.output_message_ids.push(event_id);
                self.task.terminal_result = Some(super::types::TaskResult {
                    status: "completed".to_owned(),
                    summary: Some(content.clone()),
                    output_message_ids: vec![event_id],
                    artifact_ids: Vec::new(),
                    error_code: None,
                    completed_at: Utc::now(),
                });
                self.task.finished_at = self
                    .task
                    .terminal_result
                    .as_ref()
                    .map(|result| result.completed_at);
                self.attempt = transition_attempt(&self.attempt, AttemptState::Succeeded)?;
                if let Some(turn) = self.turn.take() {
                    self.turn = Some(transition_turn(&turn, TurnState::Completed)?);
                }
                self.lease.release();
                self.event_store.register_lease(self.lease.clone());
            }
            OmniSoloEvent::TaskError { error } => {
                self.task = transition_task(&self.task, TaskState::Failed)?;
                self.task.terminal_result = Some(super::types::TaskResult {
                    status: "failed".to_owned(),
                    summary: None,
                    output_message_ids: Vec::new(),
                    artifact_ids: Vec::new(),
                    error_code: Some(error.clone()),
                    completed_at: Utc::now(),
                });
                self.task.finished_at = self
                    .task
                    .terminal_result
                    .as_ref()
                    .map(|result| result.completed_at);
                self.attempt = transition_attempt(&self.attempt, AttemptState::Failed)?;
                if let Some(turn) = self.turn.take() {
                    self.turn = Some(transition_turn(&turn, TurnState::Failed)?);
                }
                self.lease.release();
                self.event_store.register_lease(self.lease.clone());
            }
            OmniSoloEvent::UserInterventionRequired { .. } => {
                self.task = transition_task(&self.task, TaskState::WaitingInput)?;
                self.attempt = transition_attempt(&self.attempt, AttemptState::Quiescing)?;
                if let Some(turn) = self.turn.take() {
                    self.turn = Some(transition_turn(&turn, TurnState::WaitingInput)?);
                }
            }
            OmniSoloEvent::Handoff { .. } => {
                self.session = transition_session(&self.session, SessionState::HandingOff)?;
                self.task = transition_task(&self.task, TaskState::HandingOff)?;
                self.attempt = transition_attempt(&self.attempt, AttemptState::Quiescing)?;
            }
            _ => {}
        }
        Ok(())
    }
}

fn transition_session(session: &Session, next: SessionState) -> Result<Session, AdapterError> {
    let state = VersionedState {
        state: session.state.clone(),
        state_version: session.state_version,
    }
    .transition(session.state_version, next)?;
    let mut session = session.clone();
    session.state = state.state;
    session.state_version = state.state_version;
    session.updated_at = Utc::now();
    Ok(session)
}

fn transition_task(task: &Task, next: TaskState) -> Result<Task, AdapterError> {
    let state = VersionedState {
        state: task.state.clone(),
        state_version: task.state_version,
    }
    .transition(task.state_version, next)?;
    let mut task = task.clone();
    task.state = state.state;
    task.state_version = state.state_version;
    task.updated_at = Utc::now();
    Ok(task)
}

fn transition_turn(turn: &Turn, next: TurnState) -> Result<Turn, AdapterError> {
    let state = VersionedState {
        state: turn.state.clone(),
        state_version: turn.state_version,
    }
    .transition(turn.state_version, next)?;
    let mut turn = turn.clone();
    turn.state = state.state;
    turn.state_version = state.state_version;
    turn.updated_at = Utc::now();
    turn.completed_at = turn.state.is_terminal().then(Utc::now);
    Ok(turn)
}

fn transition_attempt(attempt: &Attempt, next: AttemptState) -> Result<Attempt, AdapterError> {
    let state = VersionedState {
        state: attempt.state.clone(),
        state_version: attempt.state_version,
    }
    .transition(attempt.state_version, next)?;
    let mut attempt = attempt.clone();
    attempt.state = state.state;
    attempt.state_version = state.state_version;
    attempt.finished_at = attempt.state.is_terminal().then(Utc::now);
    Ok(attempt)
}

#[cfg(test)]
mod tests {
    use super::super::lease::FenceError;
    use super::*;
    use uuid::Uuid;

    #[test]
    fn omni_solo_run_creates_session_task_and_task_level_attempt() {
        let run = OmniSoloHarnessAdapter::start(
            OmniSoloRunConfig::new("tenant-1", "finish the task")
                .with_worker("worker-1")
                .with_idempotency_key("request-1")
                .without_turn(),
        )
        .expect("run should start");

        assert_eq!(run.session().tenant_id, "tenant-1");
        assert_eq!(run.task().session_id, run.session().session_id);
        assert_eq!(run.attempt().task_id, run.task().task_id);
        assert!(run.attempt().turn_id.is_none());
        assert_eq!(run.attempt().harness_id, "omnisolo");
        assert_eq!(
            run.events()[0].idempotency_key.as_deref(),
            Some("request-1")
        );
    }

    #[test]
    fn run_config_builders_preserve_explicit_ids_and_metadata() {
        let session_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();
        let run = OmniSoloHarnessAdapter::start(
            OmniSoloRunConfig::new("tenant-1", "finish the task")
                .with_session_id(session_id)
                .with_task_id(task_id)
                .with_workspace("project-1", "workspace-1")
                .with_harness("omnisolo-custom")
                .with_worker("worker-1")
                .with_idempotency_key("request-1"),
        )
        .unwrap();

        assert_eq!(run.session().session_id, session_id);
        assert_eq!(run.task().task_id, task_id);
        assert_eq!(run.session().project_id.as_deref(), Some("project-1"));
        assert_eq!(run.session().workspace_id.as_deref(), Some("workspace-1"));
        assert_eq!(run.attempt().harness_id, "omnisolo-custom");
        assert_eq!(run.attempt().worker_id.as_deref(), Some("worker-1"));
        assert!(format!("{run:?}").contains("attempt_history_count"));

        assert_eq!(run.events().first().unwrap().durable_sequence, Some(1));
    }

    #[test]
    fn adapter_rejects_invalid_configuration_and_import_targets() {
        assert!(matches!(
            OmniSoloHarnessAdapter::start(OmniSoloRunConfig::new(" ", "task")),
            Err(AdapterError::EmptyTenant)
        ));
        assert!(matches!(
            OmniSoloHarnessAdapter::start(OmniSoloRunConfig::new("tenant-1", " ")),
            Err(AdapterError::EmptyObjective)
        ));
        assert!(matches!(
            OmniSoloHarnessAdapter::start(
                OmniSoloRunConfig::new("tenant-1", "task").with_harness(" ")
            ),
            Err(AdapterError::EmptyHarnessId)
        ));

        let mut run =
            OmniSoloHarnessAdapter::start(OmniSoloRunConfig::new("tenant-1", "task")).unwrap();
        run.record(OmniSoloEvent::TaskComplete {
            content: "done".to_owned(),
        })
        .unwrap();
        let capsule = run
            .export_capsule("future-harness", Uuid::new_v4())
            .unwrap();
        assert_eq!(
            OmniSoloHarnessAdapter::import_capsule(&capsule, "other-tenant", "future-harness"),
            Err(AdapterError::TenantMismatch)
        );
        assert_eq!(
            OmniSoloHarnessAdapter::import_capsule(&capsule, "tenant-1", "wrong-harness"),
            Err(AdapterError::TargetHarnessMismatch)
        );
    }

    #[test]
    fn omni_solo_run_preserves_explicit_session_metadata() {
        let run = OmniSoloHarnessAdapter::start(
            OmniSoloRunConfig::new("tenant-1", "finish the task").with_session_metadata(
                Some("project-1".to_owned()),
                Some("workspace-1".to_owned()),
                Some("Imported task".to_owned()),
                ["priority".to_owned()].into_iter().collect(),
                [("source".to_owned(), "capsule".to_owned())]
                    .into_iter()
                    .collect(),
            ),
        )
        .expect("run should start");

        assert_eq!(run.session().project_id.as_deref(), Some("project-1"));
        assert_eq!(run.session().workspace_id.as_deref(), Some("workspace-1"));
        assert_eq!(run.session().title.as_deref(), Some("Imported task"));
        assert!(run.session().labels.contains("priority"));
        assert_eq!(
            run.session().tags.get("source"),
            Some(&"capsule".to_owned())
        );
    }

    #[test]
    fn omni_solo_events_keep_lineage_and_durable_replay_cursors() {
        let mut run = OmniSoloHarnessAdapter::start(
            OmniSoloRunConfig::new("tenant-1", "finish the task").with_turn(),
        )
        .expect("run should start");

        let first = run
            .record(OmniSoloEvent::TextChunk {
                content: "partial".to_owned(),
            })
            .expect("transient event");
        let second = run
            .record(OmniSoloEvent::ToolCall {
                name: "read_file".to_owned(),
                args_json: "{\"path\":\"README.md\"}".to_owned(),
                result: "contents".to_owned(),
                iteration: 1,
            })
            .expect("durable event");

        assert!(first.event.durable_sequence.is_none());
        assert_eq!(second.event.durable_sequence, Some(2));
        assert_eq!(second.event.parent_event_ids.len(), 1);
        assert_eq!(
            second.event.source_attempt_id,
            Some(run.attempt().attempt_id)
        );
        assert_eq!(
            second.event.ingest_attempt_id,
            Some(run.attempt().attempt_id)
        );
        assert_eq!(run.replay(1, 2).len(), 2);
    }

    #[test]
    fn historical_events_project_and_handoff_fences_the_omnisolo_run() {
        let mut run = OmniSoloHarnessAdapter::start(
            OmniSoloRunConfig::new("tenant-1", "handoff the task").with_turn(),
        )
        .unwrap();
        for event in [
            OmniSoloEvent::IterationStarted {
                iteration: 1,
                message_count: 2,
            },
            OmniSoloEvent::CheckpointSaved {
                iteration: 1,
                path: "checkpoint-1".to_owned(),
            },
            OmniSoloEvent::RewindOccurred {
                iteration: 1,
                checkpoint_id: "checkpoint-1".to_owned(),
                reason: "operator requested".to_owned(),
            },
            OmniSoloEvent::GuardrailTripped {
                reason: "policy".to_owned(),
            },
            OmniSoloEvent::CostUpdate {
                total_cost_usd: 0.12,
            },
        ] {
            run.record(event).unwrap();
        }
        run.record(OmniSoloEvent::Handoff {
            target_agent: "opencode".to_owned(),
        })
        .unwrap();

        assert_eq!(run.session().state, SessionState::HandingOff);
        assert_eq!(run.task().state, TaskState::HandingOff);
        assert_eq!(run.attempt().state, AttemptState::Quiescing);
        assert!(
            run.canonical_records
                .iter()
                .filter(|record| matches!(record, CanonicalRecord::Unsupported { .. }))
                .count()
                >= 5
        );
    }

    #[test]
    fn waiting_for_user_input_preserves_turn_state_and_rejects_completion() {
        let mut run = OmniSoloHarnessAdapter::start(
            OmniSoloRunConfig::new("tenant-1", "need approval").with_turn(),
        )
        .unwrap();
        run.record(OmniSoloEvent::UserInterventionRequired {
            error: "approval required".to_owned(),
        })
        .unwrap();

        assert_eq!(run.task().state, TaskState::WaitingInput);
        assert_eq!(run.attempt().state, AttemptState::Quiescing);
        assert_eq!(run.turn().unwrap().state, TurnState::WaitingInput);
        assert!(matches!(
            run.record(OmniSoloEvent::TaskComplete {
                content: "late completion".to_owned(),
            }),
            Err(AdapterError::Lifecycle(_))
        ));
    }

    #[test]
    fn stale_worker_output_is_rejected_after_lease_reassignment() {
        let mut run =
            OmniSoloHarnessAdapter::start(OmniSoloRunConfig::new("tenant-1", "finish the task"))
                .expect("run should start");
        let stale_fence = run.fence_token();
        run.reassign_worker("worker-2").expect("reassignment");

        assert!(matches!(
            run.record_with_fence(
                OmniSoloEvent::TextChunk {
                    content: "late".to_owned(),
                },
                stale_fence,
            ),
            Err(AdapterError::EventStore(EventStoreError::Fence(
                FenceError::StaleGeneration
            )))
        ));
    }

    #[test]
    fn capsule_export_and_import_preserve_portable_session_identity() {
        let mut run = OmniSoloHarnessAdapter::start(
            OmniSoloRunConfig::new("tenant-1", "finish the task").with_turn(),
        )
        .expect("run should start");
        run.record(OmniSoloEvent::ToolCall {
            name: "read_file".to_owned(),
            args_json: "{\"path\":\"README.md\"}".to_owned(),
            result: "contents".to_owned(),
            iteration: 1,
        })
        .expect("tool event");
        run.record(OmniSoloEvent::TaskComplete {
            content: "done".to_owned(),
        })
        .expect("completion");

        let capsule = run
            .export_capsule("future-harness", Uuid::new_v4())
            .expect("capsule export");
        let imported =
            OmniSoloHarnessAdapter::import_capsule(&capsule, "tenant-1", "future-harness")
                .expect("capsule import");

        assert_eq!(imported.session_id, run.session().session_id);
        assert_eq!(imported.tenant_id, "tenant-1");
        assert_eq!(imported.target_harness_id, "future-harness");
        assert!(
            capsule
                .records
                .iter()
                .any(|record| matches!(record, PortableRecord::ToolResult(_)))
        );
        assert!(
            capsule
                .records
                .iter()
                .any(|record| matches!(record, PortableRecord::Message(_)))
        );
        assert!(
            capsule
                .records
                .iter()
                .any(|record| matches!(record, PortableRecord::Attempt(_)))
        );
    }

    #[test]
    fn recovery_creates_a_new_fenced_attempt_and_preserves_task_history() {
        let mut run = OmniSoloHarnessAdapter::start(
            OmniSoloRunConfig::new("tenant-1", "retry the task").with_turn(),
        )
        .expect("run should start");
        let old_attempt_id = run.attempt().attempt_id;
        run.record(OmniSoloEvent::TaskError {
            error: "temporary failure".to_owned(),
        })
        .expect("failure event");
        run.begin_recovery_attempt("worker-recovery")
            .expect("recovery attempt");

        assert_ne!(run.attempt().attempt_id, old_attempt_id);
        assert_eq!(run.attempt().parent_attempt_id, Some(old_attempt_id));
        assert_eq!(run.attempt().kind, AttemptKind::Recovery);
        assert_eq!(run.task().state, TaskState::Running);
        assert_eq!(run.turn().unwrap().sequence, 2);
        assert_eq!(run.attempt().worker_id.as_deref(), Some("worker-recovery"));
        assert!(matches!(
            run.begin_recovery_attempt("worker-again"),
            Err(AdapterError::RecoveryNotAllowed)
        ));

        let capsule = run
            .export_capsule("future-harness", Uuid::new_v4())
            .expect("recovery capsule export");
        let attempts = capsule
            .records
            .iter()
            .filter_map(|record| match record {
                PortableRecord::Attempt(attempt) => Some(attempt),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(attempts.len(), 2);
        assert!(attempts.iter().any(|attempt| {
            attempt.attempt_id == old_attempt_id && attempt.state == AttemptState::Failed
        }));
        assert!(attempts.iter().any(|attempt| {
            attempt.attempt_id == run.attempt().attempt_id
                && attempt.parent_attempt_id == Some(old_attempt_id)
        }));
    }

    #[test]
    fn successful_attempt_cannot_be_reopened_as_recovery() {
        let mut run = OmniSoloHarnessAdapter::start(
            OmniSoloRunConfig::new("tenant-1", "finish the task").with_turn(),
        )
        .expect("run should start");
        run.record(OmniSoloEvent::TaskComplete {
            content: "done".to_owned(),
        })
        .expect("completion");

        assert_eq!(
            run.begin_recovery_attempt("worker-2"),
            Err(AdapterError::RecoveryNotAllowed)
        );
        assert!(matches!(
            run.record(OmniSoloEvent::TextChunk {
                content: "late output".to_owned(),
            }),
            Err(AdapterError::TerminalAttempt)
        ));
    }

    #[test]
    fn rejected_lifecycle_event_does_not_advance_the_event_log() {
        let mut run = OmniSoloHarnessAdapter::start(
            OmniSoloRunConfig::new("tenant-1", "wait for approval").with_turn(),
        )
        .unwrap();
        run.record(OmniSoloEvent::UserInterventionRequired {
            error: "approval required".to_owned(),
        })
        .unwrap();
        let event_count = run.events().len();
        let durable_sequence = run.last_durable_sequence();
        let canonical_count = run.canonical_records.len();

        assert!(matches!(
            run.record(OmniSoloEvent::TaskComplete {
                content: "should not be accepted while waiting for input".to_owned(),
            }),
            Err(AdapterError::Lifecycle(_))
        ));
        assert_eq!(run.events().len(), event_count);
        assert_eq!(run.last_durable_sequence(), durable_sequence);
        assert_eq!(run.canonical_records.len(), canonical_count);
    }
}
