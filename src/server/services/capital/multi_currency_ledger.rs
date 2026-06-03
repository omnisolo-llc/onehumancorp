use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiCurrencyEntry {
    pub id: String,
    pub tenant_id: String,
    pub presentment_amount: i64,
    pub presentment_currency: String,
    pub settlement_amount: i64,
    pub settlement_currency: String,
    pub exchange_rate: f64,
    pub is_offline_sync: bool,
    pub safe_margin_absorbed: i64,
    pub payout_status: String,
    pub guaranteed_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct MultiCurrencyLedger {
    pool: Arc<PgPool>,
}

impl MultiCurrencyLedger {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Records a transaction, handling conversion from presentment to settlement currency.
    /// If is_offline_sync is true, it compares the cached_rate used offline with the current_rate
    /// and absorbs small differences in safe_margin_absorbed.
    pub async fn record_transaction(
        &self,
        tenant_id: &str,
        presentment_amount: i64,
        presentment_currency: &str,
        settlement_currency: &str,
        cached_rate: Option<f64>,
    ) -> Result<String, String> {
        let entry_id = Uuid::new_v4().to_string();

        // Fetch current exchange rate from DB
        let current_rate = self.get_fx_rate(presentment_currency, settlement_currency).await?;

        let mut is_offline_sync = false;
        let mut safe_margin_absorbed = 0;
        let final_rate;

        if let Some(offline_rate) = cached_rate {
            is_offline_sync = true;
            final_rate = offline_rate;

            let expected_settlement = (presentment_amount as f64 * current_rate).round() as i64;
            let offline_settlement = (presentment_amount as f64 * offline_rate).round() as i64;

            // Absorb the difference if it's within a reasonable margin (e.g., 2%)
            let diff = (expected_settlement - offline_settlement).abs();
            if diff as f64 <= (expected_settlement as f64 * 0.02) {
                safe_margin_absorbed = expected_settlement - offline_settlement;
            } else {
                // Large discrepancy might need manual review or Finance Agent intervention
                // For now, we still record it but flag it
            }
        } else {
            final_rate = current_rate;
        }

        let settlement_amount = (presentment_amount as f64 * final_rate).round() as i64;

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO ohc_multi_currency_ledger
             (id, tenant_id, presentment_amount, presentment_currency, settlement_amount, settlement_currency, exchange_rate, is_offline_sync, safe_margin_absorbed, payout_status, guaranteed_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'pending', NULL)"
        )
        .bind(&entry_id)
        .bind(tenant_id)
        .bind(presentment_amount)
        .bind(presentment_currency)
        .bind(settlement_amount)
        .bind(settlement_currency)
        .bind(final_rate)
        .bind(is_offline_sync)
        .bind(safe_margin_absorbed)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(entry_id)
    }


    pub async fn mark_payout_guaranteed(&self, tenant_id: &str, entry_id: &str) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        sqlx::query(
            "UPDATE ohc_multi_currency_ledger SET payout_status = 'guaranteed', guaranteed_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2"
        )
        .bind(entry_id)
        .bind(tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_fx_rate(&self, from: &str, to: &str) -> Result<f64, String> {
        if from == to {
            return Ok(1.0);
        }

        let row = sqlx::query("SELECT rate FROM ohc_fx_rates WHERE from_currency = $1 AND to_currency = $2")
            .bind(from)
            .bind(to)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        match row {
            Some(r) => {
                use sqlx::Row;
                Ok(r.get("rate"))
            }
            None => Err(format!("No exchange rate found from {} to {}", from, to)),
        }
    }
}
