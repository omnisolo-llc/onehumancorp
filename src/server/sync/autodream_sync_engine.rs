use std::sync::Arc;
use tokio::time::{interval, Duration};
use crate::db::DB;
use sqlx::Row;

pub struct AutoDreamSyncEngine {
    db: Arc<DB>,
}

impl AutoDreamSyncEngine {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                if let Err(e) = self.process_forecast_tick().await {
                    tracing::error!("AutoDreamSyncEngine: error during sync: {}", e);
                }
            }
        });
    }

    pub async fn process_forecast_tick(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if !self.db.is_sqlite() {
            return Ok(()); // Only run sync in standalone mode (SQLite)
        }

        let is_standalone = std::env::var("OHC_STANDALONE").unwrap_or_else(|_| "false".to_string()) == "true";
        if !is_standalone {
            return Ok(());
        }

        let pool = match &self.db.store {
            crate::db::DbStore::Sqlite(pool) => pool,
            _ => return Ok(()),
        };

        // Fetch records not synced to cloud
        let rows = sqlx::query(
            "SELECT id FROM autodream_memories WHERE synced_to_cloud = false LIMIT 100"
        )
        .fetch_all(pool)
        .await?;

        let mut synced_count = 0;
        let mut failed_count = 0;

        for row in rows {
            let id: String = row.try_get("id")?;

            // In a real implementation, we would send this to the cloud API
            // For now, we simulate a successful sync
            let sync_success = true;

            if sync_success {
                let update_result = sqlx::query(
                    "UPDATE autodream_memories SET synced_to_cloud = true WHERE id = ?"
                )
                .bind(&id)
                .execute(pool)
                .await;

                if update_result.is_ok() {
                    synced_count += 1;
                } else {
                    failed_count += 1;
                }
            } else {
                failed_count += 1;
            }
        }

        crate::telemetry::record_sync_completed_count(synced_count);
        crate::telemetry::record_sync_failed_count(failed_count);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_process_forecast_tick() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE autodream_memories (
                id TEXT PRIMARY KEY,
                synced_to_cloud BOOLEAN DEFAULT false
            )"
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query("INSERT INTO autodream_memories (id, synced_to_cloud) VALUES ('1', false), ('2', false), ('3', true)")
            .execute(&pool)
            .await
            .unwrap();

        std::env::set_var("OHC_STANDALONE", "true");

        let db = Arc::new(DB {
            pool: sqlx::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
            store: crate::db::DbStore::Sqlite(pool.clone()),
        });

        let engine = Arc::new(AutoDreamSyncEngine::new(db));

        engine.process_forecast_tick().await.unwrap();

        let unsynced_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM autodream_memories WHERE synced_to_cloud = false")
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(unsynced_count, 0);
    }
}
