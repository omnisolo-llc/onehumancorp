use std::sync::Arc;
use crate::db::DB;
use sqlx::Row;
use serde_json::json;
use ::server_ohc::orchestration::{sync_service_client::SyncServiceClient, PowerSyncPushRequest, PowerSyncPullRequest};
use tonic::transport::Channel;
use tonic::Request;
use tonic::metadata::MetadataValue;

pub struct PowerSyncOrchestrator {
    db: Arc<DB>,
    cloud_url: String,
}

impl PowerSyncOrchestrator {
    pub fn new(db: Arc<DB>, cloud_url: String) -> Self {
        Self { db, cloud_url }
    }

    pub async fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
            loop {
                interval.tick().await;
                if let Err(e) = self.push_sync().await {
                    tracing::error!("PowerSync push failed: {}", e);
                }
                if let Err(e) = self.pull_sync().await {
                    tracing::error!("PowerSync pull failed: {}", e);
                }
            }
        });
    }

    pub async fn push_sync(&self) -> Result<(), String> {
        let sqlite_pool = match &self.db.store {
            crate::db::DbStore::Sqlite(pool) => pool,
            _ => return Ok(()), // Only runs in Standalone mode with SQLite
        };

        // For simplicity, we just sync agent_missions for now.
        // We look for rows that have been updated locally and need syncing.
        let rows = sqlx::query(
            "SELECT id, status, payload, created_at, updated_at, organization_id, _sync_status, version
             FROM agent_missions
             WHERE _sync_status = 'pending'"
        )
        .fetch_all(sqlite_pool)
        .await
        .map_err(|e| e.to_string())?;

        if rows.is_empty() {
            return Ok(());
        }

        let mut payload_items = Vec::new();
        for row in rows {
            let id: String = row.get("id");
            let status: String = row.get("status");
            let payload: String = row.get("payload");
            let org_id: String = row.get("organization_id");
            let updated_at: String = row.try_get("updated_at").unwrap_or_else(|_| "".to_string());
            let version: i64 = row.try_get("version").unwrap_or(1);

            payload_items.push(json!({
                "table": "agent_missions",
                "id": id,
                "status": status,
                "payload": payload,
                "organization_id": org_id,
                "updated_at": updated_at,
                "version": version
            }));
        }

        let payload_str = serde_json::to_string(&payload_items).map_err(|e| e.to_string())?;

        // Connect to gRPC client
        let endpoint = if self.cloud_url.starts_with("http") {
            self.cloud_url.clone()
        } else {
            format!("http://{}", self.cloud_url)
        };

        let channel = Channel::from_shared(endpoint).map_err(|e| e.to_string())?.connect().await.map_err(|e| e.to_string())?;

        let mut client = SyncServiceClient::new(channel);

        let mut req = Request::new(PowerSyncPushRequest {
            payload: payload_str,
        });

        // Add internal auth using spiffe identity
        let spiffe_id = format!("spiffe://onehumancorp.io/{}/system", "system");
        req.metadata_mut().insert("x-spiffe-id", MetadataValue::try_from(spiffe_id.as_str()).unwrap());

        let res = client.power_sync_push(req).await.map_err(|e| e.to_string())?;

        if res.into_inner().status == "ok" {
            // Update _sync_status to synced
            for item in payload_items {
                let id = item["id"].as_str().unwrap();
                let _ = sqlx::query("UPDATE agent_missions SET _sync_status = 'synced' WHERE id = ?")
                    .bind(id)
                    .execute(sqlite_pool)
                    .await;
            }
        }

        Ok(())
    }

    pub async fn pull_sync(&self) -> Result<(), String> {
        let sqlite_pool = match &self.db.store {
            crate::db::DbStore::Sqlite(pool) => pool,
            _ => return Ok(()), // Only runs in Standalone mode with SQLite
        };

        // Connect to gRPC client
        let endpoint = if self.cloud_url.starts_with("http") {
            self.cloud_url.clone()
        } else {
            format!("http://{}", self.cloud_url)
        };

        let channel = Channel::from_shared(endpoint).map_err(|e| e.to_string())?.connect().await.map_err(|e| e.to_string())?;

        let mut client = SyncServiceClient::new(channel);

        let mut req = Request::new(PowerSyncPullRequest {});

        // Add internal auth using spiffe identity
        let spiffe_id = format!("spiffe://onehumancorp.io/{}/system", "system");
        req.metadata_mut().insert("x-spiffe-id", MetadataValue::try_from(spiffe_id.as_str()).unwrap());

        let res = client.power_sync_pull(req).await.map_err(|e| e.to_string())?;

        let payload = res.into_inner().payload;
        if payload.is_empty() || payload == "[]" {
            return Ok(());
        }

        let items: Vec<serde_json::Value> = serde_json::from_str(&payload).map_err(|e| e.to_string())?;

        for item in items {
            if item["table"].as_str() == Some("agent_missions") {
                let id = item["id"].as_str().unwrap_or("");
                let status = item["status"].as_str().unwrap_or("PENDING");
                let payload_data = item["payload"].as_str().unwrap_or("");
                let org_id = item["organization_id"].as_str().unwrap_or("system");
                let updated_at_str = item["updated_at"].as_str().unwrap_or("");
                let version = item["version"].as_i64().unwrap_or(1);

                if id.is_empty() {
                    continue;
                }

                let query = "
                    INSERT INTO agent_missions (id, status, payload, organization_id, updated_at, _sync_status, version)
                    VALUES (?, ?, ?, ?, ?, 'synced', ?)
                    ON CONFLICT(id) DO UPDATE SET
                        status = excluded.status,
                        payload = excluded.payload,
                        organization_id = excluded.organization_id,
                        updated_at = excluded.updated_at,
                        _sync_status = 'synced',
                        version = excluded.version
                    WHERE agent_missions.updated_at < excluded.updated_at
                ";

                if let Err(e) = sqlx::query(query)
                    .bind(id)
                    .bind(status)
                    .bind(payload_data)
                    .bind(org_id)
                    .bind(updated_at_str)
                    .bind(version)
                    .execute(sqlite_pool)
                    .await
                {
                    tracing::error!("PowerSync pull failed to save to database: error={}", e);
                }
            }
        }

        Ok(())
    }
}
