use super::queue::{Job, TaskQueue};
use crate::db::{DB, DbStore};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::Row;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct SharedTaskQueue {
    db: Arc<DB>,
    sqlite_mutex: Mutex<()>,
}

impl SharedTaskQueue {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            sqlite_mutex: Mutex::new(()),
        }
    }
}

#[async_trait]
impl TaskQueue for SharedTaskQueue {
    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let payload_json = serde_json::to_string(&serde_json::json!({
            "payload": job.payload,
            "attempts": job.attempts,
            "max_attempts": job.max_attempts,
            "run_after": job.run_after.to_rfc3339(),
            "locked_until": job.locked_until.map(|dt| dt.to_rfc3339()),
            "parent_task_id": job.parent_task_id,
        })).unwrap_or_else(|_| "{}".to_string());

        let created_at = job.created_at.to_rfc3339();
        let updated_at = job.updated_at.to_rfc3339();

        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO shared_tasks_v4 (
                        id, organization_id, title, description, status, agent_id,
                        priority, payload, parent_plan_id, dependencies, created_at, updated_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                    "#
                )
                .bind(&job.id)
                .bind(&job.tenant_id)
                .bind("Sub-Agent Job")
                .bind(None::<String>)
                .bind("PENDING")
                .bind(&job.agent_role)
                .bind("NORMAL")
                .bind(&payload_json)
                .bind(&job.parent_task_id)
                .bind("[]")
                .bind(job.created_at)
                .bind(job.updated_at)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                let _lock = self.sqlite_mutex.lock().await;
                sqlx::query(
                    r#"
                    INSERT INTO shared_tasks_v4 (
                        id, organization_id, title, description, status, agent_id,
                        priority, payload, parent_plan_id, dependencies, created_at, updated_at
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&job.id)
                .bind(&job.tenant_id)
                .bind("Sub-Agent Job")
                .bind(None::<String>)
                .bind("PENDING")
                .bind(&job.agent_role)
                .bind("NORMAL")
                .bind(&payload_json)
                .bind(&job.parent_task_id)
                .bind("[]")
                .bind(&created_at)
                .bind(&updated_at)
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    async fn enqueue_batch(&self, jobs: Vec<Job>) -> Result<(), String> {
        for job in jobs {
            self.enqueue(job).await?;
        }
        Ok(())
    }

    async fn dequeue(&self, roles: Vec<String>, _estimated_vram: i64, _estimated_tokens: i64) -> Result<Option<Job>, String> {
        if roles.is_empty() {
            return Ok(None);
        }

        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                let role_placeholders = roles.iter().enumerate().map(|(i, _)| format!("${}", i + 1)).collect::<Vec<_>>().join(",");
                let query_str = format!(
                    r#"
                    UPDATE shared_tasks_v4
                    SET status = 'RUNNING', updated_at = CURRENT_TIMESTAMP
                    WHERE id = (
                        SELECT id FROM shared_tasks_v4
                        WHERE status = 'PENDING'
                        AND agent_id IN ({})
                        AND (payload->>'run_after' IS NULL OR (payload->>'run_after')::timestamptz <= CURRENT_TIMESTAMP)
                        ORDER BY created_at ASC
                        LIMIT 1
                        FOR UPDATE SKIP LOCKED
                    )
                    RETURNING id, organization_id, agent_id, payload, status, parent_plan_id, created_at, updated_at
                    "#,
                    role_placeholders
                );

                let mut query = sqlx::query(&query_str);
                for role in &roles {
                    query = query.bind(role);
                }

                let job_opt = query.fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;

                if let Some(row) = job_opt {
                    let payload_str: String = row.try_get("payload").unwrap_or_else(|_| "{}".to_string());
                    let payload_json: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or(serde_json::Value::Null);

                    let original_payload = payload_json.get("payload").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let attempts = payload_json.get("attempts").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                    let max_attempts = payload_json.get("max_attempts").and_then(|v| v.as_i64()).unwrap_or(3) as i32;

                    let run_after_str = payload_json.get("run_after").and_then(|v| v.as_str()).unwrap_or("");
                    let run_after = chrono::DateTime::parse_from_rfc3339(run_after_str)
                        .map(|d| d.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now());

                    let job = Job {
                        id: row.get("id"),
                        tenant_id: row.get("organization_id"),
                        parent_task_id: row.try_get("parent_plan_id").unwrap_or_default(),
                        agent_role: row.try_get("agent_id").unwrap_or_default(),
                        payload: original_payload,
                        status: row.try_get("status").unwrap_or_default(),
                        attempts,
                        max_attempts,
                        run_after,
                        locked_until: None,
                        created_at: row.get("created_at"),
                        updated_at: Utc::now(),
                    };

                    tx.commit().await.map_err(|e| e.to_string())?;
                    return Ok(Some(job));
                }

                tx.commit().await.map_err(|e| e.to_string())?;
                Ok(None)
            }
            DbStore::Sqlite(sqlite_pool) => {
                let _lock = self.sqlite_mutex.lock().await;
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;

                let role_placeholders = roles.iter().map(|_| "?").collect::<Vec<_>>().join(",");
                let query_str = format!(
                    r#"
                    SELECT id, organization_id, agent_id, payload, status, parent_plan_id, created_at, updated_at
                    FROM shared_tasks_v4
                    WHERE status = 'PENDING'
                    AND agent_id IN ({})
                    AND (json_extract(payload, '$.run_after') IS NULL OR json_extract(payload, '$.run_after') <= datetime('now'))
                    ORDER BY created_at ASC
                    LIMIT 1
                    "#,
                    role_placeholders
                );

                let mut query = sqlx::query(&query_str);
                for role in &roles {
                    query = query.bind(role);
                }

                let job_opt: Option<sqlx::sqlite::SqliteRow> = query.fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;

                if let Some(row) = job_opt {
                    let id: String = row.get("id");

                    sqlx::query("UPDATE shared_tasks_v4 SET status = 'RUNNING', updated_at = ? WHERE id = ?")
                        .bind(Utc::now().to_rfc3339())
                        .bind(&id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                    let payload_str: String = row.try_get("payload").unwrap_or_else(|_| "{}".to_string());
                    let payload_json: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or(serde_json::Value::Null);

                    let original_payload = payload_json.get("payload").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let attempts = payload_json.get("attempts").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                    let max_attempts = payload_json.get("max_attempts").and_then(|v| v.as_i64()).unwrap_or(3) as i32;

                    let run_after_str = payload_json.get("run_after").and_then(|v| v.as_str()).unwrap_or("");
                    let run_after = chrono::DateTime::parse_from_rfc3339(run_after_str)
                        .map(|d| d.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now());

                    let created_str: String = row.get("created_at");
                    let created_at = chrono::DateTime::parse_from_rfc3339(&created_str)
                        .map(|d| d.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now());

                    let job = Job {
                        id,
                        tenant_id: row.get("organization_id"),
                        parent_task_id: row.try_get("parent_plan_id").unwrap_or_default(),
                        agent_role: row.try_get("agent_id").unwrap_or_default(),
                        payload: original_payload,
                        status: "RUNNING".to_string(),
                        attempts,
                        max_attempts,
                        run_after,
                        locked_until: None,
                        created_at,
                        updated_at: Utc::now(),
                    };

                    tx.commit().await.map_err(|e| e.to_string())?;
                    return Ok(Some(job));
                }

                tx.commit().await.map_err(|e| e.to_string())?;
                Ok(None)
            }
        }
    }

