#[cfg(test)]
use std::sync::Mutex;
use std::sync::LazyLock;
pub static ENV_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

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

        sqlx::query("CREATE TABLE IF NOT EXISTS agent_missions (\n                id TEXT PRIMARY KEY,\n                status TEXT NOT NULL,\n                payload TEXT,\n                tenant_id TEXT,\n                synced_to_cloud BOOLEAN DEFAULT false,\n                sync_error TEXT,\n                last_synced_at TEXT\n            )").execute(&sqlite_pool).await.unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS ohc_job_queue (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                parent_task_id TEXT,
                payload TEXT,
                status TEXT,
                worker_id TEXT,
                scheduled_at TEXT,
                completed_at TEXT,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP
            )").execute(&sqlite_pool).await.unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS department_dead_letters (id TEXT PRIMARY KEY, tenant_id TEXT, event_type TEXT, department TEXT, payload TEXT, error_message TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP)").execute(&sqlite_pool).await.unwrap();

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
                tenant_id TEXT NOT NULL,
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

        sqlx::query("CREATE TABLE IF NOT EXISTS department_dead_letters (id VARCHAR PRIMARY KEY, tenant_id VARCHAR, event_type VARCHAR, department VARCHAR, payload TEXT, error_message TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)").execute(&pg_pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS ohc_job_queue (
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

        sqlx::query("INSERT INTO swarm_truth_embeddings (memory_id, tenant_id, context, escalation_required, sync_status) VALUES (?, ?, ?, 1, 'PENDING')")
            .bind("test_mem_1")
            .bind("test_tenant")
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
            sqlx::query("SELECT payload FROM ohc_job_queue WHERE payload LIKE '%test_mem_1%'")
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
        sqlx::query("INSERT INTO agent_missions (id, status, payload, synced_to_cloud, tenant_id) VALUES ('test_cloud_1', 'CLOUD_ESCALATION', $1, false, 'tenant1')")
            .bind(&pii_payload_1)
            .execute(&sqlite_pool)
            .await
            .unwrap();

        // Test BURSTING sync
        sqlx::query("INSERT INTO agent_missions (id, status, payload, synced_to_cloud, tenant_id) VALUES ('test_burst_1', 'BURSTING', $1, false, 'tenant1')")
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

    sqlx::query("CREATE TABLE IF NOT EXISTS agent_missions (\n                id TEXT PRIMARY KEY,\n                status TEXT NOT NULL,\n                payload TEXT,\n                tenant_id TEXT,\n                synced_to_cloud BOOLEAN DEFAULT false,\n                sync_error TEXT,\n                last_synced_at TEXT\n            )").execute(&sqlite_pool).await.unwrap();

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

    let _lock = ENV_MUTEX.lock().unwrap();
    temp_env::with_vars(
        [
            ("OHC_TELEMETRY_ENABLED", Some("false")),
            ("OHC_STANDALONE_MODE", Some("true")),
        ],
        || {
            // We must block on the async task since temp_env runs synchronously
            tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
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
            });
        }
    );
}

