use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::Row;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityPlugin {
    pub plugin_id: String,
    pub name: String,
    pub version: String,
    pub manifest_url: String,
    pub status: String,
    pub registered_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicMemory {
    pub memory_id: String,
    pub context: String,
    pub vector_embedding: Option<Vec<u8>>,
    pub source_plugin: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
struct MessageModel {
    id: String,
    from_agent: String,
    to_agent: String,
    r#type: String,
    content: String,
    meeting_id: String,
    occurred_at_unix: i64,
}

pub struct SipDB {
    pool: PgPool,
    org_id: String,
    local_cache: RwLock<HashMap<String, String>>,
    cache_expirations: RwLock<HashMap<String, Instant>>,
    pub context_root: Option<String>,
}

impl SipDB {
    pub fn new(pool: PgPool, org_id: String) -> Self {
        SipDB {
            pool,
            org_id,
            local_cache: RwLock::new(HashMap::new()),
            cache_expirations: RwLock::new(HashMap::new()),
            context_root: std::env::current_dir().ok().map(|p| p.to_string_lossy().to_string()),
        }
    }

    fn get_cache(&self, key: &str) -> Option<String> {
        let expirations = self.cache_expirations.read().unwrap();
        if let Some(exp) = expirations.get(key) {
            if exp.elapsed() > Duration::from_secs(3600) {
                drop(expirations);
                let mut expirations = self.cache_expirations.write().unwrap();
                let mut cache = self.local_cache.write().unwrap();
                expirations.remove(key);
                cache.remove(key);
                return None;
            }
        }
        
        let cache = self.local_cache.read().unwrap();
        cache.get(key).cloned()
    }

    fn set_cache(&self, key: String, value: String) {
        let mut cache = self.local_cache.write().unwrap();
        let mut expirations = self.cache_expirations.write().unwrap();
        cache.insert(key.clone(), value);
        expirations.insert(key, Instant::now());
    }

    fn invalidate_cache(&self, key: &str) {
        let mut cache = self.local_cache.write().unwrap();
        let mut expirations = self.cache_expirations.write().unwrap();
        cache.remove(key);
        expirations.remove(key);
    }

    pub async fn sync_memory(&self, key: &str) -> Result<Option<String>, sqlx::Error> {
        let cache_key = format!("sip:memory:{}:{}", self.org_id, key);
        if let Some(val) = self.get_cache(&cache_key) {
            return Ok(Some(val));
        }

        let row = sqlx::query("SELECT value FROM swarm_memory WHERE key = $1 AND organization_id = $2")
            .bind(key)
            .bind(&self.org_id)
            .fetch_optional(&self.pool)
            .await?;
            
        let value: Option<String> = row.map(|r| r.get("value"));
        if let Some(ref val) = value {
            self.set_cache(cache_key, val.clone());
        }
        
        Ok(value)
    }

    pub async fn update_memory(&self, key: &str, value: &str) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO swarm_memory (key, value, updated_at, organization_id) VALUES ($1, $2, CURRENT_TIMESTAMP, $3) ON CONFLICT(key) DO UPDATE SET value=excluded.value, updated_at=CURRENT_TIMESTAMP")
            .bind(key)
            .bind(value)
            .bind(&self.org_id)
            .execute(&self.pool)
            .await?;
            
        self.invalidate_cache(&format!("sip:memory:{}:{}", self.org_id, key));
            
        Ok(())
    }

    pub async fn get_pending_missions(&self, role: &str) -> Result<Vec<crate::ohc::orchestration::Message>, sqlx::Error> {
        let query = if role == "ANY" {
            "SELECT id, payload FROM agent_missions WHERE status = 'PENDING' AND organization_id = $1 ORDER BY created_at DESC LIMIT 500"
        } else {
            "SELECT id, payload FROM agent_missions WHERE payload::json->>'role' = $1 AND status = 'PENDING' AND organization_id = $2 ORDER BY created_at DESC LIMIT 500"
        };
        
        let rows = if role == "ANY" {
            sqlx::query(query)
                .bind(&self.org_id)
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query(query)
                .bind(role)
                .bind(&self.org_id)
                .fetch_all(&self.pool)
                .await?
        };
        
        let mut missions = Vec::new();
        for row in rows {
            let id: String = row.get("id");
            let payload: String = row.get("payload");
            
            let mut msg = crate::ohc::orchestration::Message {
                id: id.clone(),
                from_agent: String::new(),
                to_agent: String::new(),
                r#type: "task".to_string(),
                content: payload.clone(),
                meeting_id: String::new(),
                occurred_at_unix: 0,
            };
            
            if let Ok(parsed) = serde_json::from_str::<MessageModel>(&payload) {
                msg = crate::ohc::orchestration::Message {
                    id: parsed.id,
                    from_agent: parsed.from_agent,
                    to_agent: parsed.to_agent,
                    r#type: parsed.r#type,
                    content: parsed.content,
                    meeting_id: parsed.meeting_id,
                    occurred_at_unix: parsed.occurred_at_unix,
                };
            }
            
            if msg.id.is_empty() {
                msg.id = id;
            }
            
            missions.push(msg);
        }
        
        Ok(missions)
    }

    pub async fn complete_mission(&self, mission_id: &str) -> Result<(), sqlx::Error> {
        let result = sqlx::query("UPDATE agent_missions SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND organization_id = $2")
            .bind(mission_id)
            .bind(&self.org_id)
            .execute(&self.pool)
            .await?;
            
        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        
        Ok(())
    }

    pub async fn heartbeat(&self, agent_id: &str, role: &str, status: &str) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO agent_status (agent_id, role, status, last_heartbeat, organization_id) VALUES ($1, $2, $3, CURRENT_TIMESTAMP, $4) ON CONFLICT(agent_id) DO UPDATE SET role=excluded.role, status=excluded.status, last_heartbeat=CURRENT_TIMESTAMP")
            .bind(agent_id)
            .bind(role)
            .bind(status)
            .bind(&self.org_id)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }

    pub async fn register_capability_plugin(&self, plugin: CapabilityPlugin) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO capability_plugins (plugin_id, name, version, manifest_url, status, registered_at, organization_id)
             VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, $6)
             ON CONFLICT(plugin_id) DO UPDATE SET
             name=excluded.name, version=excluded.version,
             manifest_url=excluded.manifest_url, status=excluded.status,
             registered_at=CURRENT_TIMESTAMP"
        )
        .bind(plugin.plugin_id)
        .bind(plugin.name)
        .bind(plugin.version)
        .bind(plugin.manifest_url)
        .bind(plugin.status)
        .bind(&self.org_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn get_capability_plugins(&self, status: &str) -> Result<Vec<CapabilityPlugin>, sqlx::Error> {
        let query = if status.is_empty() {
            "SELECT plugin_id, name, version, manifest_url, status, registered_at FROM capability_plugins WHERE organization_id = $1"
        } else {
            "SELECT plugin_id, name, version, manifest_url, status, registered_at FROM capability_plugins WHERE status = $1 AND organization_id = $2"
        };
        
        let rows = if status.is_empty() {
            sqlx::query(query)
                .bind(&self.org_id)
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query(query)
                .bind(status)
                .bind(&self.org_id)
                .fetch_all(&self.pool)
                .await?
        };
        
        let mut plugins = Vec::new();
        for row in rows {
            plugins.push(CapabilityPlugin {
                plugin_id: row.get("plugin_id"),
                name: row.get("name"),
                version: row.get("version"),
                manifest_url: row.get("manifest_url"),
                status: row.get("status"),
                registered_at: row.get("registered_at"),
            });
        }
        
        Ok(plugins)
    }

    pub async fn store_episodic_memory(&self, memory: EpisodicMemory) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, source_plugin, created_at, organization_id)
             VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP, $5)
             ON CONFLICT(memory_id) DO UPDATE SET
             context=excluded.context, vector_embedding=excluded.vector_embedding,
             source_plugin=excluded.source_plugin"
        )
        .bind(memory.memory_id)
        .bind(memory.context)
        .bind(memory.vector_embedding)
        .bind(memory.source_plugin)
        .bind(&self.org_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn get_episodic_memories_by_plugin(&self, plugin: &str) -> Result<Vec<EpisodicMemory>, sqlx::Error> {
        let query = if plugin.is_empty() {
            "SELECT memory_id, context, vector_embedding, source_plugin, created_at FROM swarm_memory_embeddings WHERE organization_id = $1"
        } else {
            "SELECT memory_id, context, vector_embedding, source_plugin, created_at FROM swarm_memory_embeddings WHERE source_plugin = $1 AND organization_id = $2"
        };
        
        let rows = if plugin.is_empty() {
            sqlx::query(query)
                .bind(&self.org_id)
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query(query)
                .bind(plugin)
                .bind(&self.org_id)
                .fetch_all(&self.pool)
                .await?
        };
        
        let mut memories = Vec::new();
        for row in rows {
            memories.push(EpisodicMemory {
                memory_id: row.get("memory_id"),
                context: row.get("context"),
                vector_embedding: row.get("vector_embedding"),
                source_plugin: row.get("source_plugin"),
                created_at: row.get("created_at"),
            });
        }
        
        Ok(memories)
    }

    pub async fn prune_buffered_metrics(&self, age_threshold: chrono::Duration) -> Result<(), sqlx::Error> {
        let threshold_time = Utc::now() - age_threshold;
        
        sqlx::query("WITH cte AS (SELECT id FROM telemetry_buffer WHERE created_at < $1 AND organization_id = $2 LIMIT 1000) DELETE FROM telemetry_buffer WHERE id IN (SELECT id FROM cte)")
            .bind(threshold_time)
            .bind(&self.org_id)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }

    pub async fn prune_stale_missions(&self, age_threshold: chrono::Duration) -> Result<(), sqlx::Error> {
        let stuck_threshold = Utc::now() - chrono::Duration::hours(1);
        let fail_threshold = Utc::now() - age_threshold;
        
        // 0. Archive stale IN_PROGRESS and BLOCKED missions to .agent-task/archive/
        let archive_rows = sqlx::query("SELECT id, status, payload FROM agent_missions WHERE (status = 'IN_PROGRESS' OR status = 'BLOCKED') AND created_at < $1 AND organization_id = $2 LIMIT 100")
            .bind(stuck_threshold)
            .bind(&self.org_id)
            .fetch_all(&self.pool)
            .await?;

        if !archive_rows.is_empty() {
            let _ = tokio::fs::create_dir_all(".agent-task/archive").await;

            for row in archive_rows {
                let id: String = row.get("id");
                let status: String = row.get("status");
                let payload: String = row.get("payload");

                let file_path = format!(".agent-task/archive/{}.json", id);
                let archive_data = serde_json::json!({
                    "id": id,
                    "status": status,
                    "payload": payload,
                    "archived_at": Utc::now().to_rfc3339()
                });

                if let Ok(json_str) = serde_json::to_string_pretty(&archive_data) {
                    let _ = tokio::fs::write(&file_path, json_str).await;
                }

                sqlx::query("DELETE FROM agent_missions WHERE id = $1 AND organization_id = $2")
                    .bind(&id)
                    .bind(&self.org_id)
                    .execute(&self.pool)
                    .await?;
            }
        }

        // 1. Mark stagnant PENDING missions as STUCK after 1 hour
        sqlx::query("UPDATE agent_missions SET status = 'STUCK' WHERE (status = 'PENDING' OR status = 'BURSTING') AND updated_at < $1 AND organization_id = $2")
            .bind(stuck_threshold)
            .bind(&self.org_id)
            .execute(&self.pool)
            .await?;
            
        // 1b. Immediately requeue STUCK missions
        sqlx::query("UPDATE agent_missions SET status = 'PENDING', updated_at = CURRENT_TIMESTAMP WHERE status = 'STUCK' AND organization_id = $1")
            .bind(&self.org_id)
            .execute(&self.pool)
            .await?;
            
        // 2. Mark missions as FAILED if they exceed the absolute age threshold
        sqlx::query("UPDATE agent_missions SET status = 'FAILED' WHERE (status = 'PENDING' OR status = 'STUCK' OR status = 'BURSTING') AND created_at < $1 AND organization_id = $2")
            .bind(fail_threshold)
            .bind(&self.org_id)
            .execute(&self.pool)
            .await?;
            
        // 3. Remove COMPLETED, or very old FAILED missions
        sqlx::query("WITH cte AS (SELECT id FROM agent_missions WHERE (status = 'COMPLETED' OR ((status = 'FAILED' OR status = 'STUCK' OR status = 'BURSTING') AND created_at < $1)) AND organization_id = $2 LIMIT 1000) DELETE FROM agent_missions WHERE id IN (SELECT id FROM cte)")
            .bind(fail_threshold)
            .bind(&self.org_id)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }

    pub async fn sync_buffered_metrics(&self, remote_endpoint: &str, batch_size: usize) -> Result<usize, sqlx::Error> {
        let batch_size = if batch_size == 0 { 500 } else { batch_size };
        
        let rows = sqlx::query("SELECT id, metric_type, payload FROM telemetry_buffer WHERE organization_id = $1 ORDER BY id ASC LIMIT $2")
            .bind(&self.org_id)
            .bind(batch_size as i64)
            .fetch_all(&self.pool)
            .await?;
            
        if rows.is_empty() {
            return Ok(0);
        }
        
        let mut records = Vec::new();
        let mut ids_to_delete = Vec::new();
        
        for row in rows {
            let id: i32 = row.get("id");
            let metric_type: String = row.get("metric_type");
            let payload: String = row.get("payload");
            
            ids_to_delete.push(id);
            records.push(serde_json::json!({
                "metric_type": metric_type,
                "payload": payload,
            }));
        }
        
        let payload_str = serde_json::to_string(&records).map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        
        let client = reqwest::Client::new();
        let response = client.post(remote_endpoint)
            .header("Content-Type", "application/json")
            .header("X-OHC-Conflict-Resolution", "force-local")
            .body(payload_str)
            .send()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
            
        if !response.status().is_success() {
            return Err(sqlx::Error::Protocol(format!("remote endpoint returned status: {}", response.status())));
        }
        
        sqlx::query("DELETE FROM telemetry_buffer WHERE id = ANY($1)")
            .bind(&ids_to_delete)
            .execute(&self.pool)
            .await?;
            
        Ok(records.len())
    }

    pub async fn sync_context_sync(&self, remote_endpoint: &str) -> Result<usize, sqlx::Error> {
        let rows = sqlx::query("SELECT memory_id, context FROM swarm_memory_embeddings WHERE organization_id = $1 ORDER BY created_at ASC LIMIT 100")
            .bind(&self.org_id)
            .fetch_all(&self.pool)
            .await?;
            
        if rows.is_empty() {
            return Ok(0);
        }
        
        let mut records = Vec::new();
        let mut ids_to_delete = Vec::new();
        
        for row in rows {
            let id: String = row.get("memory_id");
            let context: String = row.get("context");
            
            ids_to_delete.push(id.clone());
            records.push(serde_json::json!({
                "memory_id": id,
                "context": context,
            }));
        }
        
        let payload_str = serde_json::to_string(&records).map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        
        let client = reqwest::Client::new();
        let response = client.post(remote_endpoint)
            .header("Content-Type", "application/json")
            .body(payload_str)
            .send()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
            
        if !response.status().is_success() {
            return Err(sqlx::Error::Protocol(format!("remote endpoint returned status: {}", response.status())));
        }
        
        sqlx::query("DELETE FROM swarm_memory_embeddings WHERE memory_id = ANY($1)")
            .bind(&ids_to_delete)
            .execute(&self.pool)
            .await?;
            
        Ok(records.len())
    }

    pub async fn sync_missions(&self, remote_endpoint: &str) -> Result<usize, sqlx::Error> {
        let rows = sqlx::query("SELECT id, status, payload FROM agent_missions WHERE status IN ('PENDING', 'BURSTING') AND organization_id = $1 ORDER BY created_at ASC LIMIT 100")
            .bind(&self.org_id)
            .fetch_all(&self.pool)
            .await?;
            
        if rows.is_empty() {
            return Ok(0);
        }
        
        let mut records = Vec::new();
        let mut ids_to_update = Vec::new();
        
        for row in rows {
            let id: String = row.get("id");
            let status: String = row.get("status");
            let payload: String = row.get("payload");
            
            ids_to_update.push(id.clone());
            
            let mut payload_data: serde_json::Value = serde_json::from_str(&payload).unwrap_or_else(|_| serde_json::json!({}));
            payload_data["id"] = serde_json::Value::String(id);
            payload_data["status"] = serde_json::Value::String(status);
            
            records.push(payload_data);
        }
        
        let payload_str = serde_json::to_string(&records).map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        
        let client = reqwest::Client::new();
        let response = client.post(remote_endpoint)
            .header("Content-Type", "application/json")
            .header("X-OHC-Conflict-Resolution", "force-local")
            .body(payload_str)
            .send()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
            
        if !response.status().is_success() {
            return Err(sqlx::Error::Protocol(format!("remote endpoint returned status: {}", response.status())));
        }
        
        sqlx::query("UPDATE agent_missions SET status = 'SYNCED' WHERE id = ANY($1)")
            .bind(&ids_to_update)
            .execute(&self.pool)
            .await?;
            
        Ok(records.len())
    }

    pub async fn burst_mission(&self, mission_id: &str, remote_endpoint: &str) -> Result<(), sqlx::Error> {
        let result = sqlx::query("UPDATE agent_missions SET status = 'BURSTING', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND organization_id = $2")
            .bind(mission_id)
            .bind(&self.org_id)
            .execute(&self.pool)
            .await?;
            
        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        
        if !remote_endpoint.is_empty() {
            let row = sqlx::query("SELECT payload FROM agent_missions WHERE id = $1")
                .bind(mission_id)
                .fetch_one(&self.pool)
                .await?;
                
            let payload: String = row.get("payload");
            
            let client = reqwest::Client::new();
            let response = client.post(remote_endpoint)
                .header("Content-Type", "application/json")
                .body(payload)
                .send()
                .await
                .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
                
            if !response.status().is_success() {
                return Err(sqlx::Error::Protocol(format!("remote endpoint returned status: {}", response.status())));
            }
        }
        
        Ok(())
    }

    pub async fn inject_truth(&self, memory_id: &str, context: &str, embedding: Vec<f32>) -> Result<(), sqlx::Error> {
        let mut bytes = Vec::with_capacity(embedding.len() * 4);
        for f in embedding {
            bytes.extend_from_slice(&f.to_le_bytes());
        }
        
        sqlx::query("INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, created_at, organization_id) VALUES ($1, $2, $3, CURRENT_TIMESTAMP, $4) ON CONFLICT(memory_id) DO UPDATE SET context=EXCLUDED.context, vector_embedding=EXCLUDED.vector_embedding")
            .bind(memory_id)
            .bind(context)
            .bind(bytes)
            .bind(&self.org_id)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }


                        final_payload = serde_json::to_string(&json).unwrap_or(final_payload);
                    } else {
                        final_payload = format!("{}\n\n[SYSTEM GROUNDING]:\n{}", final_payload, content);
                    }
                } else {
                    final_payload = format!("{}\n\n[SYSTEM GROUNDING]:\n{}", final_payload, content);
                }
            }
        }

        self.upsert_mission(mission_id, "PENDING", &final_payload, true).await?;
        Ok(final_payload)
    }

    pub async fn inject_omni_context(&self, payload: &str) -> String {
        let mut final_payload = payload.to_string();

        if let Some(root) = &self.context_root {
            let agents_path = std::path::Path::new(root).join("AGENTS.md");
            let claude_path = std::path::Path::new(root).join("CLAUDE.md");

            let grounding = if let Ok(content) = tokio::fs::read_to_string(&agents_path).await {
                Some(content)
            } else if let Ok(content) = tokio::fs::read_to_string(&claude_path).await {
                Some(content)
            } else {
                None
            };

            if let Some(content) = grounding {
                if let Ok(mut json) = serde_json::from_str::<serde_json::Value>(&final_payload) {
                    if json.is_object() {
                        if let Some(task_content) = json.get_mut("content").and_then(|v| v.as_str()) {
                            let new_content = format!("{}\n\n[SYSTEM GROUNDING]:\n{}", task_content, content);
                            json["content"] = serde_json::Value::String(new_content);
                        } else {
                            let new_content = format!("\n\n[SYSTEM GROUNDING]:\n{}", content);
                            json["content"] = serde_json::Value::String(new_content);
                        }
                        final_payload = serde_json::to_string(&json).unwrap_or(final_payload);
                    } else {
                        final_payload = format!("{}\n\n[SYSTEM GROUNDING]:\n{}", final_payload, content);
                    }
                } else {
                    final_payload = format!("{}\n\n[SYSTEM GROUNDING]:\n{}", final_payload, content);
                }
            }
        }
        final_payload
    }

    pub async fn delegate_mission(&self, mission_id: &str, payload: &str) -> Result<String, sqlx::Error> {
        let final_payload = self.inject_omni_context(payload).await;
        self.upsert_mission(mission_id, "PENDING", &final_payload, true).await?;
        Ok(final_payload)
    }
    pub async fn upsert_mission(&self, mission_id: &str, status: &str, payload: &str, force_local: bool) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let row = sqlx::query("SELECT id FROM agent_missions WHERE id = $1 AND organization_id = $2 FOR UPDATE SKIP LOCKED")
            .bind(mission_id)
            .bind(&self.org_id)
            .fetch_optional(&mut *tx)
            .await?;

        if let Some(r) = row {
            let existing_id: String = r.get("id");
            if !existing_id.is_empty() && force_local {
                sqlx::query("UPDATE agent_missions SET status = $1, payload = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 AND organization_id = $4")
                    .bind(status)
                    .bind(payload)
                    .bind(mission_id)
                    .bind(&self.org_id)
                    .execute(&mut *tx)
                    .await?;
            }
        } else {
            let row_check = sqlx::query("SELECT id FROM agent_missions WHERE id = $1 AND organization_id = $2")
                .bind(mission_id)
                .bind(&self.org_id)
                .fetch_optional(&mut *tx)
                .await?;

            if let Some(_) = row_check {
                 if force_local {
                     sqlx::query("UPDATE agent_missions SET status = $1, payload = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 AND organization_id = $4")
                         .bind(status)
                         .bind(payload)
                         .bind(mission_id)
                         .bind(&self.org_id)
                         .execute(&mut *tx)
                         .await?;
                 }
            } else {
                 sqlx::query("INSERT INTO agent_missions (id, status, payload, created_at, updated_at, organization_id) VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $4) ON CONFLICT(id) DO NOTHING")
                     .bind(mission_id)
                     .bind(status)
                     .bind(payload)
                     .bind(&self.org_id)
                     .execute(&mut *tx)
                     .await?;
            }
        }

        tx.commit().await?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::env;

    // Helper to get SipDB with specific context_root without actual DB connection checks
    fn get_test_sip_db(context_root: Option<String>) -> SipDB {
        let pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap();
        SipDB {
            pool,
            org_id: "system".to_string(),
            local_cache: RwLock::new(HashMap::new()),
            cache_expirations: RwLock::new(HashMap::new()),
            context_root,
        }
    }

    #[tokio::test]
    async fn test_tc1_standard_delegation() {
        let payload = r#"{"content": "Build Dashboard"}"#;
        let db = get_test_sip_db(None);
        let result = db.inject_omni_context(payload).await;
        assert_eq!(result, payload);
    }

    #[tokio::test]
    async fn test_tc2_grounding_file_injection_agents() {
        let dir = env::temp_dir().join("tc2_test");
        fs::create_dir_all(&dir).unwrap();
        let agents_path = dir.join("AGENTS.md");
        fs::write(&agents_path, "Always write clean code.").unwrap();

        let payload = r#"{"content":"Build Dashboard"}"#;
        let db = get_test_sip_db(Some(dir.to_string_lossy().to_string()));
        let result = db.inject_omni_context(payload).await;

        let json: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(json["content"].as_str().unwrap(), "Build Dashboard\n\n[SYSTEM GROUNDING]:\nAlways write clean code.");

        fs::remove_dir_all(&dir).unwrap();
    }

    #[tokio::test]
    async fn test_tc3_grounding_file_injection_claude_fallback() {
        let dir = env::temp_dir().join("tc3_test");
        fs::create_dir_all(&dir).unwrap();
        let claude_path = dir.join("CLAUDE.md");
        fs::write(&claude_path, "Use specialized tokens.").unwrap();

        let payload = r#"{"content":"Build Dashboard"}"#;
        let db = get_test_sip_db(Some(dir.to_string_lossy().to_string()));
        let result = db.inject_omni_context(payload).await;

        let json: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(json["content"].as_str().unwrap(), "Build Dashboard\n\n[SYSTEM GROUNDING]:\nUse specialized tokens.");

        fs::remove_dir_all(&dir).unwrap();
    }

    #[tokio::test]
    async fn test_tc4_grounding_priority() {
        let dir = env::temp_dir().join("tc4_test");
        fs::create_dir_all(&dir).unwrap();
        let agents_path = dir.join("AGENTS.md");
        let claude_path = dir.join("CLAUDE.md");
        fs::write(&agents_path, "AGENTS content").unwrap();
        fs::write(&claude_path, "CLAUDE content").unwrap();

        let payload = r#"{"content":"Build Dashboard"}"#;
        let db = get_test_sip_db(Some(dir.to_string_lossy().to_string()));
        let result = db.inject_omni_context(payload).await;

        let json: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(json["content"].as_str().unwrap(), "Build Dashboard\n\n[SYSTEM GROUNDING]:\nAGENTS content");

        fs::remove_dir_all(&dir).unwrap();
    }

    #[tokio::test]
    async fn test_tc5_missing_files() {
        let dir = env::temp_dir().join("tc5_test");
        fs::create_dir_all(&dir).unwrap();

        let payload = r#"{"content":"Build Dashboard"}"#;
        let db = get_test_sip_db(Some(dir.to_string_lossy().to_string()));
        let result = db.inject_omni_context(payload).await;

        assert_eq!(result, payload);

        fs::remove_dir_all(&dir).unwrap();
    }
}
