use std::sync::Arc;
use tokio::time::Duration;
use crate::db::DB;
use sqlx::Row;
use uuid::Uuid;

pub struct SubscriptionChurnWorker {
    db: Arc<DB>,
}

impl SubscriptionChurnWorker {
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
                        tracing::error!("SubscriptionChurnWorker error: {}", e);
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
                    WHERE status = 'PENDING' AND next_retry_at <= NOW() AND job_type = 'subscription_churn_check'
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
                    WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP AND job_type = 'subscription_churn_check'
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
            if let (Some(subscriber_id), Some(customer_id)) = (payload.get("subscriber_id").and_then(|v| v.as_str()), payload.get("customer_id").and_then(|v| v.as_str())) {

                let (customer_name, current_health_score, has_recent_bookings) = match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        let name = sqlx::query_scalar::<_, String>("SELECT name FROM customers WHERE id = $1::uuid AND tenant_id = $2")
                            .bind(customer_id)
                            .bind(&tenant_id)
                            .fetch_optional(&self.db.pool)
                            .await
                            .unwrap_or_default()
                            .unwrap_or_else(|| "Customer".to_string());

                        let current_score: i32 = sqlx::query_scalar("SELECT health_score FROM subscribers WHERE id = $1 AND tenant_id = $2")
                            .bind(subscriber_id)
                            .bind(&tenant_id)
                            .fetch_optional(&self.db.pool)
                            .await
                            .unwrap_or_default()
                            .unwrap_or(100);

                        let recent_bookings_count: i64 = sqlx::query_scalar(
                            "SELECT COUNT(*) FROM bookings WHERE customer_id = $1::uuid AND tenant_id = $2 AND created_at >= NOW() - INTERVAL '30 days'"
                        )
                        .bind(customer_id)
                        .bind(&tenant_id)
                        .fetch_optional(&self.db.pool)
                        .await
                        .unwrap_or_default()
                        .unwrap_or(0);

                        (name, current_score, recent_bookings_count > 0)
                    },
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                        let name = sqlx::query_scalar::<_, String>("SELECT name FROM customers WHERE id = ? AND tenant_id = ?")
                            .bind(customer_id)
                            .bind(&tenant_id)
                            .fetch_optional(sqlite_pool)
                            .await
                            .unwrap_or_default()
                            .unwrap_or_else(|| "Customer".to_string());

                        let current_score: i32 = sqlx::query_scalar("SELECT health_score FROM subscribers WHERE id = ? AND tenant_id = ?")
                            .bind(subscriber_id)
                            .bind(&tenant_id)
                            .fetch_optional(sqlite_pool)
                            .await
                            .unwrap_or_default()
                            .unwrap_or(100);

                        let recent_bookings_count: i64 = sqlx::query_scalar(
                            "SELECT COUNT(*) FROM bookings WHERE customer_id = ? AND tenant_id = ? AND created_at >= datetime('now', '-30 days')"
                        )
                        .bind(customer_id)
                        .bind(&tenant_id)
                        .fetch_optional(sqlite_pool)
                        .await
                        .unwrap_or_default()
                        .unwrap_or(0);

