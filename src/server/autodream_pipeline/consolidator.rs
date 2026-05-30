use std::sync::Arc;
use crate::db::{DB, DbStore};
use crate::autodream_pipeline::llm_client::LLMClient;
use sqlx::Row;

pub struct AutoDreamConsolidator {
    pub db: Arc<DB>,
    pub llm_client: Arc<dyn LLMClient>,
}

impl AutoDreamConsolidator {
    pub fn new(db: Arc<DB>, llm_client: Arc<dyn LLMClient>) -> Self {
        Self { db, llm_client }
    }

    fn chunk_content(content: &str, chunk_size: usize) -> Vec<String> {
        let mut chunks = Vec::new();
        let words: Vec<&str> = content.split_whitespace().collect();
        let mut current_chunk = String::new();
        let mut current_size = 0;

        for word in words {
            if current_size + word.len() + 1 > chunk_size && !current_chunk.is_empty() {
                chunks.push(current_chunk.trim().to_string());
                current_chunk.clear();
                current_size = 0;
            }
            current_chunk.push_str(word);
            current_chunk.push(' ');
            current_size += word.len() + 1;
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk.trim().to_string());
        }

        chunks
    }

    pub async fn process_agent_session_data(&self) -> Result<(), String> {
        match &self.db.store {
            DbStore::Postgres => {
                let query = "
                    SELECT session_id, agent_id, context_data
                    FROM agent_session_data
                    WHERE _sync_status = 'pending'
                    LIMIT 50
                ";

                let sessions = sqlx::query(query)
                    .fetch_all(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                for row in sessions {
                    let session_id: String = row.get("session_id");
                    let agent_id: String = row.get("agent_id");
                    let context_data: String = row.get("context_data");

                    let chunks = Self::chunk_content(&context_data, 2000);

                    for chunk in chunks {
                        match self.llm_client.generate_embedding(&chunk).await {
                            Ok(embedding) => {
                                let emb_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
                                let mem_id = uuid::Uuid::new_v4().to_string();

                                sqlx::query(
                                    "INSERT INTO autodream_memories (id, tenant_id, agent_id, task_id, content, embedding, source_type)
                                     VALUES ($1, 'system', $2, $3, $4, $5::vector, 'SESSION_DATA')"
                                )
                                .bind(&mem_id)
                                .bind(&agent_id)
                                .bind(&session_id)
                                .bind(&chunk)
                                .bind(&emb_str)
                                .execute(&self.db.pool)
                                .await
                                .map_err(|e| e.to_string())?;
                            }
                            Err(e) => {
                                tracing::error!("AutoDreamConsolidator: Failed to generate embedding for session {}: {}", session_id, e);
                            }
                        }
                    }

                    sqlx::query("UPDATE agent_session_data SET _sync_status = 'processed' WHERE session_id = $1")
                        .bind(&session_id)
                        .execute(&self.db.pool)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }
            DbStore::Sqlite(sqlite_pool) => {
                let query = "
                    SELECT session_id, agent_id, context_data
                    FROM agent_session_data
                    WHERE _sync_status = 'pending'
                    LIMIT 50
                ";

                let sessions = sqlx::query(query)
                    .fetch_all(sqlite_pool)
                    .await
                    .map_err(|e| e.to_string())?;

                for row in sessions {
                    let session_id: String = row.get("session_id");
                    let agent_id: String = row.get("agent_id");
                    let context_data: String = row.get("context_data");

                    let chunks = Self::chunk_content(&context_data, 2000);

                    for chunk in chunks {
                        match self.llm_client.generate_embedding(&chunk).await {
                            Ok(embedding) => {
                                let emb_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
                                let mem_id = uuid::Uuid::new_v4().to_string();

                                sqlx::query(
                                    "INSERT INTO autodream_memories (id, tenant_id, agent_id, task_id, content, embedding, source_type)
                                     VALUES (?, 'system', ?, ?, ?, ?, 'SESSION_DATA')"
                                )
                                .bind(&mem_id)
                                .bind(&agent_id)
                                .bind(&session_id)
                                .bind(&chunk)
                                .bind(&emb_str) // Stored as a JSON string for sqlite graceful degradation
                                .execute(sqlite_pool)
                                .await
                                .map_err(|e| e.to_string())?;
                            }
                            Err(e) => {
                                tracing::error!("AutoDreamConsolidator: Failed to generate embedding for session {}: {}", session_id, e);
                            }
                        }
                    }

                    sqlx::query("UPDATE agent_session_data SET _sync_status = 'processed' WHERE session_id = ?")
                        .bind(&session_id)
                        .execute(sqlite_pool)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }
        }
        Ok(())
    }
}
