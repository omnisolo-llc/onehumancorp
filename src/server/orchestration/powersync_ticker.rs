use std::sync::Arc;
use tokio::time::{self, Duration};
use sqlx::Row;

pub struct PowerSyncTicker {
    pool: sqlx::PgPool,
    client: reqwest::Client,
}

impl PowerSyncTicker {
    pub fn new(pool: sqlx::PgPool) -> Self {
        PowerSyncTicker {
            pool,
            client: reqwest::Client::new(),
        }
    }

    pub fn start(self: Arc<Self>, mut shutdown_rx: tokio::sync::broadcast::Receiver<()>, interval: Duration) {
        tokio::spawn(async move {
            let mut ticker = time::interval(interval);
            loop {
                tokio::select! {
                    _ = shutdown_rx.recv() => {
                        println!("PowerSyncTicker shutting down");
                        break;
                    }
                    _ = ticker.tick() => {
                        if let Err(e) = self.process_sync().await {
                            eprintln!("failed to process PowerSync: {}", e);
                        }
                    }
                }
            }
        });
    }

    async fn process_sync(&self) -> Result<(), String> {
        // Find CRDT deltas that are not yet synced to the cloud (limit 100 for safety)
        let rows = sqlx::query("SELECT tenant_id, id, entity_id, data, updated_at FROM crdt_deltas WHERE synced_to_cloud = FALSE LIMIT 100")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if rows.is_empty() {
            return Ok(());
        }

        let mut grouped_deltas: std::collections::HashMap<String, Vec<crate::ohc::orchestration::DeltaItem>> = std::collections::HashMap::new();

        for row in rows {
            let tenant_id: String = match row.try_get("tenant_id") {
                Ok(v) => v,
                Err(_) => continue,
            };
            let id: String = match row.try_get("id") {
                Ok(v) => v,
                Err(_) => continue,
            };
            let entity_id: String = match row.try_get("entity_id") {
                Ok(v) => v,
                Err(_) => continue,
            };
            let data: String = match row.try_get("data") {
                Ok(v) => v,
                Err(_) => continue,
            };
            let updated_at: String = match row.try_get("updated_at") {
                Ok(v) => v,
                Err(_) => {
                    match row.try_get::<chrono::DateTime<chrono::Utc>, _>("updated_at") {
                        Ok(dt) => dt.to_rfc3339(),
                        Err(_) => continue,
                    }
                }
            };

            grouped_deltas.entry(tenant_id).or_default().push(crate::ohc::orchestration::DeltaItem {
                id,
                entity_id,
                data,
                updated_at,
            });
        }

        for (tenant_id, deltas) in grouped_deltas {
            use prost::Message;

            let req_msg = crate::ohc::orchestration::SyncMcpDeltasRequest {
                tenant_id: tenant_id.clone(),
                deltas: deltas.clone(),
            };

            let mut buf = Vec::new();
            req_msg.encode(&mut buf).unwrap();

            let cloud_url = std::env::var("OHC_CLOUD_URL").unwrap_or_else(|_| "https://api.onehumancorp.com".to_string());
            let mut req = self.client.post(format!("{}/api/v1/sync/push", cloud_url))
                .header("Content-Type", "application/x-protobuf")
                .body(buf);

            if let Ok(spiffe_token) = std::env::var("SPIFFE_IDENTITY_TOKEN") {
                req = req.header("Authorization", format!("Bearer {}", spiffe_token));
            }

            let result = tokio::time::timeout(Duration::from_millis(5000), req.send()).await;

            match result {
                Ok(Ok(resp)) => {
                    if resp.status() == reqwest::StatusCode::OK {
                        for delta in deltas {
                            let id = &delta.id;
                            let updated_at = &delta.updated_at;

                            // memory rule: "always include updated_at = ? in the WHERE clause (Optimistic Concurrency Control)"
                            match sqlx::query("UPDATE crdt_deltas SET synced_to_cloud = TRUE WHERE id = $1 AND tenant_id = $2 AND updated_at = $3")
                                .bind(id)
                                .bind(&tenant_id)
                                .bind(updated_at)
                                .execute(&self.pool)
                                .await
                            {
                                Ok(_) => {}
                                Err(e) => eprintln!("failed to update crdt_deltas: {}", e)
                            }
                        }
                    } else {
                        eprintln!("PowerSync push failed with status: {}", resp.status());
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("failed to send PowerSync request: {}", e);
                }
                Err(_) => {
                    eprintln!("PowerSync request timed out");
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

    #[tokio::test]
    async fn test_power_sync_ticker_process() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let db_url = "postgres://postgres:postgres@localhost:5432/test";
        if let Ok(pool) = sqlx::PgPool::connect_lazy(db_url) {
            if matches!(tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::query("SELECT 1").execute(&pool)).await, Ok(Ok(_))) {
                let ticker = Arc::new(PowerSyncTicker::new(pool.clone()));

                let _ = sqlx::query("CREATE TABLE IF NOT EXISTS crdt_deltas (tenant_id TEXT NOT NULL, id TEXT NOT NULL, entity_id TEXT NOT NULL, data TEXT NOT NULL, updated_at TEXT NOT NULL, synced_to_cloud BOOLEAN DEFAULT FALSE, PRIMARY KEY (tenant_id, id))").execute(&pool).await;

                let _ = sqlx::query("INSERT INTO crdt_deltas (tenant_id, id, entity_id, data, updated_at, synced_to_cloud) VALUES ('t1', 'id1', 'e1', 'data1', '2023', FALSE) ON CONFLICT DO NOTHING").execute(&pool).await;

                let res = tokio::time::timeout(Duration::from_secs(5), ticker.process_sync()).await;
                assert!(res.is_ok());

                let (shutdown_tx, shutdown_rx) = tokio::sync::broadcast::channel(1);
                ticker.start(shutdown_rx, Duration::from_millis(10));
                tokio::time::sleep(Duration::from_millis(50)).await;
                shutdown_tx.send(()).unwrap();
            }
        }
    }
}
