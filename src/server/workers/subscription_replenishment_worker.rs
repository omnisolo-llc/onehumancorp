use crate::db::DB;
use sqlx::Row;
use std::sync::Arc;
use tokio::time::Duration;
use uuid::Uuid;

pub struct SubscriptionReplenishmentWorker {
    db: Arc<DB>,
}

impl SubscriptionReplenishmentWorker {
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
                        tracing::error!("SubscriptionReplenishmentWorker error: {}", e);
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
                    WHERE status = 'PENDING' AND next_retry_at <= NOW() AND job_type = 'subscription_replenishment'
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
                    let payload: serde_json::Value = r
                        .try_get("payload")
                        .map_err(|e| format!("invalid subscription replenishment payload: {e}"))?;

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
            }
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    SELECT id, tenant_id, payload
                    FROM ohc_job_queue
                    WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP AND job_type = 'subscription_replenishment'
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
                    let payload: serde_json::Value = serde_json::from_str(&payload_str)
                        .unwrap_or_else(|_| serde_json::json!({}));

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
            if let Some(order_id) = payload.get("order_id").and_then(|v| v.as_str()) {
                let customer_id = payload
                    .get("customer_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let customer_name = match &self.db.store {
                    crate::db::DbStore::Postgres => sqlx::query_scalar::<_, String>(
                        "SELECT name FROM customers WHERE id = $1 AND tenant_id = $2",
                    )
                    .bind(customer_id)
                    .bind(&tenant_id)
                    .fetch_optional(&self.db.pool)
                    .await
                    .unwrap_or_default()
                    .unwrap_or_else(|| "Customer".to_string()),
                    crate::db::DbStore::Sqlite(sqlite_pool) => sqlx::query_scalar::<_, String>(
                        "SELECT name FROM customers WHERE id = ? AND tenant_id = ?",
                    )
                    .bind(customer_id)
                    .bind(&tenant_id)
                    .fetch_optional(sqlite_pool)
                    .await
                    .unwrap_or_default()
                    .unwrap_or_else(|| "Customer".to_string()),
                };

                let item_name = payload
                    .get("item_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("your item");

                let proposed_action = serde_json::json!({
                    "action_type": "Draft Reply",
                    "draft_reply": format!("Hi {}, you might be running low on {}! Want me to send another batch for 10% off? Just reply YES.", customer_name, item_name),
                    "order_id": order_id
                });

                let context_payload = serde_json::json!({
                    "description": format!("Customer {} bought a supply of {} recently. They are likely running out.", customer_name, item_name)
                });

                let agent_feed_item_id = Uuid::new_v4().to_string();

                match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        let _ = sqlx::query(
                            "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 'PENDING_APPROVAL', NOW(), NOW())"
                        )
                        .bind(&agent_feed_item_id)
                        .bind(&tenant_id)
                        .bind("Sales Agent")
                        .bind(context_payload.to_string())
                        .bind(proposed_action.to_string())
                        .execute(&self.db.pool)
                        .await;
                    }
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                        let _ = sqlx::query(
                            "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                        )
                        .bind(&agent_feed_item_id)
                        .bind(&tenant_id)
                        .bind("Sales Agent")
                        .bind(context_payload.to_string())
                        .bind(proposed_action.to_string())
                        .execute(sqlite_pool)
                        .await;
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
                }
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
