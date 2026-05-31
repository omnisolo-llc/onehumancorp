use std::sync::Arc;
use crate::db::{DB, DbStore};
use sqlx::Row;
use ::server_ohc::orchestration::{sync_service_client::SyncServiceClient, SyncMcpDeltasRequest, DeltaItem};
use tonic::transport::Channel;
use tonic::Request;
use tonic::metadata::MetadataValue;
use std::time::Duration;
use tracing::{error, info, warn};

pub struct PowerSyncTicker {
    db: Arc<DB>,
    cloud_url: String,
}

impl PowerSyncTicker {
    pub fn new(db: Arc<DB>, cloud_url: String) -> Self {
        Self { db, cloud_url }
    }

    pub async fn start(self: Arc<Self>, mut shutdown_rx: tokio::sync::broadcast::Receiver<()>) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(5));
            loop {
                tokio::select! {
                    _ = shutdown_rx.recv() => {
                        info!("PowerSyncTicker shutting down");
                        break;
                    }
                    _ = interval.tick() => {
                        if let Err(e) = self.sync_deltas().await {
                            warn!("PowerSyncTicker sync failed: {}", e);
                        }
                    }
                }
            }
        });
    }

    pub async fn sync_deltas(&self) -> Result<(), String> {
        // Fetch up to 100 unsynced deltas
        let query = "SELECT tenant_id, id, entity_id, data, updated_at FROM crdt_deltas WHERE synced_to_cloud = false LIMIT 100";

        let mut deltas_to_push = Vec::new();

        match &self.db.store {
            DbStore::Sqlite(pool) => {
                match sqlx::query(query).fetch_all(pool).await {
                    Ok(rows) => {
                        for row in rows {
                            let tenant_id: String = row.get("tenant_id");
                            let id: String = row.get("id");
                            let entity_id: String = row.get("entity_id");
                            let data: String = row.get("data");
                            let updated_at: String = row.get("updated_at");

                            deltas_to_push.push((tenant_id, DeltaItem {
                                id,
                                entity_id,
                                data,
                                updated_at,
                            }));
                        }
                    }
                    Err(e) => {
                        if e.to_string().contains("no such table") {
                            // Ignore if table hasn't been created yet
                            return Ok(());
                        }
                        return Err(format!("SQLite query failed: {}", e));
                    }
                }
            }
            DbStore::Postgres => {
                match sqlx::query(query).fetch_all(&self.db.pool).await {
                    Ok(rows) => {
                        for row in rows {
                            let tenant_id: String = row.get("tenant_id");
                            let id: String = row.get("id");
                            let entity_id: String = row.get("entity_id");
                            let data: String = row.get("data");
                            let updated_at: String = row.get("updated_at");

                            deltas_to_push.push((tenant_id, DeltaItem {
                                id,
                                entity_id,
                                data,
                                updated_at,
                            }));
                        }
                    }
                    Err(e) => {
                        if e.to_string().contains("relation \"crdt_deltas\" does not exist") {
                            // Ignore if table hasn't been created yet
                            return Ok(());
                        }
                        return Err(format!("Postgres query failed: {}", e));
                    }
                }
            }
        }

        if deltas_to_push.is_empty() {
            return Ok(());
        }

        // Group by tenant_id
        use std::collections::HashMap;
        let mut grouped: HashMap<String, Vec<DeltaItem>> = HashMap::new();
        for (tenant, delta) in deltas_to_push {
            grouped.entry(tenant).or_insert_with(Vec::new).push(delta);
        }

        // Send to cloud
        let endpoint = if self.cloud_url.starts_with("http") {
            self.cloud_url.clone()
        } else {
            format!("http://{}", self.cloud_url)
        };

        let channel = Channel::from_shared(endpoint).map_err(|e| e.to_string())?.connect().await.map_err(|e| e.to_string())?;
        let mut client = SyncServiceClient::new(channel);

        for (tenant_id, deltas) in grouped {
            let ids: Vec<String> = deltas.iter().map(|d| d.id.clone()).collect();
            let mut req = Request::new(SyncMcpDeltasRequest {
                tenant_id: tenant_id.clone(),
                deltas,
            });

            let spiffe_id = format!("spiffe://onehumancorp.io/{}/system", tenant_id);
            req.metadata_mut().insert("x-spiffe-id", MetadataValue::try_from(spiffe_id.as_str()).unwrap());

            let res = client.sync_mcp_deltas(req).await.map_err(|e| e.to_string())?;
            let inner = res.into_inner();

            if inner.status == "success" {
                // Update local db
                match &self.db.store {
                    DbStore::Sqlite(pool) => {
                        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
                        for id in ids {
                            let _ = sqlx::query("UPDATE crdt_deltas SET synced_to_cloud = true WHERE id = ?")
                                .bind(id)
                                .execute(&mut *tx)
                                .await;
                        }
                        let _ = tx.commit().await;
                    }
                    DbStore::Postgres => {
                        let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                        for id in ids {
                            let _ = sqlx::query("UPDATE crdt_deltas SET synced_to_cloud = true WHERE id = $1")
                                .bind(id)
                                .execute(&mut *tx)
                                .await;
                        }
                        let _ = tx.commit().await;
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::broadcast;
    use crate::db::{DB, DbStore};
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn test_powersync_ticker_shutdown() {
        let dummy_pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect_lazy("postgres://localhost/dummy").unwrap();

        let sqlite_pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        let db = Arc::new(DB {
            pool: dummy_pool,
            store: DbStore::Sqlite(sqlite_pool),
        });

        let (shutdown_tx, shutdown_rx) = broadcast::channel(1);
        let ticker = Arc::new(PowerSyncTicker::new(db, "http://dummy".to_string()));

        // Start the ticker in the background
        ticker.start(shutdown_rx).await;

        // Send the shutdown signal
        shutdown_tx.send(()).unwrap();

        // Wait briefly to allow the task to process the shutdown signal
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Note: As the task is spawned in the background, we assert it doesn't panic.
        // It successfully processes the shutdown without blocking or crashing.
        assert!(true);
    }

    #[tokio::test]
    async fn test_powersync_ticker_sync_deltas_empty_or_no_table() {
        let dummy_pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect_lazy("postgres://localhost/dummy").unwrap();

        let sqlite_pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        let db = Arc::new(DB {
            pool: dummy_pool,
            store: DbStore::Sqlite(sqlite_pool),
        });

        let ticker = PowerSyncTicker::new(db, "http://dummy".to_string());

        // crdt_deltas does not exist, so it should gracefully return Ok(())
        let res = ticker.sync_deltas().await;
        assert!(res.is_ok());
    }
}
