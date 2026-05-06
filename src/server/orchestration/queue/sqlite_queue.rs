use super::queue::{Job, TaskQueue};
use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
use std::sync::Arc;

pub struct SQLiteTaskQueue {
    pool: Arc<SqlitePool>,
}

impl SQLiteTaskQueue {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskQueue for SQLiteTaskQueue {
    async fn enqueue_batch(&self, jobs: Vec<Job>) -> Result<(), String> {
        if jobs.is_empty() { return Ok(()); }
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        for job in jobs {
            sqlx::query("INSERT INTO local_queue_jobs (id, tenant_id, task_id, role, payload) VALUES (?, ?, ?, ?, ?)")
                .bind(job.id.clone())
                .bind(job.tenant_id.clone())
                .bind(job.parent_task_id.clone())
                .bind(job.agent_role.clone())
                .bind(job.payload.as_bytes())
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
        }
        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn enqueue(&self, job: Job) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO sub_agent_jobs (id, parent_task_id, agent_role, payload, status, run_after)
             VALUES (?, ?, ?, ?, 'QUEUED', ?)"
        )
        .bind(&job.id)
        .bind(&job.parent_task_id)
        .bind(&job.agent_role)
        .bind(&job.payload)
        .bind(job.run_after)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn dequeue(&self, roles: Vec<String>) -> Result<Option<Job>, String> {
        if roles.is_empty() {
            return Ok(None);
        }

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let role_placeholders = roles.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query_str = format!(
            "SELECT id, parent_task_id, agent_role, payload, status, attempts, max_attempts, run_after, locked_until, created_at, updated_at
             FROM sub_agent_jobs
             WHERE status = 'QUEUED' AND agent_role IN ({})
             LIMIT 1",
            role_placeholders
        );

        let mut query = sqlx::query(&query_str);
        for role in &roles {
            query = query.bind(role);
        }

        let job_opt: Option<sqlx::sqlite::SqliteRow> = query.fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;

        if let Some(row) = job_opt {
            let job = Job {
                id: row.get("id"),
                parent_task_id: row.get("parent_task_id"),
                agent_role: row.get("agent_role"),
                payload: row.get("payload"),
                status: row.get("status"),
                attempts: row.get("attempts"),
                max_attempts: row.get("max_attempts"),
                run_after: row.try_get("run_after").unwrap_or_else(|_| chrono::Utc::now()),
                locked_until: row.try_get("locked_until").unwrap_or(None),
                created_at: row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now()),
            };

            sqlx::query("UPDATE sub_agent_jobs SET status = 'RUNNING', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                .bind(&job.id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

            tx.commit().await.map_err(|e| e.to_string())?;
            return Ok(Some(job));
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(None)
    }

    async fn complete(&self, job_id: &str) -> Result<(), String> {
        sqlx::query("UPDATE sub_agent_jobs SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(job_id)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn fail(&self, job_id: &str, _reason: &str) -> Result<(), String> {
        sqlx::query("UPDATE sub_agent_jobs SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(job_id)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
