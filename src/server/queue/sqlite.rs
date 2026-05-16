#![allow(dead_code)]


use async_trait::async_trait;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use std::collections::VecDeque;
use std::sync::Mutex;

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use sqlx::Row;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
use super::models::*;
pub struct SqliteTaskQueue {
    pool: sqlx::SqlitePool,
}


impl SqliteTaskQueue {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        SqliteTaskQueue { pool }
    }

    pub async fn init(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS local_queue_jobs (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                task_id TEXT NOT NULL,
                role TEXT NOT NULL,
                payload BLOB,
                status TEXT DEFAULT 'PENDING'
            );"
        ).execute(&self.pool).await?;
        Ok(())
    }
}


#[async_trait]
impl TaskQueue for SqliteTaskQueue {
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
        // Here job.payload is a String but in the SQLite table it's BLOB,
        // we can store it as text since SQLite handles it loosely or cast it.
        sqlx::query("INSERT INTO local_queue_jobs (id, tenant_id, task_id, role, payload) VALUES (?, ?, ?, ?, ?)")
            .bind(job.id)
            .bind(job.tenant_id)
            .bind(job.parent_task_id)
            .bind(job.agent_role)
            .bind(job.payload.as_bytes())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn dequeue(&self, roles: Vec<String>) -> Result<Option<Job>, String> {
        if roles.is_empty() { return Ok(None); }

        // SQLite doesn't support SELECT ... FOR UPDATE SKIP LOCKED.
        // To avoid SQLITE_BUSY lock-upgrade errors when claiming tasks in SQLite, execute an atomic UPDATE ... RETURNING query
        let role_placeholders = roles.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query_str = format!(
            "UPDATE local_queue_jobs SET status = 'RUNNING' WHERE id = (SELECT id FROM local_queue_jobs WHERE status = 'PENDING' AND role IN ({}) LIMIT 1) RETURNING id, tenant_id, task_id, role, payload, status",
            role_placeholders
        );

        let mut query = sqlx::query(&query_str);
        for role in &roles {
            query = query.bind(role);
        }

        let row = query.fetch_optional(&self.pool).await.map_err(|e| e.to_string())?;

        if let Some(row) = row {
            use sqlx::Row;
            let id: String = row.get("id");
            let tenant_id: String = row.get("tenant_id");
            let task_id: String = row.get("task_id");
            let role: String = row.get("role");
            let payload: Vec<u8> = row.get("payload");

            Ok(Some(Job {
                id,
                tenant_id,
                parent_task_id: task_id,
                agent_role: role,
                payload: String::from_utf8(payload).unwrap_or_default(),
                status: "RUNNING".to_string(),
                attempts: 1,
                max_attempts: 3,
                run_after: Utc::now(),
                locked_until: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }))
        } else {
            Ok(None)
        }
    }

    async fn complete(&self, job_id: &str, tenant_id: &str) -> Result<(), String> {
        sqlx::query("UPDATE local_queue_jobs SET status = 'COMPLETED' WHERE id = ? AND tenant_id = ?")
            .bind(job_id)
            .bind(tenant_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn fail(&self, job_id: &str, tenant_id: &str, _reason: &str) -> Result<(), String> {
        sqlx::query("UPDATE local_queue_jobs SET status = 'FAILED' WHERE id = ? AND tenant_id = ?")
            .bind(job_id)
            .bind(tenant_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn requeue(&self, job: Job) -> Result<(), String> {
        let mut payload_map: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or_else(|_| serde_json::json!({}));
        payload_map["attempts"] = serde_json::json!(job.attempts);
        payload_map["max_attempts"] = serde_json::json!(job.max_attempts);
        let new_payload = serde_json::to_string(&payload_map).unwrap_or_default();

        sqlx::query("UPDATE local_queue_jobs SET status = 'PENDING', payload = ? WHERE id = ? AND tenant_id = ?")
            .bind(new_payload)
            .bind(&job.id)
            .bind(&job.tenant_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
