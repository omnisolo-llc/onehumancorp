-- OmniSolo harness middleware durable state.
--
-- Native harness state remains adapter-owned. These tables hold the canonical
-- session/task/event projection, transfer manifests, and worker fences.

CREATE TABLE IF NOT EXISTS harness_sessions (
    session_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    project_id TEXT,
    workspace_id TEXT,
    title TEXT,
    labels JSONB NOT NULL DEFAULT '[]'::jsonb,
    tags JSONB NOT NULL DEFAULT '{}'::jsonb,
    state TEXT NOT NULL DEFAULT 'open',
    state_version BIGINT NOT NULL DEFAULT 0,
    parent_session_id TEXT,
    root_session_id TEXT NOT NULL,
    fork_source_event_id TEXT,
    active_task_id TEXT,
    retention_class TEXT,
    data_classification TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_active_at TIMESTAMPTZ,
    extensions JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE INDEX IF NOT EXISTS harness_sessions_tenant_idx
    ON harness_sessions (tenant_id, updated_at DESC);

CREATE TABLE IF NOT EXISTS harness_tasks (
    task_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL REFERENCES harness_sessions(session_id) ON DELETE CASCADE,
    parent_task_id TEXT,
    parent_attempt_id TEXT,
    kind TEXT NOT NULL,
    objective TEXT NOT NULL,
    state TEXT NOT NULL DEFAULT 'queued',
    state_version BIGINT NOT NULL DEFAULT 0,
    dependency_task_ids JSONB NOT NULL DEFAULT '[]'::jsonb,
    owner_actor JSONB,
    input_message_ids JSONB NOT NULL DEFAULT '[]'::jsonb,
    output_message_ids JSONB NOT NULL DEFAULT '[]'::jsonb,
    artifact_ids JSONB NOT NULL DEFAULT '[]'::jsonb,
    terminal_result JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    started_at TIMESTAMPTZ,
    finished_at TIMESTAMPTZ,
    extensions JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE INDEX IF NOT EXISTS harness_tasks_session_idx
    ON harness_tasks (tenant_id, session_id, state, updated_at DESC);

CREATE TABLE IF NOT EXISTS harness_turns (
    turn_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL REFERENCES harness_sessions(session_id) ON DELETE CASCADE,
    task_id TEXT NOT NULL REFERENCES harness_tasks(task_id) ON DELETE CASCADE,
    sequence BIGINT NOT NULL,
    state TEXT NOT NULL DEFAULT 'queued',
    state_version BIGINT NOT NULL DEFAULT 0,
    input_message_id TEXT,
    output_message_ids JSONB NOT NULL DEFAULT '[]'::jsonb,
    stop_reason TEXT,
    error_code TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMPTZ,
    extensions JSONB NOT NULL DEFAULT '{}'::jsonb,
    UNIQUE (task_id, sequence)
);

CREATE TABLE IF NOT EXISTS harness_runtime_configs (
    snapshot_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    snapshot_digest TEXT NOT NULL UNIQUE,
    config JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS harness_model_runtimes (
    runtime_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    model_descriptor JSONB NOT NULL,
    runtime_kind TEXT NOT NULL,
    serving_engine TEXT,
    api_dialect TEXT,
    api_version TEXT,
    endpoint TEXT,
    capability_discovery_endpoint TEXT,
    credential_ref TEXT,
    image TEXT,
    supported_revisions JSONB NOT NULL DEFAULT '[]'::jsonb,
    adapter_refs JSONB NOT NULL DEFAULT '[]'::jsonb,
    resource_profile JSONB NOT NULL DEFAULT '{}'::jsonb,
    placement JSONB NOT NULL DEFAULT '{}'::jsonb,
    capacity JSONB NOT NULL DEFAULT '{}'::jsonb,
    autoscaling JSONB NOT NULL DEFAULT '{}'::jsonb,
    readiness TEXT NOT NULL DEFAULT 'unknown',
    health_endpoint TEXT,
    draining BOOLEAN NOT NULL DEFAULT FALSE,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS harness_model_bindings (
    model_binding_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    attempt_id TEXT,
    runtime_id TEXT NOT NULL REFERENCES harness_model_runtimes(runtime_id),
    model_descriptor JSONB NOT NULL,
    realized_capabilities JSONB NOT NULL DEFAULT '[]'::jsonb,
    serving_revision TEXT,
    routing_reason TEXT,
    usage_accounting_source TEXT,
    status TEXT NOT NULL DEFAULT 'selected',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    released_at TIMESTAMPTZ,
    extensions JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE TABLE IF NOT EXISTS harness_attempts (
    attempt_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL REFERENCES harness_sessions(session_id) ON DELETE CASCADE,
    task_id TEXT NOT NULL REFERENCES harness_tasks(task_id) ON DELETE CASCADE,
    turn_id TEXT REFERENCES harness_turns(turn_id),
    parent_attempt_id TEXT,
    kind TEXT NOT NULL,
    state TEXT NOT NULL DEFAULT 'pending',
    state_version BIGINT NOT NULL DEFAULT 0,
    harness_id TEXT NOT NULL,
    model_runtime_id TEXT REFERENCES harness_model_runtimes(runtime_id),
    runtime_config_snapshot_id TEXT REFERENCES harness_runtime_configs(snapshot_id),
    model_binding_id TEXT REFERENCES harness_model_bindings(model_binding_id),
    worker_id TEXT,
    started_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    finished_at TIMESTAMPTZ,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    extensions JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE INDEX IF NOT EXISTS harness_attempts_task_idx
    ON harness_attempts (tenant_id, task_id, state, started_at DESC);

CREATE TABLE IF NOT EXISTS harness_bindings (
    binding_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL REFERENCES harness_sessions(session_id) ON DELETE CASCADE,
    task_id TEXT REFERENCES harness_tasks(task_id),
    harness_id TEXT NOT NULL,
    scope TEXT NOT NULL CHECK (scope IN ('session', 'task')),
    owner_id TEXT NOT NULL,
    workspace_mutation_scope_id TEXT NOT NULL,
    access_mode TEXT NOT NULL CHECK (access_mode IN ('read_only', 'read_write')),
    native_session_id TEXT,
    state TEXT NOT NULL DEFAULT 'creating',
    generation BIGINT NOT NULL DEFAULT 1 CHECK (generation > 0),
    capability_snapshot_id TEXT,
    adapter_config_digest TEXT,
    last_imported_native_cursor TEXT,
    last_exported_durable_sequence BIGINT,
    native_checkpoint_ref TEXT,
    worker_pool TEXT,
    state_locality TEXT,
    exact_resume_eligible BOOLEAN NOT NULL DEFAULT FALSE,
    invalidation_reason TEXT,
    native_record_digest TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_used_at TIMESTAMPTZ,
    extensions JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE UNIQUE INDEX IF NOT EXISTS harness_one_active_writable_binding
    ON harness_bindings (session_id, workspace_mutation_scope_id)
    WHERE state = 'active' AND access_mode = 'read_write';

CREATE TABLE IF NOT EXISTS harness_leases (
    lease_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    attempt_id TEXT NOT NULL REFERENCES harness_attempts(attempt_id) ON DELETE CASCADE,
    worker_id TEXT,
    generation BIGINT NOT NULL CHECK (generation > 0),
    fencing_token TEXT NOT NULL,
    state TEXT NOT NULL DEFAULT 'active',
    issued_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMPTZ,
    released_at TIMESTAMPTZ,
    UNIQUE (attempt_id, generation),
    UNIQUE (lease_id, generation)
);

CREATE TABLE IF NOT EXISTS harness_events (
    event_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL REFERENCES harness_sessions(session_id) ON DELETE CASCADE,
    task_id TEXT REFERENCES harness_tasks(task_id),
    turn_id TEXT REFERENCES harness_turns(turn_id),
    source_attempt_id TEXT REFERENCES harness_attempts(attempt_id),
    ingest_attempt_id TEXT REFERENCES harness_attempts(attempt_id),
    actor_id TEXT,
    worker_id TEXT,
    harness_id TEXT,
    binding_id TEXT REFERENCES harness_bindings(binding_id),
    binding_generation BIGINT,
    lease_id TEXT,
    lease_generation BIGINT,
    fencing_token TEXT,
    durable_sequence BIGINT,
    delivery_stream_id TEXT,
    delivery_sequence BIGINT,
    aggregate_id TEXT,
    aggregate_sequence BIGINT,
    branch_id TEXT,
    event_type TEXT NOT NULL,
    payload_schema TEXT NOT NULL,
    payload_version INTEGER NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL,
    ingested_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    correlation_id TEXT,
    causation_id TEXT,
    idempotency_key TEXT,
    durability TEXT NOT NULL CHECK (durability IN ('durable', 'transient')),
    replay_requirement TEXT NOT NULL CHECK (replay_requirement IN ('required', 'ignorable')),
    visibility TEXT,
    data_classification TEXT,
    native_provenance JSONB NOT NULL DEFAULT '{}'::jsonb,
    payload JSONB NOT NULL,
    extensions JSONB NOT NULL DEFAULT '{}'::jsonb,
    UNIQUE (session_id, event_id)
);

CREATE UNIQUE INDEX IF NOT EXISTS harness_durable_event_sequence_idx
    ON harness_events (tenant_id, session_id, durable_sequence)
    WHERE durable_sequence IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS harness_delivery_sequence_idx
    ON harness_events (tenant_id, delivery_stream_id, delivery_sequence)
    WHERE delivery_stream_id IS NOT NULL AND delivery_sequence IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS harness_event_idempotency_idx
    ON harness_events (session_id, idempotency_key)
    WHERE idempotency_key IS NOT NULL;

CREATE TABLE IF NOT EXISTS harness_event_parents (
    session_id TEXT NOT NULL,
    event_id TEXT NOT NULL,
    parent_event_id TEXT NOT NULL,
    PRIMARY KEY (session_id, event_id, parent_event_id),
    CHECK (event_id <> parent_event_id),
    FOREIGN KEY (session_id, event_id)
        REFERENCES harness_events(session_id, event_id) ON DELETE CASCADE,
    FOREIGN KEY (session_id, parent_event_id)
        REFERENCES harness_events(session_id, event_id) ON DELETE RESTRICT
);

CREATE TABLE IF NOT EXISTS harness_event_branch_heads (
    session_id TEXT NOT NULL REFERENCES harness_sessions(session_id) ON DELETE CASCADE,
    branch_id TEXT NOT NULL,
    head_event_id TEXT NOT NULL,
    expected_head_event_id TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (session_id, branch_id),
    FOREIGN KEY (session_id, head_event_id)
        REFERENCES harness_events(session_id, event_id)
);

CREATE TABLE IF NOT EXISTS harness_artifacts (
    artifact_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL REFERENCES harness_sessions(session_id) ON DELETE CASCADE,
    kind TEXT NOT NULL,
    name TEXT NOT NULL,
    media_type TEXT,
    byte_length BIGINT NOT NULL CHECK (byte_length >= 0),
    sha256 TEXT NOT NULL,
    authorized_storage_ref TEXT,
    producer_event_id TEXT REFERENCES harness_events(event_id),
    retention_class TEXT,
    provenance JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, sha256)
);

CREATE TABLE IF NOT EXISTS harness_workspace_snapshots (
    snapshot_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL REFERENCES harness_sessions(session_id) ON DELETE CASCADE,
    parent_snapshot_id TEXT,
    durable_sequence BIGINT,
    tree_digest TEXT NOT NULL,
    archive_artifact_id TEXT REFERENCES harness_artifacts(artifact_id),
    git_identity JSONB NOT NULL DEFAULT '{}'::jsonb,
    patch_manifest JSONB NOT NULL DEFAULT '{}'::jsonb,
    completeness TEXT NOT NULL DEFAULT 'complete',
    creator_attempt_id TEXT REFERENCES harness_attempts(attempt_id),
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, tree_digest)
);

CREATE TABLE IF NOT EXISTS harness_native_records (
    native_record_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL REFERENCES harness_sessions(session_id) ON DELETE CASCADE,
    attempt_id TEXT REFERENCES harness_attempts(attempt_id),
    harness_id TEXT NOT NULL,
    adapter_version TEXT NOT NULL,
    native_schema TEXT NOT NULL,
    record_kind TEXT NOT NULL,
    native_identity TEXT,
    native_cursor TEXT,
    ordinal BIGINT,
    payload_digest TEXT NOT NULL,
    object_storage_ref TEXT,
    captured_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    extensions JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE TABLE IF NOT EXISTS harness_command_inbox (
    command_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL REFERENCES harness_sessions(session_id) ON DELETE CASCADE,
    task_id TEXT REFERENCES harness_tasks(task_id),
    turn_id TEXT REFERENCES harness_turns(turn_id),
    attempt_id TEXT REFERENCES harness_attempts(attempt_id),
    lease_id TEXT,
    lease_generation BIGINT,
    fencing_token TEXT,
    idempotency_key TEXT NOT NULL,
    command_type TEXT NOT NULL,
    payload_schema TEXT NOT NULL,
    payload JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'admitted',
    outcome_event_id TEXT REFERENCES harness_events(event_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    settled_at TIMESTAMPTZ,
    UNIQUE (session_id, idempotency_key)
);

CREATE TABLE IF NOT EXISTS harness_command_outbox (
    outbox_id BIGSERIAL PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL REFERENCES harness_sessions(session_id) ON DELETE CASCADE,
    task_id TEXT REFERENCES harness_tasks(task_id),
    turn_id TEXT REFERENCES harness_turns(turn_id),
    attempt_id TEXT REFERENCES harness_attempts(attempt_id),
    command_id TEXT REFERENCES harness_command_inbox(command_id),
    event_id TEXT REFERENCES harness_events(event_id),
    idempotency_key TEXT NOT NULL,
    payload JSONB NOT NULL,
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (session_id, idempotency_key)
);

CREATE TABLE IF NOT EXISTS harness_model_runtime_workers (
    worker_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    runtime_id TEXT NOT NULL REFERENCES harness_model_runtimes(runtime_id),
    capability_version BIGINT NOT NULL DEFAULT 0,
    capabilities JSONB NOT NULL DEFAULT '{}'::jsonb,
    capacity JSONB NOT NULL DEFAULT '{}'::jsonb,
    state TEXT NOT NULL DEFAULT 'ready',
    last_heartbeat_at TIMESTAMPTZ,
    extensions JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE TABLE IF NOT EXISTS harness_inference_admissions (
    admission_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    request_id TEXT NOT NULL,
    session_id TEXT NOT NULL REFERENCES harness_sessions(session_id) ON DELETE CASCADE,
    task_id TEXT REFERENCES harness_tasks(task_id),
    turn_id TEXT REFERENCES harness_turns(turn_id),
    attempt_id TEXT REFERENCES harness_attempts(attempt_id),
    model_binding_id TEXT NOT NULL REFERENCES harness_model_bindings(model_binding_id),
    runtime_id TEXT NOT NULL REFERENCES harness_model_runtimes(runtime_id),
    worker_id TEXT NOT NULL REFERENCES harness_model_runtime_workers(worker_id),
    capacity_lease_id TEXT NOT NULL,
    capacity_generation BIGINT NOT NULL,
    fencing_token TEXT NOT NULL,
    request_digest TEXT NOT NULL,
    state TEXT NOT NULL DEFAULT 'queued',
    state_version BIGINT NOT NULL DEFAULT 0,
    final_response JSONB,
    usage JSONB,
    uncertain_reason TEXT,
    admitted_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    UNIQUE (tenant_id, request_id)
);

CREATE TABLE IF NOT EXISTS harness_handoff_operations (
    operation_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL REFERENCES harness_sessions(session_id) ON DELETE CASCADE,
    task_id TEXT REFERENCES harness_tasks(task_id),
    scope TEXT NOT NULL CHECK (scope IN ('session', 'task')),
    source_binding_id TEXT NOT NULL REFERENCES harness_bindings(binding_id),
    target_binding_id TEXT REFERENCES harness_bindings(binding_id),
    target_harness_id TEXT NOT NULL,
    state TEXT NOT NULL DEFAULT 'requested',
    state_version BIGINT NOT NULL DEFAULT 0,
    capsule_id TEXT,
    loss_report_digest TEXT,
    loss_ack_digest TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS harness_capsules (
    capsule_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL REFERENCES harness_sessions(session_id) ON DELETE CASCADE,
    handoff_id TEXT REFERENCES harness_handoff_operations(operation_id),
    schema_version INTEGER NOT NULL,
    minimum_reader_version INTEGER NOT NULL,
    producer TEXT NOT NULL,
    compiler_version INTEGER NOT NULL,
    source_binding_id TEXT,
    target_harness_id TEXT NOT NULL,
    from_durable_sequence BIGINT NOT NULL,
    to_durable_sequence BIGINT NOT NULL,
    branch_id TEXT,
    head_event_id TEXT,
    manifest JSONB NOT NULL,
    manifest_digest TEXT NOT NULL,
    record_digest TEXT NOT NULL,
    loss_report_digest TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, manifest_digest)
);

CREATE TABLE IF NOT EXISTS harness_capsule_loss_entries (
    capsule_id TEXT NOT NULL REFERENCES harness_capsules(capsule_id) ON DELETE CASCADE,
    ordinal INTEGER NOT NULL,
    source_path TEXT NOT NULL,
    reason TEXT NOT NULL,
    severity TEXT NOT NULL CHECK (severity IN ('info', 'warning', 'required', 'unsafe')),
    target_representation TEXT,
    acknowledgement_required BOOLEAN NOT NULL DEFAULT FALSE,
    capability TEXT,
    PRIMARY KEY (capsule_id, ordinal)
);

CREATE OR REPLACE FUNCTION harness_assert_same_tenant(
    p_tenant_id TEXT,
    p_reference_tenant_id TEXT,
    p_reference_name TEXT
) RETURNS VOID
LANGUAGE plpgsql
AS $$
BEGIN
    IF p_reference_tenant_id IS NULL OR p_reference_tenant_id <> p_tenant_id THEN
        RAISE EXCEPTION 'cross-tenant harness reference: %', p_reference_name;
    END IF;
END;
$$;

CREATE OR REPLACE FUNCTION harness_validate_tenant_references()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
DECLARE
    reference_tenant TEXT;
BEGIN
    IF TG_TABLE_NAME = 'harness_sessions' THEN
        IF NEW.parent_session_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_sessions
             WHERE session_id = NEW.parent_session_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_sessions.parent_session_id'
            );
        END IF;
    ELSIF TG_TABLE_NAME = 'harness_tasks' THEN
        SELECT tenant_id INTO reference_tenant
          FROM harness_sessions
         WHERE session_id = NEW.session_id;
        PERFORM harness_assert_same_tenant(
            NEW.tenant_id, reference_tenant, 'harness_tasks.session_id'
        );
    ELSIF TG_TABLE_NAME = 'harness_turns' THEN
        SELECT tenant_id INTO reference_tenant
          FROM harness_sessions
         WHERE session_id = NEW.session_id;
        PERFORM harness_assert_same_tenant(
            NEW.tenant_id, reference_tenant, 'harness_turns.session_id'
        );
        SELECT tenant_id INTO reference_tenant
          FROM harness_tasks
         WHERE task_id = NEW.task_id;
        PERFORM harness_assert_same_tenant(
            NEW.tenant_id, reference_tenant, 'harness_turns.task_id'
        );
    ELSIF TG_TABLE_NAME = 'harness_model_bindings' THEN
        SELECT tenant_id INTO reference_tenant
          FROM harness_model_runtimes
         WHERE runtime_id = NEW.runtime_id;
        PERFORM harness_assert_same_tenant(
            NEW.tenant_id, reference_tenant, 'harness_model_bindings.runtime_id'
        );
        IF NEW.attempt_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_attempts
             WHERE attempt_id = NEW.attempt_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_model_bindings.attempt_id'
            );
        END IF;
    ELSIF TG_TABLE_NAME = 'harness_attempts' THEN
        SELECT tenant_id INTO reference_tenant
          FROM harness_sessions
         WHERE session_id = NEW.session_id;
        PERFORM harness_assert_same_tenant(
            NEW.tenant_id, reference_tenant, 'harness_attempts.session_id'
        );
        SELECT tenant_id INTO reference_tenant
          FROM harness_tasks
         WHERE task_id = NEW.task_id;
        PERFORM harness_assert_same_tenant(
            NEW.tenant_id, reference_tenant, 'harness_attempts.task_id'
        );
        IF NEW.turn_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_turns
             WHERE turn_id = NEW.turn_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_attempts.turn_id'
            );
        END IF;
        IF NEW.parent_attempt_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_attempts
             WHERE attempt_id = NEW.parent_attempt_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_attempts.parent_attempt_id'
            );
        END IF;
        IF NEW.model_runtime_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_model_runtimes
             WHERE runtime_id = NEW.model_runtime_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_attempts.model_runtime_id'
            );
        END IF;
        IF NEW.runtime_config_snapshot_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_runtime_configs
             WHERE snapshot_id = NEW.runtime_config_snapshot_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_attempts.runtime_config_snapshot_id'
            );
        END IF;
        IF NEW.model_binding_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_model_bindings
             WHERE model_binding_id = NEW.model_binding_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_attempts.model_binding_id'
            );
        END IF;
    ELSIF TG_TABLE_NAME = 'harness_bindings' THEN
        SELECT tenant_id INTO reference_tenant
          FROM harness_sessions
         WHERE session_id = NEW.session_id;
        PERFORM harness_assert_same_tenant(
            NEW.tenant_id, reference_tenant, 'harness_bindings.session_id'
        );
        IF NEW.task_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_tasks
             WHERE task_id = NEW.task_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_bindings.task_id'
            );
        END IF;
    ELSIF TG_TABLE_NAME = 'harness_leases' THEN
        SELECT tenant_id INTO reference_tenant
          FROM harness_attempts
         WHERE attempt_id = NEW.attempt_id;
        PERFORM harness_assert_same_tenant(
            NEW.tenant_id, reference_tenant, 'harness_leases.attempt_id'
        );
    ELSIF TG_TABLE_NAME = 'harness_events' THEN
        SELECT tenant_id INTO reference_tenant
          FROM harness_sessions
         WHERE session_id = NEW.session_id;
        PERFORM harness_assert_same_tenant(
            NEW.tenant_id, reference_tenant, 'harness_events.session_id'
        );
        IF NEW.task_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_tasks
             WHERE task_id = NEW.task_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_events.task_id'
            );
        END IF;
        IF NEW.turn_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_turns
             WHERE turn_id = NEW.turn_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_events.turn_id'
            );
        END IF;
        IF NEW.source_attempt_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_attempts
             WHERE attempt_id = NEW.source_attempt_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_events.source_attempt_id'
            );
        END IF;
        IF NEW.ingest_attempt_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_attempts
             WHERE attempt_id = NEW.ingest_attempt_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_events.ingest_attempt_id'
            );
        END IF;
        IF NEW.binding_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_bindings
             WHERE binding_id = NEW.binding_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_events.binding_id'
            );
        END IF;
        IF NEW.lease_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_leases
             WHERE lease_id = NEW.lease_id
               AND (NEW.lease_generation IS NULL OR generation = NEW.lease_generation);
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_events.lease_id'
            );
        END IF;
    ELSIF TG_TABLE_NAME = 'harness_artifacts' THEN
        SELECT tenant_id INTO reference_tenant
          FROM harness_sessions
         WHERE session_id = NEW.session_id;
        PERFORM harness_assert_same_tenant(
            NEW.tenant_id, reference_tenant, 'harness_artifacts.session_id'
        );
        IF NEW.producer_event_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_events
             WHERE event_id = NEW.producer_event_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_artifacts.producer_event_id'
            );
        END IF;
    ELSIF TG_TABLE_NAME = 'harness_workspace_snapshots' THEN
        SELECT tenant_id INTO reference_tenant
          FROM harness_sessions
         WHERE session_id = NEW.session_id;
        PERFORM harness_assert_same_tenant(
            NEW.tenant_id, reference_tenant, 'harness_workspace_snapshots.session_id'
        );
        IF NEW.parent_snapshot_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_workspace_snapshots
             WHERE snapshot_id = NEW.parent_snapshot_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_workspace_snapshots.parent_snapshot_id'
            );
        END IF;
        IF NEW.archive_artifact_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_artifacts
             WHERE artifact_id = NEW.archive_artifact_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_workspace_snapshots.archive_artifact_id'
            );
        END IF;
        IF NEW.creator_attempt_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_attempts
             WHERE attempt_id = NEW.creator_attempt_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_workspace_snapshots.creator_attempt_id'
            );
        END IF;
    ELSIF TG_TABLE_NAME = 'harness_native_records' THEN
        SELECT tenant_id INTO reference_tenant
          FROM harness_sessions
         WHERE session_id = NEW.session_id;
        PERFORM harness_assert_same_tenant(
            NEW.tenant_id, reference_tenant, 'harness_native_records.session_id'
        );
        IF NEW.attempt_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_attempts
             WHERE attempt_id = NEW.attempt_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_native_records.attempt_id'
            );
        END IF;
    ELSIF TG_TABLE_NAME IN ('harness_command_inbox', 'harness_command_outbox') THEN
        SELECT tenant_id INTO reference_tenant
          FROM harness_sessions
         WHERE session_id = NEW.session_id;
        PERFORM harness_assert_same_tenant(
            NEW.tenant_id, reference_tenant, TG_TABLE_NAME || '.session_id'
        );
        IF NEW.task_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_tasks
             WHERE task_id = NEW.task_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, TG_TABLE_NAME || '.task_id'
            );
        END IF;
        IF NEW.turn_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_turns
             WHERE turn_id = NEW.turn_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, TG_TABLE_NAME || '.turn_id'
            );
        END IF;
        IF NEW.attempt_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_attempts
             WHERE attempt_id = NEW.attempt_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, TG_TABLE_NAME || '.attempt_id'
            );
        END IF;
        IF TG_TABLE_NAME = 'harness_command_inbox' AND NEW.outcome_event_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_events
             WHERE event_id = NEW.outcome_event_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_command_inbox.outcome_event_id'
            );
        ELSIF TG_TABLE_NAME = 'harness_command_outbox' THEN
            IF NEW.command_id IS NOT NULL THEN
                SELECT tenant_id INTO reference_tenant
                  FROM harness_command_inbox
                 WHERE command_id = NEW.command_id;
                PERFORM harness_assert_same_tenant(
                    NEW.tenant_id, reference_tenant, 'harness_command_outbox.command_id'
                );
            END IF;
            IF NEW.event_id IS NOT NULL THEN
                SELECT tenant_id INTO reference_tenant
                  FROM harness_events
                 WHERE event_id = NEW.event_id;
                PERFORM harness_assert_same_tenant(
                    NEW.tenant_id, reference_tenant, 'harness_command_outbox.event_id'
                );
            END IF;
        END IF;
    ELSIF TG_TABLE_NAME = 'harness_model_runtime_workers' THEN
        SELECT tenant_id INTO reference_tenant
          FROM harness_model_runtimes
         WHERE runtime_id = NEW.runtime_id;
        PERFORM harness_assert_same_tenant(
            NEW.tenant_id, reference_tenant, 'harness_model_runtime_workers.runtime_id'
        );
    ELSIF TG_TABLE_NAME = 'harness_inference_admissions' THEN
        SELECT tenant_id INTO reference_tenant
          FROM harness_sessions
         WHERE session_id = NEW.session_id;
        PERFORM harness_assert_same_tenant(
            NEW.tenant_id, reference_tenant, 'harness_inference_admissions.session_id'
        );
        IF NEW.task_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_tasks
             WHERE task_id = NEW.task_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_inference_admissions.task_id'
            );
        END IF;
        IF NEW.turn_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_turns
             WHERE turn_id = NEW.turn_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_inference_admissions.turn_id'
            );
        END IF;
        IF NEW.attempt_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_attempts
             WHERE attempt_id = NEW.attempt_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_inference_admissions.attempt_id'
            );
        END IF;
        SELECT tenant_id INTO reference_tenant
          FROM harness_model_bindings
         WHERE model_binding_id = NEW.model_binding_id;
        PERFORM harness_assert_same_tenant(
            NEW.tenant_id, reference_tenant, 'harness_inference_admissions.model_binding_id'
        );
        SELECT tenant_id INTO reference_tenant
          FROM harness_model_runtimes
         WHERE runtime_id = NEW.runtime_id;
        PERFORM harness_assert_same_tenant(
            NEW.tenant_id, reference_tenant, 'harness_inference_admissions.runtime_id'
        );
        SELECT tenant_id INTO reference_tenant
          FROM harness_model_runtime_workers
         WHERE worker_id = NEW.worker_id;
        PERFORM harness_assert_same_tenant(
            NEW.tenant_id, reference_tenant, 'harness_inference_admissions.worker_id'
        );
    ELSIF TG_TABLE_NAME = 'harness_handoff_operations' THEN
        SELECT tenant_id INTO reference_tenant
          FROM harness_sessions
         WHERE session_id = NEW.session_id;
        PERFORM harness_assert_same_tenant(
            NEW.tenant_id, reference_tenant, 'harness_handoff_operations.session_id'
        );
        IF NEW.task_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_tasks
             WHERE task_id = NEW.task_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_handoff_operations.task_id'
            );
        END IF;
        SELECT tenant_id INTO reference_tenant
          FROM harness_bindings
         WHERE binding_id = NEW.source_binding_id;
        PERFORM harness_assert_same_tenant(
            NEW.tenant_id, reference_tenant, 'harness_handoff_operations.source_binding_id'
        );
        IF NEW.target_binding_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_bindings
             WHERE binding_id = NEW.target_binding_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_handoff_operations.target_binding_id'
            );
        END IF;
    ELSIF TG_TABLE_NAME = 'harness_capsules' THEN
        SELECT tenant_id INTO reference_tenant
          FROM harness_sessions
         WHERE session_id = NEW.session_id;
        PERFORM harness_assert_same_tenant(
            NEW.tenant_id, reference_tenant, 'harness_capsules.session_id'
        );
        IF NEW.handoff_id IS NOT NULL THEN
            SELECT tenant_id INTO reference_tenant
              FROM harness_handoff_operations
             WHERE operation_id = NEW.handoff_id;
            PERFORM harness_assert_same_tenant(
                NEW.tenant_id, reference_tenant, 'harness_capsules.handoff_id'
            );
        END IF;
    END IF;
    RETURN NEW;
