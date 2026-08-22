use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::types::JsonMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerControlKind {
    Register,
    Heartbeat,
    CapabilitySnapshot,
    Drain,
    Shutdown,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionOperationKind {
    Handoff,
    Quiesce,
    Resume,
    Cancel,
    Snapshot,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AttemptCommandKind {
    Execute,
    Resume,
    Quiesce,
    Cancel,
    Checkpoint,
    Reconcile,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum EnvelopeError {
    MissingWorkerId,
    MissingRuntimeId,
    MissingTenantId,
    MissingSessionId,
    MissingOperationId,
    InvalidOperationGeneration,
    MissingFencingToken,
    MissingTaskId,
    MissingAttemptId,
    MissingCommandId,
    MissingLeaseId,
    InvalidLeaseGeneration,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WorkerControlEnvelope {
    pub protocol_version: u32,
    pub worker_id: String,
    pub runtime_id: String,
    pub kind: WorkerControlKind,
    pub session_id: Option<Uuid>,
    pub correlation_id: Option<Uuid>,
    pub idempotency_key: Option<String>,
    pub capability_version: u64,
    pub payload_schema: String,
    pub payload_version: u32,
    pub payload: Value,
    pub extensions: JsonMap,
}

impl WorkerControlEnvelope {
    pub fn heartbeat(worker_id: String, runtime_id: String, correlation_id: Uuid) -> Self {
        Self {
            protocol_version: 1,
            worker_id,
            runtime_id,
            kind: WorkerControlKind::Heartbeat,
            session_id: None,
            correlation_id: Some(correlation_id),
            idempotency_key: None,
            capability_version: 0,
            payload_schema: "omnisolo.worker.heartbeat.v1".to_owned(),
            payload_version: 1,
            payload: Value::Null,
            extensions: JsonMap::new(),
        }
    }

    pub fn validate(&self) -> Result<(), EnvelopeError> {
        if self.worker_id.is_empty() {
            return Err(EnvelopeError::MissingWorkerId);
        }
        if self.runtime_id.is_empty() {
            return Err(EnvelopeError::MissingRuntimeId);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SessionOperationEnvelope {
    pub protocol_version: u32,
    pub tenant_id: String,
    pub session_id: Uuid,
    pub operation_id: Uuid,
    pub operation_generation: i64,
    pub fencing_token: String,
    pub kind: SessionOperationKind,
    pub task_id: Option<Uuid>,
    pub correlation_id: Option<Uuid>,
    pub idempotency_key: Option<String>,
    pub payload_schema: String,
    pub payload_version: u32,
    pub payload: Value,
    pub extensions: JsonMap,
}

impl SessionOperationEnvelope {
    pub fn new(
        tenant_id: String,
        session_id: Uuid,
        operation_id: Uuid,
        operation_generation: i64,
        fencing_token: String,
        kind: SessionOperationKind,
    ) -> Self {
        Self {
            protocol_version: 1,
            tenant_id,
            session_id,
            operation_id,
            operation_generation,
            fencing_token,
            kind,
            task_id: None,
            correlation_id: None,
            idempotency_key: None,
            payload_schema: "omnisolo.session.operation.v1".to_owned(),
            payload_version: 1,
            payload: Value::Null,
            extensions: JsonMap::new(),
        }
    }

    pub fn validate(&self) -> Result<(), EnvelopeError> {
        if self.tenant_id.is_empty() {
            return Err(EnvelopeError::MissingTenantId);
        }
        if self.session_id.is_nil() {
            return Err(EnvelopeError::MissingSessionId);
        }
        if self.operation_id.is_nil() {
            return Err(EnvelopeError::MissingOperationId);
        }
        if self.operation_generation < 0 {
            return Err(EnvelopeError::InvalidOperationGeneration);
        }
        if self.fencing_token.is_empty() {
            return Err(EnvelopeError::MissingFencingToken);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AttemptCommandEnvelope {
    pub protocol_version: u32,
    pub tenant_id: String,
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub attempt_id: Uuid,
    pub turn_id: Option<Uuid>,
    pub command_id: Uuid,
    pub lease_id: Uuid,
    pub lease_generation: i64,
    pub fencing_token: String,
    pub kind: AttemptCommandKind,
    pub correlation_id: Option<Uuid>,
    pub idempotency_key: Option<String>,
    pub payload_schema: String,
    pub payload_version: u32,
    pub payload: Value,
    pub extensions: JsonMap,
}

impl AttemptCommandEnvelope {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tenant_id: String,
        session_id: Uuid,
        task_id: Uuid,
        attempt_id: Uuid,
        turn_id: Option<Uuid>,
        lease_id: Uuid,
        lease_generation: i64,
        fencing_token: String,
        kind: AttemptCommandKind,
    ) -> Self {
        Self {
            protocol_version: 1,
            tenant_id,
            session_id,
            task_id: Some(task_id),
            attempt_id,
            turn_id,
            command_id: Uuid::new_v4(),
            lease_id,
            lease_generation,
            fencing_token,
            kind,
            correlation_id: None,
            idempotency_key: None,
            payload_schema: "omnisolo.attempt.command.v1".to_owned(),
            payload_version: 1,
            payload: Value::Null,
            extensions: JsonMap::new(),
        }
    }

    pub fn validate(&self) -> Result<(), EnvelopeError> {
        if self.tenant_id.is_empty() {
            return Err(EnvelopeError::MissingTenantId);
        }
        if self.session_id.is_nil() {
            return Err(EnvelopeError::MissingSessionId);
        }
        if self.task_id.is_none_or(|task_id| task_id.is_nil()) {
            return Err(EnvelopeError::MissingTaskId);
        }
        if self.attempt_id.is_nil() {
            return Err(EnvelopeError::MissingAttemptId);
        }
        if self.command_id.is_nil() {
            return Err(EnvelopeError::MissingCommandId);
        }
        if self.lease_id.is_nil() {
            return Err(EnvelopeError::MissingLeaseId);
        }
        if self.lease_generation < 0 {
            return Err(EnvelopeError::InvalidLeaseGeneration);
        }
        if self.fencing_token.is_empty() {
            return Err(EnvelopeError::MissingFencingToken);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn worker_control_does_not_require_session_identity() {
        let envelope = WorkerControlEnvelope::heartbeat(
            "worker-1".to_owned(),
            "runtime-1".to_owned(),
            Uuid::new_v4(),
        );
        envelope.validate().unwrap();
        assert!(envelope.session_id.is_none());
        assert_eq!(envelope.kind, WorkerControlKind::Heartbeat);
    }

    #[test]
    fn session_operations_require_operation_generation_and_fence() {
        let envelope = SessionOperationEnvelope::new(
            "tenant-1".to_owned(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            4,
            "fence-4".to_owned(),
            SessionOperationKind::Handoff,
        );
        envelope.validate().unwrap();

        let mut missing_fence = envelope.clone();
        missing_fence.fencing_token.clear();
        assert_eq!(
            missing_fence.validate(),
            Err(EnvelopeError::MissingFencingToken)
        );
    }

    #[test]
    fn task_level_attempt_commands_allow_no_turn_but_require_task_attempt_lease() {
        let envelope = AttemptCommandEnvelope::new(
            "tenant-1".to_owned(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            None,
            Uuid::new_v4(),
            9,
            "fence-9".to_owned(),
            AttemptCommandKind::Execute,
        );
        envelope.validate().unwrap();
        assert!(envelope.turn_id.is_none());

        let mut missing_task = envelope.clone();
        missing_task.task_id = None;
        assert_eq!(missing_task.validate(), Err(EnvelopeError::MissingTaskId));
    }

    #[test]
    fn envelope_json_round_trip_preserves_versioned_unknown_fields() {
        let mut envelope = AttemptCommandEnvelope::new(
            "tenant-1".to_owned(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            Some(Uuid::new_v4()),
            Uuid::new_v4(),
            2,
            "fence-2".to_owned(),
            AttemptCommandKind::Cancel,
        );
        envelope.payload_schema = "omnisolo.harness.command.v1".to_owned();
        envelope.payload_version = 7;
        envelope.extensions.insert(
            "future_field".to_owned(),
            serde_json::json!({"enabled": true}),
        );
        let decoded: AttemptCommandEnvelope =
            serde_json::from_value(serde_json::to_value(&envelope).unwrap()).unwrap();
        assert_eq!(decoded, envelope);
        assert_eq!(decoded.payload_version, 7);
    }
}
