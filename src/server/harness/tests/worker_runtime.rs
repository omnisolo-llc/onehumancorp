use server_harness::middleware::worker::{
    AttemptCommandEnvelope, AttemptCommandKind, SessionOperationEnvelope, SessionOperationKind,
    WorkerControlEnvelope, WorkerControlKind,
};
use server_harness::middleware::worker_runtime::{HarnessWorkerRuntime, WorkerRuntimeError};
use uuid::Uuid;

#[test]
fn worker_runtime_handles_control_and_drain_state() {
    let mut worker = HarnessWorkerRuntime::new("worker-1", "codex", "codex-pool");
    let heartbeat =
        WorkerControlEnvelope::heartbeat("worker-1".to_owned(), "codex".to_owned(), Uuid::new_v4());
    assert!(worker.handle_control(heartbeat).unwrap().accepted);
    assert!(worker.health().ready);
    assert!(worker.health().accepting_new_attempts);

    let wrong_worker = WorkerControlEnvelope::heartbeat(
        "other-worker".to_owned(),
        "codex".to_owned(),
        Uuid::new_v4(),
    );
    assert_eq!(
        worker.handle_control(wrong_worker),
        Err(WorkerRuntimeError::WrongWorker)
    );
    let wrong_harness = WorkerControlEnvelope::heartbeat(
        "worker-1".to_owned(),
        "opencode".to_owned(),
        Uuid::new_v4(),
    );
    assert_eq!(
        worker.handle_control(wrong_harness),
        Err(WorkerRuntimeError::WrongHarness)
    );
    let mut invalid_control =
        WorkerControlEnvelope::heartbeat("worker-1".to_owned(), "codex".to_owned(), Uuid::new_v4());
    invalid_control.worker_id.clear();
    assert_eq!(
        worker.handle_control(invalid_control),
        Err(WorkerRuntimeError::InvalidEnvelope)
    );

    let mut drain =
        WorkerControlEnvelope::heartbeat("worker-1".to_owned(), "codex".to_owned(), Uuid::new_v4());
    drain.kind = server_harness::middleware::worker::WorkerControlKind::Drain;
    assert!(worker.handle_control(drain).unwrap().accepted);
    assert!(!worker.health().accepting_new_attempts);
}

