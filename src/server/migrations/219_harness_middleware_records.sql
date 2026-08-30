-- OmniSolo harness middleware record extension.
--
-- Migration 218 owns the lifecycle and event spine. This migration stores the
-- typed records required to export, resume, or transfer a session/task across
-- harnesses. Every record carries a tenant fence even when its payload is
-- intentionally opaque to the middleware.

CREATE TABLE IF NOT EXISTS harness_descriptors (
    harness_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    adapter_id TEXT NOT NULL,
    display_name TEXT NOT NULL,
    protocol_kind TEXT NOT NULL,
    protocol_version TEXT NOT NULL,
    adapter_version TEXT NOT NULL,
    source_revision TEXT,
    schema_revision TEXT,
    capabilities JSONB NOT NULL DEFAULT '[]'::jsonb,
    worker_image TEXT,
    worker_pool TEXT,
    state_locality TEXT,
    native_extension_namespace TEXT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, harness_id)
);

CREATE TABLE IF NOT EXISTS harness_capability_snapshots (
    snapshot_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT,
    binding_id TEXT,
    harness_id TEXT NOT NULL,
    capability_version TEXT NOT NULL,
    capabilities JSONB NOT NULL DEFAULT '[]'::jsonb,
    snapshot_digest TEXT NOT NULL,
    captured_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, snapshot_id),
    UNIQUE (tenant_id, snapshot_digest)
);

CREATE TABLE IF NOT EXISTS harness_model_descriptors (
    model_descriptor_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    model_id TEXT NOT NULL,
    family TEXT,
    provider TEXT NOT NULL,
    revision TEXT,
    weight_digest TEXT,
    tokenizer_digest TEXT,
    context_window BIGINT,
    max_output_tokens BIGINT,
    modalities JSONB NOT NULL DEFAULT '[]'::jsonb,
    capabilities JSONB NOT NULL DEFAULT '[]'::jsonb,
    quantization TEXT,
    license TEXT,
    provenance TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, model_descriptor_id)
);

CREATE TABLE IF NOT EXISTS harness_model_runtime_capabilities (
    capability_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    runtime_id TEXT NOT NULL,
    capability TEXT NOT NULL,
    capability_version TEXT,
    state TEXT NOT NULL DEFAULT 'advertised',
    constraints JSONB NOT NULL DEFAULT '{}'::jsonb,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    captured_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, runtime_id, capability)
);

CREATE TABLE IF NOT EXISTS harness_model_capacity_leases (
    capacity_lease_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    runtime_id TEXT NOT NULL,
    worker_id TEXT,
    request_id TEXT,
    state TEXT NOT NULL DEFAULT 'active',
    generation BIGINT NOT NULL DEFAULT 1,
    reserved_requests INTEGER NOT NULL DEFAULT 1,
    reserved_tokens BIGINT,
    expires_at TIMESTAMPTZ,
    released_at TIMESTAMPTZ,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    UNIQUE (tenant_id, capacity_lease_id)
);

CREATE TABLE IF NOT EXISTS harness_inference_requests (
    request_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT,
    task_id TEXT,
    turn_id TEXT,
    attempt_id TEXT,
    model_binding_id TEXT,
    runtime_id TEXT,
    worker_id TEXT,
    idempotency_key TEXT NOT NULL,
    state TEXT NOT NULL DEFAULT 'requested',
    request_schema TEXT NOT NULL,
    request_version INTEGER NOT NULL DEFAULT 1,
    request_payload JSONB NOT NULL,
    admitted_at TIMESTAMPTZ,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    uncertain_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, idempotency_key)
);

CREATE TABLE IF NOT EXISTS harness_inference_stream_records (
    stream_record_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    request_id TEXT NOT NULL,
    delivery_sequence BIGINT NOT NULL,
    record_type TEXT NOT NULL,
    payload_schema TEXT NOT NULL,
    payload_version INTEGER NOT NULL DEFAULT 1,
    payload JSONB NOT NULL,
    usage_payload JSONB,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, request_id, delivery_sequence)
);

