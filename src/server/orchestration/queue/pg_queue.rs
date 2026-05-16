use super::queue::{Job, TaskQueue};
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use std::sync::Arc;

pub struct PgTaskQueue {
    pool: Arc<PgPool>,
}

impl PgTaskQueue {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskQueue for PgTaskQueue {
    async fn enqueue_batch(&self, jobs: Vec<Job>) -> Result<(), String> {
        if jobs.is_empty() { return Ok(()); }
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let mut query_str = String::from("INSERT INTO sub_agent_jobs (id, parent_task_id, agent_role, payload, status, run_after, organization_id) VALUES ");
        let mut values = Vec::new();

        for i in 0..jobs.len() {
            let base = i * 6;
            values.push(format!("(${}, ${}, ${}, ${}, 'QUEUED', ${}, ${})", base + 1, base + 2, base + 3, base + 4, base + 5, base + 6));
        }
        query_str.push_str(&values.join(", "));

        let mut query = sqlx::query(&query_str);

        for job in jobs {
            let payload_json: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or(serde_json::Value::Null);
            query = query
                .bind(job.id)
                .bind(job.parent_task_id)
                .bind(job.agent_role)
                .bind(payload_json)
                .bind(job.run_after)
                .bind(job.tenant_id);
        }

        query.execute(&mut *tx).await.map_err(|e| e.to_string())?;
        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let payload_json: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or(serde_json::Value::Null);
        sqlx::query(
            "INSERT INTO sub_agent_jobs (id, parent_task_id, agent_role, payload, status, run_after, organization_id)
             VALUES ($1, $2, $3, $4, 'QUEUED', $5, $6)"
        )
        .bind(&job.id)
        .bind(&job.parent_task_id)
        .bind(&job.agent_role)
        .bind(payload_json)
        .bind(job.run_after)
        .bind(&job.tenant_id)
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

        let role_placeholders = roles.iter().enumerate().map(|(i, _)| format!("${}", i + 1)).collect::<Vec<_>>().join(",");
        let query_str = format!(
            "SELECT id, parent_task_id, agent_role, payload, status, attempts, max_attempts, run_after, locked_until, created_at, updated_at, organization_id
             FROM sub_agent_jobs
             WHERE status = 'QUEUED' AND agent_role IN ({})
             LIMIT 1
             FOR UPDATE SKIP LOCKED",
            role_placeholders
        );

        let mut query = sqlx::query(&query_str);
        for role in &roles {
            query = query.bind(role);
        }

        let job_opt = query.fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;

        if let Some(row) = job_opt {
            let payload_val: serde_json::Value = row.try_get("payload").unwrap_or(serde_json::Value::Null);
            let payload_str = serde_json::to_string(&payload_val).unwrap_or_default();
            let job = Job {
                id: row.get("id"),
                parent_task_id: row.try_get("parent_task_id").unwrap_or_default(),
                agent_role: row.try_get("agent_role").unwrap_or_default(),
                payload: payload_str,
                status: row.try_get("status").unwrap_or_default(),
                attempts: row.try_get("attempts").unwrap_or(0),
                max_attempts: row.try_get("max_attempts").unwrap_or(3),
                run_after: row.try_get("run_after").unwrap_or_else(|_| chrono::Utc::now()),
                locked_until: row.try_get("locked_until").unwrap_or(None),
                created_at: row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now()),
                tenant_id: row.try_get("organization_id").unwrap_or_default(),
            };

            sqlx::query("UPDATE sub_agent_jobs SET status = 'RUNNING', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                .bind(&job.id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

            tx.commit().await.map_err(|e| e.to_string())?;

            // Log queue depth decrement
            let depth_res: Option<i64> = sqlx::query_scalar("SELECT COUNT(*) FROM sub_agent_jobs WHERE status = 'QUEUED'")
                .fetch_optional(&*self.pool)
                .await
                .unwrap_or(Some(0));
            let _ = ::server_telemetry::record_swarm_queue_depth(&self.pool, "main", depth_res.unwrap_or(0) as i32).await;

            return Ok(Some(job));
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(None)
    }

    async fn complete(&self, job_id: &str) -> Result<(), String> {
        let job_opt: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar("SELECT created_at FROM sub_agent_jobs WHERE id = $1")
            .bind(job_id)
            .fetch_optional(&*self.pool)
            .await
            .unwrap_or(None);

        sqlx::query("UPDATE sub_agent_jobs SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(job_id)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(created_at) = job_opt {
            let latency = (chrono::Utc::now() - created_at).num_milliseconds() as f64 / 1000.0;
            let _ = ::server_telemetry::record_swarm_job_processing_latency(&self.pool, latency).await;
        }

        Ok(())
    }

    async fn fail(&self, job_id: &str, _reason: &str) -> Result<(), String> {
        sqlx::query("UPDATE sub_agent_jobs SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(job_id)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
