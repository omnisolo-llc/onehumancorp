use crate::db::DB;
use std::sync::Arc;

use crate::api::terminal_offline_authority as offline_payment_authority;

pub struct PosSyncWorker {
    db: Arc<DB>,
}

impl PosSyncWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl crate::queue::TaskJobHandler for PosSyncWorker {
    async fn handle(&self, job: crate::queue::Job) -> Result<(), String> {
        let payload: serde_json::Value = serde_json::from_str(&job.payload)
            .map_err(|_| "Offline payload is invalid; retain the original job for review")?;
        // Canonical producers already commit inventory and recorded cash sales.
        // Only their tenant-owned receipts authorize completing the queue job.
        offline_payment_authority::complete_committed_operation(
            &self.db.pool,
            &job.tenant_id,
            &payload,
        )
        .await
    }
}
