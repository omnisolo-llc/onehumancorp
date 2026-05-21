use std::sync::Arc;
use crate::db::{DB, DbStore};
use crate::autodream_pipeline::llm_client::LLMClient;
use tokio::time::{sleep, Duration};
use sqlx::Row;
use uuid::Uuid;
use tracing::{info, error, debug};

pub struct AutoDreamWorker {
    db: Arc<DB>,
    llm_client: Arc<dyn LLMClient>,
}

#[derive(Debug)]
struct SessionRow {
    session_id: String,
    agent_id: String,
    context_data: String,
}

impl AutoDreamWorker {
    pub fn new(db: Arc<DB>, llm_client: Arc<dyn LLMClient>) -> Self {
        Self { db, llm_client }
    }

    pub async fn run(&self) {
        info!("Starting AutoDreamWorker consolidation loop");
        loop {
            debug!("AutoDreamWorker: Starting consolidation cycle");
            if let Err(e) = self.consolidate_memories().await {
                error!("AutoDreamWorker error during consolidation: {}", e);
            }
            sleep(Duration::from_secs(60)).await;
        }
    }

    pub async fn consolidate_memories(&self) -> Result<(), Box<dyn std::error::Error>> {
        let batch_limit = 500;

        let rows = if self.db.is_sqlite() {
            match &self.db.store {
                DbStore::Sqlite(pool) => {
                    let r = sqlx::query("SELECT session_id, agent_id, context_data FROM agent_session_data LIMIT ?")
                        .bind(batch_limit)
                        .fetch_all(pool)
                        .await?;
                    r.into_iter().map(|row| SessionRow {
                        session_id: row.get("session_id"),
                        agent_id: row.get("agent_id"),
                        context_data: row.get("context_data"),
                    }).collect::<Vec<_>>()
                },
                _ => unreachable!(),
            }
        } else {
            let r = sqlx::query("SELECT session_id, agent_id, context_data FROM agent_session_data LIMIT $1")
                .bind(batch_limit)
                .fetch_all(&self.db.pool)
                .await?;
            r.into_iter().map(|row| SessionRow {
                session_id: row.get("session_id"),
                agent_id: row.get("agent_id"),
                context_data: row.get("context_data"),
            }).collect::<Vec<_>>()
        };

        if rows.is_empty() {
            debug!("AutoDreamWorker: No memories to consolidate");
            return Ok(());
        }

        info!("AutoDreamWorker: Consolidating {} memory sessions", rows.len());

        for row in rows {
            let session_id = row.session_id;
            let agent_id = row.agent_id;
            let context_data = row.context_data;

            // 1. Generate summary/consolidation (In this implementation, we treat context as content)
            // In a real scenario, we might use LLM to summarize if it's too large
            let content_to_embed = if context_data.len() > 2000 {
                &context_data[..2000] // Simple truncation for now, or use LLM to summarize
            } else {
                &context_data
            };

            // 2. Generate embedding
            let embedding = match self.llm_client.generate_embedding(content_to_embed).await {
                Ok(emb) => emb,
                Err(e) => {
                    error!("AutoDreamWorker: Failed to generate embedding for session {}: {}", session_id, e);
                    continue;
                }
            };

            let emb_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
            let memory_id = Uuid::new_v4().to_string();

            // 3. Store in autodream_memories
            // We'll use the DB helper if available, but the prompt asked for branching logic here as well
            if self.db.is_sqlite() {
                match &self.db.store {
                    DbStore::Sqlite(pool) => {
                        sqlx::query("INSERT INTO autodream_memories (id, tenant_id, agent_id, task_id, content, embedding, source_type) VALUES (?, ?, ?, ?, ?, ?, ?)")
                            .bind(&memory_id)
                            .bind("system")
                            .bind(&agent_id)
                            .bind(&session_id)
                            .bind(content_to_embed)
                            .bind(None::<Vec<u8>>) // SQLite fallback for embeddings
                            .bind("SESSION_CONSOLIDATION")
                            .execute(pool)
                            .await?;
                    },
                    _ => unreachable!(),
                }
            } else {
                sqlx::query("INSERT INTO autodream_memories (id, tenant_id, agent_id, task_id, content, embedding, source_type) VALUES ($1, $2, $3, $4, $5, $6::vector, $7)")
                    .bind(&memory_id)
                    .bind("system")
                    .bind(&agent_id)
                    .bind(&session_id)
                    .bind(content_to_embed)
                    .bind(&emb_str)
                    .bind("SESSION_CONSOLIDATION")
                    .execute(&self.db.pool)
                    .await?;
            }

            // 4. Delete from ephemeral storage
            if self.db.is_sqlite() {
                match &self.db.store {
                    DbStore::Sqlite(pool) => {
                        sqlx::query("DELETE FROM agent_session_data WHERE session_id = ?")
                            .bind(&session_id)
                            .execute(pool)
                            .await?;
                    },
                    _ => unreachable!(),
                }
            } else {
                sqlx::query("DELETE FROM agent_session_data WHERE session_id = $1")
                    .bind(&session_id)
                    .execute(&self.db.pool)
                    .await?;
            }
        }

        Ok(())
    }
}
