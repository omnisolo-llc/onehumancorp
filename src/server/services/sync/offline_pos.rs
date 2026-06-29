use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct OfflineMutation {
    pub transaction_id: String,
    pub timestamp: Option<String>,
    pub product_id: String,
    pub quantity_deducted: i32,
    pub amount: Option<i64>,
    pub payment_method: Option<String>,
    pub payment_intent_id: Option<String>,
    pub currency: Option<String>,
    pub mutation_type: Option<String>,
    pub payload: Option<String>,
}

pub struct CRDTOfflineSynchronizer;

impl CRDTOfflineSynchronizer {
    pub async fn process_batch(
        pool: &PgPool,
        tenant_id: &str,
        mutations: &[OfflineMutation],
    ) -> Result<(usize, Vec<String>), String> {
        let mut failed_count = 0;
        let mut failed_transactions = Vec::new();

        for mutation in mutations {
            if mutation.mutation_type.as_deref() == Some("draft_quote") {
                continue;
            }
            let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

            // CRDT Operation: PN-Counter decrement (increment pn_counter_n) and re-calculate inventory_count
            let query = "
                UPDATE products
                SET pn_counter_n = pn_counter_n + $1,
                    inventory_count = GREATEST(0, pn_counter_p - (pn_counter_n + $1))
                WHERE id = $2 AND tenant_id = $3
                RETURNING id
            ";

            let result = sqlx::query(query)
                .bind(mutation.quantity_deducted)
                .bind(&mutation.product_id)
                .bind(tenant_id)
                .fetch_optional(&mut *tx)
                .await;

            match result {
                Ok(Some(_)) => {
                    // Record successful pos offline transaction
                    let mutation_ts = mutation.timestamp.clone().unwrap_or_else(|| chrono::Utc::now().to_rfc3339());
                    let res = sqlx::query(
                        "INSERT INTO pos_offline_transactions (id, tenant_id, client_id, status, amount_cents, currency, payload, created_at, updated_at, _sync_status)
                         VALUES ($1, $2, $3, 'RESOLVED', $4, $5, $6::jsonb, $7::timestamptz, $7::timestamptz, 'synced')
                         ON CONFLICT (id) DO UPDATE SET status = 'RESOLVED', payload = EXCLUDED.payload, updated_at = EXCLUDED.updated_at, _sync_status = 'synced'
                         WHERE pos_offline_transactions.updated_at < EXCLUDED.updated_at"
                    )
                    .bind(&mutation.transaction_id)
                    .bind(tenant_id)
                    .bind(&mutation.transaction_id)
                    .bind(mutation.amount.unwrap_or(0))
                    .bind(mutation.currency.as_deref().unwrap_or("USD"))
                    .bind(serde_json::to_value(mutation).unwrap())
                    .bind(&mutation_ts)
                    .execute(&mut *tx)
                    .await;

                    if res.is_ok() {
                        let _ = tx.commit().await;
                        continue;
                    } else {
                        tracing::error!("Failed to record pos transaction: {:?}", res.err());
                    }
                }
                Ok(None) => {
                    tracing::warn!("Product {} not found or unauthorized for tenant {}", mutation.product_id, tenant_id); // pii-safe // pii-safe
                }
                Err(e) => {
                    tracing::error!("Failed to deduct inventory for product {}: {}", mutation.product_id, e);
                }
            }

            // If we reach here, it's a failure
            failed_count += 1;
            failed_transactions.push(mutation.transaction_id.clone());

            let _ = tx.rollback().await;

            // Record failure in pos_offline_transactions
            let mutation_ts = mutation.timestamp.clone().unwrap_or_else(|| chrono::Utc::now().to_rfc3339());
            let _ = sqlx::query(
                "INSERT INTO pos_offline_transactions (id, tenant_id, client_id, status, amount_cents, currency, payload, created_at, updated_at, _sync_status)
                 VALUES ($1, $2, $3, 'FAILED', $4, $5, $6::jsonb, $7::timestamptz, $7::timestamptz, 'synced')
                 ON CONFLICT (id) DO UPDATE SET status = 'FAILED', payload = EXCLUDED.payload, updated_at = EXCLUDED.updated_at, _sync_status = 'synced'
                 WHERE pos_offline_transactions.updated_at < EXCLUDED.updated_at"
            )
            .bind(&mutation.transaction_id)
            .bind(tenant_id)
            .bind(&mutation.transaction_id)
            .bind(mutation.amount.unwrap_or(0))
            .bind(mutation.currency.as_deref().unwrap_or("USD"))
            .bind(serde_json::to_value(mutation).unwrap())
            .bind(&mutation_ts)
            .execute(pool)
            .await;

            // Enqueue Operations AI Agent job for graceful failure handling
            let agent_payload = serde_json::json!({
                "workflow": "ohc_business_swarm",
                "task": "Handle offline POS sync failure",
                "context": format!("Transaction {} failed to sync offline due to inventory discrepancy or decline.", mutation.transaction_id),
                "action": "OperationsAgent: generate a plain-language alert for the business owner and draft a follow-up message to the customer regarding the declined offline transaction."
            }).to_string();

            let _ = sqlx::query(
                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload) VALUES ($1, $2, 'agent_task', $3::jsonb)"
            )
            .bind(Uuid::new_v4().to_string())
            .bind(tenant_id)
            .bind(agent_payload)
            .execute(pool)
            .await;
        }

        Ok((failed_count, failed_transactions))
    }
}