    async fn complete(&self, job_id: &str) -> Result<(), String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query("UPDATE shared_tasks_v4 SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                    .bind(job_id)
                    .execute(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                let _lock = self.sqlite_mutex.lock().await;
                sqlx::query("UPDATE shared_tasks_v4 SET status = 'COMPLETED', updated_at = ? WHERE id = ?")
                    .bind(Utc::now().to_rfc3339())
                    .bind(job_id)
                    .execute(sqlite_pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    async fn fail(&self, job_id: &str, _reason: &str) -> Result<(), String> {
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                let row = sqlx::query("SELECT payload FROM shared_tasks_v4 WHERE id = $1 FOR UPDATE")
                    .bind(job_id)
                    .fetch_optional(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    let payload_str: String = r.try_get("payload").unwrap_or_else(|_| "{}".to_string());
                    let mut payload_json: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or(serde_json::Value::Null);

                    let current_attempts = payload_json.get("attempts").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                    let max_attempts = payload_json.get("max_attempts").and_then(|v| v.as_i64()).unwrap_or(3) as i32;
                    let next_attempt = current_attempts + 1;

                    if next_attempt >= max_attempts {
                        sqlx::query("UPDATE shared_tasks_v4 SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                            .bind(job_id)
                            .execute(&mut *tx)
                            .await
                            .map_err(|e| e.to_string())?;
                    } else {
                        if let Some(obj) = payload_json.as_object_mut() {
                            obj.insert("attempts".to_string(), serde_json::json!(next_attempt));
                        }
                        let new_payload_str = serde_json::to_string(&payload_json).unwrap_or_else(|_| "{}".to_string());

                        sqlx::query("UPDATE shared_tasks_v4 SET status = 'PENDING', payload = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2")
                            .bind(&new_payload_str)
                            .bind(job_id)
                            .execute(&mut *tx)
                            .await
                            .map_err(|e| e.to_string())?;
                    }
                }

                tx.commit().await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                let _lock = self.sqlite_mutex.lock().await;
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;

                let row = sqlx::query("SELECT payload FROM shared_tasks_v4 WHERE id = ?")
                    .bind(job_id)
                    .fetch_optional(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    let payload_str: String = r.try_get("payload").unwrap_or_else(|_| "{}".to_string());
                    let mut payload_json: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or(serde_json::Value::Null);

                    let current_attempts = payload_json.get("attempts").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                    let max_attempts = payload_json.get("max_attempts").and_then(|v| v.as_i64()).unwrap_or(3) as i32;
                    let next_attempt = current_attempts + 1;

                    if next_attempt >= max_attempts {
                        sqlx::query("UPDATE shared_tasks_v4 SET status = 'FAILED', updated_at = ? WHERE id = ?")
                            .bind(Utc::now().to_rfc3339())
                            .bind(job_id)
                            .execute(&mut *tx)
                            .await
                            .map_err(|e| e.to_string())?;
                    } else {
                        if let Some(obj) = payload_json.as_object_mut() {
                            obj.insert("attempts".to_string(), serde_json::json!(next_attempt));
                        }
                        let new_payload_str = serde_json::to_string(&payload_json).unwrap_or_else(|_| "{}".to_string());

                        sqlx::query("UPDATE shared_tasks_v4 SET status = 'PENDING', payload = ?, updated_at = ? WHERE id = ?")
                            .bind(&new_payload_str)
                            .bind(Utc::now().to_rfc3339())
                            .bind(job_id)
                            .execute(&mut *tx)
                            .await
                            .map_err(|e| e.to_string())?;
                    }
                }

                tx.commit().await.map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn test_shared_task_queue_sqlite() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE shared_tasks_v4 (
                id TEXT PRIMARY KEY,
                organization_id TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL,
                agent_id TEXT,
                priority TEXT NOT NULL,
                payload TEXT,
                parent_plan_id TEXT,
                dependencies TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )"
        ).execute(&pool).await.unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
            store: DbStore::Sqlite(pool),
        });

        let queue = SharedTaskQueue::new(db);

        let job = Job {
            id: "job-1".to_string(),
            tenant_id: "org-1".to_string(),
            parent_task_id: "parent-1".to_string(),
            agent_role: "agent-a".to_string(),
            payload: "hello".to_string(),
            status: "PENDING".to_string(),
            attempts: 0,
            max_attempts: 3,
            run_after: Utc::now(),
            locked_until: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        queue.enqueue(job).await.unwrap();

        let dequeued_opt = queue.dequeue(vec!["agent-a".to_string()], 1, 1).await.unwrap();
        assert!(dequeued_opt.is_some());
        let dequeued = dequeued_opt.unwrap();
        assert_eq!(dequeued.id, "job-1");
        assert_eq!(dequeued.payload, "hello");
        assert_eq!(dequeued.attempts, 0);

        queue.complete("job-1").await.unwrap();

        let row = sqlx::query("SELECT status FROM shared_tasks_v4 WHERE id = 'job-1'")
            .fetch_one(match &queue.db.store { DbStore::Sqlite(p) => p, _ => unreachable!() })
            .await
            .unwrap();
        let status: String = row.get("status");
        assert_eq!(status, "COMPLETED");
    }

    #[tokio::test]
    async fn test_shared_task_queue_fail_retry() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE shared_tasks_v4 (
                id TEXT PRIMARY KEY,
                organization_id TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL,
                agent_id TEXT,
                priority TEXT NOT NULL,
                payload TEXT,
                parent_plan_id TEXT,
                dependencies TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )"
        ).execute(&pool).await.unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
            store: DbStore::Sqlite(pool.clone()),
        });

        let queue = SharedTaskQueue::new(db);

        let job = Job {
            id: "job-fail".to_string(),
            tenant_id: "org-1".to_string(),
            parent_task_id: "parent-1".to_string(),
            agent_role: "agent-a".to_string(),
            payload: "failme".to_string(),
            status: "PENDING".to_string(),
            attempts: 0,
            max_attempts: 2,
            run_after: Utc::now(),
            locked_until: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        queue.enqueue(job).await.unwrap();

        // First attempt -> dequeues
        let _ = queue.dequeue(vec!["agent-a".to_string()], 1, 1).await.unwrap();

        // Fail it -> should retry
        queue.fail("job-fail", "some error").await.unwrap();

        let row = sqlx::query("SELECT status FROM shared_tasks_v4 WHERE id = 'job-fail'")
            .fetch_one(&pool)
            .await
            .unwrap();
        let status: String = row.get("status");
        assert_eq!(status, "PENDING"); // Requeued

        // Second attempt -> dequeues
        let dequeued2 = queue.dequeue(vec!["agent-a".to_string()], 1, 1).await.unwrap().unwrap();
        assert_eq!(dequeued2.attempts, 1);

        // Fail it again -> should poison pill (max attempts = 2)
        queue.fail("job-fail", "some error again").await.unwrap();

        let row2 = sqlx::query("SELECT status FROM shared_tasks_v4 WHERE id = 'job-fail'")
            .fetch_one(&pool)
            .await
            .unwrap();
        let status2: String = row2.get("status");
        assert_eq!(status2, "FAILED"); // Poisoned
    }

    #[tokio::test]
    async fn test_shared_task_queue_concurrent() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE shared_tasks_v4 (
                id TEXT PRIMARY KEY,
                organization_id TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL,
                agent_id TEXT,
                priority TEXT NOT NULL,
                payload TEXT,
                parent_plan_id TEXT,
                dependencies TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )"
        ).execute(&pool).await.unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
            store: DbStore::Sqlite(pool),
        });

        let queue = Arc::new(SharedTaskQueue::new(db));

        // Enqueue 10 tasks
        for i in 0..10 {
            queue.enqueue(Job {
                id: format!("job-{}", i),
                tenant_id: "org-1".to_string(),
                parent_task_id: "".to_string(),
                agent_role: "agent-x".to_string(),
                payload: "".to_string(),
                status: "PENDING".to_string(),
                attempts: 0,
                max_attempts: 1,
                run_after: Utc::now(),
                locked_until: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }).await.unwrap();
        }

        // 5 concurrent workers trying to dequeue
        let mut handles = vec![];
        for _ in 0..5 {
            let q = queue.clone();
            handles.push(tokio::spawn(async move {
                let mut claimed = 0;
                while let Ok(Some(_job)) = q.dequeue(vec!["agent-x".to_string()], 1, 1).await {
                    claimed += 1;
                    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                }
                claimed
            }));
        }

        let mut total_claimed = 0;
        for h in handles {
            total_claimed += h.await.unwrap();
        }

        assert_eq!(total_claimed, 10);
    }
}
