use std::sync::Arc;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::db::{DB, DbStore};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub organization_id: String,
    pub parent_task_id: String,
    pub agent_role: String,
    pub payload: serde_json::Value,
    pub status: String,
    pub attempts: i32,
    pub max_attempts: i32,
    pub run_after: DateTime<Utc>,
    pub locked_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct SubAgentQueue {
    db: Arc<DB>,
}

impl SubAgentQueue {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn enqueue(
        &self,
        organization_id: &str,
        parent_task_id: &str,
        agent_role: &str,
        payload: serde_json::Value,
    ) -> Result<String, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let pool = &self.db.pool;
                sqlx::query(
                    r#"
                    INSERT INTO sub_agent_jobs (
                        id, organization_id, parent_task_id, agent_role, payload, status,
                        attempts, max_attempts, run_after, created_at, updated_at
                    ) VALUES ($1, $2, $3, $4, $5, 'QUEUED', 0, 3, $6, $7, $8)
                    "#,
                )
                .bind(&id)
                .bind(organization_id)
                .bind(parent_task_id)
                .bind(agent_role)
                .bind(&payload)
                .bind(now)
                .bind(now)
                .bind(now)
                .execute(pool)
                .await
                .map_err(|e: sqlx::Error| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO sub_agent_jobs (
                        id, organization_id, parent_task_id, agent_role, payload, status,
                        attempts, max_attempts, run_after, created_at, updated_at
                    ) VALUES ($1, $2, $3, $4, $5, 'QUEUED', 0, 3, $6, $7, $8)
                    "#,
                )
                .bind(&id)
                .bind(organization_id)
                .bind(parent_task_id)
                .bind(agent_role)
                .bind(serde_json::to_string(&payload).unwrap())
                .bind(now)
                .bind(now)
                .bind(now)
                .execute(sqlite_pool)
                .await
                .map_err(|e: sqlx::Error| e.to_string())?;
            }
        }
        Ok(id)
    }

    pub async fn acquire(&self) -> Result<Option<Job>, String> {
        let now = Utc::now();
        let future_time = now + Duration::try_minutes(5).unwrap();

        match &self.db.store {
            DbStore::Postgres => {
                let row = sqlx::query(
                    r#"
                    UPDATE sub_agent_jobs
                    SET status = 'RUNNING',
                        locked_until = $1,
                        attempts = attempts + 1,
                        updated_at = $2
                    WHERE id = (
                        SELECT id
                        FROM sub_agent_jobs
                        WHERE ((status = 'QUEUED' AND run_after <= $3)
                           OR (status = 'RUNNING' AND locked_until <= $3))
                           AND attempts < max_attempts
                        ORDER BY run_after ASC
                        FOR UPDATE SKIP LOCKED
                        LIMIT 1
                    )
                    RETURNING id, organization_id, parent_task_id, agent_role, payload, status, attempts, max_attempts, run_after, locked_until, created_at, updated_at
                    "#,
                )
                .bind(future_time)
                .bind(now)
                .bind(now)
                .fetch_optional(&self.db.pool)
                .await
                .map_err(|e: sqlx::Error| e.to_string())?;

                if let Some(r) = row {
                    Ok(Some(Job {
                        id: r.try_get("id").map_err(|e: sqlx::Error| e.to_string())?,
                        organization_id: r.try_get("organization_id").map_err(|e: sqlx::Error| e.to_string())?,
                        parent_task_id: r.try_get("parent_task_id").map_err(|e: sqlx::Error| e.to_string())?,
                        agent_role: r.try_get("agent_role").map_err(|e: sqlx::Error| e.to_string())?,
                        payload: r.try_get("payload").map_err(|e: sqlx::Error| e.to_string())?,
                        status: r.try_get("status").map_err(|e: sqlx::Error| e.to_string())?,
                        attempts: r.try_get("attempts").map_err(|e: sqlx::Error| e.to_string())?,
                        max_attempts: r.try_get("max_attempts").map_err(|e: sqlx::Error| e.to_string())?,
                        run_after: r.try_get("run_after").map_err(|e: sqlx::Error| e.to_string())?,
                        locked_until: r.try_get("locked_until").ok(),
                        created_at: r.try_get("created_at").map_err(|e: sqlx::Error| e.to_string())?,
                        updated_at: r.try_get("updated_at").map_err(|e: sqlx::Error| e.to_string())?,
                    }))
                } else {
                    Ok(None)
                }
            }
            DbStore::Sqlite(sqlite_pool) => {
                let row = sqlx::query(
                    r#"
                    UPDATE sub_agent_jobs
                    SET status = 'RUNNING',
                        locked_until = $1,
                        attempts = attempts + 1,
                        updated_at = $2
                    WHERE id = (
                        SELECT id
                        FROM sub_agent_jobs
                        WHERE ((status = 'QUEUED' AND run_after <= $3)
                           OR (status = 'RUNNING' AND locked_until <= $3))
                           AND attempts < max_attempts
                        ORDER BY run_after ASC
                        LIMIT 1
                    )
                    RETURNING id, organization_id, parent_task_id, agent_role, payload, status, attempts, max_attempts, run_after, locked_until, created_at, updated_at
                    "#,
                )
                .bind(future_time)
                .bind(now)
                .bind(now)
                .fetch_optional(sqlite_pool)
                .await
                .map_err(|e: sqlx::Error| e.to_string())?;

                if let Some(r) = row {
                    let id: String = r.try_get("id").map_err(|e: sqlx::Error| e.to_string())?;
                    let payload_str: String = r.try_get("payload").map_err(|e: sqlx::Error| e.to_string())?;
                    let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or(serde_json::json!({}));

                    let locked_until: Option<DateTime<Utc>> = r.try_get("locked_until").ok();
                    let _locked_until = locked_until;

                    let run_after: DateTime<Utc> = r.try_get("run_after").unwrap_or(Utc::now());

                    let created_at: DateTime<Utc> = r.try_get("created_at").unwrap_or(Utc::now());

                    let attempts: i32 = r.try_get::<i32, _>("attempts").map_err(|e: sqlx::Error| e.to_string())?;

                    Ok(Some(Job {
                        id,
                        organization_id: r.try_get("organization_id").map_err(|e: sqlx::Error| e.to_string())?,
                        parent_task_id: r.try_get("parent_task_id").map_err(|e: sqlx::Error| e.to_string())?,
                        agent_role: r.try_get("agent_role").map_err(|e: sqlx::Error| e.to_string())?,
                        payload,
                        status: "RUNNING".to_string(),
                        attempts,
                        max_attempts: r.try_get("max_attempts").map_err(|e: sqlx::Error| e.to_string())?,
                        run_after,
                        locked_until: Some(future_time),
                        created_at,
                        updated_at: now,
                    }))
                } else {
                    Ok(None)
                }
            }
        }
    }

    pub async fn complete_job(&self, job_id: &str) -> Result<(), String> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let pool = &self.db.pool;
                sqlx::query(
                    r#"
                    UPDATE sub_agent_jobs
                    SET status = 'COMPLETED', updated_at = $1
                    WHERE id = $2
                    "#,
                )
                .bind(now)
                .bind(job_id)
                .execute(pool)
                .await
                .map_err(|e: sqlx::Error| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    UPDATE sub_agent_jobs
                    SET status = 'COMPLETED', updated_at = $1
                    WHERE id = $2
                    "#,
                )
                .bind(now)
                .bind(job_id)
                .execute(sqlite_pool)
                .await
                .map_err(|e: sqlx::Error| e.to_string())?;
            }
        }
        Ok(())
    }

    pub async fn fail_job(&self, job_id: &str) -> Result<(), String> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let pool = &self.db.pool;
                sqlx::query(
                    r#"
                    UPDATE sub_agent_jobs
                    SET status = CASE
                            WHEN attempts >= max_attempts THEN 'FAILED'
                            ELSE 'QUEUED'
                        END,
                        run_after = CASE
                            WHEN attempts >= max_attempts THEN run_after
                            ELSE $1 + make_interval(secs := power(2, attempts) * 10)
                        END,
                        locked_until = NULL,
                        updated_at = $1
                    WHERE id = $2
                    "#,
                )
                .bind(now)
                .bind(job_id)
                .execute(pool)
                .await
                .map_err(|e: sqlx::Error| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    UPDATE sub_agent_jobs
                    SET status = CASE
                            WHEN attempts >= max_attempts THEN 'FAILED'
                            ELSE 'QUEUED'
                        END,
                        run_after = CASE
                            WHEN attempts >= max_attempts THEN run_after
                            ELSE datetime($1, '+' || (10 * (1 << attempts)) || ' seconds')
                        END,
                        locked_until = NULL,
                        updated_at = $1
                    WHERE id = $2
                    "#,
                )
                .bind(now)
                .bind(job_id)
                .execute(sqlite_pool)
                .await
                .map_err(|e: sqlx::Error| e.to_string())?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::str::FromStr;

    async fn setup_db() -> Arc<DB> {
        let dummy_pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap();

        let conn_opts = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:")
            .unwrap()
            .create_if_missing(true);

        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect_with(conn_opts)
            .await.unwrap();

        let db = DB { pool: dummy_pool, store: DbStore::Sqlite(sqlite_pool) };

        match &db.store {
            DbStore::Sqlite(pool) => {
                sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS sub_agent_jobs (
                        id TEXT PRIMARY KEY,
                        organization_id TEXT NOT NULL DEFAULT 'system',
                        parent_task_id TEXT,
                        agent_role TEXT NOT NULL,
                        payload TEXT NOT NULL,
                        status TEXT NOT NULL DEFAULT 'QUEUED',
                        attempts INTEGER DEFAULT 0,
                        max_attempts INTEGER DEFAULT 3,
                        run_after TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                        locked_until TIMESTAMPTZ,
                        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                        updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
                    );
                    "#
                ).execute(pool).await.unwrap();
            }
            _ => {}
        }

        Arc::new(db)
    }

    #[tokio::test]
    async fn test_enqueue_and_acquire() {
        let db = setup_db().await;
        let queue = SubAgentQueue::new(db);

        let parent_task_id = "task-123";
        let agent_role = "worker";
        let payload = serde_json::json!({"key": "value"});
        let org_id = "org-1";

        let job_id = queue.enqueue(org_id, parent_task_id, agent_role, payload.clone()).await.unwrap();

        let acquired_job = queue.acquire().await.unwrap().unwrap();

        assert_eq!(acquired_job.id, job_id);
        assert_eq!(acquired_job.parent_task_id, parent_task_id);
        assert_eq!(acquired_job.agent_role, agent_role);
        assert_eq!(acquired_job.payload, payload);
        assert_eq!(acquired_job.status, "RUNNING");
        assert_eq!(acquired_job.attempts, 1);

        // Trying to acquire again should return None because the job is locked
        let second_acquire = queue.acquire().await.unwrap();
        assert!(second_acquire.is_none());
    }

    #[tokio::test]
    async fn test_complete_job() {
        let db = setup_db().await;
        let queue = SubAgentQueue::new(db);

        let job_id = queue.enqueue("org-1", "task-1", "worker", serde_json::json!({})).await.unwrap();

        // Acquire to set status to RUNNING
        queue.acquire().await.unwrap();

        queue.complete_job(&job_id).await.unwrap();

        // Let's verify it in DB (SQLite specific for test)
        match &queue.db.store {
            DbStore::Sqlite(pool) => {
                let row = sqlx::query("SELECT status FROM sub_agent_jobs WHERE id = $1")
                    .bind(&job_id)
                    .fetch_one(pool)
                    .await
                    .unwrap();
                let status: String = row.try_get("status").unwrap();
                assert_eq!(status, "COMPLETED");
            }
            _ => panic!("Test expected SQLite"),
        }
    }

    #[tokio::test]
    async fn test_fail_job() {
        let db = setup_db().await;
        let queue = SubAgentQueue::new(db);

        let job_id = queue.enqueue("org-1", "task-1", "worker", serde_json::json!({})).await.unwrap();

        // Attempt 1
        queue.acquire().await.unwrap();
        queue.fail_job(&job_id).await.unwrap();

        // Manually update the job's run_after back to current time so acquire picks it up immediately
        match &queue.db.store {
            DbStore::Sqlite(pool) => {
                sqlx::query("UPDATE sub_agent_jobs SET run_after = CURRENT_TIMESTAMP WHERE id = $1")
                    .bind(&job_id)
                    .execute(pool).await.unwrap();
            }
            _ => panic!("Test expected SQLite"),
        }

        // Should be back to QUEUED
        let job = queue.acquire().await.unwrap().unwrap();
        assert_eq!(job.attempts, 2);

        queue.fail_job(&job_id).await.unwrap();

        match &queue.db.store {
            DbStore::Sqlite(pool) => {
                sqlx::query("UPDATE sub_agent_jobs SET run_after = CURRENT_TIMESTAMP WHERE id = $1")
                    .bind(&job_id)
                    .execute(pool).await.unwrap();
            }
            _ => panic!("Test expected SQLite"),
        }

        // Attempt 3
        let job = queue.acquire().await.unwrap().unwrap();
        assert_eq!(job.attempts, 3);

        queue.fail_job(&job_id).await.unwrap();

        // Should be FAILED now since max_attempts is 3
        let empty_acquire = queue.acquire().await.unwrap();
        assert!(empty_acquire.is_none());

        match &queue.db.store {
            DbStore::Sqlite(pool) => {
                let row = sqlx::query("SELECT status FROM sub_agent_jobs WHERE id = $1")
                    .bind(&job_id)
                    .fetch_one(pool)
                    .await
                    .unwrap();
                let status: String = row.try_get("status").unwrap();
                assert_eq!(status, "FAILED");
            }
            _ => panic!("Test expected SQLite"),
        }
    }
}
