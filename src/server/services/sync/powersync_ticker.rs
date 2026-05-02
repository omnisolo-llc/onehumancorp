use std::sync::Arc;
use sqlx::Row;
use tokio::time::{self, Duration};
use crate::db::{DB, DbStore};

pub struct PowerSyncTicker {
    db: Arc<DB>,
    cloud_url: String,
    client: reqwest::Client,
}

impl PowerSyncTicker {
    pub fn new(db: Arc<DB>, cloud_url: String) -> Self {
        Self {
            db,
            cloud_url,
            client: reqwest::Client::builder().timeout(Duration::from_secs(5)).build().unwrap_or_default(),
        }
    }

    pub fn start(self) {
        let ticker_self = Arc::new(self);
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                if let Err(e) = ticker_self.process_sync().await {
                    eprintln!("PowerSyncTicker error: {}", e);
                }
            }
        });
    }

    async fn process_sync(&self) -> Result<(), String> {
        let query_str = "SELECT tenant_id, id, entity_id, data, updated_at FROM crdt_deltas WHERE synced_to_cloud = FALSE LIMIT 500";

        let mut payload = Vec::new();

        match &self.db.store {
            DbStore::Sqlite(sqlite_pool) => {
                let rows = sqlx::query(query_str)
                    .fetch_all(sqlite_pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if rows.is_empty() {
                    return Ok(());
                }

                for row in &rows {
                    let tenant_id: String = row.get("tenant_id");
                    let id: String = row.get("id");
                    let entity_id: String = row.get("entity_id");
                    let data: String = row.get("data");
                    let updated_at: String = row.get("updated_at");

                    payload.push(serde_json::json!({
                        "tenant_id": tenant_id,
                        "id": id,
                        "entity_id": entity_id,
                        "data": data,
                        "updated_at": updated_at,
                    }));
                }
            }
            DbStore::Postgres => {
                // Not running in standalone?
                return Ok(());
            }
        }

        if payload.is_empty() {
            return Ok(());
        }

        let endpoint = format!("{}/api/v1/sync/push", self.cloud_url);

        let mut req = self.client.post(&endpoint).json(&serde_json::json!({
            "payload": serde_json::to_string(&payload).unwrap_or_default()
        }));

        if let Ok(spiffe_token) = std::env::var("SPIFFE_IDENTITY_TOKEN") {
            req = req.header("Authorization", format!("Bearer {}", spiffe_token));
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    match &self.db.store {
                        DbStore::Sqlite(sqlite_pool) => {
                            for item in payload {
                                let tenant_id = item["tenant_id"].as_str().unwrap_or_default();
                                let id = item["id"].as_str().unwrap_or_default();
                                let updated_at = item["updated_at"].as_str().unwrap_or_default();
                                let _ = sqlx::query("UPDATE crdt_deltas SET synced_to_cloud = TRUE WHERE tenant_id = ? AND id = ? AND updated_at = ?")
                                    .bind(tenant_id)
                                    .bind(id)
                                    .bind(updated_at)
                                    .execute(sqlite_pool)
                                    .await;
                            }
                        }
                        _ => {}
                    }
                } else {
                    eprintln!("PowerSyncTicker push failed with status: {}", resp.status());
                }
            }
            Err(e) => {
                eprintln!("PowerSyncTicker failed to send push request: {}", e);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use axum::{routing::post, Router, extract::Json};
    use serde_json::Value;

    #[tokio::test]
    async fn test_power_sync_ticker_process_sync() {
        // Mock SQLite database
        use std::str::FromStr;
        let conn_opts = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:")
            .unwrap()
            .create_if_missing(true);
        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect_with(conn_opts)
            .await
            .unwrap();

        sqlx::query("CREATE TABLE crdt_deltas (tenant_id TEXT NOT NULL, id TEXT NOT NULL, entity_id TEXT NOT NULL, data TEXT NOT NULL, updated_at TEXT NOT NULL, synced_to_cloud BOOLEAN DEFAULT FALSE, PRIMARY KEY (tenant_id, id));")
            .execute(&sqlite_pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO crdt_deltas (tenant_id, id, entity_id, data, updated_at) VALUES ('tenant_1', 'id_1', 'entity_1', 'some_data', '123')")
            .execute(&sqlite_pool)
            .await
            .unwrap();

        let dummy_pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap();
        let db = Arc::new(DB { pool: dummy_pool, store: DbStore::Sqlite(sqlite_pool.clone()) });

        // Mock axum server
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        let app = Router::new().route(
            "/api/v1/sync/push",
            post(move |Json(payload): Json<Value>| async move {
                let p = payload.get("payload").unwrap().as_str().unwrap();
                let arr: Vec<Value> = serde_json::from_str(p).unwrap();
                assert_eq!(arr.len(), 1);
                assert_eq!(arr[0].get("id").unwrap().as_str().unwrap(), "id_1");

                called_clone.store(true, Ordering::SeqCst);
                axum::http::StatusCode::OK
            }),
        );

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let cloud_url = format!("http://{}", addr);
        let ticker = PowerSyncTicker::new(db, cloud_url);

        let _ = tokio::time::timeout(tokio::time::Duration::from_millis(500), ticker.process_sync()).await.unwrap();

        assert!(called.load(Ordering::SeqCst), "API endpoint was not called");

        // Verify that it is marked as synced
        let row = sqlx::query("SELECT synced_to_cloud FROM crdt_deltas WHERE id = 'id_1'")
            .fetch_one(&sqlite_pool)
            .await
            .unwrap();

        let synced: bool = row.get("synced_to_cloud");
        assert!(synced, "Row should be marked as synced");
    }
}
