use std::sync::Arc;
use tokio::time::Duration;
use crate::db::DB;
use sqlx::Row;
use ohc_builtin_agent::llm::LlmClient;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LlmClassificationResult {
    pub intent: String,
    pub confidence: f64,
}

pub struct TriageIngestionWorker {
    db: Arc<DB>,
    llm: Option<Arc<dyn LlmClient>>,
}

impl TriageIngestionWorker {
    pub fn new(db: Arc<DB>, llm: Option<Arc<dyn LlmClient>>) -> Self {
        Self { db, llm }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            loop {
                match self.poll().await {
                    Ok(true) => {
                        // Processed a job, continue immediately
                        continue;
                    }
                    Ok(false) => {
                        // No jobs, sleep
                        tokio::time::sleep(Duration::from_millis(1000)).await;
                    }
                    Err(e) => {
                        tracing::error!("TriageIngestionWorker error: {}", e);
                        tokio::time::sleep(Duration::from_millis(5000)).await;
                    }
                }
            }
        });
    }

    pub async fn poll(&self) -> Result<bool, String> {
        let job = match &self.db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                let job = sqlx::query(
                    r#"
                    SELECT id, tenant_id, payload
                    FROM ohc_job_queue
                    WHERE status = 'PENDING' AND job_type = 'triage_ingestion'
                    ORDER BY created_at ASC
                    FOR UPDATE SKIP LOCKED
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(row) = job {
                    let job_id: String = row.get("id");
                    sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = NOW() WHERE id = $1")
                        .bind(&job_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;

                    Some((job_id, row.get::<String, _>("tenant_id"), row.get::<serde_json::Value, _>("payload")))
                } else {
                    None
                }
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                let job = sqlx::query(
                    r#"
                    SELECT id, tenant_id, payload
                    FROM ohc_job_queue
                    WHERE status = 'PENDING' AND job_type = 'triage_ingestion'
                    ORDER BY created_at ASC
                    LIMIT 1
                    "#
                )
                .fetch_optional(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(row) = job {
                    let job_id: String = row.get("id");
                    let payload_str: String = row.get("payload");
                    let payload: serde_json::Value = serde_json::from_str(&payload_str).map_err(|e| e.to_string())?;

                    sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&job_id)
                        .execute(sqlite_pool)
                        .await
                        .map_err(|e| e.to_string())?;

                    Some((job_id, row.get::<String, _>("tenant_id"), payload))
                } else {
                    None
                }
            }
        };

        if let Some((job_id, tenant_id, payload)) = job {
            tracing::info!("Processing triage job {}", job_id);

            let message_content = payload.get("content").and_then(|v| v.as_str()).unwrap_or("");

            let mut intent = "UNKNOWN".to_string();
            let mut confidence = 0.0;

            if let Some(llm) = &self.llm {
                let prompt = format!(
                    "Classify the following customer message intent into one of: SUPPORT, SALES, SPAM, UNKNOWN.\n\
                     Return ONLY a valid JSON object with 'intent' (string) and 'confidence' (number 0.0-1.0).\n\
                     Message: '{}'", message_content
                );

                let req = ohc_builtin_agent::types::ChatRequest {
                    model: "gpt-4o-mini".to_string(),
                    system: "You are an intent classification agent. Return ONLY JSON.".to_string(),
                    messages: vec![ohc_builtin_agent::types::Message {
                        role: ohc_builtin_agent::types::Role::User,
                        content: prompt,
                        tool_calls: vec![],
                        tool_results: vec![],
                        response_id: None, previous_response_id: None,

                    }],
                    tools: vec![],
                    max_tokens: 1000,
                    temperature: 0.0,




                };

                if let Ok(resp) = llm.chat(req).await {
                    if let Ok(result) = serde_json::from_str::<LlmClassificationResult>(&resp.message.content) {
                        intent = result.intent;
                        confidence = result.confidence;
                    }
                }
            } else {
                // Fallback deterministic logic for testing when no LLM is present
                if message_content.to_lowercase().contains("buy") {
                    intent = "SALES".to_string();
                    confidence = 0.9;
                } else if message_content.to_lowercase().contains("help") {
                    intent = "SUPPORT".to_string();
                    confidence = 0.9;
                }
            }

            // Update job queue with the result
            match &self.db.store {
                crate::db::DbStore::Postgres => {
                    if let Err(e) = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = NOW(), result = $1 WHERE id = $2")
                        .bind(serde_json::json!({"intent": intent, "confidence": confidence}))
                        .bind(&job_id)
                        .execute(&self.db.pool).await { tracing::error!("Failed to update job queue: {}", e); }

                    // In a real scenario, we'd also insert this result into a triage_items table or trigger the next worker.
                    if let Err(e) = sqlx::query(
                        "INSERT INTO triage_items (id, tenant_id, source_id, source_type, content, intent, status) VALUES ($1, $2, $3, $4, $5, $6, 'PENDING') ON CONFLICT DO NOTHING"
                    )
                    .bind(uuid::Uuid::new_v4().to_string())
                    .bind(&tenant_id)
                    .bind(payload.get("message_id").and_then(|v| v.as_str()).unwrap_or(""))
                    .bind(payload.get("source").and_then(|v| v.as_str()).unwrap_or(""))
                    .bind(&message_content)
                    .bind(&intent)
                    .execute(&self.db.pool).await { tracing::error!("Failed to update job queue: {}", e); }
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    if let Err(e) = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP, result = ? WHERE id = ?")
                        .bind(serde_json::json!({"intent": intent, "confidence": confidence}).to_string())
                        .bind(&job_id)
                        .execute(sqlite_pool).await { tracing::error!("Failed to update job queue: {}", e); }

                    if let Err(e) = sqlx::query(
                        "INSERT INTO triage_items (id, tenant_id, source_id, source_type, content, intent, status) VALUES (?, ?, ?, ?, ?, ?, 'PENDING') ON CONFLICT DO NOTHING"
                    )
                    .bind(uuid::Uuid::new_v4().to_string())
                    .bind(&tenant_id)
                    .bind(payload.get("message_id").and_then(|v| v.as_str()).unwrap_or(""))
                    .bind(payload.get("source").and_then(|v| v.as_str()).unwrap_or(""))
                    .bind(&message_content)
                    .bind(&intent)
                    .execute(sqlite_pool).await { tracing::error!("Failed to update job queue: {}", e); }
                }
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DB;
    use ohc_builtin_agent::llm::LlmClient;
    use ohc_builtin_agent::types::{ChatRequest, ChatResponse, Message, Role, Usage};
    use async_trait::async_trait;

    struct MockLlmClient {
        intent: String,
        confidence: f64,
    }

    #[async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync + 'static>> {
            let result = LlmClassificationResult {
                intent: self.intent.clone(),
                confidence: self.confidence,
            };
            Ok(ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: serde_json::to_string(&result).unwrap(),
                    tool_calls: vec![],
                    tool_results: vec![],
                    response_id: None, previous_response_id: None,

                },
                usage: Usage {
                    input_tokens: 10,
                    output_tokens: 10,
                    cache_creation_input_tokens: 0,
                    cache_read_input_tokens: 0,
                },
                stop_reason: "stop".to_string(),
                response_id: None,
            })
        }


    }

    #[tokio::test]
    async fn test_worker_polls_and_classifies_with_llm() {
        let pool = crate::db::create_sqlite_pool_for_test().await;
        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://").unwrap(), // Mocked
            store: crate::db::DbStore::Sqlite(pool.clone()),
        });

        // Setup table
        sqlx::query("CREATE TABLE ohc_job_queue (id TEXT PRIMARY KEY, tenant_id TEXT, job_type TEXT, payload TEXT, status TEXT, result TEXT, created_at DATETIME DEFAULT CURRENT_TIMESTAMP, updated_at DATETIME DEFAULT CURRENT_TIMESTAMP, next_retry_at DATETIME DEFAULT CURRENT_TIMESTAMP)")
            .execute(&pool).await.unwrap();

        sqlx::query("CREATE TABLE triage_items (id TEXT PRIMARY KEY, tenant_id TEXT, source_id TEXT, source_type TEXT, content TEXT, intent TEXT, status TEXT, created_at DATETIME DEFAULT CURRENT_TIMESTAMP)")
            .execute(&pool).await.unwrap();

        // Insert job
        sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, ?, ?, ?)")
            .bind("job1")
            .bind("tenant1")
            .bind("triage_ingestion")
            .bind(serde_json::json!({"content": "I need help with my order", "message_id": "msg1", "source": "email"}).to_string())
            .bind("PENDING")
            .execute(&pool).await.unwrap();

        let mock_llm = Arc::new(MockLlmClient {
            intent: "SUPPORT".to_string(),
            confidence: 0.95,
        });

        let worker = TriageIngestionWorker::new(db.clone(), Some(mock_llm));

        let result = worker.poll().await.unwrap();
        assert!(result);

        // Verify job completed
        let job_status: String = sqlx::query_scalar("SELECT status FROM ohc_job_queue WHERE id = 'job1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(job_status, "COMPLETED");

        let job_result: String = sqlx::query_scalar("SELECT result FROM ohc_job_queue WHERE id = 'job1'")
            .fetch_one(&pool).await.unwrap();
        let parsed_result: LlmClassificationResult = serde_json::from_str(&job_result).unwrap();
        assert_eq!(parsed_result.intent, "SUPPORT");
        assert_eq!(parsed_result.confidence, 0.95);

        // Verify triage item created
        let count: i64 = sqlx::query_scalar("SELECT count(*) FROM triage_items WHERE intent = 'SUPPORT'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_worker_polls_no_jobs() {
        let pool = crate::db::create_sqlite_pool_for_test().await;
        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://").unwrap(), // Mocked
            store: crate::db::DbStore::Sqlite(pool.clone()),
        });
        sqlx::query("CREATE TABLE ohc_job_queue (id TEXT PRIMARY KEY, tenant_id TEXT, job_type TEXT, payload TEXT, status TEXT, result TEXT, created_at DATETIME DEFAULT CURRENT_TIMESTAMP, updated_at DATETIME DEFAULT CURRENT_TIMESTAMP, next_retry_at DATETIME DEFAULT CURRENT_TIMESTAMP)")
            .execute(&pool).await.unwrap();

        let worker = TriageIngestionWorker::new(db.clone(), None);
        let result = worker.poll().await.unwrap();
        assert!(!result);
    }
}
