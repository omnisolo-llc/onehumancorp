const MIGRATION: &str = include_str!("../../migrations/218_harness_middleware.sql");

#[test]
fn migration_contains_every_canonical_middleware_table() {
    for table in [
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
    ] {
        assert!(
            MIGRATION.contains(table),
            "missing middleware table: {table}"
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
