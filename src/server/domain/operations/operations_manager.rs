use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{PgPool, Postgres, Transaction};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OperationsError {
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),
    #[error("Unauthorized or missing tenant ID")]
    Unauthorized,
    #[error("Failed to execute action: {0}")]
    ExecutionError(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionIntent {
    pub tenant_id: String,
    pub action_type: String,
    pub payload: Value,
}

pub struct OperationsManager {
    pool: PgPool,
}

impl OperationsManager {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn execute_action(&self, intent: ActionIntent) -> Result<(), OperationsError> {
        if intent.tenant_id.is_empty() {
            return Err(OperationsError::Unauthorized);
        }

        let mut tx = self.pool.begin().await?;

        // 1. Enforce Row-Level Security (RLS) for multi-tenancy
        sqlx::query(
            "SELECT set_config('rls.tenant_id', $1, true)"
        )
        .bind(&intent.tenant_id)
        .execute(&mut *tx)
        .await?;

        // 2. Perform the operation based on action_type
        match intent.action_type.as_str() {
            "BOOKING_REQUEST" => {
                self.handle_booking(&mut tx, &intent).await?;
            }
            "INVENTORY_DEDUCTION" => {
                self.handle_inventory(&mut tx, &intent).await?;
            }
            _ => {
                tx.rollback().await?;
                return Err(OperationsError::ExecutionError(format!("Unknown action type: {}", intent.action_type)));
            }
        }

        // 3. Commit the transaction
        tx.commit().await?;

        Ok(())
    }

    async fn handle_booking(&self, _tx: &mut Transaction<'_, Postgres>, _intent: &ActionIntent) -> Result<(), OperationsError> {
        Ok(())
    }

    async fn handle_inventory(&self, _tx: &mut Transaction<'_, Postgres>, _intent: &ActionIntent) -> Result<(), OperationsError> {
        Ok(())
    }
}
