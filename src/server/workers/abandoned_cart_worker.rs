use std::sync::Arc;
use sqlx::PgPool;
use std::time::Duration;
use uuid::Uuid;

use crate::db::DB;

pub struct AbandonedCartWorker {
    pub pool: Arc<PgPool>,
}

impl AbandonedCartWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { pool: Arc::new(db.pool.clone()) }
    }

    pub async fn run_scan(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Find abandoned carts that haven't been drafted for recovery yet
        let pending_carts: Vec<(String, String, String, Option<String>, Option<i64>)> = sqlx::query_as(
            r#"
            SELECT c.id, ac.id, ac.tenant_id, c.customer_id, c.total_amount_cents
            FROM abandoned_carts ac
            JOIN carts c ON ac.cart_id = c.id
            WHERE ac.status = 'PENDING'
            LIMIT 50
            "#
        )
        .fetch_all(&*self.pool)
        .await?;

        for (cart_id, abandoned_cart_id, tenant_id, customer_id, total_amount_cents) in pending_carts {
            let feed_item_id = Uuid::new_v4().to_string();

            // Build the recovery draft string (simulated LLM generation for now, real deployment could call llm logic here)
            let recovery_message = format!("Hi there, we noticed you left some items in your cart. Your total is ${:.2}. Would you like to complete your order?", total_amount_cents.unwrap_or(0) as f64 / 100.0);

            let context = serde_json::json!({
                "cart_id": cart_id,
                "customer_id": customer_id,
                "amount_cents": total_amount_cents,
            });

            let action = serde_json::json!({
                "feature_type": "cart_recovery.dispatch",
                "message": recovery_message,
                "cart_id": cart_id,
                "abandoned_cart_id": abandoned_cart_id,
            });

            // Insert into agent_feed_items
            let _ = sqlx::query(
                r#"
                INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state)
                VALUES ($1, $2, 'Customer Success', $3, $4, 'PENDING_APPROVAL')
                "#
            )
            .bind(&feed_item_id)
            .bind(&tenant_id)
            .bind(sqlx::types::Json(context))
            .bind(sqlx::types::Json(action))
            .execute(&*self.pool)
            .await;

            // Mark abandoned_cart as processed
            let _ = sqlx::query(
                "UPDATE abandoned_carts SET status = 'DRAFTED' WHERE id = $1 AND tenant_id = $2"
            )
            .bind(&abandoned_cart_id)
            .bind(&tenant_id)
            .execute(&*self.pool)
            .await;

            // Trigger mesh notification
            let cache = crate::api::agent_feed::get_agent_feed_cache();
            cache.invalidate_by_tag(&format!("agent_feed_tenant:{}", tenant_id)).await;

            let client = crate::api::agent_feed::get_redis_client();
            let topic = format!("agent_feed:{}", tenant_id);
            if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                let _: Result<(), _> = redis::cmd("PUBLISH").arg(topic).arg("{\"event\":\"new_draft\"}").query_async(&mut conn).await;
            }
        }

        Ok(())
    }

    pub fn start_background_loop(self: Arc<Self>) {
        tokio::spawn(async move {
            loop {
                if let Err(e) = self.run_scan().await {
                    tracing::error!("Abandoned cart worker error: {}", e);
                }
                tokio::time::sleep(Duration::from_secs(60)).await;
            }
        });
    }
}