CREATE TABLE IF NOT EXISTS harness_inference_final_responses (
    final_response_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    request_id TEXT NOT NULL,
    state TEXT NOT NULL,
    response_schema TEXT NOT NULL,
    response_version INTEGER NOT NULL DEFAULT 1,
    response_payload JSONB,
    usage_payload JSONB,
    error_payload JSONB,
    uncertain BOOLEAN NOT NULL DEFAULT FALSE,
    completed_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, request_id)
);

CREATE TABLE IF NOT EXISTS harness_actors (
    actor_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    actor_type TEXT NOT NULL,
    display_name TEXT,
    parent_actor_id TEXT,
    delegation_lineage JSONB NOT NULL DEFAULT '[]'::jsonb,
    authority_context JSONB NOT NULL DEFAULT '{}'::jsonb,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, actor_id)
);

CREATE TABLE IF NOT EXISTS harness_actor_bindings (
    actor_binding_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    binding_id TEXT NOT NULL,
    generation BIGINT NOT NULL,
    canonical_actor_id TEXT NOT NULL,
    native_actor_id TEXT NOT NULL,
    actor_type TEXT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    captured_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, binding_id, generation, canonical_actor_id)
);

CREATE TABLE IF NOT EXISTS harness_operation_fences (
    fence_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    operation_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    task_id TEXT,
    worker_id TEXT NOT NULL,
    harness_id TEXT NOT NULL,
    generation BIGINT NOT NULL,
    fencing_token TEXT NOT NULL,
    state TEXT NOT NULL DEFAULT 'active',
    issued_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMPTZ,
    released_at TIMESTAMPTZ,
    UNIQUE (tenant_id, operation_id, generation),
    UNIQUE (tenant_id, fencing_token)
);

CREATE TABLE IF NOT EXISTS harness_messages (
    message_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    task_id TEXT,
    turn_id TEXT,
    role TEXT NOT NULL,
    actor_id TEXT,
    origin TEXT,
    phase TEXT,
    parent_message_id TEXT,
    correlation_id TEXT,
    status TEXT NOT NULL DEFAULT 'settled',
    visible_to_user BOOLEAN NOT NULL DEFAULT TRUE,
    redaction_state TEXT,
    native_provenance JSONB NOT NULL DEFAULT '{}'::jsonb,
    extensions JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    settled_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS harness_content_parts (
    content_part_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    message_id TEXT NOT NULL,
    ordinal INTEGER NOT NULL,
    part_type TEXT NOT NULL,
    payload JSONB NOT NULL,
    artifact_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, message_id, ordinal)
);

CREATE TABLE IF NOT EXISTS harness_tool_definition_snapshots (
    snapshot_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    qualified_name TEXT NOT NULL,
    description TEXT,
    source TEXT NOT NULL,
    input_schema JSONB NOT NULL,
    output_schema JSONB,
    annotations JSONB NOT NULL DEFAULT '{}'::jsonb,
    version TEXT NOT NULL,
    digest TEXT NOT NULL,
    captured_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, digest)
);

CREATE TABLE IF NOT EXISTS harness_tool_calls (
    tool_call_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    task_id TEXT NOT NULL,
    turn_id TEXT,
    attempt_id TEXT NOT NULL,
    definition_snapshot_id TEXT NOT NULL,
    raw_input JSONB NOT NULL,
    parsed_input JSONB,
    state TEXT NOT NULL DEFAULT 'pending',
    started_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    finished_at TIMESTAMPTZ,
    retry_of TEXT,
    native_provenance JSONB NOT NULL DEFAULT '{}'::jsonb,
    observed_effect_ids JSONB NOT NULL DEFAULT '[]'::jsonb,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE TABLE IF NOT EXISTS harness_tool_progress (
    progress_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    tool_call_id TEXT NOT NULL,
    sequence BIGINT NOT NULL,
    chunk_type TEXT NOT NULL,
    content JSONB NOT NULL DEFAULT '[]'::jsonb,
    progress_percent SMALLINT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, tool_call_id, sequence)
);

CREATE TABLE IF NOT EXISTS harness_tool_results (
    result_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    tool_call_id TEXT NOT NULL,
    state TEXT NOT NULL,
    content JSONB NOT NULL DEFAULT '[]'::jsonb,
    structured_output JSONB,
    error_payload JSONB,
    observed_effect_ids JSONB NOT NULL DEFAULT '[]'::jsonb,
    completed_at TIMESTAMPTZ,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    UNIQUE (tenant_id, tool_call_id)
);

