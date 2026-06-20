use sqlx::{PgPool, Row};
use tracing::{info, error};
use std::time::Duration;
use crate::integrations::stripe::webhooks::StripeWebhookEvent;
use uuid::Uuid;

pub async fn run_stripe_webhook_worker(pool: PgPool) {
    info!("Starting Stripe webhook worker...");
    loop {
        // Simple SKIP LOCKED implementation to pull pending jobs
        let job_res = sqlx::query(
            r#"
            SELECT id, tenant_id, payload
            FROM ohc_job_queue
            WHERE job_type = 'stripe_webhook' AND status = 'PENDING'
            FOR UPDATE SKIP LOCKED
            LIMIT 1
            "#
        )
        .fetch_optional(&pool)
        .await;

        match job_res {
            Ok(Some(row)) => {
                let job_id: String = row.get("id");
                let tenant_id: String = row.get("tenant_id");
                info!("Processing stripe webhook job ID: {}", job_id);

                let _ = sqlx::query(
                    "UPDATE ohc_job_queue SET status = 'PROCESSING' WHERE id = $1"
                )
                .bind(&job_id)
                .execute(&pool)
                .await;

                let payload_value: serde_json::Value = row.get("payload");
                let payload: Result<StripeWebhookEvent, _> = serde_json::from_value(payload_value);

                if let Ok(event) = payload {
                    if event.r#type == Some("payment_intent.succeeded".to_string()) {
                        info!("Payment intent succeeded for job ID: {}", job_id);

                        let object = event.data.and_then(|d| d.object);
                        let idempotency_key = object.as_ref()
                            .and_then(|o| o.get("metadata"))
                            .and_then(|m| m.get("idempotency_key"))
                            .and_then(|v| v.as_str())
                            .unwrap_or_default()
                            .to_string();

                        if !idempotency_key.is_empty() {
                            let mut tx = match pool.begin().await {
                                Ok(tx) => tx,
                                Err(e) => {
                                    error!("Failed to begin transaction: {}", e);
                                    let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED' WHERE id = $1").bind(&job_id).execute(&pool).await;
                                    continue;
                                }
                            };

                            let event_id = object.as_ref()
                                .and_then(|o| o.get("id"))
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string();

                            let update_res = sqlx::query("UPDATE payment_intents SET status = 'succeeded', stripe_payment_intent_id = $1 WHERE idempotency_key = $2 AND tenant_id = $3")
                                .bind(&event_id)
                                .bind(&idempotency_key)
                                .bind(&tenant_id)
                                .execute(&mut *tx)
                                .await;

                            if update_res.is_ok() {
                                let payment_info_res: Result<(f64, String), sqlx::Error> = sqlx::query_as("SELECT amount, currency FROM payment_intents WHERE idempotency_key = $1")
                                    .bind(&idempotency_key)
                                    .fetch_one(&mut *tx)
                                    .await;

                                if let Ok(payment_info) = payment_info_res {
                                    let tx_id = Uuid::new_v4().to_string();
                                    let _ = sqlx::query("INSERT INTO ledger_transactions (tenant_id, tx_id, amount, currency) VALUES ($1, $2, $3, $4)")
                                        .bind(&tenant_id)
                                        .bind(&tx_id)
                                        .bind(payment_info.0)
                                        .bind(&payment_info.1)
                                        .execute(&mut *tx)
                                        .await;

                                    let account_id = "default_revenue";
                                    let _ = sqlx::query("INSERT INTO ledger_accounts (tenant_id, account_id, currency, balance) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING")
                                        .bind(&tenant_id)
                                        .bind(account_id)
                                        .bind(&payment_info.1)
                                        .bind(0.0)
                                        .execute(&mut *tx)
                                        .await;

                                    let entry_id = Uuid::new_v4().to_string();
                                    let _ = sqlx::query("INSERT INTO ledger_entries (tenant_id, entry_id, tx_id, account_id, direction, amount) VALUES ($1, $2, $3, $4, 'CREDIT', $5)")
                                        .bind(&tenant_id)
                                        .bind(&entry_id)
                                        .bind(&tx_id)
                                        .bind(account_id)
                                        .bind(payment_info.0)
                                        .execute(&mut *tx)
                                        .await;

                                    let _ = sqlx::query("UPDATE ledger_accounts SET balance = balance + $1 WHERE tenant_id = $2 AND account_id = $3")
                                        .bind(payment_info.0)
                                        .bind(&tenant_id)
                                        .bind(account_id)
                                        .execute(&mut *tx)
                                        .await;

                                    // Notify Finance Agent
                                    let _ = sqlx::query("INSERT INTO agent_action_requests (id, tenant_id, source, agent_type, action_type, payload, status) VALUES ($1, $2, 'payment_ledger', 'finance', 'payment_succeeded', $3, 'pending')")
                                        .bind(Uuid::new_v4().to_string())
                                        .bind(&tenant_id)
                                        .bind(serde_json::json!({
                                            "event": "payment_succeeded",
                                            "amount": payment_info.0,
                                            "currency": payment_info.1,
                                            "idempotency_key": idempotency_key
                                        }))
                                        .execute(&mut *tx)
                                        .await;
                                }
                            }

                            let _ = tx.commit().await;
                        }
                    }
                }

                let _ = sqlx::query(
                    "UPDATE ohc_job_queue SET status = 'COMPLETED' WHERE id = $1"
                )
                .bind(&job_id)
                .execute(&pool)
                .await;
            }
            Ok(None) => {
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
            Err(e) => {
                error!("Error fetching webhook jobs: {:?}", e);
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}
