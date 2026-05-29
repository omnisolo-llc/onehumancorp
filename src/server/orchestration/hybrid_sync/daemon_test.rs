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
            .await;

        let pg_pool = match pg_pool {
            Ok(p) => p,
            Err(_) => {
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

#[tokio::test]
async fn test_hybrid_sync_daemon_telemetry_opt_out() {
    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());

    let pg_pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&database_url)
        .await;

    let pg_pool = match pg_pool {
        Ok(p) => p,
        Err(_) => return,
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
        )"
    ).execute(&sqlite_pool).await.unwrap();

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
    let _old_standalone = std::env::var("STANDALONE_MODE");

    unsafe {
        std::env::set_var("OHC_TELEMETRY_ENABLED", "false");
        std::env::set_var("STANDALONE_MODE", "true");
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
            std::env::set_var("STANDALONE_MODE", val);
        } else {
            std::env::remove_var("STANDALONE_MODE");
        }
    }
}

#[tokio::test]
async fn test_hybrid_sync_missions_daemon_redaction() {
    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());

    let pg_pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&database_url)
        .await;

    let pg_pool = match pg_pool {
        Ok(p) => p,
        Err(_) => return,
    };

    sqlx::query(
        "CREATE TABLE agent_missions (
            id TEXT PRIMARY KEY,
            status TEXT NOT NULL,
            payload TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            tenant_id TEXT NOT NULL DEFAULT 'system',
            cloud_mission_id TEXT,
            sync_error TEXT,
            last_synced_at TIMESTAMP,
            synced_to_cloud BOOLEAN DEFAULT 0,
            _sync_status TEXT DEFAULT 'pending',
            version INTEGER DEFAULT 1,
            mission_log TEXT
        )"
    )
    .execute(&sqlite_pool)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS agent_missions (
            id VARCHAR PRIMARY KEY,
            status VARCHAR NOT NULL,
            payload TEXT,
            tenant_id VARCHAR
        )"
    ).execute(&pg_pool).await.unwrap();

    let raw_payload = serde_json::json!({
        "email": "mission@example.com",
        "safe_data": "start the agent"
    }).to_string();

    sqlx::query("INSERT INTO agent_missions (id, status, payload, synced_to_cloud, _sync_status) VALUES (?, ?, ?, 0, 'pending')")
        .bind("mission_test_1")
        .bind("PENDING")
        .bind(&raw_payload)
        .execute(&sqlite_pool)
        .await
        .unwrap();

    let daemon = super::daemon::HybridSyncDaemon::new(sqlite_pool.clone(), pg_pool.clone());
    daemon.sync_missions_step().await.unwrap();

    // Verify SQLite status is updated
    let row = sqlx::query("SELECT synced_to_cloud, _sync_status FROM agent_missions WHERE id = 'mission_test_1'")
        .fetch_one(&sqlite_pool)
        .await
        .unwrap();
    use sqlx::Row;
    let synced: bool = row.get("synced_to_cloud");
    let status: String = row.get("_sync_status");
    assert!(synced);
    assert_eq!(status, "synced");

    // Verify PG received it and it's redacted
    let pg_row = sqlx::query("SELECT payload FROM agent_missions WHERE id = 'mission_test_1'")
        .fetch_one(&pg_pool)
        .await
        .unwrap();
    let pg_payload: String = pg_row.get("payload");
    assert!(pg_payload.contains("[REDACTED]"));
    assert!(!pg_payload.contains("mission@example.com"));
    assert!(pg_payload.contains("safe_data"));
}
