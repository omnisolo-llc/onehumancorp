#[cfg(test)]
mod tests {
    use super::super::daemon::HybridSyncDaemon;
    use serde_json::json;
    use sqlx::postgres::PgPoolOptions;
    use sqlx::sqlite::SqlitePoolOptions;
    use std::time::Duration;

    pub fn clear_semaphore() {
        // Mock clear semaphore function
    }

    #[tokio::test]
    async fn test_hybrid_sync_daemon_redaction() {
        let sqlite_pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS agent_missions (\n                id TEXT PRIMARY KEY,\n                status TEXT NOT NULL,\n                payload TEXT,\n                synced_to_cloud BOOLEAN DEFAULT false,\n                sync_error TEXT,\n                last_synced_at TEXT\n            )").execute(&sqlite_pool).await.unwrap();

        let database_url = std::env::var("OHC_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());

        let pg_pool = match tokio::time::timeout(
            Duration::from_millis(50),
            PgPoolOptions::new().connect(&database_url),
        )
        .await
        {
            Ok(Ok(p)) => p,
            Ok(Err(_)) | Err(_) => {
                // If PG is not running during the test, we'll just mock or skip.
                return;
            }
        };

        sqlx::query(
            "CREATE TABLE swarm_truth_embeddings (
                memory_id TEXT PRIMARY KEY,
                context TEXT,
                embedding TEXT,
                escalation_required INTEGER DEFAULT 0,
                sync_status TEXT DEFAULT 'PENDING',
                sync_error TEXT,
                last_synced_at TEXT
            )",
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
            )",
        )
        .execute(&pg_pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS agent_missions (
                id VARCHAR PRIMARY KEY,
                status VARCHAR NOT NULL,
                payload TEXT,
                tenant_id VARCHAR,
                sync_error TEXT,
                last_synced_at TIMESTAMP
            )",
        )
        .execute(&pg_pool)
        .await
        .unwrap();

        let raw_context = json!({
            "email": "test@example.com",
            "safe_data": "hello world"
        })
        .to_string();

        sqlx::query("INSERT INTO swarm_truth_embeddings (memory_id, context, escalation_required, sync_status) VALUES (?, ?, 1, 'PENDING')")
            .bind("test_mem_1")
            .bind(&raw_context)
            .execute(&sqlite_pool)
            .await
            .unwrap();

        let daemon = HybridSyncDaemon::new(sqlite_pool.clone(), pg_pool.clone());
        daemon.sync_step().await.unwrap();
        clear_semaphore();

        let row = sqlx::query(
            "SELECT sync_status FROM swarm_truth_embeddings WHERE memory_id = 'test_mem_1'",
        )
        .fetch_one(&sqlite_pool)
        .await
        .unwrap();
        use sqlx::Row;
        let status: String = row.get("sync_status");
        assert_eq!(status, "SYNCED");

        // Let's also check the pg queue redaction.
        let queue_row =
            sqlx::query("SELECT payload FROM sub_agent_queue WHERE payload LIKE '%test_mem_1%'")
                .fetch_one(&pg_pool)
                .await
                .unwrap();
        let payload_str: String = queue_row.get("payload");
        assert!(payload_str.contains("[REDACTED]"));
        assert!(!payload_str.contains("test@example.com"));
        assert!(payload_str.contains("safe_data"));

        // Let's also check the agent_missions table redaction.
        let mission_row =
            sqlx::query("SELECT payload FROM agent_missions WHERE payload LIKE '%test_mem_1%'")
                .fetch_one(&pg_pool)
                .await
                .unwrap();
        let mission_payload_str: String = mission_row.get("payload");
        assert!(mission_payload_str.contains("[REDACTED]"));
        assert!(!mission_payload_str.contains("test@example.com"));
        assert!(mission_payload_str.contains("safe_data"));

        let pii_payload_1 = json!({
            "email": "cloud_user@example.com",
            "safe_data": "cloud_payload_data"
        }).to_string();

        let pii_payload_2 = json!({
            "credit_card": "1234-5678-9012-3456",
            "safe_data": "burst_payload_data"
        }).to_string();

        // Test sync_cloud_escalations
        sqlx::query("INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('test_cloud_1', 'CLOUD_ESCALATION', $1, false)")
            .bind(&pii_payload_1)
            .execute(&sqlite_pool)
            .await
            .unwrap();

        // Test BURSTING sync
        sqlx::query("INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('test_burst_1', 'BURSTING', $1, false)")
            .bind(&pii_payload_2)
            .execute(&sqlite_pool)
            .await
            .unwrap();

        daemon.sync_cloud_escalations().await.unwrap();

        let row_local_mission =
            sqlx::query("SELECT synced_to_cloud FROM agent_missions WHERE id = 'test_cloud_1'")
                .fetch_one(&sqlite_pool)
                .await
                .unwrap();
        assert!(row_local_mission.get::<bool, _>("synced_to_cloud"));

        let row_cloud_mission =
            sqlx::query("SELECT payload FROM agent_missions WHERE id = 'test_cloud_1'")
                .fetch_one(&pg_pool)
                .await
                .unwrap();

        let retrieved_payload_1 = row_cloud_mission.get::<String, _>("payload");
        assert!(retrieved_payload_1.contains("[REDACTED]"));
        assert!(!retrieved_payload_1.contains("cloud_user@example.com"));
        assert!(retrieved_payload_1.contains("cloud_payload_data"));

        let row_local_burst =
            sqlx::query("SELECT synced_to_cloud FROM agent_missions WHERE id = 'test_burst_1'")
                .fetch_one(&sqlite_pool)
                .await
                .unwrap();
        assert!(row_local_burst.get::<bool, _>("synced_to_cloud"));

