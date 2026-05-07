use super::queue::{Job, TaskQueue};
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use std::sync::Arc;

pub struct PostgresTaskQueue {
    pool: Arc<PgPool>,
}

impl PostgresTaskQueue {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskQueue for PostgresTaskQueue {
    async fn enqueue_batch(&self, jobs: Vec<Job>) -> Result<(), String> {
        if jobs.is_empty() { return Ok(()); }
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        for job in jobs {
            sqlx::query(
                "INSERT INTO sub_agent_jobs (id, organization_id, parent_task_id, agent_role, payload, status, run_after)
                 VALUES ($1, $2, $3, $4, $5, 'QUEUED', $6)"
            )
            .bind(&job.id)
            .bind(&job.tenant_id)
            .bind(&job.parent_task_id)
            .bind(&job.agent_role)
            .bind(serde_json::from_str::<serde_json::Value>(&job.payload).unwrap_or_else(|_| serde_json::json!({})))
            .bind(job.run_after)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }
        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn enqueue(&self, job: Job) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO sub_agent_jobs (id, organization_id, parent_task_id, agent_role, payload, status, run_after)
             VALUES ($1, $2, $3, $4, $5, 'QUEUED', $6)"
        )
        .bind(&job.id)
        .bind(&job.tenant_id)
        .bind(&job.parent_task_id)
        .bind(&job.agent_role)
        .bind(serde_json::from_str::<serde_json::Value>(&job.payload).unwrap_or_else(|_| serde_json::json!({})))
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

        // Bypass RLS for queue operations to avoid issues where the queue processor
        // might not have the correct tenant context.
        sqlx::query("SET LOCAL ROLE ohc_bypassrls")
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        let row_opt = sqlx::query(
            "UPDATE sub_agent_jobs
             SET status = 'RUNNING', updated_at = CURRENT_TIMESTAMP
             WHERE id = (
                 SELECT id
                 FROM sub_agent_jobs
                 WHERE status = 'QUEUED'
                   AND agent_role = ANY($1)
                   AND run_after <= CURRENT_TIMESTAMP
                 ORDER BY run_after ASC
                 LIMIT 1
                 FOR UPDATE SKIP LOCKED
             )
             RETURNING id, organization_id, parent_task_id, agent_role, payload, status, attempts, max_attempts, run_after, locked_until, created_at, updated_at"
        )
        .bind(&roles)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(row) = row_opt {
            let payload: serde_json::Value = row.get("payload");
            let job = Job {
                id: row.get("id"),
                tenant_id: row.try_get("organization_id").unwrap_or_default(),
                parent_task_id: row.get("parent_task_id"),
                agent_role: row.get("agent_role"),
                payload: serde_json::to_string(&payload).unwrap_or_default(),
                status: row.get("status"),
                attempts: row.get("attempts"),
                max_attempts: row.get("max_attempts"),
                run_after: row.try_get("run_after").unwrap_or_else(|_| chrono::Utc::now()),
                locked_until: row.try_get("locked_until").unwrap_or(None),
                created_at: row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now()),
            };

            tx.commit().await.map_err(|e| e.to_string())?;
            return Ok(Some(job));
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(None)
    }

    async fn complete(&self, job_id: &str) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        sqlx::query("SET LOCAL ROLE ohc_bypassrls")
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query("UPDATE sub_agent_jobs SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(job_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn fail(&self, job_id: &str, _reason: &str) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        sqlx::query("SET LOCAL ROLE ohc_bypassrls")
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query("UPDATE sub_agent_jobs SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(job_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
