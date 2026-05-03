use std::sync::Arc;
use tokio::time::{self, Duration};
use sqlx::Row;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CrdtDelta {
    pub id: String,
    pub tenant_id: String,
    pub entity_id: String,
    pub data: String,
    pub updated_at: String,
}

pub struct PowerSyncTicker {
    pool: sqlx::sqlite::SqlitePool,
    client: reqwest::Client,
    cloud_url: String,
}

impl PowerSyncTicker {
    pub fn new(pool: sqlx::sqlite::SqlitePool, cloud_url: String) -> Self {
        PowerSyncTicker {
            pool,
            client: reqwest::Client::new(),
            cloud_url,
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
                            eprintln!("failed to process powersync: {}", e);
                        }
                    }
                }
            }
        });
    }

    async fn process_sync(&self) -> Result<(), String> {
        // Find deltas that need to be synced
        let rows = sqlx::query("SELECT id, tenant_id, entity_id, data, updated_at FROM crdt_deltas WHERE synced_to_cloud = 0 OR synced_to_cloud = FALSE")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if rows.is_empty() {
            return Ok(());
        }

        let mut deltas = Vec::new();
        for row in rows {
            let delta = CrdtDelta {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                entity_id: row.get("entity_id"),
                data: row.get("data"),
                updated_at: row.get("updated_at"),
            };
            deltas.push(delta);
        }

        let payload = serde_json::json!({ "deltas": deltas });

        let mut req = self.client.post(format!("{}/api/v1/sync/push", self.cloud_url))
            .header("Content-Type", "application/json")
            .json(&payload);

        if let Ok(spiffe_token) = std::env::var("SPIFFE_IDENTITY_TOKEN") {
            req = req.header("Authorization", format!("Bearer {}", spiffe_token));
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    // Update successfully synced items, using OCC on updated_at
                    let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
                    for delta in deltas {
                        sqlx::query("UPDATE crdt_deltas SET synced_to_cloud = 1 WHERE id = ? AND tenant_id = ? AND updated_at <= ?")
                            .bind(&delta.id)
                            .bind(&delta.tenant_id)
                            .bind(&delta.updated_at)
                            .execute(&mut *tx)
                            .await
                            .map_err(|e| e.to_string())?;
                    }
                    tx.commit().await.map_err(|e| e.to_string())?;
                } else {
                    eprintln!("powersync push failed with status: {}", resp.status());
                }
            }
            Err(e) => {
                eprintln!("failed to send powersync request: {}", e);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{routing::post, Router};
    use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};
    use std::str::FromStr;
    use axum::extract::Json;

    #[derive(serde::Deserialize)]
    struct SyncRequest {
        deltas: Vec<CrdtDelta>,
    }

    async fn handle_sync(Json(payload): Json<SyncRequest>) -> axum::http::StatusCode {
        if !payload.deltas.is_empty() {
            axum::http::StatusCode::OK
        } else {
            axum::http::StatusCode::BAD_REQUEST
        }
    }

    #[tokio::test]
    async fn test_power_sync_ticker() {
        // Setup mock API server
        let mock_server = Router::new()
            .route("/api/v1/sync/push", post(handle_sync));

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let cloud_url = format!("http://127.0.0.1:{}", port);

        tokio::spawn(async move {
            axum::serve(listener, mock_server).await.unwrap();
        });

        // Setup test database
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:")
            .unwrap()
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .connect_with(conn_opts)
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE crdt_deltas (
                id TEXT,
                tenant_id TEXT,
                entity_id TEXT,
                data TEXT,
                updated_at TEXT,
                synced_to_cloud BOOLEAN,
                PRIMARY KEY (id, tenant_id)
            )"
        )
        .execute(&pool)
        .await
        .unwrap();

        // Insert pending delta
        sqlx::query(
            "INSERT INTO crdt_deltas (id, tenant_id, entity_id, data, updated_at, synced_to_cloud)
             VALUES ('d1', 't1', 'e1', 'some_data', '2023-01-01T00:00:00Z', 0)"
        )
        .execute(&pool)
        .await
        .unwrap();

        let ticker = Arc::new(PowerSyncTicker::new(pool.clone(), cloud_url));
        let (shutdown_tx, shutdown_rx) = tokio::sync::broadcast::channel(1);

        ticker.start(shutdown_rx, Duration::from_millis(10));

        // Wait for sync to process
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Verify it was updated
        let row = sqlx::query("SELECT synced_to_cloud FROM crdt_deltas WHERE id = 'd1'")
            .fetch_one(&pool)
            .await
            .unwrap();

        let synced: bool = row.get("synced_to_cloud");
        assert!(synced, "Delta should be marked as synced");

        shutdown_tx.send(()).unwrap();
    }
}
