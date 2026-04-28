pub mod store;
use crate::db::DB;
use std::sync::Arc;
use sqlx::Row;
use tokio::time::{sleep, Duration};
use chrono::{Utc, DateTime};

pub struct AutoDreamWorker {
    db: Arc<DB>,
}

impl AutoDreamWorker {
    pub fn new(db: Arc<DB>) -> Self {
        AutoDreamWorker { db }
    }


    pub fn start(&self) {
        println!("Starting AutoDream worker");
        
        let db = self.db.clone();
        tokio::spawn(async move {
            loop {
                println!("AutoDream: running pruning pipeline...");
                if let Err(e) = Self::prune_stale_sessions(&db).await {
                    println!("AutoDream: pruning failed: {}", e);
                }
                sleep(Duration::from_secs(60)).await;
            }
        });
        
        let db = self.db.clone();
        tokio::spawn(async move {
            loop {
                println!("AutoDream: running completed tasks ingestion pipeline...");
                if let Err(e) = Self::ingest_completed_tasks(&db).await {
                    println!("AutoDream: tasks ingestion failed: {}", e);
                }
                if let Err(e) = Self::process_db_memories(&db).await {
                    println!("AutoDream: DB memories processing failed: {}", e);
                }
                if let Err(e) = Self::process_fs_memories(&db).await {
                    println!("AutoDream: FS memories processing failed: {}", e);
                }
                if let Err(e) = Self::process_mesh_messages(&db).await {
                    println!("AutoDream: Mesh messages processing failed: {}", e);
                }
                sleep(Duration::from_secs(120)).await;
            }
        });
        
        let db = self.db.clone();
        tokio::spawn(async move {
            loop {
                println!("AutoDream: running conflict resolution pipeline...");
                // TODO: implement conflict resolution
                sleep(Duration::from_secs(1800)).await;
            }
        });
    }

