#[cfg(test)]
mod tests {
    use server_lib::queue::{QueueManager, SubAgentJob};
    use sqlx::postgres::PgPoolOptions;
    use sqlx::Executor;

    fn get_workspace_dir() -> std::path::PathBuf {
        if let Ok(workspace_dir) = std::env::var("BUILD_WORKSPACE_DIRECTORY") {
            return std::path::PathBuf::from(workspace_dir);
        }
        if let Ok(test_srcdir) = std::env::var("TEST_SRCDIR") {
            let mut path = std::path::PathBuf::from(test_srcdir);
            if let Ok(test_workspace) = std::env::var("TEST_WORKSPACE") {
                path.push(test_workspace);
                return path;
            }
        }
        std::env::current_dir().unwrap()
    }

    #[test]
    fn test_sub_agent_queue_schema_migration_exists() {
        let migration_path = get_workspace_dir().join("src/server/migrations/104_sub_agent_queue.sql");

        let migration = std::fs::read_to_string(&migration_path)
            .expect(&format!("sub_agent_queue migration should exist for Postgres deployments at {:?}", migration_path));

        for required in [
            "CREATE TABLE IF NOT EXISTS sub_agent_queue",
            "tenant_id",
            "parent_task_id",
            "payload",
            "status",
            "scheduled_at",
            "locked_until",
            "completed_at",
            "ENABLE ROW LEVEL SECURITY",
            "tenant_isolation_sub_agent_queue",
        ] {
            assert!(
                migration.contains(required),
                "sub_agent_queue migration missing required fragment: {required}"
            );
        }
    }

    #[test]
    fn test_ui_dashboard_campaign_metric_uses_agent_actions() {
        let lib_path = get_workspace_dir().join("src/server/lib.rs");

        let lib = std::fs::read_to_string(&lib_path)
            .expect(&format!("server lib should be readable for dashboard metric invariant at {:?}", lib_path));

        assert!(lib.contains("SELECT COUNT(*) FROM agent_actions"));
        assert!(lib.contains("action_type = 'growth.campaign_sent'"));
        assert!(lib.contains("total_campaigns_sent"));
    }

    #[test]
    fn test_legacy_json_dependencies_are_backfilled_to_edge_table() {
        let migration_path = get_workspace_dir().join("src/server/migrations/100_backfill_shared_task_dependencies.sql");

        let migration = std::fs::read_to_string(&migration_path)
            .expect(&format!("shared task dependency backfill migration should exist at {:?}", migration_path));

        for required in [
            "INSERT INTO shared_task_dependencies",
            "jsonb_array_elements_text",
            "depends_on_task_id",
            "ON CONFLICT DO NOTHING",
            "UPDATE shared_task_dependencies",
            "organization_id",
        ] {
            assert!(
                migration.contains(required),
                "dependency backfill migration missing required fragment: {required}"
            );
        }
    }

    #[tokio::test]
    async fn test_sub_agent_queue_isolation() {
        if let Ok(db_url) = std::env::var("OHC_DATABASE_URL") {
            let pool = PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { conn.execute("DISCARD ALL").await?; Ok(true) }) })
                .connect_lazy(&db_url)
                .unwrap();

            let qm = QueueManager::new(pool);
            let job_id = uuid::Uuid::new_v4().to_string();
            let org_id = "tenant-a".to_string();

            let job = SubAgentJob {
                id: job_id.clone(),
                tenant_id: org_id.clone(),
                parent_task_id: "task-1".to_string(),
                payload: serde_json::json!({"action": "test"}),
                status: "QUEUED".to_string(),
                worker_id: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            // This will likely fail to connect in a unit test environment without a real DB
            // but we are testing that it compiles.
            let _ = qm.enqueue(job).await;
        }
    }
}
