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

                // Prioritize backlog by moving stuck missions to failed or bursting
                if let Err(e) = self.db.sanitize_backlog().await {
                    tracing::error!("MaintenanceWorker: Failed to sanitize backlog: {}", e);
                }

                if let Err(e) = self.db.cleanup_stagnant_missions(3600).await { // 1 hour timeout
                    tracing::error!("MaintenanceWorker: Failed to cleanup stagnant missions: {}", e);
                }
            }
        });
    }
}
