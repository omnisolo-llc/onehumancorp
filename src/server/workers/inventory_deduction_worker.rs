use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use uuid::Uuid;
use tokio::time::timeout;

pub struct InventoryDeductionWorker {
    pub db: Arc<DB>,
}

impl InventoryDeductionWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(500));
            loop {
                interval.tick().await;

                // Dequeue INVENTORY_DEDUCTION_LEDGER
                let job_row = sqlx::query(
                    "UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP
                     WHERE id = (
                         SELECT id FROM ohc_job_queue
                         WHERE status = 'PENDING' AND job_type = 'INVENTORY_DEDUCTION_LEDGER' AND next_retry_at <= CURRENT_TIMESTAMP
                         ORDER BY next_retry_at ASC
                         LIMIT 1
                         FOR UPDATE SKIP LOCKED
                     ) RETURNING id, tenant_id, payload"
                )
                .fetch_optional(&db.pool)
                .await
                .unwrap_or(None);

                if let Some(row) = job_row {
                    use sqlx::Row;
                    let job_id: String = row.get("id");
                    let tenant_id: String = row.get("tenant_id");
                    let payload: serde_json::Value = row.get("payload");

                    let payload_str = payload.to_string();

                    let res = sqlx::query("INSERT INTO ohc_universal_ledger (id, tenant_id, department, action_type, state_change) VALUES ($1, $2, 'Operations', 'INVENTORY_DEDUCTION', $3::jsonb)")
                        .bind(Uuid::new_v4().to_string())
                        .bind(&tenant_id)
                        .bind(&payload_str)
                        .execute(&db.pool)
                        .await;

                    if res.is_ok() {
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                            .bind(&job_id)
                            .execute(&db.pool)
                            .await;
                    } else {
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                            .bind(&job_id)
                            .execute(&db.pool)
                            .await;
                    }
                }
            }
        });
    }
}
