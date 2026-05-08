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
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(300)); // Every 5 minutes
            loop {
                interval.tick().await;
                if let Err(e) = self.db.cleanup_stagnant_missions(3600).await { // 1 hour timeout
                    tracing::error!("MaintenanceWorker: Failed to cleanup stagnant missions: {}", e);
                }

                // Integrate with SipDB pruning logic for deeper queue sanitization if possible
                // SipDB::new requires org_id, so we perform generic pruning across all reachable missions in DB.
                // Mission prioritization and stuck-to-failed transitions are handled by cleanup_stagnant_missions in db.rs
                // but we can ensure a more aggressive sweep here if needed.
            }
        });
    }
}
