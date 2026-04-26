use crate::db::DB;
use std::sync::Arc;
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

    async fn prune_stale_sessions(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error>> {
        let threshold = Utc::now() - chrono::Duration::hours(24);
        
        let mut tx = db.pool.begin().await?;
        db.set_organization_context(&mut *tx, "system").await?;

        let stale_sessions = db.delete_stale_sessions(&mut *tx, threshold).await?;
        
        for (id, data) in stale_sessions {
             println!("AutoDream: pruned session {}: {}", id, data);
             
             // Mock summarization and injection for now
             let summary = format!("Summarized context from session {}: {}", id, data);
             db.inject_truth(&mut *tx, &format!("session-summary-{}", id), &summary, "[0.0]").await?;
        }
        
        db.delete_stale_sessions_cleanup(&mut *tx, threshold).await?;
        tx.commit().await?;

        Ok(())
    }

    async fn ingest_completed_tasks(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error>> {
        let mut tx = db.pool.begin().await?;
        db.set_organization_context(&mut *tx, "system").await?;

        let tasks = db.get_completed_tasks(&mut *tx).await?;
        
        for (id, org_id, payload) in tasks {
            let summary = format!("Summary of task: {}", payload);
            let mem_id = uuid::Uuid::new_v4().to_string();
            
            // Mock embedding
            let embedding = "[0.1]"; 
            
            db.insert_agent_memory(&mut *tx, &mem_id, &org_id, &id, &summary, embedding).await?;
            db.mark_task_auto_dreamed(&mut *tx, &id).await?;
            
            println!("AutoDream: ingested completed task {}", id);
        }
        
        tx.commit().await?;

        Ok(())
    }
}
