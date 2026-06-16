use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct AgentJob {
    pub id: uuid::Uuid,
    pub tenant_id: String,
    pub job_type: String,
    pub payload: Value,
    pub status: String,
    pub attempts: i32,
    pub max_attempts: i32,
    pub run_at: DateTime<Utc>,
    pub locked_at: Option<DateTime<Utc>>,
    pub locked_by: Option<String>,
}

#[async_trait]
pub trait AgentJobQueue: Send + Sync {
    async fn enqueue(&self, job: AgentJob) -> Result<(), sqlx::Error>;
    async fn dequeue(&self, worker_id: &str) -> Result<Option<AgentJob>, sqlx::Error>;
    async fn complete(&self, id: uuid::Uuid, tenant_id: &str) -> Result<(), sqlx::Error>;
    async fn fail(&self, id: uuid::Uuid, tenant_id: &str) -> Result<(), sqlx::Error>;
}

pub struct PostgresAgentJobQueue {
    pool: sqlx::PgPool,
}

impl PostgresAgentJobQueue {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AgentJobQueue for PostgresAgentJobQueue {
    async fn enqueue(&self, job: AgentJob) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &job.tenant_id).await.map_err(|e| sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        sqlx::query(
            r#"
            INSERT INTO agent_jobs (id, tenant_id, job_type, payload, status, attempts, max_attempts, run_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(job.id)
        .bind(&job.tenant_id)
        .bind(&job.job_type)
        .bind(&job.payload)
        .bind(&job.status)
        .bind(job.attempts)
        .bind(job.max_attempts)
        .bind(job.run_at)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    async fn dequeue(&self, worker_id: &str) -> Result<Option<AgentJob>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SET LOCAL ROLE ohc_bypassrls").execute(&mut *tx).await?;

        let row = sqlx::query(
            r#"
            UPDATE agent_jobs
            SET status = 'processing', locked_at = now(), locked_by = $1, attempts = attempts + 1
            WHERE id = (
                SELECT id FROM agent_jobs
                WHERE status = 'pending' AND run_at <= now()
                ORDER BY run_at ASC
                FOR UPDATE SKIP LOCKED
                LIMIT 1
            ) RETURNING *
            "#
        )
        .bind(worker_id)
        .fetch_optional(&mut *tx)
        .await?;

        tx.commit().await?;

        if let Some(row) = row {
            use sqlx::Row;
            Ok(Some(AgentJob {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                job_type: row.get("job_type"),
                payload: row.get("payload"),
                status: row.get("status"),
                attempts: row.get("attempts"),
                max_attempts: row.get("max_attempts"),
                run_at: row.get("run_at"),
                locked_at: row.get("locked_at"),
                locked_by: row.get("locked_by"),
            }))
        } else {
            Ok(None)
        }
    }

    async fn complete(&self, id: uuid::Uuid, tenant_id: &str) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        sqlx::query(
            r#"
            UPDATE agent_jobs
            SET status = 'completed'
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    async fn fail(&self, id: uuid::Uuid, tenant_id: &str) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        // get current attempts
        let row = sqlx::query(
            r#"
            SELECT attempts, max_attempts FROM agent_jobs WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .fetch_one(&mut *tx)
        .await?;

        use sqlx::Row;
        let attempts: i32 = row.get("attempts");
        let max_attempts: i32 = row.get("max_attempts");

        let status = if attempts >= max_attempts {
            "dead_letter"
        } else {
            "pending"
        };

        // Exponential backoff
        let run_at = if status == "pending" {
            let delay_seconds = 2_i64.pow(attempts as u32);
            Utc::now() + chrono::Duration::seconds(delay_seconds)
        } else {
            Utc::now()
        };

        sqlx::query(
            r#"
            UPDATE agent_jobs
            SET status = $3, run_at = $4
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(status)
        .bind(run_at)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }
}
