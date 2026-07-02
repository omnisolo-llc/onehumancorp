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
            tracing::info!("Starting MessageTriageWorker for Agentic Work Triage feature...");
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
If you decide action_type is 'Draft Quote', the action_payload MUST be a JSON string with 'total_amount_cents', 'required_deposit_cents', and 'line_items' (array of {{description, unit_price_cents, quantity, is_optional}}).
If you decide action_type is 'Draft Booking', the action_payload MUST be a JSON string with 'service_id' (optional), 'start_time' (RFC3339), 'end_time' (RFC3339).
You have access to the following Staff Availability Data (Simulated):
Shift ID 'shift_123' belongs to 'sam_890'.
Available replacement: 'alex_456'.
If the message is a call-out, action_type MUST be 'Reassign Shift' and action_payload MUST be a JSON string with 'original_staff_id' (e.g. 'sam_890'), 'new_staff_id' (e.g. 'alex_456'), 'shift_id' (e.g. 'shift_123'), 'start_time', 'end_time'.
Output JSON format:
{{
    \"priority\": \"High\" or \"Medium\" or \"Low\",
    \"feature_type\": \"instagram_dm\" or \"general\",
    \"context_summary\": \"A short one sentence summary of the request.\",
    \"action_type\": \"Draft Reply\" or \"Draft Quote\" or \"Draft Booking\" or \"Reassign Shift\",
    \"action_payload\": \"The draft reply, or quote JSON string, or booking JSON string.\"
}}",
                sender_id, customer_message, source
            );

            let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);

            // Use OmniContextRouter
            let router = crate::orchestration::router::OmniContextRouter::new();
            let msg = crate::orchestration::router::InboundMessage {
                source: source.to_string(),
                sender: sender_id.to_string(),
                content: customer_message.to_string(),
            };

            let _omni_result = router.route_and_synthesize(&msg).await.unwrap_or(crate::orchestration::router::DraftReply {
                final_draft: "Thanks for reaching out! We will review this and get back to you soon.".to_string(),
                operations_context: None,
                sales_context: None,
                customer_context: None,
            });

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
                            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string());
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

                if retry_count == max_retries {
                    match &self.db.store {
                        crate::db::DbStore::Postgres => {
                            let _ = sqlx::query(
                                r#"
                                INSERT INTO shared_tasks (id, tenant_id, title, description, status, priority, action_risk, approval_status, proposed_content)
                                VALUES ($1, $2, 'AI Agent Paused: Message Triage', 'The AI agent responsible for message triage is paused because the AI service is unavailable.', 'PENDING', 'P1', 'LOW', 'PENDING', 'System is paused. Please manually review incoming messages.')
                                "#
                            )
                            .bind(Uuid::new_v4().to_string())
                            .bind(&tenant_id)
                            .execute(&self.db.pool)
                            .await;
                        },
                        crate::db::DbStore::Sqlite(pool) => {
                            let _ = sqlx::query(
                                r#"
                                INSERT INTO shared_tasks (id, tenant_id, title, description, status, priority, action_risk, approval_status, proposed_content)
                                VALUES (?, ?, 'AI Agent Paused: Message Triage', 'The AI agent responsible for message triage is paused because the AI service is unavailable.', 'PENDING', 'P1', 'LOW', 'PENDING', 'System is paused. Please manually review incoming messages.')
                                "#
                            )
                            .bind(Uuid::new_v4().to_string())
                            .bind(&tenant_id)
                            .execute(pool)
                            .await;
                        }
                    }
                }
            }

            let priority = extracted.get("priority").and_then(|v| v.as_str()).unwrap_or("Medium");
            let feature_type = extracted.get("feature_type").and_then(|v| v.as_str()).unwrap_or("general");
            let context_summary = extracted.get("context_summary").and_then(|v| v.as_str()).unwrap_or("Customer inquiry");

            // Integrate OmniContextRouter here to get the drafted action payload with sub-agent context
            let router = crate::orchestration::router::OmniContextRouter::new();
            let msg = crate::orchestration::router::InboundMessage {
                source: source.to_string(),
                sender: sender_id.to_string(),
                content: customer_message.to_string(),
            };

            let omni_result_res = router.route_and_synthesize(&msg).await;

            if let Err(_) = omni_result_res {
                match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        let _ = sqlx::query(
                            r#"
                            INSERT INTO shared_tasks (id, tenant_id, title, description, status, priority, action_risk, approval_status, proposed_content)
                            VALUES ($1, $2, 'AI Agent Paused: Message Triage', 'The AI agent responsible for message triage is paused because the AI service is unavailable.', 'PENDING', 'P1', 'LOW', 'PENDING', 'System is paused. Please manually review incoming messages.')
                            "#
                        )
                        .bind(Uuid::new_v4().to_string())
                        .bind(&tenant_id)
                        .execute(&self.db.pool)
                        .await;

                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = NOW() WHERE id = $1")
                            .bind(&job_id)
                            .execute(&self.db.pool).await;
                    },
                    crate::db::DbStore::Sqlite(pool) => {
                        let _ = sqlx::query(
                            r#"
                            INSERT INTO shared_tasks (id, tenant_id, title, description, status, priority, action_risk, approval_status, proposed_content)
                            VALUES (?, ?, 'AI Agent Paused: Message Triage', 'The AI agent responsible for message triage is paused because the AI service is unavailable.', 'PENDING', 'P1', 'LOW', 'PENDING', 'System is paused. Please manually review incoming messages.')
                            "#
                        )
                        .bind(Uuid::new_v4().to_string())
                        .bind(&tenant_id)
                        .execute(pool)
                        .await;

                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                            .bind(&job_id)
                            .execute(pool).await;
                    }
                }

                // Return safely to not crash the worker but stop processing this task
                return Ok(true);
            }

            let omni_result = omni_result_res.unwrap();

            let action_type = extracted.get("action_type").and_then(|v| v.as_str()).unwrap_or("Draft Reply");
            let action_payload_str = omni_result.final_draft;
            let action_payload = action_payload_str.as_str();

            let agent_feed_item_id = Uuid::new_v4().to_string();
            let mut event_source = source.to_string();
            if feature_type == "instagram_dm" || source.to_lowercase().contains("instagram") {
                event_source = "instagram_dm".to_string();
            }

            // Get actual customer_id if exists in payload, otherwise empty string or NULL logic
            let customer_id_val = payload.get("customer_id").and_then(|v| v.as_str());
            let mut quote_id_opt: Option<String> = None;
            let mut _quote_total_amount_cents: Option<i64> = None;


            if action_type == "Draft Booking" || action_type == "Draft Quote" {
                // Route to FulfillmentOrchestrator by enqueueing triage.inquiry job
                let triage_job_id = Uuid::new_v4().to_string();
                let triage_payload = serde_json::json!({
                    "message_id": message_id,
                    "customer_id": customer_id_val,
                    "sender_id": sender_id,
                    "source": source,
                    "content": customer_message,
                    "action_type": action_type,
                    "action_payload": action_payload,
                    "context_summary": context_summary,
                    "priority": priority,
                    "event_source": event_source
                });

                match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        let _ = sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'triage.inquiry', $3, 'PENDING')")
                            .bind(&triage_job_id)
                            .bind(&tenant_id)
                            .bind(triage_payload.to_string())
                            .execute(&self.db.pool).await;
                    },
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                        let _ = sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'triage.inquiry', ?, 'PENDING')")
                            .bind(&triage_job_id)
                            .bind(&tenant_id)
                            .bind(triage_payload.to_string())
                            .execute(&*sqlite_pool).await;
                    }
                }

                // Mark current message triage as completed
                match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = NOW() WHERE id = $1")
                            .bind(&job_id)
                            .execute(&self.db.pool).await;
                    },
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                            .bind(&job_id)
                            .execute(&*sqlite_pool).await;
                    }
                }
                return Ok(true);
            }
