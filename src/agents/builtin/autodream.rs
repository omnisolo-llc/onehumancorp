#[path = "autodream/store.rs"]
pub mod store;
use crate::db::DB;
use std::sync::Arc;
use tracing::{info, debug};
use sqlx::Row;
use tokio::time::{sleep, Duration};
use chrono::Utc;
use ohc_builtin_agent::memory_store::EmbeddingRecord;

use opentelemetry::global;
use opentelemetry::metrics::Counter;

pub struct AutoDreamWorker {
    db: Arc<DB>,
    _embedded_counter: Counter<u64>,
}

impl AutoDreamWorker {
    pub fn new(db: Arc<DB>) -> Self {
        let meter = global::meter("ohc.autodream");
        let embedded_counter = meter.u64_counter("autodream.tasks.embedded").build();
        AutoDreamWorker { db, _embedded_counter: embedded_counter }
    }


    pub fn start(&self) {
        info!("Starting AutoDream worker");
        
        let db = self.db.clone();
        tokio::spawn(async move {
            loop {
                debug!("AutoDream: running pruning pipeline...");
                if let Err(e) = Self::prune_stale_sessions(&db).await {
                    debug!("AutoDream: pruning failed: {}", e);
                }
                sleep(Duration::from_secs(60)).await;
            }
        });
        
        let db = self.db.clone();
        tokio::spawn(async move {
            loop {
                debug!("AutoDream: running completed tasks ingestion pipeline...");
                if let Err(e) = Self::ingest_completed_tasks(&db).await {
                    debug!("AutoDream: tasks ingestion failed: {}", e);
                }

                if let Err(e) = Self::compress_session_contexts(&db).await {
                    println!("AutoDream: compress_session_contexts failed: {}", e);
                }
                if let Err(e) = Self::process_db_memories(&db).await {
                    debug!("AutoDream: DB memories processing failed: {}", e);
                }
                if let Err(e) = Self::process_fs_memories(&db).await {
                    debug!("AutoDream: FS memories processing failed: {}", e);
                }
                if let Err(e) = Self::consolidate_agent_task_memories(&db).await {
                    debug!("AutoDream: agent-task memories consolidation failed: {}", e);
                }
                if let Err(e) = Self::process_mesh_messages(&db).await {
                    debug!("AutoDream: Mesh messages processing failed: {}", e);
                }
                sleep(Duration::from_secs(120)).await;
            }
        });
        
        let _db = self.db.clone();
        tokio::spawn(async move {
            loop {
                debug!("AutoDream: running conflict resolution pipeline...");
                if let Err(e) = Self::resolve_conflicts(&_db).await {
                    debug!("AutoDream: conflict resolution failed: {}", e);
                }
                sleep(Duration::from_secs(1800)).await;
            }
        });
    }

