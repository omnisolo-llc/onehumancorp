const MIGRATION: &str = include_str!("../../migrations/218_harness_middleware.sql");
const RECORD_EXTENSION: &str = include_str!("../../migrations/219_harness_middleware_records.sql");
const MYSQL_RECORD_EXTENSION: &str =
    include_str!("../../db/migrations/219_harness_middleware_records_mysql.sql");

#[test]
fn migration_contains_every_canonical_middleware_table() {
    let lifecycle_tables = [
        "harness_sessions",
        "harness_tasks",
        "harness_turns",
        "harness_runtime_configs",
        "harness_model_runtimes",
        "harness_model_bindings",
        "harness_attempts",
        "harness_bindings",
        "harness_leases",
        "harness_events",
        "harness_event_parents",
        "harness_event_branch_heads",
        "harness_artifacts",
        "harness_workspace_snapshots",
        "harness_native_records",
        "harness_command_inbox",
        "harness_command_outbox",
        "harness_model_runtime_workers",
        "harness_inference_admissions",
        "harness_handoff_operations",
        "harness_capsules",
        "harness_capsule_loss_entries",
    ];
    let record_tables = [
        "harness_descriptors",
        "harness_capability_snapshots",
        "harness_model_descriptors",
        "harness_model_runtime_capabilities",
        "harness_model_capacity_leases",
        "harness_inference_requests",
        "harness_inference_stream_records",
        "harness_inference_final_responses",
        "harness_actors",
        "harness_actor_bindings",
        "harness_operation_fences",
        "harness_messages",
        "harness_content_parts",
        "harness_tool_definition_snapshots",
        "harness_tool_calls",
        "harness_tool_progress",
        "harness_tool_results",
        "harness_interactions",
        "harness_interaction_responses",
        "harness_plans",
        "harness_plan_steps",
        "harness_todos",
        "harness_goals",
        "harness_processes",
        "harness_process_chunks",
        "harness_workspace_descriptors",
        "harness_usage_records",
        "harness_error_records",
        "harness_compactions",
        "harness_portable_context_checkpoints",
        "harness_resume_checkpoints",
        "harness_native_record_sets",
        "harness_capsule_record_batches",
        "harness_handoff_loss_reports",
    ];

    assert_eq!(MIGRATION.matches("CREATE TABLE IF NOT EXISTS").count(), 22);
    assert_eq!(
        RECORD_EXTENSION
            .matches("CREATE TABLE IF NOT EXISTS")
            .count(),
        34
    );
    assert_eq!(
        MYSQL_RECORD_EXTENSION
            .matches("CREATE TABLE IF NOT EXISTS")
            .count(),
        34
    );

    for table in lifecycle_tables {
        assert!(
            MIGRATION.contains(table),
            "missing middleware table: {table}"
        );
    }
    for table in record_tables {
        assert!(
            RECORD_EXTENSION.contains(table),
            "missing typed middleware table: {table}"
        );
        assert!(
            MYSQL_RECORD_EXTENSION.contains(table),
            "missing MySQL typed middleware table: {table}"
        );
    }
}

#[test]
fn migration_enforces_tenant_fences_and_ordering() {
    for marker in [
        "tenant_id TEXT NOT NULL",
        "durable_sequence",
        "delivery_sequence",
        "source_attempt_id",
        "ingest_attempt_id",
        "ON harness_events (tenant_id, session_id, durable_sequence)",
        "ON harness_events (tenant_id, delivery_stream_id, delivery_sequence)",
        "generation = p_generation",
        "fencing_token = p_fencing_token",
        "required replay event must be durable",
        "parent_sequence >= child_sequence",
        "UNIQUE (session_id, idempotency_key)",
        "UNIQUE (tenant_id, request_id)",
        "ENABLE ROW LEVEL SECURITY",
        "FORCE ROW LEVEL SECURITY",
        "current_setting('app.current_tenant'",
        "harness_validate_tenant_references",
        "WITH CHECK (EXISTS",
    ] {
        assert!(
            MIGRATION.contains(marker),
            "missing migration contract: {marker}"
        );
    }
}

#[test]
fn record_extension_preserves_transfer_and_model_runtime_contracts() {
    for marker in [
        "native_extension_namespace",
        "capability_version",
        "request_payload",
        "delivery_sequence",
        "uncertain_effect",
        "policy_decision",
        "dependency_step_ids",
        "source_durable_ranges",
        "exact_resume_eligible",
        "record_digest",
        "acknowledgement_required",
        "ALTER TABLE %I ENABLE ROW LEVEL SECURITY",
        "current_setting(''app.current_tenant''",
    ] {
        assert!(
            RECORD_EXTENSION.contains(marker),
            "missing typed transfer contract: {marker}"
        );
    }
}

#[test]
fn resolved_model_selection_storage_is_nullable_and_dialect_appropriate() {
    assert!(
        RECORD_EXTENSION.contains(
            "ALTER TABLE harness_model_bindings\n    ADD COLUMN IF NOT EXISTS resolved_model JSONB;"
        ),
        "PostgreSQL model bindings must add nullable JSONB storage idempotently"
    );
    for marker in [
        "SET @exists = (",
        "FROM information_schema.COLUMNS",
        "TABLE_SCHEMA = DATABASE()",
        "TABLE_NAME = 'harness_model_bindings'",
        "COLUMN_NAME = 'resolved_model'",
        "SET @ddl = IF(",
        "'ALTER TABLE harness_model_bindings ADD COLUMN resolved_model JSON'",
        "PREPARE resolved_model_stmt FROM @ddl;",
        "EXECUTE resolved_model_stmt;",
        "DEALLOCATE PREPARE resolved_model_stmt;",
    ] {
        assert!(
            MYSQL_RECORD_EXTENSION.contains(marker),
            "missing retry-safe MySQL resolved model migration marker: {marker}"
        );
    }
}
