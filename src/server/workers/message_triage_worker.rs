use std::sync::Arc;
use tokio::time::Duration;
use crate::db::DB;
use sqlx::Row;
use uuid::Uuid;

pub struct MessageTriageWorker {
    db: Arc<DB>,
}

impl MessageTriageWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
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
                        tracing::error!("MessageTriageWorker error: {}", e);
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
                let row = sqlx::query(
                    r#"
                    SELECT id, tenant_id, payload
                    FROM ohc_job_queue
                    WHERE status = 'PENDING' AND next_retry_at <= NOW() AND job_type = 'message_triage'
                    ORDER BY next_retry_at ASC, created_at ASC
                    FOR UPDATE SKIP LOCKED
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let res = if let Some(r) = row {
                    let id: String = r.get("id");
                    let tenant_id: String = r.get("tenant_id");
                    let payload_str: String = r.get("payload");
                    let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));

                    sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = NOW() WHERE id = $1")
                        .bind(&id)
                        .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                    Some((id, tenant_id, payload))
                } else {
                    None
                };
                tx.commit().await.map_err(|e| e.to_string())?;
                res
            },
            crate::db::DbStore::Sqlite(pool) => {
                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    SELECT id, tenant_id, payload
                    FROM ohc_job_queue
                    WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP AND job_type = 'message_triage'
                    ORDER BY next_retry_at ASC, created_at ASC
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let res = if let Some(r) = row {
                    let id: String = r.get("id");
                    let tenant_id: String = r.get("tenant_id");
                    let payload_str: String = r.get("payload");
                    let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));

                    sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&id)
                        .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                    Some((id, tenant_id, payload))
                } else {
                    None
                };
                tx.commit().await.map_err(|e| e.to_string())?;
                res
            }
        };

        if let Some((job_id, tenant_id, payload)) = job {
            let message_id = payload.get("message_id").and_then(|v| v.as_str()).unwrap_or("");
            let source = payload.get("source").and_then(|v| v.as_str()).unwrap_or("unknown");
            let customer_message = payload.get("content").and_then(|v| v.as_str()).unwrap_or("");
            let sender_id = payload.get("sender_id").and_then(|v| v.as_str()).unwrap_or("unknown");

            // Extract intent & context using LLM
            let prompt = format!(
                "You are an AI order and task triage assistant for a business.
Analyze the following incoming customer message.
Message from {}: '{}'
Source: {}

Please extract the context, priority, and draft a response.
Output JSON format:
{{
    \"priority\": \"High\" or \"Medium\" or \"Low\",
    \"context_summary\": \"A short one sentence summary of the request.\",
    \"draft_reply\": \"The draft reply to the user.\"
}}",
                sender_id, customer_message, source
            );

            let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);

            let mut extracted = serde_json::json!({
                "priority": "Medium",
                "context_summary": "Customer inquiry",
                "draft_reply": "Thanks for reaching out! We will review this and get back to you soon."
            });

            let llm_call = async {
                match std::env::var("OHC_INBOX_DRAFT_LLM_PROVIDER")
                    .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
                    .as_deref()
                {
                    Ok("minimax") => {
                        let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                        if !api_key.is_empty() {
                            crate::minimax::MinimaxClient::new(api_key).reason(&compressed_prompt).await
                        } else {
                            crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await
                        }
                    }
                    _ => crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await,
                }
            };

            if let Ok(Ok(reply)) = tokio::time::timeout(Duration::from_secs(60), llm_call).await {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&reply) {
                    if parsed.is_object() {
                        extracted = parsed;
                    }
                }
            }

            let priority = extracted.get("priority").and_then(|v| v.as_str()).unwrap_or("Medium");
            let context_summary = extracted.get("context_summary").and_then(|v| v.as_str()).unwrap_or("Customer inquiry");
            let draft_reply = extracted.get("draft_reply").and_then(|v| v.as_str()).unwrap_or("Thanks for reaching out! We will review this and get back to you soon.");

            let triage_item_id = Uuid::new_v4().to_string();
            let action_id = Uuid::new_v4().to_string();

            // Get actual customer_id if exists in payload, otherwise empty string or NULL logic
            let customer_id_val = payload.get("customer_id").and_then(|v| v.as_str());

            match &self.db.store {
                crate::db::DbStore::Postgres => {
                    let _ = sqlx::query("UPDATE inbox_messages SET draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
                        .bind(&draft_reply)
                        .bind(&message_id)
                        .bind(&tenant_id)
                        .execute(&self.db.pool).await;

                    if let Err(e) = sqlx::query(
                        "INSERT INTO triage_items (id, tenant_id, customer_id, source, priority, context, status) VALUES ($1, $2, $3, $4, $5, $6, 'pending')"
                    )
                    .bind(&triage_item_id)
                    .bind(&tenant_id)
                    .bind(customer_id_val)
                    .bind(&source)
                    .bind(&priority)
                    .bind(&context_summary)
                    .execute(&self.db.pool).await {
                        tracing::error!("Failed to insert triage item: {}", e);
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = NOW() WHERE id = $1")
                            .bind(&job_id)
                            .execute(&self.db.pool).await;
                        return Ok(false);
                    }

                    if let Err(e) = sqlx::query(
                        "INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload) VALUES ($1, $2, $3, $4, $5)"
                    )
                    .bind(&action_id)
                    .bind(&triage_item_id)
                    .bind(&tenant_id)
                    .bind("Draft Reply")
                    .bind(&draft_reply)
                    .execute(&self.db.pool).await {
                        tracing::error!("Failed to insert triage action: {}", e);
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = NOW() WHERE id = $1")
                            .bind(&job_id)
                            .execute(&self.db.pool).await;
                        return Ok(false);
                    }

                    let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = NOW() WHERE id = $1")
                        .bind(&job_id)
                        .execute(&self.db.pool).await;
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    let _ = sqlx::query("UPDATE inbox_messages SET draft_reply = ? WHERE id = ? AND tenant_id = ?")
                        .bind(&draft_reply)
                        .bind(&message_id)
                        .bind(&tenant_id)
                        .execute(sqlite_pool).await;

                    if let Err(e) = sqlx::query(
                        "INSERT INTO triage_items (id, tenant_id, customer_id, source, priority, context, status) VALUES (?, ?, ?, ?, ?, ?, 'pending')"
                    )
                    .bind(&triage_item_id)
                    .bind(&tenant_id)
                    .bind(customer_id_val)
                    .bind(&source)
                    .bind(&priority)
                    .bind(&context_summary)
                    .execute(sqlite_pool).await {
                        tracing::error!("Failed to insert triage item (SQLite): {}", e);
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                            .bind(&job_id)
                            .execute(sqlite_pool).await;
                        return Ok(false);
                    }

                    if let Err(e) = sqlx::query(
                        "INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload) VALUES (?, ?, ?, ?, ?)"
                    )
                    .bind(&action_id)
                    .bind(&triage_item_id)
                    .bind(&tenant_id)
                    .bind("Draft Reply")
                    .bind(&draft_reply)
                    .execute(sqlite_pool).await {
                        tracing::error!("Failed to insert triage action (SQLite): {}", e);
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                            .bind(&job_id)
                            .execute(sqlite_pool).await;
                        return Ok(false);
                    }

                    let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&job_id)
                        .execute(sqlite_pool).await;
                }
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }
}
