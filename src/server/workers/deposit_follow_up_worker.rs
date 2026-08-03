use std::sync::Arc;
use tokio::time::Duration;
use crate::db::DB;
use sqlx::Row;
use uuid::Uuid;

pub struct DepositFollowUpWorker {
    db: Arc<DB>,
}

impl DepositFollowUpWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            loop {
                match self.poll().await {
                    Ok(true) => continue,
                    Ok(false) => tokio::time::sleep(Duration::from_secs(60)).await,
                    Err(e) => {
                        tracing::error!("DepositFollowUpWorker error: {}", e);
                        tokio::time::sleep(Duration::from_secs(60)).await;
                    }
                }
            }
        });
    }

    pub async fn poll(&self) -> Result<bool, String> {
        let row_data = match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query(
                    r#"
                    SELECT q.id, q.tenant_id, q.customer_id, q.total_amount_cents, c.name as customer_name
                    FROM quotes q
                    JOIN customers c ON q.customer_id = c.id
                    WHERE q.status = 'SENT'
                      AND q.updated_at < NOW() - INTERVAL '48 hours'
                      AND (q.last_follow_up_at IS NULL OR q.last_follow_up_at < NOW() - INTERVAL '48 hours')
                      AND q.follow_up_count < 3
                    LIMIT 1
                    "#
                )
                .fetch_optional(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?
                .map(|r| {
                    (
                        r.get::<String, _>("id"),
                        r.get::<String, _>("tenant_id"),
                        r.get::<String, _>("customer_name"),
                        r.get::<i64, _>("total_amount_cents")
                    )
                })
            },
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query(
                    r#"
                    SELECT q.id, q.tenant_id, q.customer_id, q.total_amount_cents, c.name as customer_name
                    FROM quotes q
                    JOIN customers c ON q.customer_id = c.id
                    WHERE q.status = 'SENT'
                      AND q.updated_at < datetime('now', '-2 days')
                      AND (q.last_follow_up_at IS NULL OR q.last_follow_up_at < datetime('now', '-2 days'))
                      AND q.follow_up_count < 3
                    LIMIT 1
                    "#
                )
                .fetch_optional(pool)
                .await
                .map_err(|e| e.to_string())?
                .map(|r| {
                    (
                        r.get::<String, _>("id"),
                        r.get::<String, _>("tenant_id"),
                        r.get::<String, _>("customer_name"),
                        r.get::<i64, _>("total_amount_cents")
                    )
                })
            }
        };

        if let Some((quote_id, tenant_id, customer_name, amount)) = row_data {

            let follow_up_msg = format!(
                "Hi {}, just following up on the estimate for ${:.2}. Let me know if you have any questions or are ready to move forward with the deposit!",
                customer_name, (amount as f64) / 100.0
            );

            let agent_feed_item_id = Uuid::new_v4().to_string();

            match &self.db.store {
                crate::db::DbStore::Postgres => {
                    sqlx::query(
                        "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 'PENDING_APPROVAL', NOW(), NOW())"
                    )
                    .bind(&agent_feed_item_id)
                    .bind(&tenant_id)
                    .bind("deposit_follow_up")
                    .bind(serde_json::json!({
                        "quote_id": quote_id,
                        "customer_name": customer_name,
                        "amount_cents": amount
                    }))
                    .bind(serde_json::json!({
                        "action_type": "Draft Follow-up",
                        "draft_reply": follow_up_msg,
                        "quote_id": quote_id
                    }))
                    .execute(&self.db.pool).await.map_err(|e| e.to_string())?;

                    sqlx::query("UPDATE quotes SET last_follow_up_at = NOW(), follow_up_count = follow_up_count + 1 WHERE id = $1")
                        .bind(quote_id)
                        .execute(&self.db.pool).await.map_err(|e| e.to_string())?;
                },
                crate::db::DbStore::Sqlite(pool) => {
                     sqlx::query(
                        "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                    )
                    .bind(&agent_feed_item_id)
                    .bind(&tenant_id)
                    .bind("deposit_follow_up")
                    .bind(serde_json::json!({
                        "quote_id": quote_id,
                        "customer_name": customer_name,
                        "amount_cents": amount
                    }).to_string())
                    .bind(serde_json::json!({
                        "action_type": "Draft Follow-up",
                        "draft_reply": follow_up_msg,
                        "quote_id": quote_id
                    }).to_string())
                    .execute(pool).await.map_err(|e| e.to_string())?;

                    sqlx::query("UPDATE quotes SET last_follow_up_at = CURRENT_TIMESTAMP, follow_up_count = follow_up_count + 1 WHERE id = ?")
                        .bind(quote_id.to_string())
                        .execute(pool).await.map_err(|e| e.to_string())?;
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[tokio::test]
    async fn test_follow_up_message_generation() {
        let customer_name = "Carlos";
        let amount = 15000;
        let follow_up_msg = format!(
            "Hi {}, just following up on the estimate for ${:.2}. Let me know if you have any questions or are ready to move forward with the deposit!",
            customer_name, (amount as f64) / 100.0
        );
        assert_eq!(follow_up_msg, "Hi Carlos, just following up on the estimate for $150.00. Let me know if you have any questions or are ready to move forward with the deposit!");
    }
}