END;
$$;

DROP TRIGGER IF EXISTS harness_sessions_tenant_references ON harness_sessions;
CREATE TRIGGER harness_sessions_tenant_references
    BEFORE INSERT OR UPDATE ON harness_sessions
    FOR EACH ROW EXECUTE FUNCTION harness_validate_tenant_references();
DROP TRIGGER IF EXISTS harness_tasks_tenant_references ON harness_tasks;
CREATE TRIGGER harness_tasks_tenant_references
    BEFORE INSERT OR UPDATE ON harness_tasks
    FOR EACH ROW EXECUTE FUNCTION harness_validate_tenant_references();
DROP TRIGGER IF EXISTS harness_turns_tenant_references ON harness_turns;
CREATE TRIGGER harness_turns_tenant_references
    BEFORE INSERT OR UPDATE ON harness_turns
    FOR EACH ROW EXECUTE FUNCTION harness_validate_tenant_references();
DROP TRIGGER IF EXISTS harness_model_bindings_tenant_references ON harness_model_bindings;
CREATE TRIGGER harness_model_bindings_tenant_references
    BEFORE INSERT OR UPDATE ON harness_model_bindings
    FOR EACH ROW EXECUTE FUNCTION harness_validate_tenant_references();
DROP TRIGGER IF EXISTS harness_attempts_tenant_references ON harness_attempts;
CREATE TRIGGER harness_attempts_tenant_references
    BEFORE INSERT OR UPDATE ON harness_attempts
    FOR EACH ROW EXECUTE FUNCTION harness_validate_tenant_references();
