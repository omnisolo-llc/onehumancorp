use std::collections::{HashMap, HashSet};

use super::worker::{
    AttemptCommandEnvelope, SessionOperationEnvelope, WorkerControlEnvelope, WorkerControlKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkerRuntimeError {
    InvalidEnvelope,
    WrongWorker,
    WrongHarness,
    WrongPool,
    Draining,
    StaleOperation,
    OperationConflict,
    StaleLease,
    LeaseConflict,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkerReply {
    pub accepted: bool,
    pub duplicate: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkerHealth {
    pub worker_id: String,
    pub pool_id: String,
    pub harness_id: String,
    pub ready: bool,
    pub accepting_new_attempts: bool,
    pub active_attempts: usize,
}

#[derive(Clone, Debug)]
pub struct HarnessWorkerRuntime {
    worker_id: String,
    harness_id: String,
    pool_id: String,
    ready: bool,
    accepting_new_attempts: bool,
    operations: HashMap<uuid::Uuid, (i64, String)>,
    attempts: HashMap<uuid::Uuid, (uuid::Uuid, i64, String)>,
    terminal_attempts: HashSet<uuid::Uuid>,
    command_fences: HashMap<uuid::Uuid, (uuid::Uuid, uuid::Uuid, i64, String)>,
    idempotency_fences: HashMap<(uuid::Uuid, String), (uuid::Uuid, i64, String)>,
}

impl HarnessWorkerRuntime {
    pub fn new(
        worker_id: impl Into<String>,
        harness_id: impl Into<String>,
        pool_id: impl Into<String>,
    ) -> Self {
        Self {
            worker_id: worker_id.into(),
            harness_id: harness_id.into(),
            pool_id: pool_id.into(),
            ready: true,
            accepting_new_attempts: true,
            operations: HashMap::new(),
            attempts: HashMap::new(),
            terminal_attempts: HashSet::new(),
            command_fences: HashMap::new(),
            idempotency_fences: HashMap::new(),
        }
    }

    pub fn retire_attempt(&mut self, attempt: uuid::Uuid) {
        self.terminal_attempts.insert(attempt);
    }

    pub fn health(&self) -> WorkerHealth {
        WorkerHealth {
            worker_id: self.worker_id.clone(),
            pool_id: self.pool_id.clone(),
            harness_id: self.harness_id.clone(),
            ready: self.ready,
            accepting_new_attempts: self.accepting_new_attempts,
            active_attempts: self
                .attempts
                .keys()
                .filter(|id| !self.terminal_attempts.contains(id))
                .count(),
        }
    }

    pub fn handle_control(
        &mut self,
        envelope: WorkerControlEnvelope,
    ) -> Result<WorkerReply, WorkerRuntimeError> {
        envelope
            .validate()
            .map_err(|_| WorkerRuntimeError::InvalidEnvelope)?;
        if envelope.worker_id != self.worker_id {
            return Err(WorkerRuntimeError::WrongWorker);
        }
        if envelope.runtime_id != self.harness_id {
            return Err(WorkerRuntimeError::WrongHarness);
        }
        match envelope.kind {
            WorkerControlKind::Register
            | WorkerControlKind::Heartbeat
            | WorkerControlKind::CapabilitySnapshot => {
                self.ready = true;
            }
            WorkerControlKind::Drain => {
                self.accepting_new_attempts = false;
            }
            WorkerControlKind::Shutdown => {
                self.ready = false;
                self.accepting_new_attempts = false;
            }
        }
        Ok(WorkerReply {
            accepted: true,
            duplicate: false,
        })
    }

    pub fn handle_session_operation(
        &mut self,
        envelope: SessionOperationEnvelope,
    ) -> Result<WorkerReply, WorkerRuntimeError> {
        envelope
            .validate()
            .map_err(|_| WorkerRuntimeError::InvalidEnvelope)?;
        self.validate_worker(
            Some(envelope.worker_id.as_str()),
            Some(envelope.harness_id.as_str()),
            Some(envelope.pool_id.as_str()),
        )?;
        let operation = self.operations.get(&envelope.operation_id).cloned();
        if let Some((generation, token)) = operation {
            if envelope.operation_generation < generation {
                return Err(WorkerRuntimeError::StaleOperation);
            }
            if envelope.operation_generation == generation {
                if token == envelope.fencing_token {
                    return Ok(WorkerReply {
                        accepted: true,
                        duplicate: true,
                    });
                }
                return Err(WorkerRuntimeError::OperationConflict);
            }
        }
        self.operations.insert(
            envelope.operation_id,
            (envelope.operation_generation, envelope.fencing_token),
        );
        Ok(WorkerReply {
            accepted: true,
            duplicate: false,
        })
    }

    pub fn handle_attempt_command(
        &mut self,
        envelope: AttemptCommandEnvelope,
    ) -> Result<WorkerReply, WorkerRuntimeError> {
        if envelope.lease_generation <= 0 {
            return Err(WorkerRuntimeError::StaleLease);
        }
        envelope
            .validate()
            .map_err(|_| WorkerRuntimeError::InvalidEnvelope)?;
        let pool_id = (!envelope.pool_id.is_empty()).then_some(envelope.pool_id.as_str());
        self.validate_worker(
            Some(envelope.worker_id.as_str()),
            Some(envelope.harness_id.as_str()),
            pool_id,
        )?;
        if !self.accepting_new_attempts
            && matches!(
                envelope.kind,
                super::worker::AttemptCommandKind::Start
                    | super::worker::AttemptCommandKind::Execute
                    | super::worker::AttemptCommandKind::Resume
            )
        {
            return Err(WorkerRuntimeError::Draining);
        }
        if let Some((lease_id, generation, token)) = self.attempts.get(&envelope.attempt_id) {
            if envelope.lease_generation < *generation {
                return Err(WorkerRuntimeError::StaleLease);
            }
            if envelope.lease_generation == *generation
                && (*lease_id != envelope.lease_id || token != &envelope.fencing_token)
            {
                return Err(WorkerRuntimeError::LeaseConflict);
            }
        }

        if let Some((attempt_id, lease_id, generation, token)) =
            self.command_fences.get(&envelope.command_id)
        {
            if *attempt_id != envelope.attempt_id {
                return Err(WorkerRuntimeError::LeaseConflict);
            }
            if envelope.lease_generation < *generation {
                return Err(WorkerRuntimeError::StaleLease);
            }
            if *lease_id == envelope.lease_id && token == &envelope.fencing_token {
                return Ok(WorkerReply {
                    accepted: true,
                    duplicate: true,
                });
            }
            return Ok(WorkerReply {
                accepted: true,
                duplicate: true,
            });
        }

        if let Some(idempotency_key) = envelope.idempotency_key.as_deref()
            && let Some((lease_id, generation, token)) = self
                .idempotency_fences
                .get(&(envelope.attempt_id, idempotency_key.to_owned()))
        {
            if envelope.lease_generation < *generation {
                return Err(WorkerRuntimeError::StaleLease);
            }
            if *lease_id == envelope.lease_id && token == &envelope.fencing_token {
                return Ok(WorkerReply {
                    accepted: true,
                    duplicate: true,
                });
            }
            return Ok(WorkerReply {
                accepted: true,
                duplicate: true,
            });
        }

        self.attempts.insert(
            envelope.attempt_id,
            (
                envelope.lease_id,
                envelope.lease_generation,
                envelope.fencing_token.clone(),
            ),
        );
        self.command_fences.insert(
            envelope.command_id,
            (
                envelope.attempt_id,
                envelope.lease_id,
                envelope.lease_generation,
                envelope.fencing_token.clone(),
            ),
        );
        if let Some(idempotency_key) = envelope.idempotency_key {
            self.idempotency_fences.insert(
                (envelope.attempt_id, idempotency_key),
                (
                    envelope.lease_id,
                    envelope.lease_generation,
                    envelope.fencing_token,
                ),
            );
        }
        Ok(WorkerReply {
            accepted: true,
            duplicate: false,
        })
    }

    pub fn rollback_session_operation(&mut self, envelope: &SessionOperationEnvelope) {
        if self
            .operations
            .get(&envelope.operation_id)
            .is_some_and(|(generation, token)| {
                *generation == envelope.operation_generation && token == &envelope.fencing_token
            })
        {
            self.operations.remove(&envelope.operation_id);
        }
    }

    pub fn rollback_attempt_command(&mut self, envelope: &AttemptCommandEnvelope) {
        if self.command_fences.get(&envelope.command_id).is_some_and(
            |(attempt_id, lease_id, generation, token)| {
                *attempt_id == envelope.attempt_id
                    && *lease_id == envelope.lease_id
                    && *generation == envelope.lease_generation
                    && token == &envelope.fencing_token
            },
        ) {
            self.command_fences.remove(&envelope.command_id);
        }
        if let Some(idempotency_key) = envelope.idempotency_key.as_deref()
            && self
                .idempotency_fences
                .get(&(envelope.attempt_id, idempotency_key.to_owned()))
                .is_some_and(|(lease_id, generation, token)| {
                    *lease_id == envelope.lease_id
                        && *generation == envelope.lease_generation
                        && token == &envelope.fencing_token
                })
        {
            self.idempotency_fences
                .remove(&(envelope.attempt_id, idempotency_key.to_owned()));
        }
    }

    fn validate_worker(
        &self,
        worker_id: Option<&str>,
        harness_id: Option<&str>,
        pool_id: Option<&str>,
    ) -> Result<(), WorkerRuntimeError> {
        if worker_id.is_some_and(|worker_id| worker_id != self.worker_id) {
            return Err(WorkerRuntimeError::WrongWorker);
        }
        if harness_id.is_some_and(|harness_id| harness_id != self.harness_id) {
            return Err(WorkerRuntimeError::WrongHarness);
        }
        if pool_id.is_some_and(|pool_id| pool_id != self.pool_id) {
            return Err(WorkerRuntimeError::WrongPool);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::worker::{AttemptCommandKind, SessionOperationKind};
    use uuid::Uuid;

    #[test]
    fn terminal_attempts_leave_health_but_preserve_fencing() {
        let mut runtime = HarnessWorkerRuntime::new("worker-1", "codex", "pool-1");
        let attempt = Uuid::new_v4();
        let lease = Uuid::new_v4();
        let accepted = command(
            attempt,
            Uuid::new_v4(),
            lease,
            2,
            "current",
            AttemptCommandKind::Execute,
        );
        runtime.handle_attempt_command(accepted).unwrap();
        assert_eq!(runtime.health().active_attempts, 1);
        runtime.retire_attempt(attempt);
        assert_eq!(runtime.health().active_attempts, 0);
        let stale = command(
            attempt,
            Uuid::new_v4(),
            lease,
            1,
            "stale",
            AttemptCommandKind::Cancel,
        );
        assert_eq!(
            runtime.handle_attempt_command(stale),
            Err(WorkerRuntimeError::StaleLease)
        );
    }

    fn operation(operation_id: Uuid, generation: i64, token: &str) -> SessionOperationEnvelope {
        SessionOperationEnvelope::new(
            "tenant-1".to_owned(),
            Uuid::from_u128(1),
            operation_id,
            generation,
            token.to_owned(),
            SessionOperationKind::Resume,
        )
        .with_worker("worker-1", "codex", "pool-1", 1)
    }

    fn command(
        attempt_id: Uuid,
        command_id: Uuid,
        lease_id: Uuid,
        generation: i64,
        token: &str,
        kind: AttemptCommandKind,
    ) -> AttemptCommandEnvelope {
        let mut envelope = AttemptCommandEnvelope::new(
            "tenant-1".to_owned(),
            Uuid::from_u128(1),
            Uuid::from_u128(2),
            attempt_id,
            None,
            lease_id,
            generation,
            token.to_owned(),
            kind,
        )
        .with_worker("worker-1", "codex", 1)
        .with_pool("pool-1");
        envelope.command_id = command_id;
        envelope
    }

    #[test]
    fn runtime_unit_contract_covers_control_fences_and_rollbacks() {
        let mut runtime = HarnessWorkerRuntime::new("worker-1", "codex", "pool-1");
        assert_eq!(runtime.health().active_attempts, 0);

        let mut invalid = WorkerControlEnvelope::heartbeat(
            "worker-1".to_owned(),
            "codex".to_owned(),
            Uuid::new_v4(),
        );
        invalid.worker_id.clear();
        assert_eq!(
            runtime.handle_control(invalid),
            Err(WorkerRuntimeError::InvalidEnvelope)
        );
        let wrong_worker = WorkerControlEnvelope::heartbeat(
            "other".to_owned(),
            "codex".to_owned(),
            Uuid::new_v4(),
        );
        assert_eq!(
            runtime.handle_control(wrong_worker),
            Err(WorkerRuntimeError::WrongWorker)
        );
        let wrong_harness = WorkerControlEnvelope::heartbeat(
            "worker-1".to_owned(),
            "other".to_owned(),
            Uuid::new_v4(),
        );
        assert_eq!(
            runtime.handle_control(wrong_harness),
            Err(WorkerRuntimeError::WrongHarness)
        );
        for kind in [
            WorkerControlKind::Register,
            WorkerControlKind::Heartbeat,
            WorkerControlKind::CapabilitySnapshot,
        ] {
            let mut control = WorkerControlEnvelope::heartbeat(
                "worker-1".to_owned(),
                "codex".to_owned(),
                Uuid::new_v4(),
            );
            control.kind = kind;
            assert!(runtime.handle_control(control).unwrap().accepted);
        }
        let mut drain = WorkerControlEnvelope::heartbeat(
            "worker-1".to_owned(),
            "codex".to_owned(),
            Uuid::new_v4(),
        );
        drain.kind = WorkerControlKind::Drain;
        runtime.handle_control(drain).unwrap();
        assert!(!runtime.health().accepting_new_attempts);

        let mut runtime = HarnessWorkerRuntime::new("worker-1", "codex", "pool-1");

        let mut shutdown = HarnessWorkerRuntime::new("worker-1", "codex", "pool-1");
        let mut control = WorkerControlEnvelope::heartbeat(
            "worker-1".to_owned(),
            "codex".to_owned(),
            Uuid::new_v4(),
        );
        control.kind = WorkerControlKind::Shutdown;
        shutdown.handle_control(control).unwrap();
        assert!(!shutdown.health().ready);

        let operation_id = Uuid::from_u128(3);
        let first_operation = operation(operation_id, 1, "op-1");
        assert!(
            !runtime
                .handle_session_operation(first_operation.clone())
                .unwrap()
                .duplicate
        );
        assert!(
            runtime
                .handle_session_operation(first_operation.clone())
                .unwrap()
                .duplicate
        );
        let mut conflict = first_operation.clone();
        conflict.fencing_token = "other".to_owned();
        assert_eq!(
            runtime.handle_session_operation(conflict),
            Err(WorkerRuntimeError::OperationConflict)
        );
        let mut stale = first_operation.clone();
        stale.operation_generation = 0;
        assert_eq!(
            runtime.handle_session_operation(stale),
            Err(WorkerRuntimeError::InvalidEnvelope)
        );
        let mut newer = first_operation.clone();
        newer.operation_generation = 2;
        newer.fencing_token = "op-2".to_owned();
        assert!(
            !runtime
                .handle_session_operation(newer.clone())
                .unwrap()
                .duplicate
        );
        let mut stale_operation = newer.clone();
        stale_operation.operation_generation = 1;
        stale_operation.fencing_token = "op-old".to_owned();
        assert_eq!(
            runtime.handle_session_operation(stale_operation),
            Err(WorkerRuntimeError::StaleOperation)
        );
        runtime.rollback_session_operation(&newer);
        assert!(!runtime.handle_session_operation(newer).unwrap().duplicate);
        let mut missing_operation = first_operation.clone();
        missing_operation.operation_id = Uuid::from_u128(999);
        runtime.rollback_session_operation(&missing_operation);
        for (worker_id, harness_id, pool_id, error) in [
            ("other", "codex", "pool-1", WorkerRuntimeError::WrongWorker),
            (
                "worker-1",
                "other",
                "pool-1",
                WorkerRuntimeError::WrongHarness,
            ),
            ("worker-1", "codex", "other", WorkerRuntimeError::WrongPool),
        ] {
            assert_eq!(
                runtime.handle_session_operation(
                    operation(Uuid::new_v4(), 1, "routing")
                        .with_worker(worker_id, harness_id, pool_id, 1)
                ),
                Err(error)
            );
        }

        let attempt_id = Uuid::from_u128(4);
        let lease_id = Uuid::from_u128(5);
        let command_id = Uuid::from_u128(6);
        let first_command = command(
            attempt_id,
            command_id,
            lease_id,
            1,
            "lease-1",
            AttemptCommandKind::Execute,
        );
        assert!(
            !runtime
                .handle_attempt_command(first_command.clone())
                .unwrap()
                .duplicate
        );
        assert!(
            runtime
                .handle_attempt_command(first_command.clone())
                .unwrap()
                .duplicate
        );

        let mut wrong_attempt = first_command.clone();
        wrong_attempt.attempt_id = Uuid::from_u128(7);
        assert_eq!(
            runtime.handle_attempt_command(wrong_attempt),
            Err(WorkerRuntimeError::LeaseConflict)
        );
        let mut wrong_lease = first_command.clone();
        wrong_lease.command_id = Uuid::from_u128(8);
        wrong_lease.lease_id = Uuid::from_u128(9);
        assert_eq!(
            runtime.handle_attempt_command(wrong_lease),
            Err(WorkerRuntimeError::LeaseConflict)
        );
        let mut stale_lease = first_command.clone();
        stale_lease.command_id = Uuid::from_u128(10);
        stale_lease.lease_generation = 0;
        assert_eq!(
            runtime.handle_attempt_command(stale_lease),
            Err(WorkerRuntimeError::StaleLease)
        );

        let mut newer_command = first_command.clone();
        newer_command.command_id = Uuid::from_u128(11);
        newer_command.lease_generation = 2;
        newer_command.fencing_token = "lease-2".to_owned();
        assert!(
            !runtime
                .handle_attempt_command(newer_command.clone())
                .unwrap()
                .duplicate
        );
        let mut stale_again = newer_command.clone();
        stale_again.command_id = Uuid::from_u128(12);
        stale_again.lease_generation = 1;
        assert_eq!(
            runtime.handle_attempt_command(stale_again),
            Err(WorkerRuntimeError::StaleLease)
        );
        runtime.attempts.remove(&attempt_id);
        let mut command_fence_stale = newer_command.clone();
        command_fence_stale.lease_generation = 1;
        command_fence_stale.fencing_token = "lease-old".to_owned();
        assert_eq!(
            runtime.handle_attempt_command(command_fence_stale),
            Err(WorkerRuntimeError::StaleLease)
        );
        let mut command_fence_conflict = newer_command.clone();
        command_fence_conflict.fencing_token = "different".to_owned();
        assert!(
            runtime
                .handle_attempt_command(command_fence_conflict)
                .unwrap()
                .duplicate
        );

        let mut idempotent = command(
            attempt_id,
            Uuid::from_u128(13),
            lease_id,
            3,
            "lease-3",
            AttemptCommandKind::Steer,
        );
        idempotent.idempotency_key = Some("idem".to_owned());
        assert!(
            !runtime
                .handle_attempt_command(idempotent.clone())
                .unwrap()
                .duplicate
        );
        let mut idempotent_same = idempotent.clone();
        idempotent_same.command_id = Uuid::from_u128(14);
        assert!(
            runtime
                .handle_attempt_command(idempotent_same)
                .unwrap()
                .duplicate
        );
        let mut idempotent_other_token = idempotent.clone();
        idempotent_other_token.command_id = Uuid::from_u128(15);
        idempotent_other_token.lease_generation = 4;
        idempotent_other_token.fencing_token = "lease-4".to_owned();
        assert!(
            runtime
                .handle_attempt_command(idempotent_other_token)
                .unwrap()
                .duplicate
        );

        runtime.attempts.remove(&attempt_id);
        let mut idempotency_stale = idempotent.clone();
        idempotency_stale.command_id = Uuid::from_u128(16);
        idempotency_stale.lease_generation = 2;
        idempotency_stale.fencing_token = "lease-old".to_owned();
        assert_eq!(
            runtime.handle_attempt_command(idempotency_stale),
            Err(WorkerRuntimeError::StaleLease)
        );

        runtime.rollback_attempt_command(&idempotent);
        assert!(
            !runtime
                .handle_attempt_command(idempotent)
                .unwrap()
                .duplicate
        );
        let mut missing_command = first_command.clone();
        missing_command.command_id = Uuid::from_u128(998);
        missing_command.idempotency_key = Some("missing".to_owned());
        runtime.rollback_attempt_command(&missing_command);

        let mut wrong_pool = command(
            Uuid::from_u128(20),
            Uuid::from_u128(21),
            Uuid::from_u128(22),
            1,
            "pool-fence",
            AttemptCommandKind::Wait,
        );
        wrong_pool.pool_id = "other-pool".to_owned();
        assert_eq!(
            runtime.handle_attempt_command(wrong_pool),
            Err(WorkerRuntimeError::WrongPool)
        );

        let mut draining = HarnessWorkerRuntime::new("worker-1", "codex", "pool-1");
        let mut drain = WorkerControlEnvelope::heartbeat(
            "worker-1".to_owned(),
            "codex".to_owned(),
            Uuid::new_v4(),
        );
        drain.kind = WorkerControlKind::Drain;
        draining.handle_control(drain).unwrap();
        assert_eq!(
            draining.handle_attempt_command(command(
                Uuid::from_u128(30),
                Uuid::from_u128(31),
                Uuid::from_u128(32),
                1,
                "drain-fence",
                AttemptCommandKind::Start,
            )),
            Err(WorkerRuntimeError::Draining)
        );
        assert!(
            draining
                .handle_attempt_command(command(
                    Uuid::from_u128(30),
                    Uuid::from_u128(33),
                    Uuid::from_u128(34),
                    1,
                    "drain-safe-fence",
                    AttemptCommandKind::Wait,
                ))
                .unwrap()
                .accepted
        );
    }
}
