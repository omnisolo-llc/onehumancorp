
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::db::{DB, DbStore};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncDelta {
    pub id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub payload: serde_json::Value,
    pub updated_at: i64,
}

#[async_trait::async_trait]
pub trait SyncDeltas {
    async fn sync_deltas(&self, deltas: Vec<SyncDelta>) -> Result<(), String>;
}

pub struct CloudSyncService {
    db: Arc<DB>,
}

impl CloudSyncService {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl SyncDeltas for CloudSyncService {
    async fn sync_deltas(&self, deltas: Vec<SyncDelta>) -> Result<(), String> {
        let is_standalone = crate::is_standalone_runtime();
        let telemetry_enabled = ::server_config::is_telemetry_enabled();

        if is_standalone && !telemetry_enabled {
            tracing::debug!("Standalone mode, telemetry disabled, skipping cloud delta sync entirely to enforce local sovereignty.");
            return Ok(());
        }

        let mut futures = Vec::new();
        for delta in deltas {
            let db_clone = self.db.clone();
            futures.push(async move {
                let payload_str = serde_json::to_string(&delta.payload).map_err(|e| e.to_string())?;
                if db_clone.is_sqlite() {
                    if let DbStore::Sqlite(ref pool) = db_clone.store {
                        let query = r#"
                            INSERT INTO mcp_sync_deltas (id, entity_type, entity_id, payload, updated_at)
                            VALUES ($1, $2, $3, $4, $5)
                            ON CONFLICT (id) DO UPDATE SET
                            payload = excluded.payload,
                            updated_at = excluded.updated_at
                            WHERE mcp_sync_deltas.updated_at < excluded.updated_at
                        "#;
                        sqlx::query(query)
                            .bind(&delta.id)
                            .bind(&delta.entity_type)
                            .bind(&delta.entity_id)
                            .bind(&payload_str)
                            .bind(delta.updated_at)
                            .execute(pool)
                            .await
                            .map_err(|e| e.to_string())?;
                    }
                } else {
                    let query = r#"
                        INSERT INTO mcp_sync_deltas (id, entity_type, entity_id, payload, updated_at)
                        VALUES ($1, $2, $3, $4, $5)
                        ON CONFLICT (id) DO UPDATE SET
                        payload = EXCLUDED.payload,
                        updated_at = EXCLUDED.updated_at
                        WHERE mcp_sync_deltas.updated_at < EXCLUDED.updated_at
                    "#;
                    sqlx::query(query)
                        .bind(&delta.id)
                        .bind(&delta.entity_type)
                        .bind(&delta.entity_id)
                        .bind(&payload_str)
                        .bind(delta.updated_at)
                        .execute(&db_clone.pool)
                        .await
                        .map_err(|e| e.to_string())?;
                }
                Ok::<(), String>(())
            });
        }

        let results = futures::future::join_all(futures).await;
        for res in results {
            res?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {

    // We mock the DB for tests, but doing so via integration tests is better.
    // For unit testing here, we avoid starting a full postgres instance by relying on standard traits or a simple mock.
}