DROP TRIGGER IF EXISTS harness_bindings_tenant_references ON harness_bindings;
CREATE TRIGGER harness_bindings_tenant_references
    BEFORE INSERT OR UPDATE ON harness_bindings
    FOR EACH ROW EXECUTE FUNCTION harness_validate_tenant_references();
DROP TRIGGER IF EXISTS harness_leases_tenant_references ON harness_leases;
CREATE TRIGGER harness_leases_tenant_references
    BEFORE INSERT OR UPDATE ON harness_leases
    FOR EACH ROW EXECUTE FUNCTION harness_validate_tenant_references();
DROP TRIGGER IF EXISTS harness_events_tenant_references ON harness_events;
CREATE TRIGGER harness_events_tenant_references
    BEFORE INSERT OR UPDATE ON harness_events
    FOR EACH ROW EXECUTE FUNCTION harness_validate_tenant_references();
DROP TRIGGER IF EXISTS harness_artifacts_tenant_references ON harness_artifacts;
CREATE TRIGGER harness_artifacts_tenant_references
    BEFORE INSERT OR UPDATE ON harness_artifacts
    FOR EACH ROW EXECUTE FUNCTION harness_validate_tenant_references();
DROP TRIGGER IF EXISTS harness_workspace_snapshots_tenant_references ON harness_workspace_snapshots;
CREATE TRIGGER harness_workspace_snapshots_tenant_references
    BEFORE INSERT OR UPDATE ON harness_workspace_snapshots
    FOR EACH ROW EXECUTE FUNCTION harness_validate_tenant_references();
