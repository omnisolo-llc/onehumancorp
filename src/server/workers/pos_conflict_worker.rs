use std::sync::Arc;
use crate::db::DB;
use crate::queue::{TaskJobHandler as JobHandler, Job as OHCJob};
use serde_json::json;
use async_trait::async_trait;

pub struct PosConflictWorker {
    pub db: Arc<DB>,
}

impl PosConflictWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl JobHandler for PosConflictWorker {
    async fn handle(&self, job: OHCJob) -> Result<(), String> {
        let db = self.db.clone();

        let payload: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or(json!({}));
        let product_id = payload.get("product_id").and_then(|v| v.as_str()).unwrap_or("");
        let transaction_id = payload.get("transaction_id").and_then(|v| v.as_str()).unwrap_or("");
        let tenant_id = job.tenant_id.clone();

        let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;

        let order_row = sqlx::query(
            "SELECT o.id FROM orders o
             JOIN order_items oi ON o.id = oi.order_id
             WHERE oi.product_id = $1 AND o.tenant_id = $2 AND o.status = 'PENDING'
             LIMIT 1"
        )
        .bind(product_id)
        .bind(&tenant_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let message = if let Some(row) = order_row {
            use sqlx::Row;
            let order_id: String = row.get("id");

            sqlx::query("UPDATE orders SET status = 'Requires Intervention' WHERE id = $1")
                .bind(&order_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

            format!("An in-store sale overlapped with an online order ({}). Operations has secured the in-store sale. Customer Success has drafted an apology and alternative offer for the online customer.", order_id)
        } else {
            "An in-store sale overlapped with an online order. Operations has secured the in-store sale. Please review recent online orders to address the out-of-stock item.".to_string()
        };

        let state_change = json!({
            "job_id": job.id,
            "product_id": product_id,
            "transaction_id": transaction_id,
            "message": message,
            "status": "Requires Intervention"
        });

        let ledger_id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO ohc_universal_ledger (id, tenant_id, department, action_type, state_change)
             VALUES ($1, $2, 'Operations', 'pos_conflict_resolution', $3::jsonb)"
        )
        .bind(&ledger_id)
        .bind(&tenant_id)
        .bind(&state_change)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let event_id = uuid::Uuid::new_v4().to_string();
        let event_payload = json!({
            "product_id": product_id,
            "transaction_id": transaction_id,
            "message": message
        });
        sqlx::query(
            "INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status)
             VALUES ($1, $2, 'operations', 'InventoryConflictEvent', $3::jsonb, 'PENDING')"
        )
        .bind(&event_id)
        .bind(&tenant_id)
        .bind(&event_payload)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_pos_conflict_worker() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
        let db = Arc::new(DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });
        let worker = PosConflictWorker::new(db.clone());

        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-conflict-test', 'Test') ON CONFLICT DO NOTHING").execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO orders (id, tenant_id, status) VALUES ('order-1', 'tenant-conflict-test', 'PENDING') ON CONFLICT DO NOTHING").execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO order_items (id, tenant_id, order_id, product_id) VALUES ('oi-1', 'tenant-conflict-test', 'order-1', 'prod-1') ON CONFLICT DO NOTHING").execute(&pool).await.unwrap();

        let job = OHCJob {
            id: "job-c1".to_string(),
            tenant_id: "tenant-conflict-test".to_string(),
            job_type: "POS_INVENTORY_CONFLICT_RESOLUTION".to_string(),
            payload: json!({"product_id": "prod-1", "transaction_id": "tx-1"}).to_string(),
            status: "PENDING".to_string(),
            retry_count: 0,
            max_retries: 3,
            next_retry_at: chrono::Utc::now(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            parent_task_id: "".to_string(),
            locked_until: None,
        };

        let res = worker.handle(job).await;
        assert!(res.is_ok());

        let status: (String,) = sqlx::query_as("SELECT status FROM orders WHERE id = 'order-1'").fetch_one(&pool).await.unwrap();
        assert_eq!(status.0, "Requires Intervention");

        let ledger_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM ohc_universal_ledger WHERE action_type = 'pos_conflict_resolution' AND tenant_id = 'tenant-conflict-test'").fetch_one(&pool).await.unwrap();
        assert_eq!(ledger_count.0, 1);
    }
}
