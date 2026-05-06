use crate::ohc::orchestration::Job;
use super::queue::TaskQueue;
use async_trait::async_trait;
use std::sync::Arc;
use sqlx::{SqlitePool, Row};
use prost::Message;

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
    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let mut buf = Vec::new();
        job.encode(&mut buf).map_err(|e| e.to_string())?;

        let locked_until = if job.locked_until > 0 { job.locked_until } else { 0 };

        sqlx::query(
            "INSERT INTO jobs (id, tenant_id, agent_role, status, run_after, locked_until, created_at, protobuf_blob)
             VALUES (?, ?, ?, ?, datetime(?, 'unixepoch'), CASE WHEN ? > 0 THEN datetime(?, 'unixepoch') ELSE NULL END, datetime(?, 'unixepoch'), ?)"
        )
        .bind(&job.id)
        .bind(&job.tenant_id)
        .bind(&job.agent_role)
        .bind(&job.status)
        .bind(job.run_after)
        .bind(locked_until)
        .bind(locked_until)
        .bind(job.created_at)
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
        let query = format!(
            "SELECT id, protobuf_blob
             FROM jobs
             WHERE status = 'pending' AND (locked_until IS NULL OR locked_until < datetime('now')) AND run_after <= datetime('now') AND agent_role IN ({})
             ORDER BY created_at ASC LIMIT 1",
            roles_placeholders
        );

        let mut query_builder = sqlx::query(&query);
        for role in &roles {
            query_builder = query_builder.bind(role);
        }

        let row: Option<sqlx::sqlite::SqliteRow> = query_builder
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(row) = row {
            let id: String = row.get("id");
            let blob: Vec<u8> = row.get("protobuf_blob");

            sqlx::query("UPDATE jobs SET locked_until = datetime('now', '+5 minutes') WHERE id = ?")
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
        sqlx::query("UPDATE jobs SET status = 'completed' WHERE id = ?")
            .bind(job_id)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn fail(&self, job_id: &str, _reason: &str) -> Result<(), String> {
        sqlx::query("UPDATE jobs SET status = 'failed' WHERE id = ?")
            .bind(job_id)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