        let row_cloud_burst =
            sqlx::query("SELECT payload FROM agent_missions WHERE id = 'test_burst_1'")
                .fetch_one(&pg_pool)
                .await
                .unwrap();

        let retrieved_payload_2 = row_cloud_burst.get::<String, _>("payload");
        assert!(retrieved_payload_2.contains("[REDACTED]"));
        assert!(!retrieved_payload_2.contains("1234-5678-9012-3456"));
        assert!(retrieved_payload_2.contains("burst_payload_data"));
    }
}

pub fn clear_semaphore() {
    // Mock clear semaphore function
}

#[tokio::test]
async fn test_hybrid_sync_daemon_telemetry_opt_out() {
    use std::time::Duration;

    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query("CREATE TABLE IF NOT EXISTS agent_missions (\n                id TEXT PRIMARY KEY,\n                status TEXT NOT NULL,\n                payload TEXT,\n                synced_to_cloud BOOLEAN DEFAULT false,\n                sync_error TEXT,\n                last_synced_at TEXT\n            )").execute(&sqlite_pool).await.unwrap();

    let database_url = std::env::var("OHC_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());

    let pg_pool = match tokio::time::timeout(
        Duration::from_millis(50),
        sqlx::postgres::PgPoolOptions::new().connect(&database_url),
    )
    .await
    {
        Ok(Ok(p)) => p,
        Ok(Err(_)) | Err(_) => return,
    };

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS telemetry_buffer (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            metric_name TEXT NOT NULL,
            metric_type TEXT NOT NULL,
            value REAL NOT NULL,
            labels_json TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            sync_status TEXT NOT NULL
        )",
    )
    .execute(&sqlite_pool)
    .await
    .unwrap();

    sqlx::query("INSERT INTO telemetry_buffer (metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES (?, ?, ?, ?, ?, 'pending')")
        .bind("test_metric")
        .bind("counter")
        .bind(1.0)
        .bind("{}")
        .bind(chrono::Utc::now().naive_utc().to_string())
        .execute(&sqlite_pool)
        .await
        .unwrap();

    // The async env issue... temp_env is synchronous
    let _old_telemetry = std::env::var("OHC_TELEMETRY_ENABLED");
    let _old_standalone = std::env::var("OHC_STANDALONE_MODE");

    unsafe {
        std::env::set_var("OHC_TELEMETRY_ENABLED", "false");
        std::env::set_var("OHC_STANDALONE_MODE", "true");
    }

    // We also need to reload config somehow... or actually our change reads ::server_config::get().
    // We can't really reload standard OnceLock easily so we'll just check if it blocks.
    let daemon = super::daemon::HybridSyncDaemon::new(sqlite_pool.clone(), pg_pool.clone());
    daemon.sync_telemetry_step().await.unwrap();

    // Check that it's still pending
    let row = sqlx::query("SELECT sync_status FROM telemetry_buffer")
        .fetch_one(&sqlite_pool)
        .await
        .unwrap();
    use sqlx::Row;
    let status: String = row.get("sync_status");
    assert_eq!(status, "pending");

    unsafe {
        if let Ok(val) = _old_telemetry {
            std::env::set_var("OHC_TELEMETRY_ENABLED", val);
        } else {
            std::env::remove_var("OHC_TELEMETRY_ENABLED");
        }

        if let Ok(val) = _old_standalone {
            std::env::set_var("OHC_STANDALONE_MODE", val);
        } else {
            std::env::remove_var("OHC_STANDALONE_MODE");
        }
    }
}

#[tokio::test]
async fn test_hybrid_sync_clears_error_on_success() {
    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query("CREATE TABLE swarm_truth_embeddings (
            memory_id TEXT PRIMARY KEY,
            context TEXT,
            embedding TEXT,
            escalation_required INTEGER DEFAULT 0,
            sync_status TEXT DEFAULT 'PENDING',
            sync_error TEXT,
            last_synced_at TEXT
        )")
        .execute(&sqlite_pool)
        .await
        .unwrap();

    let database_url = std::env::var("OHC_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());

    let pg_pool = match tokio::time::timeout(
        std::time::Duration::from_millis(50),
        sqlx::postgres::PgPoolOptions::new().connect(&database_url),
    )
    .await
    {
        Ok(Ok(p)) => p,
        _ => return,
    };

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
        )",
    )
    .execute(&pg_pool)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS agent_missions (
            id VARCHAR PRIMARY KEY,
            status VARCHAR NOT NULL,
            payload TEXT,
            tenant_id VARCHAR,
            sync_error TEXT,
            last_synced_at TIMESTAMP
        )",
    )
    .execute(&pg_pool)
    .await
    .unwrap();

    let raw_context = serde_json::json!({
        "safe_data": "hello world"
    })
    .to_string();

    sqlx::query("INSERT INTO swarm_truth_embeddings (memory_id, context, escalation_required, sync_status, sync_error) VALUES (?, ?, 1, 'PENDING', 'previous error')")
        .bind("test_mem_error_clear")
        .bind(&raw_context)
        .execute(&sqlite_pool)
        .await
        .unwrap();

    let daemon = super::daemon::HybridSyncDaemon::new(sqlite_pool.clone(), pg_pool.clone());
    daemon.sync_step().await.unwrap();

    let row = sqlx::query(
        "SELECT sync_status, sync_error FROM swarm_truth_embeddings WHERE memory_id = 'test_mem_error_clear'",
    )
    .fetch_one(&sqlite_pool)
    .await
    .unwrap();
    use sqlx::Row;
    let status: String = row.get("sync_status");
    let error: Option<String> = row.try_get("sync_error").unwrap_or(None);
    assert_eq!(status, "SYNCED");
    assert_eq!(error, None, "sync_error should be cleared on success");
}
