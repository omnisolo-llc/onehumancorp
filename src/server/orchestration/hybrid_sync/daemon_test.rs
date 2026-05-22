#[cfg(test)]
mod tests {
    use super::super::daemon::HybridSyncDaemon;
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::postgres::PgPoolOptions;
    use serde_json::json;

    #[tokio::test]
    async fn test_hybrid_sync_daemon_redaction() {
        let sqlite_pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());

        let pg_pool = PgPoolOptions::new()
            .connect(&database_url)
            .await.expect("PostgreSQL must be running for daemon tests");

        sqlx::query(
            "CREATE TABLE swarm_truth_embeddings (
                memory_id TEXT PRIMARY KEY,
                context TEXT,
                embedding TEXT,
                escalation_required INTEGER DEFAULT 0,
                sync_status TEXT DEFAULT 'PENDING'
            )"
        )
        .execute(&sqlite_pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS sub_agent_queue (
                id VARCHAR PRIMARY KEY,
                tenant_id VARCHAR NOT NULL,
                parent_task_id VARCHAR,
                payload TEXT,
                status VARCHAR,
                worker_id VARCHAR,
                scheduled_at TIMESTAMP,
                completed_at TIMESTAMP,
                created_at TIMESTAMP,
                updated_at TIMESTAMP
            )"
        ).execute(&pg_pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS agent_missions (
                id VARCHAR PRIMARY KEY,
                status VARCHAR NOT NULL,
                payload TEXT,
                tenant_id VARCHAR
            )"
        ).execute(&pg_pool).await.unwrap();

        let raw_context = json!({
            "email": "test@example.com",
            "safe_data": "hello world"
        }).to_string();

        sqlx::query("INSERT INTO swarm_truth_embeddings (memory_id, context, escalation_required, sync_status) VALUES (?, ?, 1, 'PENDING')")
            .bind("test_mem_1")
            .bind(&raw_context)
            .execute(&sqlite_pool)
            .await
            .unwrap();

        let daemon = HybridSyncDaemon::new(sqlite_pool.clone(), pg_pool.clone());
        daemon.sync_step().await.unwrap();

        let row = sqlx::query("SELECT sync_status FROM swarm_truth_embeddings WHERE memory_id = 'test_mem_1'")
            .fetch_one(&sqlite_pool)
            .await
            .unwrap();
        use sqlx::Row;
        let status: String = row.get("sync_status");
        assert_eq!(status, "SYNCED");

        // Let's also check the pg queue redaction.
        let queue_row = sqlx::query("SELECT payload FROM sub_agent_queue WHERE payload LIKE '%test_mem_1%'")
            .fetch_one(&pg_pool)
            .await
            .unwrap();
        let payload_str: String = queue_row.get("payload");
        assert!(payload_str.contains("[REDACTED]"));
        assert!(!payload_str.contains("test@example.com"));
        assert!(payload_str.contains("safe_data"));

        // Let's also check the agent_missions table redaction.
        let mission_row = sqlx::query("SELECT payload FROM agent_missions WHERE payload LIKE '%test_mem_1%'")
            .fetch_one(&pg_pool)
            .await
            .unwrap();
        let mission_payload_str: String = mission_row.get("payload");
        assert!(mission_payload_str.contains("[REDACTED]"));
        assert!(!mission_payload_str.contains("test@example.com"));
        assert!(mission_payload_str.contains("safe_data"));
    }
}
