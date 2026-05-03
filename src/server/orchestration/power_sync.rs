use std::sync::Arc;
use crate::db::DB;
use sqlx::Row;
use serde_json::json;

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
                    eprintln!("PowerSync push failed: {}", e);
                }
                if let Err(e) = self.pull_sync().await {
                    eprintln!("PowerSync pull failed: {}", e);
                }
            }
        });
    }

    pub async fn push_sync(&self) -> Result<(), String> {
        let sqlite_pool = match &self.db.store {
            crate::db::DbStore::Sqlite(pool) => pool,
            _ => return Ok(()), // Only runs in Standalone mode with SQLite
        };

        let mut payload_items = Vec::new();

        let tables = vec!["agent_missions", "shared_tasks", "swarm_tasks", "agent_memories"];
        for table in &tables {
            let query = format!(
                "SELECT * FROM {} WHERE _sync_status = 'pending'",
                table
            );

            if let Ok(rows) = sqlx::query(&query).fetch_all(sqlite_pool).await {
                for row in rows {
                    let id: String = row.try_get("id").unwrap_or_default();
                    if id.is_empty() { continue; }

                    let status: String = row.try_get("status").unwrap_or_default();

                    let payload: String = if *table == "agent_memories" {
                        row.try_get("raw_content").unwrap_or_default()
                    } else {
                        row.try_get("payload").unwrap_or_default()
                    };

                    let org_id: String = row.try_get("organization_id").unwrap_or_else(|_| "system".to_string());
                    let updated_at: chrono::DateTime<chrono::Utc> = row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now());
                    let version: i64 = row.try_get("version").unwrap_or(1);

                    payload_items.push(json!({
                        "table": *table,
                        "id": id,
                        "status": status,
                        "payload": payload,
                        "organization_id": org_id,
                        "updated_at": updated_at.to_rfc3339(),
                        "version": version
                    }));
                }
            }
        }

        if payload_items.is_empty() {
            return Ok(());
        }

        let endpoint = if self.cloud_url.starts_with("http") {
            format!("{}/api/v1/sync/push", self.cloud_url)
        } else {
            format!("http://{}/api/v1/sync/push", self.cloud_url)
        };

        let spiffe_id = format!("spiffe://onehumancorp.io/{}/system", "system");

        let client = reqwest::Client::new();
        let res = client.post(&endpoint)
            .header("x-spiffe-id", spiffe_id)
            .json(&json!({ "rows": payload_items }))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            for item in payload_items {
                let table = item["table"].as_str().unwrap_or("agent_missions");
                let id = item["id"].as_str().unwrap_or("");
                if id.is_empty() { continue; }
                let query = format!("UPDATE {} SET _sync_status = 'synced' WHERE id = ?", table);
                let _ = sqlx::query(&query)
                    .bind(id)
                    .execute(sqlite_pool)
                    .await;
            }
        } else {
            return Err(format!("Push failed with status: {}", res.status()));
        }

        Ok(())
    }

    pub async fn pull_sync(&self) -> Result<(), String> {
        let sqlite_pool = match &self.db.store {
            crate::db::DbStore::Sqlite(pool) => pool,
            _ => return Ok(()),
        };

        // For simplicity we just use agent_missions last sync time as a proxy for all tables,
        // though in production we might track it per-table or overall.
        let last_sync: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar(
            "SELECT MAX(updated_at) FROM agent_missions WHERE _sync_status = 'synced'"
        )
        .fetch_optional(sqlite_pool)
        .await
        .unwrap_or(None);

        let mut url = if self.cloud_url.starts_with("http") {
            format!("{}/api/v1/sync/pull", self.cloud_url)
        } else {
            format!("http://{}/api/v1/sync/pull", self.cloud_url)
        };

        if let Some(time) = last_sync {
            url.push_str(&format!("?after={}", time.to_rfc3339()));
        }

        let spiffe_id = format!("spiffe://onehumancorp.io/{}/system", "system");

        let client = reqwest::Client::new();
        let res = client.get(&url)
            .header("x-spiffe-id", spiffe_id)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            let body: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
            if let Some(rows) = body["rows"].as_array() {
                for row in rows {
                    let table = row["table"].as_str().unwrap_or("agent_missions");
                    let id = row["id"].as_str().unwrap_or("");
                    let status = row["status"].as_str().unwrap_or("");
                    let payload = row["payload"].as_str().unwrap_or("");
                    let org_id = row["organization_id"].as_str().unwrap_or("system");
                    let updated_at_str = row["updated_at"].as_str().unwrap_or("");
                    let version = row["version"].as_i64().unwrap_or(1);

                    if id.is_empty() {
                        continue;
                    }

                    let updated_at = if updated_at_str.is_empty() {
                        chrono::Utc::now()
                    } else {
                        chrono::DateTime::parse_from_rfc3339(updated_at_str)
                            .map(|d| d.with_timezone(&chrono::Utc))
                            .unwrap_or_else(|_| chrono::Utc::now())
                    };

                    let query = if table == "agent_memories" {
                        format!("INSERT INTO {} (id, task_id, raw_content, organization_id, updated_at, version, _sync_status)
                                 VALUES (?, '', ?, ?, ?, ?, 'synced')
                                 ON CONFLICT(id) DO UPDATE SET
                                 raw_content = excluded.raw_content, updated_at = excluded.updated_at, version = excluded.version, _sync_status = 'synced'
                                 WHERE {}.updated_at < excluded.updated_at OR {}.version < excluded.version", table, table, table)
                    } else if table == "shared_tasks" || table == "swarm_tasks" {
                        format!("INSERT INTO {} (id, title, status, payload, organization_id, updated_at, version, _sync_status)
                                 VALUES (?, 'Sync Task', ?, ?, ?, ?, ?, 'synced')
                                 ON CONFLICT(id) DO UPDATE SET
                                 status = excluded.status, payload = excluded.payload, updated_at = excluded.updated_at, version = excluded.version, _sync_status = 'synced'
                                 WHERE {}.updated_at < excluded.updated_at OR {}.version < excluded.version", table, table, table)
                    } else {
                        format!("INSERT INTO {} (id, status, payload, organization_id, updated_at, version, _sync_status)
                                 VALUES (?, ?, ?, ?, ?, ?, 'synced')
                                 ON CONFLICT(id) DO UPDATE SET
                                 status = excluded.status, payload = excluded.payload, updated_at = excluded.updated_at, version = excluded.version, _sync_status = 'synced'
                                 WHERE {}.updated_at < excluded.updated_at OR {}.version < excluded.version", table, table, table)
                    };

                    if table == "agent_memories" {
                        let _ = sqlx::query(&query)
                            .bind(id)
                            .bind(payload)
                            .bind(org_id)
                            .bind(updated_at)
                            .bind(version as i64)
                            .execute(sqlite_pool)
                            .await;
                    } else {
                        let _ = sqlx::query(&query)
                            .bind(id)
                            .bind(status)
                            .bind(payload)
                            .bind(org_id)
                            .bind(updated_at)
                            .bind(version as i64)
                            .execute(sqlite_pool)
                            .await;
                    }
                }
            }
        } else {
            return Err(format!("Pull failed with status: {}", res.status()));
        }

        Ok(())
    }
}
