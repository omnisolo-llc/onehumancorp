use std::sync::Arc;
use tokio::time::Duration;
use crate::db::DB;
use sqlx::Row;
use uuid::Uuid;

pub struct MissedLeadRecoveryWorker {
    db: Arc<DB>,
}

impl MissedLeadRecoveryWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            loop {
                match self.poll().await {
                    Ok(true) => continue,
                    Ok(false) => tokio::time::sleep(Duration::from_secs(60)).await,
                    Err(e) => {
                        tracing::error!("MissedLeadRecoveryWorker error: {}", e);
                        tokio::time::sleep(Duration::from_secs(60)).await;
                    }
                }
            }
        });
    }

    pub async fn poll(&self) -> Result<bool, String> {
        let row_data = match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query(
                    r#"
                    SELECT id, tenant_id, source, original_content, customer_id
                    FROM omni_inbox_messages
                    WHERE status = 'unread'
                      AND created_at < NOW() - INTERVAL '2 hours'
                      AND (draft_reply IS NULL OR draft_reply = '')
                    LIMIT 1
                    "#
                )
                .fetch_optional(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?
                .map(|r| {
                    (
                        r.get::<String, _>("id"),
                        r.get::<String, _>("tenant_id"),
                        r.get::<String, _>("source"),
                        r.get::<String, _>("original_content"),
                        r.try_get::<String, _>("customer_id").ok(),
                        r.get::<String, _>("tone")
                    )
                })
            },
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query(
                    r#"
                    SELECT m.id, m.tenant_id, m.source, m.original_content, m.customer_id, COALESCE(p.tone_instructions, '') as tone
                    FROM omni_inbox_messages m
                    LEFT JOIN auto_reply_policies p ON m.tenant_id = p.tenant_id
                    WHERE m.status = 'unread'
                      AND m.created_at < datetime('now', '-' || COALESCE(p.delay_minutes, 5) || ' minutes')
                      AND (p.enabled IS NULL OR p.enabled = 1)
                      AND (m.draft_reply IS NULL OR m.draft_reply = '')
                    LIMIT 1
                    "#
                )
                .fetch_optional(pool)
                .await
                .map_err(|e| e.to_string())?
                .map(|r| {
                    (
                        r.get::<String, _>("id"),
                        r.get::<String, _>("tenant_id"),
                        r.get::<String, _>("source"),
                        r.get::<String, _>("original_content"),
                        r.try_get::<String, _>("customer_id").ok(),
                        r.get::<String, _>("tone")
                    )
                })
            }
        };

        if let Some((message_id, tenant_id, source, original_content, customer_id, tone)) = row_data {
            let customer_name = if let Some(cid) = &customer_id {
                match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query_scalar::<_, String>("SELECT name FROM customers WHERE id = $1 AND tenant_id = $2")
                            .bind(cid)
                            .bind(&tenant_id)
                            .fetch_optional(&self.db.pool)
                            .await
                            .unwrap_or_default()
                            .unwrap_or_else(|| "Customer".to_string())
                    },
                    crate::db::DbStore::Sqlite(pool) => {
                        sqlx::query_scalar::<_, String>("SELECT name FROM customers WHERE id = ? AND tenant_id = ?")
                            .bind(cid)
                            .bind(&tenant_id)
                            .fetch_optional(pool)
                            .await
                            .unwrap_or_default()
                            .unwrap_or_else(|| "Customer".to_string())
                    }
                }
            } else {
                "Customer".to_string()
            };

            #[derive(serde::Deserialize, Default)]
            struct IntentResponse {
                is_safe: bool,
                draft_reply: String,
            }

            let prompt = format!(
                "You are an AI assistant acting as the Customer Relationship Agent for a business. \
                A customer ({}) sent an inquiry via {} a while ago and we haven't replied. \
                Their message: '{}'
\
                Instructions from the owner: '{}'
\
                First, determine if this is a safe, new inquiry (e.g. asking for a quote, booking, or service) \
                versus an angry complaint, spam, or a message that requires human intervention. \
                If it is safe, draft a friendly, short follow-up message to apologize for the delay, ask for necessary details (like address or photo if relevant), and mention we can get a quote/booking sorted. \
                Reply strictly in JSON format with two keys: `is_safe` (boolean) and `draft_reply` (string).",
                customer_name, source, original_content, tone
            );

            let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);

            let mut follow_up_msg = format!(
                "Hi {}, sorry for the delay! We're currently reviewing your request and will get back to you shortly. Did you still need help?",
                customer_name
            );
            let mut is_safe = true;

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
                        if !reply.trim().is_empty() {
                            // Extract JSON block if surrounded by markdown
                            let json_str = reply.trim();
                            let start_idx = json_str.find('{').unwrap_or(0);
                            let end_idx = json_str.rfind('}').unwrap_or(json_str.len() - 1);

                            if start_idx <= end_idx {
                                let clean_json = &json_str[start_idx..=end_idx];
                                if let Ok(parsed) = serde_json::from_str::<IntentResponse>(clean_json) {
                                    is_safe = parsed.is_safe;
                                    follow_up_msg = parsed.draft_reply;
                                    break;
                                }
                            }
                        }
                        retry_count += 1;
                    }
                    _ => {
                        retry_count += 1;
                    }
                }
            }

            if !is_safe {
                // If it's not safe, mark it as skipped so we don't keep polling
                match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        let _ = sqlx::query("UPDATE omni_inbox_messages SET status = 'skipped_auto_reply' WHERE id = $1 AND tenant_id = $2")
                            .bind(&message_id)
                            .bind(&tenant_id)
                            .execute(&self.db.pool)
                            .await;
                    },
                    crate::db::DbStore::Sqlite(pool) => {
                        let _ = sqlx::query("UPDATE omni_inbox_messages SET status = 'skipped_auto_reply' WHERE id = ? AND tenant_id = ?")
                            .bind(&message_id)
                            .bind(&tenant_id)
                            .execute(pool)
                            .await;
                    }
                }
                return Ok(true);
            }

            let agent_feed_item_id = Uuid::new_v4().to_string();
            let daily_work_item_id = Uuid::new_v4().to_string();

            let context_payload = serde_json::json!({
                "description": format!("The customer ({}) hasn't received a reply for the configured threshold.", customer_name)
            });

            let proposed_action = serde_json::json!({
                "action_type": "Draft Follow-up",
                "message": format!("The Assistant recovered 1 missed lead today, securing potential revenue. The Salesperson sent a recovery message for {}.", customer_name),
                "draft_reply": follow_up_msg.clone(),
                "message_id": message_id
            });

            let customer_info = serde_json::json!({
                "name": customer_name,
                "message": original_content
            });

            let suggested_actions = serde_json::json!({
                "draft_reply": follow_up_msg.clone()
            });

            match &self.db.store {
                crate::db::DbStore::Postgres => {
                    let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                    sqlx::query(
                        "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 'APPROVED', NOW(), NOW())"
                    )
                    .bind(&agent_feed_item_id)
                    .bind(&tenant_id)
                    .bind("Lead Recovery Agent")
                    .bind(context_payload.to_string())
                    .bind(proposed_action.to_string())
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    sqlx::query(
                        "INSERT INTO daily_work_items (id, tenant_id, intent, customer_info, suggested_actions, status, created_at, updated_at) VALUES ($1, $2, 'missed_lead_recovery', $3, $4, 'PENDING', NOW(), NOW())"
                    )
                    .bind(&daily_work_item_id)
                    .bind(&tenant_id)
                    .bind(sqlx::types::Json(&customer_info))
                    .bind(sqlx::types::Json(&suggested_actions))
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    sqlx::query("UPDATE omni_inbox_messages SET status = 'auto_replied', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
                        .bind(&follow_up_msg)
                        .bind(&message_id)
                        .bind(&tenant_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;
                },
                crate::db::DbStore::Sqlite(pool) => {
                    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

                    sqlx::query(
                        "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 'APPROVED', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                    )
                    .bind(&agent_feed_item_id)
                    .bind(&tenant_id)
                    .bind("Lead Recovery Agent")
                    .bind(context_payload.to_string())
                    .bind(proposed_action.to_string())
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    sqlx::query(
                        "INSERT INTO daily_work_items (id, tenant_id, intent, customer_info, suggested_actions, status, created_at, updated_at) VALUES (?, ?, 'missed_lead_recovery', ?, ?, 'PENDING', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                    )
                    .bind(&daily_work_item_id)
                    .bind(&tenant_id)
                    .bind(customer_info.to_string())
                    .bind(suggested_actions.to_string())
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    sqlx::query("UPDATE omni_inbox_messages SET status = 'auto_replied', draft_reply = ? WHERE id = ? AND tenant_id = ?")
                        .bind(&follow_up_msg)
                        .bind(&message_id)
                        .bind(&tenant_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;
                }
            }

            return Ok(true);
        }

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DbStore;

    async fn setup_test_db() -> Option<Arc<DB>> {
        let sqlite_pool = crate::db::create_sqlite_pool_for_test().await;
        let pool = crate::db::create_dummy_pg_pool().await;
        let db = DB {
            store: DbStore::Sqlite(sqlite_pool.clone()),
            pool,
        };
        crate::db::DB::run_migrations(&db).await.unwrap();
        // create auto_reply_policies table for test since migrations might not run correctly or we just need the table
        if let DbStore::Sqlite(pool) = &db.store {
            sqlx::query("CREATE TABLE IF NOT EXISTS auto_reply_policies (id TEXT PRIMARY KEY, tenant_id TEXT, enabled BOOLEAN, delay_minutes INTEGER, tone_instructions TEXT)")
                .execute(pool).await.unwrap();
            sqlx::query("CREATE TABLE IF NOT EXISTS daily_work_items (id TEXT PRIMARY KEY, tenant_id TEXT, signal_id TEXT, intent TEXT, customer_info TEXT, suggested_actions TEXT, status TEXT, created_at TEXT, updated_at TEXT)")
                .execute(pool).await.unwrap();
        }
        Some(Arc::new(db))
    }

    #[tokio::test]
    async fn test_missed_lead_recovery_worker() {
        let db = setup_test_db().await.unwrap();

        if let DbStore::Sqlite(pool) = &db.store {
            let msg_id = Uuid::new_v4().to_string();
            let tenant_id = "tenant-1".to_string();
            let customer_id = Uuid::new_v4().to_string();

            // Insert customer
            sqlx::query("INSERT INTO customers (id, tenant_id, name) VALUES (?, ?, 'John Doe')")
                .bind(&customer_id)
                .bind(&tenant_id)
                .execute(pool)
                .await
                .unwrap();

            // Insert a stale unread message
            sqlx::query("INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, status, customer_id, created_at) VALUES (?, ?, 'instagram_dm', 'How much for a cake?', 'How much for a cake?', 'English', 'unread', ?, datetime('now', '-6 minutes'))")
                .bind(&msg_id)
                .bind(&tenant_id)
                .bind(&customer_id)
                .execute(pool)
                .await
                .unwrap();

            let worker = MissedLeadRecoveryWorker::new(db.clone());
            let processed = worker.poll().await.unwrap();
            assert!(processed, "Worker should process the stale message");

            // Verify the status was updated to auto_replied and draft_reply is set
            let (status, draft): (String, Option<String>) = sqlx::query_as("SELECT status, draft_reply FROM omni_inbox_messages WHERE id = ?")
                .bind(&msg_id)
                .fetch_one(pool)
                .await
                .unwrap();

            assert_eq!(status, "auto_replied");
            assert!(draft.is_some());
            assert!(!draft.unwrap().is_empty());

            // Verify agent feed item was created
            let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM agent_feed_items WHERE tenant_id = ? AND event_source = 'Lead Recovery Agent'")
                .bind(&tenant_id)
                .fetch_one(pool)
                .await
                .unwrap();
            assert_eq!(count, 1);
        }
    }
}
