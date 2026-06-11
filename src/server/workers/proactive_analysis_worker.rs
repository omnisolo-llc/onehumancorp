use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use uuid::Uuid;
use sqlx::Row;
use serde_json::json;
use tokio::time::{interval, timeout};

const AI_AGENT_TIMEOUT: Duration = Duration::from_secs(60);
const DB_OP_TIMEOUT: Duration = Duration::from_secs(2);

pub struct ProactiveAnalysisWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl ProactiveAnalysisWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(300), // run every 5 mins
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            let mut interval = interval(interval_duration);
            loop {
                interval.tick().await;
                if let Err(e) = Self::poll(&db).await {
                    ::server_telemetry::record_error_signal("ProactiveAnalysisWorker error");
                    tracing::error!("ProactiveAnalysisWorker error: {}", e);
                }
            }
        });
    }

    pub async fn poll(db: &Arc<DB>) -> Result<(), String> {
        let poll_op = async {
            match &db.store {
                crate::db::DbStore::Postgres => {
                    let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;

                    // Get active tenants
                    let tenants = sqlx::query("SELECT id FROM tenants LIMIT 100")
                        .fetch_all(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                    for tenant_row in tenants {
                        let tenant_id: String = tenant_row.try_get("id").unwrap_or_default();
                        if tenant_id.is_empty() { continue; }

                        let triage_items = sqlx::query("SELECT id, source, context FROM triage_items WHERE tenant_id = $1 AND status = 'pending' LIMIT 10")
                            .bind(&tenant_id)
                            .fetch_all(&mut *tx)
                            .await
                            .map_err(|e| e.to_string())?;

                        let mut triage_context = Vec::new();
                        for row in triage_items {
                            let id: String = row.try_get("id").unwrap_or_default();
                            let source: String = row.try_get("source").unwrap_or_default();
                            let context: String = row.try_get("context").unwrap_or_default();
                            triage_context.push(json!({"id": id, "source": source, "context": context}));
                        }

                        let pending_orders = sqlx::query("SELECT id, total_amount FROM orders WHERE tenant_id = $1 AND status = 'pending' LIMIT 10")
                            .bind(&tenant_id)
                            .fetch_all(&mut *tx)
                            .await
                            .map_err(|e| e.to_string())?;

                        let mut orders_context = Vec::new();
                        for row in pending_orders {
                            let id: String = row.try_get("id").unwrap_or_default();
                            let total_amount_f64: f64 = match row.try_get::<f64, _>("total_amount") {
                                Ok(f) => f,
                                Err(_) => 0.0 // fallback
                            };
                            orders_context.push(json!({"id": id, "total_amount": total_amount_f64.to_string()}));
                        }

                        if triage_context.is_empty() && orders_context.is_empty() {
                            continue;
                        }

                        let context_json = json!({
                            "triage_items": triage_context,
                            "pending_orders": orders_context
                        });

                        let prompt = format!(
                            "Analyze this business context and suggest 1 actionable insight to the owner. Context: {}. Return JSON with 'action_type' and 'payload'. Example: {{\"action_type\": \"SendReminder\", \"payload\": {{\"message\": \"Draft follow-up for order 123\"}}}}",
                            context_json.to_string()
                        );

                        let ai_op = async {
                            if let Ok(mut client) = ::server_ohc::orchestration::hub_service_client::HubServiceClient::connect(std::env::var("OHC_HUB_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".to_string())).await {
                                let reason_req = ::server_ohc::orchestration::ReasonRequest {
                                    prompt,
                                    from_agent_id: "ProactiveContextAgent".to_string(),
                                };
                                if let Ok(res) = client.reason(tonic::Request::new(reason_req)).await {
                                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&res.into_inner().content) {
                                        return Ok(v);
                                    }
                                }
                            }
                            Err("AI call failed".to_string())
                        };

                        if let Ok(Ok(insight_json)) = timeout(AI_AGENT_TIMEOUT, ai_op).await {
                            let action_type = insight_json.get("action_type").and_then(|v| v.as_str()).unwrap_or("UnknownAction").to_string();
                            let payload = insight_json.get("payload").unwrap_or(&json!({})).clone();

                            let item_id = Uuid::new_v4().to_string();
                            let _ = sqlx::query(
                                r#"
                                INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
                                VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
                                "#
                            )
                            .bind(&item_id)
                            .bind(&tenant_id)
                            .bind("Proactive Context Agent")
                            .bind(sqlx::types::Json(&context_json))
                            .bind(sqlx::types::Json(&json!({"action_type": action_type, "payload": payload})))
                            .bind("PENDING_APPROVAL")
                            .execute(&mut *tx)
                            .await;

                        }
                    }
                    tx.commit().await.map_err(|e| e.to_string())?;
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    // Sqlite implementation logic similar to Postgres
                    let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;

                    let tenants = sqlx::query("SELECT id FROM tenants LIMIT 100")
                        .fetch_all(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                    for tenant_row in tenants {
                        let tenant_id: String = tenant_row.try_get("id").unwrap_or_default();
                        if tenant_id.is_empty() { continue; }

                        let triage_items = sqlx::query("SELECT id, source, context FROM triage_items WHERE tenant_id = ? AND status = 'pending' LIMIT 10")
                            .bind(&tenant_id)
                            .fetch_all(&mut *tx)
                            .await
                            .map_err(|e| e.to_string())?;

                        let mut triage_context = Vec::new();
                        for row in triage_items {
                            let id: String = row.try_get("id").unwrap_or_default();
                            let source: String = row.try_get("source").unwrap_or_default();
                            let context: String = row.try_get("context").unwrap_or_default();
                            triage_context.push(json!({"id": id, "source": source, "context": context}));
                        }

                        let pending_orders = sqlx::query("SELECT id, total_amount FROM orders WHERE tenant_id = ? AND status = 'pending' LIMIT 10")
                            .bind(&tenant_id)
                            .fetch_all(&mut *tx)
                            .await
                            .map_err(|e| e.to_string())?;

                        let mut orders_context = Vec::new();
                        for row in pending_orders {
                            let id: String = row.try_get("id").unwrap_or_default();
                            let total_amount: f64 = row.try_get::<f64, _>("total_amount").unwrap_or(0.0);
                            orders_context.push(json!({"id": id, "total_amount": total_amount.to_string()}));
                        }

                        if triage_context.is_empty() && orders_context.is_empty() {
                            continue;
                        }

                        let context_json = json!({
                            "triage_items": triage_context,
                            "pending_orders": orders_context
                        });

                        let prompt = format!(
                            "Analyze this business context and suggest 1 actionable insight to the owner. Context: {}. Return JSON with 'action_type' and 'payload'. Example: {{\"action_type\": \"SendReminder\", \"payload\": {{\"message\": \"Draft follow-up for order 123\"}}}}",
                            context_json.to_string()
                        );

                        let ai_op = async {
                            if let Ok(mut client) = ::server_ohc::orchestration::hub_service_client::HubServiceClient::connect(std::env::var("OHC_HUB_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".to_string())).await {
                                let reason_req = ::server_ohc::orchestration::ReasonRequest {
                                    prompt,
                                    from_agent_id: "ProactiveContextAgent".to_string(),
                                };
                                if let Ok(res) = client.reason(tonic::Request::new(reason_req)).await {
                                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&res.into_inner().content) {
                                        return Ok(v);
                                    }
                                }
                            }
                            Err("AI call failed".to_string())
                        };

                        if let Ok(Ok(insight_json)) = timeout(AI_AGENT_TIMEOUT, ai_op).await {
                            let action_type = insight_json.get("action_type").and_then(|v| v.as_str()).unwrap_or("UnknownAction").to_string();
                            let payload = insight_json.get("payload").unwrap_or(&json!({})).clone();

                            let item_id = Uuid::new_v4().to_string();
                            let _ = sqlx::query(
                                r#"
                                INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
                                VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
                                "#
                            )
                            .bind(&item_id)
                            .bind(&tenant_id)
                            .bind("Proactive Context Agent")
                            .bind(serde_json::to_string(&context_json).unwrap_or_default())
                            .bind(serde_json::to_string(&json!({"action_type": action_type, "payload": payload})).unwrap_or_default())
                            .bind("PENDING_APPROVAL")
                            .execute(&mut *tx)
                            .await;
                        }
                    }
                    tx.commit().await.map_err(|e| e.to_string())?;
                }
            }
            Ok::<(), String>(())
        };

        match timeout(DB_OP_TIMEOUT, poll_op).await {
            Ok(res) => res,
            Err(_) => Err("Database timeout during ProactiveAnalysisWorker::poll".to_string()),
        }
    }
}
