use super::daemon::OmniContextSyncDaemon;
use crate::db::{DB, DbStore};
use crate::orchestration::queue::{TaskQueue, Job};
use sqlx::{sqlite::SqlitePoolOptions, postgres::PgPoolOptions};
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;

struct MockTaskQueue {
    enqueued: Arc<Mutex<Vec<Job>>>,
}

#[async_trait]
impl TaskQueue for MockTaskQueue {
    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let mut q = self.enqueued.lock().await;
        q.push(job);
        Ok(())
    }

    async fn dequeue(&self, _roles: Vec<String>) -> Result<Option<Job>, String> {
        Ok(None)
    }
    async fn complete(&self, _job_id: &str) -> Result<(), String> {
        Ok(())
    }
    async fn fail(&self, _job_id: &str, _reason: &str) -> Result<(), String> {
        Ok(())
    }
}

#[tokio::test]
async fn test_omni_context_sync_daemon_success() {
    let sqlite_pool = SqlitePoolOptions::new().connect_lazy("sqlite::memory:").unwrap();
    let pg_pool = PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
        .connect_lazy("postgres://dummy").unwrap();

    let db = Arc::new(DB {
        pool: pg_pool.clone(),
        store: DbStore::Sqlite(sqlite_pool.clone()),
    });

    sqlx::query(
        "CREATE TABLE agent_missions (
            id TEXT PRIMARY KEY,
            organization_id TEXT,
            status TEXT,
            payload TEXT,
            synced_to_cloud BOOLEAN DEFAULT FALSE,
            sync_error TEXT,
            last_synced_at DATETIME
        )"
    ).execute(&sqlite_pool).await.unwrap();

    let raw_payload = serde_json::json!({
        "email": "test@test.com",
        "data": "safe",
        "nested": {
            "password": "secret_password"
        }
    }).to_string();

    sqlx::query(
        "INSERT INTO agent_missions (id, organization_id, status, payload, synced_to_cloud)
         VALUES ('m1', 'org1', 'CLOUD_ESCALATION', ?, FALSE)"
    )
    .bind(&raw_payload)
    .execute(&sqlite_pool).await.unwrap();

    let mock_queue = Arc::new(MockTaskQueue {
        enqueued: Arc::new(Mutex::new(Vec::new())),
    });

    let daemon = OmniContextSyncDaemon::new(db, mock_queue.clone());
    daemon.run().await.unwrap();

    // Verify task enqueued and PII redacted
    let enqueued = mock_queue.enqueued.lock().await;
    assert_eq!(enqueued.len(), 1);
    let job = &enqueued[0];

    let sanitized_payload: serde_json::Value = serde_json::from_str(&job.payload).unwrap();

    assert_eq!(sanitized_payload["email"], "[REDACTED]");
    assert_eq!(sanitized_payload["data"], "safe");
    assert_eq!(sanitized_payload["nested"]["password"], "[REDACTED]");

    // Verify status updated in SQLite
    let row: (bool,) = sqlx::query_as("SELECT synced_to_cloud FROM agent_missions WHERE id = 'm1'")
        .fetch_one(&sqlite_pool).await.unwrap();

    assert!(row.0, "synced_to_cloud should be true");
}
