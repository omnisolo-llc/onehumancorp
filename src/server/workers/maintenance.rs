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
            let mut interval = interval(Duration::from_secs(60)); // Check every minute
            loop {
                interval.tick().await;
                if let Err(e) = self.db.cleanup_stagnant_missions(60).await { // 60s timeout
                    tracing::error!("MaintenanceWorker: Failed to cleanup stagnant missions: {}", e);
                }
            }
        });
    }
}
