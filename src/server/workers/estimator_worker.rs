use std::sync::Arc;
use tokio::time::Duration;
use tokio::time::timeout;
use crate::db::DB;
use sqlx::Row;
use uuid::Uuid;

const AI_AGENT_TIMEOUT: Duration = Duration::from_secs(60);
const MAX_RETRIES: u32 = 3;

pub struct EstimatorAgentWorker {
    db: Arc<DB>,
}

impl EstimatorAgentWorker {
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
                        tracing::error!("EstimatorAgentWorker error: {}", e);
                        tokio::time::sleep(Duration::from_millis(5000)).await;
                    }
                }
            }
        });
    }

    async fn poll(&self) -> Result<bool, String> {
        let job = match &self.db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    "SELECT id, tenant_id, payload FROM ohc_job_queue
                     WHERE job_type = 'LeadReceived' AND status = 'PENDING' AND (next_retry_at IS NULL OR next_retry_at <= NOW())
                     ORDER BY created_at ASC
                     FOR UPDATE SKIP LOCKED
                     LIMIT 1"
                )
                .fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    let id: String = r.get("id");
                    let tenant_id: String = r.get("tenant_id");
                    let payload: serde_json::Value = r.get("payload");

                    sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = NOW() WHERE id = $1")
                        .bind(&id)
                        .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;
                    Some((id, tenant_id, payload))
                } else {
                    None
                }
            },
            crate::db::DbStore::Sqlite(pool) => {
                let row = sqlx::query(
                    "SELECT id, tenant_id, payload FROM ohc_job_queue
                     WHERE job_type = 'LeadReceived' AND status = 'PENDING' AND (next_retry_at IS NULL OR next_retry_at <= CURRENT_TIMESTAMP)
                     ORDER BY created_at ASC
                     LIMIT 1"
                )
                .fetch_optional(pool).await.map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    let id: String = r.get("id");
                    let tenant_id: String = r.get("tenant_id");
                    let payload_str: String = r.get("payload");
                    let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or(serde_json::json!({}));

                    sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&id)
                        .execute(pool).await.map_err(|e| e.to_string())?;

                    Some((id, tenant_id, payload))
                } else {
                    None
                }
            }
        };

        if let Some((job_id, tenant_id, job_payload)) = job {
            let lead_id = job_payload.get("lead_id").and_then(|v| v.as_str()).unwrap_or("");

            if lead_id.is_empty() {
                self.mark_job_failed(&job_id).await?;
                return Ok(true);
            }

            // Fetch lead details
            let lead_description = match &self.db.store {
                crate::db::DbStore::Postgres => {
                    let row = sqlx::query("SELECT description, customer_id FROM service_leads WHERE id = $1 AND tenant_id = $2")
                        .bind(lead_id)
                        .bind(&tenant_id)
                        .fetch_optional(&self.db.pool).await.map_err(|e| e.to_string())?;
                    row.map(|r| {
                        let desc: Option<String> = r.try_get("description").unwrap_or_default();
                        desc.unwrap_or_default()
                    }).unwrap_or_default()
                },
                crate::db::DbStore::Sqlite(pool) => {
                    let row = sqlx::query("SELECT description, customer_id FROM service_leads WHERE id = ? AND tenant_id = ?")
                        .bind(lead_id)
                        .bind(&tenant_id)
                        .fetch_optional(pool).await.map_err(|e| e.to_string())?;
                    row.map(|r| {
                        let desc: Option<String> = r.try_get("description").unwrap_or_default();
                        desc.unwrap_or_default()
                    }).unwrap_or_default()
                }
            };

            let customer_id = match &self.db.store {
                crate::db::DbStore::Postgres => {
                    let row = sqlx::query("SELECT customer_id FROM service_leads WHERE id = $1 AND tenant_id = $2")
                        .bind(lead_id)
                        .bind(&tenant_id)
                        .fetch_optional(&self.db.pool).await.map_err(|e| e.to_string())?;
                    row.map(|r| {
                        let c_id: Option<uuid::Uuid> = r.try_get("customer_id").unwrap_or_default();
                        c_id.map(|u| u.to_string()).unwrap_or_else(|| Uuid::new_v4().to_string())
                    }).unwrap_or_else(|| Uuid::new_v4().to_string())
                },
                crate::db::DbStore::Sqlite(pool) => {
                    let row = sqlx::query("SELECT customer_id FROM service_leads WHERE id = ? AND tenant_id = ?")
                        .bind(lead_id)
                        .bind(&tenant_id)
                        .fetch_optional(pool).await.map_err(|e| e.to_string())?;
                    row.map(|r| {
                        let c_id: Option<String> = r.try_get("customer_id").unwrap_or_default();
                        c_id.unwrap_or_else(|| Uuid::new_v4().to_string())
                    }).unwrap_or_else(|| Uuid::new_v4().to_string())
                }
            };

            if lead_description.is_empty() {
                tracing::warn!("Lead description is empty for lead_id: {}", lead_id);
                self.mark_job_failed(&job_id).await?;
                return Ok(true);
            }

            // Simple heuristic lookup (RAG equivalent for this implementation)
            let mut service_name = "Custom Project Scope".to_string();
            let mut rate_cents = 150000; // $1500.00 default

            let heuristics_res = match &self.db.store {
                crate::db::DbStore::Postgres => {
                    sqlx::query("SELECT service_category, base_rate_cents FROM pricing_heuristics WHERE tenant_id = $1")
                        .bind(&tenant_id)
                        .fetch_all(&self.db.pool).await.ok()
                },
                crate::db::DbStore::Sqlite(pool) => {
                    sqlx::query("SELECT service_category, base_rate_cents FROM pricing_heuristics WHERE tenant_id = ?")
                        .bind(&tenant_id)
                        .fetch_all(pool).await.ok()
                }
            };

            if let Some(heuristics) = heuristics_res {
                for h in heuristics {
                    let category: String = h.get("service_category");
                    let cents: i64 = h.get("base_rate_cents");
                    if lead_description.to_lowercase().contains(&category.to_lowercase()) {
                        service_name = category;
                        rate_cents = cents;
                        break;
                    }
                }
            }

            let prompt = format!("You are the Estimator Agent. A customer submitted the following request: '{}'.
The pricing heuristic matched is '{}' with a base rate of {} cents.
Create a structured quote. Output MUST be a JSON object with 'total_amount_cents' (integer), 'required_deposit_cents' (integer, usually 50% or 20%), and 'line_items' (array of objects with 'description', 'unit_price_cents', 'quantity', 'is_optional').",
                lead_description, service_name, rate_cents);

            let mut attempts = 0;
            let mut ai_response = String::new();
            while attempts < MAX_RETRIES {
                let ai_op = async {
                    if let Ok(mut client) = ::server_ohc::orchestration::hub_service_client::HubServiceClient::connect(std::env::var("OHC_HUB_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".to_string())).await {
                        let reason_req = ::server_ohc::orchestration::ReasonRequest {
                            prompt: prompt.clone(),
                            from_agent_id: "Estimator Agent".into(),
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

            let mut parsed_quote = serde_json::json!({
                "total_amount_cents": rate_cents,
                "required_deposit_cents": rate_cents / 2,
                "line_items": [
                    {
                        "description": service_name,
                        "unit_price_cents": rate_cents,
                        "quantity": 1,
                        "is_optional": false
                    }
                ]
            });

            if !ai_response.is_empty() {
                let json_start = ai_response.find('{').unwrap_or(0);
                let json_end = ai_response.rfind('}').unwrap_or(ai_response.len().saturating_sub(1)) + 1;
                if json_start < json_end {
                    let json_str = &ai_response[json_start..json_end];
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(json_str) {
                        if parsed.get("total_amount_cents").is_some() && parsed.get("line_items").is_some() {
                            parsed_quote = parsed;
                        }
                    }
                }
            }

            let quote_id = Uuid::new_v4().to_string();
            let total_amount = parsed_quote.get("total_amount_cents").and_then(|v| v.as_i64()).unwrap_or(rate_cents);
            let required_deposit = parsed_quote.get("required_deposit_cents").and_then(|v| v.as_i64()).unwrap_or(rate_cents / 2);

            let customer_uuid = Uuid::parse_str(&customer_id).unwrap_or_else(|_| Uuid::new_v4());

            match &self.db.store {
                crate::db::DbStore::Postgres => {
                    let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                    let _ = sqlx::query(
                        "INSERT INTO quotes (id, tenant_id, customer_id, status, total_amount, required_deposit, checkout_url, created_at, updated_at) VALUES ($1, $2, $3, 'DRAFT', $4, $5, NULL, NOW(), NOW())"
                    )
                    .bind(Uuid::parse_str(&quote_id).unwrap_or_else(|_| Uuid::new_v4()))
                    .bind(&tenant_id)
                    .bind(customer_uuid)
                    .bind(total_amount)
                    .bind(required_deposit)
                    .execute(&mut *tx).await;

                    if let Some(items) = parsed_quote.get("line_items").and_then(|v| v.as_array()) {
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
                            .bind(Uuid::parse_str(&quote_id).unwrap_or_else(|_| Uuid::new_v4()))
                            .bind(desc)
                            .bind(price)
                            .bind(qty)
                            .bind(is_opt)
                            .execute(&mut *tx).await;
                        }
                    }
                    let _ = tx.commit().await;
                },
                crate::db::DbStore::Sqlite(pool) => {
                    let _ = sqlx::query(
                        "INSERT INTO quotes (id, tenant_id, customer_id, status, total_amount, required_deposit, checkout_url, created_at, updated_at) VALUES (?, ?, ?, 'DRAFT', ?, ?, NULL, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                    )
                    .bind(&quote_id)
                    .bind(&tenant_id)
                    .bind(customer_uuid.to_string())
                    .bind(total_amount)
                    .bind(required_deposit)
                    .execute(pool).await;

                    if let Some(items) = parsed_quote.get("line_items").and_then(|v| v.as_array()) {
                        for item in items {
                            let item_id = Uuid::new_v4().to_string();
                            let desc = item.get("description").and_then(|v| v.as_str()).unwrap_or("");
                            let price = item.get("unit_price_cents").and_then(|v| v.as_i64()).unwrap_or(0);
                            let qty = item.get("quantity").and_then(|v| v.as_i64()).unwrap_or(1) as i32;
                            let is_opt = item.get("is_optional").and_then(|v| v.as_bool()).unwrap_or(false);
                            let _ = sqlx::query(
                                "INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                            )
                            .bind(&item_id)
                            .bind(&quote_id)
                            .bind(desc)
                            .bind(price)
                            .bind(qty)
                            .bind(is_opt)
                            .execute(pool).await;
                        }
                    }
                }
            }

            let agent_feed_item_id = Uuid::new_v4().to_string();
            let context_summary = format!("Draft Quote: {} for Customer", service_name);

            let action_payload_for_feed = serde_json::json!({
                "action_type": "Draft Quote",
                "quote_id": quote_id,
                "feature_type": "quote_draft",
                "total_amount_cents": total_amount,
                "required_deposit_cents": required_deposit,
                "service_name": service_name,
                "line_items": parsed_quote.get("line_items")
            });

            match &self.db.store {
                crate::db::DbStore::Postgres => {
                    let _ = sqlx::query(
                        "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 'PENDING_APPROVAL', NOW(), NOW())"
                    )
                    .bind(&agent_feed_item_id)
                    .bind(&tenant_id)
                    .bind("Estimator Agent")
                    .bind(serde_json::json!({
                        "feature_type": "quote_draft",
                        "context": context_summary,
                        "lead_id": lead_id
                    }))
                    .bind(&action_payload_for_feed)
                    .execute(&self.db.pool).await;

                    let _ = sqlx::query(
                        "INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at) VALUES ($1, $2, 'Sales', $3, 'DRAFT', 'DraftForReview', $4, NOW(), NOW())"
                    )
                    .bind(&agent_feed_item_id)
                    .bind(&tenant_id)
                    .bind(&context_summary)
                    .bind(&action_payload_for_feed)
                    .execute(&self.db.pool).await;
                },
                crate::db::DbStore::Sqlite(pool) => {
                    let _ = sqlx::query(
                        "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                    )
                    .bind(&agent_feed_item_id)
                    .bind(&tenant_id)
                    .bind("Estimator Agent")
                    .bind(serde_json::json!({
                        "feature_type": "quote_draft",
                        "context": context_summary,
                        "lead_id": lead_id
                    }).to_string())
                    .bind(action_payload_for_feed.to_string())
                    .execute(pool).await;

                    let _ = sqlx::query(
                        "INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at) VALUES (?, ?, 'Sales', ?, 'DRAFT', 'DraftForReview', ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                    )
                    .bind(&agent_feed_item_id)
                    .bind(&tenant_id)
                    .bind(&context_summary)
                    .bind(action_payload_for_feed.to_string())
                    .execute(pool).await;
                }
            }

            self.mark_job_completed(&job_id).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn mark_job_failed(&self, job_id: &str) -> Result<(), String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = NOW() WHERE id = $1")
                    .bind(job_id)
                    .execute(&self.db.pool).await.map_err(|e| e.to_string())?;
            },
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                    .bind(job_id)
                    .execute(pool).await.map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    async fn mark_job_completed(&self, job_id: &str) -> Result<(), String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = NOW() WHERE id = $1")
                    .bind(job_id)
                    .execute(&self.db.pool).await.map_err(|e| e.to_string())?;
            },
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                    .bind(job_id)
                    .execute(pool).await.map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
}
