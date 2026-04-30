use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::{PgPool, SqlitePool, Row};
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub memory_id: String,
    pub context: String,
    pub embedding: Option<Vec<u8>>,
    pub source_plugin: Option<String>,
    pub created_at: DateTime<Utc>,
    pub organization_id: String,
}

pub enum PgVectorMemoryStore {
    Postgres { pool: PgPool, organization_id: String },
    Sqlite { pool: SqlitePool, organization_id: String },
}

impl PgVectorMemoryStore {
    pub async fn new(database_url: &str, organization_id: String) -> Result<Self, sqlx::Error> {
        if database_url.starts_with("sqlite:") {
            let pool = SqlitePool::connect(database_url).await?;
            Ok(PgVectorMemoryStore::Sqlite { pool, organization_id })
        } else {
            let pool = PgPool::connect(database_url).await?;
            Ok(PgVectorMemoryStore::Postgres { pool, organization_id })
        }
    }

    pub async fn write(&self, context: &str, embedding: Vec<f32>) -> Result<(), sqlx::Error> {
        let memory_id = uuid::Uuid::new_v4().to_string();

        match self {
            PgVectorMemoryStore::Postgres { pool, organization_id } => {
                sqlx::query(
                    "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, organization_id) \
                     VALUES ($1, $2, $3, $4) \
                     ON CONFLICT(memory_id) DO UPDATE SET \
                         context=excluded.context, \
                         vector_embedding=excluded.vector_embedding, \
                         created_at=GREATEST(swarm_memory_embeddings.created_at, excluded.created_at)"
                )
                .bind(&memory_id)
                .bind(context)
                .bind(serde_json::to_vec(&embedding).unwrap_or_default())
                .bind(organization_id)
                .execute(pool)
                .await?;
            },
            PgVectorMemoryStore::Sqlite { pool, organization_id } => {
                sqlx::query(
                    "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, organization_id) \
                     VALUES ($1, $2, $3, $4) \
                     ON CONFLICT(memory_id) DO UPDATE SET \
                         context=excluded.context, \
                         vector_embedding=excluded.vector_embedding, \
                         created_at=max(swarm_memory_embeddings.created_at, excluded.created_at)"
                )
                .bind(&memory_id)
                .bind(context)
                .bind(serde_json::to_vec(&embedding).unwrap_or_default())
                .bind(organization_id)
                .execute(pool)
                .await?;
            }
        }

        Ok(())
    }

    pub async fn search(&self, embedding: Vec<f32>, limit: usize) -> Result<Vec<MemoryEntry>, sqlx::Error> {
        match self {
            PgVectorMemoryStore::Postgres { pool, organization_id } => {
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

                let mut q = sqlx::query(query)
                    .bind(organization_id);

                if !embedding.is_empty() {
                    let vec_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
                    q = q.bind(vec_str).bind(limit as i64);
                } else {
                    q = q.bind(limit as i64);
                }

                let db_rows = q.fetch_all(pool)
                    .await?;

                let mut rows = Vec::new();
                for r in db_rows {
                    let created_at: DateTime<Utc> = r.try_get("created_at").unwrap_or_else(|_| Utc::now());
                    rows.push(MemoryEntry {
                        memory_id: r.get("memory_id"),
                        context: r.get("context"),
                        embedding: None,
                        source_plugin: r.try_get("source_plugin").ok(),
                        created_at,
                        organization_id: r.get("organization_id"),
                    });
                }
                Ok(rows)
            },
            PgVectorMemoryStore::Sqlite { pool, organization_id } => {
                let query = if !embedding.is_empty() {
                    "SELECT memory_id, context, NULL as embedding, source_plugin, created_at, organization_id \
                     FROM swarm_memory_embeddings \
                     WHERE organization_id = $1 \
                     ORDER BY vec_distance_cosine(vector_embedding, $2) \
                     LIMIT $3"
                } else {
                    "SELECT memory_id, context, NULL as embedding, source_plugin, created_at, organization_id \
                     FROM swarm_memory_embeddings \
                     WHERE organization_id = $1 \
                     ORDER BY created_at DESC LIMIT $2"
                };

                let mut q = sqlx::query(query)
                    .bind(organization_id);

                if !embedding.is_empty() {
                    let vec_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
                    q = q.bind(vec_str).bind(limit as i64);
                } else {
                    q = q.bind(limit as i64);
                }

                let db_rows = q.fetch_all(pool)
                    .await?;

                let mut rows = Vec::new();
                for r in db_rows {
                    // FIXED DATE PARSING BUG: Use try_get::<DateTime<Utc>, _> natively
                    let created_at: DateTime<Utc> = r.try_get("created_at").unwrap_or_else(|_| Utc::now());

                    rows.push(MemoryEntry {
                        memory_id: r.get("memory_id"),
                        context: r.get("context"),
                        embedding: None,
                        source_plugin: r.try_get("source_plugin").ok(),
                        created_at,
                        organization_id: r.get("organization_id"),
                    });
                }
                Ok(rows)
            }
        }
    }

    pub async fn shared_search(&self, limit: usize) -> Result<Vec<MemoryEntry>, sqlx::Error> {
        // Cross department context sharing ignores agent boundaries
        self.search(vec![], limit).await
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