#[tokio::test]
async fn test_hybrid_sync_clears_error_on_success() {
    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query("CREATE TABLE swarm_truth_embeddings (
            memory_id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
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
        "CREATE TABLE IF NOT EXISTS ohc_job_queue (
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

    sqlx::query("INSERT INTO swarm_truth_embeddings (memory_id, tenant_id, context, escalation_required, sync_status, sync_error) VALUES (?, ?, ?, 1, 'PENDING', 'previous error')")
        .bind("test_mem_error_clear")
        .bind("test_tenant")
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

#[tokio::test]
async fn test_hybrid_sync_pos_offline_transactions() {
        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS pos_offline_transactions (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                client_id TEXT NOT NULL,
                amount_cents BIGINT NOT NULL,
                currency TEXT NOT NULL DEFAULT 'USD',
                payload TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'PENDING',
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP
            )"
        ).execute(&sqlite_pool).await.unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS department_dead_letters (id TEXT PRIMARY KEY, tenant_id TEXT, event_type TEXT, department TEXT, payload TEXT, error_message TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP)").execute(&sqlite_pool).await.unwrap();

        let database_url = std::env::var("OHC_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());

        let pg_pool = match tokio::time::timeout(
            std::time::Duration::from_millis(50),
            sqlx::postgres::PgPoolOptions::new().connect(&database_url),
        )
        .await
        {
            Ok(Ok(p)) => p,
            Ok(Err(_)) | Err(_) => return,
        };

        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-pos-sync-test', 'POS Sync Tenant') ON CONFLICT DO NOTHING")
            .execute(&pg_pool).await.unwrap();

        sqlx::query(
            "INSERT INTO pos_offline_transactions (id, tenant_id, client_id, amount_cents, currency, payload, status) VALUES ('pos-tx-1', 'tenant-pos-sync-test', 'client-1', 1500, 'USD', '{\"test\":\"payload\"}', 'PENDING')"
        ).execute(&sqlite_pool).await.unwrap();

        let daemon = super::daemon::HybridSyncDaemon::new(sqlite_pool.clone(), pg_pool.clone());
        daemon.sync_pos_offline_transactions().await.unwrap();

        let synced_row = sqlx::query("SELECT status FROM pos_offline_transactions WHERE id = 'pos-tx-1'")
            .fetch_one(&sqlite_pool)
            .await
            .unwrap();

        let status: String = sqlx::Row::get(&synced_row, "status");
        assert_eq!(status, "SYNCED");

        let pg_row = sqlx::query("SELECT status FROM pos_offline_transactions WHERE id = 'pos-tx-1'")
            .fetch_optional(&pg_pool)
            .await
            .unwrap();
        assert!(pg_row.is_some());

        let job_row = sqlx::query("SELECT id FROM ohc_job_queue WHERE job_type = 'offline_pos_sync' AND payload::jsonb->>'pos_transaction_id' = 'pos-tx-1'")
            .fetch_optional(&pg_pool)
            .await
            .unwrap();
        assert!(job_row.is_some());
    }
    #[tokio::test]
    async fn test_hybrid_sync_pos_offline_transactions_chaos_degradation() {
        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS pos_offline_transactions (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, client_id TEXT NOT NULL, amount_cents BIGINT NOT NULL, currency TEXT NOT NULL DEFAULT 'USD', payload TEXT NOT NULL, status TEXT NOT NULL DEFAULT 'PENDING', device_signature TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP, updated_at TEXT DEFAULT CURRENT_TIMESTAMP)").execute(&sqlite_pool).await.unwrap();

        sqlx::query("INSERT INTO pos_offline_transactions (id, tenant_id, client_id, amount_cents, currency, payload, status) VALUES ('pos-tx-chaos-1', 'tenant-pos-chaos-test', 'client-1', 1500, 'USD', '{\"test\":\"payload\"}', 'PENDING')").execute(&sqlite_pool).await.unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS department_dead_letters (id TEXT PRIMARY KEY, tenant_id TEXT, event_type TEXT, department TEXT, payload TEXT, error_message TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP)").execute(&sqlite_pool).await.unwrap();

        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());
        let pg_pool = match tokio::time::timeout(std::time::Duration::from_millis(50), sqlx::postgres::PgPoolOptions::new().connect(&database_url)).await { Ok(Ok(p)) => p, _ => return, };

        let _ = sqlx::query("DROP TABLE IF EXISTS pos_offline_transactions CASCADE").execute(&pg_pool).await;
        let daemon = super::daemon::HybridSyncDaemon::new(sqlite_pool.clone(), pg_pool.clone());
        let _ = daemon.sync_pos_offline_transactions().await;

        let synced_row = sqlx::query("SELECT status FROM pos_offline_transactions WHERE id = 'pos-tx-chaos-1'").fetch_one(&sqlite_pool).await.unwrap();
        let status: String = sqlx::Row::get(&synced_row, "status");
        assert_eq!(status, "PENDING");
    }


    #[tokio::test]
    async fn test_prune_stuck_missions_and_queue() {
        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS agent_missions (
                id TEXT PRIMARY KEY,
                status TEXT NOT NULL,
                payload TEXT,
                tenant_id TEXT,
                synced_to_cloud BOOLEAN DEFAULT false,
                sync_error TEXT,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
                last_synced_at TEXT
            )").execute(&sqlite_pool).await.unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS department_dead_letters (id TEXT PRIMARY KEY, tenant_id TEXT, event_type TEXT, department TEXT, payload TEXT, error_message TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP)").execute(&sqlite_pool).await.unwrap();

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
            "CREATE TABLE IF NOT EXISTS agent_missions (
                id VARCHAR PRIMARY KEY,
                status VARCHAR NOT NULL,
                payload JSONB,
                tenant_id VARCHAR,
                sync_error TEXT,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                last_synced_at TIMESTAMP
            )",
        )
        .execute(&pg_pool)
        .await
        .unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS department_dead_letters (id VARCHAR PRIMARY KEY, tenant_id VARCHAR, event_type VARCHAR, department VARCHAR, payload TEXT, error_message TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)").execute(&pg_pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS ohc_job_queue (
                id VARCHAR PRIMARY KEY,
                tenant_id VARCHAR NOT NULL,
                parent_task_id VARCHAR,
                payload JSONB,
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

        // Insert stuck tasks
        sqlx::query("INSERT INTO agent_missions (id, status, last_synced_at, tenant_id, payload) VALUES ('stuck_mission_sqlite', 'IN_PROGRESS', datetime('now', '-2 hour'), 'tenant1', '{}')")
            .execute(&sqlite_pool).await.unwrap();

        sqlx::query("INSERT INTO agent_missions (id, status, last_synced_at, tenant_id, payload) VALUES ('stuck_mission_pg', 'RUNNING', NOW() - INTERVAL '2 hours', 'tenant1', '{}')")
            .execute(&pg_pool).await.unwrap();

        sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, status, updated_at, payload) VALUES ('stuck_queue_sqlite', 'tenant1', 'RUNNING', datetime('now', '-2 hour'), '{}')")
            .execute(&sqlite_pool).await.unwrap();

        sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, status, updated_at, payload) VALUES ('stuck_queue_pg', 'tenant1', 'RUNNING', NOW() - INTERVAL '2 hours', '{}')")
            .execute(&pg_pool).await.unwrap();

        let daemon = super::daemon::HybridSyncDaemon::new(sqlite_pool.clone(), pg_pool.clone());
        daemon.prune_stuck_agent_missions().await.unwrap();
        daemon.prune_stuck_ohc_job_queue().await.unwrap();

        // Verify SQLite mission is failed
        let row_sqlite = sqlx::query("SELECT status FROM agent_missions WHERE id = 'stuck_mission_sqlite'")
            .fetch_optional(&sqlite_pool).await.unwrap();
        use sqlx::Row;
        assert_eq!(row_sqlite.unwrap().get::<String, _>("status"), "FAILED");

        // Verify PG mission is failed
        let row_pg = sqlx::query("SELECT status FROM agent_missions WHERE id = 'stuck_mission_pg'")
            .fetch_optional(&pg_pool).await.unwrap();
        assert_eq!(row_pg.unwrap().get::<String, _>("status"), "FAILED");

        // Verify SQLite queue is failed
        let row_queue_sqlite = sqlx::query("SELECT status FROM ohc_job_queue WHERE id = 'stuck_queue_sqlite'")
            .fetch_one(&sqlite_pool).await.unwrap();
        assert_eq!(row_queue_sqlite.get::<String, _>("status"), "FAILED");

        // Verify PG queue is failed
        let row_queue = sqlx::query("SELECT status FROM ohc_job_queue WHERE id = 'stuck_queue_pg'")
            .fetch_one(&pg_pool).await.unwrap();
        assert_eq!(row_queue.get::<String, _>("status"), "FAILED");

        // Verify dead letters were created for missions
        let dl_sqlite_mission: (i64,) = sqlx::query_as("SELECT count(*) FROM department_dead_letters WHERE event_type = 'mission_stuck'")
            .fetch_one(&sqlite_pool).await.unwrap();
        assert_eq!(dl_sqlite_mission.0, 1);

        let dl_pg_mission: (i64,) = sqlx::query_as("SELECT count(*) FROM department_dead_letters WHERE event_type = 'mission_stuck'")
            .fetch_one(&pg_pool).await.unwrap();
        assert_eq!(dl_pg_mission.0, 1);

        // Verify dead letters were created for running jobs
        let dl_sqlite_job: (i64,) = sqlx::query_as("SELECT count(*) FROM department_dead_letters WHERE event_type = 'job_stuck'")
            .fetch_one(&sqlite_pool).await.unwrap();
        assert_eq!(dl_sqlite_job.0, 1);

        let dl_pg_job: (i64,) = sqlx::query_as("SELECT count(*) FROM department_dead_letters WHERE event_type = 'job_stuck'")
            .fetch_one(&pg_pool).await.unwrap();
        assert_eq!(dl_pg_job.0, 1);
    }

    #[tokio::test]
    async fn test_prune_stuck_queued_items() {
        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS ohc_job_queue (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                parent_task_id TEXT,
                payload TEXT,
                status TEXT,
                worker_id TEXT,
                scheduled_at TEXT,
                completed_at TEXT,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP
            )").execute(&sqlite_pool).await.unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS department_dead_letters (id TEXT PRIMARY KEY, tenant_id TEXT, event_type TEXT, department TEXT, payload TEXT, error_message TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP)").execute(&sqlite_pool).await.unwrap();

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

        sqlx::query("CREATE TABLE IF NOT EXISTS department_dead_letters (id VARCHAR PRIMARY KEY, tenant_id VARCHAR, event_type VARCHAR, department VARCHAR, payload TEXT, error_message TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)").execute(&pg_pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS ohc_job_queue (
                id VARCHAR PRIMARY KEY,
                tenant_id VARCHAR NOT NULL,
                parent_task_id VARCHAR,
                payload JSONB,
                status VARCHAR,
                worker_id VARCHAR,
                scheduled_at TIMESTAMP,
                completed_at TIMESTAMP,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
        )
        .execute(&pg_pool)
        .await
        .unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS department_dead_letters (id VARCHAR PRIMARY KEY, tenant_id VARCHAR, event_type VARCHAR, department VARCHAR, payload TEXT, error_message TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)").execute(&pg_pool).await.unwrap();

        // Insert stuck queued tasks
        sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, status, created_at, updated_at, payload) VALUES ('stuck_queued_sqlite', 'tenant1', 'QUEUED', datetime('now', '-25 hour'), datetime('now', '-25 hour'), '{}')")
            .execute(&sqlite_pool).await.unwrap();

        sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, status, created_at, updated_at, payload) VALUES ('stuck_queued_pg', 'tenant1', 'QUEUED', NOW() - INTERVAL '25 hours', NOW() - INTERVAL '25 hours', '{}')")
            .execute(&pg_pool).await.unwrap();

        // Insert stuck running tasks
        sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, status, updated_at, payload) VALUES ('stuck_running_sqlite', 'tenant1', 'RUNNING', datetime('now', '-2 hour'), '{}')")
            .execute(&sqlite_pool).await.unwrap();

        sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, status, updated_at, payload) VALUES ('stuck_running_pg', 'tenant1', 'RUNNING', NOW() - INTERVAL '2 hours', '{}')")
            .execute(&pg_pool).await.unwrap();

        let daemon = super::daemon::HybridSyncDaemon::new(sqlite_pool.clone(), pg_pool.clone());
        daemon.prune_stuck_ohc_job_queue().await.unwrap();

        // Verify SQLite queue is deleted
        let row_queue_sqlite = sqlx::query("SELECT status FROM ohc_job_queue WHERE id = \'stuck_queued_sqlite\'").fetch_optional(&sqlite_pool).await.unwrap();
        assert!(row_queue_sqlite.is_none());

        // Verify PG queue is deleted
        let row_queue = sqlx::query("SELECT status FROM ohc_job_queue WHERE id = \'stuck_queued_pg\'").fetch_optional(&pg_pool).await.unwrap();
        assert!(row_queue.is_none());

        // Verify SQLite running is failed
        use sqlx::Row;
        let row_running_sqlite = sqlx::query("SELECT status FROM ohc_job_queue WHERE id = \'stuck_running_sqlite\'").fetch_one(&sqlite_pool).await.unwrap();
        assert_eq!(row_running_sqlite.get::<String, _>("status"), "FAILED");

        // Verify PG running is failed
        let row_running = sqlx::query("SELECT status FROM ohc_job_queue WHERE id = \'stuck_running_pg\'").fetch_one(&pg_pool).await.unwrap();
        assert_eq!(row_running.get::<String, _>("status"), "FAILED");

        // Verify dead letters were created for running jobs
        let dl_sqlite: (i64,) = sqlx::query_as("SELECT count(*) FROM department_dead_letters WHERE id = 'stuck_running_sqlite'")
            .fetch_one(&sqlite_pool).await.unwrap();
        assert_eq!(dl_sqlite.0, 1);

        let dl_pg: (i64,) = sqlx::query_as("SELECT count(*) FROM department_dead_letters WHERE id = 'stuck_running_pg'")
            .fetch_one(&pg_pool).await.unwrap();
        assert_eq!(dl_pg.0, 1);
    }

    #[tokio::test]
    async fn test_agent_mission_failure_categorization() {
        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS agent_missions (
                id TEXT PRIMARY KEY,
                status TEXT NOT NULL,
                payload TEXT,
                tenant_id TEXT,
                synced_to_cloud BOOLEAN DEFAULT false,
                sync_error TEXT,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
                last_synced_at TEXT
            )").execute(&sqlite_pool).await.unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS department_dead_letters (id TEXT PRIMARY KEY, tenant_id TEXT, event_type TEXT, department TEXT, payload TEXT, error_message TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP)").execute(&sqlite_pool).await.unwrap();

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
            "CREATE TABLE IF NOT EXISTS agent_missions (
                id VARCHAR PRIMARY KEY,
                status VARCHAR NOT NULL,
                payload TEXT,
                tenant_id VARCHAR,
                sync_error TEXT,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                last_synced_at TIMESTAMP
            )",
        )
        .execute(&pg_pool)
        .await
        .unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS department_dead_letters (id VARCHAR PRIMARY KEY, tenant_id VARCHAR, event_type VARCHAR, department VARCHAR, payload TEXT, error_message TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)").execute(&pg_pool).await.unwrap();

        // Insert stuck mission
        sqlx::query("INSERT INTO agent_missions (id, status, last_synced_at, tenant_id, payload) VALUES ('stuck_mission_sqlite_cat', 'IN_PROGRESS', datetime('now', '-2 hour'), 'tenant1', '{}')")
            .execute(&sqlite_pool).await.unwrap();

        sqlx::query("INSERT INTO agent_missions (id, status, last_synced_at, tenant_id, payload) VALUES ('stuck_mission_pg_cat', 'RUNNING', NOW() - INTERVAL '2 hours', 'tenant1', '{}')")
            .execute(&pg_pool).await.unwrap();

        let daemon = super::daemon::HybridSyncDaemon::new(sqlite_pool.clone(), pg_pool.clone());
        daemon.prune_stuck_agent_missions().await.unwrap();

        // Verify SQLite mission failure category
        let dl_sqlite: (i64,) = sqlx::query_as("SELECT count(*) FROM department_dead_letters WHERE id = 'stuck_mission_sqlite_cat'")
            .fetch_one(&sqlite_pool).await.unwrap();
        assert_eq!(dl_sqlite.0, 1);

        // Verify PG mission failure category
        let dl_pg: (i64,) = sqlx::query_as("SELECT count(*) FROM department_dead_letters WHERE id = 'stuck_mission_pg_cat'")
            .fetch_one(&pg_pool).await.unwrap();
        assert_eq!(dl_pg.0, 1);
    }
