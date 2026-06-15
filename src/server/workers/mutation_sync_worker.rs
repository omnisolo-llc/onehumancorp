use std::sync::Arc;
use crate::db::DB;
use crate::queue::{TaskJobHandler as JobHandler, Job as OHCJob};
use async_trait::async_trait;

pub struct MutationSyncWorker {
    pub db: Arc<DB>,
}

impl MutationSyncWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl JobHandler for MutationSyncWorker {
    async fn handle(&self, job: OHCJob) -> Result<(), String> {
        let payload: serde_json::Value = serde_json::from_str(&job.payload).map_err(|e| e.to_string())?;
        let action_type = payload.get("action_type").and_then(|v| v.as_str()).unwrap_or("");
        let sync_event_id = payload.get("sync_event_id").and_then(|v| v.as_str()).unwrap_or("");

        let mut tx = match self.db.pool.begin().await {
            Ok(tx) => tx,
            Err(e) => return Err(format!("Failed to begin transaction: {}", e)),
        };

        if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &job.tenant_id).await {
            return Err(format!("Failed to set org context: {}", e));
        }

        if action_type == "InventoryUpdate" {
            // General generic handling, forward to pos logic or standard conflict event
            let event_payload = payload.get("payload").and_then(|v| v.as_str()).unwrap_or("");
            if let Ok(data) = serde_json::from_str::<serde_json::Value>(event_payload) {
                let product_id = data.get("product_id").and_then(|v| v.as_str()).unwrap_or("");
                let qty = data.get("quantity").and_then(|v| v.as_i64()).unwrap_or(0);

                let current_stock_res = sqlx::query("SELECT available_quantity FROM products WHERE id = $1 AND tenant_id = $2 FOR UPDATE")
                    .bind(product_id)
                    .bind(&job.tenant_id)
                    .fetch_optional(&mut *tx)
                    .await;

                if let Ok(Some(row)) = current_stock_res {
                    use sqlx::Row;
                    let stock: i32 = row.get("available_quantity");
                    if stock < qty as i32 {
                        // Conflict
                        let notification_id = uuid::Uuid::new_v4().to_string();
                        let notification_payload = serde_json::json!({
                            "product_id": product_id,
                            "expected_stock": qty,
                            "actual_stock": stock,
                            "message": format!("Inventory Sync Conflict: {} sold out offline.", product_id),
                            "transaction_id": sync_event_id
                        }).to_string();

                        let _ = sqlx::query(
                            "INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status)
                             VALUES ($1, $2, 'operations', 'InventoryConflictEvent', $3::jsonb, 'PENDING')"
                        )
                        .bind(&notification_id)
                        .bind(&job.tenant_id)
                        .bind(&notification_payload)
                        .execute(&mut *tx)
                        .await;
                    }
                }
            }
        }

        let _ = sqlx::query("UPDATE sync_events SET synced_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(sync_event_id)
            .execute(&mut *tx)
            .await;

        let _ = tx.commit().await;

        Ok(())
    }
}
