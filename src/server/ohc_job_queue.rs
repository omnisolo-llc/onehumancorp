use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OhcJob {
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

#[async_trait]
pub trait OhcTaskQueue: Send + Sync {
    async fn enqueue(&self, job: OhcJob) -> Result<(), String>;
    async fn dequeue(&self, job_types: Vec<String>) -> Result<Option<OhcJob>, String>;
    async fn complete(&self, job_id: &str) -> Result<(), String>;
    async fn fail(&self, job_id: &str, reason: &str) -> Result<(), String>;
}

pub struct PgOhcJobQueue {
    pool: Arc<PgPool>,
}

impl PgOhcJobQueue {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OhcTaskQueue for PgOhcJobQueue {
    async fn enqueue(&self, job: OhcJob) -> Result<(), String> {
        let payload_json: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or(serde_json::Value::Null);

        sqlx::query(
            "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, retry_count, max_retries, next_retry_at)
             VALUES ($1, $2, $3, $4, 'PENDING', $5, $6, $7)"
        )
        .bind(&job.id)
        .bind(&job.tenant_id)
        .bind(&job.job_type)
        .bind(payload_json)
        .bind(job.retry_count)
        .bind(job.max_retries)
        .bind(job.next_retry_at)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn dequeue(&self, job_types: Vec<String>) -> Result<Option<OhcJob>, String> {
        if job_types.is_empty() {
            return Ok(None);
        }

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let type_placeholders = job_types.iter().enumerate().map(|(i, _)| format!("${}", i + 1)).collect::<Vec<_>>().join(",");
        let query_str = format!(
            "UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP
             WHERE id = (
                 SELECT id FROM ohc_job_queue
                 WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP AND job_type IN ({})
                 ORDER BY next_retry_at ASC, created_at ASC
                 LIMIT 1
                 FOR UPDATE SKIP LOCKED
             ) RETURNING id, tenant_id, job_type, payload, status, retry_count, max_retries, next_retry_at, created_at, updated_at",
            type_placeholders
        );

        let mut query = sqlx::query(&query_str);
        for t in &job_types {
            query = query.bind(t);
        }

        let job_opt = query.fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;

        if let Some(row) = job_opt {
            let payload_val: serde_json::Value = row.try_get("payload").unwrap_or(serde_json::Value::Null);
            let payload_str = serde_json::to_string(&payload_val).unwrap_or_default();

            let job = OhcJob {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                job_type: row.get("job_type"),
                payload: payload_str,
                status: row.get("status"),
                retry_count: row.get("retry_count"),
                max_retries: row.get("max_retries"),
                next_retry_at: row.get("next_retry_at"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };

            tx.commit().await.map_err(|e| e.to_string())?;
            return Ok(Some(job));
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(None)
    }

    async fn complete(&self, job_id: &str) -> Result<(), String> {
        sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(job_id)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn fail(&self, job_id: &str, _reason: &str) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let row = sqlx::query("SELECT retry_count, max_retries FROM ohc_job_queue WHERE id = $1 FOR UPDATE")
            .bind(job_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            let current_attempts: i32 = r.try_get("retry_count").unwrap_or(0);
            let max_attempts: i32 = r.try_get("max_retries").unwrap_or(3);
            let next_attempt = current_attempts + 1;

            if next_attempt >= max_attempts {
                // Dead-letter queue transition
                sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', retry_count = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2")
                    .bind(next_attempt)
                    .bind(job_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
            } else {
                // Exponential backoff
                let backoff_seconds = 1 << next_attempt;
                let new_run_after = chrono::Utc::now() + chrono::Duration::seconds(backoff_seconds as i64);
                sqlx::query("UPDATE ohc_job_queue SET status = 'PENDING', retry_count = $1, next_retry_at = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3")
                    .bind(next_attempt)
                    .bind(new_run_after)
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
