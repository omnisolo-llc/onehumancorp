use std::sync::Arc;
use crate::db::DB;
use uuid::Uuid;
use ohc_builtin_agent::llm::LlmClient;

fn build_newsletter_llm_client() -> Option<Arc<dyn LlmClient>> {
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

    let mut config = if let Some(endpoint) = endpoint {
        ohc_builtin_agent::llm::openai::OpenAIClientConfig::openai_compatible(key, endpoint, Some(model.clone()))
    } else {
        ohc_builtin_agent::llm::openai::OpenAIClientConfig::openai(key)
    };
    Some(Arc::new(ohc_builtin_agent::llm::openai::OpenAIClient::from_config(config)))
}

pub struct NewsletterWorker {
    pub db: Arc<DB>,
}

impl NewsletterWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(86400 * 7)); // Weekly CRON

            // Delay the first execution by a bit to avoid running instantly on every server restart
            tokio::time::sleep(std::time::Duration::from_secs(300)).await;

            loop {
                interval.tick().await;

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
                    // Collect context: new products, recent bookings, etc.
                    let products_context = match &db.store {
                        crate::db::DbStore::Postgres => {
                            sqlx::query_scalar::<_, String>(
                                "SELECT STRING_AGG(name, ', ') FROM catalog_items WHERE tenant_id = $1 AND created_at > NOW() - INTERVAL '7 days'"
                            )
                            .bind(&tenant_id)
                            .fetch_one(&db.pool)
                            .await
                            .unwrap_or_default()
                        },
                        crate::db::DbStore::Sqlite(_) => {
                            sqlx::query_scalar::<_, String>(
                                "SELECT GROUP_CONCAT(name, ', ') FROM catalog_items WHERE tenant_id = $1 AND created_at > datetime('now', '-7 days')"
                            )
                            .bind(&tenant_id)
                            .fetch_one(&db.pool)
                            .await
                            .unwrap_or_default()
                        }
                    };

                    let context = format!("New items this week: {}", products_context);

                    let mut subject = "Weekly Update from OHC".to_string();
                    let mut body_markdown = format!("Hello!\n\nHere are some highlights from the past week:\n\n{}\n\nCheers!", context);
                    let mut body_html = format!("<h1>Hello!</h1><p>Here are some highlights from the past week:</p><p>{}</p><p>Cheers!</p>", context);

                    let llm = build_newsletter_llm_client();
                    if let Some(llm_client) = llm {
                        let system_prompt = "You are a marketing assistant for a small business. Draft a short, engaging weekly newsletter based on the context provided. Respond ONLY with a JSON object containing three keys: 'subject', 'body_markdown', and 'body_html'. Keep it concise and mobile-friendly.";
                        let user_prompt = format!("Context: {}", context);

                        let req = ohc_builtin_agent::types::ChatRequest {
                            model: "default".to_string(),
                            system: system_prompt.to_string(),
                            messages: vec![ohc_builtin_agent::types::Message::user(user_prompt)],
                            tools: vec![],
                            max_tokens: 500,
                            temperature: 0.7,
                        };

                        if let Ok(resp) = llm_client.chat(req).await {
                                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&resp.message.content) {
                                    if let Some(s) = parsed.get("subject").and_then(|v| v.as_str()) { subject = s.to_string(); }
                                    if let Some(m) = parsed.get("body_markdown").and_then(|v| v.as_str()) { body_markdown = m.to_string(); }
                                    if let Some(h) = parsed.get("body_html").and_then(|v| v.as_str()) { body_html = h.to_string(); }
                                }
                        }
                    }

                    let draft_id = Uuid::new_v4().to_string();

                    match &db.store {
                        crate::db::DbStore::Postgres => {
                             let _ = sqlx::query("INSERT INTO newsletter_drafts (id, tenant_id, subject, body_html, body_markdown, status) VALUES ($1, $2, $3, $4, $5, 'Draft')")
                                .bind(&draft_id)
                                .bind(&tenant_id)
                                .bind(&subject)
                                .bind(&body_html)
                                .bind(&body_markdown)
                                .execute(&db.pool)
                                .await;
                        },
                        crate::db::DbStore::Sqlite(_) => {
                             let _ = sqlx::query("INSERT INTO newsletter_drafts (id, tenant_id, subject, body_html, body_markdown, status) VALUES ($1, $2, $3, $4, $5, 'Draft')")
                                .bind(&draft_id)
                                .bind(&tenant_id)
                                .bind(&subject)
                                .bind(&body_html)
                                .bind(&body_markdown)
                                .execute(&db.pool)
                                .await;
                        }
                    }

                    // Insert notification
                    let notif_id = Uuid::new_v4().to_string();
                    match &db.store {
                         crate::db::DbStore::Postgres => {
                             let _ = sqlx::query("INSERT INTO triage_items (id, tenant_id, source, priority, context, action_type, action_payload) VALUES ($1, $2, $3, $4, $5, $6, $7)")
                                .bind(&notif_id)
                                .bind(&tenant_id)
                                .bind("System")
                                .bind("Normal")
                                .bind("Weekly Newsletter Draft Ready! Review and send.")
                                .bind("Approve Newsletter")
                                .bind(&draft_id)
                                .execute(&db.pool)
                                .await;
                         },
                         crate::db::DbStore::Sqlite(_) => {
                             let _ = sqlx::query("INSERT INTO triage_items (id, tenant_id, source, priority, context, action_type, action_payload) VALUES ($1, $2, $3, $4, $5, $6, $7)")
                                .bind(&notif_id)
                                .bind(&tenant_id)
                                .bind("System")
                                .bind("Normal")
                                .bind("Weekly Newsletter Draft Ready! Review and send.")
                                .bind("Approve Newsletter")
                                .bind(&draft_id)
                                .execute(&db.pool)
                                .await;
                         }
                    }
                }
            }
        });
    }
}
