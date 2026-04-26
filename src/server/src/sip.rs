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
}

impl SipDB {
    pub fn new(pool: PgPool, org_id: String) -> Self {
        SipDB {
            pool,
            org_id,
            local_cache: RwLock::new(HashMap::new()),
            cache_expirations: RwLock::new(HashMap::new()),
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
        
        // 1. Mark stagnant PENDING missions as STUCK after 1 hour
        sqlx::query("UPDATE agent_missions SET status = 'STUCK' WHERE (status = 'PENDING' OR status = 'BURSTING') AND created_at < $1 AND organization_id = $2")
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
}