    async fn ingest_completed_tasks(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error>> {
        let rows = sqlx::query(
            "SELECT id, entity_id, agent_id FROM state_machine_transitions WHERE entity_type = 'shared_task' AND to_state = 'COMPLETED'"
        ).fetch_all(&db.pool).await?;

        let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
        if api_key.is_empty() {
            return Ok(());
        }
        let client = crate::minimax::MinimaxClient::new(api_key);

        for row in rows {
            let transition_id: String = row.get("id");
            let entity_id: String = row.get("entity_id");

            let task_row_res = sqlx::query("SELECT organization_id, title, payload FROM shared_tasks_v4 WHERE id = $1")
                .bind(&entity_id)
                .fetch_optional(&db.pool)
                .await?;

            if let Some(task_row) = task_row_res {
                let org_id: String = task_row.get("organization_id");
                let title: String = task_row.get("title");
                let payload: String = task_row.get("payload");

                let context_str = format!("Task: {}\nResult: {}", title, payload);

                if let Ok(embedding) = client.generate_embedding(&context_str).await {
                    let mem_id = format!("task-{}", entity_id);
                    let sip_db = crate::sip::SipDB::new(db.pool.clone(), org_id);
                    let _ = sip_db.inject_truth(&mem_id, &context_str, embedding).await;
                }
            }

            sqlx::query("DELETE FROM state_machine_transitions WHERE id = $1")
                .bind(&transition_id)
                .execute(&db.pool)
                .await?;
        }

        Ok(())
    }


    async fn ingest_completed_tasks(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error>> {
        let rows = sqlx::query(
            "SELECT id, entity_id, agent_id FROM state_machine_transitions WHERE entity_type = 'shared_task' AND to_state = 'COMPLETED'"
        ).fetch_all(&db.pool).await?;

        let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
        if api_key.is_empty() {
            return Ok(());
        }
        let client = crate::minimax::MinimaxClient::new(api_key);

        for row in rows {
            let transition_id: String = row.get("id");
            let entity_id: String = row.get("entity_id");

            let task_row_res = sqlx::query("SELECT organization_id, title, payload FROM shared_tasks_v4 WHERE id = $1")
                .bind(&entity_id)
                .fetch_optional(&db.pool)
                .await?;

            if let Some(task_row) = task_row_res {
                let org_id: String = task_row.get("organization_id");
                let title: String = task_row.get("title");
                let payload: String = task_row.get("payload");

                let context_str = format!("Task: {}\nResult: {}", title, payload);

                if let Ok(embedding) = client.generate_embedding(&context_str).await {
                    let mem_id = format!("task-{}", entity_id);
                    let sip_db = crate::sip::SipDB::new(db.pool.clone(), org_id);
                    let _ = sip_db.inject_truth(&mem_id, &context_str, embedding).await;
                }
            }

            sqlx::query("DELETE FROM state_machine_transitions WHERE id = $1")
                .bind(&transition_id)
                .execute(&db.pool)
                .await?;
        }

        Ok(())
    }

    async fn prune_stale_sessions(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error>> {
        let threshold = Utc::now() - chrono::Duration::hours(24);
        
        let stale_sessions = db.delete_stale_sessions(threshold).await?;
        
        for (id, data) in stale_sessions {
             println!("AutoDream: pruned session {}: {}", id, data);
             
             // Mock summarization and injection for now
             let summary = format!("Summarized context from session {}: {}", id, data);
             db.inject_truth(&format!("session-summary-{}", id), &summary, "[0.0]").await?;
        }
        
        Ok(())
    }

    async fn ingest_completed_tasks(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error>> {
        let tasks = db.get_completed_tasks().await?;
        
        for (id, org_id, payload) in tasks {
            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
            let client = crate::minimax::MinimaxClient::new(api_key);
            let prompt = format!("Summarize the key technical decisions, user preferences, and permanent facts from these logs:\n{}", payload);
            let summary = client.reason(&prompt).await.unwrap_or_else(|e| {
                println!("AutoDream: failed to summarize logs: {}. Using raw payload.", e);
                format!("Summary of task: {}", payload)
            });
            
            let mem_id = uuid::Uuid::new_v4().to_string();
            
            let embedding = match client.generate_embedding(&summary).await {
                Ok(emb) => serde_json::to_string(&emb).unwrap(),
                Err(e) => {
                    println!("AutoDream: failed to generate embedding: {}", e);
                    "[0.0]".to_string()
                }
            };
            
            db.insert_agent_memory(&mem_id, &org_id, &id, &summary, &embedding).await?;
            db.mark_task_auto_dreamed(&id).await?;
            
            println!("AutoDream: ingested completed task {}", id);
        }
        
        Ok(())
    }

    pub async fn consolidate_epoch(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("AutoDream: consolidating epoch...");
        Ok(())
    }

    pub async fn search_memories(&self, embedding: &str, limit: i32) -> Result<Vec<crate::ohc::orchestration::TruthSearchResult>, Box<dyn std::error::Error>> {
        println!("AutoDream: searching memories with embedding {} and limit {}", embedding, limit);
        Ok(vec![
            crate::ohc::orchestration::TruthSearchResult {
                id: "mem1".to_string(),
                content: "Mock memory content".to_string(),
                score: 0.9,
            }
        ])
    }

    async fn process_db_memories(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error>> {
        let rows = sqlx::query("SELECT session_id, agent_id, context_data FROM agent_session_data ORDER BY last_accessed ASC LIMIT 100")
            .fetch_all(&db.pool)
            .await?;

        let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
        let client = crate::minimax::MinimaxClient::new(api_key);

        for row in rows {
            let session_id: String = row.get("session_id");
            let agent_id: String = row.get("agent_id");
            let context_data: String = row.get("context_data");

            match client.generate_embedding(&context_data).await {
                Ok(embedding) => {
                    let emb_str = serde_json::to_string(&embedding).unwrap();
                    let mem_id = uuid::Uuid::new_v4().to_string();
                    
                    db.insert_agent_memory(&mem_id, "system", &format!("session-{}", session_id), &context_data, &emb_str).await?;
                    
                    sqlx::query("DELETE FROM agent_session_data WHERE session_id = $1")
                        .bind(&session_id)
                        .execute(&db.pool)
                        .await?;
                }
                Err(e) => {
                    println!("AutoDreamWorker: failed to embed session {}: {}", session_id, e);
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

        let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
        let client = crate::minimax::MinimaxClient::new(api_key);

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "yml") {
                let content = tokio::fs::read_to_string(&path).await?;
                
                match client.generate_embedding(&content).await {
                    Ok(embedding) => {
                        let emb_str = serde_json::to_string(&embedding).unwrap();
                        let mem_id = uuid::Uuid::new_v4().to_string();
                        
                        db.insert_agent_memory(&mem_id, "system", "fs-agent", &content, &emb_str).await?;
                        
                        tokio::fs::remove_file(path).await?;
                    }
                    Err(e) => {
                        println!("AutoDreamWorker: failed to embed fs memory {:?}: {}", path, e);
                    }
                }
            }
        }
        Ok(())
    }

    async fn process_mesh_messages(_db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error>> {
        println!("AutoDreamWorker: stub for process_mesh_messages");
        Ok(())
    }
}
