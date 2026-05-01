use std::sync::Arc;
use tokio::time::{self, Duration};
use sqlx::PgPool;
use reqwest::Client;

pub struct PowerSyncTicker {
    pool: PgPool,
    client: Client,
    cloud_url: String,
}

impl PowerSyncTicker {
    pub fn new(pool: PgPool, cloud_url: String) -> Self {
        PowerSyncTicker {
            pool,
            client: Client::new(),
            cloud_url,
        }
    }

    pub fn start(self: Arc<Self>, mut shutdown_rx: tokio::sync::broadcast::Receiver<()>, interval: Duration) {
        tokio::spawn(async move {
            let mut ticker = time::interval(interval);
            loop {
                tokio::select! {
                    _ = shutdown_rx.recv() => {
                        if let Ok(_) = shutdown_rx.recv().await { println!("PowerSyncTicker shutting down"); break; } else { break; }
                    }
                    _ = ticker.tick() => {
                        if let Err(e) = self.push_sync_data().await {
                            eprintln!("PowerSync: failed to push data to cloud: {}", e);
                        }
                    }
                }
            }
        });
    }

    async fn push_sync_data(&self) -> Result<(), String> {
        // Query modified rows
        let rows = sqlx::query("SELECT id, tenant_id, data FROM crdt_deltas WHERE synced_to_cloud = false LIMIT 50")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if rows.is_empty() {
            return Ok(());
        }

        use sqlx::Row;
        let mut payloads = Vec::new();
        let mut ids = Vec::new();
        for row in rows {
            let id: String = row.get("id");
            let data: String = row.get("data");
            payloads.push(data);
            ids.push(id);
        }

        let endpoint = format!("{}/api/v1/sync/push", self.cloud_url);
        let payload = serde_json::json!({
            "payloads": payloads,
        });

        let resp = self.client.post(&endpoint)
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if resp.status().is_success() {
            for id in ids {
                let _ = sqlx::query("UPDATE crdt_deltas SET synced_to_cloud = true WHERE id = $1")
                    .bind(id)
                    .execute(&self.pool)
                    .await;
            }
        } else {
            return Err(format!("Cloud returned status: {}", resp.status()));
        }

        Ok(())
    }
}
