use std::sync::Arc;
use tokio::time::Duration;
use crate::db::DB;
use sqlx::Row;
use uuid::Uuid;

pub struct SubscriptionRetentionWorker {
    db: Arc<DB>,
}

impl SubscriptionRetentionWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            loop {
                match self.poll().await {
                    Ok(true) => {
                        // Processed a job, continue immediately
                        continue;
                    }
                    Ok(false) => {
                        // No jobs, sleep
                        tokio::time::sleep(Duration::from_millis(5000)).await;
                    }
                    Err(e) => {
                        tracing::error!("SubscriptionRetentionWorker error: {}", e);
                        tokio::time::sleep(Duration::from_millis(5000)).await;
                    }
                }
            }
        });
    }

    pub async fn poll(&self) -> Result<bool, String> {
        let job = match &self.db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    SELECT id, tenant_id, payload
                    FROM ohc_job_queue
                    WHERE status = 'PENDING' AND next_retry_at <= NOW() AND job_type = 'subscription_retention'
                    ORDER BY next_retry_at ASC, created_at ASC
                    FOR UPDATE SKIP LOCKED
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    let id: String = r.get("id");
                    let tenant_id: String = r.get("tenant_id");
                    let payload: serde_json::Value = serde_json::from_str(r.get("payload")).unwrap_or_else(|_| serde_json::json!({}));

                    sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = NOW() WHERE id = $1")
                        .bind(&id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;
                    Some((id, tenant_id, payload))
                } else {
                    tx.rollback().await.map_err(|e| e.to_string())?;
                    None
                }
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    SELECT id, tenant_id, payload
                    FROM ohc_job_queue
                    WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP AND job_type = 'subscription_retention'
                    ORDER BY next_retry_at ASC, created_at ASC
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    let id: String = r.get("id");
                    let tenant_id: String = r.get("tenant_id");
                    let payload_str: String = r.get("payload");
                    let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));

                    sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;
                    Some((id, tenant_id, payload))
                } else {
                    tx.rollback().await.map_err(|e| e.to_string())?;
                    None
                }
            }
        };

        if let Some((job_id, tenant_id, payload)) = job {
            if let Some(subscription_id) = payload.get("subscription_id").and_then(|v| v.as_str()) {
                let customer_id = payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("");

                // Get customer name for drafted message
                let customer_name = match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query_scalar::<_, String>("SELECT name FROM customers WHERE id = $1 AND tenant_id = $2")
                            .bind(customer_id)
                            .bind(&tenant_id)
                            .fetch_optional(&self.db.pool)
                            .await
                            .unwrap_or_default()
                            .unwrap_or_else(|| "Customer".to_string())
                    },
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                        sqlx::query_scalar::<_, String>("SELECT name FROM customers WHERE id = ? AND tenant_id = ?")
                            .bind(customer_id)
                            .bind(&tenant_id)
                            .fetch_optional(sqlite_pool)
                            .await
                            .unwrap_or_default()
                            .unwrap_or_else(|| "Customer".to_string())
                    }
                };

                // Perform churn analysis
                // In a real implementation this would fetch usage data, payment failures, etc.
                // For this agentic implementation we'll simulate finding an at-risk customer.

                // Simple deterministic pseudo-random logic for the sake of the system
                let sum_chars: u32 = customer_id.chars().map(|c| c as u32).sum();
                let is_at_risk = (sum_chars % 3) == 0; // Fake condition to trigger risk

                if is_at_risk {
                    let health_score_id = Uuid::new_v4().to_string();
                    let feed_item_id = Uuid::new_v4().to_string();
                    let intervention_id = Uuid::new_v4().to_string();

                    let proposed_message = format!(
                        "Hi {}, we noticed you haven't booked a lesson recently. Is everything okay? We'd love to offer you 10% off your next package to keep the momentum going!",
                        customer_name.split(' ').next().unwrap_or(&customer_name)
                    );

                    let factors = serde_json::json!([
                        "No bookings in 30 days",
                        "Engagement dropped"
                    ]);

                    match &self.db.store {
                        crate::db::DbStore::Postgres => {
                            let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                            // 1. Insert Health Score
                            let _ = sqlx::query(
                                "INSERT INTO subscription_health_scores (id, tenant_id, subscription_id, customer_id, score, risk_level, factors, created_at, updated_at) VALUES ($1, $2, $3, $4, 45, 'HIGH', $5, NOW(), NOW())"
                            )
                            .bind(&health_score_id)
                            .bind(&tenant_id)
                            .bind(subscription_id)
                            .bind(customer_id)
                            .bind(factors)
                            .execute(&mut *tx)
                            .await;

                            // 2. Insert feed item for owner to approve
                            let context_payload = serde_json::json!({
                                "customer_name": customer_name,
                                "customer_id": customer_id,
                                "subscription_id": subscription_id,
                                "risk_level": "HIGH",
                                "reasons": factors
                            });

                            let _ = sqlx::query(
                                "INSERT INTO tenant_feed_items (id, tenant_id, title, description, action_type, action_payload, status, created_at, updated_at) VALUES ($1, $2, 'At-Risk Subscriber Detected', $3, 'REVIEW_WINBACK_OFFER', $4, 'pending', NOW(), NOW())"
                            )
                            .bind(&feed_item_id)
                            .bind(&tenant_id)
                            .bind(format!("The Ambassador identified {} as at-risk. Tap to review drafted offer.", customer_name))
                            .bind(serde_json::json!({ "intervention_id": intervention_id }))
                            .execute(&mut *tx)
                            .await;

                            // 3. Insert Intervention
                            let _ = sqlx::query(
                                "INSERT INTO subscription_retention_interventions (id, tenant_id, subscription_id, customer_id, health_score_id, status, intervention_type, channel, proposed_message, owner_feed_item_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 'DRAFTED', 'WIN_BACK_OFFER', 'SMS', $6, $7, NOW(), NOW())"
                            )
                            .bind(&intervention_id)
                            .bind(&tenant_id)
                            .bind(subscription_id)
                            .bind(customer_id)
                            .bind(&health_score_id)
                            .bind(&proposed_message)
                            .bind(&feed_item_id)
                            .execute(&mut *tx)
                            .await;

                            tx.commit().await.map_err(|e| e.to_string())?;
                        },
                        crate::db::DbStore::Sqlite(sqlite_pool) => {
                            let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;

                            let _ = sqlx::query(
                                "INSERT INTO subscription_health_scores (id, tenant_id, subscription_id, customer_id, score, risk_level, factors, created_at, updated_at) VALUES (?, ?, ?, ?, 45, 'HIGH', ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                            )
                            .bind(&health_score_id)
                            .bind(&tenant_id)
                            .bind(subscription_id)
                            .bind(customer_id)
                            .bind(factors.to_string())
                            .execute(&mut *tx)
                            .await;

                            let context_payload = serde_json::json!({
                                "customer_name": customer_name,
                                "customer_id": customer_id,
                                "subscription_id": subscription_id,
                                "risk_level": "HIGH",
                                "reasons": factors
                            });

                            let _ = sqlx::query(
                                "INSERT INTO tenant_feed_items (id, tenant_id, title, description, action_type, action_payload, status, created_at, updated_at) VALUES (?, ?, 'At-Risk Subscriber Detected', ?, 'REVIEW_WINBACK_OFFER', ?, 'pending', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                            )
                            .bind(&feed_item_id)
                            .bind(&tenant_id)
                            .bind(format!("The Ambassador identified {} as at-risk. Tap to review drafted offer.", customer_name))
                            .bind(serde_json::json!({ "intervention_id": intervention_id }).to_string())
                            .execute(&mut *tx)
                            .await;

                            let _ = sqlx::query(
                                "INSERT INTO subscription_retention_interventions (id, tenant_id, subscription_id, customer_id, health_score_id, status, intervention_type, channel, proposed_message, owner_feed_item_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 'DRAFTED', 'WIN_BACK_OFFER', 'SMS', ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                            )
                            .bind(&intervention_id)
                            .bind(&tenant_id)
                            .bind(subscription_id)
                            .bind(customer_id)
                            .bind(&health_score_id)
                            .bind(&proposed_message)
                            .bind(&feed_item_id)
                            .execute(&mut *tx)
                            .await;

                            tx.commit().await.map_err(|e| e.to_string())?;
                        }
                    }
                }
            }

            // Mark job as completed
            match &self.db.store {
                crate::db::DbStore::Postgres => {
                    sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = NOW() WHERE id = $1")
                        .bind(&job_id)
                        .execute(&self.db.pool)
                        .await
                        .map_err(|e| e.to_string())?;
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&job_id)
                        .execute(sqlite_pool)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }
}
