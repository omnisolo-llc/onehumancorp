use crate::ohc::orchestration::Job;
use crate::queue::TaskQueue;
use async_trait::async_trait;
use std::sync::Arc;
use sqlx::{SqlitePool, Row};
use prost::Message;

pub struct SqliteTaskQueue {
    pool: Arc<SqlitePool>,
}

impl SqliteTaskQueue {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        SqliteTaskQueue { pool }
    }
}

#[async_trait]
impl TaskQueue for SqliteTaskQueue {
    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let mut buf = Vec::new();
        job.encode(&mut buf).map_err(|e| e.to_string())?;

        let locked_until = if job.locked_until > 0 { Some(job.locked_until) } else { None };

        sqlx::query(
            "INSERT INTO shared_tasks (id, organization_id, parent_plan_id, agent_role, payload, status, attempts, max_attempts, run_after, locked_until, created_at, updated_at, protobuf_blob)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, datetime(?, 'unixepoch'), CASE WHEN ? IS NOT NULL THEN datetime(?, 'unixepoch') ELSE NULL END, datetime(?, 'unixepoch'), datetime(?, 'unixepoch'), ?)"
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
        .bind(locked_until)
        .bind(locked_until)
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

        let roles_placeholders = roles.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        let query_str = format!(
            "SELECT id, protobuf_blob
             FROM shared_tasks
             WHERE status = 'pending' AND (locked_until IS NULL OR locked_until < strftime('%s', 'now')) AND run_after <= strftime('%s', 'now') AND agent_role IN ({}) AND protobuf_blob IS NOT NULL
             ORDER BY created_at ASC LIMIT 1",
            roles_placeholders
        );

        let mut query = sqlx::query(&query_str);
        for role in &roles {
            query = query.bind(role);
        }

        let row: Option<sqlx::sqlite::SqliteRow> = query
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(row) = row {
            let id: String = row.get("id");
            let blob: Vec<u8> = row.get("protobuf_blob");

            sqlx::query("UPDATE shared_tasks SET locked_until = strftime('%s', 'now', '+5 minutes') WHERE id = ?")
                .bind(&id)
                .execute(&*self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if let Ok(job) = Job::decode(&blob[..]) {
                return Ok(Some(job));
            }
        }

        Ok(None)
    }

    async fn complete(&self, job_id: &str) -> Result<(), String> {
        sqlx::query("UPDATE shared_tasks SET status = 'completed', updated_at = strftime('%s', 'now') WHERE id = ?")
            .bind(job_id)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn fail(&self, job_id: &str, _reason: &str) -> Result<(), String> {
        sqlx::query("UPDATE shared_tasks SET status = 'failed', updated_at = strftime('%s', 'now') WHERE id = ?")
            .bind(job_id)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
