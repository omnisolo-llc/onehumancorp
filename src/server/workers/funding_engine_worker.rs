use std::sync::Arc;
use crate::db::DB;
use uuid::Uuid;
use ohc_builtin_agent::llm::LlmClient;
use sqlx::Row;

fn build_funding_llm_client() -> Option<Arc<dyn LlmClient>> {
    let key = std::env::var("OHC_LLM_API_KEY")
        .or_else(|_| std::env::var("OPENAI_API_KEY"))
        .unwrap_or_default();

    if key.is_empty() {
        return None;
    }

    let endpoint = std::env::var("OPENAI_BASE_URL")
        .or_else(|_| std::env::var("OHC_OPENAI_BASE_URL"))
        .or_else(|_| std::env::var("OHC_LLM_BASE_URL"))
        .or_else(|_| std::env::var("OHC_LLM_ENDPOINT"))
        .ok();

    let model = std::env::var("OHC_LLM_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string());

    let config = if let Some(endpoint) = endpoint {
        ohc_builtin_agent::llm::openai::OpenAIClientConfig::openai_compatible(key, endpoint, Some(model.clone()))
    } else {
        ohc_builtin_agent::llm::openai::OpenAIClientConfig::openai(key)
    };
    Some(Arc::new(ohc_builtin_agent::llm::openai::OpenAIClient::from_config(config)))
}

pub struct FundingEngineWorker {
    pub db: Arc<DB>,
}

impl FundingEngineWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(86400)); // Daily CRON

            // Delay the first execution
            tokio::time::sleep(std::time::Duration::from_secs(300)).await;

            let llm = build_funding_llm_client();

            loop {
                interval.tick().await;

                let tenants: Vec<String> = match &db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query_scalar("SELECT id::text FROM tenants")
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

                for tenant_id_str in tenants {
                    let tenant_id = match uuid::Uuid::parse_str(&tenant_id_str) {
                        Ok(id) => id,
                        Err(_) => continue,
                    };

                    let tenant_tier: String = match &db.store {
                        crate::db::DbStore::Postgres => {
                            sqlx::query_scalar("SELECT tier FROM tenants WHERE id = $1")
                                .bind(&tenant_id)
                                .fetch_one(&db.pool)
                                .await
                                .unwrap_or_else(|_| "Unknown".to_string())
                        },
                        crate::db::DbStore::Sqlite(_) => {
                            sqlx::query_scalar("SELECT tier FROM tenants WHERE id = $1")
                                .bind(&tenant_id.to_string())
                                .fetch_one(&db.pool)
                                .await
                                .unwrap_or_else(|_| "Unknown".to_string())
                        }
                    };

                    // Simulate matching logic...
                    // In a real scenario, this would query a vector DB for actual grants
                    let simulated_grant_name = "Downtown Revitalization Grant";
                    let simulated_amount = 10000.00;

                    if let Some(llm_client) = &llm {
                        let system_prompt = "You are a Legal Agent. Draft a 500-word grant application essay for a local business grant. Respond ONLY with a JSON object containing keys: 'grant_name', 'amount', 'draft_proposal_text', and 'deadline_iso8601'. Ensure the tone is professional, persuasive, and addresses how funds will be used for growth.";
                        let user_prompt = format!("Draft an application for '{}' for $10,000. The business tier is {}.", simulated_grant_name, tenant_tier);

                        let req = ohc_builtin_agent::types::ChatRequest {
                            model: "default".to_string(),
                            system: ::server_pricing::compression::reduce_tokens(&system_prompt),
                            messages: vec![ohc_builtin_agent::types::Message::user(::server_pricing::compression::reduce_tokens(&user_prompt))],
                            tools: vec![],
                            max_tokens: 1500,
                            temperature: 0.7,
                        };

                        let mut attempts = 0;
                        let max_retries = 3;
                        let mut ai_resp = None;

                        while attempts < max_retries {
                            let chat_future = llm_client.chat(req.clone());
                            match tokio::time::timeout(std::time::Duration::from_secs(60), chat_future).await {
                                Ok(Ok(resp)) => {
                                    ai_resp = Some(resp);
                                    break;
                                },
                                Ok(Err(_)) | Err(_) => {
                                    attempts += 1;
                                    if attempts < max_retries {
                                        tokio::time::sleep(std::time::Duration::from_secs(2u64.pow(attempts as u32))).await;
                                    } else {
                                        let notif_id = Uuid::new_v4();
                                        match &db.store {
                                            crate::db::DbStore::Postgres => {
                                                if let Err(e) = sqlx::query("INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES ($1, $2, $3, $4::jsonb, $5::jsonb, $6, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                                                    .bind(&notif_id.to_string())
                                                    .bind(&tenant_id.to_string())
                                                    .bind("Funding Engine Agent")
                                                    .bind(serde_json::json!({"description": "AI Agent Paused: The Funding Engine Agent"}))
                                                    .bind(serde_json::json!({"proposed_content": "System is paused. LLM API is unavailable."}))
                                                    .bind("PAUSED")
                                                    .execute(&db.pool)
                                                    .await {
                                                        tracing::error!("Failed to insert PAUSED state for Funding Engine Agent: {}", e);
                                                    }
                                            },
                                            crate::db::DbStore::Sqlite(_) => {
                                                if let Err(e) = sqlx::query("INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                                                    .bind(&notif_id.to_string())
                                                    .bind(&tenant_id.to_string())
                                                    .bind("Funding Engine Agent")
                                                    .bind(serde_json::json!({"description": "AI Agent Paused: The Funding Engine Agent"}).to_string())
                                                    .bind(serde_json::json!({"proposed_content": "System is paused. LLM API is unavailable."}).to_string())
                                                    .bind("PAUSED")
                                                    .execute(&db.pool)
                                                    .await {
                                                        tracing::error!("Failed to insert PAUSED state for Funding Engine Agent: {}", e);
                                                    }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if let Some(resp) = ai_resp {
                            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&resp.message.content) {
                                let grant_name = parsed.get("grant_name").and_then(|v| v.as_str()).unwrap_or(simulated_grant_name);
                                let amount = parsed.get("amount").and_then(|v| v.as_f64()).unwrap_or(simulated_amount);
                                let draft_proposal_text = parsed.get("draft_proposal_text").and_then(|v| v.as_str()).unwrap_or("Fallback draft text.");
                                let deadline_str = parsed.get("deadline_iso8601").and_then(|v| v.as_str()).unwrap_or("2025-12-31T23:59:59Z");

                                let deadline = chrono::DateTime::parse_from_rfc3339(deadline_str)
                                    .map(|dt| dt.with_timezone(&chrono::Utc))
                                    .unwrap_or_else(|_| chrono::Utc::now() + chrono::Duration::days(30));

                                let opp_id = Uuid::new_v4();

                                match &db.store {
                                    crate::db::DbStore::Postgres => {
                                        let amount_str = format!("{}", amount);
                                        if let Err(e) = sqlx::query("INSERT INTO funding_opportunities (id, tenant_id, grant_name, amount, draft_proposal_text, deadline) VALUES ($1, $2, $3, $4::numeric, $5, $6)")
                                            .bind(&opp_id)
                                            .bind(&tenant_id)
                                            .bind(grant_name)
                                            .bind(amount_str)
                                            .bind(draft_proposal_text)
                                            .bind(&deadline)
                                            .execute(&db.pool)
                                            .await;
                                    },
                                    crate::db::DbStore::Sqlite(_) => {
                                        let _ = sqlx::query("INSERT INTO funding_opportunities (id, tenant_id, grant_name, amount, draft_proposal_text, deadline) VALUES ($1, $2, $3, $4, $5, $6)")
                                            .bind(&opp_id.to_string())
                                            .bind(&tenant_id.to_string())
                                            .bind(grant_name)
                                            .bind(amount)
                                            .bind(draft_proposal_text)
                                            .bind(&deadline)
                                            .execute(&db.pool)
                                            .await;
                                    }
                                }

                                // Insert notification
                                let notif_id = Uuid::new_v4();
                                match &db.store {
                                     crate::db::DbStore::Postgres => {
                                         let _ = sqlx::query("INSERT INTO triage_items (id, tenant_id, source, priority, context, action_type, action_payload) VALUES ($1, $2, $3, $4, $5, $6, $7)")
                                            .bind(&notif_id)
                                            .bind(&tenant_id)
                                            .bind("Finance Dept")
                                            .bind("High")
                                            .bind(format!("✨ Finance Dept: We found a ${} grant you qualify for. Proposal drafted.", amount))
                                            .bind("Review Proposal")
                                            .bind(&opp_id.to_string())
                                            .execute(&db.pool)
                                            .await;
                                     },
                                     crate::db::DbStore::Sqlite(_) => {
                                         let _ = sqlx::query("INSERT INTO triage_items (id, tenant_id, source, priority, context, action_type, action_payload) VALUES ($1, $2, $3, $4, $5, $6, $7)")
                                            .bind(&notif_id.to_string())
                                            .bind(&tenant_id.to_string())
                                            .bind("Finance Dept")
                                            .bind("High")
                                            .bind(format!("✨ Finance Dept: We found a ${} grant you qualify for. Proposal drafted.", amount))
                                            .bind("Review Proposal")
                                            .bind(&opp_id.to_string())
                                            .execute(&db.pool)
                                            .await;
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