CREATE TABLE IF NOT EXISTS harness_interactions (
    interaction_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    task_id TEXT,
    turn_id TEXT,
    attempt_id TEXT,
    interaction_kind TEXT NOT NULL,
    risk TEXT NOT NULL DEFAULT 'low',
    state TEXT NOT NULL DEFAULT 'requested',
    policy_decision JSONB,
    request_payload JSONB NOT NULL,
    response_payload JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    resolved_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS harness_interaction_responses (
    response_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    interaction_id TEXT NOT NULL,
    ordinal INTEGER NOT NULL,
    response_kind TEXT NOT NULL,
    payload JSONB NOT NULL,
    actor_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, interaction_id, ordinal)
);

CREATE TABLE IF NOT EXISTS harness_plans (
    plan_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    task_id TEXT,
    attempt_id TEXT,
    state TEXT NOT NULL DEFAULT 'draft',
    summary TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS harness_plan_steps (
    step_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    plan_id TEXT NOT NULL,
    ordinal INTEGER NOT NULL,
    description TEXT NOT NULL,
    state TEXT NOT NULL DEFAULT 'pending',
    dependency_step_ids JSONB NOT NULL DEFAULT '[]'::jsonb,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    UNIQUE (tenant_id, plan_id, ordinal)
);

CREATE TABLE IF NOT EXISTS harness_todos (
    todo_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    task_id TEXT,
    plan_id TEXT,
    ordinal INTEGER NOT NULL,
    content TEXT NOT NULL,
    state TEXT NOT NULL DEFAULT 'pending',
    owner_actor_id TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, session_id, ordinal)
);

