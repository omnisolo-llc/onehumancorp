use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiCurrencyLedgerEntry {
    pub id: String,
    pub tenant_id: String,
    pub amount: f64,
    pub source_currency: String,
    pub target_currency: String,
    pub exchange_rate: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

pub struct MultiCurrencyLedger {
    pool: Arc<PgPool>,
}

impl MultiCurrencyLedger {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn append_entry(&self, tenant_id: &str, amount: f64, source_currency: &str, target_currency: &str, exchange_rate: f64, status: &str) -> Result<String, String> {
        let entry_id = Uuid::new_v4().to_string();
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO multi_currency_ledger (id, tenant_id, amount, source_currency, target_currency, exchange_rate, status, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP)"
        )
        .bind(&entry_id)
        .bind(tenant_id)
        .bind(amount)
        .bind(source_currency)
        .bind(target_currency)
        .bind(exchange_rate)
        .bind(status)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(entry_id)
    }
}
