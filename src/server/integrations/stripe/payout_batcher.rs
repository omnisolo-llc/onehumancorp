use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;
use serde_json::json;

/// 💰 Miser Cost Analysis:
/// Stripe charges a flat fee of $0.25 plus 0.25% for instant payouts,
/// or a flat fee for standard payouts.
/// By batching smaller payouts into larger chunks (e.g. daily or weekly),
/// we save the fixed $0.25 fee for every individual payout that would
/// otherwise have been initiated.
///
/// Estimated savings:
/// If a business processes 100 small payouts of $10 each:
/// - Unbatched: 100 * $0.25 = $25.00 in fixed fees
/// - Batched (1 payout of $1000): 1 * $0.25 = $0.25 in fixed fees
/// - Total savings = $24.75 per 100 transactions!
pub struct PayoutBatcher {
    pool: Option<Arc<PgPool>>,
    batch_threshold_cents: i64,
}

impl PayoutBatcher {
    pub fn new(pool: Option<Arc<PgPool>>, batch_threshold_cents: i64) -> Self {
        PayoutBatcher {
            pool,
            batch_threshold_cents,
        }
    }

    async fn append_ledger_entry_tx(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, account_id: &str, amount_cents: i64) -> Result<(), String> {
        let entry_id = Uuid::new_v4().to_string();
        let payload = json!({ "amount": amount_cents });

        sqlx::query(
            "INSERT INTO ohc_universal_ledger (id, tenant_id, department, action_type, state_change, created_at)
             VALUES ($1, $2, 'Finance', 'PayoutBatchEvent', $3, CURRENT_TIMESTAMP)"
        )
        .bind(&entry_id)
        .bind(account_id)
        .bind(&payload)
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_pending_balance_tx(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, account_id: &str) -> Result<i64, String> {
        let row = sqlx::query(
            "SELECT COALESCE(CAST(SUM((state_change->>'amount')::BIGINT) AS BIGINT), 0) as balance
             FROM ohc_universal_ledger
             WHERE tenant_id = $1 AND action_type = 'PayoutBatchEvent'"
        )
        .bind(account_id)
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        let balance: i64 = row.get("balance");
        Ok(balance)
    }

    pub async fn get_pending_balance(&self, account_id: &str) -> Result<i64, String> {
        if let Some(pool) = &self.pool {
            let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
            ::server_common::auth_utils::set_org_context(&mut *tx, account_id).await.map_err(|e| e.to_string())?;

            let balance = Self::get_pending_balance_tx(&mut tx, account_id).await?;
            tx.commit().await.map_err(|e| e.to_string())?;
            Ok(balance)
        } else {
            Ok(0)
        }
    }

    fn hash_account_id(account_id: &str) -> i64 {
        // Use a stable, deterministic hash algorithm (djb2) to avoid cross-version hash drift.
        let mut hash: i64 = 5381;
        for c in account_id.bytes() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(c as i64);
        }
        hash
    }

    /// Records a pending payout for a connected account.
    /// Returns Some(amount_to_payout_in_cents) if the threshold is reached and we should execute the payout.
    pub async fn record_payout(&self, account_id: &str, amount_cents: i64) -> Result<Option<i64>, String> {
        if let Some(pool) = &self.pool {
            let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
            ::server_common::auth_utils::set_org_context(&mut *tx, account_id).await.map_err(|e| e.to_string())?;

            let lock_id = Self::hash_account_id(account_id);
            sqlx::query("SELECT pg_advisory_xact_lock($1)")
                .bind(lock_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

            Self::append_ledger_entry_tx(&mut tx, account_id, amount_cents).await?;
            let current_balance = Self::get_pending_balance_tx(&mut tx, account_id).await?;

            if current_balance >= self.batch_threshold_cents {
                // Clear the balance by appending a negative event
                Self::append_ledger_entry_tx(&mut tx, account_id, -current_balance).await?;
                tx.commit().await.map_err(|e| e.to_string())?;
                Ok(Some(current_balance))
            } else {
                tx.commit().await.map_err(|e| e.to_string())?;
                Ok(None)
            }
        } else {
            // Fallback for tests / memory mode, always payout immediately to prevent data loss
            Ok(Some(amount_cents))
        }
    }

    pub async fn force_payout(&self, account_id: &str) -> Result<Option<i64>, String> {
        if let Some(pool) = &self.pool {
            let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
            ::server_common::auth_utils::set_org_context(&mut *tx, account_id).await.map_err(|e| e.to_string())?;

            let lock_id = Self::hash_account_id(account_id);
            sqlx::query("SELECT pg_advisory_xact_lock($1)")
                .bind(lock_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

            let current_balance = Self::get_pending_balance_tx(&mut tx, account_id).await?;
            if current_balance > 0 {
                Self::append_ledger_entry_tx(&mut tx, account_id, -current_balance).await?;
                tx.commit().await.map_err(|e| e.to_string())?;
                return Ok(Some(current_balance));
            }
            tx.commit().await.map_err(|e| e.to_string())?;
            Ok(None)
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_record_payout_no_pool() {
        let batcher = PayoutBatcher::new(None, 10000);
        // With no pool, it falls back to immediate payout
        let result = batcher.record_payout("acct_1", 2000).await.unwrap();
        assert_eq!(result, Some(2000));

        let pending = batcher.get_pending_balance("acct_1").await.unwrap();
        assert_eq!(pending, 0);

        let force_result = batcher.force_payout("acct_1").await.unwrap();
        assert_eq!(force_result, None);
    }

    #[tokio::test]
    async fn test_record_payout_with_pool() {
        let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());
        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(pool) => pool,
            Err(_) => {
                tracing::info!("Skipping test due to no postgres");
                return;
            }
        };
        let batcher = PayoutBatcher::new(Some(Arc::new(pool)), 10000); // $100 threshold

        // clear state
        let _ = batcher.force_payout("acct_2").await;

        assert_eq!(batcher.record_payout("acct_2", 2000).await.unwrap(), None);
        assert_eq!(batcher.record_payout("acct_2", 3000).await.unwrap(), None);
        assert_eq!(batcher.get_pending_balance("acct_2").await.unwrap(), 5000);

        // Reaches threshold
        assert_eq!(batcher.record_payout("acct_2", 6000).await.unwrap(), Some(11000));
        assert_eq!(batcher.get_pending_balance("acct_2").await.unwrap(), 0);
    }
}

#[cfg(test)]
mod batching_cost_tests {
    use super::*;

    #[test]
    fn test_batch_threshold_saves_fees() {
        let pool: Option<Arc<PgPool>> = None;
        let threshold = crate::integrations::stripe::routing::PaymentRouter::BATCH_PAYOUT_THRESHOLD_CENTS;
        let batcher = PayoutBatcher::new(pool, threshold);

        // Simulating the routing check directly as that's what prevents unbatched fees
        assert_eq!(crate::integrations::stripe::routing::PaymentRouter::should_batch_payout(1000), true);
        assert_eq!(crate::integrations::stripe::routing::PaymentRouter::should_batch_payout(10000), false); // Threshold reached

        // Verification that the batcher initializes correctly with the correct threshold.
        assert_eq!(batcher.batch_threshold_cents, 10000);
    }
}
