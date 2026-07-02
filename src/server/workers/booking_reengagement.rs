use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;

use uuid::Uuid;
use sqlx::Row;
use serde_json::json;

pub struct BookingReengagementWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl BookingReengagementWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(60 * 60 * 24), // Run daily
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;

        let llm = ohc_builtin_agent::llm::openai::OpenAIClientConfig::openai(
            std::env::var("OHC_LLM_API_KEY")
                .or_else(|_| std::env::var("OPENAI_API_KEY"))
                .unwrap_or_default()
        );
        let llm_client = if llm.api_key.is_empty() {
            None
        } else {
            Some(Arc::new(ohc_builtin_agent::llm::openai::OpenAIClient::from_config(llm)))
        };

        tokio::spawn(async move {
            let pool = db.pool.clone();

            // Initial delay for tests to pick it up without hammering the DB immediately
            tokio::time::sleep(Duration::from_secs(5)).await;

            let mut interval = tokio::time::interval(interval_duration);

            // consume first tick which is immediate
            interval.tick().await;

            loop {
                // Attempt to acquire distributed lock
                let lock_key = "ohc:lock:global:booking_reengagement";
                let has_lock = match &db.store {
                    crate::db::DbStore::Postgres => {
                        let result: Result<Option<bool>, _> = sqlx::query_scalar("SELECT pg_try_advisory_lock(hashtext($1))")
                            .bind(lock_key)
                            .fetch_one(&pool).await;
                        result.unwrap_or(Some(false)).unwrap_or(false)
                    },
                    crate::db::DbStore::Sqlite(_) => {
                        // SQLite uses a simple table for locks if needed, or we just rely on single-process
                        true
                    }
                };

                if has_lock {
                // Scan for dormant customers:
                // Historically booked > 1 times, but no bookings in the last 14 days,
                // and no re-engagement feed item pending for them.

                let mut extracted_customers = Vec::new();

                match &db.store {
                    crate::db::DbStore::Postgres => {
                        let rows = sqlx::query(
                            r#"
                            WITH customer_stats AS (
                                SELECT tenant_id, customer_id, COUNT(*) as total_bookings, MAX(start_time) as last_booking
                                FROM bookings
                                GROUP BY tenant_id, customer_id
                            )
                            SELECT cs.tenant_id, cs.customer_id, c.name
                            FROM customer_stats cs
                            JOIN customers c ON cs.customer_id = c.id AND cs.tenant_id = c.tenant_id
                            WHERE cs.total_bookings > 1
                            AND cs.last_booking < CURRENT_TIMESTAMP - INTERVAL '14 days'
                            AND NOT EXISTS (
                                SELECT 1 FROM agent_feed_items afi
                                WHERE afi.tenant_id = cs.tenant_id
                                AND afi.event_source = 'Sales/CS Agent'
                                AND afi.context_payload->>'customer_id' = cs.customer_id::text
                            )
                            "#
                        )
                        .fetch_all(&pool)
                        .await;

                        if let Ok(customers) = rows {
                            for r in customers {
                                let tenant_id: String = r.get("tenant_id");
                                let customer_id: String = match r.try_get::<uuid::Uuid, _>("customer_id") {
                                    Ok(u) => u.to_string(),
                                    Err(_) => r.get::<String, _>("customer_id")
                                };
                                let customer_name: String = r.get("name");
                                extracted_customers.push((tenant_id, customer_id, customer_name));
                            }
                        }
                    },
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                         let rows = sqlx::query(
                            r#"
                            WITH customer_stats AS (
                                SELECT tenant_id, customer_id, COUNT(*) as total_bookings, MAX(start_time) as last_booking
                                FROM bookings
                                GROUP BY tenant_id, customer_id
                            )
                            SELECT cs.tenant_id, cs.customer_id, c.name
                            FROM customer_stats cs
                            JOIN customers c ON cs.customer_id = c.id AND cs.tenant_id = c.tenant_id
                            WHERE cs.total_bookings > 1
                            AND cs.last_booking < datetime('now', '-14 days')
                            AND NOT EXISTS (
                                SELECT 1 FROM agent_feed_items afi
                                WHERE afi.tenant_id = cs.tenant_id
                                AND afi.event_source = 'Sales/CS Agent'
                                AND json_extract(afi.context_payload, '$.customer_id') = cs.customer_id
                            )
                            "#
                        )
                        .fetch_all(sqlite_pool)
                        .await;

                        if let Ok(customers) = rows {
                            for r in customers {
                                let tenant_id: String = r.get("tenant_id");
                                let customer_id: String = match r.try_get::<uuid::Uuid, _>("customer_id") {
                                    Ok(u) => u.to_string(),
                                    Err(_) => r.get::<String, _>("customer_id")
                                };
                                let customer_name: String = r.get("name");
                                extracted_customers.push((tenant_id, customer_id, customer_name));
                            }
                        }
                    }
                };

                for (tenant_id, customer_id, customer_name) in extracted_customers {

                        let mut drafted_message = format!("Hi {}, I noticed we haven't had a session in a while! Hope everything is going great with your progress. Would you like to jump back in this week? I have some slots available. Here is a quick booking link: [Link]", customer_name);

                        // If LLM is available, draft a personalized message
                        if let Some(ref llm) = llm_client {
                            use ohc_builtin_agent::llm::LlmClient;
                            let prompt = format!("Draft a short, friendly SMS message to re-engage a customer named {} who hasn't booked a service in 14 days. Include a placeholder [Link] for booking. Keep it under 2 sentences.", customer_name);
                            let req = ohc_builtin_agent_core::types::ChatRequest {
                                model: "gpt-4o-mini".to_string(),
                                system: "You are a helpful customer success assistant. Reply with only the message text.".to_string(),
                                messages: vec![ohc_builtin_agent_core::types::Message::user(prompt)],
                                tools: vec![],
                                max_tokens: 150,
                                temperature: 0.7,
                            };
                            if let Ok(resp) = llm.chat(req).await {
                                drafted_message = resp.message.content.trim().to_string();
                            }
                        }


                        let context_payload = serde_json::json!({
                            "customer_id": customer_id,
                            "description": format!("AI detected that {} is a returning customer who hasn't booked in 14 days. This follow-up helps maintain momentum.", customer_name)
                        });

                        let proposed_action = serde_json::json!({
                            "action_type": "send_message",
                            "feature_type": "booking_reengagement",
                            "message": format!("Approve Re-engagement for {}", customer_name),
                            "draft_message": drafted_message
                        });

                        match &db.store {
                            crate::db::DbStore::Postgres => {
                                let _ = sqlx::query(
                                    r#"
                                    INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state)
                                    VALUES ($1, $2, 'Sales/CS Agent', $3, $4, 'PENDING_APPROVAL')
                                    "#
                                )
                                .bind(Uuid::new_v4().to_string())
                                .bind(&tenant_id)
                                .bind(context_payload.clone())
                                .bind(proposed_action.clone())
                                .execute(&pool)
                                .await;
                            },
                            crate::db::DbStore::Sqlite(sqlite_pool) => {
                                 let _ = sqlx::query(
                                    r#"
                                    INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state)
                                    VALUES (?, ?, 'Sales/CS Agent', ?, ?, 'PENDING_APPROVAL')
                                    "#
                                )
                                .bind(Uuid::new_v4().to_string())
                                .bind(&tenant_id)
                                .bind(context_payload.to_string())
                                .bind(proposed_action.to_string())
                                .execute(sqlite_pool)
                                .await;
                            }
                        }
                    }

                    if let crate::db::DbStore::Postgres = &db.store {
                        let _: Result<Option<bool>, _> = sqlx::query_scalar("SELECT pg_advisory_unlock(hashtext($1))")
                            .bind("ohc:lock:global:booking_reengagement")
                            .fetch_one(&pool).await;
                    }
                }

                interval.tick().await;
            }
        });
    }
}
