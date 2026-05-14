use sqlx::{PgPool, SqlitePool};
use tracing::{info, error, warn};

pub struct MemoryPruner {
    pub cloud_pool: Option<PgPool>,
    pub standalone_pool: Option<SqlitePool>,
    pub threshold_days: i64,
}

impl MemoryPruner {
    pub fn new(cloud_pool: Option<PgPool>, standalone_pool: Option<SqlitePool>, threshold_days: i64) -> Self {
        Self { cloud_pool, standalone_pool, threshold_days }
    }

    pub async fn run_prune_pass(&self) -> Result<u64, sqlx::Error> {
        let mut total_pruned = 0;

        if let Some(pool) = &self.cloud_pool {
            match sqlx::query!("DELETE FROM vector_memory WHERE last_accessed < NOW() - INTERVAL '$1 days' RETURNING id", self.threshold_days as f64).fetch_all(pool).await {
                Ok(rows) => total_pruned += rows.len() as u64,
                Err(e) => error!("Failed cloud prune: {}", e),
            }
        }

        if let Some(pool) = &self.standalone_pool {
            let limit = chrono::Utc::now() - chrono::Duration::days(self.threshold_days);
            match sqlx::query("DELETE FROM vector_memory WHERE last_accessed < ?").bind(limit).execute(pool).await {
                Ok(res) => total_pruned += res.rows_affected(),
                Err(e) => error!("Failed standalone prune: {}", e),
            }
        }

        Ok(total_pruned)
    }
}