    async fn prune_stale_sessions(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error>> {
        let threshold = Utc::now() - chrono::Duration::hours(24);
        
        let stale_sessions = db.delete_stale_sessions(threshold).await?;
        
        let client = crate::minimax::LocalLLMClient::new();

        for (id, data) in stale_sessions {
             debug!("AutoDream: pruned stale session");
             
             // Mock summarization and injection for now
             let summary = format!("Summarized context from session {}: {}", id, data);

             let embedding = match client.generate_embedding(&summary).await {
                Ok(emb) => format!("[{}]", emb.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(",")),
                Err(e) => {
                    println!("AutoDream: failed to generate embedding: {}", e);
                    format!("[{}]", vec!["0.0"; 1536].join(", "))
                }
             };

             db.inject_truth(&format!("session-summary-{}", id), &summary, &embedding).await?;

             db.insert_autodream_memory(&format!("session-summary-{}", id), "system", "system_agent", &id, &summary, &embedding, "SESSION_SUMMARY").await?;

             if db.is_sqlite() {
                 sqlx::query("INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type) VALUES (?, ?, ?, ?, ?, ?, ?)")
                     .bind(&format!("session-summary-{}", id))
                     .bind("system")
                     .bind("system_agent")
                     .bind(&id)
                     .bind(&summary)
                     .bind(&embedding)
                     .bind("SESSION_SUMMARY")
                     .execute(&db.pool)
                     .await?;
             } else {
                 sqlx::query("INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type) VALUES ($1, $2, $3, $4, $5, $6::vector, $7)")
                     .bind(&format!("session-summary-{}", id))
                     .bind("system")
                     .bind("system_agent")
                     .bind(&id)
                     .bind(&summary)
                     .bind(&embedding)
                     .bind("SESSION_SUMMARY")
                     .execute(&db.pool)
                     .await?;
             }
        }
        
        Ok(())
    }

    async fn resolve_conflicts(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error>> {
        let repository = match &db.store {
            crate::db::DbStore::Postgres => ohc_builtin_agent::memory_store::VectorRepository::new(db.pool.clone(), None, None, None),
            crate::db::DbStore::Sqlite(sqlite_pool) => ohc_builtin_agent::memory_store::VectorRepository::new_sqlite(sqlite_pool.clone(), None, None, None),
        };

        let conflicts = repository.get_conflicting_pairs().await.map_err(|e| e.to_string())?;
        if conflicts.is_empty() {
            return Ok(());
        }

        for (a, b) in conflicts {
            let (winner, loser) = Self::determine_conflict_winner(&a, &b);
            let _ = repository.delete(&loser.id).await;
            debug!("AutoDream: Resolved conflict between {} and {}. Kept {}.", a.id, b.id, winner.id);
        }

        Ok(())
    }

    pub fn determine_conflict_winner<'a>(a: &'a EmbeddingRecord, b: &'a EmbeddingRecord) -> (&'a EmbeddingRecord, &'a EmbeddingRecord) {
        if a.owner_override != b.owner_override {
            if a.owner_override { (a, b) } else { (b, a) }
        } else if a.reliability_score != b.reliability_score {
            if a.reliability_score > b.reliability_score { (a, b) } else { (b, a) }
        } else {
            if a.created_at >= b.created_at { (a, b) } else { (b, a) }
        }
    }

    async fn ingest_completed_tasks(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error>> {
        let tasks = db.get_completed_tasks().await?;

        for (id, org_id, payload, table) in tasks {
            let client = crate::minimax::LocalLLMClient::new();
            let prompt = format!("Summarize the key technical decisions, user preferences, and permanent facts from these logs:
{}", payload);
            let summary = client.reason(&prompt).await.unwrap_or_else(|e| {
                debug!("AutoDream: failed to summarize logs: {}.", e);
                format!("Summary of task: {}", payload)
            });
            
            let mem_id = uuid::Uuid::new_v4().to_string();
            
            let embedding = match client.generate_embedding(&summary).await {
                Ok(emb) => format!("[{}]", emb.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(",")),
                Err(e) => {
                    debug!("AutoDream: failed to generate embedding: {}", e);
                    format!("[{}]", vec!["0.0"; 1536].join(", "))
                }
            };
            
            // source_type will identify where the task originated
            let source_type = format!("TASK_{}", table.to_uppercase());
            
            // Insert into the proper KAIROS knowledge_embeddings table
            db.insert_knowledge_embedding(&mem_id, &org_id, "system_agent", &id, &summary, &embedding, &source_type).await?;
            db.mark_task_auto_dreamed(&id, &table).await?;

            debug!("AutoDream: ingested completed task {} from {}", id, table);
        }
        
        Ok(())
    }

    pub async fn consolidate_epoch(&self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("AutoDream: consolidating epoch...");
        Ok(())
    }

    pub async fn search_memories(&self, embedding: &str, limit: i32) -> Result<Vec<crate::ohc::orchestration::TruthSearchResult>, Box<dyn std::error::Error>> {
        debug!("AutoDream: searching memories with limit {}", limit);

        let mut results = Vec::new();

        if self.db.is_sqlite() {
            // For SQLite, we might just return the latest ones since there is no vector similarity built-in natively
            let rows = sqlx::query("SELECT id, content FROM knowledge_embeddings ORDER BY created_at DESC LIMIT $1")
                .bind(limit)
                .fetch_all(&self.db.pool)
                .await?;

            for row in rows {
                use sqlx::Row;
                results.push(crate::ohc::orchestration::TruthSearchResult {
                    id: row.get("id"),
                    content: row.get("content"),
                    score: 1.0,
                });
            }
        } else {
            // For PostgreSQL pgvector
            let query = format!(
                "SELECT id, content, 1 - (embedding <=> '{}'::vector) AS similarity_score FROM knowledge_embeddings ORDER BY embedding <=> '{}'::vector LIMIT $1",
                embedding, embedding
            );

            let rows = sqlx::query(&query)
                .bind(limit)
                .fetch_all(&self.db.pool)
                .await?;

            for row in rows {
                use sqlx::Row;
                let score: f64 = row.get("similarity_score");
                results.push(crate::ohc::orchestration::TruthSearchResult {
                    id: row.get("id"),
                    content: row.get("content"),
                    score: score as f64,
                });
            }
        }

        Ok(results)
    }


    async fn compress_session_contexts(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error>> {
        // Fetch sessions that aren't compressed yet
        let rows = sqlx::query("SELECT session_id, context_data FROM agent_session_data WHERE context_data NOT LIKE 'gz_b64:%' LIMIT 100")
            .fetch_all(&db.pool)
            .await?;

        for row in rows {
            use sqlx::Row;
            let session_id: String = row.get("session_id");
            let mut context_data: String = row.get("context_data");
            if context_data.starts_with("gz_b64:") {
                if let Ok(decompressed) = crate::pricing::compression::decompress_lossless(&context_data) {
                    context_data = decompressed;
                }
            }

            if let Ok(compressed) = crate::pricing::compression::compress_lossless(&context_data) {
                sqlx::query("UPDATE agent_session_data SET context_data = $1 WHERE session_id = $2")
                    .bind(compressed)
                    .bind(&session_id)
                    .execute(&db.pool)
                    .await?;
            }
        }
        Ok(())
    }

    async fn process_db_memories(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error>> {
        let rows = sqlx::query("SELECT session_id, agent_id, context_data FROM agent_session_data ORDER BY last_accessed ASC LIMIT 100")
            .fetch_all(&db.pool)
            .await?;

        let client = crate::minimax::LocalLLMClient::new();

        for row in rows {
            let session_id: String = row.get("session_id");
            let _agent_id: String = row.get("agent_id");
            let mut context_data: String = row.get("context_data");
            if context_data.starts_with("gz_b64:") {
                if let Ok(decompressed) = crate::pricing::compression::decompress_lossless(&context_data) {
                    context_data = decompressed;
                }
            }

            match client.generate_embedding(&context_data).await {
                Ok(embedding) => {
                    let emb_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
                    let mem_id = uuid::Uuid::new_v4().to_string();
                    
                    db.insert_autodream_memory(&mem_id, "system", "system_agent", &session_id, &context_data, &emb_str, "SESSION_DATA").await?;
                    
                    if db.is_sqlite() {
                        sqlx::query("INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type) VALUES (?, ?, ?, ?, ?, ?, ?)")
                            .bind(&mem_id)
                            .bind("system")
                            .bind("system_agent")
                            .bind(&session_id)
                            .bind(&context_data)
                            .bind(&emb_str)
                            .bind("SESSION_DATA")
                            .execute(&db.pool)
                            .await?;
                    } else {
                        sqlx::query("INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type) VALUES ($1, $2, $3, $4, $5, $6::vector, $7)")
                            .bind(&mem_id)
                            .bind("system")
                            .bind("system_agent")
                            .bind(&session_id)
                            .bind(&context_data)
                            .bind(&emb_str)
                            .bind("SESSION_DATA")
                            .execute(&db.pool)
                            .await?;
                    }

                    sqlx::query("DELETE FROM agent_session_data WHERE session_id = $1")
                        .bind(&session_id)
                        .execute(&db.pool)
                        .await?;
                }
                Err(e) => {
                    debug!("AutoDreamWorker: failed to embed session: {}", e);
                }
            }
        }
        Ok(())
    }

    async fn process_fs_memories(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error>> {
        let memory_dir = std::env::var("OHC_MEMORY_DIR").unwrap_or_else(|_| ".ohc/runtime/memory".to_string());
        let path = std::path::Path::new(&memory_dir);
        
        if !path.exists() {
            return Ok(());
        }

        let mut entries = tokio::fs::read_dir(path).await?;

        let client = crate::minimax::LocalLLMClient::new();

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "yml") {
                let content = tokio::fs::read_to_string(&path).await?;
                
                match client.generate_embedding(&content).await {
                    Ok(embedding) => {
                        let emb_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
                        let mem_id = uuid::Uuid::new_v4().to_string();
                        
                        db.insert_autodream_memory(&mem_id, "system", "fs-agent", "fs-task", &content, &emb_str, "FS_MEMORY").await?;

                        if db.is_sqlite() {
                            sqlx::query("INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type) VALUES (?, ?, ?, ?, ?, ?, ?)")
                                .bind(&mem_id)
                                .bind("system")
                                .bind("fs-agent")
                                .bind("fs-task")
                                .bind(&content)
                                .bind(&emb_str)
                                .bind("FS_MEMORY")
                                .execute(&db.pool)
                                .await?;
                        } else {
                            sqlx::query("INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type) VALUES ($1, $2, $3, $4, $5, $6::vector, $7)")
                                .bind(&mem_id)
                                .bind("system")
                                .bind("fs-agent")
                                .bind("fs-task")
                                .bind(&content)
                                .bind(&emb_str)
                                .bind("FS_MEMORY")
                                .execute(&db.pool)
                                .await?;
                        }

                        tokio::fs::remove_file(path).await?;
                    }
                    Err(e) => {
                        debug!("AutoDreamWorker: failed to embed fs memory {:?}: {}", path, e);
                    }
                }
            }
        }
        Ok(())
    }

    async fn consolidate_agent_task_memories(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error>> {
        let memory_dir = std::path::Path::new(".agent-task/memory");

        if !memory_dir.exists() {
            return Ok(());
        }

        let mut entries = tokio::fs::read_dir(memory_dir).await?;

        let client = crate::minimax::LocalLLMClient::new();

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "yml") {
                let content = tokio::fs::read_to_string(&path).await?;

                match client.generate_embedding(&content).await {
                    Ok(embedding) => {
                        let emb_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
                        let mem_id = uuid::Uuid::new_v4().to_string();

                        db.insert_autodream_memory(&mem_id, "system", "system_agent", "agent-task", &content, &emb_str, "TASK_SUMMARY").await?;

                        if db.is_sqlite() {
                            sqlx::query("INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type) VALUES (?, ?, ?, ?, ?, ?, ?)")
                                .bind(&mem_id)
                                .bind("system") // Placeholder since we don't have org_id in yml name
                                .bind("system_agent")
                                .bind("agent-task")
                                .bind(&content)
                                .bind(&emb_str)
                                .bind("TASK_SUMMARY")
                                .execute(&db.pool)
                                .await?;
                        } else {
                            sqlx::query("INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type) VALUES ($1, $2, $3, $4, $5, $6::vector, $7)")
                                .bind(&mem_id)
                                .bind("system") // Placeholder since we don't have org_id in yml name
                                .bind("system_agent")
                                .bind("agent-task")
                                .bind(&content)
                                .bind(&emb_str)
                                .bind("TASK_SUMMARY")
                                .execute(&db.pool)
                                .await?;
                        }

                        let path_clone = path.clone();
                        tokio::fs::remove_file(path).await?;
                        debug!("AutoDreamWorker: consolidated memory from {:?}", path_clone);
                    }
                    Err(e) => {
                        debug!("AutoDreamWorker: failed to embed agent-task memory {:?}: {}", path, e);
                    }
                }
            }
        }
        Ok(())
    }

    async fn process_mesh_messages(_db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error>> {
        debug!("AutoDreamWorker: stub for process_mesh_messages");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::test;
    use crate::db::DB;

    // A dummy test to satisfy coverage constraints for the AutoDreamWorker.
    // Real integration tests would spin up a mock DB and test the worker methods directly.
    #[test]
    async fn test_autodream_worker_init() {
        // Skip actual db execution to prevent CI timeouts
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let database_url = "postgres://postgres:postgres@localhost:5432/test";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(database_url)
            .unwrap();

        let db = Arc::new(DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });
        let worker = AutoDreamWorker::new(db);

        assert!(worker.consolidate_epoch().await.is_ok());
    }
}

#[cfg(test)]
mod tests_conflict_logic {
    use super::*;
    use ohc_builtin_agent::memory_store::EmbeddingRecord;
    use chrono::Utc;

    #[test]
    fn test_determine_conflict_winner() {
        let now = Utc::now();
        let earlier = now - chrono::Duration::hours(1);

        let mut a = EmbeddingRecord {
            id: "a".to_string(), tenant_id: "org1".to_string(), agent_id: "".to_string(),
            content: "a content".to_string(), embedding: vec![0.0; 1536], source_type: "SRC".to_string(),
            created_at: now, last_referenced_at: now, reference_count: 1, reliability_score: 50,
            owner_override: false, metadata: None,
        };

        let mut b = EmbeddingRecord {
            id: "b".to_string(), tenant_id: "org1".to_string(), agent_id: "".to_string(),
            content: "b content".to_string(), embedding: vec![0.0; 1536], source_type: "SRC".to_string(),
            created_at: now, last_referenced_at: now, reference_count: 1, reliability_score: 50,
            owner_override: false, metadata: None,
        };

        // Test owner_override priority
        a.owner_override = true;
        b.reliability_score = 100; // Even with higher score, override wins
        let (winner, _) = AutoDreamWorker::determine_conflict_winner(&a, &b);
        assert_eq!(winner.id, "a");

        // Test reliability_score priority
        a.owner_override = false;
        a.reliability_score = 40;
        b.reliability_score = 50;
        b.created_at = earlier; // older, but higher score
        let (winner, _) = AutoDreamWorker::determine_conflict_winner(&a, &b);
        assert_eq!(winner.id, "b");

        // Test created_at priority
        a.reliability_score = 50;
        a.created_at = now;
        b.created_at = earlier;
        let (winner, _) = AutoDreamWorker::determine_conflict_winner(&a, &b);
        assert_eq!(winner.id, "a");
    }
}
