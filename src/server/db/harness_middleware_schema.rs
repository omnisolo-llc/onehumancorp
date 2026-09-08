#[test]
fn harness_middleware_schema_contains_canonical_records_and_fences() {
    let migration = include_str!("../migrations/218_harness_middleware.sql");
    for required in [
        "harness_sessions",
        "harness_tasks",
        "harness_turns",
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
        "harness_model_runtimes",
        "harness_model_runtime_workers",
        "harness_inference_admissions",
        "harness_handoff_operations",
        "harness_capsules",
        "harness_capsule_loss_entries",
        "tenant_id",
        "source_attempt_id",
        "ingest_attempt_id",
        "durable_sequence",
        "delivery_sequence",
        "fencing_token",
        "harness_current_lease",
        "harness_event_writer_fence",
        "required replay event must be durable",
        "harness_one_active_writable_binding",
        "ENABLE ROW LEVEL SECURITY",
        "FORCE ROW LEVEL SECURITY",
        "current_setting('app.current_tenant'",
    ] {
        assert!(
            migration.contains(required),
            "migration is missing required contract marker: {required}"
        );
    }
}

#[test]
fn harness_middleware_schema_has_stale_write_predicates_and_dag_guards() {
    let migration = include_str!("../migrations/218_harness_middleware.sql");
    assert!(migration.contains("generation = p_generation"));
    assert!(migration.contains("fencing_token = p_fencing_token"));
    assert!(migration.contains("parent_event_id"));
    assert!(migration.contains("parent_sequence >= child_sequence"));
    assert!(migration.contains("CHECK (event_id <> parent_event_id)"));
    assert!(migration.contains("ON harness_events (tenant_id, session_id, durable_sequence)"));
    assert!(
        migration.contains("ON harness_events (tenant_id, delivery_stream_id, delivery_sequence)")
    );
    assert!(migration.contains("UNIQUE (session_id, idempotency_key)"));
    assert!(migration.contains("UNIQUE (tenant_id, request_id)"));
}

#[test]
fn mysql_harness_middleware_schema_is_native_and_tenant_aware() {
    let migration = include_str!("migrations/218_harness_middleware_mysql.sql");
    let statements = super::sql_middleware::split_sql_statements(migration);

    assert_eq!(
        statements.len(),
        22,
        "the MySQL companion should contain one statement per canonical table"
    );
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
            migration.contains(&format!("CREATE TABLE IF NOT EXISTS {table}")),
            "missing MySQL middleware table: {table}"
        );
    }

    for postgres_only in [
        "JSONB",
        "TIMESTAMPTZ",
        "::jsonb",
        "ROW LEVEL SECURITY",
        "current_setting(",
        "CREATE UNIQUE INDEX",
        "BIGSERIAL",
    ] {
        assert!(
            !migration.contains(postgres_only),
            "MySQL migration contains PostgreSQL-only syntax: {postgres_only}"
        );
    }

    for mysql_contract in [
        "ENGINE=InnoDB",
        "JSON NOT NULL",
        "AUTO_INCREMENT",
        "tenant_id VARCHAR(191) NOT NULL",
        "FOREIGN KEY (tenant_id, session_id)",
        "UNIQUE KEY uq_harness_bindings_active_writable\n        (session_id, workspace_mutation_scope_id, state, access_mode)",
        "UNIQUE KEY uq_harness_events_durable_sequence",
        "UNIQUE KEY uq_harness_events_delivery_sequence",
        "CREATE TABLE IF NOT EXISTS harness_event_parents (",
        "FOREIGN KEY (session_id, parent_event_id)",
        "PRIMARY KEY (capsule_id, ordinal)",
    ] {
        assert!(
            migration.contains(mysql_contract),
            "missing MySQL middleware contract: {mysql_contract}"
        );
    }
}
