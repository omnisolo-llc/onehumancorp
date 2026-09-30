use crate::db::DB;
use omnisolo_builtin_agent::gpt_researcher::ResearcherLlmClient;
use omnisolo_builtin_agent::types::{ChatRequest, ChatResponse, Message, Usage};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

// We reuse a similar adapter structure as draft_quote_worker
struct AdapterLlm {}

#[async_trait::async_trait]
impl ResearcherLlmClient for AdapterLlm {
    async fn chat(
        &self,
        req: ChatRequest,
    ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut prompt = req.system.clone();
        for msg in &req.messages {
            prompt.push_str("\n\n");
            prompt.push_str(&msg.content);
        }

        let is_test_mode = cfg!(test);

        #[cfg(test)]
        let forced_response = if prompt.contains("test-in-scope") {
            Some(r#"{"in_scope": true, "reason": "test"}"#.to_string())
        } else if prompt.contains("test-out-of-scope") {
            Some(r#"{"in_scope": false, "reason": "test"}"#.to_string())
        } else {
            None
        };
        #[cfg(not(test))]
        let forced_response: Option<String> = None;

        let (response_text, usage) = if let Some(response) = forced_response {
            (
                response,
                Usage {
                    input_tokens: 10,
                    output_tokens: 20,
                    cache_creation_input_tokens: 0,
                    cache_read_input_tokens: 0,
                },
            )
        } else if is_test_mode {
            (
                r#"{"in_scope": true, "reason": "test fallback"}"#.to_string(),
                Usage {
                    input_tokens: 10,
                    output_tokens: 20,
                    cache_creation_input_tokens: 0,
                    cache_read_input_tokens: 0,
                },
            )
        } else {
            let client = crate::minimax::LocalLLMClient::new();
            let observed = client.reason_with_usage(&prompt, req.max_tokens).await?;
            let counts = observed
                .counts
                .ok_or("Local provider omitted usage; intake accounting requires reconciliation")?;
            let input_tokens = i32::try_from(counts.input)
                .map_err(|_| "Local input usage exceeds supported range")?;
            let output_tokens = i32::try_from(counts.output)
                .map_err(|_| "Local output usage exceeds supported range")?;
            let cache_read_input_tokens = i32::try_from(counts.cached_input)
                .map_err(|_| "Local cache usage exceeds supported range")?;
            (
                observed.text,
                Usage {
                    input_tokens,
                    output_tokens,
                    cache_read_input_tokens,
                    cache_creation_input_tokens: 0,
                },
            )
        };

        Ok(ChatResponse {
            message: Message::assistant(response_text),
            usage,
            stop_reason: "stop".to_string(),
            response_id: None,
        })
    }
}

pub struct InquiryIntakeWorker {
    db: Arc<DB>,
}

#[derive(Deserialize)]
struct QualificationResponse {
    in_scope: bool,
}

impl InquiryIntakeWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            tracing::info!("Starting InquiryIntakeWorker...");
            loop {
                match self.poll().await {
                    Ok(true) => continue,
                    Ok(false) => tokio::time::sleep(std::time::Duration::from_millis(1000)).await,
                    Err(e) => {
                        tracing::error!("InquiryIntakeWorker error: {}", e);
                        tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
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
                    WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP AND job_type = 'inquiry_intake_agent'
                    ORDER BY next_retry_at ASC, created_at ASC
                    FOR UPDATE SKIP LOCKED
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let res = if let Some(r) = row {
                    use sqlx::Row;
                    let id: String = r.get("id");
                    let tenant_id: String = r.get("tenant_id");
                    let payload_str: String = r.get("payload");
                    let payload: serde_json::Value = serde_json::from_str(&payload_str)
                        .unwrap_or_else(|_| serde_json::json!({}));

                    sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                        .bind(&id)
                        .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                    Some((id, tenant_id, payload))
                } else {
                    None
                };
                tx.commit().await.map_err(|e| e.to_string())?;
                res
            }
            crate::db::DbStore::Sqlite(pool) => {
                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    SELECT id, tenant_id, payload
                    FROM ohc_job_queue
                    WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP AND job_type = 'inquiry_intake_agent'
                    ORDER BY next_retry_at ASC, created_at ASC
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let res = if let Some(r) = row {
                    use sqlx::Row;
                    let id: String = r.get("id");
                    let tenant_id: String = r.get("tenant_id");
                    let payload_str: String = r.get("payload");
                    let payload: serde_json::Value = serde_json::from_str(&payload_str)
                        .unwrap_or_else(|_| serde_json::json!({}));

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
            let inquiry_id = payload
                .get("inquiry_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let raw_message = payload
                .get("raw_message")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            if inquiry_id.is_empty() {
                self.mark_job_failed(&job_id).await;
                return Ok(true);
            }

            let mut pg_tx_opt = None;
            let mut sqlite_tx_opt = None;

            if matches!(&self.db.store, crate::db::DbStore::Postgres) {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                if let Err(_e) =
                    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await
                {
                    self.mark_job_failed(&job_id).await;
                    let _ = tx.commit().await;
                    return Ok(true);
                }
                pg_tx_opt = Some(tx);
            } else if let crate::db::DbStore::Sqlite(pool) = &self.db.store {
                let tx = pool.begin().await.map_err(|e| e.to_string())?;
                sqlite_tx_opt = Some(tx);
            }

            // Load OHC-03 Policy
            let policy_json: Option<String> =
                if matches!(&self.db.store, crate::db::DbStore::Postgres) {
                    sqlx::query_scalar(
                        "SELECT settings->>'ohc_03_policy' FROM tenants WHERE id = $1",
                    )
                    .bind(&tenant_id)
                    .fetch_optional(&mut **pg_tx_opt.as_mut().unwrap())
                    .await
                    .unwrap_or(None)
                } else {
                    sqlx::query_scalar(
                    "SELECT json_extract(settings, '$.ohc_03_policy') FROM tenants WHERE id = ?",
                )
                .bind(&tenant_id)
                .fetch_optional(&mut **sqlite_tx_opt.as_mut().unwrap())
                .await
                .unwrap_or(None)
                };

            let policy_text = match policy_json {
                Some(text) if !text.trim().is_empty() => text,
                _ => {
                    tracing::warn!("OHC-03 Policy is unavailable. Failing inquiry intake job.");
                    if matches!(&self.db.store, crate::db::DbStore::Postgres) {
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                        .bind(&job_id)
                        .execute(&mut **pg_tx_opt.as_mut().unwrap()).await;
                    } else {
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&job_id)
                        .execute(&mut **sqlite_tx_opt.as_mut().unwrap()).await;
                    }
                    if let Some(tx) = pg_tx_opt.take() {
                        let _ = tx.commit().await;
                    }
                    if let Some(tx) = sqlite_tx_opt.take() {
                        let _ = tx.commit().await;
                    }
                    return Ok(true);
                }
            };

            #[derive(sqlx::FromRow, serde::Serialize)]
            struct Service {
                id: Uuid,
                name: String,
                base_price_cents: i64,
            }

            let services = if matches!(&self.db.store, crate::db::DbStore::Postgres) {
                sqlx::query_as::<_, Service>(
                    "SELECT id, name, base_price_cents FROM service_items WHERE tenant_id = $1",
                )
                .bind(&tenant_id)
                .fetch_all(&mut **pg_tx_opt.as_mut().unwrap())
                .await
                .unwrap_or_default()
            } else {
                sqlx::query_as::<_, Service>(
                    "SELECT id, name, base_price_cents FROM service_items WHERE tenant_id = ?",
                )
                .bind(&tenant_id)
                .fetch_all(&mut **sqlite_tx_opt.as_mut().unwrap())
                .await
                .unwrap_or_default()
            };

            let catalog_json =
                serde_json::to_string(&services).unwrap_or_else(|_| "[]".to_string());

            let system_prompt = format!(
                "You are an expert lead qualification AI. You must adhere to the following business policy when evaluating leads:
<policy>
{}
</policy>

You have the following service catalog:
{}

Given a customer inquiry, evaluate if it complies with the policy constraints and matches a service we offer.
Respond with a JSON object containing exactly one boolean field: 'in_scope'. Set it to true if the lead is valid and should be quoted, false otherwise.",
                policy_text, catalog_json
            );

            let llm = Arc::new(AdapterLlm {});
            let req = ChatRequest {
                model: "default-model".to_string(),
                system: system_prompt,
                messages: vec![Message::user(raw_message.to_string())],
                temperature: 0.1,
                max_tokens: 500,
                tools: vec![],
            };

            let is_in_scope = match llm.chat(req).await {
                Ok(r) => {
                    let json_str = r.message.content.trim();
                    let json_str = json_str.strip_prefix("```json").unwrap_or(json_str);
                    let json_str = json_str.strip_suffix("```").unwrap_or(json_str).trim();
                    if let Ok(resp) = serde_json::from_str::<QualificationResponse>(json_str) {
                        resp.in_scope
                    } else {
                        false
                    }
                }
                Err(_) => {
                    if matches!(&self.db.store, crate::db::DbStore::Postgres) {
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                        .bind(&job_id)
                        .execute(&mut **pg_tx_opt.as_mut().unwrap()).await;
                    } else {
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&job_id)
                        .execute(&mut **sqlite_tx_opt.as_mut().unwrap()).await;
                    }
                    if let Some(tx) = pg_tx_opt.take() {
                        let _ = tx.commit().await;
                    }
                    if let Some(tx) = sqlite_tx_opt.take() {
                        let _ = tx.commit().await;
                    }
                    return Ok(true);
                }
            };

            if is_in_scope {
                // Transition inquiry to QUOTED and queue draft quote
                let quote_id = Uuid::new_v4();

                let is_err = if matches!(&self.db.store, crate::db::DbStore::Postgres) {
                    let tx = pg_tx_opt.as_mut().unwrap();
                    let r1 = sqlx::query("UPDATE inquiries SET status = 'QUOTED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
                        .bind(inquiry_id).bind(&tenant_id).execute(&mut **tx).await;
                    let r2 = sqlx::query("INSERT INTO quotes (id, tenant_id, status, total_amount_cents, required_deposit_cents) VALUES ($1, $2, 'DRAFTING', 0, 0)")
                        .bind(quote_id.to_string()).bind(&tenant_id).execute(&mut **tx).await;

                    let draft_job_id = Uuid::new_v4().to_string();
                    let draft_payload = serde_json::json!({
                        "quote_id": quote_id.to_string(),
                        "inquiry": raw_message
                    });
                    let r3 = sqlx::query("INSERT INTO ohc_job_queue (id, parent_task_id, job_type, payload, status, next_retry_at, tenant_id) VALUES ($1, '', 'draft_quote_agent', $2, 'PENDING', CURRENT_TIMESTAMP, $3)")
                        .bind(draft_job_id).bind(&draft_payload).bind(&tenant_id).execute(&mut **tx).await;

                    let r4 = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                        .bind(&job_id).execute(&mut **tx).await;

                    r1.is_err() || r2.is_err() || r3.is_err() || r4.is_err()
                } else {
                    let tx = sqlite_tx_opt.as_mut().unwrap();
                    let r1 = sqlx::query("UPDATE inquiries SET status = 'QUOTED', updated_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?")
                        .bind(inquiry_id).bind(&tenant_id).execute(&mut **tx).await;
                    let r2 = sqlx::query("INSERT INTO quotes (id, tenant_id, status, total_amount_cents, required_deposit_cents) VALUES (?, ?, 'DRAFTING', 0, 0)")
                        .bind(quote_id.to_string()).bind(&tenant_id).execute(&mut **tx).await;

                    let draft_job_id = Uuid::new_v4().to_string();
                    let draft_payload = serde_json::json!({
                        "quote_id": quote_id.to_string(),
                        "inquiry": raw_message
                    });
                    let r3 = sqlx::query("INSERT INTO ohc_job_queue (id, parent_task_id, job_type, payload, status, next_retry_at, tenant_id) VALUES (?, '', 'draft_quote_agent', ?, 'PENDING', CURRENT_TIMESTAMP, ?)")
                        .bind(draft_job_id).bind(&draft_payload).bind(&tenant_id).execute(&mut **tx).await;

                    let r4 = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&job_id).execute(&mut **tx).await;

                    r1.is_err() || r2.is_err() || r3.is_err() || r4.is_err()
                };

                if is_err {
                    self.mark_job_failed(&job_id).await;
                }
            } else {
                // Out of scope, mark inquiry as closed
                let is_err = if matches!(&self.db.store, crate::db::DbStore::Postgres) {
                    let tx = pg_tx_opt.as_mut().unwrap();
                    let r1 = sqlx::query("UPDATE inquiries SET status = 'CLOSED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
                        .bind(inquiry_id).bind(&tenant_id).execute(&mut **tx).await;
                    let r2 = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                        .bind(&job_id).execute(&mut **tx).await;
                    r1.is_err() || r2.is_err()
                } else {
                    let tx = sqlite_tx_opt.as_mut().unwrap();
                    let r1 = sqlx::query("UPDATE inquiries SET status = 'CLOSED', updated_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?")
                        .bind(inquiry_id).bind(&tenant_id).execute(&mut **tx).await;
                    let r2 = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&job_id).execute(&mut **tx).await;
                    r1.is_err() || r2.is_err()
                };

                if is_err {
                    self.mark_job_failed(&job_id).await;
                }
            }

            if let Some(tx) = pg_tx_opt.take() {
                let _ = tx.commit().await;
            }
            if let Some(tx) = sqlite_tx_opt.take() {
                let _ = tx.commit().await;
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn mark_job_failed(&self, job_id: &str) {
        if matches!(&self.db.store, crate::db::DbStore::Postgres) {
            let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                .bind(job_id)
                .execute(&self.db.pool).await;
        } else {
            let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                .bind(job_id)
                .execute(&self.db.pool).await;
        }
    }
}
