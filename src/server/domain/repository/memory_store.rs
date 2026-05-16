use std::sync::Arc;
use crate::db::{DB, DbStore};
use sqlx::Row;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: String,
    pub tenant_id: String,
    pub agent_id: String,
    pub content: String,
    pub embedding: Option<Vec<f32>>,
    pub source_type: String,
    pub created_at: Option<DateTime<Utc>>,
}

pub struct MemoryStore {
    db: Arc<DB>,
}

impl MemoryStore {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_memory(&self, memory: Memory) -> Result<Memory, String> {
        let embedding_val: Option<Vec<f32>> = memory.embedding.clone();

        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                // Setting tenant context for RLS
                sqlx::query("SET app.current_tenant = $1")
                    .bind(&memory.tenant_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                sqlx::query(
                    r#"
                    INSERT INTO consolidated_memory (id, tenant_id, agent_id, content, embedding, source_type, created_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    "#
                )
                .bind(&memory.id)
                .bind(&memory.tenant_id)
                .bind(&memory.agent_id)
                .bind(&memory.content)
                .bind(embedding_val)
                .bind(&memory.source_type)
                .bind(memory.created_at.unwrap_or_else(Utc::now))
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;
                Ok(memory)
            }
            DbStore::Sqlite(pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO consolidated_memory (id, tenant_id, agent_id, content, source_type, created_at)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    "#
                )
                .bind(&memory.id)
                .bind(&memory.tenant_id)
                .bind(&memory.agent_id)
                .bind(&memory.content)
                .bind(&memory.source_type)
                .bind(memory.created_at.unwrap_or_else(Utc::now))
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;

                Ok(memory)
            }
        }
    }

    pub async fn search_similar_memories(&self, tenant_id: &str, query_embedding: Vec<f32>, limit: i64) -> Result<Vec<Memory>, String> {
        let vector = query_embedding;

        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                // Crucial for RLS
                sqlx::query("SET app.current_tenant = $1")
                    .bind(tenant_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                let rows = sqlx::query(
                    r#"
                    SELECT id, tenant_id, agent_id, content, source_type, created_at,
                           embedding::vector
                    FROM consolidated_memory
                    WHERE tenant_id = $1
                    ORDER BY embedding <-> $2
                    LIMIT $3
                    "#
                )
                .bind(tenant_id)
                .bind(vector)
                .bind(limit)
                .fetch_all(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;

                let memories = rows.into_iter().map(|row| {
                    let emb: Option<Vec<f32>> = row.try_get("embedding").ok();
                    Memory {
                        id: row.get("id"),
                        tenant_id: row.get("tenant_id"),
                        agent_id: row.get::<Option<String>, _>("agent_id").unwrap_or_default(),
                        content: row.get("content"),
                        embedding: emb,
                        source_type: row.get("source_type"),
                        created_at: row.try_get("created_at").ok(),
                    }
                }).collect();

                Ok(memories)
            }
            DbStore::Sqlite(pool) => {
                // SQLite doesn't have pgvector, so just return latest ones
                let rows = sqlx::query(
                    r#"
                    SELECT id, tenant_id, agent_id, content, source_type, created_at
                    FROM consolidated_memory
                    WHERE tenant_id = $1
                    ORDER BY created_at DESC
                    LIMIT $2
                    "#
                )
                .bind(tenant_id)
                .bind(limit)
                .fetch_all(pool)
                .await
                .map_err(|e| e.to_string())?;

                let memories = rows.into_iter().map(|row| Memory {
                    id: row.get("id"),
                    tenant_id: row.get("tenant_id"),
                    agent_id: row.get::<Option<String>, _>("agent_id").unwrap_or_default(),
                    content: row.get("content"),
                    embedding: None,
                    source_type: row.get("source_type"),
                    created_at: row.try_get("created_at").ok(),
                }).collect();

                Ok(memories)
            }
        }
    }
}

include!("memory_store_tests.rs");
