use dashmap::DashMap;

use tokio::sync::OnceCell;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EpisodicMemory {
    pub session_id: String,
    pub tenant_id: String,
    pub content: String,
    pub updated_at: DateTime<Utc>,
}

pub struct AgentMemoryService {
    redis_client: Option<redis::Client>,
    redis_conn: OnceCell<redis::aio::MultiplexedConnection>,
    fallback_cache: DashMap<String, EpisodicMemory>,

}

impl AgentMemoryService {
    pub fn new(redis_client: Option<redis::Client>) -> Self {
        Self {
            redis_client,
            redis_conn: OnceCell::new(),
            fallback_cache: DashMap::new(),

        }
    }

    async fn get_redis_conn(&self) -> Option<redis::aio::MultiplexedConnection> {
        if let Some(client) = &self.redis_client {
            let conn = self.redis_conn.get_or_try_init(|| async {
                client.get_multiplexed_tokio_connection().await
            }).await;

            if let Ok(conn) = conn {
                return Some(conn.clone());
            }
        }
        None
    }

    fn build_key(tenant_id: &str, session_id: &str) -> String {
        format!("ohc:mem:{}:{}", tenant_id, session_id)
    }

    pub async fn save_episodic_memory(&self, tenant_id: &str, session_id: &str, content: &str) -> Result<(), String> {
        let key = Self::build_key(tenant_id, session_id);
        let memory = EpisodicMemory {
            session_id: session_id.to_string(),
            tenant_id: tenant_id.to_string(),
            content: content.to_string(),
            updated_at: Utc::now(),
        };

        if let Some(mut conn) = self.get_redis_conn().await {
            use redis::AsyncCommands;
            let serialized = serde_json::to_string(&memory).map_err(|e| e.to_string())?;
            // 7 days TTL (7 * 24 * 60 * 60 = 604800 seconds)
            let _: () = conn.set_ex(&key, serialized, 604800).await.map_err(|e| e.to_string())?;
            return Ok(());
        }

        // Fallback to in-memory store
        self.fallback_cache.insert(key, memory);
        Ok(())
    }

    pub async fn retrieve_recent_memory(&self, tenant_id: &str, session_id: &str) -> Result<Option<EpisodicMemory>, String> {
        let key = Self::build_key(tenant_id, session_id);

        if let Some(mut conn) = self.get_redis_conn().await {
            use redis::AsyncCommands;
            let result: Option<String> = conn.get(&key).await.map_err(|e| e.to_string())?;
            if let Some(serialized) = result {
                let memory: EpisodicMemory = serde_json::from_str(&serialized).map_err(|e| e.to_string())?;
                // Verify tenant_id to ensure strict isolation
                if memory.tenant_id != tenant_id {
                    return Err(format!("Tenant isolation violation: expected tenant_id {}, found {}", tenant_id, memory.tenant_id));
                }
                return Ok(Some(memory));
            }
            return Ok(None);
        }

        // Fallback to in-memory store
        if let Some(memory) = self.fallback_cache.get(&key) {
            // Verify tenant_id to ensure strict isolation
            if memory.tenant_id != tenant_id {
                return Err(format!("Tenant isolation violation: expected tenant_id {}, found {}", tenant_id, memory.tenant_id));
            }
            return Ok(Some(memory.clone()));
        }
        Ok(None)
    }
    pub async fn retrieve_tenant_memory(&self, tenant_id: &str) -> Result<Vec<EpisodicMemory>, String> {
        let pattern = format!("ohc:mem:{}:*", tenant_id);
        let mut memories = Vec::new();

        if let Some(mut conn) = self.get_redis_conn().await {
            use redis::AsyncCommands;
            let mut keys = Vec::new();
            {
                let mut iter: redis::AsyncIter<String> = conn.scan_match(&pattern).await.map_err(|e| e.to_string())?;
                while let Some(key) = iter.next_item().await {
                    keys.push(key);
                }
            }
            for key in keys {
                if let Ok(Some(serialized)) = conn.get::<_, Option<String>>(&key).await {
                    if let Ok(memory) = serde_json::from_str::<EpisodicMemory>(&serialized) {
                        if memory.tenant_id == tenant_id {
                            memories.push(memory);
                        }
                    }
                }
            }
            return Ok(memories);
        }

        // Fallback to in-memory store
        for entry in self.fallback_cache.iter() {
            let key = entry.key();
            let memory = entry.value();
            if key.starts_with(&format!("ohc:mem:{}:", tenant_id)) && memory.tenant_id == tenant_id {
                memories.push(memory.clone());
            }
        }

        Ok(memories)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_save_and_retrieve_fallback() {
        let service = AgentMemoryService::new(None);

        let tenant_id = "tenant_a";
        let session_id = "session_123";
        let content = "Hello, this is a test.";

        service.save_episodic_memory(tenant_id, session_id, content).await.unwrap();

        let retrieved = service.retrieve_recent_memory(tenant_id, session_id).await.unwrap().unwrap();
        assert_eq!(retrieved.content, content);
        assert_eq!(retrieved.tenant_id, tenant_id);
        assert_eq!(retrieved.session_id, session_id);
    }

    #[tokio::test]
    async fn test_tenant_isolation() {
        let service = AgentMemoryService::new(None);

        let tenant_a = "tenant_a";
        let tenant_b = "tenant_b";
        let session_id = "session_123";
        let content = "Tenant A content";

        service.save_episodic_memory(tenant_a, session_id, content).await.unwrap();

        // Attempting to retrieve Tenant A's memory using Tenant B's ID should return None
        // since the key is prefixed with tenant_id (ohc:mem:tenant_b:session_123 vs ohc:mem:tenant_a:session_123)
        let retrieved = service.retrieve_recent_memory(tenant_b, session_id).await.unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_tenant_isolation_malicious_key_manipulation() {
        let service = AgentMemoryService::new(None);

        let tenant_a = "tenant_a";
        let tenant_b = "tenant_b";
        let session_id = "session_123";
        let content = "Tenant A content";

        service.save_episodic_memory(tenant_a, session_id, content).await.unwrap();

        // Directly inserting a corrupted entry to simulate a potential exploit where data belongs to Tenant A but is under Tenant B's key
        let malicious_key = AgentMemoryService::build_key(tenant_b, session_id);
        let malicious_memory = EpisodicMemory {
            session_id: session_id.to_string(),
            tenant_id: tenant_a.to_string(), // The data claims to belong to Tenant A
            content: "Malicious content".to_string(),
            updated_at: Utc::now(),
        };

        service.fallback_cache.insert(malicious_key, malicious_memory);

        // When Tenant B tries to retrieve it, it should fail because the tenant_id inside the memory (tenant_a)
        // does not match the requested tenant_id (tenant_b).
        let result = service.retrieve_recent_memory(tenant_b, session_id).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Tenant isolation violation"));
    }
}
