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


use super::models::*;
pub struct PostgresTaskQueue {
    pool: sqlx::PgPool,
}


impl PostgresTaskQueue {
    pub fn new(pool: sqlx::PgPool) -> Self {
        PostgresTaskQueue { pool }
    }
}


#[async_trait]
impl TaskQueue for PostgresTaskQueue {
    async fn requeue(&self, job: Job) -> Result<(), String> {
        let mut payload_map: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or_else(|_| serde_json::json!({}));
        payload_map["attempts"] = serde_json::json!(job.attempts);
        payload_map["max_attempts"] = serde_json::json!(job.max_attempts);
        let new_payload = serde_json::to_string(&payload_map).unwrap_or_default();

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        set_org_context(&mut *tx, &job.tenant_id).await.map_err(|e| e.to_string())?;
        sqlx::query("UPDATE sub_agent_queue SET status = 'QUEUED', payload = $3, scheduled_at = $4 WHERE id = $1 AND tenant_id = $2")
            .bind(&job.id)
            .bind(&job.tenant_id)
            .bind(new_payload)
            .bind(job.run_after)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn enqueue_batch(&self, jobs: Vec<Job>) -> Result<(), String> {
        if jobs.is_empty() { return Ok(()); }
        let mut builder = sqlx::QueryBuilder::new("INSERT INTO sub_agent_queue (id, tenant_id, parent_task_id, payload, status, scheduled_at) ");
        builder.push_values(jobs.into_iter(), |mut b, job| {
            let run_after = job.run_after;
            let mut payload_map: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or_else(|_| serde_json::json!({}));
            payload_map["agent_role"] = serde_json::Value::String(job.agent_role.clone());
            payload_map["attempts"] = serde_json::json!(job.attempts);
            payload_map["max_attempts"] = serde_json::json!(job.max_attempts);
            let new_payload = serde_json::to_string(&payload_map).unwrap_or_default();
            let org_id = if job.tenant_id.is_empty() {
                payload_map["tenant_id"].as_str().unwrap_or("").to_string()
            } else {
                job.tenant_id.clone()
            };
            b.push_bind(job.id)
             .push_bind(org_id)
             .push_bind(job.parent_task_id)
             .push_bind(new_payload)
             .push_bind("PENDING")
             .push_bind(run_after);
        });
        builder.build().execute(&self.pool).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let run_after = job.run_after;

        let mut payload_map: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or_else(|_| serde_json::json!({}));
        payload_map["agent_role"] = serde_json::Value::String(job.agent_role.clone());
        payload_map["attempts"] = serde_json::json!(job.attempts);
        payload_map["max_attempts"] = serde_json::json!(job.max_attempts);

        let new_payload = serde_json::to_string(&payload_map).unwrap_or_default();

        let org_id = if job.tenant_id.is_empty() {
            payload_map["tenant_id"].as_str().unwrap_or("").to_string()
        } else {
            job.tenant_id.clone()
        };

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        set_org_context(&mut *tx, &org_id).await.map_err(|e| e.to_string())?;
        sqlx::query("INSERT INTO sub_agent_queue (id, tenant_id, parent_task_id, payload, status, scheduled_at) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(job.id)
            .bind(org_id)
            .bind(job.parent_task_id)
            .bind(new_payload)
            .bind("PENDING")
            .bind(run_after)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn dequeue(&self, roles: Vec<String>) -> Result<Option<Job>, String> {
        if roles.is_empty() { return Ok(None); }
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        sqlx::query("SET LOCAL ROLE ohc_bypassrls").execute(&mut *tx).await.map_err(|e| e.to_string())?;
        let row = sqlx::query("UPDATE sub_agent_queue SET status = 'RUNNING' WHERE id = (SELECT id FROM sub_agent_queue WHERE status = 'PENDING' AND scheduled_at <= CURRENT_TIMESTAMP AND payload::json->>'agent_role' = ANY($1) ORDER BY scheduled_at ASC LIMIT 1 FOR UPDATE SKIP LOCKED) RETURNING id, tenant_id, parent_task_id, payload, status, scheduled_at")
            .bind(&roles)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        tx.commit().await.map_err(|e| e.to_string())?;

        if let Some(row) = row {
            let id: String = row.get("id");
            let tenant_id: String = row.get("tenant_id");
            let parent_task_id: String = row.get("parent_task_id");
            let payload: String = row.get("payload");
            let status: String = row.get("status");
            let scheduled_at: DateTime<Utc> = row.get("scheduled_at");

            let mut j = Job {
                id,
                tenant_id: tenant_id,
                parent_task_id,
                agent_role: String::new(),
                payload: payload.clone(),
                status,
                attempts: 0,
                max_attempts: 3,
                run_after: scheduled_at,
                locked_until: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            let payload_map: serde_json::Value = serde_json::from_str(&payload).unwrap_or_else(|_| serde_json::json!({}));
            if let Some(role) = payload_map["agent_role"].as_str() {
                j.agent_role = role.to_string();
            }
            if let Some(attempts) = payload_map["attempts"].as_i64() {
                j.attempts = attempts as i32;
            }
            if let Some(max_attempts) = payload_map["max_attempts"].as_i64() {
                j.max_attempts = max_attempts as i32;
            }

            j.attempts += 1;

            Ok(Some(j))
        } else {
            Ok(None)
        }
    }

    async fn complete(&self, job_id: &str, tenant_id: &str) -> Result<(), String> {
        sqlx::query("UPDATE sub_agent_queue SET status = 'COMPLETED', completed_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
            .bind(job_id)
            .bind(tenant_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn fail(&self, job_id: &str, tenant_id: &str, reason: &str) -> Result<(), String> {
        let error_payload = serde_json::to_string(&serde_json::json!({"error": reason}))
            .unwrap_or_else(|_| "{}".to_string());
        sqlx::query("UPDATE sub_agent_queue SET status = 'FAILED', payload = COALESCE(payload::jsonb, '{}'::jsonb) || $2::jsonb WHERE id = $1 AND tenant_id = $3")
            .bind(job_id)
            .bind(error_payload)
            .bind(tenant_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
