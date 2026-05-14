use chrono::{DateTime, Utc};
use sqlx::Row;
use crate::memory_store::types::{EmbeddingRecord, VectorMemoryStore};
use crate::memory_store::repository::VectorRepository;

impl VectorRepository {
    pub async fn prune_stale(&self, older_than: DateTime<Utc>) -> Result<(), String> {
        match &self.store {
            VectorMemoryStore::Postgres(pool) => {
                sqlx::query("DELETE FROM consolidated_memory WHERE (last_referenced_at < $1 AND owner_override = FALSE AND reference_count < 5 AND source_type = 'TASK_SUMMARY') OR (reliability_score < 20 AND owner_override = FALSE)")
                    .bind(older_than)
                    .execute(&*pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            VectorMemoryStore::Sqlite(pool) => {
                sqlx::query("DELETE FROM consolidated_memory WHERE (last_referenced_at < ? AND owner_override = FALSE AND reference_count < 5 AND source_type = 'TASK_SUMMARY') OR (reliability_score < 20 AND owner_override = FALSE)")
                    .bind(older_than)
                    .execute(&*pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
}
