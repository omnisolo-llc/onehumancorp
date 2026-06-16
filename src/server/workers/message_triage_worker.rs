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
                "You are The Ambassador, an AI customer support and triage assistant for a business.
Your goal is to politely, accurately, and concisely respond to customer inquiries across various channels based on the provided context.
Analyze the following incoming customer message.
Message from {}: '{}'
Source: {}

Please extract the context, priority, and decide if the request needs a Quote, a Booking, or a General Reply. Note if the source is Instagram DM, whatsapp or similar, explicitly mention the feature type as instagram_dm.
If you decide action_type is 'Draft Quote', the action_payload MUST be a JSON string with 'total_amount_cents', 'required_deposit_cents', and 'line_items' (array of {{description, unit_price_cents, quantity, is_optional}}).
Output JSON format:
{{
    \"priority\": \"High\" or \"Medium\" or \"Low\",
    \"feature_type\": \"instagram_dm\" or \"general\",
    \"context_summary\": \"A short one sentence summary of the request.\",
    \"action_type\": \"Draft Reply\" or \"Draft Quote\" or \"Draft Booking\",
    \"action_payload\": \"The drafted reply acting as The Ambassador, or quote JSON string, or booking details.\"
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
            let mut quote_id_opt: Option<String> = None;

            if action_type == "Draft Quote" {
                if let Ok(quote_data) = serde_json::from_str::<serde_json::Value>(&action_payload) {
                    let draft_quote_id = Uuid::new_v4();
                    quote_id_opt = Some(draft_quote_id.to_string());
                    let total_amount = quote_data.get("total_amount_cents").and_then(|v| v.as_i64()).unwrap_or(0);
                    let required_deposit = quote_data.get("required_deposit_cents").and_then(|v| v.as_i64()).unwrap_or(0);
                    let customer_id_uuid = customer_id_val.and_then(|v| Uuid::parse_str(v).ok()).unwrap_or_else(Uuid::new_v4);

                    match &self.db.store {
                        crate::db::DbStore::Postgres => {
                            if let Ok(mut tx) = self.db.pool.begin().await {
                                let _ = sqlx::query(
                                    "INSERT INTO quotes (id, tenant_id, customer_id, status, total_amount, required_deposit, checkout_url, created_at, updated_at) VALUES ($1, $2, $3, 'DRAFT', $4, $5, NULL, NOW(), NOW())"
                                )
                                .bind(draft_quote_id)
                                .bind(&tenant_id)
                                .bind(customer_id_uuid)
                                .bind(total_amount)
                                .bind(required_deposit)
                                .execute(&mut *tx).await;

                                if let Some(items) = quote_data.get("line_items").and_then(|v| v.as_array()) {
                                    for item in items {
                                        let item_id = Uuid::new_v4();
                                        let desc = item.get("description").and_then(|v| v.as_str()).unwrap_or("");
                                        let price = item.get("unit_price_cents").and_then(|v| v.as_i64()).unwrap_or(0);
                                        let qty = item.get("quantity").and_then(|v| v.as_i64()).unwrap_or(1) as i32;
                                        let is_opt = item.get("is_optional").and_then(|v| v.as_bool()).unwrap_or(false);
                                        let _ = sqlx::query(
                                            "INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())"
                                        )
                                        .bind(item_id)
                                        .bind(draft_quote_id)
                                        .bind(desc)
                                        .bind(price)
                                        .bind(qty)
                                        .bind(is_opt)
                                        .execute(&mut *tx).await;
                                    }
                                }
                                let _ = tx.commit().await;
                            }
                        },
                        crate::db::DbStore::Sqlite(sqlite_pool) => {
                            let _ = sqlx::query(
                                "INSERT INTO quotes (id, tenant_id, customer_id, status, total_amount, required_deposit, checkout_url, created_at, updated_at) VALUES (?, ?, ?, 'DRAFT', ?, ?, NULL, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                            )
                            .bind(draft_quote_id.to_string())
                            .bind(&tenant_id)
                            .bind(customer_id_uuid.to_string())
                            .bind(total_amount)
                            .bind(required_deposit)
                            .execute(sqlite_pool).await;

                            if let Some(items) = quote_data.get("line_items").and_then(|v| v.as_array()) {
                                for item in items {
                                    let item_id = Uuid::new_v4();
                                    let desc = item.get("description").and_then(|v| v.as_str()).unwrap_or("");
                                    let price = item.get("unit_price_cents").and_then(|v| v.as_i64()).unwrap_or(0);
                                    let qty = item.get("quantity").and_then(|v| v.as_i64()).unwrap_or(1) as i32;
                                    let is_opt = item.get("is_optional").and_then(|v| v.as_bool()).unwrap_or(false);
                                    let _ = sqlx::query(
                                        "INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                                    )
                                    .bind(item_id.to_string())
                                    .bind(draft_quote_id.to_string())
                                    .bind(desc)
                                    .bind(price)
                                    .bind(qty)
                                    .bind(is_opt)
                                    .execute(sqlite_pool).await;
                                }
                            }
                        }
                    }
                }
            }

            match &self.db.store {
                crate::db::DbStore::Postgres => {
                    let _ = sqlx::query("UPDATE omni_inbox_messages SET draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
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
                        "inbox_message_id": message_id,
                        "quote_id": quote_id_opt
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
                    let _ = sqlx::query("UPDATE omni_inbox_messages SET draft_reply = ? WHERE id = ? AND tenant_id = ?")
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
                        "inbox_message_id": message_id,
                        "quote_id": quote_id_opt
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
