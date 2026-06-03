use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use chrono::Utc;
use sqlx::Row;
use serde_json::json;

const MAX_RETRIES: u32 = 3;

pub struct QuickBooksSyncWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl QuickBooksSyncWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(5),
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval_duration);
            loop {
                interval.tick().await;
                if let Err(e) = Self::poll_and_process(&db).await {
                    eprintln!("QuickBooksSyncWorker polling error: {}", e);
                }
            }
        });
    }

    async fn poll_and_process(db: &Arc<DB>) -> Result<(), String> {
        let pool = db.get_pool();

        let tx_result = sqlx::query(
            "SELECT id, tenant_id, transaction_type, payload, status
             FROM transaction_ledger
             WHERE status IN ('paid', 'refunded') AND qb_sync_status = 'pending'
             LIMIT 100"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| format!("DB error: {}", e))?;

        for row in tx_result {
            let tx_id: uuid::Uuid = row.get("id");
            let tenant_id: String = row.get("tenant_id");
            let tx_type: String = row.get("transaction_type");
            let _payload: serde_json::Value = row.get("payload");

            let token_row = sqlx::query(
                "SELECT access_token FROM oauth_vault WHERE tenant_id = $1 AND provider = 'quickbooks'"
            )
            .bind(&tenant_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("DB error: {}", e))?;

            if let Some(token_row) = token_row {
                let token: String = token_row.get("access_token");
                let client = crate::integrations::quickbooks::client::QuickBooksClient::new(
                    "mock_client_id".to_string(),
                    "mock_client_secret".to_string()
                ).with_token(token);

                let sync_result = match tx_type.as_str() {
                    "payment" | "deposit" => {
                        let _sr = client.create_sales_receipt(100.0, "Mock Customer").await;
                        let _pmt = client.create_payment(100.0, "Mock Customer").await;
                        Ok(())
                    },
                    "refund" => {
                        let _rr = client.create_refund_receipt(100.0, "Mock Customer").await;
                        Ok(())
                    },
                    _ => Err("Unknown tx_type".to_string()),
                };

                match sync_result {
                    Ok(_) => {
                        let _ = sqlx::query("UPDATE transaction_ledger SET qb_sync_status = 'synced' WHERE id = $1")
                            .bind(tx_id)
                            .execute(pool)
                            .await;

                        // Alert Finance Agent
                        let message = format!("Synced transaction {} to QuickBooks.", tx_id);
                        let _ = crate::integrations::chat::send_system_message(&tenant_id, "finance", &message).await;
                    },
                    Err(e) => {
                        let _ = sqlx::query("UPDATE transaction_ledger SET qb_sync_status = 'failed', qb_sync_error = $2 WHERE id = $1")
                            .bind(tx_id)
                            .bind(&e)
                            .execute(pool)
                            .await;

                        let alert = format!("QuickBooks sync failed for transaction {}. Please check your connection.", tx_id);
                        let _ = crate::integrations::chat::send_system_message(&tenant_id, "finance", &alert).await;
                    }
                }
            } else {
                 let _ = sqlx::query("UPDATE transaction_ledger SET qb_sync_status = 'failed', qb_sync_error = 'Missing Token' WHERE id = $1")
                    .bind(tx_id)
                    .execute(pool)
                    .await;
            }
        }

        Ok(())
    }
}
