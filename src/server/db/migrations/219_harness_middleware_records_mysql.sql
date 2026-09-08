-- OmniSolo harness middleware record extension for MySQL 8.0.13+.
-- Every table is tenant-owned because MySQL has no row-level security.

CREATE TABLE IF NOT EXISTS harness_descriptors (
    harness_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    adapter_id VARCHAR(191) NOT NULL,
    display_name VARCHAR(191) NOT NULL,
    protocol_kind VARCHAR(191) NOT NULL,
    protocol_version VARCHAR(191) NOT NULL,
    adapter_version VARCHAR(191) NOT NULL,
    source_revision VARCHAR(191),
    schema_revision VARCHAR(191),
    capabilities JSON NOT NULL DEFAULT ('[]'),
    worker_image TEXT,
    worker_pool VARCHAR(191),
    state_locality VARCHAR(191),
    native_extension_namespace VARCHAR(191) NOT NULL,
    metadata JSON NOT NULL DEFAULT ('{}'),
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    updated_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (harness_id),
    UNIQUE KEY uq_hdesc_tenant_harness (tenant_id, harness_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_capability_snapshots (
    snapshot_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191),
    binding_id VARCHAR(191),
    harness_id VARCHAR(191) NOT NULL,
    capability_version VARCHAR(191) NOT NULL,
    capabilities JSON NOT NULL DEFAULT ('[]'),
    snapshot_digest VARCHAR(191) NOT NULL,
    captured_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (snapshot_id),
    UNIQUE KEY uq_hcaps_tenant_snapshot (tenant_id, snapshot_id),
    UNIQUE KEY uq_hcaps_tenant_digest (tenant_id, snapshot_digest)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_model_descriptors (
    model_descriptor_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    model_id VARCHAR(191) NOT NULL,
    family VARCHAR(191),
    provider VARCHAR(191) NOT NULL,
    revision VARCHAR(191),
    weight_digest VARCHAR(191),
    tokenizer_digest VARCHAR(191),
    context_window BIGINT,
    max_output_tokens BIGINT,
    modalities JSON NOT NULL DEFAULT ('[]'),
    capabilities JSON NOT NULL DEFAULT ('[]'),
    quantization VARCHAR(191),
    license VARCHAR(191),
    provenance TEXT,
    metadata JSON NOT NULL DEFAULT ('{}'),
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (model_descriptor_id),
    UNIQUE KEY uq_hmodel_tenant_descriptor (tenant_id, model_descriptor_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_model_runtime_capabilities (
    capability_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    runtime_id VARCHAR(191) NOT NULL,
    capability VARCHAR(191) NOT NULL,
    capability_version VARCHAR(191),
    state VARCHAR(191) NOT NULL DEFAULT 'advertised',
    constraints JSON NOT NULL DEFAULT ('{}'),
    metadata JSON NOT NULL DEFAULT ('{}'),
    captured_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (capability_id),
    UNIQUE KEY uq_hmrc_tenant_runtime_capability (tenant_id, runtime_id, capability)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_model_capacity_leases (
    capacity_lease_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    runtime_id VARCHAR(191) NOT NULL,
    worker_id VARCHAR(191),
    request_id VARCHAR(191),
    state VARCHAR(191) NOT NULL DEFAULT 'active',
    generation BIGINT NOT NULL DEFAULT 1,
    reserved_requests INT NOT NULL DEFAULT 1,
    reserved_tokens BIGINT,
    expires_at DATETIME(6),
    released_at DATETIME(6),
    metadata JSON NOT NULL DEFAULT ('{}'),
    PRIMARY KEY (capacity_lease_id),
    UNIQUE KEY uq_hmcl_tenant_lease (tenant_id, capacity_lease_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_inference_requests (
    request_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191),
    task_id VARCHAR(191),
    turn_id VARCHAR(191),
    attempt_id VARCHAR(191),
    model_binding_id VARCHAR(191),
    runtime_id VARCHAR(191),
    worker_id VARCHAR(191),
    idempotency_key VARCHAR(191) NOT NULL,
    state VARCHAR(191) NOT NULL DEFAULT 'requested',
    request_schema VARCHAR(191) NOT NULL,
    request_version INT NOT NULL DEFAULT 1,
    request_payload JSON NOT NULL,
    admitted_at DATETIME(6),
    started_at DATETIME(6),
    completed_at DATETIME(6),
    uncertain_reason TEXT,
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (request_id),
    UNIQUE KEY uq_hir_tenant_idempotency (tenant_id, idempotency_key)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_inference_stream_records (
    stream_record_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    request_id VARCHAR(191) NOT NULL,
    delivery_sequence BIGINT NOT NULL,
    record_type VARCHAR(191) NOT NULL,
    payload_schema VARCHAR(191) NOT NULL,
    payload_version INT NOT NULL DEFAULT 1,
    payload JSON NOT NULL,
    usage_payload JSON,
    occurred_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (stream_record_id),
    UNIQUE KEY uq_hisr_tenant_request_sequence (tenant_id, request_id, delivery_sequence)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_inference_final_responses (
    final_response_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    request_id VARCHAR(191) NOT NULL,
    state VARCHAR(191) NOT NULL,
    response_schema VARCHAR(191) NOT NULL,
    response_version INT NOT NULL DEFAULT 1,
    response_payload JSON,
    usage_payload JSON,
    error_payload JSON,
    uncertain BOOLEAN NOT NULL DEFAULT FALSE,
    completed_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (final_response_id),
    UNIQUE KEY uq_hifr_tenant_request (tenant_id, request_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_actors (
    actor_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    actor_type VARCHAR(191) NOT NULL,
    display_name VARCHAR(191),
    parent_actor_id VARCHAR(191),
    delegation_lineage JSON NOT NULL DEFAULT ('[]'),
    authority_context JSON NOT NULL DEFAULT ('{}'),
    metadata JSON NOT NULL DEFAULT ('{}'),
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (actor_id),
    UNIQUE KEY uq_hactors_tenant_actor (tenant_id, actor_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_actor_bindings (
    actor_binding_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    binding_id VARCHAR(191) NOT NULL,
    generation BIGINT NOT NULL,
    canonical_actor_id VARCHAR(191) NOT NULL,
    native_actor_id VARCHAR(191) NOT NULL,
    actor_type VARCHAR(191) NOT NULL,
    metadata JSON NOT NULL DEFAULT ('{}'),
    captured_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (actor_binding_id),
    UNIQUE KEY uq_hab_tenant_binding_generation_actor (tenant_id, binding_id, generation, canonical_actor_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_operation_fences (
    fence_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    operation_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191),
    worker_id VARCHAR(191) NOT NULL,
    harness_id VARCHAR(191) NOT NULL,
    generation BIGINT NOT NULL,
    fencing_token VARCHAR(191) NOT NULL,
    state VARCHAR(191) NOT NULL DEFAULT 'active',
    issued_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    expires_at DATETIME(6),
    released_at DATETIME(6),
    PRIMARY KEY (fence_id),
    UNIQUE KEY uq_hof_tenant_operation_generation (tenant_id, operation_id, generation),
    UNIQUE KEY uq_hof_tenant_token (tenant_id, fencing_token)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_messages (
    message_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191),
    turn_id VARCHAR(191),
    role VARCHAR(191) NOT NULL,
    actor_id VARCHAR(191),
    origin VARCHAR(191),
    phase VARCHAR(191),
    parent_message_id VARCHAR(191),
    correlation_id VARCHAR(191),
    status VARCHAR(191) NOT NULL DEFAULT 'settled',
    visible_to_user BOOLEAN NOT NULL DEFAULT TRUE,
    redaction_state VARCHAR(191),
    native_provenance JSON NOT NULL DEFAULT ('{}'),
    extensions JSON NOT NULL DEFAULT ('{}'),
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    settled_at DATETIME(6),
    PRIMARY KEY (message_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_content_parts (
    content_part_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    message_id VARCHAR(191) NOT NULL,
    ordinal INT NOT NULL,
    part_type VARCHAR(191) NOT NULL,
    payload JSON NOT NULL,
    artifact_id VARCHAR(191),
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (content_part_id),
    UNIQUE KEY uq_hcp_tenant_message_ordinal (tenant_id, message_id, ordinal)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_tool_definition_snapshots (
    snapshot_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    qualified_name VARCHAR(191) NOT NULL,
    description TEXT,
    source VARCHAR(191) NOT NULL,
    input_schema JSON NOT NULL,
    output_schema JSON,
    annotations JSON NOT NULL DEFAULT ('{}'),
    version VARCHAR(191) NOT NULL,
    digest VARCHAR(191) NOT NULL,
    captured_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (snapshot_id),
    UNIQUE KEY uq_htds_tenant_digest (tenant_id, digest)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_tool_calls (
    tool_call_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191) NOT NULL,
    turn_id VARCHAR(191),
    attempt_id VARCHAR(191) NOT NULL,
    definition_snapshot_id VARCHAR(191) NOT NULL,
    raw_input JSON NOT NULL,
    parsed_input JSON,
    state VARCHAR(191) NOT NULL DEFAULT 'pending',
    started_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    finished_at DATETIME(6),
    retry_of VARCHAR(191),
    native_provenance JSON NOT NULL DEFAULT ('{}'),
    observed_effect_ids JSON NOT NULL DEFAULT ('[]'),
    metadata JSON NOT NULL DEFAULT ('{}'),
    PRIMARY KEY (tool_call_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_tool_progress (
    progress_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    tool_call_id VARCHAR(191) NOT NULL,
    sequence BIGINT NOT NULL,
    chunk_type VARCHAR(191) NOT NULL,
    content JSON NOT NULL DEFAULT ('[]'),
    progress_percent SMALLINT,
    metadata JSON NOT NULL DEFAULT ('{}'),
    occurred_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (progress_id),
    UNIQUE KEY uq_htp_tenant_tool_sequence (tenant_id, tool_call_id, sequence)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_tool_results (
    result_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    tool_call_id VARCHAR(191) NOT NULL,
    state VARCHAR(191) NOT NULL,
    content JSON NOT NULL DEFAULT ('[]'),
    structured_output JSON,
    error_payload JSON,
    observed_effect_ids JSON NOT NULL DEFAULT ('[]'),
    completed_at DATETIME(6),
    metadata JSON NOT NULL DEFAULT ('{}'),
    PRIMARY KEY (result_id),
    UNIQUE KEY uq_htr_tenant_tool (tenant_id, tool_call_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_interactions (
    interaction_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191),
    turn_id VARCHAR(191),
    attempt_id VARCHAR(191),
    interaction_kind VARCHAR(191) NOT NULL,
    risk VARCHAR(191) NOT NULL DEFAULT 'low',
    state VARCHAR(191) NOT NULL DEFAULT 'requested',
    policy_decision JSON,
    request_payload JSON NOT NULL,
    response_payload JSON,
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    resolved_at DATETIME(6),
    PRIMARY KEY (interaction_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_interaction_responses (
    response_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    interaction_id VARCHAR(191) NOT NULL,
    ordinal INT NOT NULL,
    response_kind VARCHAR(191) NOT NULL,
    payload JSON NOT NULL,
    actor_id VARCHAR(191),
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (response_id),
    UNIQUE KEY uq_hirsp_tenant_interaction_ordinal (tenant_id, interaction_id, ordinal)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_plans (
    plan_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191),
    attempt_id VARCHAR(191),
    state VARCHAR(191) NOT NULL DEFAULT 'draft',
    summary TEXT,
    metadata JSON NOT NULL DEFAULT ('{}'),
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    updated_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (plan_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_plan_steps (
    step_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    plan_id VARCHAR(191) NOT NULL,
    ordinal INT NOT NULL,
    description TEXT NOT NULL,
    state VARCHAR(191) NOT NULL DEFAULT 'pending',
    dependency_step_ids JSON NOT NULL DEFAULT ('[]'),
    metadata JSON NOT NULL DEFAULT ('{}'),
    started_at DATETIME(6),
    completed_at DATETIME(6),
    PRIMARY KEY (step_id),
    UNIQUE KEY uq_hps_tenant_plan_ordinal (tenant_id, plan_id, ordinal)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_todos (
    todo_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191),
    plan_id VARCHAR(191),
    ordinal INT NOT NULL,
    content TEXT NOT NULL,
    state VARCHAR(191) NOT NULL DEFAULT 'pending',
    owner_actor_id VARCHAR(191),
    metadata JSON NOT NULL DEFAULT ('{}'),
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    updated_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (todo_id),
    UNIQUE KEY uq_htodo_tenant_session_ordinal (tenant_id, session_id, ordinal)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_goals (
    goal_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191),
    parent_goal_id VARCHAR(191),
    state VARCHAR(191) NOT NULL DEFAULT 'active',
    objective TEXT NOT NULL,
    budget JSON NOT NULL DEFAULT ('{}'),
    usage_payload JSON NOT NULL DEFAULT ('{}'),
    metadata JSON NOT NULL DEFAULT ('{}'),
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    updated_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (goal_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_processes (
    process_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191),
    attempt_id VARCHAR(191),
    state VARCHAR(191) NOT NULL DEFAULT 'created',
    command JSON NOT NULL DEFAULT ('[]'),
    working_directory TEXT,
    environment JSON NOT NULL DEFAULT ('{}'),
    process_group VARCHAR(191),
    native_process_ref VARCHAR(191),
    exit_code INT,
    signal_name VARCHAR(191),
    started_at DATETIME(6),
    finished_at DATETIME(6),
    metadata JSON NOT NULL DEFAULT ('{}'),
    PRIMARY KEY (process_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_process_chunks (
    chunk_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    process_id VARCHAR(191) NOT NULL,
    sequence BIGINT NOT NULL,
    stream VARCHAR(191) NOT NULL,
    content LONGBLOB NOT NULL,
    content_encoding VARCHAR(191) NOT NULL DEFAULT 'raw',
    occurred_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (chunk_id),
    UNIQUE KEY uq_hpc_tenant_process_sequence (tenant_id, process_id, sequence)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_workspace_descriptors (
    workspace_descriptor_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    workspace_id VARCHAR(191),
    root_path TEXT NOT NULL,
    source VARCHAR(191) NOT NULL,
    mutation_policy JSON NOT NULL DEFAULT ('{}'),
    isolation_mode VARCHAR(191),
    repository JSON NOT NULL DEFAULT ('{}'),
    metadata JSON NOT NULL DEFAULT ('{}'),
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (workspace_descriptor_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_usage_records (
    usage_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191),
    turn_id VARCHAR(191),
    attempt_id VARCHAR(191),
    inference_request_id VARCHAR(191),
    category VARCHAR(191) NOT NULL,
    provider VARCHAR(191),
    model_id VARCHAR(191),
    input_tokens BIGINT,
    output_tokens BIGINT,
    cached_tokens BIGINT,
    reasoning_tokens BIGINT,
    latency_ms BIGINT,
    cost_micros BIGINT,
    usage_payload JSON NOT NULL DEFAULT ('{}'),
    occurred_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (usage_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_error_records (
    error_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191),
    turn_id VARCHAR(191),
    attempt_id VARCHAR(191),
    component VARCHAR(191) NOT NULL,
    error_code VARCHAR(191) NOT NULL,
    error_class VARCHAR(191),
    message TEXT NOT NULL,
    retryable BOOLEAN NOT NULL DEFAULT FALSE,
    uncertain_effect BOOLEAN NOT NULL DEFAULT FALSE,
    details JSON NOT NULL DEFAULT ('{}'),
    occurred_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    resolved_at DATETIME(6),
    PRIMARY KEY (error_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_compactions (
    compaction_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191),
    turn_id VARCHAR(191),
    from_durable_sequence BIGINT NOT NULL,
    to_durable_sequence BIGINT NOT NULL,
    summary TEXT NOT NULL,
    retained_message_ids JSON NOT NULL DEFAULT ('[]'),
    checkpoint_id VARCHAR(191),
    provenance JSON NOT NULL DEFAULT ('{}'),
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (compaction_id)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_portable_context_checkpoints (
    checkpoint_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191),
    turn_id VARCHAR(191),
    durable_sequence BIGINT NOT NULL,
    summary TEXT NOT NULL,
    selected_message_ids JSON NOT NULL DEFAULT ('[]'),
    source_durable_ranges JSON NOT NULL DEFAULT ('[]'),
    retained_ancestor_event_id VARCHAR(191),
    compaction_provenance TEXT,
    usage_payload JSON NOT NULL DEFAULT ('{}'),
    integrity_digest VARCHAR(191) NOT NULL,
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (checkpoint_id),
    UNIQUE KEY uq_hpcp_tenant_digest (tenant_id, integrity_digest)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_resume_checkpoints (
    checkpoint_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191),
    turn_id VARCHAR(191),
    attempt_id VARCHAR(191),
    binding_id VARCHAR(191),
    harness_id VARCHAR(191) NOT NULL,
    native_session_id VARCHAR(191),
    native_cursor VARCHAR(191),
    durable_sequence BIGINT NOT NULL,
    exact_resume_eligible BOOLEAN NOT NULL DEFAULT FALSE,
    portable_context_checkpoint_id VARCHAR(191),
    state_payload JSON NOT NULL DEFAULT ('{}'),
    checkpoint_digest VARCHAR(191) NOT NULL,
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (checkpoint_id),
    UNIQUE KEY uq_hrc_tenant_digest (tenant_id, checkpoint_digest)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_native_record_sets (
    record_set_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    session_id VARCHAR(191) NOT NULL,
    task_id VARCHAR(191),
    attempt_id VARCHAR(191),
    binding_id VARCHAR(191),
    harness_id VARCHAR(191) NOT NULL,
    native_namespace VARCHAR(191) NOT NULL,
    record_ids JSON NOT NULL DEFAULT ('[]'),
    records JSON NOT NULL DEFAULT ('[]'),
    record_digest VARCHAR(191) NOT NULL,
    captured_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (record_set_id),
    UNIQUE KEY uq_hnrs_tenant_digest (tenant_id, record_digest)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_capsule_record_batches (
    batch_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    capsule_id VARCHAR(191) NOT NULL,
    ordinal INT NOT NULL,
    record_kind VARCHAR(191) NOT NULL,
    record_schema VARCHAR(191) NOT NULL,
    record_version INT NOT NULL DEFAULT 1,
    payload JSON NOT NULL,
    record_digest VARCHAR(191) NOT NULL,
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (batch_id),
    UNIQUE KEY uq_hcrb_tenant_capsule_ordinal (tenant_id, capsule_id, ordinal)
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS harness_handoff_loss_reports (
    report_id VARCHAR(191) NOT NULL,
    tenant_id VARCHAR(191) NOT NULL,
    operation_id VARCHAR(191),
    capsule_id VARCHAR(191),
    source_harness_id VARCHAR(191) NOT NULL,
    target_harness_id VARCHAR(191) NOT NULL,
    report_digest VARCHAR(191) NOT NULL,
    acknowledgement_required BOOLEAN NOT NULL DEFAULT FALSE,
    acknowledgement_digest VARCHAR(191),
    entries JSON NOT NULL DEFAULT ('[]'),
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    acknowledged_at DATETIME(6),
    PRIMARY KEY (report_id),
    UNIQUE KEY uq_hhlr_tenant_digest (tenant_id, report_digest)
) ENGINE=InnoDB;

SET @exists = (
    SELECT COUNT(*)
    FROM information_schema.COLUMNS
    WHERE TABLE_SCHEMA = DATABASE()
      AND TABLE_NAME = 'harness_model_bindings'
      AND COLUMN_NAME = 'resolved_model'
);
SET @ddl = IF(
    @exists > 0,
    'SELECT 1',
    'ALTER TABLE harness_model_bindings ADD COLUMN resolved_model JSON'
);
PREPARE resolved_model_stmt FROM @ddl;
EXECUTE resolved_model_stmt;
DEALLOCATE PREPARE resolved_model_stmt;
