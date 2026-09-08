-- OmniSolo harness middleware durable state for MySQL 8.0.13+.
--
-- MySQL does not provide PostgreSQL RLS or partial indexes. Composite
-- tenant-aware foreign keys enforce the same-tenant reference boundary, and
-- nullable unique keys provide the partial-unique behavior used by events.

CREATE TABLE IF NOT EXISTS harness_sessions (
    session_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    project_id VARCHAR(191),
    workspace_id VARCHAR(191),
    title TEXT,
    labels JSON NOT NULL DEFAULT ('[]'),
    tags JSON NOT NULL DEFAULT ('{}'),
    state VARCHAR(191) NOT NULL DEFAULT 'open',
    state_version BIGINT NOT NULL DEFAULT 0,
    parent_session_id VARCHAR(191),
    root_session_id VARCHAR(191) NOT NULL,
    fork_source_event_id VARCHAR(191),
    active_task_id VARCHAR(191),
    retention_class VARCHAR(191),
    data_classification VARCHAR(191),
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    updated_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    last_active_at DATETIME(6),
    extensions JSON NOT NULL DEFAULT ('{}'),
    PRIMARY KEY (session_id),
    UNIQUE KEY uq_harness_sessions_tenant_session (tenant_id, session_id),
    KEY harness_sessions_tenant_idx (tenant_id, updated_at)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_tasks (
    task_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    parent_task_id VARCHAR(191),
    parent_attempt_id VARCHAR(191),
    kind VARCHAR(191) NOT NULL,
    objective LONGTEXT NOT NULL,
    state VARCHAR(191) NOT NULL DEFAULT 'queued',
    state_version BIGINT NOT NULL DEFAULT 0,
    dependency_task_ids JSON NOT NULL DEFAULT ('[]'),
    owner_actor JSON,
    input_message_ids JSON NOT NULL DEFAULT ('[]'),
    output_message_ids JSON NOT NULL DEFAULT ('[]'),
    artifact_ids JSON NOT NULL DEFAULT ('[]'),
    terminal_result JSON,
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    updated_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    started_at DATETIME(6),
    finished_at DATETIME(6),
    extensions JSON NOT NULL DEFAULT ('{}'),
    PRIMARY KEY (task_id),
    UNIQUE KEY uq_harness_tasks_tenant_task (tenant_id, task_id),
    KEY harness_tasks_session_idx (tenant_id, session_id, state, updated_at),
    CONSTRAINT fk_harness_tasks_session
        FOREIGN KEY (tenant_id, session_id)
        REFERENCES harness_sessions (tenant_id, session_id)
        ON DELETE CASCADE
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_turns (
    turn_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191) NOT NULL,
    sequence BIGINT NOT NULL,
    state VARCHAR(191) NOT NULL DEFAULT 'queued',
    state_version BIGINT NOT NULL DEFAULT 0,
    input_message_id VARCHAR(191),
    output_message_ids JSON NOT NULL DEFAULT ('[]'),
    stop_reason VARCHAR(191),
    error_code VARCHAR(191),
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    updated_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    completed_at DATETIME(6),
    extensions JSON NOT NULL DEFAULT ('{}'),
    PRIMARY KEY (turn_id),
    UNIQUE KEY uq_harness_turns_tenant_turn (tenant_id, turn_id),
    UNIQUE KEY uq_harness_turns_task_sequence (task_id, sequence),
    KEY harness_turns_session_idx (tenant_id, session_id, sequence),
    CONSTRAINT fk_harness_turns_session
        FOREIGN KEY (tenant_id, session_id)
        REFERENCES harness_sessions (tenant_id, session_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_harness_turns_task
        FOREIGN KEY (tenant_id, task_id)
        REFERENCES harness_tasks (tenant_id, task_id)
        ON DELETE CASCADE
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_runtime_configs (
    snapshot_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    snapshot_digest VARCHAR(191) NOT NULL,
    config JSON NOT NULL,
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (snapshot_id),
    UNIQUE KEY uq_harness_runtime_configs_tenant_snapshot (tenant_id, snapshot_id),
    UNIQUE KEY uq_harness_runtime_configs_digest (snapshot_digest)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_model_runtimes (
    runtime_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    model_descriptor JSON NOT NULL,
    runtime_kind VARCHAR(191) NOT NULL,
    serving_engine VARCHAR(191),
    api_dialect VARCHAR(191),
    api_version VARCHAR(191),
    endpoint TEXT,
    capability_discovery_endpoint TEXT,
    credential_ref VARCHAR(191),
    image TEXT,
    supported_revisions JSON NOT NULL DEFAULT ('[]'),
    adapter_refs JSON NOT NULL DEFAULT ('[]'),
    resource_profile JSON NOT NULL DEFAULT ('{}'),
    placement JSON NOT NULL DEFAULT ('{}'),
    capacity JSON NOT NULL DEFAULT ('{}'),
    autoscaling JSON NOT NULL DEFAULT ('{}'),
    readiness VARCHAR(191) NOT NULL DEFAULT 'unknown',
    health_endpoint TEXT,
    draining BOOLEAN NOT NULL DEFAULT FALSE,
    metadata JSON NOT NULL DEFAULT ('{}'),
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    updated_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (runtime_id),
    UNIQUE KEY uq_harness_model_runtimes_tenant_runtime (tenant_id, runtime_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_model_bindings (
    model_binding_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    attempt_id VARCHAR(191),
    runtime_id VARCHAR(191) NOT NULL,
    model_descriptor JSON NOT NULL,
    realized_capabilities JSON NOT NULL DEFAULT ('[]'),
    serving_revision VARCHAR(191),
    routing_reason TEXT,
    usage_accounting_source VARCHAR(191),
    status VARCHAR(191) NOT NULL DEFAULT 'selected',
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    released_at DATETIME(6),
    extensions JSON NOT NULL DEFAULT ('{}'),
    PRIMARY KEY (model_binding_id),
    UNIQUE KEY uq_harness_model_bindings_tenant_binding (tenant_id, model_binding_id),
    KEY harness_model_bindings_runtime_idx (tenant_id, runtime_id),
    CONSTRAINT fk_harness_model_bindings_runtime
        FOREIGN KEY (tenant_id, runtime_id)
        REFERENCES harness_model_runtimes (tenant_id, runtime_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_attempts (
    attempt_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191) NOT NULL,
    turn_id VARCHAR(191),
    parent_attempt_id VARCHAR(191),
    kind VARCHAR(191) NOT NULL,
    state VARCHAR(191) NOT NULL DEFAULT 'pending',
    state_version BIGINT NOT NULL DEFAULT 0,
    harness_id VARCHAR(191) NOT NULL,
    model_runtime_id VARCHAR(191),
    runtime_config_snapshot_id VARCHAR(191),
    model_binding_id VARCHAR(191),
    worker_id VARCHAR(191),
    started_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    finished_at DATETIME(6),
    metadata JSON NOT NULL DEFAULT ('{}'),
    extensions JSON NOT NULL DEFAULT ('{}'),
    PRIMARY KEY (attempt_id),
    UNIQUE KEY uq_harness_attempts_tenant_attempt (tenant_id, attempt_id),
    KEY harness_attempts_task_idx (tenant_id, task_id, state, started_at),
    CONSTRAINT fk_harness_attempts_session
        FOREIGN KEY (tenant_id, session_id)
        REFERENCES harness_sessions (tenant_id, session_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_harness_attempts_task
        FOREIGN KEY (tenant_id, task_id)
        REFERENCES harness_tasks (tenant_id, task_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_harness_attempts_turn
        FOREIGN KEY (tenant_id, turn_id)
        REFERENCES harness_turns (tenant_id, turn_id),
    CONSTRAINT fk_harness_attempts_runtime
        FOREIGN KEY (tenant_id, model_runtime_id)
        REFERENCES harness_model_runtimes (tenant_id, runtime_id),
    CONSTRAINT fk_harness_attempts_config
        FOREIGN KEY (tenant_id, runtime_config_snapshot_id)
        REFERENCES harness_runtime_configs (tenant_id, snapshot_id),
    CONSTRAINT fk_harness_attempts_binding
        FOREIGN KEY (tenant_id, model_binding_id)
        REFERENCES harness_model_bindings (tenant_id, model_binding_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_bindings (
    binding_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191),
    harness_id VARCHAR(191) NOT NULL,
    scope VARCHAR(191) NOT NULL,
    owner_id VARCHAR(191) NOT NULL,
    workspace_mutation_scope_id VARCHAR(191) NOT NULL,
    access_mode VARCHAR(191) NOT NULL,
    native_session_id VARCHAR(191),
    state VARCHAR(191) NOT NULL DEFAULT 'creating',
    generation BIGINT NOT NULL DEFAULT 1,
    capability_snapshot_id VARCHAR(191),
    adapter_config_digest VARCHAR(191),
    last_imported_native_cursor VARCHAR(191),
    last_exported_durable_sequence BIGINT,
    native_checkpoint_ref VARCHAR(191),
    worker_pool VARCHAR(191),
    state_locality VARCHAR(191),
    exact_resume_eligible BOOLEAN NOT NULL DEFAULT FALSE,
    invalidation_reason TEXT,
    native_record_digest VARCHAR(191),
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    last_used_at DATETIME(6),
    extensions JSON NOT NULL DEFAULT ('{}'),
    PRIMARY KEY (binding_id),
    UNIQUE KEY uq_harness_bindings_tenant_binding (tenant_id, binding_id),
    -- MySQL has no partial indexes; this still guarantees one active
    -- read-write binding per session/workspace mutation scope.
    UNIQUE KEY uq_harness_bindings_active_writable
        (session_id, workspace_mutation_scope_id, state, access_mode),
    KEY harness_bindings_session_idx (tenant_id, session_id, state),
    CONSTRAINT ck_harness_bindings_scope CHECK (scope IN ('session', 'task')),
    CONSTRAINT ck_harness_bindings_access_mode CHECK (access_mode IN ('read_only', 'read_write')),
    CONSTRAINT ck_harness_bindings_generation CHECK (generation > 0),
    CONSTRAINT fk_harness_bindings_session
        FOREIGN KEY (tenant_id, session_id)
        REFERENCES harness_sessions (tenant_id, session_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_harness_bindings_task
        FOREIGN KEY (tenant_id, task_id)
        REFERENCES harness_tasks (tenant_id, task_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_leases (
    lease_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    attempt_id VARCHAR(191) NOT NULL,
    worker_id VARCHAR(191),
    generation BIGINT NOT NULL,
    fencing_token VARCHAR(191) NOT NULL,
    state VARCHAR(191) NOT NULL DEFAULT 'active',
    issued_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    expires_at DATETIME(6),
    released_at DATETIME(6),
    PRIMARY KEY (lease_id),
    UNIQUE KEY uq_harness_leases_tenant_lease_generation (tenant_id, lease_id, generation),
    UNIQUE KEY uq_harness_leases_attempt_generation (attempt_id, generation),
    KEY harness_leases_active_idx (tenant_id, attempt_id, state),
    CONSTRAINT ck_harness_leases_generation CHECK (generation > 0),
    CONSTRAINT fk_harness_leases_attempt
        FOREIGN KEY (tenant_id, attempt_id)
        REFERENCES harness_attempts (tenant_id, attempt_id)
        ON DELETE CASCADE
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_events (
    event_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191),
    turn_id VARCHAR(191),
    source_attempt_id VARCHAR(191),
    ingest_attempt_id VARCHAR(191),
    actor_id VARCHAR(191),
    worker_id VARCHAR(191),
    harness_id VARCHAR(191),
    binding_id VARCHAR(191),
    binding_generation BIGINT,
    lease_id VARCHAR(191),
    lease_generation BIGINT,
    fencing_token VARCHAR(191),
    durable_sequence BIGINT,
    delivery_stream_id VARCHAR(191),
    delivery_sequence BIGINT,
    aggregate_id VARCHAR(191),
    aggregate_sequence BIGINT,
    branch_id VARCHAR(191),
    event_type VARCHAR(191) NOT NULL,
    payload_schema VARCHAR(191) NOT NULL,
    payload_version INT NOT NULL,
    occurred_at DATETIME(6) NOT NULL,
    ingested_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    correlation_id VARCHAR(191),
    causation_id VARCHAR(191),
    idempotency_key VARCHAR(191),
    durability VARCHAR(191) NOT NULL,
    replay_requirement VARCHAR(191) NOT NULL,
    visibility VARCHAR(191),
    data_classification VARCHAR(191),
    native_provenance JSON NOT NULL DEFAULT ('{}'),
    payload JSON NOT NULL,
    extensions JSON NOT NULL DEFAULT ('{}'),
    PRIMARY KEY (event_id),
    UNIQUE KEY uq_harness_events_tenant_event (tenant_id, event_id),
    UNIQUE KEY uq_harness_events_session_event (session_id, event_id),
    UNIQUE KEY uq_harness_events_durable_sequence (tenant_id, session_id, durable_sequence),
    UNIQUE KEY uq_harness_events_delivery_sequence (tenant_id, delivery_stream_id, delivery_sequence),
    UNIQUE KEY uq_harness_events_idempotency (session_id, idempotency_key),
    KEY harness_events_session_durable_idx (tenant_id, session_id, durable_sequence),
    KEY harness_events_delivery_idx (tenant_id, delivery_stream_id, delivery_sequence),
    CONSTRAINT ck_harness_events_durability CHECK (durability IN ('durable', 'transient')),
    CONSTRAINT ck_harness_events_replay CHECK (replay_requirement IN ('required', 'ignorable')),
    CONSTRAINT fk_harness_events_session
        FOREIGN KEY (tenant_id, session_id)
        REFERENCES harness_sessions (tenant_id, session_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_harness_events_task
        FOREIGN KEY (tenant_id, task_id)
        REFERENCES harness_tasks (tenant_id, task_id),
    CONSTRAINT fk_harness_events_turn
        FOREIGN KEY (tenant_id, turn_id)
        REFERENCES harness_turns (tenant_id, turn_id),
    CONSTRAINT fk_harness_events_source_attempt
        FOREIGN KEY (tenant_id, source_attempt_id)
        REFERENCES harness_attempts (tenant_id, attempt_id),
    CONSTRAINT fk_harness_events_ingest_attempt
        FOREIGN KEY (tenant_id, ingest_attempt_id)
        REFERENCES harness_attempts (tenant_id, attempt_id),
    CONSTRAINT fk_harness_events_binding
        FOREIGN KEY (tenant_id, binding_id)
        REFERENCES harness_bindings (tenant_id, binding_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_event_parents (
    session_id VARCHAR(191) NOT NULL,
    event_id VARCHAR(191) NOT NULL,
    parent_event_id VARCHAR(191) NOT NULL,
    PRIMARY KEY (session_id, event_id, parent_event_id),
    CONSTRAINT ck_harness_event_parents_not_self CHECK (event_id <> parent_event_id),
    CONSTRAINT fk_harness_event_parents_event
        FOREIGN KEY (session_id, event_id)
        REFERENCES harness_events (session_id, event_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_harness_event_parents_parent
        FOREIGN KEY (session_id, parent_event_id)
        REFERENCES harness_events (session_id, event_id)
        ON DELETE RESTRICT
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_event_branch_heads (
    session_id VARCHAR(191) NOT NULL,
    branch_id VARCHAR(191) NOT NULL,
    head_event_id VARCHAR(191) NOT NULL,
    expected_head_event_id VARCHAR(191),
    updated_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (session_id, branch_id),
    CONSTRAINT fk_harness_event_branch_heads_session
        FOREIGN KEY (session_id)
        REFERENCES harness_sessions (session_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_harness_event_branch_heads_head
        FOREIGN KEY (session_id, head_event_id)
        REFERENCES harness_events (session_id, event_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_artifacts (
    artifact_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    kind VARCHAR(191) NOT NULL,
    name TEXT NOT NULL,
    media_type VARCHAR(191),
    byte_length BIGINT NOT NULL,
    sha256 VARCHAR(191) NOT NULL,
    authorized_storage_ref TEXT,
    producer_event_id VARCHAR(191),
    retention_class VARCHAR(191),
    provenance JSON NOT NULL DEFAULT ('{}'),
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (artifact_id),
    UNIQUE KEY uq_harness_artifacts_tenant_artifact (tenant_id, artifact_id),
    UNIQUE KEY uq_harness_artifacts_tenant_sha256 (tenant_id, sha256),
    CONSTRAINT ck_harness_artifacts_byte_length CHECK (byte_length >= 0),
    CONSTRAINT fk_harness_artifacts_session
        FOREIGN KEY (tenant_id, session_id)
        REFERENCES harness_sessions (tenant_id, session_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_harness_artifacts_producer_event
        FOREIGN KEY (tenant_id, producer_event_id)
        REFERENCES harness_events (tenant_id, event_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_workspace_snapshots (
    snapshot_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    parent_snapshot_id VARCHAR(191),
    durable_sequence BIGINT,
    tree_digest VARCHAR(191) NOT NULL,
    archive_artifact_id VARCHAR(191),
    git_identity JSON NOT NULL DEFAULT ('{}'),
    patch_manifest JSON NOT NULL DEFAULT ('{}'),
    completeness VARCHAR(191) NOT NULL DEFAULT 'complete',
    creator_attempt_id VARCHAR(191),
    reason TEXT,
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (snapshot_id),
    UNIQUE KEY uq_harness_workspace_snapshots_tenant_snapshot (tenant_id, snapshot_id),
    UNIQUE KEY uq_harness_workspace_snapshots_tenant_tree (tenant_id, tree_digest),
    CONSTRAINT fk_harness_workspace_snapshots_session
        FOREIGN KEY (tenant_id, session_id)
        REFERENCES harness_sessions (tenant_id, session_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_harness_workspace_snapshots_archive
        FOREIGN KEY (tenant_id, archive_artifact_id)
        REFERENCES harness_artifacts (tenant_id, artifact_id),
    CONSTRAINT fk_harness_workspace_snapshots_creator
        FOREIGN KEY (tenant_id, creator_attempt_id)
        REFERENCES harness_attempts (tenant_id, attempt_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_native_records (
    native_record_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    attempt_id VARCHAR(191),
    harness_id VARCHAR(191) NOT NULL,
    adapter_version VARCHAR(191) NOT NULL,
    native_schema VARCHAR(191) NOT NULL,
    record_kind VARCHAR(191) NOT NULL,
    native_identity VARCHAR(191),
    native_cursor VARCHAR(191),
    ordinal BIGINT,
    payload_digest VARCHAR(191) NOT NULL,
    object_storage_ref TEXT,
    captured_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    extensions JSON NOT NULL DEFAULT ('{}'),
    PRIMARY KEY (native_record_id),
    UNIQUE KEY uq_harness_native_records_tenant_record (tenant_id, native_record_id),
    CONSTRAINT fk_harness_native_records_session
        FOREIGN KEY (tenant_id, session_id)
        REFERENCES harness_sessions (tenant_id, session_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_harness_native_records_attempt
        FOREIGN KEY (tenant_id, attempt_id)
        REFERENCES harness_attempts (tenant_id, attempt_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_command_inbox (
    command_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191),
    turn_id VARCHAR(191),
    attempt_id VARCHAR(191),
    lease_id VARCHAR(191),
    lease_generation BIGINT,
    fencing_token VARCHAR(191),
    idempotency_key VARCHAR(191) NOT NULL,
    command_type VARCHAR(191) NOT NULL,
    payload_schema VARCHAR(191) NOT NULL,
    payload JSON NOT NULL,
    status VARCHAR(191) NOT NULL DEFAULT 'admitted',
    outcome_event_id VARCHAR(191),
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    settled_at DATETIME(6),
    PRIMARY KEY (command_id),
    UNIQUE KEY uq_harness_command_inbox_tenant_command (tenant_id, command_id),
    UNIQUE KEY uq_harness_command_inbox_idempotency (session_id, idempotency_key),
    CONSTRAINT fk_harness_command_inbox_session
        FOREIGN KEY (tenant_id, session_id)
        REFERENCES harness_sessions (tenant_id, session_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_harness_command_inbox_task
        FOREIGN KEY (tenant_id, task_id)
        REFERENCES harness_tasks (tenant_id, task_id),
    CONSTRAINT fk_harness_command_inbox_turn
        FOREIGN KEY (tenant_id, turn_id)
        REFERENCES harness_turns (tenant_id, turn_id),
    CONSTRAINT fk_harness_command_inbox_attempt
        FOREIGN KEY (tenant_id, attempt_id)
        REFERENCES harness_attempts (tenant_id, attempt_id),
    CONSTRAINT fk_harness_command_inbox_outcome
        FOREIGN KEY (tenant_id, outcome_event_id)
        REFERENCES harness_events (tenant_id, event_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_command_outbox (
    outbox_id BIGINT NOT NULL AUTO_INCREMENT,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191),
    turn_id VARCHAR(191),
    attempt_id VARCHAR(191),
    command_id VARCHAR(191),
    event_id VARCHAR(191),
    idempotency_key VARCHAR(191) NOT NULL,
    payload JSON NOT NULL,
    published_at DATETIME(6),
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (outbox_id),
    UNIQUE KEY uq_harness_command_outbox_idempotency (session_id, idempotency_key),
    KEY harness_command_outbox_pending_idx (tenant_id, published_at, created_at),
    CONSTRAINT fk_harness_command_outbox_session
        FOREIGN KEY (tenant_id, session_id)
        REFERENCES harness_sessions (tenant_id, session_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_harness_command_outbox_task
        FOREIGN KEY (tenant_id, task_id)
        REFERENCES harness_tasks (tenant_id, task_id),
    CONSTRAINT fk_harness_command_outbox_turn
        FOREIGN KEY (tenant_id, turn_id)
        REFERENCES harness_turns (tenant_id, turn_id),
    CONSTRAINT fk_harness_command_outbox_attempt
        FOREIGN KEY (tenant_id, attempt_id)
        REFERENCES harness_attempts (tenant_id, attempt_id),
    CONSTRAINT fk_harness_command_outbox_command
        FOREIGN KEY (tenant_id, command_id)
        REFERENCES harness_command_inbox (tenant_id, command_id),
    CONSTRAINT fk_harness_command_outbox_event
        FOREIGN KEY (tenant_id, event_id)
        REFERENCES harness_events (tenant_id, event_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_model_runtime_workers (
    worker_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    runtime_id VARCHAR(191) NOT NULL,
    capability_version BIGINT NOT NULL DEFAULT 0,
    capabilities JSON NOT NULL DEFAULT ('{}'),
    capacity JSON NOT NULL DEFAULT ('{}'),
    state VARCHAR(191) NOT NULL DEFAULT 'ready',
    last_heartbeat_at DATETIME(6),
    extensions JSON NOT NULL DEFAULT ('{}'),
    PRIMARY KEY (worker_id),
    UNIQUE KEY uq_harness_runtime_workers_tenant_worker (tenant_id, worker_id),
    CONSTRAINT fk_harness_runtime_workers_runtime
        FOREIGN KEY (tenant_id, runtime_id)
        REFERENCES harness_model_runtimes (tenant_id, runtime_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_inference_admissions (
    admission_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    request_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191),
    turn_id VARCHAR(191),
    attempt_id VARCHAR(191),
    model_binding_id VARCHAR(191) NOT NULL,
    runtime_id VARCHAR(191) NOT NULL,
    worker_id VARCHAR(191) NOT NULL,
    capacity_lease_id VARCHAR(191) NOT NULL,
    capacity_generation BIGINT NOT NULL,
    fencing_token VARCHAR(191) NOT NULL,
    request_digest VARCHAR(191) NOT NULL,
    state VARCHAR(191) NOT NULL DEFAULT 'queued',
    state_version BIGINT NOT NULL DEFAULT 0,
    final_response JSON,
    -- Keep the canonical logical name; usage is reserved by MySQL.
    `usage` JSON,
    uncertain_reason TEXT,
    admitted_at DATETIME(6),
    completed_at DATETIME(6),
    PRIMARY KEY (admission_id),
    UNIQUE KEY uq_harness_inference_admissions_request (tenant_id, request_id),
    CONSTRAINT fk_harness_inference_admissions_session
        FOREIGN KEY (tenant_id, session_id)
        REFERENCES harness_sessions (tenant_id, session_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_harness_inference_admissions_task
        FOREIGN KEY (tenant_id, task_id)
        REFERENCES harness_tasks (tenant_id, task_id),
    CONSTRAINT fk_harness_inference_admissions_turn
        FOREIGN KEY (tenant_id, turn_id)
        REFERENCES harness_turns (tenant_id, turn_id),
    CONSTRAINT fk_harness_inference_admissions_attempt
        FOREIGN KEY (tenant_id, attempt_id)
        REFERENCES harness_attempts (tenant_id, attempt_id),
    CONSTRAINT fk_harness_inference_admissions_binding
        FOREIGN KEY (tenant_id, model_binding_id)
        REFERENCES harness_model_bindings (tenant_id, model_binding_id),
    CONSTRAINT fk_harness_inference_admissions_runtime
        FOREIGN KEY (tenant_id, runtime_id)
        REFERENCES harness_model_runtimes (tenant_id, runtime_id),
    CONSTRAINT fk_harness_inference_admissions_worker
        FOREIGN KEY (tenant_id, worker_id)
        REFERENCES harness_model_runtime_workers (tenant_id, worker_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_handoff_operations (
    operation_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191),
    scope VARCHAR(191) NOT NULL,
    source_binding_id VARCHAR(191) NOT NULL,
    target_binding_id VARCHAR(191),
    target_harness_id VARCHAR(191) NOT NULL,
    state VARCHAR(191) NOT NULL DEFAULT 'requested',
    state_version BIGINT NOT NULL DEFAULT 0,
    capsule_id VARCHAR(191),
    loss_report_digest VARCHAR(191),
    loss_ack_digest VARCHAR(191),
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    completed_at DATETIME(6),
    PRIMARY KEY (operation_id),
    UNIQUE KEY uq_harness_handoff_operations_tenant_operation (tenant_id, operation_id),
    CONSTRAINT ck_harness_handoff_operations_scope CHECK (scope IN ('session', 'task')),
    CONSTRAINT fk_harness_handoff_operations_session
        FOREIGN KEY (tenant_id, session_id)
        REFERENCES harness_sessions (tenant_id, session_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_harness_handoff_operations_task
        FOREIGN KEY (tenant_id, task_id)
        REFERENCES harness_tasks (tenant_id, task_id),
    CONSTRAINT fk_harness_handoff_operations_source
        FOREIGN KEY (tenant_id, source_binding_id)
        REFERENCES harness_bindings (tenant_id, binding_id),
    CONSTRAINT fk_harness_handoff_operations_target
        FOREIGN KEY (tenant_id, target_binding_id)
        REFERENCES harness_bindings (tenant_id, binding_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_capsules (
    capsule_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    handoff_id VARCHAR(191),
    schema_version INT NOT NULL,
    minimum_reader_version INT NOT NULL,
    producer VARCHAR(191) NOT NULL,
    compiler_version INT NOT NULL,
    source_binding_id VARCHAR(191),
    target_harness_id VARCHAR(191) NOT NULL,
    from_durable_sequence BIGINT NOT NULL,
    to_durable_sequence BIGINT NOT NULL,
    branch_id VARCHAR(191),
    head_event_id VARCHAR(191),
    manifest JSON NOT NULL,
    manifest_digest VARCHAR(191) NOT NULL,
    record_digest VARCHAR(191) NOT NULL,
    loss_report_digest VARCHAR(191) NOT NULL,
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (capsule_id),
    UNIQUE KEY uq_harness_capsules_tenant_capsule (tenant_id, capsule_id),
    UNIQUE KEY uq_harness_capsules_manifest_digest (tenant_id, manifest_digest),
    CONSTRAINT fk_harness_capsules_session
        FOREIGN KEY (tenant_id, session_id)
        REFERENCES harness_sessions (tenant_id, session_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_harness_capsules_handoff
        FOREIGN KEY (tenant_id, handoff_id)
        REFERENCES harness_handoff_operations (tenant_id, operation_id),
    CONSTRAINT fk_harness_capsules_source_binding
        FOREIGN KEY (tenant_id, source_binding_id)
        REFERENCES harness_bindings (tenant_id, binding_id),
    CONSTRAINT fk_harness_capsules_head_event
        FOREIGN KEY (tenant_id, head_event_id)
        REFERENCES harness_events (tenant_id, event_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_capsule_loss_entries (
    capsule_id VARCHAR(191) NOT NULL,
    ordinal INT NOT NULL,
    source_path TEXT NOT NULL,
    reason TEXT NOT NULL,
    severity VARCHAR(191) NOT NULL,
    target_representation TEXT,
    acknowledgement_required BOOLEAN NOT NULL DEFAULT FALSE,
    capability VARCHAR(191),
    PRIMARY KEY (capsule_id, ordinal),
    CONSTRAINT ck_harness_capsule_loss_entries_severity
        CHECK (severity IN ('info', 'warning', 'required', 'unsafe')),
    CONSTRAINT fk_harness_capsule_loss_entries_capsule
        FOREIGN KEY (capsule_id)
        REFERENCES harness_capsules (capsule_id)
        ON DELETE CASCADE
) ENGINE=InnoDB;
