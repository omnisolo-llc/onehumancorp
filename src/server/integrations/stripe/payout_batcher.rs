use redis::{AsyncCommands, Client};
use std::sync::Arc;

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
    redis_client: Option<Arc<Client>>,
    batch_threshold_cents: i64,
}

impl PayoutBatcher {
    pub fn new(redis_url: Option<String>, batch_threshold_cents: i64) -> Self {
        let redis_client = if let Some(url) = redis_url {
            Client::open(url).ok().map(Arc::new)
        } else {
            None
        };
        PayoutBatcher {
            redis_client,
            batch_threshold_cents,
        }
    }

    /// Records a pending payout for a connected account.
    /// Returns Some(amount_to_payout_in_cents) if the threshold is reached and we should execute the payout.
    pub async fn record_payout(&self, account_id: &str, amount_cents: i64) -> Result<Option<i64>, String> {
        if let Some(client) = &self.redis_client {
            let mut conn = client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;
            let key = format!("stripe_payout_batch:{}", account_id);
            let current_balance: i64 = conn.incr(&key, amount_cents).await.map_err(|e| e.to_string())?;

            if current_balance >= self.batch_threshold_cents {
                let _ : () = conn.del(&key).await.unwrap_or(());
                Ok(Some(current_balance))
            } else {
                Ok(None)
            }
        } else {
            // Fallback for tests / memory mode, always payout immediately to prevent data loss
            Ok(Some(amount_cents))
        }
    }

    pub async fn get_pending_balance(&self, account_id: &str) -> Result<i64, String> {
        if let Some(client) = &self.redis_client {
            let mut conn = client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;
            let key = format!("stripe_payout_batch:{}", account_id);
            let balance: Option<i64> = conn.get(&key).await.ok();
            Ok(balance.unwrap_or(0))
        } else {
            Ok(0)
        }
    }

    pub async fn force_payout(&self, account_id: &str) -> Result<Option<i64>, String> {
        if let Some(client) = &self.redis_client {
            let mut conn = client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;
            let key = format!("stripe_payout_batch:{}", account_id);
            let balance: Option<i64> = conn.get(&key).await.ok();
            if let Some(b) = balance {
                if b > 0 {
                    let _ : () = conn.del(&key).await.unwrap_or(());
                    return Ok(Some(b));
                }
            }
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
    async fn test_record_payout_no_redis() {
        let batcher = PayoutBatcher::new(None, 10000);
        // With no redis, it falls back to immediate payout
        let result = batcher.record_payout("acct_1", 2000).await.unwrap();
        assert_eq!(result, Some(2000));
    }

    #[tokio::test]
    async fn test_record_payout_with_redis() {
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            let batcher = PayoutBatcher::new(Some(redis_url), 10000); // $100 threshold

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
}