DROP TRIGGER IF EXISTS harness_native_records_tenant_references ON harness_native_records;
CREATE TRIGGER harness_native_records_tenant_references
    BEFORE INSERT OR UPDATE ON harness_native_records
    FOR EACH ROW EXECUTE FUNCTION harness_validate_tenant_references();
DROP TRIGGER IF EXISTS harness_command_inbox_tenant_references ON harness_command_inbox;
CREATE TRIGGER harness_command_inbox_tenant_references
    BEFORE INSERT OR UPDATE ON harness_command_inbox
    FOR EACH ROW EXECUTE FUNCTION harness_validate_tenant_references();
DROP TRIGGER IF EXISTS harness_command_outbox_tenant_references ON harness_command_outbox;
CREATE TRIGGER harness_command_outbox_tenant_references
    BEFORE INSERT OR UPDATE ON harness_command_outbox
    FOR EACH ROW EXECUTE FUNCTION harness_validate_tenant_references();
DROP TRIGGER IF EXISTS harness_model_runtime_workers_tenant_references ON harness_model_runtime_workers;
CREATE TRIGGER harness_model_runtime_workers_tenant_references
    BEFORE INSERT OR UPDATE ON harness_model_runtime_workers
    FOR EACH ROW EXECUTE FUNCTION harness_validate_tenant_references();
