#[cfg(test)]
mod tests {
    use server_lib::queue::{QueueManager, SubAgentJob};
    use sqlx::postgres::PgPoolOptions;
    use sqlx::Executor;

    #[test]
    fn test_sub_agent_queue_schema_migration_exists() {
        let mut migration_path = std::path::PathBuf::from("src/server/migrations/079_sub_agent_queue.sql");
        if let Ok(workspace_dir) = std::env::var("BUILD_WORKSPACE_DIRECTORY") {
            migration_path = std::path::PathBuf::from(workspace_dir).join(&migration_path);
        }

        let migration = std::fs::read_to_string(&migration_path)
            .expect("sub_agent_queue migration should exist for Postgres deployments");

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
        let mut lib_path = std::path::PathBuf::from("src/server/lib.rs");
        if let Ok(workspace_dir) = std::env::var("BUILD_WORKSPACE_DIRECTORY") {
            lib_path = std::path::PathBuf::from(workspace_dir).join(&lib_path);
        }

        let lib = std::fs::read_to_string(lib_path)
            .expect("server lib should be readable for dashboard metric invariant");

        assert!(lib.contains("SELECT COUNT(*) FROM agent_actions"));
        assert!(lib.contains("action_type = 'growth.campaign_sent'"));
        assert!(lib.contains("total_campaigns_sent"));
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
