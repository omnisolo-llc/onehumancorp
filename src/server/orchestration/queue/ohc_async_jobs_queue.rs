use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OHCAsyncJob {
    pub id: String,
    pub tenant_id: String,
    pub job_type: String,
    pub payload: String,
    pub status: String,
    pub retry_count: i32,
    pub max_retries: i32,
    pub next_retry_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct OHCAsyncJobsQueue {
    pool: Arc<PgPool>,
}

impl OHCAsyncJobsQueue {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn enqueue(&self, tenant_id: &str, job_type: &str, payload: &serde_json::Value) -> Result<String, String> {
        let job_id = Uuid::new_v4().to_string();
        let payload_str = serde_json::to_string(payload).unwrap_or_else(|_| "{}".to_string());
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO ohc_async_jobs (id, tenant_id, job_type, payload, status, retry_count, max_retries, next_retry_at, created_at, updated_at)
             VALUES ($1, $2, $3, $4::jsonb, 'PENDING', 0, 3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
        )
        .bind(&job_id)
        .bind(tenant_id)
        .bind(job_type)
        .bind(&payload_str)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        ::server_telemetry::record_queue_length_sync(1, ::server_telemetry::get_deployment_mode());
        Ok(job_id)
    }

    pub async fn dequeue(&self, job_types: Vec<&str>) -> Result<Option<OHCAsyncJob>, String> {
        if job_types.is_empty() {
            return Ok(None);
        }

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_system_context(&mut *tx).await.map_err(|e| e.to_string())?;

        let type_placeholders = job_types.iter().enumerate().map(|(i, _)| format!("${}", i + 1)).collect::<Vec<_>>().join(",");
        let query_str = format!(
            "UPDATE ohc_async_jobs SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP
             WHERE id = (
                 SELECT id FROM ohc_async_jobs
                 WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP AND job_type IN ({})
                 ORDER BY next_retry_at ASC, created_at ASC
                 LIMIT 1
                 FOR UPDATE SKIP LOCKED
             ) RETURNING id, tenant_id, job_type, payload, status, retry_count, max_retries, next_retry_at, created_at, updated_at",
            type_placeholders
        );

        let mut query = sqlx::query(&query_str);
        for jt in &job_types {
            query = query.bind(jt);
        }

        let job_opt = tokio::time::timeout(std::time::Duration::from_secs(60), query.fetch_optional(&mut *tx))
            .await
            .map_err(|_| "Timeout fetching job from queue".to_string())?
            .map_err(|e| e.to_string())?;

        if let Some(row) = job_opt {
            ::server_telemetry::record_queue_length_sync(-1, ::server_telemetry::get_deployment_mode());
            use sqlx::Row;
            let payload_val: serde_json::Value = row.try_get("payload").unwrap_or(serde_json::Value::Null);
            let payload_str = serde_json::to_string(&payload_val).unwrap_or_default();

            let job = OHCAsyncJob {
                id: row.try_get("id").unwrap_or_default(),
                tenant_id: row.try_get("tenant_id").unwrap_or_default(),
                job_type: row.try_get("job_type").unwrap_or_default(),
                payload: payload_str,
                status: row.try_get("status").unwrap_or_default(),
                retry_count: row.try_get("retry_count").unwrap_or(0),
                max_retries: row.try_get("max_retries").unwrap_or(3),
                next_retry_at: row.try_get("next_retry_at").unwrap_or_else(|_| Utc::now()),
                created_at: row.try_get("created_at").unwrap_or_else(|_| Utc::now()),
                updated_at: row.try_get("updated_at").unwrap_or_else(|_| Utc::now()),
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

        let _ = sqlx::query("UPDATE ohc_async_jobs SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(job_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn fail(&self, job_id: &str, reason: &str) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_system_context(&mut *tx).await.map_err(|e| e.to_string())?;

        let row = sqlx::query("SELECT retry_count, max_retries, tenant_id, payload FROM ohc_async_jobs WHERE id = $1 FOR UPDATE")
            .bind(job_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            use sqlx::Row;
            let current_retry_count: i32 = r.try_get("retry_count").unwrap_or(0);
            let max_retries: i32 = r.try_get("max_retries").unwrap_or(3);
            let next_attempt = current_retry_count + 1;

            if next_attempt >= max_retries {
                // Poison pill -> mark FAILED and send to dead letter
                let tenant_id: String = r.try_get("tenant_id").unwrap_or_default();
                let payload: serde_json::Value = r.try_get("payload").unwrap_or_else(|_| serde_json::json!({}));
                let payload_str = serde_json::to_string(&payload).unwrap_or_default();

                sqlx::query("INSERT INTO department_dead_letters (id, tenant_id, event_type, department, payload, error_message) VALUES ($1, $2, $3, $4, $5, $6)")
                    .bind(job_id)
                    .bind(&tenant_id)
                    .bind("async_job_failed")
                    .bind("ohc_async_jobs")
                    .bind(&payload_str)
                    .bind(reason)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                sqlx::query("UPDATE ohc_async_jobs SET status = 'FAILED', retry_count = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2")
                    .bind(next_attempt)
                    .bind(job_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
            } else {
                // Exponential backoff
                let backoff_seconds = 1 << next_attempt;
                let new_next_retry_at = Utc::now() + chrono::Duration::seconds(backoff_seconds as i64);

                sqlx::query("UPDATE ohc_async_jobs SET status = 'PENDING', retry_count = $1, next_retry_at = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3")
                    .bind(next_attempt)
                    .bind(new_next_retry_at)
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