DROP TRIGGER IF EXISTS harness_inference_admissions_tenant_references ON harness_inference_admissions;
CREATE TRIGGER harness_inference_admissions_tenant_references
    BEFORE INSERT OR UPDATE ON harness_inference_admissions
    FOR EACH ROW EXECUTE FUNCTION harness_validate_tenant_references();
DROP TRIGGER IF EXISTS harness_handoff_operations_tenant_references ON harness_handoff_operations;
CREATE TRIGGER harness_handoff_operations_tenant_references
    BEFORE INSERT OR UPDATE ON harness_handoff_operations
    FOR EACH ROW EXECUTE FUNCTION harness_validate_tenant_references();
DROP TRIGGER IF EXISTS harness_capsules_tenant_references ON harness_capsules;
CREATE TRIGGER harness_capsules_tenant_references
    BEFORE INSERT OR UPDATE ON harness_capsules
    FOR EACH ROW EXECUTE FUNCTION harness_validate_tenant_references();

CREATE OR REPLACE FUNCTION harness_current_lease(
    p_attempt_id TEXT,
    p_lease_id TEXT,
    p_generation BIGINT,
    p_fencing_token TEXT
) RETURNS BOOLEAN
LANGUAGE SQL
AS $$
    SELECT EXISTS (
        SELECT 1
        FROM harness_leases
        WHERE attempt_id = p_attempt_id
          AND lease_id = p_lease_id
          AND generation = p_generation
          AND fencing_token = p_fencing_token
          AND state = 'active'
          AND (expires_at IS NULL OR expires_at > CURRENT_TIMESTAMP)
    )
