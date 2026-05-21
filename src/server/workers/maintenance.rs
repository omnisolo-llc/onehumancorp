use std::sync::Arc;
use crate::db::DB;
use tokio::time::{interval, Duration};

pub struct MaintenanceWorker {
    db: DB,
}

impl MaintenanceWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db: (*db).clone() }
    }

    pub fn start(self: Arc<Self>) {
        let db_arc = Arc::new(self.db.clone());
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(300)); // Every 5 minutes
            loop {
                interval.tick().await;

                // Consolidate mission cleanup into SipDB for robust PENDING->STUCK->FAILED transitions
                let sip_db = crate::orchestration::sip::SipDB::new(db_arc.clone());
                if let Err(e) = sip_db.prune_all_tenants(chrono::Duration::hours(24)).await {
                    tracing::error!("MaintenanceWorker: Failed to prune stale missions: {}", e);
                }

                if let Err(e) = db_arc.cleanup_stagnant_missions(3600).await { // 1 hour timeout (legacy fallback)
                    tracing::error!("MaintenanceWorker: Failed to cleanup stagnant missions: {}", e);
                }
            }
        });
    }
}
