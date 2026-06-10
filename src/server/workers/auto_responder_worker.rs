use std::sync::Arc;
use crate::db::{DB, DbStore};
use crate::queue::{QueueManager, SubAgentJob};
use std::time::Duration;
use chrono::Utc;
use uuid::Uuid;
use serde_json::json;

pub struct AutoResponderWorker {
    pub db: Arc<DB>,
    pub queue: Arc<QueueManager>,
}

impl AutoResponderWorker {
    pub fn new(db: Arc<DB>) -> Self {
        let queue = Arc::new(QueueManager::new(db.pool.clone()));
        Self { db, queue }
    }

    pub fn start(&self) {
        let worker_id = format!("auto-responder-{}", Uuid::new_v4());
        let queue = self.queue.clone();
        let db = self.db.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(2));
            loop {
                interval.tick().await;
                match queue.poll(&worker_id).await {
                    Ok(Some(job)) => {
                        if job.payload["agent_role"] == "customer_auto_reply" {
                            if let Err(e) = Self::handle_job(&db, job.clone()).await {
                                tracing::error!("AutoResponderWorker failed job {}: {}", job.id, e);
                                // QueueManager::poll -> start_polling pattern handles retries,
                                // but since we're using a manual loop for demonstration of the pattern:
                                let attempts = job.payload.get("attempts").and_then(|v| v.as_i64()).unwrap_or(0);
                                if attempts < 3 {
                                    let mut new_payload = job.payload.clone();
                                    new_payload["attempts"] = json!(attempts + 1);
                                    let _ = queue.requeue(&job.id, &job.tenant_id, new_payload).await;
                                } else {
                                    let _ = queue.mark_failed(&job.id, &e, &job.tenant_id).await;
                                }
                            } else {
                                let _ = queue.mark_completed(&job.id, &job.tenant_id).await;
                            }
                        }
                    }
                    Ok(None) => {}
                    Err(e) => tracing::error!("AutoResponderWorker poll error: {}", e),
                }
            }
        });
    }

    async fn handle_job(db: &Arc<DB>, job: SubAgentJob) -> Result<(), String> {
        let tenant_id = &job.tenant_id;
        let inbox_message_id = job.payload.get("inbox_message_id").and_then(|v| v.as_str()).ok_or("Missing inbox_message_id")?;
        let customer_message = job.payload.get("content").and_then(|v| v.as_str()).unwrap_or("");
        let source = job.payload.get("source").and_then(|v| v.as_str()).unwrap_or("unknown");

        // 1. Fetch Tenant Context
        let (business_name, industry) = match &db.store {
            DbStore::Postgres => {
                let row = sqlx::query("SELECT name, industry FROM tenants WHERE id = $1")
                    .bind(tenant_id)
                    .fetch_optional(&db.pool).await.map_err(|e| e.to_string())?;
                row.map(|r| (r.get::<String, _>("name"), r.get::<String, _>("industry"))).unwrap_or(("A Business".into(), "SMB".into()))
            }
            DbStore::Sqlite(pool) => {
                let row = sqlx::query("SELECT name, industry FROM tenants WHERE id = ?")
                    .bind(tenant_id)
                    .fetch_optional(pool).await.map_err(|e| e.to_string())?;
                row.map(|r| (r.get::<String, _>("name"), r.get::<String, _>("industry"))).unwrap_or(("A Business".into(), "SMB".into()))
            }
        };

        // 2. Fetch Business Knowledge (Inventory summary)
        let inventory_summary = match &db.store {
            DbStore::Postgres => {
                let rows = sqlx::query("SELECT name, inventory_count FROM products WHERE tenant_id = $1 AND inventory_count < 20")
                    .bind(tenant_id)
                    .fetch_all(&db.pool).await.unwrap_or_default();
                rows.iter().map(|r| format!("{}: {} left", r.get::<String, _>("name"), r.get::<i32, _>("inventory_count"))).collect::<Vec<_>>().join(", ")
            }
            _ => "Context unavailable".to_string(),
        };

        // 3. AI Processing
        let prompt = format!(
            "You are the Intelligent Customer Auto-Responder for {}. Industry: {}. \
             Current Low Stock: {}. \
             Customer wrote (via {}): \"{}\". \
             Generate a helpful, warm response. If you can fully answer (e.g. greeting, hours, general info), do so. \
             If unsure or it requires an owner decision, provide a draft and set confidence to 'REVIEW'. \
             Output JSON: {{\"reply\": \"...\", \"confidence\": \"CONFIDENT\" | \"REVIEW\", \"explanation\": \"...\"}}",
            business_name, industry, inventory_summary, source, customer_message
        );

        let ai_response = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
            Ok("minimax") => {
                let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                crate::minimax::MinimaxClient::new(api_key).reason(&prompt).await.unwrap_or_default()
            }
            _ => crate::minimax::LocalLLMClient::new().reason(&prompt).await.unwrap_or_default()
        };

        let parsed: serde_json::Value = serde_json::from_str(&ai_response).unwrap_or(json!({
            "reply": "Thank you for reaching out! We've received your message and will get back to you shortly.",
            "confidence": "REVIEW",
            "explanation": "Fallback response used."
        }));

        let reply = parsed["reply"].as_str().unwrap_or("Thank you!");
        let confidence_str = parsed["confidence"].as_str().unwrap_or("REVIEW");
        let confidence_score = if confidence_str == "CONFIDENT" { 0.95 } else { 0.45 };
        let status = if confidence_str == "CONFIDENT" { "sent" } else { "draft" };

        // 4. Update Database
        match &db.store {
            DbStore::Postgres => {
                sqlx::query(
                    "UPDATE inbox_messages SET draft_reply = $1, status = $2, handled_by_ai = TRUE, confidence_score = $3, ai_metadata = $4 WHERE id = $5 AND tenant_id = $6"
                )
                .bind(reply)
                .bind(status)
                .bind(confidence_score)
                .bind(json!({"explanation": parsed["explanation"]}))
                .bind(inbox_message_id)
                .bind(tenant_id)
                .execute(&db.pool).await.map_err(|e| e.to_string())?;

                // If confident, also log it as an action
                if status == "sent" {
                     let _ = sqlx::query(
                        "INSERT INTO agent_actions (id, tenant_id, agent_id, action_type, description, payload) VALUES ($1, $2, $3, $4, $5, $6)"
                    )
                    .bind(Uuid::new_v4().to_string())
                    .bind(tenant_id)
                    .bind("auto_responder")
                    .bind("customer.auto_reply_sent")
                    .bind(format!("Auto-replied to {} message", source))
                    .bind(json!({"reply": reply, "inbox_message_id": inbox_message_id}))
                    .execute(&db.pool).await;
                }
            }
            DbStore::Sqlite(pool) => {
                 let _ = sqlx::query(
                    "UPDATE inbox_messages SET draft_reply = ?, status = ?, handled_by_ai = 1, confidence_score = ? WHERE id = ? AND tenant_id = ?"
                )
                .bind(reply)
                .bind(status)
                .bind(confidence_score)
                .bind(inbox_message_id)
                .bind(tenant_id)
                .execute(pool).await;
            }
        }

        Ok(())
    }
}
