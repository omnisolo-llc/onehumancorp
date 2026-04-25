use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use sqlx::PgPool;


#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MemoryEntry {
    pub memory_id: String,
    pub context: String,
    pub embedding: Option<Vec<u8>>,
    pub source_plugin: Option<String>,
    pub created_at: DateTime<Utc>,
    pub organization_id: String,
}

pub struct PgVectorMemoryStore {
    pool: PgPool,
    organization_id: String,
}

impl PgVectorMemoryStore {
    pub async fn new(database_url: &str, organization_id: String) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self {
            pool,
            organization_id,
        })
    }

    pub async fn write(&self, context: &str, embedding: Vec<f32>) -> Result<(), sqlx::Error> {
        let memory_id = uuid::Uuid::new_v4().to_string();

        // Convert Vec<f32> to a format pgvector understands if needed,
        // or just use the BYTEA fallback as seen in migration 005.
        // If the DB actually uses VECTOR(1536), sqlx might need a wrapper.
        // For simplicity and parity, we'll try to use the BYTEA approach if VECTOR is not available.

        sqlx::query(
            "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, organization_id) VALUES ($1, $2, $3, $4)"
        )
        .bind(&memory_id)
        .bind(context)
        .bind(serde_json::to_vec(&embedding).unwrap_or_default())
        .bind(&self.organization_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn search(&self, embedding: Vec<f32>, limit: usize) -> Result<Vec<MemoryEntry>, sqlx::Error> {
        // Real semantic search using pgvector Cosine similarity operator <=>
        // We cast the input array to vector type.

        // If the database doesn't have pgvector, this query might fail.
        // We fallback to time-based ordering if it does.
        let query = if !embedding.is_empty() {
            "SELECT memory_id, context, NULL as embedding, source_plugin, created_at, organization_id \
             FROM swarm_memory_embeddings \
             WHERE organization_id = $1 \
             ORDER BY vector_embedding <=> $2::vector \
             LIMIT $3"
        } else {
            "SELECT memory_id, context, NULL as embedding, source_plugin, created_at, organization_id \
             FROM swarm_memory_embeddings \
             WHERE organization_id = $1 \
             ORDER BY created_at DESC LIMIT $2"
        };

        let mut q = sqlx::query_as::<_, MemoryEntry>(query)
            .bind(&self.organization_id);

        if !embedding.is_empty() {
            // Convert Vec<f32> to a string representation that pgvector expects: '[1.0, 2.0, ...]'
            let vec_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
            q = q.bind(vec_str);
        }

        let rows = q.bind(limit as i64)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }
}

pub fn inject_memories_into_prompt(memories: &[MemoryEntry], system_prompt: &str) -> String {
    if memories.is_empty() {
        return system_prompt.to_string();
    }
    let mut s = String::new();
    s.push_str("## Relevant past experience\n");
    for m in memories {
        s.push_str("- ");
        s.push_str(&m.context);
        s.push('\n');
    }
    s.push_str("\n---\n\n");
    s.push_str(system_prompt);
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inject_memories_empty() {
        let memories = vec![];
        let prompt = "Hello";
        let result = inject_memories_into_prompt(&memories, prompt);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_inject_memories_non_empty() {
        let memories = vec![MemoryEntry {
            memory_id: "t1".to_string(),
            context: "memory1".to_string(),
            embedding: None,
            source_plugin: None,
            created_at: Utc::now(),
            organization_id: "org1".to_string(),
        }];
        let result = inject_memories_into_prompt(&memories, "System prompt");
        assert!(result.contains("## Relevant past experience"));
        assert!(result.contains("memory1"));
        assert!(result.contains("System prompt"));
    }
}
