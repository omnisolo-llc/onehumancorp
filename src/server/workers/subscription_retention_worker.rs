use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;

use uuid::Uuid;
use sqlx::Row;
use serde_json::json;
use tokio::time::timeout;

const DB_OP_TIMEOUT: Duration = Duration::from_secs(2);

pub struct SubscriptionRetentionWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl SubscriptionRetentionWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(10), // Run frequently
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            let pool = db.pool.clone();
            loop {
                tokio::time::sleep(interval_duration).await;

                let poll_op = async {
                    match &db.store {
                        crate::db::DbStore::Postgres => {
                            let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
                            let row = sqlx::query(
                                r#"
                                SELECT id, tenant_id, payload FROM ohc_job_queue
                                WHERE status = 'PENDING' AND job_type = 'subscription_retention_check'
                                AND next_retry_at <= CURRENT_TIMESTAMP
                                ORDER BY created_at ASC
                                FOR UPDATE SKIP LOCKED
                                LIMIT 1
                                "#
                            )
                            .fetch_optional(&mut *tx)
                            .await
                            .map_err(|e| e.to_string())?;

                            if let Some(r) = row {
                                let id: String = r.get("id");
                                let tenant_id: String = r.get("tenant_id");
                                let payload: serde_json::Value = serde_json::from_str(r.get("payload")).unwrap_or(json!({}));

                                sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                                    .bind(&id)
                                    .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                                tx.commit().await.map_err(|e| e.to_string())?;
                                Ok::<_, String>(Some((id, tenant_id, payload)))
                            } else {
                                tx.rollback().await.map_err(|e| e.to_string())?;
                                Ok::<_, String>(None)
                            }
                        },
                        crate::db::DbStore::Sqlite(sqlite_pool) => {
                            let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;
                            let row = sqlx::query(
                                r#"
                                SELECT id, tenant_id, payload FROM ohc_job_queue
                                WHERE status = 'PENDING' AND job_type = 'subscription_retention_check'
                                AND next_retry_at <= CURRENT_TIMESTAMP
                                ORDER BY created_at ASC
                                LIMIT 1
                                "#
                            )
                            .fetch_optional(&mut *tx)
                            .await
                            .map_err(|e| e.to_string())?;

                            if let Some(r) = row {
                                let id: String = r.get("id");
                                let tenant_id: String = r.get("tenant_id");
                                let payload_str: String = r.get("payload");
                                let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or(json!({}));

                                sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                                    .bind(&id)
                                    .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                                tx.commit().await.map_err(|e| e.to_string())?;
                                Ok::<_, String>(Some((id, tenant_id, payload)))
                            } else {
                                tx.rollback().await.map_err(|e| e.to_string())?;
                                Ok::<_, String>(None)
                            }
                        }
                    }
                };

                let task = match timeout(DB_OP_TIMEOUT, poll_op).await {
                    Ok(Ok(Some(res))) => res,
                    Ok(Ok(None)) => continue,
                    _ => continue,
                };

                let (job_id, tenant_id, payload) = task;
                let customer_id = payload.get("customer_id").and_then(|c| c.as_str()).unwrap_or("");

                // Double check if customer is still at risk
                let is_at_risk = match &db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query_scalar::<_, Option<bool>>(
                            r#"
                            WITH customer_activity AS (
                                SELECT customer_id, MAX(created_at) as last_activity
                                FROM orders
                                WHERE tenant_id = $1 AND customer_id = $2
                                GROUP BY customer_id
                                UNION ALL
                                SELECT customer_id, MAX(start_time) as last_activity
                                FROM bookings
                                WHERE tenant_id = $1 AND customer_id = $2
                                GROUP BY customer_id
                            ),
                            max_activity AS (
                                SELECT customer_id, MAX(last_activity) as last_active_at
                                FROM customer_activity
                                GROUP BY customer_id
                            )
                            SELECT (ma.last_active_at IS NULL OR ma.last_active_at < CURRENT_TIMESTAMP - INTERVAL '21 days')
                            FROM subscriptions s
                            LEFT JOIN max_activity ma ON s.customer_id = ma.customer_id
                            WHERE s.tenant_id = $1 AND s.customer_id = $2 AND s.status = 'active'
                            "#
                        )
                        .bind(&tenant_id)
                        .bind(&customer_id)
                        .fetch_one(&pool)
                        .await
                        .unwrap_or(Some(false))
                        .unwrap_or(false)
                    },
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                         sqlx::query_scalar::<_, Option<bool>>(
                            r#"
                            WITH customer_activity AS (
                                SELECT customer_id, MAX(created_at) as last_activity
                                FROM orders
                                WHERE tenant_id = ? AND customer_id = ?
                                GROUP BY customer_id
                                UNION ALL
                                SELECT customer_id, MAX(start_time) as last_activity
                                FROM bookings
                                WHERE tenant_id = ? AND customer_id = ?
                                GROUP BY customer_id
                            ),
                            max_activity AS (
                                SELECT customer_id, MAX(last_activity) as last_active_at
                                FROM customer_activity
                                GROUP BY customer_id
                            )
                            SELECT (ma.last_active_at IS NULL OR ma.last_active_at < datetime('now', '-21 days'))
                            FROM subscriptions s
                            LEFT JOIN max_activity ma ON s.customer_id = ma.customer_id
                            WHERE s.tenant_id = ? AND s.customer_id = ? AND s.status = 'active'
                            "#
                        )
                        .bind(&tenant_id)
                        .bind(&customer_id)
                        .bind(&tenant_id)
                        .bind(&customer_id)
                        .bind(&tenant_id)
                        .bind(&customer_id)
                        .fetch_one(sqlite_pool)
                        .await
                        .unwrap_or(Some(false))
                        .unwrap_or(false)
                    }
                };

                if is_at_risk {
                    let customer_name = match &db.store {
                        crate::db::DbStore::Postgres => {
                            sqlx::query_scalar::<_, String>("SELECT name FROM customers WHERE id = $1 AND tenant_id = $2")
                            .bind(&customer_id).bind(&tenant_id)
                            .fetch_optional(&pool).await.unwrap_or(None).unwrap_or("Valued Customer".to_string())
                        },
                        crate::db::DbStore::Sqlite(sqlite_pool) => {
                             sqlx::query_scalar::<_, String>("SELECT name FROM customers WHERE id = ? AND tenant_id = ?")
                            .bind(&customer_id).bind(&tenant_id)
                            .fetch_optional(sqlite_pool).await.unwrap_or(None).unwrap_or("Valued Customer".to_string())
                        }
                    };

                    let owner_name = match &db.store {
                        crate::db::DbStore::Postgres => {
                            sqlx::query_scalar::<_, String>("SELECT business_name FROM tenants WHERE id = $1")
                            .bind(&tenant_id)
                            .fetch_optional(&pool).await.unwrap_or(None).unwrap_or("Our Team".to_string())
                        },
                        crate::db::DbStore::Sqlite(sqlite_pool) => {
                             sqlx::query_scalar::<_, String>("SELECT business_name FROM tenants WHERE id = ?")
                            .bind(&tenant_id)
                            .fetch_optional(sqlite_pool).await.unwrap_or(None).unwrap_or("Our Team".to_string())
                        }
                    };

                    let context_payload = json!({
                        "feature_type": "subscription_retention",
                        "customer_name": customer_name,
                        "description": "Health Score dropped due to no bookings in 21 days."
                    });

                    match &db.store {
                        crate::db::DbStore::Postgres => {
                            let ai_prompt = format!("Generate a friendly, context-aware win-back SMS message for a customer named {} who hasn't ordered or booked in a while. Suggest they book a new slot with {} and offer them 10% off their next package to keep the momentum going. Only output the message text.", customer_name, owner_name);
                             let ai_prompt_reduced = crate::pricing::compression::reduce_tokens(&ai_prompt);
                             let drafted_message = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
                                 Ok("gemini") => crate::minimax::LocalLLMClient::new().reason(&ai_prompt_reduced).await.unwrap_or_else(|_| "Hi! It's been a while, would you like to book a new session?".to_string()),
                                 Ok("minimax") => {
                                     let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                                     crate::minimax::MinimaxClient::new(api_key).reason(&ai_prompt_reduced).await.unwrap_or_else(|_| "Hi! It's been a while, would you like to book a new session?".to_string())
                                 }
                                 _ => crate::minimax::LocalLLMClient::new().reason(&ai_prompt_reduced).await.unwrap_or_else(|_| "Hi! It's been a while, would you like to book a new session?".to_string()),
                             };
                            let proposed_action = json!({
                                 "action_type": "send_message",
                                 "draft_reply": drafted_message,
                                 "message": drafted_message
                             });
                             let feed_id = Uuid::new_v4().to_string();
                             let _ = sqlx::query(
                                 r#"
                                 INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
                                 VALUES ($1, $2, 'Customer Success', $3, $4, 'PENDING_APPROVAL', NOW(), NOW())
                                 "#
                             )
                             .bind(&feed_id)
                             .bind(&tenant_id)
                             .bind(sqlx::types::Json(context_payload))
                             .bind(sqlx::types::Json(proposed_action))
                             .execute(&db.pool)
                             .await;

                             // Invalidate cache
                             let cache = crate::api::agent_feed::get_agent_feed_cache();
                             let tag = format!("agent_feed_tenant:{}", tenant_id);
                             cache.invalidate_by_tag(&tag).await;
                        },
                        crate::db::DbStore::Sqlite(sqlite_pool) => {
                             let ai_prompt = format!("Generate a friendly, context-aware win-back SMS message for a customer named {} who hasn't ordered or booked in a while. Suggest they book a new slot with {} and offer them 10% off their next package to keep the momentum going. Only output the message text.", customer_name, owner_name);
                             let ai_prompt_reduced = crate::pricing::compression::reduce_tokens(&ai_prompt);
                             let drafted_message = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
                                 Ok("gemini") => crate::minimax::LocalLLMClient::new().reason(&ai_prompt_reduced).await.unwrap_or_else(|_| "Hi! It's been a while, would you like to book a new session?".to_string()),
                                 Ok("minimax") => {
                                     let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                                     crate::minimax::MinimaxClient::new(api_key).reason(&ai_prompt_reduced).await.unwrap_or_else(|_| "Hi! It's been a while, would you like to book a new session?".to_string())
                                 }
                                 _ => crate::minimax::LocalLLMClient::new().reason(&ai_prompt_reduced).await.unwrap_or_else(|_| "Hi! It's been a while, would you like to book a new session?".to_string()),
                             };

                             let proposed_action = json!({
                                 "action_type": "send_message",
                                 "draft_reply": drafted_message,
                                 "message": drafted_message
                             });

                             let _ = sqlx::query(
                                 r#"
                                 INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
                                 VALUES (?, ?, 'Customer Success', ?, ?, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
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

                 match &db.store {
                     crate::db::DbStore::Postgres => {
                          let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                          .bind(&job_id).execute(&pool).await;
                     },
                     crate::db::DbStore::Sqlite(sqlite_pool) => {
                           let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                          .bind(&job_id).execute(sqlite_pool).await;
                     }
                 }
            }
        });
    }
}
