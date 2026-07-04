use sqlx::PgPool;
use tracing::{info, error};
use std::time::Duration;
use uuid::Uuid;
use chrono::Utc;

pub struct EscalationWorker {
    pool: PgPool,
    sla_threshold_minutes: i64,
}

impl EscalationWorker {
    pub fn new(pool: PgPool, sla_threshold_minutes: i64) -> Self {
        Self { pool, sla_threshold_minutes }
    }

    pub async fn run(&self) {
        info!("Starting EscalationWorker loop");
        loop {
            if let Err(e) = self.process_sla_violations().await {
                error!("Error processing SLA violations: {}", e);
            }
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    }

    async fn process_sla_violations(&self) -> Result<(), String> {
        let threshold_time = Utc::now() - chrono::Duration::minutes(self.sla_threshold_minutes);

        // Find pending orders that have exceeded the SLA threshold
        let violations: Vec<(String, String, serde_json::Value)> = sqlx::query_as(
            r#"
            SELECT o.id, o.tenant_id,
                   jsonb_build_object('order_id', o.id, 'customer_id', o.customer_id, 'total_amount', o.total_amount, 'status', o.status, 'created_at', o.created_at)
            FROM orders o
            WHERE o.status = 'pending' AND o.created_at < $1
            AND NOT EXISTS (
                SELECT 1 FROM agent_feed_items afi
                WHERE afi.tenant_id = o.tenant_id
                AND afi.event_source = 'order_sla_breach'
                AND afi.context_payload->>'order_id' = o.id
            )
            "#
        )
        .bind(threshold_time)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        for (order_id, tenant_id, payload) in violations {
            info!("SLA violation detected for order {} tenant {}", order_id, tenant_id);

            let agent_feed_service = crate::services::agent_feed::service::AgentFeedService::new(self.pool.clone());

            match agent_feed_service.process_event(&tenant_id, "order_sla_breach", &payload).await {
                Ok(_) => info!("Successfully escalated order {}", order_id),
                Err(e) => error!("Failed to escalate order {}: {}", order_id, e),
            }
        }

        Ok(())
    }
}