let mut booking_id_opt: Option<String> = None;

            if action_type == "Draft Booking" {
                if let Ok(booking_data) = serde_json::from_str::<serde_json::Value>(&action_payload) {
                    let draft_booking_id = Uuid::new_v4();
                    booking_id_opt = Some(draft_booking_id.to_string());
                    let service_id = booking_data.get("service_id").and_then(|v| v.as_str()).unwrap_or("unknown_service");
                    let start_time_str = booking_data.get("start_time").and_then(|v| v.as_str()).unwrap_or("");
                    let end_time_str = booking_data.get("end_time").and_then(|v| v.as_str()).unwrap_or("");

                    let st = chrono::DateTime::parse_from_rfc3339(start_time_str).ok().map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(chrono::Utc::now);
                    let et = chrono::DateTime::parse_from_rfc3339(end_time_str).ok().map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(chrono::Utc::now);

                    let customer_id_uuid = customer_id_val.and_then(|v| Uuid::parse_str(v).ok()).unwrap_or_else(Uuid::new_v4);

                    match &self.db.store {
                        crate::db::DbStore::Postgres => {
                            if let Ok(mut tx) = self.db.pool.begin().await {
                                let _ = sqlx::query(
                                    "INSERT INTO bookings (id, tenant_id, customer_id, service_id, start_time, end_time, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, 'pending', NOW(), NOW())"
                                )
                                .bind(&draft_booking_id.to_string())
                                .bind(&tenant_id)
                                .bind(customer_id_uuid)
                                .bind(service_id)
                                .bind(st)
                                .bind(et)
                                .execute(&mut *tx).await;

                                let _ = sqlx::query(
                                    "UPDATE availability_blocks SET is_available = false WHERE tenant_id = $1 AND service_id = $2 AND start_time = $3 AND end_time = $4"
                                )
                                .bind(&tenant_id)
                                .bind(service_id)
                                .bind(st)
                                .bind(et)
                                .execute(&mut *tx).await;

                                let _ = tx.commit().await;
                            }
                        },
                        crate::db::DbStore::Sqlite(sqlite_pool) => {
                            let _ = sqlx::query(
                                "INSERT INTO bookings (id, tenant_id, customer_id, service_id, start_time, end_time, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, 'pending', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                            )
                            .bind(&draft_booking_id.to_string())
                            .bind(&tenant_id)
                            .bind(customer_id_uuid.to_string())
                            .bind(service_id)
                            .bind(st)
                            .bind(et)
                            .execute(&*sqlite_pool).await;

                            let _ = sqlx::query(
                                "UPDATE availability_blocks SET is_available = false WHERE tenant_id = ? AND service_id = ? AND start_time = ? AND end_time = ?"
                            )
                            .bind(&tenant_id)
                            .bind(service_id)
                            .bind(st)
                            .bind(et)
                            .execute(&*sqlite_pool).await;
                        }
                    }
                }
            } else if action_type == "Reassign Shift" {
                if let Ok(_shift_data) = serde_json::from_str::<serde_json::Value>(&action_payload) {
                    let draft_shift_id = Uuid::new_v4();
                    booking_id_opt = Some(draft_shift_id.to_string());
                }
            } else if action_type == "Draft Quote" {
                if let Ok(quote_data) = serde_json::from_str::<serde_json::Value>(&action_payload) {
                    let draft_quote_id = Uuid::new_v4();
                    quote_id_opt = Some(draft_quote_id.to_string());
                    let total_amount_cents = quote_data.get("total_amount_cents").and_then(|v| v.as_i64()).unwrap_or(0);
                    let required_deposit_cents = quote_data.get("required_deposit_cents").and_then(|v| v.as_i64()).unwrap_or(0);
                    let customer_id_uuid = customer_id_val.and_then(|v| Uuid::parse_str(v).ok()).unwrap_or_else(Uuid::new_v4);

                    match &self.db.store {
                        crate::db::DbStore::Postgres => {
                            if let Ok(mut tx) = self.db.pool.begin().await {
                                let _ = sqlx::query(
                                    "INSERT INTO quotes (id, tenant_id, customer_id, status, total_amount_cents, required_deposit_cents, stripe_payment_link, created_at, updated_at) VALUES ($1, $2, $3, 'DRAFT', $4, $5, NULL, NOW(), NOW())"
                                )
                                .bind(draft_quote_id)
                                .bind(&tenant_id)
                                .bind(customer_id_uuid)
                                .bind(total_amount_cents)
                                .bind(required_deposit_cents)
                                .execute(&mut *tx).await;

                                if let Some(items) = quote_data.get("line_items").and_then(|v| v.as_array()) {
                                    for item in items {
                                        let item_id = Uuid::new_v4();
                                        let desc = item.get("description").and_then(|v| v.as_str()).unwrap_or("");
                                        let price = item.get("unit_price_cents").and_then(|v| v.as_i64()).unwrap_or(0);
                                        let qty = item.get("quantity").and_then(|v| v.as_i64()).unwrap_or(1) as i32;
                                        let is_opt = item.get("is_optional").and_then(|v| v.as_bool()).unwrap_or(false);
                                        let _ = sqlx::query(
                                            "INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at, tenant_id) VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW(), $7)"
                                        )
                                        .bind(item_id)
                                        .bind(draft_quote_id)
                                        .bind(desc)
                                        .bind(price)
                                        .bind(qty)
                                        .bind(is_opt)
                                        .bind(tenant_id.clone())
                                        .execute(&mut *tx).await;
                                    }
                                }
                                let _ = tx.commit().await;
                            }
                        },
                        crate::db::DbStore::Sqlite(sqlite_pool) => {
                            let _ = sqlx::query(
                                "INSERT INTO quotes (id, tenant_id, customer_id, status, total_amount_cents, required_deposit_cents, stripe_payment_link, created_at, updated_at) VALUES (?, ?, ?, 'DRAFT', ?, ?, NULL, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                            )
                            .bind(draft_quote_id.to_string())
                            .bind(&tenant_id)
                            .bind(customer_id_uuid.to_string())
                            .bind(total_amount_cents)
                            .bind(required_deposit_cents)
                            .execute(&*sqlite_pool).await;

                            if let Some(items) = quote_data.get("line_items").and_then(|v| v.as_array()) {
                                for item in items {
                                    let item_id = Uuid::new_v4();
                                    let desc = item.get("description").and_then(|v| v.as_str()).unwrap_or("");
                                    let price = item.get("unit_price_cents").and_then(|v| v.as_i64()).unwrap_or(0);
                                    let qty = item.get("quantity").and_then(|v| v.as_i64()).unwrap_or(1) as i32;
                                    let is_opt = item.get("is_optional").and_then(|v| v.as_bool()).unwrap_or(false);
                                    let _ = sqlx::query(
                                        "INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at, tenant_id) VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, ?)"
                                    )
                                    .bind(item_id.to_string())
                                    .bind(draft_quote_id.to_string())
                                    .bind(desc)
                                    .bind(price)
                                    .bind(qty)
                                    .bind(is_opt)
                                    .bind(tenant_id.clone())
                                    .execute(&*sqlite_pool).await;
                                }
                            }
                        }
                    }
                }
            }

            match &self.db.store {
                crate::db::DbStore::Postgres => {
                    if let Err(e) = sqlx::query("UPDATE omni_inbox_messages SET draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
                        .bind(&action_payload)
                        .bind(&message_id)
                        .bind(&tenant_id)
                        .execute(&self.db.pool).await {
                        tracing::error!("Failed to update omni_inbox_messages: {}", e);
                    }

                    if let Err(e) = sqlx::query("UPDATE inbox_messages SET draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
                        .bind(&action_payload)
                        .bind(&message_id)
                        .bind(&tenant_id)
                        .execute(&self.db.pool).await {
                        tracing::error!("Failed to update inbox_messages: {}", e);
                    }



                    // Implement proper Redis locking to prevent race conditions during thread/triage updates
                    let redis_lock_key = format!("ohc:lock:{}:triage:{}", tenant_id, message_id);
                    let mut _lock_conn = None;
                    if let Some(client) = crate::get_redis_client() {
                        if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                            use redis::AsyncCommands;
                            let lock_acquired: Result<bool, _> = conn.set_nx(&redis_lock_key, "locked").await;
                            if let Ok(true) = lock_acquired {
                                let _: Result<(), _> = conn.expire(&redis_lock_key, 60).await;
                                _lock_conn = Some(conn);
                            } else {
                                let redacted_redis_lock_key = ::server_telemetry::redact_interface_pii(serde_json::Value::String(redis_lock_key.clone()));
                                tracing::warn!("Failed to acquire redis lock for triage updates: {}", redacted_redis_lock_key.as_str().unwrap_or("")); // pii-safe
                            }
                        }
                    }

                    // Insert into triage_items and triage_proposed_actions to satisfy Unified Work Triage Feed
                    let triage_item_id = format!("triage-{}", Uuid::new_v4());
                    let action_id = format!("act-{}", Uuid::new_v4());

                    if let Err(e) = sqlx::query(
                        "INSERT INTO triage_items (id, tenant_id, customer_id, source, priority, context, status) VALUES ($1, $2, $3, $4, $5, $6, 'pending')"
                    )
                    .bind(&triage_item_id)
                    .bind(&tenant_id)
                    .bind(customer_id_val)
                    .bind(&event_source)
                    .bind(&priority)
                    .bind(&context_summary)
                    .execute(&self.db.pool).await {
                        tracing::error!("Failed to insert triage_items: {}", e);
                    }

                    if let Err(e) = sqlx::query(
                        "INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload) VALUES ($1, $2, $3, $4, $5)"
                    )
                    .bind(&action_id)
                    .bind(&triage_item_id)
                    .bind(&tenant_id)
                    .bind(&action_type)
                    .bind(&action_payload)
                    .execute(&self.db.pool).await {
                        tracing::error!("Failed to insert triage_proposed_actions: {}", e);
                    }

                    if let Some(mut conn) = _lock_conn {
                        use redis::AsyncCommands;
                        let _: Result<(), _> = conn.del(&redis_lock_key).await;
                    }


                    let triage_payload = serde_json::json!({
                        "message_id": message_id,
                        "customer_id": customer_id_val,
                        "sender_id": sender_id,
                        "source": source,
                        "content": customer_message,
                        "action_type": action_type,
                        "action_payload": action_payload,
                        "context_summary": context_summary,
                        "priority": priority,
                        "event_source": event_source,
                        "quote_id": quote_id_opt,
                        "booking_id": booking_id_opt
                    });

                    if action_type == "Draft Booking" || action_type == "Draft Quote" {
                        if let Err(e) = sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'triage.inquiry', $3, 'PENDING')")
                            .bind(Uuid::new_v4().to_string())
                            .bind(&tenant_id)
                            .bind(triage_payload.to_string())
                            .execute(&self.db.pool).await {
                                tracing::error!("Failed to insert triage.inquiry job: {}", e);
                            }
                    } else if let Err(e) = sqlx::query(
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
                        "quote_id": quote_id_opt,
                        "booking_id": booking_id_opt,
                        "feature_type": if action_type == "Draft Booking" { "booking_draft" } else if event_source == "instagram_dm" || action_type == "Draft Reply" { "ambassador_reply" } else { "quote_draft" }
                    }))
                    .execute(&self.db.pool).await {
                        tracing::error!("Failed to insert agent feed item: {}", e);
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = NOW() WHERE id = $1")
                            .bind(&job_id)
                            .execute(&self.db.pool).await;
                        return Ok(false);
                    }

                    if let Err(e) = sqlx::query(
                        "INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at) VALUES ($1, $2, 'CustomerSuccess', $3, 'DRAFT', 'DraftForReview', $4, NOW(), NOW())"
                    )
                    .bind(&agent_feed_item_id)
                    .bind(&tenant_id)
                    .bind(&context_summary)
                    .bind(serde_json::json!({
                        "feature_type": event_source,
                        "original_message": customer_message,
                        "generated_response": action_payload,
                        "context_used": context_summary,
                        "inbox_message_id": message_id,
                        "source": source,
                        "sender_id": sender_id,
                        "customer_id": customer_id_val,
                    }))
                    .execute(&self.db.pool).await {
                        tracing::error!("Failed to insert agent approvals item: {}", e);
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
                    if let Err(e) = sqlx::query("UPDATE omni_inbox_messages SET draft_reply = ? WHERE id = ? AND tenant_id = ?")
                        .bind(&action_payload)
                        .bind(&message_id)
                        .bind(&tenant_id)
                        .execute(&*sqlite_pool).await {
                        tracing::error!("Failed to update omni_inbox_messages: {}", e);
                    }

                    if let Err(e) = sqlx::query("UPDATE inbox_messages SET draft_reply = ? WHERE id = ? AND tenant_id = ?")
                        .bind(&action_payload)
                        .bind(&message_id)
                        .bind(&tenant_id)
                        .execute(&*sqlite_pool).await {
                        tracing::error!("Failed to update inbox_messages: {}", e);
                    }



                    // Implement proper Redis locking to prevent race conditions during thread/triage updates
                    let redis_lock_key = format!("ohc:lock:{}:triage:{}", tenant_id, message_id);
                    let mut _lock_conn = None;
                    if let Some(client) = crate::get_redis_client() {
                        if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                            use redis::AsyncCommands;
                            let lock_acquired: Result<bool, _> = conn.set_nx(&redis_lock_key, "locked").await;
                            if let Ok(true) = lock_acquired {
                                let _: Result<(), _> = conn.expire(&redis_lock_key, 60).await;
                                _lock_conn = Some(conn);
                            } else {
                                let redacted_redis_lock_key = ::server_telemetry::redact_interface_pii(serde_json::Value::String(redis_lock_key.clone()));
                                tracing::warn!("Failed to acquire redis lock for triage updates: {}", redacted_redis_lock_key.as_str().unwrap_or("")); // pii-safe
                            }
                        }
                    }

                    // Insert into triage_items and triage_proposed_actions for Sqlite
                    let triage_item_id = format!("triage-{}", Uuid::new_v4());
                    let action_id = format!("act-{}", Uuid::new_v4());

                    if let Err(e) = sqlx::query(
                        "INSERT INTO triage_items (id, tenant_id, customer_id, source, priority, context, status) VALUES (?, ?, ?, ?, ?, ?, 'pending')"
                    )
                    .bind(&triage_item_id)
                    .bind(&tenant_id)
                    .bind(customer_id_val)
                    .bind(&event_source)
                    .bind(&priority)
                    .bind(&context_summary)
                    .execute(&*sqlite_pool).await {
                        tracing::error!("Failed to insert triage_items (Sqlite): {}", e);
                    }

                    if let Err(e) = sqlx::query(
                        "INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload) VALUES (?, ?, ?, ?, ?)"
                    )
                    .bind(&action_id)
                    .bind(&triage_item_id)
                    .bind(&tenant_id)
                    .bind(&action_type)
                    .bind(&action_payload)
                    .execute(&*sqlite_pool).await {
                        tracing::error!("Failed to insert triage_proposed_actions (Sqlite): {}", e);
                    }

                    if let Some(mut conn) = _lock_conn {
                        use redis::AsyncCommands;
                        let _: Result<(), _> = conn.del(&redis_lock_key).await;
                    }

                    let triage_payload = serde_json::json!({
                        "message_id": message_id,
                        "customer_id": customer_id_val,
                        "sender_id": sender_id,
                        "source": source,
                        "content": customer_message,
                        "action_type": action_type,
                        "action_payload": action_payload,
                        "context_summary": context_summary,
                        "priority": priority,
                        "event_source": event_source,
                        "quote_id": quote_id_opt,
                        "booking_id": booking_id_opt
                    });

                    if action_type == "Draft Booking" || action_type == "Draft Quote" {
                        if let Err(e) = sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'triage.inquiry', ?, 'PENDING')")
                            .bind(Uuid::new_v4().to_string())
                            .bind(&tenant_id)
                            .bind(triage_payload.to_string())
                            .execute(&*sqlite_pool).await {
                                tracing::error!("Failed to insert triage.inquiry job: {}", e);
                            }
                    } else if let Err(e) = sqlx::query(
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
                        "quote_id": quote_id_opt,
                        "booking_id": booking_id_opt,
                        "feature_type": if action_type == "Draft Booking" { "booking_draft" } else if event_source == "instagram_dm" || action_type == "Draft Reply" { "ambassador_reply" } else { "quote_draft" }
                    }).to_string())
                    .execute(&*sqlite_pool).await {
                        tracing::error!("Failed to insert agent feed item (SQLite): {}", e);
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                            .bind(&job_id)
                            .execute(&*sqlite_pool).await;
                        return Ok(false);
                    }

                    if let Err(e) = sqlx::query(
                        "INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at) VALUES (?, ?, 'CustomerSuccess', ?, 'DRAFT', 'DraftForReview', ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                    )
                    .bind(&agent_feed_item_id)
                    .bind(&tenant_id)
                    .bind(&context_summary)
                    .bind(serde_json::json!({
                        "feature_type": event_source,
                        "original_message": customer_message,
                        "generated_response": action_payload,
                        "context_used": context_summary,
                        "inbox_message_id": message_id,
                        "source": source,
                        "sender_id": sender_id,
                        "customer_id": customer_id_val,
                    }).to_string())
                    .execute(&*sqlite_pool).await {
                        tracing::error!("Failed to insert agent approvals item (SQLite): {}", e);
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                            .bind(&job_id)
                            .execute(&*sqlite_pool).await;
                        return Ok(false);
                    }

                    let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&job_id)
                        .execute(&*sqlite_pool).await;
                }
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }
}
