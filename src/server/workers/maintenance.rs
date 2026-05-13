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
    use crate::db::{DB, DbStore};

    #[tokio::test]
    async fn test_maintenance_worker_starts() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
        if !db_url.starts_with("sqlite") && std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let pool = sqlx::sqlite::SqlitePoolOptions::new().max_connections(1)
            .connect_lazy("sqlite::memory:")
            .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
            store: DbStore::Sqlite(pool),

        });

        let worker = Arc::new(MaintenanceWorker::new(db.clone()));
        worker.start();

        // Let it run for a moment to ensure it doesn't immediately crash
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }
}
