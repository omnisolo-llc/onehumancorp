use crate::db::DB;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct AutoDream {
    pub id: String,
    pub entity_type: String,
    pub payload: serde_json::Value,
}

pub async fn process_forecast_tick(db: Arc<DB>) -> Result<(), Box<dyn std::error::Error>> {
    if !db.is_sqlite() {
        return Ok(());
    }

    if let crate::db::DbStore::Sqlite(ref pool) = db.store {
        let mut payloads = Vec::new();

        let embedding_fut = sqlx::query(
            r#"
            SELECT id, prompt FROM embedding_cache
            WHERE synced_to_cloud = 0
            "#,
        )
        .fetch_all(pool);

        let mission_fut = sqlx::query(
            r#"
            SELECT id, payload FROM agent_missions
            WHERE synced_to_cloud = 0
            "#,
        )
        .fetch_all(pool);

        let (embedding_res, mission_res) = tokio::join!(embedding_fut, mission_fut);
        let embedding_rows = embedding_res?;
        let mission_rows = mission_res?;

        for row in &embedding_rows {
            let id: String = row.try_get("id")?;
            let prompt: String = row.try_get("prompt")?;
            payloads.push(AutoDream {
                id,
                entity_type: "embedding_cache".to_string(),
                payload: serde_json::json!({ "prompt": prompt }),
            });
        }

        for row in &mission_rows {
            let id: String = row.try_get("id")?;
            let payload_str: String = row.try_get("payload")?;
            let parsed_payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));
            payloads.push(AutoDream {
                id,
                entity_type: "agent_mission".to_string(),
                payload: parsed_payload,
            });
        }

        if payloads.is_empty() {
            return Ok(());
        }

        let cloud_url = std::env::var("OHC_CLOUD_URL").unwrap_or_else(|_| "https://api.onehumancorp.com".to_string());
        let sync_url = format!("{}/api/v1/sync/autodream", cloud_url);

        let client = reqwest::Client::new();
        // In tests, if OHC_TEST_BYPASS_HTTP is set, we bypass HTTP and assume success.
        let mut sync_successful = std::env::var("OHC_TEST_BYPASS_HTTP").is_ok();

        if !sync_successful {
            match client.post(&sync_url).json(&payloads).send().await {
                Ok(resp) if resp.status().is_success() => {
                    sync_successful = true;
                }
                Ok(resp) => {
                    ::server_telemetry::record_error_signal("[bug] Cloud sync failed with status");
                    tracing::error!("Cloud sync failed with status: {}", resp.status());
                }
                Err(e) => {
                    ::server_telemetry::record_error_signal("[bug] Cloud sync request failed");
                    tracing::error!("Cloud sync request failed: {}", e);
                }
            }
        }

        let mut synced_embeddings = 0;
        let mut failed_embeddings = 0;
        let mut synced_missions = 0;
        let mut failed_missions = 0;

        if sync_successful {
            let mut embedding_updates = Vec::new();
            for row in embedding_rows {
                let id: String = row.try_get("id")?;
                embedding_updates.push(async move {
                    let res = sqlx::query("UPDATE embedding_cache SET synced_to_cloud = 1 WHERE id = $1")
                        .bind(&id)
                        .execute(pool)
                        .await;
                    res.is_ok()
                });
            }

            let mut mission_updates = Vec::new();
            for row in mission_rows {
                let id: String = row.try_get("id")?;
                mission_updates.push(async move {
                    let res = sqlx::query("UPDATE agent_missions SET synced_to_cloud = 1 WHERE id = $1")
                        .bind(&id)
                        .execute(pool)
                        .await;
                    res.is_ok()
                });
            }

            let embedding_results = futures::future::join_all(embedding_updates).await;
            for ok in embedding_results {
                if ok { synced_embeddings += 1; } else { failed_embeddings += 1; }
            }

            let mission_results = futures::future::join_all(mission_updates).await;
            for ok in mission_results {
                if ok { synced_missions += 1; } else { failed_missions += 1; }
            }
        } else {
            failed_embeddings += embedding_rows.len();
            failed_missions += mission_rows.len();
        }

        let total_synced = synced_embeddings + synced_missions;
        let total_failed = failed_embeddings + failed_missions;

        if total_synced > 0 && ::server_config::is_telemetry_enabled() {
            let _ = sqlx::query(
                r#"
                INSERT INTO telemetry_buffer (metric_name, metric_type, metric_value, labels)
                VALUES ($1, $2, $3, $4)
                "#,
            )
            .bind("ohc_autodream_sync_completed_total")
            .bind("counter")
            .bind(total_synced as f64)
            .bind("{}")
            .execute(pool)
            .await;
        }

        if total_failed > 0 && ::server_config::is_telemetry_enabled() {
            let _ = sqlx::query(
                r#"
                INSERT INTO telemetry_buffer (metric_name, metric_type, metric_value, labels)
                VALUES ($1, $2, $3, $4)
                "#,
            )
            .bind("ohc_autodream_sync_failed_total")
            .bind("counter")
            .bind(total_failed as f64)
            .bind("{}")
            .execute(pool)
            .await;
        }
    }

    Ok(())
}

