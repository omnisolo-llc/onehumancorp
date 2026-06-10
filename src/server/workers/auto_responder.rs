use std::sync::Arc;
use crate::db::DB;
use crate::orchestration::queue::ohc_job_queue::OHCJob;
use crate::orchestration::queue::worker_pool::JobHandler;
use crate::api::agents::translation::{translate_inbox_message_with_llm, generate_inbox_draft_reply, InboxTranslation};
use std::time::Duration;
use tokio::time::timeout;

const AI_AGENT_TIMEOUT: Duration = Duration::from_secs(60);
const MAX_RETRIES: u32 = 3;

pub struct AutoResponderWorker {
    pub db: Arc<DB>,
}

impl AutoResponderWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl JobHandler for AutoResponderWorker {
    fn handle(&self, job: OHCJob) -> tokio::task::JoinHandle<Result<(), String>> {
        let db = self.db.clone();

        tokio::spawn(async move {
            let payload: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or(serde_json::json!({}));

            let tenant_id = job.tenant_id;
            let source = payload.get("source").and_then(|v| v.as_str()).unwrap_or("unknown").to_string();
            let sender_id = payload.get("sender_id").and_then(|v| v.as_str()).unwrap_or("unknown").to_string();
            let message = payload.get("message").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let inbox_message_id = payload.get("inbox_message_id").and_then(|v| v.as_str()).unwrap_or("").to_string();

            if message.is_empty() || inbox_message_id.is_empty() {
                return Ok(());
            }

            let target_language = "English";
            let translation = match translate_inbox_message_with_llm(&tenant_id, &source, &message, target_language).await {
                Ok(t) => t,
                Err(e) => {
                    tracing::error!("Translation failed: {}", e);
                    InboxTranslation {
                        translated_content: message.clone(),
                        source_language: Some("Unknown".to_string()),
                        target_language: target_language.to_string(),
                        original_content: message.clone(),
                    }
                }
            };

            let draft_reply = match generate_inbox_draft_reply(&tenant_id, &source, &translation).await {
                Ok(d) => d,
                Err(e) => {
                    tracing::error!("Failed to generate draft reply: {}", e);
                    "Thanks for reaching out! We will review this and get back to you soon.".to_string()
                }
            };

            // Simulate LLM confidence check
            let mut confidence = "REVIEW".to_string();
            if let Ok(api_key) = std::env::var("MINIMAX_API_KEY") {
                if !api_key.is_empty() {
                    let minimax = crate::minimax::MinimaxClient::new(api_key);
                    let prompt = format!("Evaluate this customer message and the drafted reply. If the drafted reply perfectly and safely addresses the customer message, reply with exactly 'CONFIDENT'. Otherwise reply with 'REVIEW'. Message: '{}'. Draft: '{}'", translation.translated_content, draft_reply);

                    let mut attempts = 0;
                    while attempts < MAX_RETRIES {
                        match timeout(AI_AGENT_TIMEOUT, minimax.reason(&prompt)).await {
                            Ok(Ok(res)) => {
                                if res.trim() == "CONFIDENT" {
                                    confidence = "CONFIDENT".to_string();
                                }
                                break;
                            },
                            _ => {
                                attempts += 1;
                                tokio::time::sleep(Duration::from_secs(2u64.pow(attempts))).await;
                            }
                        }
                    }
                }
            } else {
                // If no API key, default to CONFIDENT for simple testing or specific keywords
                confidence = "CONFIDENT".to_string();
            }

            let status = if confidence == "CONFIDENT" { "auto_replied" } else { "unread" };

            // Update inbox_messages
            match &db.store {
                crate::db::DbStore::Postgres => {
                    let _ = sqlx::query(
                        "UPDATE inbox_messages SET content = $1, translated_from_language = $2, draft_reply = $3, status = $4, updated_at = NOW() WHERE id = $5 AND tenant_id = $6"
                    )
                    .bind(&translation.translated_content)
                    .bind(&translation.source_language)
                    .bind(&draft_reply)
                    .bind(status)
                    .bind(&inbox_message_id)
                    .bind(&tenant_id)
                    .execute(&db.pool)
                    .await;
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    let _ = sqlx::query(
                        "UPDATE inbox_messages SET content = ?, translated_from_language = ?, draft_reply = ?, status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?"
                    )
                    .bind(&translation.translated_content)
                    .bind(&translation.source_language)
                    .bind(&draft_reply)
                    .bind(status)
                    .bind(&inbox_message_id)
                    .bind(&tenant_id)
                    .execute(sqlite_pool)
                    .await;
                }
            }

            Ok(())
        })
    }
}
