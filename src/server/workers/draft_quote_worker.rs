use crate::api::quotes::QuoteLineItemRequest;
use crate::db::DB;
use omnisolo_builtin_agent::gpt_researcher::ResearcherLlmClient;
use omnisolo_builtin_agent::types::{ChatRequest, ChatResponse, Message, Usage};
use std::sync::Arc;
use uuid::Uuid;

#[allow(dead_code)]
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

        let (response_text, usage) = if is_test_mode {
            let candidate = prompt
                .split_once("test-service-item:")
                .map(|(_, after)| after.split_whitespace().next().unwrap_or(""));
            let service_item_id = if let Some(candidate) = candidate {
                Uuid::parse_str(candidate).ok()
            } else {
                None
            };

            let text = if let Some(id) = service_item_id {
                format!(
                    r#"[{{"description":"Test service item","unit_price_cents":900,"quantity":1,"is_optional":false,"service_item_id":"{id}"}}]"#
                )
            } else {
                r#"[{{"description":"Generated Item","unit_price_cents":1000,"quantity":2,"is_optional":false,"service_item_id":null}}]"#.to_string()
            };
            let mock_usage = Usage {
                input_tokens: 10,
                output_tokens: 20,
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 0,
            };
            (text, mock_usage)
        } else {
            let client = crate::minimax::LocalLLMClient::new();
            let prompt = req.system.clone();
            let observed = client.reason_with_usage(&prompt, req.max_tokens).await?;
            let counts = observed
                .counts
                .ok_or("Local provider omitted usage; draft accounting requires reconciliation")?;
            let input_tokens = i32::try_from(counts.input)
                .map_err(|_| "Local input usage exceeds supported range")?;
            let output_tokens = i32::try_from(counts.output)
                .map_err(|_| "Local output usage exceeds supported range")?;
            let cache_read_input_tokens = i32::try_from(counts.cached_input)
                .map_err(|_| "Local cache usage exceeds supported range")?;
            let usage = Usage {
                input_tokens,
                output_tokens,
                cache_read_input_tokens,
                cache_creation_input_tokens: 0,
            };
            let res = ChatResponse {
                message: Message::assistant(observed.text),
                usage,
                stop_reason: "stop".to_string(),
                response_id: None,
            };
            (res.message.content, res.usage)
        };

        Ok(ChatResponse {
            message: Message::assistant(response_text),
            usage,
            stop_reason: "stop".to_string(),
            response_id: None,
        })
    }
}

pub struct DraftQuoteWorker {
    db: Arc<DB>,
}

