use std::sync::Arc;
use tokio::sync::Mutex;
use crate::db::{DB, DbStore};
use chrono::Utc;
use serde_json::Value;

pub struct SubAgentWorker {
    db: Arc<DB>,
    sqlite_mu: Arc<Mutex<()>>,
}

#[derive(Debug)]
pub struct SubAgentJob {
    pub id: String,
    pub tenant_id: String,
    pub parent_task_id: Option<String>,
    pub payload: Option<String>,
    pub status: String,
}

impl SubAgentWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            sqlite_mu: Arc::new(Mutex::new(())),
        }
    }

    pub async fn poll_and_claim(&self, worker_id: &str) -> Result<Option<SubAgentJob>, String> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                let row = sqlx::query(
                    r#"
                    UPDATE sub_agent_queue
                    SET status = 'RUNNING', worker_id = $1, updated_at = $2
                    WHERE id = (
                        SELECT id FROM sub_agent_queue
                        WHERE status = 'QUEUED'
                        AND (scheduled_at IS NULL OR scheduled_at <= CURRENT_TIMESTAMP)
                        ORDER BY created_at ASC
                        LIMIT 1
                        FOR UPDATE SKIP LOCKED
                    )
                    RETURNING id, tenant_id, parent_task_id, payload, status
                    "#
                )
                .bind(worker_id)
                .bind(now)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    use sqlx::Row;
                    Ok(Some(SubAgentJob {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        parent_task_id: r.try_get("parent_task_id").unwrap_or(None),
                        payload: r.try_get("payload").unwrap_or(None),
                        status: r.get("status"),
                    }))
                } else {
                    Ok(None)
                }
            }
            DbStore::Sqlite(sqlite_pool) => {
                let _lock = self.sqlite_mu.lock().await;
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;

                let row = sqlx::query(
                    r#"
                    UPDATE sub_agent_queue
                    SET status = 'RUNNING', worker_id = ?, updated_at = ?
                    WHERE id = (
                        SELECT id FROM sub_agent_queue
                        WHERE status = 'QUEUED'
                        AND (scheduled_at IS NULL OR scheduled_at <= CURRENT_TIMESTAMP)
                        ORDER BY created_at ASC
                        LIMIT 1
                    )
                    RETURNING id, tenant_id, parent_task_id, payload, status
                    "#
                )
                .bind(worker_id)
                .bind(now.to_rfc3339())
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    use sqlx::Row;
                    Ok(Some(SubAgentJob {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        parent_task_id: r.try_get("parent_task_id").unwrap_or(None),
                        payload: r.try_get("payload").unwrap_or(None),
                        status: r.get("status"),
                    }))
                } else {
                    Ok(None)
                }
            }
        }
    }


    pub async fn start_polling<F, Fut>(
        self: Arc<Self>,
        worker_id: String,
        interval_ms: u64,
        mut handler: F,
    )
    where
        F: FnMut(SubAgentJob) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Result<(), String>> + Send + 'static,
    {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(interval_ms));

        loop {
            interval.tick().await;

            while let Ok(Some(job)) = self.poll_and_claim(&worker_id).await {
                let job_id = job.id.clone();
                match handler(job).await {
                    Ok(_) => {
                        let _ = self.mark_completed(&job_id).await;
                    }
                    Err(e) => {
                        let _ = self.mark_failed(&job_id, &e).await;
                    }
                }
            }
        }
    }

    pub async fn mark_completed(&self, job_id: &str) -> Result<(), String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query("UPDATE sub_agent_queue SET status = 'COMPLETED', updated_at = $1 WHERE id = $2")
                    .bind(Utc::now())
                    .bind(job_id)
                    .execute(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(())
            }
            DbStore::Sqlite(sqlite_pool) => {
                let _lock = self.sqlite_mu.lock().await;
                sqlx::query("UPDATE sub_agent_queue SET status = 'COMPLETED', updated_at = ? WHERE id = ?")
                    .bind(Utc::now().to_rfc3339())
                    .bind(job_id)
                    .execute(sqlite_pool)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(())
            }
        }
    }

    pub async fn mark_failed(&self, job_id: &str, error_msg: &str) -> Result<(), String> {
        let error_payload = serde_json::to_string(&serde_json::json!({"error": error_msg})).unwrap_or_default();
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query("UPDATE sub_agent_queue SET status = 'FAILED', payload = COALESCE(payload::jsonb, '{}'::jsonb) || $1::jsonb, updated_at = $2 WHERE id = $3")
                    .bind(error_payload)
                    .bind(Utc::now())
                    .bind(job_id)
                    .execute(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(())
            }
            DbStore::Sqlite(sqlite_pool) => {
                let _lock = self.sqlite_mu.lock().await;
                sqlx::query("UPDATE sub_agent_queue SET status = 'FAILED', payload = json_patch(COALESCE(payload, '{}'), ?), updated_at = ? WHERE id = ?")
                    .bind(error_payload)
                    .bind(Utc::now().to_rfc3339())
                    .bind(job_id)
                    .execute(sqlite_pool)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_db() -> sqlx::SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS sub_agent_queue (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                parent_task_id TEXT,
                payload TEXT,
                status TEXT,
                worker_id TEXT,
                scheduled_at TEXT,
                created_at TEXT,
                updated_at TEXT
            )"
        )
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_subagent_worker_claim() {
        let pool = setup_db().await;
        let db = Arc::new(crate::db::DB {
            store: DbStore::Sqlite(pool.clone()),
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
        });

        let worker = SubAgentWorker::new(db);

        // Insert job
        sqlx::query("INSERT INTO sub_agent_queue (id, tenant_id, status) VALUES ('job1', 'org1', 'QUEUED')")
            .execute(&pool)
            .await
            .unwrap();

        // Claim
        let job = worker.poll_and_claim("worker1").await.unwrap();
        assert!(job.is_some());
        let job = job.unwrap();
        assert_eq!(job.id, "job1");
        assert_eq!(job.status, "RUNNING");

        // Mark completed
        worker.mark_completed("job1").await.unwrap();

        let status: (String,) = sqlx::query_as("SELECT status FROM sub_agent_queue WHERE id = 'job1'")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(status.0, "COMPLETED");
    }

    #[tokio::test]
    async fn test_subagent_worker_fail() {
        let pool = setup_db().await;
        let db = Arc::new(crate::db::DB {
            store: DbStore::Sqlite(pool.clone()),
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
        });

        let worker = SubAgentWorker::new(db);

        // Insert job
        sqlx::query("INSERT INTO sub_agent_queue (id, tenant_id, status) VALUES ('job2', 'org1', 'QUEUED')")
            .execute(&pool)
            .await
            .unwrap();

        let job = worker.poll_and_claim("worker1").await.unwrap().unwrap();

        worker.mark_failed("job2", "Some error").await.unwrap();

        let row: (String, String) = sqlx::query_as("SELECT status, payload FROM sub_agent_queue WHERE id = 'job2'")
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(row.0, "FAILED");
        assert!(row.1.contains("Some error"));
    }
}