$$;

CREATE OR REPLACE FUNCTION harness_reject_stale_event_writer()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    IF NEW.ingest_attempt_id IS NOT NULL
       AND NOT harness_current_lease(
           NEW.ingest_attempt_id,
           NEW.lease_id,
           NEW.lease_generation,
           NEW.fencing_token
       ) THEN
        RAISE EXCEPTION 'stale harness event writer for attempt %', NEW.ingest_attempt_id;
    END IF;
    IF NEW.replay_requirement = 'required' AND NEW.durability <> 'durable' THEN
        RAISE EXCEPTION 'required replay event must be durable';
    END IF;
    RETURN NEW;
END;
$$;

DROP TRIGGER IF EXISTS harness_event_writer_fence ON harness_events;
CREATE TRIGGER harness_event_writer_fence
    BEFORE INSERT OR UPDATE ON harness_events
    FOR EACH ROW EXECUTE FUNCTION harness_reject_stale_event_writer();

CREATE OR REPLACE FUNCTION harness_validate_event_parent()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
DECLARE
    child_sequence BIGINT;
    parent_sequence BIGINT;
BEGIN
    SELECT durable_sequence INTO child_sequence
      FROM harness_events
     WHERE session_id = NEW.session_id AND event_id = NEW.event_id;
    SELECT durable_sequence INTO parent_sequence
      FROM harness_events
     WHERE session_id = NEW.session_id AND event_id = NEW.parent_event_id;
    IF child_sequence IS NULL OR parent_sequence IS NULL
       OR parent_sequence >= child_sequence THEN
        RAISE EXCEPTION 'event parent must be durable and earlier';
    END IF;
    RETURN NEW;
