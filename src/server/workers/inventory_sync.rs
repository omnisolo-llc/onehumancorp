use sqlx::{PgPool, Row};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InventoryUpdateJob {
    pub tenant_id: String,
    pub product_id: String,
    pub variant_id: Option<String>,
    pub delta: i32,
}

pub struct InventorySyncWorker {
    pool: PgPool,
}

impl InventorySyncWorker {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn run_worker_loop(&self) {
        loop {
            match self.process_next_job().await {
                Ok(true) => {
                    // Processed a job, loop immediately to check for more
                    continue;
                }
                Ok(false) => {
                    // No jobs available, back off
                    sleep(Duration::from_millis(1000)).await;
                }
                Err(e) => {
                    tracing::error!("InventorySyncWorker error: {}", e);
                    sleep(Duration::from_millis(5000)).await;
                }
            }
        }
    }

    async fn process_next_job(&self) -> Result<bool, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        // 1. Fetch job with strict isolation
        let job_row = sqlx::query(
            "SELECT id, tenant_id, payload FROM ohc_job_queue
             WHERE status = 'PENDING' AND job_type = 'inventory_sync'
             ORDER BY created_at ASC
             FOR UPDATE SKIP LOCKED LIMIT 1"
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let row = match job_row {
            Some(r) => r,
            None => {
                let _ = tx.rollback().await;
                return Ok(false);
            }
        };

        let job_id: String = row.get("id");
        let tenant_id: String = row.get("tenant_id");
        let payload_json: serde_json::Value = row.get("payload");

        // Mark as processing
        sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING' WHERE id = $1")
            .bind(&job_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        let update_job: InventoryUpdateJob = match serde_json::from_value(payload_json) {
            Ok(job) => job,
            Err(_) => {
                // Invalid payload, mark failed
                sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED' WHERE id = $1")
                    .bind(&job_id)
                    .execute(&mut *tx)
                    .await
                    .ok();
                let _ = tx.commit().await;
                return Ok(true);
            }
        };

        // 2. Apply change to products table
        let query = "
            UPDATE products
            SET inventory_count = GREATEST(0, inventory_count + $1)
            WHERE id = $2 AND tenant_id = $3
            RETURNING inventory_count
        ";

        let update_res = sqlx::query(query)
            .bind(update_job.delta)
            .bind(&update_job.product_id)
            .bind(&tenant_id)
            .fetch_optional(&mut *tx)
            .await;

        let new_inventory_count: i32 = match update_res {
            Ok(Some(r)) => r.try_get("inventory_count").unwrap_or(0),
            _ => {
                sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED' WHERE id = $1")
                    .bind(&job_id)
                    .execute(&mut *tx)
                    .await
                    .ok();
                let _ = tx.commit().await;
                return Ok(true);
            }
        };

        // Update product_variants if variant_id provided
        if let Some(vid) = update_job.variant_id.clone() {
             sqlx::query("UPDATE product_variants SET inventory_count = GREATEST(0, inventory_count + $1) WHERE id = $2 AND tenant_id = $3")
                .bind(update_job.delta)
                .bind(&vid)
                .bind(&tenant_id)
                .execute(&mut *tx)
                .await
                .ok(); // Ignore errors on variant for now to ensure ledger updates
        }

        // 3. Record in inventory_ledger
        let _ = sqlx::query(
            "INSERT INTO inventory_ledger (id, tenant_id, product_id, variant_id, quantity, version) VALUES ($1, $2, $3, $4, $5, 1)"
        )
        .bind(Uuid::new_v4().to_string())
        .bind(&tenant_id)
        .bind(&update_job.product_id)
        .bind(update_job.variant_id)
        .bind(update_job.delta)
        .execute(&mut *tx)
        .await;

        // 4. AI Operations Agent integration
        if new_inventory_count <= 5 && update_job.delta < 0 {
             let agent_payload = serde_json::json!({
                "workflow": "ohc_business_swarm",
                "task": "Inventory alert",
                "context": format!("Product {} dropped to low inventory ({} remaining)", update_job.product_id, new_inventory_count),
                "action": "OperationsAgent: generate a plain-language alert for the business owner and trigger a restock reminder."
            }).to_string();

            let _ = sqlx::query(
                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload) VALUES ($1, $2, 'agent_task', $3::jsonb)"
            )
            .bind(Uuid::new_v4().to_string())
            .bind(&tenant_id)
            .bind(agent_payload)
            .execute(&mut *tx)
            .await;
        }

        // 5. Complete job
        sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED' WHERE id = $1")
            .bind(&job_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        let _ = tx.commit().await;
        Ok(true)
    }
}
