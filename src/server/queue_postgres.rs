use crate::ohc::orchestration::Job;
use crate::queue::TaskQueue;
use async_trait::async_trait;
use std::sync::Arc;
use sqlx::{PgPool, Row};
use prost::Message;

pub struct PostgresTaskQueue {
    pool: Arc<PgPool>,
}

impl PostgresTaskQueue {
    pub fn new(pool: Arc<PgPool>) -> Self {
        PostgresTaskQueue { pool }
    }
}

#[async_trait]
impl TaskQueue for PostgresTaskQueue {
    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let mut buf = Vec::new();
        job.encode(&mut buf).map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO shared_tasks (id, organization_id, parent_plan_id, agent_role, payload, status, attempts, max_attempts, run_after, locked_until, created_at, updated_at, protobuf_blob)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, to_timestamp($9::double precision), CASE WHEN $10 > 0 THEN to_timestamp($10::double precision) ELSE NULL END, to_timestamp($11::double precision), to_timestamp($12::double precision), $13)"
        )
        .bind(&job.id)
        .bind(&job.tenant_id)
        .bind(&job.parent_task_id)
        .bind(&job.agent_role)
        .bind(&job.payload)
        .bind(&job.status)
        .bind(job.attempts)
        .bind(job.max_attempts)
        .bind(job.run_after)
        .bind(if job.locked_until > 0 { Some(job.locked_until) } else { None })
        .bind(job.created_at)
        .bind(job.updated_at)
        .bind(buf)
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

        let roles_placeholders = roles.iter().enumerate().map(|(i, _)| format!("${}", i + 1)).collect::<Vec<_>>().join(", ");
        let query_str = format!(
            "SELECT id, protobuf_blob
             FROM shared_tasks
             WHERE status = 'pending' AND (locked_until IS NULL OR locked_until < extract(epoch from now())::bigint) AND run_after <= extract(epoch from now())::bigint AND agent_role IN ({}) AND protobuf_blob IS NOT NULL
             ORDER BY created_at ASC LIMIT 1 FOR UPDATE SKIP LOCKED",
            roles_placeholders
        );

        let mut query = sqlx::query(&query_str);
        for role in &roles {
            query = query.bind(role);
        }

        let row_opt = query.fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;

        if let Some(row) = row_opt {
            let id: String = row.get("id");
            let blob: Vec<u8> = row.get("protobuf_blob");

            sqlx::query("UPDATE shared_tasks SET locked_until = extract(epoch from now())::bigint + 300 WHERE id = $1")
                .bind(&id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

            tx.commit().await.map_err(|e| e.to_string())?;

            if let Ok(job) = Job::decode(&blob[..]) {
                return Ok(Some(job));
            }
        }

        Ok(None)
    }

    async fn complete(&self, job_id: &str) -> Result<(), String> {
        sqlx::query("UPDATE shared_tasks SET status = 'completed', updated_at = extract(epoch from now())::bigint WHERE id = $1")
            .bind(job_id)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn fail(&self, job_id: &str, _reason: &str) -> Result<(), String> {
        sqlx::query("UPDATE shared_tasks SET status = 'failed', updated_at = extract(epoch from now())::bigint WHERE id = $1")
            .bind(job_id)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
