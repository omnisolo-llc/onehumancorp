use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use uuid::Uuid;
use ohc_builtin_agent::llm::LlmClient;
use sqlx::Row;

pub struct ProactiveAnalysisWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
    pub llm: Option<Arc<dyn LlmClient>>,
}

impl ProactiveAnalysisWorker {
    pub fn new(db: Arc<DB>) -> Self {
        let key = std::env::var("OHC_LLM_API_KEY")
            .or_else(|_| std::env::var("OPENAI_API_KEY"))
            .unwrap_or_default();

        let mut llm: Option<Arc<dyn LlmClient>> = None;
        if !key.is_empty() {
            let endpoint = std::env::var("OPENAI_BASE_URL")
                .or_else(|_| std::env::var("OHC_OPENAI_BASE_URL"))
                .or_else(|_| std::env::var("OHC_LLM_BASE_URL"))
                .or_else(|_| std::env::var("OHC_LLM_ENDPOINT"))
                .ok();

            let model = std::env::var("OHC_LLM_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string());

            let config = if let Some(endpoint) = endpoint {
                ohc_builtin_agent::llm::openai::OpenAIClientConfig::openai_compatible(key, endpoint, Some(model))
            } else {
                ohc_builtin_agent::llm::openai::OpenAIClientConfig::openai(key)
            };
            llm = Some(Arc::new(ohc_builtin_agent::llm::openai::OpenAIClient::from_config(config)));
        }

        Self {
            db,
            poll_interval: Duration::from_secs(60),
            llm,
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;
        let llm = self.llm.clone();

        tokio::spawn(async move {
            let _pool = db.pool.clone();
            loop {
                tokio::time::sleep(interval_duration).await;
                let _ = Self::run_analysis_for_all_tenants(&db, &llm).await;
            }
        });
    }

    pub async fn run_analysis_for_all_tenants(db: &Arc<DB>, llm: &Option<Arc<dyn LlmClient>>) -> Result<(), String> {
        let tenants = match &db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
                ::server_common::auth_utils::set_system_context(&mut *tx).await.map_err(|e| e.to_string())?;
                let rows = sqlx::query("SELECT id FROM tenants")
                    .fetch_all(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                tx.commit().await.map_err(|e| e.to_string())?;
                rows.into_iter().map(|r| r.get::<String, _>("id")).collect::<Vec<String>>()
            },
            crate::db::DbStore::Sqlite(pool) => {
                let rows = sqlx::query("SELECT id FROM tenants")
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                rows.into_iter().map(|r| r.get::<String, _>("id")).collect::<Vec<String>>()
            }
        };

        for tenant_id in tenants {
            let _ = Self::run_analysis_for_tenant(db, tenant_id, llm).await;
        }

        Ok(())
    }

    pub async fn run_analysis_for_tenant(db: &Arc<DB>, tenant_id: String, llm: &Option<Arc<dyn LlmClient>>) -> Result<(), String> {
        // Find if there is an existing pending proactive insight for this tenant to avoid spamming
        let has_pending = match &db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
                ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| e.to_string())?;
                let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM triage_items WHERE tenant_id = $1 AND source = 'Proactive Context Agent' AND status = 'pending'")
                    .bind(&tenant_id)
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                tx.commit().await.map_err(|e| e.to_string())?;
                count.0 > 0
            },
            crate::db::DbStore::Sqlite(pool) => {
                let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM triage_items WHERE tenant_id = ? AND source = 'Proactive Context Agent' AND status = 'pending'")
                    .bind(&tenant_id)
                    .fetch_one(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                count.0 > 0
            }
        };

        if has_pending {
            return Ok(());
        }

        let mut context = String::from("You have 2 estimates pending from yesterday. Tap to review drafted follow-up messages.");
        let mut action_type = String::from("Draft Approval");
        let mut action_payload = String::from("Send follow-up to Alice: 'Hi Alice, just checking if you had any questions on the estimate?'");

        if let Some(llm_client) = llm {
            // Use LLM to generate the prompt
            let req = ohc_builtin_agent::types::ChatRequest {
                model: std::env::var("OHC_LLM_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string()),
                system: "You are a proactive context agent. Generate a JSON containing 'context', 'action_type', and 'action_payload' fields representing a high-value operational insight for a business owner based on a simulation of pending tasks.".to_string(),
                messages: vec![
                    ohc_builtin_agent::types::Message {
                        role: ohc_builtin_agent::types::Role::User,
                        content: "Generate insight.".to_string(),
                        tool_calls: vec![],
                        tool_results: vec![],
                        response_id: None,
                        previous_response_id: None,
                    }
                ],
                tools: vec![],
                max_tokens: 1000,
                temperature: 0.5_f32,
            };

            if let Ok(resp) = llm_client.chat(req).await {
                let msg = resp.message;
                let content = msg.content;
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(ctx) = parsed.get("context").and_then(|v| v.as_str()) {
                        context = ctx.to_string();
                    }
                    if let Some(at) = parsed.get("action_type").and_then(|v| v.as_str()) {
                        action_type = at.to_string();
                    }
                    if let Some(ap) = parsed.get("action_payload").and_then(|v| v.as_str()) {
                        action_payload = ap.to_string();
                    }
                }
            }
        }

        let triage_id = Uuid::new_v4().to_string();
        let action_id = Uuid::new_v4().to_string();

        match &db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
                ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| e.to_string())?;

                sqlx::query("INSERT INTO triage_items (id, tenant_id, source, priority, context, status) VALUES ($1, $2, 'Proactive Context Agent', 'High', $3, 'pending') ON CONFLICT (id) DO NOTHING")
                    .bind(&triage_id)
                    .bind(&tenant_id)
                    .bind(&context)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                sqlx::query("INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO NOTHING")
                    .bind(&action_id)
                    .bind(&triage_id)
                    .bind(&tenant_id)
                    .bind(&action_type)
                    .bind(&action_payload)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;
            },
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query("INSERT INTO triage_items (id, tenant_id, source, priority, context, status) VALUES (?, ?, 'Proactive Context Agent', 'High', ?, 'pending') ON CONFLICT (id) DO NOTHING")
                    .bind(&triage_id)
                    .bind(&tenant_id)
                    .bind(&context)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;

                sqlx::query("INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload) VALUES (?, ?, ?, ?, ?) ON CONFLICT (id) DO NOTHING")
                    .bind(&action_id)
                    .bind(&triage_id)
                    .bind(&tenant_id)
                    .bind(&action_type)
                    .bind(&action_payload)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }
}
