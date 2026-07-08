use std::sync::Arc;
use crate::db::DB;
use crate::db::DbStore;
use std::time::Duration;
use uuid::Uuid;
use chrono::Utc;
use crate::services::agent_memory::service::{AgentMemoryService, AgentSessionSummary};

pub struct AgentSessionSummarizer {
    pub db: Arc<DB>,
}

impl AgentSessionSummarizer {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(300)); // Every 5 minutes
            loop {
                interval.tick().await;
                let _ = Self::poll(&db).await;
            }
        });
    }

    pub async fn poll(db: &Arc<DB>) -> Result<bool, String> {
        let rows = match &db.store {
            DbStore::Postgres => {
                let query = r#"
                    SELECT id, tenant_id, agent_id, session_id, context_data
                    FROM agent_session_data
                    WHERE _sync_status = 'pending' OR _sync_status IS NULL
                    LIMIT 50
                "#;
                sqlx::query(query)
                    .fetch_all(&db.pool)
                    .await
                    .map_err(|e| e.to_string())?
            },
            DbStore::Sqlite(_) => return Ok(false), // PgVector not available in sqlite backend for this feature
        };

        let memory_service = AgentMemoryService::new(None).with_db(db.pool.clone());

        for row in rows {
            use sqlx::Row;
            let id: Uuid = row.get("id");
            let tenant_id: String = row.get("tenant_id");
            let agent_id: String = row.get("agent_id");
            let session_id: String = row.get("session_id");
            let context_data: String = row.get("context_data");

            // Mark as processing
            if let DbStore::Postgres = &db.store {
                let _ = sqlx::query("UPDATE agent_session_data SET _sync_status = 'processing' WHERE id = $1")
                    .bind(id)
                    .execute(&db.pool)
                    .await;
            }

            // MOCK: Generate summary and embedding
            // In a real implementation this would call an LLM and an embedding endpoint.
            let summary_text = format!("Summary of session {}: {}", session_id, context_data.chars().take(100).collect::<String>());
            let mock_embedding = vec![0.1; 1536];

            let summary = AgentSessionSummary {
                id: Uuid::new_v4().to_string(),
                tenant_id: tenant_id.clone(),
                agent_id: agent_id.clone(),
                session_id: session_id.clone(),
                customer_id: None, // Typically extracted from context
                turn_index: 0,
                summary: summary_text,
                raw_state: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            let res = memory_service.save_agent_session_summary(&summary, Some(mock_embedding)).await;

            if let DbStore::Postgres = &db.store {
                if res.is_ok() {
                    let _ = sqlx::query("UPDATE agent_session_data SET _sync_status = 'summarized' WHERE id = $1")
                        .bind(id)
                        .execute(&db.pool)
                        .await;
                } else {
                    let _ = sqlx::query("UPDATE agent_session_data SET _sync_status = 'failed' WHERE id = $1")
                        .bind(id)
                        .execute(&db.pool)
                        .await;
                }
            }
        }

        Ok(true)
    }
}