CREATE TABLE IF NOT EXISTS harness_goals (
    goal_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    task_id TEXT,
    parent_goal_id TEXT,
    state TEXT NOT NULL DEFAULT 'active',
    objective TEXT NOT NULL,
    budget JSONB NOT NULL DEFAULT '{}'::jsonb,
    usage_payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS harness_processes (
    process_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    task_id TEXT,
    attempt_id TEXT,
    state TEXT NOT NULL DEFAULT 'created',
    command JSONB NOT NULL DEFAULT '[]'::jsonb,
    working_directory TEXT,
    environment JSONB NOT NULL DEFAULT '{}'::jsonb,
    process_group TEXT,
    native_process_ref TEXT,
    exit_code INTEGER,
    signal_name TEXT,
    started_at TIMESTAMPTZ,
    finished_at TIMESTAMPTZ,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE TABLE IF NOT EXISTS harness_process_chunks (
    chunk_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    process_id TEXT NOT NULL,
    sequence BIGINT NOT NULL,
    stream TEXT NOT NULL,
    content BYTEA NOT NULL,
    content_encoding TEXT NOT NULL DEFAULT 'raw',
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, process_id, sequence)
);

CREATE TABLE IF NOT EXISTS harness_workspace_descriptors (
    workspace_descriptor_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    workspace_id TEXT,
    root_path TEXT NOT NULL,
    source TEXT NOT NULL,
    mutation_policy JSONB NOT NULL DEFAULT '{}'::jsonb,
    isolation_mode TEXT,
    repository JSONB NOT NULL DEFAULT '{}'::jsonb,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS harness_usage_records (
    usage_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    task_id TEXT,
    turn_id TEXT,
    attempt_id TEXT,
    inference_request_id TEXT,
    category TEXT NOT NULL,
    provider TEXT,
    model_id TEXT,
    input_tokens BIGINT,
    output_tokens BIGINT,
    cached_tokens BIGINT,
    reasoning_tokens BIGINT,
    latency_ms BIGINT,
    cost_micros BIGINT,
    usage_payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS harness_error_records (
    error_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    task_id TEXT,
    turn_id TEXT,
    attempt_id TEXT,
    component TEXT NOT NULL,
    error_code TEXT NOT NULL,
    error_class TEXT,
    message TEXT NOT NULL,
    retryable BOOLEAN NOT NULL DEFAULT FALSE,
    uncertain_effect BOOLEAN NOT NULL DEFAULT FALSE,
    details JSONB NOT NULL DEFAULT '{}'::jsonb,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    resolved_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS harness_compactions (
    compaction_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    task_id TEXT,
    turn_id TEXT,
    from_durable_sequence BIGINT NOT NULL,
    to_durable_sequence BIGINT NOT NULL,
    summary TEXT NOT NULL,
    retained_message_ids JSONB NOT NULL DEFAULT '[]'::jsonb,
    checkpoint_id TEXT,
    provenance JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS harness_portable_context_checkpoints (
    checkpoint_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    task_id TEXT,
    turn_id TEXT,
    durable_sequence BIGINT NOT NULL,
    summary TEXT NOT NULL,
    selected_message_ids JSONB NOT NULL DEFAULT '[]'::jsonb,
    source_durable_ranges JSONB NOT NULL DEFAULT '[]'::jsonb,
    retained_ancestor_event_id TEXT,
    compaction_provenance TEXT,
    usage_payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    integrity_digest TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, integrity_digest)
);

CREATE TABLE IF NOT EXISTS harness_resume_checkpoints (
    checkpoint_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    task_id TEXT,
    turn_id TEXT,
    attempt_id TEXT,
    binding_id TEXT,
    harness_id TEXT NOT NULL,
    native_session_id TEXT,
    native_cursor TEXT,
    durable_sequence BIGINT NOT NULL,
    exact_resume_eligible BOOLEAN NOT NULL DEFAULT FALSE,
    portable_context_checkpoint_id TEXT,
    state_payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    checkpoint_digest TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, checkpoint_digest)
);

CREATE TABLE IF NOT EXISTS harness_native_record_sets (
    record_set_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    task_id TEXT,
    attempt_id TEXT,
    binding_id TEXT,
    harness_id TEXT NOT NULL,
    native_namespace TEXT NOT NULL,
    record_ids JSONB NOT NULL DEFAULT '[]'::jsonb,
    records JSONB NOT NULL DEFAULT '[]'::jsonb,
    record_digest TEXT NOT NULL,
    captured_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, record_digest)
);

CREATE TABLE IF NOT EXISTS harness_capsule_record_batches (
    batch_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    capsule_id TEXT NOT NULL,
    ordinal INTEGER NOT NULL,
    record_kind TEXT NOT NULL,
    record_schema TEXT NOT NULL,
    record_version INTEGER NOT NULL DEFAULT 1,
    payload JSONB NOT NULL,
    record_digest TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, capsule_id, ordinal)
);

CREATE TABLE IF NOT EXISTS harness_handoff_loss_reports (
    report_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    operation_id TEXT,
    capsule_id TEXT,
    source_harness_id TEXT NOT NULL,
    target_harness_id TEXT NOT NULL,
    report_digest TEXT NOT NULL,
    acknowledgement_required BOOLEAN NOT NULL DEFAULT FALSE,
    acknowledgement_digest TEXT,
    entries JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    acknowledged_at TIMESTAMPTZ,
    UNIQUE (tenant_id, report_digest)
);

ALTER TABLE harness_model_bindings
    ADD COLUMN IF NOT EXISTS resolved_model JSONB;

DO $$
DECLARE
    table_name TEXT;
BEGIN
    FOREACH table_name IN ARRAY ARRAY[
        'harness_descriptors',
        'harness_capability_snapshots',
        'harness_model_descriptors',
        'harness_model_runtime_capabilities',
        'harness_model_capacity_leases',
        'harness_inference_requests',
        'harness_inference_stream_records',
        'harness_inference_final_responses',
        'harness_actors',
        'harness_actor_bindings',
        'harness_operation_fences',
        'harness_messages',
        'harness_content_parts',
        'harness_tool_definition_snapshots',
        'harness_tool_calls',
        'harness_tool_progress',
        'harness_tool_results',
        'harness_interactions',
        'harness_interaction_responses',
        'harness_plans',
        'harness_plan_steps',
        'harness_todos',
        'harness_goals',
        'harness_processes',
        'harness_process_chunks',
        'harness_workspace_descriptors',
        'harness_usage_records',
        'harness_error_records',
        'harness_compactions',
        'harness_portable_context_checkpoints',
        'harness_resume_checkpoints',
        'harness_native_record_sets',
        'harness_capsule_record_batches',
        'harness_handoff_loss_reports'
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
