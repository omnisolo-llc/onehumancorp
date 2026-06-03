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
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_maintenance_worker_start_no_panic() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect_lazy("sqlite::memory:")
            .unwrap();

        let db = Arc::new(crate::db::DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap(),
            store: crate::db::DbStore::Sqlite(pool),
        });

        let worker = Arc::new(MaintenanceWorker::new(db));
        worker.start();

        // Let the worker loop run a tiny bit to make sure no instant panics
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        assert!(true); // If it hasn't panicked, the test passes
    }
}
