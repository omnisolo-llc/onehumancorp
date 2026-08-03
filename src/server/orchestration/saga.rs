use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum SagaStatus {
    Running,
    Completed,
    Compensating,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SagaStepStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Compensated,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SagaExecution {
    pub id: Uuid,
    pub tenant_id: String,
    pub saga_type: String,
    pub status: String,
    pub context: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SagaStep {
    pub id: Uuid,
    pub saga_id: Uuid,
    pub tenant_id: String,
    pub step_name: String,
    pub agent_type: String,
    pub status: String,
    pub retry_count: i32,
}

pub struct SagaOrchestrator {
    pool: PgPool,
}

impl SagaOrchestrator {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn start_saga(
        &self,
        tenant_id: &str,
        saga_type: &str,
        context: serde_json::Value,
    ) -> Result<Uuid, Box<dyn std::error::Error + Send + Sync>> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind::<&str>(tenant_id)
            .execute(&mut *tx)
            .await?;

        let id: Uuid = sqlx::query_scalar(
            "INSERT INTO saga_executions (tenant_id, saga_type, status, context)
             VALUES ($1, $2, 'running', $3)
             RETURNING id"
        )
        .bind(tenant_id)
        .bind(saga_type)
        .bind(context)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(id)
    }

    pub async fn add_step(
        &self,
        tenant_id: &str,
        saga_id: Uuid,
        step_name: &str,
        agent_type: &str,
    ) -> Result<Uuid, Box<dyn std::error::Error + Send + Sync>> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind::<&str>(tenant_id)
            .execute(&mut *tx)
            .await?;

        let id: Uuid = sqlx::query_scalar(
            "INSERT INTO saga_steps (saga_id, tenant_id, step_name, agent_type, status)
             VALUES ($1, $2, $3, $4, 'pending')
             RETURNING id"
        )
        .bind(saga_id)
        .bind(tenant_id)
        .bind(step_name)
        .bind(agent_type)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(id)
    }

    pub async fn complete_step(
        &self,
        tenant_id: &str,
        step_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind::<&str>(tenant_id)
            .execute(&mut *tx)
            .await?;

        sqlx::query(
            "UPDATE saga_steps SET status = 'completed', updated_at = CURRENT_TIMESTAMP
             WHERE id = $1 AND tenant_id = $2"
        )
        .bind(step_id)
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn fail_step(
        &self,
        tenant_id: &str,
        step_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
         let mut tx = self.pool.begin().await?;

        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind::<&str>(tenant_id)
            .execute(&mut *tx)
            .await?;

        sqlx::query(
            "UPDATE saga_steps SET status = 'failed', updated_at = CURRENT_TIMESTAMP
             WHERE id = $1 AND tenant_id = $2"
        )
        .bind(step_id)
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

        // Also mark the saga as compensating
        sqlx::query(
             "UPDATE saga_executions
              SET status = 'compensating', updated_at = CURRENT_TIMESTAMP
              WHERE id = (SELECT saga_id FROM saga_steps WHERE id = $1)"
        )
        .bind(step_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

     pub async fn complete_saga(
        &self,
        tenant_id: &str,
        saga_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind::<&str>(tenant_id)
            .execute(&mut *tx)
            .await?;

        sqlx::query(
            "UPDATE saga_executions SET status = 'completed', updated_at = CURRENT_TIMESTAMP
             WHERE id = $1 AND tenant_id = $2"
        )
        .bind(saga_id)
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }
}
