use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;

use uuid::Uuid;
use sqlx::Row;
use serde_json::json;
use tokio::time::timeout;

const DB_OP_TIMEOUT: Duration = Duration::from_secs(2);

pub struct SubscriptionChurnWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl SubscriptionChurnWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(15),
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
                                WHERE status = 'PENDING' AND job_type = 'subscription_churn_prediction'
                                AND next_retry_at <= CURRENT_TIMESTAMP
                                ORDER BY created_at ASC
                                LIMIT 1 FOR UPDATE SKIP LOCKED
                                "#
                            )
                            .fetch_optional(&mut *tx)
                            .await
                            .map_err(|e| e.to_string())?;

                            if let Some(r) = row {
                                let id: String = r.get("id");
                                let tenant_id: String = r.get("tenant_id");
                                let payload: serde_json::Value = r.get("payload");

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
                                WHERE status = 'PENDING' AND job_type = 'subscription_churn_prediction'
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

                // Draft a re-engagement message and push to Agent Feed.
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

                let context_payload = json!({
                    "feature_type": "subscription_churn_winback",
                    "customer_name": customer_name,
                    "description": format!("Health Score dropped due to no bookings in 21 days for {}. Subscriber is at risk of churning.", customer_name)
                });

                match &db.store {
                    crate::db::DbStore::Postgres => {
                        let ai_prompt = format!("Draft a personalized win-back message offering a free 15-minute consultation to get back on track for a subscriber named {} who hasn't been active recently.", customer_name);
                        let ai_prompt_reduced = crate::pricing::compression::reduce_tokens(&ai_prompt);
                        let drafted_message = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
                            Ok("gemini") => crate::minimax::LocalLLMClient::new().reason(&ai_prompt_reduced).await.unwrap_or_else(|_| "Hi! We noticed you haven't been active recently. Would you like a free 15-minute consultation?".to_string()),
                            Ok("minimax") => {
                                let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                                crate::minimax::MinimaxClient::new(api_key).reason(&ai_prompt_reduced).await.unwrap_or_else(|_| "Hi! We noticed you haven't been active recently. Would you like a free 15-minute consultation?".to_string())
                            }
                            _ => crate::minimax::LocalLLMClient::new().reason(&ai_prompt_reduced).await.unwrap_or_else(|_| "Hi! We noticed you haven't been active recently. Would you like a free 15-minute consultation?".to_string()),
                        };

                        let proposed_action = json!({
                            "action_type": "send_message",
                            "draft_action": drafted_message,
                            "message": drafted_message
                        });

                        let _ = sqlx::query(
                            r#"
                            INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
                            VALUES ($1, $2, 'sales', $3, $4, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
                            "#
                        )
                        .bind(Uuid::new_v4().to_string())
                        .bind(&tenant_id)
                        .bind(sqlx::types::Json(context_payload))
                        .bind(sqlx::types::Json(proposed_action))
                        .execute(&db.pool)
                        .await;
                    },
                    crate::db::DbStore::Sqlite(_) => {
                         let ai_prompt = format!("Draft a personalized win-back message offering a free 15-minute consultation to get back on track for a subscriber named {} who hasn't been active recently.", customer_name);
                         let ai_prompt_reduced = crate::pricing::compression::reduce_tokens(&ai_prompt);
                         let drafted_message = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
                             Ok("gemini") => crate::minimax::LocalLLMClient::new().reason(&ai_prompt_reduced).await.unwrap_or_else(|_| "Hi! We noticed you haven't been active recently. Would you like a free 15-minute consultation?".to_string()),
                             Ok("minimax") => {
                                 let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                                 crate::minimax::MinimaxClient::new(api_key).reason(&ai_prompt_reduced).await.unwrap_or_else(|_| "Hi! We noticed you haven't been active recently. Would you like a free 15-minute consultation?".to_string())
                             }
                             _ => crate::minimax::LocalLLMClient::new().reason(&ai_prompt_reduced).await.unwrap_or_else(|_| "Hi! We noticed you haven't been active recently. Would you like a free 15-minute consultation?".to_string()),
                         };

                         let proposed_action = json!({
                             "action_type": "send_message",
                             "draft_action": drafted_message,
                             "message": drafted_message
                         });

                         let _ = sqlx::query(
                             r#"
                             INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
                             VALUES (?, ?, 'sales', ?, ?, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
                             "#
                         )
                         .bind(Uuid::new_v4().to_string())
                         .bind(&tenant_id)
                         .bind(sqlx::types::Json(context_payload))
                         .bind(sqlx::types::Json(proposed_action))
                         .execute(&db.pool)
                         .await;
                    }
                }

                // Mark Job as Completed
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
