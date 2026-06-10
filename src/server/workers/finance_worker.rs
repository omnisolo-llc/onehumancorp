use std::sync::Arc;
use crate::db::DB;


pub struct FinanceWorker {
    db: Arc<DB>,
}

impl FinanceWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn handle(&self, job: crate::queue::Job) -> Result<Result<(), String>, String> {
        let payload: serde_json::Value = serde_json::from_str(&job.payload).unwrap();

        let expense_id = payload.get("expense_id").and_then(|v| v.as_str()).unwrap_or("");

        // Simulating the OCR output
        let mut tx = match self.db.pool.begin().await {
            Ok(tx) => tx,
            Err(e) => {
                tracing::error!("Failed to begin transaction: {}", e);
                return Err("Failed to begin db transaction".into());
            }
        };

        if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &job.tenant_id).await {
            tracing::error!("Failed to set org context: {}", e);
            return Err("Failed to set org context".into());
        }

        // Dummy processing logic for OCR extracting values
        let amount_cents = 4500;
        let merchant = "Home Depot";
        let category = "Supplies";

        sqlx::query("UPDATE expenses SET amount_cents = $1, merchant = $2, category = $3 WHERE id = $4")
            .bind(amount_cents)
            .bind(merchant)
            .bind(category)
            .bind(expense_id)
            .execute(&mut *tx)
            .await
            .unwrap();

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(Ok(()))
    }
}
