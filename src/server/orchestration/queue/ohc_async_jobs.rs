use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OHCAsyncJob {
    pub id: String,
    pub tenant_id: String,
    pub event_type: String,
    pub payload: String,
    pub status: String,
    pub retry_count: i32,
    pub next_retry_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct OHCAsyncJobQueue {
    pool: Arc<PgPool>,
}

impl OHCAsyncJobQueue {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn enqueue(&self, tenant_id: &str, event_type: &str, payload: &serde_json::Value) -> Result<String, String> {
        let job_id = Uuid::new_v4().to_string();
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO ohc_async_jobs (id, tenant_id, event_type, payload, status, next_retry_at)
             VALUES ($1, $2, $3, $4, 'PENDING', CURRENT_TIMESTAMP)"
        )
        .bind(&job_id)
        .bind(tenant_id)
        .bind(event_type)
        .bind(payload)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(job_id)
    }

    pub async fn dequeue(&self, event_types: Vec<&str>) -> Result<Option<OHCAsyncJob>, String> {
        if event_types.is_empty() {
            return Ok(None);
        }

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_system_context(&mut *tx).await.map_err(|e| e.to_string())?;

        let type_placeholders = event_types.iter().enumerate().map(|(i, _)| format!("${}", i + 1)).collect::<Vec<_>>().join(",");
        let query_str = format!(
            "UPDATE ohc_async_jobs SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP
             WHERE id = (
                 SELECT id FROM ohc_async_jobs
                 WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP AND event_type IN ({})
                 ORDER BY next_retry_at ASC, created_at ASC
                 LIMIT 1
                 FOR UPDATE SKIP LOCKED
             ) RETURNING id, tenant_id, event_type, payload, status, retry_count, next_retry_at, created_at, updated_at",
            type_placeholders
        );

        let mut query = sqlx::query(&query_str);
        for et in &event_types {
            query = query.bind(et);
        }

        let job_opt = tokio::time::timeout(std::time::Duration::from_secs(60), query.fetch_optional(&mut *tx)).await.map_err(|_| "Timeout fetching job from queue".to_string())?.map_err(|e| e.to_string())?;

        if let Some(row) = job_opt {
            use sqlx::Row;
            let payload_val: serde_json::Value = row.try_get("payload").unwrap_or(serde_json::Value::Null);
            let payload_str = serde_json::to_string(&payload_val).unwrap_or_default();

            let job = OHCAsyncJob {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                event_type: row.get("event_type"),
                payload: payload_str,
                status: row.get("status"),
                retry_count: row.try_get("retry_count").unwrap_or(0),
                next_retry_at: row.try_get("next_retry_at").unwrap_or_else(|_| chrono::Utc::now()),
                created_at: row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now()),
            };

            tx.commit().await.map_err(|e| e.to_string())?;
            return Ok(Some(job));
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(None)
    }

    pub async fn complete(&self, job_id: &str) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_system_context(&mut *tx).await.map_err(|e| e.to_string())?;

        sqlx::query("UPDATE ohc_async_jobs SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(job_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn fail(&self, job_id: &str, max_retries: i32, reason: &str) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_system_context(&mut *tx).await.map_err(|e| e.to_string())?;

        let row = sqlx::query("SELECT retry_count, tenant_id, payload FROM ohc_async_jobs WHERE id = $1 FOR UPDATE")
            .bind(job_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            use sqlx::Row;
            let current_retries: i32 = r.try_get("retry_count").unwrap_or(0);
            let next_retry = current_retries + 1;

            if next_retry >= max_retries {
                // Dead letter
                let tenant_id: String = r.try_get("tenant_id").unwrap_or_default();
                let payload: serde_json::Value = r.try_get("payload").unwrap_or_else(|_| serde_json::json!({}));
                let payload_str = serde_json::to_string(&payload).unwrap_or_default();
                sqlx::query("INSERT INTO department_dead_letters (id, tenant_id, event_type, department, payload, error_message) VALUES ($1, $2, $3, $4, $5, $6)")
                    .bind(job_id)
                    .bind(&tenant_id)
                    .bind("async_job_failed")
                    .bind("async_queue")
                    .bind(&payload_str)
                    .bind(reason)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                sqlx::query("UPDATE ohc_async_jobs SET status = 'FAILED', retry_count = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2")
                    .bind(next_retry)
                    .bind(job_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
            } else {
                // Exponential backoff
                let backoff_seconds = 1 << next_retry;
                let new_run_after = (chrono::Utc::now() + chrono::Duration::seconds(backoff_seconds as i64)).to_rfc3339();
                sqlx::query("UPDATE ohc_async_jobs SET status = 'PENDING', retry_count = $1, next_retry_at = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3")
                    .bind(next_retry)
                    .bind(&new_run_after)
                    .bind(job_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
