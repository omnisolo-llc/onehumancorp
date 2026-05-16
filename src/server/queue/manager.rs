use ::server_common::auth_utils::set_org_context;
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
use ::server_common::auth_utils::set_org_context;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

use super::models::*;
use super::worker::*;
use super::memory::*;
use super::postgres::*;
use super::sqlite::*;
use super::redis::*;
pub struct QueueManager {
    pool: sqlx::PgPool,
}


impl QueueManager {
    pub fn new(pool: sqlx::PgPool) -> Self {
        QueueManager { pool }
    }

    pub async fn enqueue(&self, job: SubAgentJob) -> Result<(), sqlx::Error> {
        let payload_str = serde_json::to_string(&job.payload).unwrap_or_default();

        let mut tx = self.pool.begin().await?;
        set_org_context(&mut *tx, &job.tenant_id).await?;
        sqlx::query("INSERT INTO sub_agent_queue (id, tenant_id, parent_task_id, payload, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(job.id)
            .bind(job.tenant_id)
            .bind(job.parent_task_id)
            .bind(payload_str)
            .bind("QUEUED")
            .bind(job.created_at)
            .bind(job.updated_at)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn poll(&self, worker_id: &str) -> Result<Option<SubAgentJob>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SET LOCAL ROLE ohc_bypassrls").execute(&mut *tx).await?;
        let row = sqlx::query("UPDATE sub_agent_queue SET status = 'RUNNING', worker_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = (SELECT id FROM sub_agent_queue WHERE status = 'QUEUED' AND (scheduled_at IS NULL OR scheduled_at <= CURRENT_TIMESTAMP) ORDER BY created_at ASC LIMIT 1 FOR UPDATE SKIP LOCKED) RETURNING id, tenant_id, parent_task_id, payload, status, worker_id, created_at, updated_at")
            .bind(worker_id)
            .fetch_optional(&mut *tx)
            .await?;
        tx.commit().await?;

        if let Some(row) = row {
            let payload_str: String = row.get("payload");
            let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));

            Ok(Some(SubAgentJob {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                parent_task_id: row.get("parent_task_id"),
                payload,
                status: row.get("status"),
                worker_id: row.get("worker_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn mark_completed(&self, job_id: &str, tenant_id: &str) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        set_org_context(&mut *tx, tenant_id).await?;
        sqlx::query("UPDATE sub_agent_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
            .bind(job_id)
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }


    pub async fn requeue(&self, job_id: &str, tenant_id: &str, payload: serde_json::Value) -> Result<(), sqlx::Error> {
        let payload_str = serde_json::to_string(&payload).unwrap_or_default();
        // Since SubAgentJob's polling uses `status = 'QUEUED'`, and some implementations might not filter by scheduled_at,
        // we can still add a simple delay by using tokio::time::sleep here or rely on the caller to backoff,
        // or actually update the scheduled_at column if the poll query respects it.
        // Wait, QueueManager::poll does: `SELECT id FROM sub_agent_queue WHERE status = 'QUEUED' ORDER BY created_at ASC`
        // It does NOT use `scheduled_at`!
        // To implement a true backoff, we need to add `AND (scheduled_at IS NULL OR scheduled_at <= CURRENT_TIMESTAMP)`.

        // Update the row.
        let mut tx = self.pool.begin().await?;
        set_org_context(&mut *tx, tenant_id).await?;
        sqlx::query("UPDATE sub_agent_queue SET status = 'QUEUED', payload = $3, updated_at = CURRENT_TIMESTAMP, scheduled_at = CURRENT_TIMESTAMP + INTERVAL '5 seconds' WHERE id = $1 AND tenant_id = $2")
            .bind(job_id)
            .bind(tenant_id)
            .bind(payload_str)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn mark_failed(&self, job_id: &str, _reason: &str, tenant_id: &str) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        set_org_context(&mut *tx, tenant_id).await?;
        sqlx::query("UPDATE sub_agent_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
            .bind(job_id)
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn start_polling<F, Fut>(&self, worker_id: &str, interval: Duration, handler: F, mut shutdown_rx: tokio::sync::broadcast::Receiver<()>)
    where
        F: Fn(SubAgentJob) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<(), String>> + Send + 'static,
    {
        let mut interval = tokio::time::interval(interval);

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    loop {
                        match self.poll(worker_id).await {
                            Ok(Some(job)) => {
                                tracing::debug!("QueueManager dispatched job: {}", job.id);
                                let mut attempts = job.payload.get("attempts").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                                let max_attempts = job.payload.get("max_attempts").and_then(|v| v.as_i64()).unwrap_or(3) as i32;
                                attempts += 1;
                                let handle_res = tokio::time::timeout(tokio::time::Duration::from_secs(60), handler(job.clone())).await;
                                let handler_res = match handle_res {
                                    Ok(Ok(())) => Ok(()),
                                    Ok(Err(e)) => Err(e),
                                    Err(_) => Err("Timeout executing job".to_string()),
                                };
                                match handler_res {
                                    Ok(_) => {
                                        tracing::info!("Job handler succeeded: {}", job.id);
                                        let _ = self.mark_completed(&job.id, &job.tenant_id).await;
                                    }
                                    Err(e) => {
                                        tracing::error!("Job handler failed: {}, error: {}", job.id, e);
                                        if attempts < max_attempts {
                                            let mut retry_job = job.clone();
                                            retry_job.payload["attempts"] = serde_json::json!(attempts);
                                            retry_job.payload["max_attempts"] = serde_json::json!(max_attempts);
                                            let _ = self.requeue(&job.id, &job.tenant_id, retry_job.payload).await;
                                        } else {
                                            let _ = self.mark_failed(&job.id, &e, &job.tenant_id).await;
                                        }
                                    }
                                }
                            }
                            Ok(None) => {
                                break;
                            }
                            Err(e) => {
                                tracing::error!("Failed to poll queue: {}", e);
                                break;
                            }
                        }
                    }
                }
                _ = shutdown_rx.recv() => {
                    tracing::info!("QueueManager polling shutting down");
                    break;
                }
            }
        }
    }
}


pub struct TaskQueueService {
    pool: sqlx::PgPool,
}


impl TaskQueueService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        TaskQueueService { pool }
    }

    pub async fn push_task(&self, task: SharedTaskModel) -> Result<(), sqlx::Error> {
        let payload_str = serde_json::to_string(&task.payload).unwrap_or_default();
        let deps_str = serde_json::to_string(&task.dependencies).unwrap_or_else(|_| "[]".to_string());

        let mut tx = self.pool.begin().await?;
        set_org_context(&mut *tx, &task.tenant_id).await?;
        sqlx::query("INSERT INTO shared_tasks (id, parent_id, epic_id, title, status, assigned_agent, payload, tenant_id, dependencies) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9::jsonb)")
            .bind(task.id)
            .bind(task.parent_id)
            .bind(task.epic_id)
            .bind(task.title)
            .bind("PENDING")
            .bind(task.assigned_agent)
            .bind(payload_str)
            .bind(task.tenant_id)
            .bind(deps_str)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn claim_task(&self, agent_id: &str) -> Result<Option<SharedTaskModel>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SET LOCAL ROLE ohc_bypassrls").execute(&mut *tx).await?;
        let row = sqlx::query("UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent = $1 WHERE id = (SELECT st.id FROM shared_tasks st WHERE st.status = 'PENDING' AND (st.assigned_agent IS NULL OR st.assigned_agent = $1) AND NOT EXISTS (SELECT 1 FROM jsonb_array_elements_text(st.dependencies) AS dep_id JOIN shared_tasks parent ON parent.id::text = dep_id WHERE parent.status != 'COMPLETED') ORDER BY st.created_at ASC LIMIT 1 FOR UPDATE SKIP LOCKED) RETURNING id, tenant_id, parent_id, epic_id, title, status, assigned_agent, payload, dependencies::text AS dependencies, created_at, updated_at")
            .bind(agent_id)
            .fetch_optional(&mut *tx)
            .await?;
        tx.commit().await?;

        if let Some(row) = row {
            let payload_str: String = row.get("payload");
            let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));
            let deps_str: String = row.get("dependencies");
            let dependencies: serde_json::Value = serde_json::from_str(&deps_str).unwrap_or_else(|_| serde_json::json!([]));

            Ok(Some(SharedTaskModel {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                parent_id: row.get("parent_id"),
                epic_id: row.get("epic_id"),
                title: row.get("title"),
                status: row.get("status"),
                assigned_agent: row.get("assigned_agent"),
                payload,
                dependencies,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn complete_task(&self, task_id: &str) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        set_org_context(&mut *tx, "system").await?;
        sqlx::query("UPDATE shared_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(task_id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }



    pub async fn fail_task(&self, task_id: &str, reason: &str) -> Result<(), sqlx::Error> {
        let payload_update = serde_json::to_string(&serde_json::json!({"error": reason})).unwrap_or_else(|_| "{}".to_string());
        // We could merge this better using jsonb operators or just save status
        let mut tx = self.pool.begin().await?;
        set_org_context(&mut *tx, "system").await?;
        sqlx::query("UPDATE shared_tasks SET status = 'FAILED', payload = COALESCE(payload, '{}'::jsonb) || $2::jsonb, updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(task_id)
            .bind(payload_update)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }


    pub async fn get_completed_tasks(&self, limit: i64) -> Result<Vec<SharedTaskModel>, sqlx::Error> {
        let rows = sqlx::query("SELECT id, tenant_id, parent_id, epic_id, title, status, assigned_agent, payload, dependencies::text AS dependencies, created_at, updated_at FROM shared_tasks WHERE status = 'COMPLETED' LIMIT $1")
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;

        let mut tasks = Vec::new();
        for row in rows {
            let payload_str: String = row.get("payload");
            let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));
            let deps_str: String = row.get("dependencies");
            let dependencies: serde_json::Value = serde_json::from_str(&deps_str).unwrap_or_else(|_| serde_json::json!([]));

            tasks.push(SharedTaskModel {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                parent_id: row.get("parent_id"),
                epic_id: row.get("epic_id"),
                title: row.get("title"),
                status: row.get("status"),
                assigned_agent: row.get("assigned_agent"),
                payload,
                dependencies,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(tasks)
    }
}
