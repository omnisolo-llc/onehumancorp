use dashmap::DashMap;

use tokio::sync::OnceCell;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use sqlx::PgPool;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EpisodicMemory {
    pub session_id: String,
    pub tenant_id: String,
    pub content: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AgentSessionSummary {
    pub id: String,
    pub tenant_id: String,
    pub agent_id: String,
    pub session_id: String,
    pub customer_id: Option<String>,
    pub turn_index: i32,
    pub summary: String,
    pub raw_state: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct AgentMemoryService {
    redis_client: Option<redis::Client>,
    redis_conn: OnceCell<redis::aio::MultiplexedConnection>,
    fallback_cache: DashMap<String, EpisodicMemory>,
    db_pool: Option<PgPool>,
}

impl AgentMemoryService {
    pub fn new(redis_client: Option<redis::Client>) -> Self {
        Self {
            redis_client,
            redis_conn: OnceCell::new(),
            fallback_cache: DashMap::new(),
            db_pool: None,
        }
    }

    pub fn with_db(mut self, pool: PgPool) -> Self {
        self.db_pool = Some(pool);
        self
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

    pub async fn save_agent_session_summary(&self, summary: &AgentSessionSummary, embedding: Option<Vec<f32>>) -> Result<(), String> {
        let pool = self.db_pool.as_ref().ok_or("Database pool not configured")?;

        let emb_str = embedding.map(|emb| format!("[{}]", emb.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(",")));

        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

        // Ensure RLS
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&summary.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            INSERT INTO agent_session_summaries (id, tenant_id, agent_id, session_id, customer_id, turn_index, summary, summary_embedding, raw_state, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8::vector, $9, $10, $11)
            ON CONFLICT (id) DO UPDATE SET
                turn_index = EXCLUDED.turn_index,
                summary = EXCLUDED.summary,
                summary_embedding = EXCLUDED.summary_embedding,
                raw_state = EXCLUDED.raw_state,
                updated_at = EXCLUDED.updated_at
            "#
        )
        .bind(&summary.id)
        .bind(&summary.tenant_id)
        .bind(&summary.agent_id)
        .bind(&summary.session_id)
        .bind(&summary.customer_id)
        .bind(summary.turn_index)
        .bind(&summary.summary)
        .bind(&emb_str)
        .bind(&summary.raw_state)
        .bind(summary.created_at)
        .bind(summary.updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn pre_flight_rehydrate(&self, tenant_id: &str, customer_id: Option<&str>, query_embedding: Option<&Vec<f32>>, limit: i64) -> Result<Vec<AgentSessionSummary>, String> {
        let pool = self.db_pool.as_ref().ok_or("Database pool not configured")?;

        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

        // Ensure RLS
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        let query_str = if let Some(embedding) = query_embedding {
            let emb_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));

            if customer_id.is_some() {
                 sqlx::query(
                    r#"
                    SELECT id, tenant_id, agent_id, session_id, customer_id, turn_index, summary, raw_state, created_at, updated_at
                    FROM agent_session_summaries
                    WHERE tenant_id = $1 AND customer_id = $2
                    ORDER BY summary_embedding <=> $3::vector
                    LIMIT $4
                    "#
                )
                .bind(tenant_id)
                .bind(customer_id.unwrap())
                .bind(emb_str)
                .bind(limit)
            } else {
                 sqlx::query(
                    r#"
                    SELECT id, tenant_id, agent_id, session_id, customer_id, turn_index, summary, raw_state, created_at, updated_at
                    FROM agent_session_summaries
                    WHERE tenant_id = $1
                    ORDER BY summary_embedding <=> $2::vector
                    LIMIT $3
                    "#
                )
                .bind(tenant_id)
                .bind(emb_str)
                .bind(limit)
            }
        } else {
            if customer_id.is_some() {
                sqlx::query(
                    r#"
                    SELECT id, tenant_id, agent_id, session_id, customer_id, turn_index, summary, raw_state, created_at, updated_at
                    FROM agent_session_summaries
                    WHERE tenant_id = $1 AND customer_id = $2
                    ORDER BY created_at DESC
                    LIMIT $3
                    "#
                )
                .bind(tenant_id)
                .bind(customer_id.unwrap())
                .bind(limit)
            } else {
                sqlx::query(
                    r#"
                    SELECT id, tenant_id, agent_id, session_id, customer_id, turn_index, summary, raw_state, created_at, updated_at
                    FROM agent_session_summaries
                    WHERE tenant_id = $1
                    ORDER BY created_at DESC
                    LIMIT $2
                    "#
                )
                .bind(tenant_id)
                .bind(limit)
            }
        };

        let rows = query_str.fetch_all(&mut *tx).await.map_err(|e| e.to_string())?;

        let mut summaries = Vec::new();
        for row in rows {
            use sqlx::Row;
            summaries.push(AgentSessionSummary {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                agent_id: row.get("agent_id"),
                session_id: row.get("session_id"),
                customer_id: row.try_get("customer_id").unwrap_or(None),
                turn_index: row.get("turn_index"),
                summary: row.get("summary"),
                raw_state: row.try_get("raw_state").unwrap_or(None),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(summaries)
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

    #[tokio::test]
    async fn test_pre_flight_rehydrate_tenant_isolation() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let database_url = "postgres://postgres:postgres@localhost:5432/test";
        let pool_res = sqlx::postgres::PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect(database_url)
            .await;

        let pool = match pool_res {
            Ok(p) => p,
            Err(_) => return,
        };

        let service = AgentMemoryService::new(None).with_db(pool.clone());

        // Ensure table exists for testing
        sqlx::query("CREATE EXTENSION IF NOT EXISTS vector;").execute(&pool).await.unwrap_or_default();
        sqlx::query("CREATE TABLE IF NOT EXISTS agent_session_summaries (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, agent_id TEXT NOT NULL, session_id TEXT NOT NULL, customer_id TEXT, turn_index INTEGER NOT NULL DEFAULT 0, summary TEXT NOT NULL, summary_embedding vector(1536), raw_state JSONB, created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP);").execute(&pool).await.unwrap_or_default();
        sqlx::query("ALTER TABLE agent_session_summaries ENABLE ROW LEVEL SECURITY;").execute(&pool).await.unwrap_or_default();

        // Bypass RLS to insert mock data
        let mut tx = pool.begin().await.unwrap();
        sqlx::query("SELECT set_config('app.current_tenant', '', true)").execute(&mut *tx).await.unwrap();
        sqlx::query("DELETE FROM agent_session_summaries;").execute(&mut *tx).await.unwrap();

        let summary_a = AgentSessionSummary {
            id: "sum_a".to_string(),
            tenant_id: "tenant_a".to_string(),
            agent_id: "agent_1".to_string(),
            session_id: "sess_a".to_string(),
            customer_id: None,
            turn_index: 1,
            summary: "A summary".to_string(),
            raw_state: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let summary_b = AgentSessionSummary {
            id: "sum_b".to_string(),
            tenant_id: "tenant_b".to_string(),
            agent_id: "agent_1".to_string(),
            session_id: "sess_b".to_string(),
            customer_id: None,
            turn_index: 1,
            summary: "B summary".to_string(),
            raw_state: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        tx.commit().await.unwrap();

        service.save_agent_session_summary(&summary_a, None).await.unwrap();
        service.save_agent_session_summary(&summary_b, None).await.unwrap();

        // Query as tenant_a
        let results_a = service.pre_flight_rehydrate("tenant_a", None, None, 10).await.unwrap();
        assert_eq!(results_a.len(), 1);
        assert_eq!(results_a[0].id, "sum_a");

        // Query as tenant_b
        let results_b = service.pre_flight_rehydrate("tenant_b", None, None, 10).await.unwrap();
        assert_eq!(results_b.len(), 1);
        assert_eq!(results_b[0].id, "sum_b");
    }
}