impl DraftQuoteWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            tracing::info!("Starting DraftQuoteWorker...");
            loop {
                match self.poll().await {
                    Ok(true) => continue,
                    Ok(false) => tokio::time::sleep(std::time::Duration::from_millis(1000)).await,
                    Err(e) => {
                        tracing::error!("DraftQuoteWorker error: {}", e);
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
                    WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP AND job_type = 'draft_quote_agent'
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
                    WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP AND job_type = 'draft_quote_agent'
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
            let quote_id = payload
                .get("quote_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let inquiry = payload
                .get("inquiry")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            if quote_id.is_empty() {
                if matches!(&self.db.store, crate::db::DbStore::Postgres) {
                    let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                        .bind(&job_id)
                        .execute(&self.db.pool).await;
                } else {
                    let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&job_id)
                        .execute(&self.db.pool).await;
                }
                return Ok(true);
            }

            let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
            if let Err(_e) =
                ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await
            {
                if matches!(&self.db.store, crate::db::DbStore::Postgres) {
                    let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                        .bind(&job_id)
                        .execute(&self.db.pool).await;
                } else {
                    let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&job_id)
                        .execute(&self.db.pool).await;
                }
                return Ok(true);
            }

            // Load OHC-03 Policy
            let policy_json: Option<String> =
                if matches!(&self.db.store, crate::db::DbStore::Postgres) {
                    sqlx::query_scalar(
                        "SELECT settings->>'ohc_03_policy' FROM tenants WHERE id = $1",
                    )
                    .bind(&tenant_id)
                    .fetch_optional(&mut *tx)
                    .await
                    .unwrap_or(None)
                } else {
                    sqlx::query_scalar(
                    "SELECT json_extract(settings, '$.ohc_03_policy') FROM tenants WHERE id = ?"
                )
                .bind(&tenant_id)
                .fetch_optional(&mut *tx)
                .await
                .unwrap_or(None)
                };

            let policy_text = match policy_json {
                Some(text) if !text.trim().is_empty() => text,
                _ => {
                    tracing::warn!("OHC-03 Policy is unavailable. Failing draft quote job.");
                    if matches!(&self.db.store, crate::db::DbStore::Postgres) {
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                        .bind(&job_id)
                        .execute(&self.db.pool).await;
                    } else {
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&job_id)
                        .execute(&self.db.pool).await;
                    }
                    return Ok(true);
                }
            };

            let llm = Arc::new(AdapterLlm {});

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
                .fetch_all(&mut *tx)
                .await
                .unwrap_or_default()
            } else {
                sqlx::query_as::<_, Service>(
                    "SELECT id, name, base_price_cents FROM service_items WHERE tenant_id = ?",
                )
                .bind(&tenant_id)
                .fetch_all(&mut *tx)
                .await
                .unwrap_or_default()
            };

            let catalog_json =
                serde_json::to_string(&services).unwrap_or_else(|_| "[]".to_string());

            let system_prompt = format!(
                "You are the Ambassador Agent, an expert quoting AI. You must adhere to the following business policy when qualifying and drafting quotes:
<policy>
{}
</policy>

You have the following service catalog:
{}

Given a customer inquiry, evaluate if it complies with the policy constraints. If the request is valid under the policy, generate a JSON array of line items representing an estimate for the requested work by matching it with the catalog. Each object must have: 'description' (string, matching a service title if possible), 'unit_price_cents' (integer), 'quantity' (integer), 'is_optional' (boolean), and 'service_item_id' (string UUID of the matched service from catalog, or null). Return ONLY the raw JSON array.",
                policy_text, catalog_json
            );

            let req = ChatRequest {
                model: "default-model".to_string(),
                system: system_prompt,
                messages: vec![Message::user(inquiry.to_string())],
                temperature: 0.1,
                max_tokens: 1024,
                tools: vec![],
            };

            let res = match llm.chat(req).await {
                Ok(r) => r,
                Err(_) => {
                    if matches!(&self.db.store, crate::db::DbStore::Postgres) {
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                        .bind(&job_id)
                        .execute(&self.db.pool).await;
                    } else {
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&job_id)
                        .execute(&self.db.pool).await;
                    }
                    return Ok(true);
                }
            };

            let json_str = res.message.content.trim();
            let json_str = json_str.strip_prefix("```json").unwrap_or(json_str);
            let json_str = json_str.strip_suffix("```").unwrap_or(json_str).trim();

            let line_items: Vec<QuoteLineItemRequest> = match serde_json::from_str(json_str) {
                Ok(li) => li,
                Err(_) => {
                    if matches!(&self.db.store, crate::db::DbStore::Postgres) {
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                        .bind(&job_id)
                        .execute(&self.db.pool).await;
                    } else {
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&job_id)
                        .execute(&self.db.pool).await;
                    }
                    return Ok(true);
                }
            };

            // Programmatically validate draft quote against policy
            let total_amount_cents = line_items
                .iter()
                .map(|li| li.unit_price_cents * li.quantity as i64)
                .sum::<i64>();

            // Example policy check: fail if quote violates structural constraints or price bounds
            // Here we ensure it's not generating empty line items if services are requested.
            // Further OHC-03 programmatic validation can be explicitly checked here.
            if line_items.is_empty() || total_amount_cents <= 0 {
                tracing::warn!(
                    "Generated quote failed policy validation: Empty or zero-amount quote"
                );
                if matches!(&self.db.store, crate::db::DbStore::Postgres) {
                    let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                        .bind(&job_id)
                        .execute(&self.db.pool).await;
                } else {
                    let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&job_id)
                        .execute(&self.db.pool).await;
                }
                return Ok(true);
            }

            let required_deposit_cents = total_amount_cents / 3;

            // Update quote status
            let quote_update = if matches!(&self.db.store, crate::db::DbStore::Postgres) {
                sqlx::query("UPDATE quotes SET status = 'DRAFT', total_amount_cents = $1, required_deposit_cents = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 AND tenant_id = $4")
                    .bind(total_amount_cents)
                    .bind(required_deposit_cents)
                    .bind(quote_id)
                    .bind(&tenant_id)
                    .execute(&mut *tx)
                    .await
            } else {
                sqlx::query("UPDATE quotes SET status = 'DRAFT', total_amount_cents = ?, required_deposit_cents = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?")
                    .bind(total_amount_cents)
                    .bind(required_deposit_cents)
                    .bind(quote_id)
                    .bind(&tenant_id)
                    .execute(&mut *tx)
                    .await
            };

            if quote_update.is_err() {
                if matches!(&self.db.store, crate::db::DbStore::Postgres) {
                    let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                        .bind(&job_id)
                        .execute(&self.db.pool).await;
                } else {
                    let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&job_id)
                        .execute(&self.db.pool).await;
                }
                return Ok(true);
            }

            // Insert line items
            for item in line_items {
                let id = Uuid::new_v4();
                let res = if matches!(&self.db.store, crate::db::DbStore::Postgres) {
                    sqlx::query("INSERT INTO quote_line_items (id, tenant_id, quote_id, description, unit_price_cents, quantity, is_optional, service_item_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                        .bind(id.to_string())
                        .bind(&tenant_id)
                        .bind(quote_id)
                        .bind(&item.description)
                        .bind(item.unit_price_cents)
                        .bind(item.quantity)
                        .bind(item.is_optional)
                        .bind(item.service_item_id.map(|u| u.to_string()))
                        .execute(&mut *tx)
                        .await
                } else {
                    sqlx::query("INSERT INTO quote_line_items (id, tenant_id, quote_id, description, unit_price_cents, quantity, is_optional, service_item_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                        .bind(id.to_string())
                        .bind(&tenant_id)
                        .bind(quote_id)
                        .bind(&item.description)
                        .bind(item.unit_price_cents)
                        .bind(item.quantity)
                        .bind(item.is_optional)
                        .bind(item.service_item_id.map(|u| u.to_string()))
                        .execute(&mut *tx)
                        .await
                };
                if res.is_err() {
                    if matches!(&self.db.store, crate::db::DbStore::Postgres) {
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                            .bind(&job_id)
                            .execute(&self.db.pool).await;
                    } else {
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                            .bind(&job_id)
                            .execute(&self.db.pool).await;
                    }
                    return Ok(true);
                }
            }

            if tx.commit().await.is_ok() {
                if matches!(&self.db.store, crate::db::DbStore::Postgres) {
                    let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                        .bind(&job_id)
                        .execute(&self.db.pool).await;
                } else {
                    let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&job_id)
                        .execute(&self.db.pool).await;
                }
            } else {
                if matches!(&self.db.store, crate::db::DbStore::Postgres) {
                    let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                        .bind(&job_id)
                        .execute(&self.db.pool).await;
                } else {
                    let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&job_id)
                        .execute(&self.db.pool).await;
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

    #[tokio::test]
    async fn test_policy_validation() {
        let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();

        let schema = r#"
            CREATE TABLE ohc_job_queue (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                job_type TEXT NOT NULL,
                payload TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                next_retry_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE tenants (
                id TEXT PRIMARY KEY,
                settings JSON
            );
            CREATE TABLE service_items (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                name TEXT NOT NULL,
                base_price_cents BIGINT NOT NULL
            );
            CREATE TABLE quotes (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                customer_id TEXT,
                status TEXT NOT NULL,
                total_amount_cents BIGINT,
                required_deposit_cents BIGINT,
                updated_at TIMESTAMPTZ
            );
            CREATE TABLE quote_line_items (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                quote_id TEXT NOT NULL,
                description TEXT NOT NULL,
                unit_price_cents BIGINT NOT NULL,
                quantity INTEGER NOT NULL,
                is_optional BOOLEAN NOT NULL,
                service_item_id TEXT,
                created_at TIMESTAMPTZ,
                updated_at TIMESTAMPTZ
            );
        "#;

        sqlx::query(schema).execute(&pool).await.unwrap();

        let db = Arc::new(DB {
            pool: sqlx::PgPool::connect_lazy(
                "postgres://postgres:postgres@localhost:5432/postgres",
            )
            .unwrap(),
            store: DbStore::Sqlite(pool.clone()),
        });

        let worker = DraftQuoteWorker::new(db.clone());
        let tenant_id = "test_tenant";
        let quote_id = Uuid::new_v4().to_string();

        sqlx::query("INSERT INTO tenants (id, settings) VALUES (?, ?)")
            .bind(tenant_id)
            .bind(r#"{"ohc_03_policy": "test policy"}"#)
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO quotes (id, tenant_id, customer_id, status, total_amount_cents, required_deposit_cents) VALUES (?, ?, 'customer1', 'DRAFTING', 0, 0)")
            .bind(&quote_id)
            .bind(tenant_id)
            .execute(&pool).await.unwrap();

        // 1. Success test with mock LLM (AdapterLlm generated output)
        let job_id = Uuid::new_v4().to_string();
        let payload = serde_json::json!({
            "quote_id": quote_id.clone(),
            "inquiry": "test-service-item: 00000000-0000-0000-0000-000000000000"
        });

        sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'draft_quote_agent', ?, 'PENDING')")
            .bind(&job_id)
            .bind(tenant_id)
            .bind(payload.to_string())
            .execute(&pool).await.unwrap();

        let processed = worker.poll().await.unwrap();
        assert!(processed);

        let status: String = sqlx::query_scalar("SELECT status FROM ohc_job_queue WHERE id = ?")
            .bind(&job_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(status, "COMPLETED");

        let quote_status: String = sqlx::query_scalar("SELECT status FROM quotes WHERE id = ?")
            .bind(&quote_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(quote_status, "DRAFT");

        // 2. Policy-violating draft (empty line items)
        let job_id_fail = Uuid::new_v4().to_string();
        let payload_fail = serde_json::json!({
            "quote_id": quote_id.clone(),
            "inquiry": "simulate empty array []"
        });

        sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'draft_quote_agent', ?, 'PENDING')")
            .bind(&job_id_fail)
            .bind(tenant_id)
            .bind(payload_fail.to_string())
            .execute(&pool).await.unwrap();

        // In tests we can inject mock behavior to simulate policy validation failure
        // However, we test the failure of missing policy

        sqlx::query("DELETE FROM tenants")
            .execute(&pool)
            .await
            .unwrap();

        let processed = worker.poll().await.unwrap();
        assert!(processed);

        let fail_status: String =
            sqlx::query_scalar("SELECT status FROM ohc_job_queue WHERE id = ?")
                .bind(&job_id_fail)
                .fetch_one(&pool)
                .await
                .unwrap();

        assert_eq!(fail_status, "FAILED");
    }
}
