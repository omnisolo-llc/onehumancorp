use std::sync::Arc;
use tokio::time::{interval, Duration};
use crate::db::DbStore;

pub struct ProactiveAgentWorker {
    pub db: Arc<crate::db::DB>,
}

impl ProactiveAgentWorker {
    pub fn new(db: Arc<crate::db::DB>) -> Self {
        Self { db }
    }

    pub fn start(&self) {
        let db = self.db.clone();

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(30)); // Run every 30s to scan

            loop {
                interval.tick().await;
                if let Err(e) = Self::scan_for_proactive_actions(&db).await {
                    tracing::error!("ProactiveAgentWorker scan failed: {}", e);
                }
            }
        });
    }

    async fn scan_for_proactive_actions(db: &crate::db::DB) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if matches!(&db.store, DbStore::Postgres) {
            use sqlx::Row;
            // Find products with low inventory that don't already have an active proactive approval
            let rows = sqlx::query(
                "SELECT id, tenant_id, name, inventory_count FROM products
                 WHERE inventory_count < 10 AND item_type = 'Product'
                 AND NOT EXISTS (
                     SELECT 1 FROM agent_approvals
                     WHERE department = 'proactive'
                     AND payload->>'item_id' = products.id
                 )
                 LIMIT 50"
            )
            .fetch_all(&db.pool)
            .await?;

            for row in rows {
                let id = uuid::Uuid::new_v4().to_string();
                let product_id: String = row.try_get("id").unwrap_or_default();
                let tenant_id: String = row.try_get("tenant_id").unwrap_or_default();
                let name: String = row.try_get("name").unwrap_or_default();

                let payload_json = serde_json::json!({
                    "title": "Low Inventory Alert",
                    "item_id": product_id,
                    "item_name": name,
                    "trigger": "low_inventory"
                });

                let desc = format!("{} is running low. Drafted email to supplier to restock. Send?", name);

                let _ = sqlx::query(
                    "INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload) VALUES ($1, $2, 'proactive', $3, 'DRAFT', 'LOW', $4)"
                )
                .bind(&id)
                .bind(&tenant_id)
                .bind(&desc)
                .bind(&payload_json)
                .execute(&db.pool)
                .await;
            }
        }
        Ok(())
    }
}