END;
$$;

DROP TRIGGER IF EXISTS harness_event_parent_order ON harness_event_parents;
CREATE TRIGGER harness_event_parent_order
    AFTER INSERT OR UPDATE ON harness_event_parents
    FOR EACH ROW EXECUTE FUNCTION harness_validate_event_parent();

-- Canonical middleware state is tenant-owned. Parent/head/loss-entry rows are
-- reachable only through tenant-scoped parent records and are protected by
-- companion policies below.
DO $$
DECLARE
    table_name TEXT;
BEGIN
    FOREACH table_name IN ARRAY ARRAY[
        'harness_sessions',
        'harness_tasks',
        'harness_turns',
        'harness_runtime_configs',
        'harness_model_runtimes',
        'harness_model_bindings',
        'harness_attempts',
        'harness_bindings',
        'harness_leases',
        'harness_events',
        'harness_artifacts',
        'harness_workspace_snapshots',
        'harness_native_records',
        'harness_command_inbox',
        'harness_command_outbox',
        'harness_model_runtime_workers',
        'harness_inference_admissions',
        'harness_handoff_operations',
        'harness_capsules'
    ]
    LOOP
        EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', table_name);
        EXECUTE format('ALTER TABLE %I FORCE ROW LEVEL SECURITY', table_name);
        EXECUTE format('DROP POLICY IF EXISTS %I ON %I',
            'harness_tenant_isolation_' || table_name, table_name);
        EXECUTE format(
            'CREATE POLICY %I ON %I USING (tenant_id = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id = current_setting(''app.current_tenant'', true))',
            'harness_tenant_isolation_' || table_name,
            table_name
        );
    END LOOP;
