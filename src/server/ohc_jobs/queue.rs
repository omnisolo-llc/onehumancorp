use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Job {
    pub id: Uuid,
    pub tenant_id: String,
    pub job_type: String,
    pub payload: sqlx::types::Json<serde_json::Value>,
    pub status: String,
    pub retry_count: i32,
    pub next_retry_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct JobQueue {
    pool: PgPool,
}

impl JobQueue {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn enqueue(
        &self,
        tenant_id: &str,
        job_type: &str,
        payload: serde_json::Value,
    ) -> Result<Job, sqlx::Error> {
        let job: Job = sqlx::query_as!(
            Job,
            r#"
            INSERT INTO ohc_job_queue (tenant_id, job_type, payload)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, job_type, payload as "payload: sqlx::types::Json<serde_json::Value>", status, retry_count, next_retry_at, created_at, updated_at
            "#,
            tenant_id,
            job_type,
            sqlx::types::Json(payload) as _,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(job)
    }

    pub async fn dequeue(&self) -> Result<Option<Job>, sqlx::Error> {
        let job: Option<Job> = sqlx::query_as!(
            Job,
            r#"
            UPDATE ohc_job_queue
            SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP
            WHERE id = (
                SELECT id
                FROM ohc_job_queue
                WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP
                ORDER BY created_at ASC
                FOR UPDATE SKIP LOCKED
                LIMIT 1
            )
            RETURNING id, tenant_id, job_type, payload as "payload: sqlx::types::Json<serde_json::Value>", status, retry_count, next_retry_at, created_at, updated_at
            "#
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(job)
    }

    pub async fn complete(&self, job_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE ohc_job_queue
            SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            "#,
            job_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn fail(&self, job_id: Uuid, retry_delay: std::time::Duration) -> Result<(), sqlx::Error> {
        // Calculate next retry time
        // Exponential backoff logic should ideally be applied here based on retry_count
        let next_retry_at = Utc::now() + chrono::Duration::from_std(retry_delay).unwrap_or(chrono::Duration::seconds(60));

        sqlx::query!(
            r#"
            UPDATE ohc_job_queue
            SET status = CASE WHEN retry_count >= 3 THEN 'FAILED' ELSE 'PENDING' END,
                retry_count = retry_count + 1,
                next_retry_at = $2,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            "#,
            job_id,
            next_retry_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
