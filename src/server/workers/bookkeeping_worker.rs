use std::sync::Arc;
use tokio::time::{interval, Duration};
use crate::db::DB;
use crate::services::bookkeeping::BookkeepingService;
use sqlx::Row;

pub struct BookkeepingWorker {
    db: Arc<DB>,
}

impl BookkeepingWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(3600)); // Run every hour
            loop {
                interval.tick().await;
                if let Err(e) = self.run_batch().await {
                    tracing::error!("Bookkeeping worker error: {}", e);
                }
            }
        });
    }

    async fn run_batch(&self) -> Result<(), String> {
        tracing::info!("Running bookkeeping worker batch...");

        // Find all tenants that need an update.
        // For simplicity, we just fetch all tenants.
        let tenants: Vec<String> = match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query("SELECT tenant_id FROM tenants")
                    .fetch_all(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?
                    .into_iter()
                    .map(|r| r.get(0))
                    .collect()
            }
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query("SELECT tenant_id FROM tenants")
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?
                    .into_iter()
                    .map(|r| r.get(0))
                    .collect()
            }
        };

        let service = BookkeepingService::new(self.db.clone());

        for tenant_id in tenants {
            // Check if we should generate an insight (e.g. if one doesn't exist for today)
            // For now, we just try to generate it.
            if let Err(e) = service.generate_insight(&tenant_id).await {
                tracing::warn!("Failed to generate insight for tenant {}: {}", tenant_id, e);
            }
        }

        Ok(())
    }
}
