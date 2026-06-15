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

Please extract the context, priority, and decide if the request needs a Quote, a Booking, or a General Reply. Note if the source is Instagram DM, whatsapp or similar, explicitly mention the feature type as instagram_dm.
Output JSON format:
{{
    \"priority\": \"High\" or \"Medium\" or \"Low\",
    \"feature_type\": \"instagram_dm\" or \"general\",
    \"context_summary\": \"A short one sentence summary of the request.\",
    \"action_type\": \"Draft Reply\" or \"Draft Quote\" or \"Draft Booking\",
    \"action_payload\": \"The draft reply, or quote details, or booking details.\"
}}",
                sender_id, customer_message, source
            );

            let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);

            let mut extracted = serde_json::json!({
                "priority": "Medium",
                "feature_type": "general",
                "context_summary": "Customer inquiry",
                "action_type": "Draft Reply",
                "action_payload": "Thanks for reaching out! We will review this and get back to you soon."
            });

            let max_retries = 3;
            let mut retry_count = 0;

            while retry_count < max_retries {
                let compressed_prompt_clone = compressed_prompt.clone();
                let llm_call = async {
                    match std::env::var("OHC_INBOX_DRAFT_LLM_PROVIDER")
                        .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
                        .as_deref()
                    {
                        Ok("minimax") => {
                            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                            if !api_key.is_empty() {
                                crate::minimax::MinimaxClient::new(api_key).reason(&compressed_prompt_clone).await
                            } else {
                                crate::minimax::LocalLLMClient::new().reason(&compressed_prompt_clone).await
                            }
                        }
                        _ => crate::minimax::LocalLLMClient::new().reason(&compressed_prompt_clone).await,
                    }
                };

                match tokio::time::timeout(Duration::from_secs(60), llm_call).await {
                    Ok(Ok(reply)) => {
                        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&reply) {
                            if parsed.is_object() && parsed.get("priority").is_some() && parsed.get("context_summary").is_some() && parsed.get("action_type").is_some() && parsed.get("action_payload").is_some() {
                                extracted = parsed;
                                break;
                            }
                        }
                        retry_count += 1;
                        tracing::warn!("LLM returned invalid format in MessageTriageWorker (attempt {}/{})", retry_count, max_retries);
                        if retry_count < max_retries {
                            tokio::time::sleep(Duration::from_secs(2u64.pow(retry_count as u32))).await;
                        }
                    }
                    Ok(Err(e)) => {
                        retry_count += 1;
                        tracing::warn!("LLM error in MessageTriageWorker (attempt {}/{}): {}", retry_count, max_retries, e);
                        if retry_count < max_retries {
                            tokio::time::sleep(Duration::from_secs(2u64.pow(retry_count as u32))).await;
                        }
                    }
                    Err(_) => {
                        retry_count += 1;
                        tracing::warn!("LLM timeout in MessageTriageWorker (attempt {}/{}): 60s exceeded", retry_count, max_retries);
                        if retry_count < max_retries {
                            tokio::time::sleep(Duration::from_secs(2u64.pow(retry_count as u32))).await;
                        }
                    }
                }
            }

            let priority = extracted.get("priority").and_then(|v| v.as_str()).unwrap_or("Medium");
            let feature_type = extracted.get("feature_type").and_then(|v| v.as_str()).unwrap_or("general");
            let context_summary = extracted.get("context_summary").and_then(|v| v.as_str()).unwrap_or("Customer inquiry");
            let action_type = extracted.get("action_type").and_then(|v| v.as_str()).unwrap_or("Draft Reply");
            let action_payload = extracted.get("action_payload").and_then(|v| v.as_str()).unwrap_or("Thanks for reaching out! We will review this and get back to you soon.");

            let agent_feed_item_id = Uuid::new_v4().to_string();
            let mut event_source = source.to_string();
            if feature_type == "instagram_dm" || source.to_lowercase().contains("instagram") {
                event_source = "instagram_dm".to_string();
            }

            // Get actual customer_id if exists in payload, otherwise empty string or NULL logic
            let customer_id_val = payload.get("customer_id").and_then(|v| v.as_str());

            match &self.db.store {
                crate::db::DbStore::Postgres => {
                    let _ = sqlx::query("UPDATE inbox_messages SET draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
                        .bind(&action_payload)
                        .bind(&message_id)
                        .bind(&tenant_id)
                        .execute(&self.db.pool).await;

                    if let Err(e) = sqlx::query(
                        "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 'PENDING_APPROVAL', NOW(), NOW())"
                    )
                    .bind(&agent_feed_item_id)
                    .bind(&tenant_id)
                    .bind(&event_source)
                    .bind(serde_json::json!({
                        "customer_message": customer_message,
                        "feature_type": event_source,
                        "priority": priority,
                        "context": context_summary,
                        "inbox_message_id": message_id,
                        "customer_id": customer_id_val
                    }))
                    .bind(serde_json::json!({
                        "action_type": action_type,
                        "draft_reply": action_payload,
                        "inbox_message_id": message_id
                    }))
                    .execute(&self.db.pool).await {
                        tracing::error!("Failed to insert agent feed item: {}", e);
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
                        .bind(&action_payload)
                        .bind(&message_id)
                        .bind(&tenant_id)
                        .execute(sqlite_pool).await;

                    if let Err(e) = sqlx::query(
                        "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                    )
                    .bind(&agent_feed_item_id)
                    .bind(&tenant_id)
                    .bind(&event_source)
                    .bind(serde_json::json!({
                        "customer_message": customer_message,
                        "feature_type": event_source,
                        "priority": priority,
                        "context": context_summary,
                        "inbox_message_id": message_id,
                        "customer_id": customer_id_val
                    }).to_string())
                    .bind(serde_json::json!({
                        "action_type": action_type,
                        "draft_reply": action_payload,
                        "inbox_message_id": message_id
                    }).to_string())
                    .execute(sqlite_pool).await {
                        tracing::error!("Failed to insert agent feed item (SQLite): {}", e);
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