pub fn start_autodream_sync_engine(db: Arc<DB>) {
    if !db.is_sqlite() {
        return;
    }
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            if let Err(e) = process_forecast_tick(db.clone()).await {
                ::server_telemetry::record_error_signal("[bug] AutoDream Sync Engine error");
                tracing::error!("AutoDream Sync Engine error: {}", e);
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DB;
    use sqlx::sqlite::SqlitePoolOptions;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_autodream_sync_process_forecast_tick() {
        unsafe { std::env::set_var("OHC_TEST_BYPASS_HTTP", "1"); }

        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .expect("Failed to connect to sqlite memory");

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS embedding_cache (
                id TEXT PRIMARY KEY,
                prompt TEXT NOT NULL,
                embedding BLOB,
                synced_to_cloud BOOLEAN DEFAULT 0,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS agent_missions (
                id TEXT PRIMARY KEY,
                status TEXT NOT NULL,
                payload TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                tenant_id TEXT NOT NULL DEFAULT '',
                cloud_mission_id TEXT,
                sync_error TEXT,
                last_synced_at TIMESTAMP,
                synced_to_cloud BOOLEAN DEFAULT 0,
                _sync_status TEXT DEFAULT 'pending',
                version INTEGER DEFAULT 1,
                mission_log TEXT
            );
            CREATE TABLE IF NOT EXISTS telemetry_buffer (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                metric_name TEXT NOT NULL,
                metric_type TEXT NOT NULL,
                metric_value REAL NOT NULL,
                labels TEXT NOT NULL,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            );
            "#,
        )
        .execute(&pool)
        .await
        .expect("Database URL or operation failed in test");

        // Insert unsynced embedding
        sqlx::query(
            r#"
            INSERT INTO embedding_cache (id, prompt, synced_to_cloud)
            VALUES ('emb1', 'test_prompt', 0);
            "#,
        )
        .execute(&pool)
        .await
        .expect("Database URL or operation failed in test");

        // Insert synced embedding
        sqlx::query(
            r#"
            INSERT INTO embedding_cache (id, prompt, synced_to_cloud)
            VALUES ('emb2', 'test_prompt2', 1);
            "#,
        )
        .execute(&pool)
        .await
        .expect("Database URL or operation failed in test");

        // Insert unsynced mission
        sqlx::query(
            r#"
            INSERT INTO agent_missions (id, status, payload, synced_to_cloud)
            VALUES ('miss1', 'pending', '{}', 0);
            "#,
        )
        .execute(&pool)
        .await
        .expect("Database URL or operation failed in test");

        let db = Arc::new(DB {
            pool: crate::db::get_pool(), // Fake PG pool
            store: crate::db::DbStore::Sqlite(pool.clone()),
        });

        // Run sync
        process_forecast_tick(db).await.expect("Database URL or operation failed in test");

        // Verify embeddings
        let unsynced_embeddings: i64 = sqlx::query_scalar("SELECT count(*) FROM embedding_cache WHERE synced_to_cloud = 0")
            .fetch_one(&pool)
            .await
            .expect("Database URL or operation failed in test");
        assert_eq!(unsynced_embeddings, 0);

        let synced_embeddings: i64 = sqlx::query_scalar("SELECT count(*) FROM embedding_cache WHERE synced_to_cloud = 1")
            .fetch_one(&pool)
            .await
            .expect("Database URL or operation failed in test");
        assert_eq!(synced_embeddings, 2);

        // Verify missions
        let unsynced_missions: i64 = sqlx::query_scalar("SELECT count(*) FROM agent_missions WHERE synced_to_cloud = 0")
            .fetch_one(&pool)
            .await
            .expect("Database URL or operation failed in test");
        assert_eq!(unsynced_missions, 0);

        let synced_missions: i64 = sqlx::query_scalar("SELECT count(*) FROM agent_missions WHERE synced_to_cloud = 1")
            .fetch_one(&pool)
            .await
            .expect("Database URL or operation failed in test");
        assert_eq!(synced_missions, 1);
    }
}