#[test]
fn worker_runtime_fences_session_operations_and_attempt_commands() {
    let mut worker = HarnessWorkerRuntime::new("worker-1", "codex", "codex-pool");
    let tenant_id = "tenant-1".to_owned();
    let session_id = Uuid::new_v4();
    let operation_id = Uuid::new_v4();
    let mut operation = SessionOperationEnvelope::new(
        tenant_id.clone(),
        session_id,
        operation_id,
        1,
        "session-fence-1".to_owned(),
        SessionOperationKind::Resume,
    )
    .with_worker("worker-1", "codex", "codex-pool", 1);

    let accepted = worker.handle_session_operation(operation.clone()).unwrap();
    assert!(accepted.accepted);
    assert!(!accepted.duplicate);
    assert!(
        worker
            .handle_session_operation(operation.clone())
            .unwrap()
            .duplicate
    );
    let mut newer_operation = operation.clone();
    newer_operation.operation_generation = 2;
    newer_operation.fencing_token = "session-fence-2".to_owned();
    assert!(
        !worker
            .handle_session_operation(newer_operation.clone())
            .unwrap()
            .duplicate
    );
    assert_eq!(
        worker.handle_session_operation(operation.clone()),
        Err(WorkerRuntimeError::StaleOperation)
    );

    let wrong_pool = operation
        .clone()
        .with_worker("worker-1", "codex", "other-pool", 1);
    assert_eq!(
        worker.handle_session_operation(wrong_pool),
        Err(WorkerRuntimeError::WrongPool)
    );

    operation = operation.with_worker("other-worker", "codex", "codex-pool", 1);
    assert_eq!(
        worker.handle_session_operation(operation),
        Err(WorkerRuntimeError::WrongWorker)
    );

    let task_id = Uuid::new_v4();
    let attempt_id = Uuid::new_v4();
    let lease_id = Uuid::new_v4();
    let mut command = AttemptCommandEnvelope::new(
        tenant_id,
        session_id,
        task_id,
        attempt_id,
        None,
        lease_id,
        1,
        "lease-fence-1".to_owned(),
        AttemptCommandKind::Execute,
    )
    .with_worker("worker-1", "codex", 1);
    assert!(
        worker
            .handle_attempt_command(command.clone())
            .unwrap()
            .accepted
    );
    let mut wrong_harness_command = command.clone();
    wrong_harness_command.harness_id = "opencode".to_owned();
    assert_eq!(
        worker.handle_attempt_command(wrong_harness_command),
        Err(WorkerRuntimeError::WrongHarness)
    );
    let mut wrong_worker_command = command.clone();
    wrong_worker_command.worker_id = "other-worker".to_owned();
    assert_eq!(
        worker.handle_attempt_command(wrong_worker_command),
        Err(WorkerRuntimeError::WrongWorker)
    );
    let mut steer = command.clone();
    steer.kind = AttemptCommandKind::Steer;
    steer.command_id = Uuid::new_v4();
    steer.idempotency_key = Some("steer-1".to_owned());
    assert!(
        !worker
            .handle_attempt_command(steer.clone())
            .unwrap()
            .duplicate
    );
    assert!(worker.handle_attempt_command(steer).unwrap().duplicate);

    let mut idempotent_duplicate = command.clone();
    idempotent_duplicate.command_id = Uuid::new_v4();
    idempotent_duplicate.idempotency_key = Some("steer-1".to_owned());
    assert!(
        worker
            .handle_attempt_command(idempotent_duplicate)
            .unwrap()
            .duplicate
    );

    let mut same_command_other_attempt = command.clone();
    same_command_other_attempt.attempt_id = Uuid::new_v4();
    same_command_other_attempt.command_id = command.command_id;
    same_command_other_attempt.lease_id = Uuid::new_v4();
    same_command_other_attempt.fencing_token = "other-attempt-fence".to_owned();
    same_command_other_attempt.idempotency_key = None;
    assert_eq!(
        worker.handle_attempt_command(same_command_other_attempt),
        Err(WorkerRuntimeError::LeaseConflict)
    );

    let mut command_conflict = command.clone();
    command_conflict.lease_generation = 2;
    command_conflict.fencing_token = "lease-fence-2".to_owned();
    assert!(
        worker
            .handle_attempt_command(command_conflict)
            .unwrap()
            .duplicate
    );

    let mut idempotency_conflict = AttemptCommandEnvelope::new(
        "tenant-1".to_owned(),
        session_id,
        task_id,
        attempt_id,
        None,
        lease_id,
        2,
        "lease-fence-2".to_owned(),
        AttemptCommandKind::Steer,
    )
    .with_worker("worker-1", "codex", 1);
    idempotency_conflict.idempotency_key = Some("steer-1".to_owned());
    assert!(
        worker
            .handle_attempt_command(idempotency_conflict)
            .unwrap()
            .duplicate
    );

    let mut command_id_stale = command.clone();
    command_id_stale.attempt_id = Uuid::new_v4();
    command_id_stale.lease_generation = 0;
    assert_eq!(
        worker.handle_attempt_command(command_id_stale),
        Err(WorkerRuntimeError::StaleLease)
    );

    command.lease_generation = 0;
    assert_eq!(
        worker.handle_attempt_command(command),
        Err(WorkerRuntimeError::StaleLease)
    );

    let mut next = AttemptCommandEnvelope::new(
        "tenant-1".to_owned(),
        session_id,
        task_id,
        attempt_id,
        None,
        lease_id,
        2,
        "lease-fence-2".to_owned(),
        AttemptCommandKind::Resume,
    )
    .with_worker("worker-1", "codex", 1);
    assert!(
        worker
            .handle_attempt_command(next.clone())
            .unwrap()
            .accepted
    );
    next.lease_generation = 1;
    assert_eq!(
        worker.handle_attempt_command(next),
        Err(WorkerRuntimeError::StaleLease)
    );

    let mut operation_stale = SessionOperationEnvelope::new(
        "tenant-1".to_owned(),
        session_id,
        Uuid::new_v4(),
        0,
        "fence".to_owned(),
        SessionOperationKind::Snapshot,
    )
    .with_worker("worker-1", "codex", "codex-pool", 1);
    assert_eq!(
        worker.handle_session_operation(operation_stale.clone()),
        Err(WorkerRuntimeError::InvalidEnvelope)
    );
    operation_stale.operation_generation = 1;
    worker
        .handle_session_operation(operation_stale.clone())
        .unwrap();
    worker.rollback_session_operation(&operation_stale);
    assert!(
        !worker
            .handle_session_operation(operation_stale.clone())
            .unwrap()
            .duplicate
    );
    let mut operation_conflict = operation_stale.clone();
    operation_conflict.fencing_token = "other-fence".to_owned();
    assert_eq!(
        worker.handle_session_operation(operation_conflict),
        Err(WorkerRuntimeError::OperationConflict)
    );

    let mut wrong_pool = AttemptCommandEnvelope::new(
        "tenant-1".to_owned(),
        session_id,
        task_id,
        Uuid::new_v4(),
        None,
        Uuid::new_v4(),
        1,
        "pool-fence".to_owned(),
        AttemptCommandKind::Wait,
    )
    .with_worker("worker-1", "codex", 1)
    .with_pool("other-pool");
    assert_eq!(
        worker.handle_attempt_command(wrong_pool.clone()),
        Err(WorkerRuntimeError::WrongPool)
    );
    wrong_pool = wrong_pool.with_pool("codex-pool");
    assert!(
        !worker
            .handle_attempt_command(wrong_pool.clone())
            .unwrap()
            .duplicate
    );
    let mut lease_conflict = wrong_pool.clone();
    lease_conflict.lease_id = Uuid::new_v4();
    assert_eq!(
        worker.handle_attempt_command(lease_conflict),
        Err(WorkerRuntimeError::LeaseConflict)
    );

    let recoverable = AttemptCommandEnvelope::new(
        "tenant-1".to_owned(),
        session_id,
        task_id,
        Uuid::new_v4(),
        None,
        Uuid::new_v4(),
        1,
        "recover-fence".to_owned(),
        AttemptCommandKind::Execute,
    )
    .with_worker("worker-1", "codex", 1);
    worker.handle_attempt_command(recoverable.clone()).unwrap();
    worker.rollback_attempt_command(&recoverable);
    assert!(
        !worker
            .handle_attempt_command(recoverable.clone())
            .unwrap()
            .duplicate
    );
    let mut recoverable_idempotent = recoverable;
    recoverable_idempotent.command_id = Uuid::new_v4();
    recoverable_idempotent.idempotency_key = Some("recoverable".to_owned());
    worker
        .handle_attempt_command(recoverable_idempotent.clone())
        .unwrap();
    worker.rollback_attempt_command(&recoverable_idempotent);
    assert!(
        !worker
            .handle_attempt_command(recoverable_idempotent)
            .unwrap()
            .duplicate
    );

    let mut drain =
        WorkerControlEnvelope::heartbeat("worker-1".to_owned(), "codex".to_owned(), Uuid::new_v4());
    drain.kind = WorkerControlKind::Drain;
    worker.handle_control(drain).unwrap();
    let draining = AttemptCommandEnvelope::new(
        "tenant-1".to_owned(),
        session_id,
        task_id,
        Uuid::new_v4(),
        None,
        Uuid::new_v4(),
        1,
        "drain-fence".to_owned(),
        AttemptCommandKind::Start,
    )
    .with_worker("worker-1", "codex", 1);
    assert_eq!(
        worker.handle_attempt_command(draining),
        Err(WorkerRuntimeError::Draining)
    );

    let mut shutdown =
        WorkerControlEnvelope::heartbeat("worker-1".to_owned(), "codex".to_owned(), Uuid::new_v4());
    shutdown.kind = WorkerControlKind::Shutdown;
    worker.handle_control(shutdown).unwrap();
    assert!(!worker.health().ready);
}