END
$$;

ALTER TABLE harness_event_parents ENABLE ROW LEVEL SECURITY;
ALTER TABLE harness_event_parents FORCE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS harness_tenant_isolation_event_parents ON harness_event_parents;
CREATE POLICY harness_tenant_isolation_event_parents ON harness_event_parents
    USING (EXISTS (
        SELECT 1 FROM harness_sessions
        WHERE harness_sessions.session_id = harness_event_parents.session_id
          AND harness_sessions.tenant_id = current_setting('app.current_tenant', true)
    ))
    WITH CHECK (EXISTS (
        SELECT 1 FROM harness_sessions
        WHERE harness_sessions.session_id = harness_event_parents.session_id
          AND harness_sessions.tenant_id = current_setting('app.current_tenant', true)
    ));

ALTER TABLE harness_event_branch_heads ENABLE ROW LEVEL SECURITY;
ALTER TABLE harness_event_branch_heads FORCE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS harness_tenant_isolation_event_branch_heads ON harness_event_branch_heads;
CREATE POLICY harness_tenant_isolation_event_branch_heads ON harness_event_branch_heads
    USING (EXISTS (
        SELECT 1 FROM harness_sessions
        WHERE harness_sessions.session_id = harness_event_branch_heads.session_id
          AND harness_sessions.tenant_id = current_setting('app.current_tenant', true)
    ))
    WITH CHECK (EXISTS (
        SELECT 1 FROM harness_sessions
        WHERE harness_sessions.session_id = harness_event_branch_heads.session_id
          AND harness_sessions.tenant_id = current_setting('app.current_tenant', true)
    ));

ALTER TABLE harness_capsule_loss_entries ENABLE ROW LEVEL SECURITY;
ALTER TABLE harness_capsule_loss_entries FORCE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS harness_tenant_isolation_capsule_loss_entries ON harness_capsule_loss_entries;
CREATE POLICY harness_tenant_isolation_capsule_loss_entries ON harness_capsule_loss_entries
    USING (EXISTS (
        SELECT 1 FROM harness_capsules
        WHERE harness_capsules.capsule_id = harness_capsule_loss_entries.capsule_id
          AND harness_capsules.tenant_id = current_setting('app.current_tenant', true)
    ))
    WITH CHECK (EXISTS (
        SELECT 1 FROM harness_capsules
        WHERE harness_capsules.capsule_id = harness_capsule_loss_entries.capsule_id
          AND harness_capsules.tenant_id = current_setting('app.current_tenant', true)
    ));
