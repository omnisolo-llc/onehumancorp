use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OHCJob {
    pub id: String,
    pub tenant_id: String,
    pub parent_task_id: Option<String>,
    pub job_type: String,
    pub payload: String,
    pub status: String,
    pub retry_count: i32,
    pub max_retries: i32,
    pub next_retry_at: DateTime<Utc>,
    pub locked_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct OHCJobQueue {
    pool: Arc<PgPool>,
}

impl OHCJobQueue {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn enqueue(&self, tenant_id: &str, job_type: &str, payload: &serde_json::Value, parent_task_id: Option<&str>, max_retries: i32) -> Result<String, String> {
        let job_id = Uuid::new_v4().to_string();
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO ohc_job_queue (id, tenant_id, parent_task_id, job_type, payload, status, max_retries, next_retry_at)
             VALUES ($1, $2, $3, $4, $5, 'PENDING', $6, CURRENT_TIMESTAMP)"
        )
        .bind(&job_id)
        .bind(tenant_id)
        .bind(parent_task_id)
        .bind(job_type)
        .bind(payload)
        .bind(max_retries)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(job_id)
    }

    pub async fn dequeue(&self, job_types: Vec<&str>) -> Result<Option<OHCJob>, String> {
        if job_types.is_empty() {
            return Ok(None);
        }

        // We use system context to fetch from any tenant, but only those matching job_types
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_system_context(&mut *tx).await.map_err(|e| e.to_string())?;

        let type_placeholders = job_types.iter().enumerate().map(|(i, _)| format!("${}", i + 1)).collect::<Vec<_>>().join(",");
        let query_str = format!(
            "UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP
             WHERE id = (
                 SELECT id FROM ohc_job_queue
                 WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP AND job_type IN ({})
                 ORDER BY next_retry_at ASC, created_at ASC
                 LIMIT 1
                 FOR UPDATE SKIP LOCKED
             ) RETURNING id, tenant_id, parent_task_id, job_type, payload, status, retry_count, max_retries, next_retry_at, locked_until, created_at, updated_at",
            type_placeholders
        );

        let mut query = sqlx::query(&query_str);
        for jt in &job_types {
            query = query.bind(jt);
        }

        let job_opt = query.fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;

        if let Some(row) = job_opt {
            use sqlx::Row;
            let payload_val: serde_json::Value = row.try_get("payload").unwrap_or(serde_json::Value::Null);
            let payload_str = serde_json::to_string(&payload_val).unwrap_or_default();

            let job = OHCJob {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                parent_task_id: row.try_get("parent_task_id").unwrap_or(None),
                job_type: row.get("job_type"),
                payload: payload_str,
                status: row.get("status"),
                retry_count: row.try_get("retry_count").unwrap_or(0),
                max_retries: row.try_get("max_retries").unwrap_or(3),
                next_retry_at: row.try_get("next_retry_at").unwrap_or_else(|_| chrono::Utc::now()),
                locked_until: row.try_get("locked_until").unwrap_or(None),
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

        sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(job_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn fail(&self, job_id: &str) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_system_context(&mut *tx).await.map_err(|e| e.to_string())?;

        let row = sqlx::query("SELECT retry_count, max_retries FROM ohc_job_queue WHERE id = $1 FOR UPDATE")
            .bind(job_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            use sqlx::Row;
            let current_retries: i32 = r.try_get("retry_count").unwrap_or(0);
            let max_retries: i32 = r.try_get("max_retries").unwrap_or(3);
            let next_retry = current_retries + 1;

            if next_retry >= max_retries {
                // Dead letter
                sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', retry_count = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2")
                    .bind(next_retry)
                    .bind(job_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
            } else {
                // Exponential backoff
                let backoff_seconds = 1 << next_retry;
                let new_run_after = (chrono::Utc::now() + chrono::Duration::seconds(backoff_seconds as i64)).to_rfc3339();
                sqlx::query("UPDATE ohc_job_queue SET status = 'PENDING', retry_count = $1, next_retry_at = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3")
                    .bind(next_retry)
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