                        (name, current_score, recent_bookings_count > 0)
                    }
                };

                let mut new_health_score = current_health_score;
                let mut churn_risk_status = "healthy";

                if !has_recent_bookings {
                    new_health_score = std::cmp::max(0, new_health_score - 20);
                } else {
                    new_health_score = std::cmp::min(100, new_health_score + 10);
                }

                if new_health_score <= 50 {
                    churn_risk_status = "at_risk";
                }

                // Update subscriber health
                match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query("UPDATE subscribers SET health_score = $1, churn_risk_status = $2, last_health_check_at = NOW() WHERE id = $3 AND tenant_id = $4")
                            .bind(new_health_score)
                            .bind(churn_risk_status)
                            .bind(subscriber_id)
                            .bind(&tenant_id)
                            .execute(&self.db.pool)
                            .await
                            .unwrap_or_default();
                    },
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                        sqlx::query("UPDATE subscribers SET health_score = ?, churn_risk_status = ?, last_health_check_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?")
                            .bind(new_health_score)
                            .bind(churn_risk_status)
                            .bind(subscriber_id)
                            .bind(&tenant_id)
                            .execute(sqlite_pool)
                            .await
                            .unwrap_or_default();
                    }
                }

                // Avoid duplicate feed spam by checking if there's an already pending ambassador reply for this churn risk
                let is_already_pending = match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query_scalar::<_, i64>(
                            "SELECT COUNT(*) FROM agent_feed_items WHERE tenant_id = $1 AND event_source = 'The Ambassador' AND lifecycle_state = 'PENDING_APPROVAL' AND context_payload->>'subscriber_id' = $2"
                        )
                        .bind(&tenant_id)
                        .bind(subscriber_id)
                        .fetch_optional(&self.db.pool)
                        .await
                        .unwrap_or_default()
                        .unwrap_or(0) > 0
                    },
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                        sqlx::query_scalar::<_, i64>(
                            "SELECT COUNT(*) FROM agent_feed_items WHERE tenant_id = ? AND event_source = 'The Ambassador' AND lifecycle_state = 'PENDING_APPROVAL' AND json_extract(context_payload, '$.subscriber_id') = ?"
                        )
                        .bind(&tenant_id)
                        .bind(subscriber_id)
                        .fetch_optional(sqlite_pool)
                        .await
                        .unwrap_or_default()
                        .unwrap_or(0) > 0
                    }
                };

                if churn_risk_status == "at_risk" && !is_already_pending {
                    // Try preferred LLM, fallback to safe defaults if env isn't provided
                    let llm_key = std::env::var("GEMINI_API_KEY").or_else(|_| std::env::var("MINIMAX_API_KEY")).unwrap_or_default();
                    let draft = if !llm_key.is_empty() {
                        let provider = crate::minimax::MinimaxClient::new(llm_key);
                        let prompt = format!(
                            "You are the Customer Success Ambassador for the business. \
                            Draft a friendly, personalized check-in and win-back message for {name} who hasn't booked a service with us in over 30 days. \
                            Offer them a quick 10% discount to re-engage.",
                            name = customer_name,
                        );

                        provider.reason(&prompt).await.unwrap_or_else(|_| format!("Hi {}, we noticed you haven't booked in a while. Is everything okay? We'd love to offer you 10% off your next session to keep the momentum going!", customer_name))
                    } else {
                        format!("Hi {}, we noticed you haven't booked in a while. Is everything okay? We'd love to offer you 10% off your next session to keep the momentum going!", customer_name)
                    };

                    let proposed_action = serde_json::json!({
                        "action_type": "Draft Reply",
                        "draft_reply": draft,
                        "subscriber_id": subscriber_id,
                        "customer_id": customer_id
                    });

                    let context_payload = serde_json::json!({
                        "description": format!("Customer {}'s subscription health score dropped to {}. They have no recent bookings.", customer_name, new_health_score),
                        "subscriber_id": subscriber_id
                    });

                    let agent_feed_item_id = Uuid::new_v4().to_string();

                    match &self.db.store {
                        crate::db::DbStore::Postgres => {
                            let _ = sqlx::query(
                                "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 'PENDING_APPROVAL', NOW(), NOW())"
                            )
                            .bind(&agent_feed_item_id)
                            .bind(&tenant_id)
                            .bind("The Ambassador")
                            .bind(context_payload.to_string())
                            .bind(proposed_action.to_string())
                            .execute(&self.db.pool)
                            .await;
                        },
                        crate::db::DbStore::Sqlite(sqlite_pool) => {
                            let _ = sqlx::query(
                                "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                            )
                            .bind(&agent_feed_item_id)
                            .bind(&tenant_id)
                            .bind("The Ambassador")
                            .bind(context_payload.to_string())
                            .bind(proposed_action.to_string())
                            .execute(sqlite_pool)
                            .await;
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
