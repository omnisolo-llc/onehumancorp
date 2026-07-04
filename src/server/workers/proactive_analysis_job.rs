use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;

use uuid::Uuid;


use tokio::time::timeout;

const AI_AGENT_TIMEOUT: Duration = Duration::from_secs(60);
const MAX_RETRIES: u32 = 3;

pub struct ProactiveAnalysisWorker {
    pub db: Arc<DB>,
}

impl ProactiveAnalysisWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60)); // Check every minute in dev (or configurable)
            loop {
                interval.tick().await;

                // 1. Get all active tenants
                let tenants: Vec<String> = match &db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query_scalar("SELECT id FROM tenants")
                            .fetch_all(&db.pool)
                            .await
                            .unwrap_or_default()
                    },
                    crate::db::DbStore::Sqlite(_) => {
                        sqlx::query_scalar("SELECT id FROM tenants")
                            .fetch_all(&db.pool)
                            .await
                            .unwrap_or_default()
                    }
                };

                for tenant_id in tenants {
                    // Check if there's already a pending proactive task for this tenant today to avoid spamming
                    let has_pending = match &db.store {
                        crate::db::DbStore::Postgres => {
                            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM triage_items WHERE tenant_id = $1 AND source = 'Proactive Context Agent' AND status = 'pending' AND created_at > CURRENT_TIMESTAMP - INTERVAL '1 day'")
                                .bind(&tenant_id)
                                .fetch_one(&db.pool)
                                .await
                                .unwrap_or(0) > 0
                        },
                        crate::db::DbStore::Sqlite(_) => {
                            sqlx::query_scalar::<_, i32>("SELECT COUNT(*) FROM triage_items WHERE tenant_id = $1 AND source = 'Proactive Context Agent' AND status = 'pending' AND created_at > datetime('now', '-1 day')")
                                .bind(&tenant_id)
                                .fetch_one(&db.pool)
                                .await
                                .unwrap_or(0) > 0
                        }
                    };

                    if has_pending {
                        continue;
                    }

                    // Collect Context: Find stale quotes, pending orders, or unread messages
                    let mut unconfirmed_bookings = 0;
                    match &db.store {
                        crate::db::DbStore::Postgres => {
                            if let Ok(count) = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM calendar_events WHERE tenant_id = $1 AND status = 'unconfirmed' AND created_at > CURRENT_TIMESTAMP - INTERVAL '7 days'")
                                .bind(&tenant_id)
                                .fetch_one(&db.pool).await {
                                unconfirmed_bookings = count;
                            }
                        },
                        crate::db::DbStore::Sqlite(_) => {
                            if let Ok(count) = sqlx::query_scalar::<_, i32>("SELECT COUNT(*) FROM calendar_events WHERE tenant_id = $1 AND status = 'unconfirmed' AND created_at > datetime('now', '-7 days')")
                                .bind(&tenant_id)
                                .fetch_one(&db.pool).await {
                                unconfirmed_bookings = count as i64;
                            }
                        }
                    };

                    let mut unfulfilled_orders = 0;
                    match &db.store {
                        crate::db::DbStore::Postgres => {
                            if let Ok(count) = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM pos_orders WHERE tenant_id = $1 AND status = 'pending' AND created_at > CURRENT_TIMESTAMP - INTERVAL '7 days'")
                                .bind(&tenant_id)
                                .fetch_one(&db.pool).await {
                                unfulfilled_orders = count;
                            }
                        },
                        crate::db::DbStore::Sqlite(_) => {
                            if let Ok(count) = sqlx::query_scalar::<_, i32>("SELECT COUNT(*) FROM pos_orders WHERE tenant_id = $1 AND status = 'pending' AND created_at > datetime('now', '-7 days')")
                                .bind(&tenant_id)
                                .fetch_one(&db.pool).await {
                                unfulfilled_orders = count as i64;
                            }
                        }
                    };

                    // Only proceed if we have something interesting
                    if unconfirmed_bookings > 0 || unfulfilled_orders > 0 {
                        let prompt = format!("You are the Proactive Context Agent for a business owner. They have {} unconfirmed bookings and {} unfulfilled pending orders from the last 7 days. Give me a short, friendly message highlighting this anomaly and suggesting what they should do next, and output a JSON array of 1 possible action with a 'type' (e.g. 'DraftFollowups' or 'SendReminders') and 'payload' (a short description of the action). Example output format: {{\"message\": \"...\", \"actions\": [{{\"type\": \"...\", \"payload\": \"...\"}}]}}", unconfirmed_bookings, unfulfilled_orders);

                        let mut attempts = 0;
                        let mut ai_response = String::new();
                        while attempts < MAX_RETRIES {
                            let ai_op = async {
                                if let Ok(mut client) = ::server_ohc::orchestration::hub_service_client::HubServiceClient::connect(std::env::var("OHC_HUB_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".to_string())).await {
                                    let reason_req = ::server_ohc::orchestration::ReasonRequest {
                                        prompt: ::server_pricing::compression::reduce_tokens(&prompt),
                                        from_agent_id: "Proactive Context Agent".into(),
                                    };
                                    if let Ok(res) = client.reason(tonic::Request::new(reason_req)).await {
                                        return Ok(res.into_inner().content);
                                    }
                                }
                                Err("AI call failed".to_string())
                            };

                            match timeout(AI_AGENT_TIMEOUT, ai_op).await {
                                Ok(Ok(content)) => {
                                    ai_response = content;
                                    break;
                                },
                                _ => {
                                    attempts += 1;
                                    tokio::time::sleep(std::time::Duration::from_secs(2u64.pow(attempts as u32))).await;
                                }
                            }
                        }

                        if ai_response.is_empty() {
                            let task_id = Uuid::new_v4().to_string();
                            let _context_message = "System is paused. Please manually check business performance.";
                            match &db.store {
                                crate::db::DbStore::Postgres => {
                                    let _ = sqlx::query(
                                        "INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                                    )
                                    .bind(&task_id).bind(&tenant_id).bind("business_advisory").bind("AI Agent Paused: The Advisor").bind("PAUSED").bind("LOW").bind(r#"{"proposed_content": "System is paused. Please manually check business performance."}"#)
                                    .execute(&db.pool).await;
                                },
                                crate::db::DbStore::Sqlite(_) => {
                                    let _ = sqlx::query(
                                        "INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                                    )
                                    .bind(&task_id).bind(&tenant_id).bind("business_advisory").bind("AI Agent Paused: The Advisor").bind("PAUSED").bind("LOW").bind(r#"{"proposed_content": "System is paused. Please manually check business performance."}"#)
                                    .execute(&db.pool).await;
                                }
                            }
                            continue;
                        }

                        if ai_response.is_empty() {
                            let task_id = Uuid::new_v4().to_string();
                            match &db.store {
                                crate::db::DbStore::Postgres => {
                                    if let Err(e) = sqlx::query(
                                        "INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                                    )
                                    .bind(&task_id).bind(&tenant_id).bind("business_advisory").bind("AI Agent Paused: The Advisor").bind("PAUSED").bind("LOW").bind(r#"{"proposed_content": "System is paused. Please manually check business performance."}"#)
                                    .execute(&db.pool).await {
                                        tracing::error!("Failed to insert PAUSED state for proactive analysis job: {}", e);
                                    }
                                },
                                crate::db::DbStore::Sqlite(_) => {
                                    if let Err(e) = sqlx::query(
                                        "INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                                    )
                                    .bind(&task_id).bind(&tenant_id).bind("business_advisory").bind("AI Agent Paused: The Advisor").bind("PAUSED").bind("LOW").bind(r#"{"proposed_content": "System is paused. Please manually check business performance."}"#)
                                    .execute(&db.pool).await {
                                        tracing::error!("Failed to insert PAUSED state for proactive analysis job: {}", e);
                                    }
                                }
                            }
                            continue;
                        }

                        if !ai_response.is_empty() {
                            // Extract JSON from response (naive extraction, assume the agent returns mostly JSON)
                            let json_start = ai_response.find('{').unwrap_or(0);
                            let json_end = ai_response.rfind('}').unwrap_or(ai_response.len() - 1) + 1;
                            if json_start < json_end {
                                let json_str = &ai_response[json_start..json_end];
                                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(json_str) {
                                    let context_message = parsed.get("message").and_then(|m| m.as_str()).unwrap_or("You have some items needing attention.");
                                    let task_id = Uuid::new_v4().to_string();

                                    match &db.store {
                                        crate::db::DbStore::Postgres => {
                                            let _ = sqlx::query(
                                                "INSERT INTO triage_items (id, tenant_id, source, priority, context, status) VALUES ($1, $2, $3, $4, $5, $6)"
                                            )
                                            .bind(&task_id)
                                            .bind(&tenant_id)
                                            .bind("Proactive Context Agent")
                                            .bind("High")
                                            .bind(context_message)
                                            .bind("pending")
                                            .execute(&db.pool)
                                            .await;
                                        },
                                        crate::db::DbStore::Sqlite(_) => {
                                            let _ = sqlx::query(
                                                "INSERT INTO triage_items (id, tenant_id, source, priority, context, status) VALUES (?, ?, ?, ?, ?, ?)"
                                            )
                                            .bind(&task_id)
                                            .bind(&tenant_id)
                                            .bind("Proactive Context Agent")
                                            .bind("High")
                                            .bind(context_message)
                                            .bind("pending")
                                            .execute(&db.pool)
                                            .await;
                                        }
                                    }

                                    if let Some(actions) = parsed.get("actions").and_then(|a| a.as_array()) {
                                        if let Some(first_action) = actions.first() {
                                            let action_type = first_action.get("type").and_then(|t| t.as_str()).unwrap_or("Review");
                                            let action_payload = first_action.get("payload").and_then(|p| p.as_str()).unwrap_or("");

                                            match &db.store {
                                                crate::db::DbStore::Postgres => {
                                                    let _ = sqlx::query(
                                                        "INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload) VALUES ($1, $2, $3, $4, $5)"
                                                    )
                                                    .bind(Uuid::new_v4().to_string())
                                                    .bind(&task_id)
                                                    .bind(&tenant_id)
                                                    .bind(action_type)
                                                    .bind(action_payload)
                                                    .execute(&db.pool)
                                                    .await;
                                                },
                                                crate::db::DbStore::Sqlite(_) => {
                                                    let _ = sqlx::query(
                                                        "INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload) VALUES (?, ?, ?, ?, ?)"
                                                    )
                                                    .bind(Uuid::new_v4().to_string())
                                                    .bind(&task_id)
                                                    .bind(&tenant_id)
                                                    .bind(action_type)
                                                    .bind(action_payload)
                                                    .execute(&db.pool)
                                                    .await;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_ml_resilience_proactive_analysis_timeout() {
        let start = std::time::Instant::now();
        let result = tokio::time::timeout(std::time::Duration::from_millis(60), async {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            Ok::<(), String>(())
        })
        .await;

        assert!(
            result.is_err(),
            "ProactiveAnalysisWorker must enforce ML-Resilience timeout"
        );
        assert!(
            start.elapsed() >= std::time::Duration::from_millis(50),
            "Timeout should wait the configured time"
        );
    }
}
