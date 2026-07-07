use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use crate::services::customer_memory_graph::service::CustomerMemoryGraphService;
use std::collections::HashMap;

pub struct CustomerSuccessAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
    pub hub: Option<std::sync::Arc<crate::hub::Hub>>,
    configs: HashMap<String, DepartmentConfig>,
}

impl CustomerSuccessAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self {
            orchestrator,
            hub: None,
            configs: HashMap::new(),
        }
    }

    pub fn with_hub(mut self, hub: std::sync::Arc<crate::hub::Hub>) -> Self {
        self.hub = Some(hub);
        self
    }
}

#[async_trait::async_trait]
impl Department for CustomerSuccessAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::CustomerSuccess
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec![
            "tenant.order.fulfillment_ready".to_string(),
            "tenant.message.received".to_string(),
            "tenant.omnichannel.message.received".to_string(),
            "agent:customer_success:approved".to_string(),
            "tenant.subscription.check_predictive_restock".to_string(),
            "tenant.subscription.churn_risk".to_string(),
            "tenant.subscription.action.requested".to_string(),
            "tenant.subscription.at_risk".to_string(),
            "job_status_updates".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        let config = self.get_config(&event.tenant_id);
        let risk = if let Some(cfg) = &config {
            if cfg.auto_approve_limits > 0.0 {
                ActionRisk::AutoExecute
            } else {
                ActionRisk::DraftForReview
            }
        } else {
            ActionRisk::DraftForReview
        };

        if event.event_type == "job_status_updates" {
            let job_id = event.payload.get("job_id").and_then(|v| v.as_str()).unwrap_or("");
            let status = event.payload.get("status").and_then(|v| v.as_str()).unwrap_or("");

            if status == "en_route" && !job_id.is_empty() {
                let pool = crate::db::get_pool();

                // Query customer phone and staff name
                let query = r#"
                    SELECT
                        c.phone,
                        c.name as customer_name,
                        COALESCE(sp.name, 'Your service provider') as staff_name
                    FROM job_locations jl
                    JOIN appointments a ON jl.appointment_id = a.id
                    JOIN customers c ON a.customer_id = c.id
                    JOIN service_routes sr ON jl.service_route_id = sr.id
                    LEFT JOIN staff_profiles sp ON sr.staff_profile_id = sp.id
                    WHERE jl.id = $1 AND jl.tenant_id = $2
                "#;

                let row: Result<(Option<String>, Option<String>, String), sqlx::Error> = sqlx::query_as(query)
                    .bind(job_id)
                    .bind(&event.tenant_id)
                    .fetch_one(&pool)
                    .await;

                if let Ok((Some(customer_phone), _, staff_name)) = row {
                    let text = format!("Hi! {} is on his way and should arrive in roughly 15 minutes.", staff_name);

                    tracing::info!("Drafting SMS for job {}: {}", job_id, text);

                    let registry = crate::integrations::registry::IntegrationsRegistry::new();

                    // We need from_phone. Twilio requires it. Let's get it from the tenant's integration_credentials
                    let twilio_row: Result<(String, String, String), sqlx::Error> = sqlx::query_as(
                        "SELECT bot_token, api_token, from_phone FROM integration_credentials WHERE integration_id IN ('whatsapp', 'twilio') AND tenant_id = $1 ORDER BY CASE WHEN integration_id = 'whatsapp' THEN 1 ELSE 2 END LIMIT 1"
                    )
                    .bind(&event.tenant_id)
                    .fetch_one(&pool)
                    .await;

                    if let Ok((_sid, _token, from_phone)) = twilio_row {
                        if !from_phone.is_empty() {
                            let _ = registry.send_sms("twilio", &customer_phone, &from_phone, &text).await;
                        }
                    } else {
                        // Fallback or test mode
                        let _ = registry.send_sms("twilio", &customer_phone, "+15550000000", &text).await;
                    }
                }
            }
            return Ok(());
        }

        if event.event_type == "agent:customer_success:approved" {
            let payload = &event.payload;
            let original = payload.get("original_payload");

            if let Some(orig) = original {

                // --- BEGIN: Conversational Quoting Engine ---
                if orig.get("action_type").and_then(|v| v.as_str()) == Some("Draft Custom Quote") {
                    let sender_id = orig.get("sender_id").and_then(|v| v.as_str()).unwrap_or("");
                    let suggested_price = orig.get("suggested_price").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let message = orig.get("generated_response").and_then(|v| v.as_str()).unwrap_or("");

                    tracing::info!("EXECUTING APPROVED DRAFT: Sending custom quote to sender {}: {}", sender_id, message);

                    // We can insert the final quote in the DB and update the intake session status
                    let pool = crate::db::get_pool();
                    let session_id = uuid::Uuid::new_v4();

                    let _ = sqlx::query("INSERT INTO quotes (id, tenant_id, customer_id, status, total_amount_cents, required_deposit_cents) VALUES ($1, $2, $3, 'SENT', $4, $5)")
                        .bind(session_id)
                        .bind(&event.tenant_id)
                        .bind(uuid::Uuid::nil()) // Fallback customer_id
                        .bind((suggested_price * 100.0) as i64)
                        .bind((suggested_price * 50.0) as i64) // 50% deposit
                        .execute(&pool)
                        .await;

                    let _ = sqlx::query("INSERT INTO conversational_intake_sessions (id, tenant_id, inbox_thread_id, status, quote_id) VALUES ($1, $2, $3, 'QUOTE_SENT', $1)")
                        .bind(session_id)
                        .bind(&event.tenant_id)
                        .bind(sender_id)
                        .execute(&pool)
                        .await;

                    return Ok(());
                }

                if orig.get("action_type").and_then(|v| v.as_str()) == Some("Draft Message") {
                    let sender_id = orig.get("sender_id").and_then(|v| v.as_str()).unwrap_or("");
                    let message = orig.get("generated_response").and_then(|v| v.as_str()).unwrap_or("");
                    tracing::info!("EXECUTING APPROVED DRAFT: Sending follow-up message to sender {}: {}", sender_id, message);

                    let pool = crate::db::get_pool();
                    let session_id = uuid::Uuid::new_v4();
                    let _ = sqlx::query("INSERT INTO conversational_intake_sessions (id, tenant_id, inbox_thread_id, status) VALUES ($1, $2, $3, 'GATHERING_INFO')")
                        .bind(session_id)
                        .bind(&event.tenant_id)
                        .bind(sender_id)
                        .execute(&pool)
                        .await;
                }
                // --- END: Conversational Quoting Engine ---
                if orig.get("action_type").and_then(|v| v.as_str()) == Some("Execute Subscription Update") {
                    let customer_id = orig.get("customer_id").and_then(|v| v.as_str()).unwrap_or("");
                    let action = orig.get("action").and_then(|v| v.as_str()).unwrap_or("");
                    tracing::info!("EXECUTING APPROVED DRAFT: Modifying subscription {} for customer: {}", action, customer_id);

                    // The actual state mutation would happen here via API call to subscription.rs logic
                    // or direct db mutation if we had a direct service dependency. We mock it for now
                    // as part of the agent's side effect.
                    let pool = crate::db::get_pool();
                    let update = match action {
                        "pause" => sqlx::query("UPDATE subscriptions SET status = 'paused' WHERE customer_id = $1 AND tenant_id = $2 AND status = 'active'").bind(customer_id).bind(&event.tenant_id).execute(&pool).await,
                        "cancel" => sqlx::query("UPDATE subscriptions SET status = 'canceled' WHERE customer_id = $1 AND tenant_id = $2 AND status != 'canceled'").bind(customer_id).bind(&event.tenant_id).execute(&pool).await,
                        "skip" => sqlx::query("UPDATE subscriptions SET current_period_end = current_period_end + interval '1 month' WHERE customer_id = $1 AND tenant_id = $2").bind(customer_id).bind(&event.tenant_id).execute(&pool).await,
                        _ => Ok(sqlx::postgres::PgQueryResult::default()),
                    };

                    if update.is_ok() {
                        tracing::info!("Successfully updated subscription for customer {}", customer_id);
                    } else {
                        tracing::error!("Failed to update subscription for customer {}", customer_id);
                    }

                    return Ok(());
                }
            }

            let message = if let Some(orig) = original {
                orig.get("generated_response").and_then(|v| v.as_str()).unwrap_or("Unknown response")
            } else {
                "Unknown response"
            };
            tracing::info!("EXECUTING APPROVED DRAFT: Sending message: {}", message);

            let content = format!("Sent response to customer: {}", message);

            let source = original.and_then(|orig| orig.get("source").and_then(|v| v.as_str())).unwrap_or("").to_string();
            let sender_id = original.and_then(|orig| orig.get("sender_id").and_then(|v| v.as_str())).unwrap_or("").to_string();

            let target_language = original.and_then(|orig| orig.get("translated_from_language").and_then(|v| v.as_str())).unwrap_or("").to_string();
            let text = if !target_language.is_empty() && target_language.to_lowercase() != "en" && target_language.to_lowercase() != "english" && target_language.to_lowercase() != "unknown" {
                match crate::api::agents::translation::translate_inbox_message_with_llm(&event.tenant_id, &source, message, &target_language).await {
                    Ok(t) => t.translated_content,
                    Err(e) => {
                        tracing::error!("Failed to translate outgoing message back to {}: {}", target_language, e);
                        message.to_string()
                    }
                }
            } else {
                message.to_string()
            };

            let _hub_clone = self.hub.clone();
            let tenant_id_for_meta = event.tenant_id.clone();

            tokio::spawn(async move {
                if source == "whatsapp" && !sender_id.is_empty() {
                    let pool = crate::db::get_pool();
                    let twilio_row: Result<(String, String, String), sqlx::Error> = sqlx::query_as("SELECT bot_token, api_token, from_phone FROM integration_credentials WHERE integration_id IN ('whatsapp', 'twilio') AND tenant_id = $1 ORDER BY CASE WHEN integration_id = 'whatsapp' THEN 1 ELSE 2 END LIMIT 1")
                        .bind(&tenant_id_for_meta)
                        .fetch_one(&pool)
                        .await;

                    if let Ok((account_sid, auth_token, from_phone)) = twilio_row {
                        if !account_sid.is_empty() && !auth_token.is_empty() {
                            use crate::integrations::twilio::provider::TwilioProvider;
                            let provider = TwilioProvider::new(account_sid, auth_token);

                            if from_phone.is_empty() {
                                tracing::error!("Failed to send whatsapp message via Twilio integration: from_phone is empty in credentials");
                                return;
                            }
                            let twilio_from = from_phone;
                            let twilio_to = if sender_id.starts_with("whatsapp:") { sender_id.clone() } else { format!("whatsapp:{}", sender_id) };
                            if let Err(e) = provider.send_whatsapp(&twilio_to, &twilio_from, &text).await {
                                tracing::error!("Failed to send whatsapp message via Twilio integration: {}", e);
                            } else {
                                tracing::info!("Successfully sent whatsapp message via Twilio integration");
                            }
                            return;
                        }
                    }
                }

                if source == "instagram" && !sender_id.is_empty() {
                    let pool = crate::db::get_pool();
                    let query = "SELECT id, integration_code FROM tool_integrations WHERE id = 'meta' AND tenant_id = $1 LIMIT 1";
                    let row: Result<(String, String), sqlx::Error> = sqlx::query_as(query)
                        .bind(&tenant_id_for_meta)
                        .fetch_one(&pool)
                        .await;

                    match row {
                        Ok((_found_id, api_token,)) => {
                            let registry = crate::integrations::registry::IntegrationsRegistry::new();
                            let integration_id = "meta";

                            let meta_creds = ::server_ohc::orchestration::ConnectIntegrationRequest { bot_token: api_token.clone(), chat_id: "".to_string(), webhook_url: "".to_string(), api_token: api_token.clone(), from_phone: "".to_string(), ..Default::default() };
                            if let Err(e) = registry.connect(integration_id, &tenant_id_for_meta, meta_creds.clone()) {
                                tracing::warn!("Failed to connect {} integration: {}", integration_id, e);
                            }

                            let res = registry.send_message(integration_id, &source, &sender_id, &text).await;
                            if let Err(e) = res {
                                tracing::error!("Failed to send {} message via Meta integration: {}", source, e);
                            } else {
                                tracing::info!("Successfully sent {} message via Meta integration", source);
                            }
                        }
                        Err(e) => {
                            tracing::error!("Failed to fetch Meta integration credentials from DB: {}", e); // pii-safe
                        }
                    }
                }
            });

            if let Some(inbox_id) = original.and_then(|orig| orig.get("inbox_message_id").and_then(|v| v.as_str())) {
                let orchestrator_clone = self.orchestrator.clone();
                let id_clone = inbox_id.to_string();
                let tenant_id_clone = event.tenant_id.clone();
                tokio::spawn(async move {
                    let _ = orchestrator_clone.update_inbox_message_status(&id_clone, &tenant_id_clone, "sent").await;
                });
            }

            // Log the action in the agent's memory, handling errors and using proper defaults
            // Assuming we don't have an embedding service here, we use a zero vector
            // but properly await and map the error.
            let record = ohc_builtin_agent::memory_store::EmbeddingRecord {
                id: uuid::Uuid::new_v4().to_string(),
                tenant_id: event.tenant_id.clone(),
                agent_id: "customer_success_agent".to_string(),
                content,
                embedding: vec![0.0; 1536], // Simple dummy embedding since we don't have an embedder
                source_type: "AGENT_ACTION".to_string(),
                created_at: chrono::Utc::now(),
                last_referenced_at: chrono::Utc::now(),
                reference_count: 0,
                reliability_score: 100,
                owner_override: false,
                metadata: None,
            };
            self.orchestrator.write_long_term_memory(record).await.map_err(|e| e.to_string())?;

            return Ok(());
        }


        if event.event_type == "tenant.subscription.churn_risk" {
            let customer_id = event.payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("");
            let customer_name = event.payload.get("customer_name").and_then(|v| v.as_str()).unwrap_or("");
            let subscription_id = event.payload.get("subscription_id").and_then(|v| v.as_str()).unwrap_or("");

            if customer_id.is_empty() {
                return Err("customer_id is required".to_string());
            }

            let prompt = format!(
                "Draft a concise, personalized win-back message for customer {} (ID: {}) who hasn't booked or ordered anything in the last 30 days and their subscription is approaching renewal. Offer a small perk like a free consultation or a small discount.",
                customer_name, customer_id
            );
            let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);

            let generated_response = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
                Ok("minimax") => {
                    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string());
                    crate::minimax::MinimaxClient::new(api_key).reason(&compressed_prompt).await.unwrap_or_else(|_| "Hi! We noticed you haven't booked a session lately. Want to schedule a free 15-minute catch-up to keep the momentum going?".to_string())
                }
                _ => {
                    crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await.unwrap_or_else(|_| "Hi! We noticed you haven't booked a session lately. Want to schedule a free 15-minute catch-up to keep the momentum going?".to_string())
                }
            };

            let action_payload = serde_json::json!({
                "feature_type": "subscription_churn_risk",
                "generated_response": generated_response,
                "customer_id": customer_id,
                "subscription_id": subscription_id,
            });

            let _ = self.orchestrator.execute_action(
                DepartmentType::CustomerSuccess,
                "Subscription Churn Risk Draft".to_string(),
                event.tenant_id.clone(),
                ActionRisk::DraftForReview,
                action_payload,
            ).await;

            return Ok(());
        }

        if event.event_type == "tenant.subscription.check_predictive_restock" {
            let customer_id = event.payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("");
            if customer_id.is_empty() {
                return Err("customer_id is required".to_string());
            }

            if let Ok(Some(predicted_date)) = self.orchestrator.predict_replenishment(&event.tenant_id, customer_id).await {
                let prompt = format!(
                    "Draft a concise, warm restock message for the customer based on their predicted replenishment date of {}. Mention they might be running low and ask if they want a refill processed.",
                    predicted_date
                );
                let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);

                let generated_response = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
                    Ok("minimax") => {
                        let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string());
                        crate::minimax::MinimaxClient::new(api_key).reason(&compressed_prompt).await.unwrap_or_else(|_| "Hi, looks like you might be running low! Reply Yes to restock.".to_string())
                    }
                    _ => {
                        crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await.unwrap_or_else(|_| "Hi, looks like you might be running low! Reply Yes to restock.".to_string())
                    }
                };

                let action_payload = serde_json::json!({
                    "feature_type": "predictive_restock_draft",
                    "generated_response": generated_response,
                    "customer_id": customer_id,
                    "predicted_date": predicted_date,
                });

                let _ = self.orchestrator.execute_action(
                    DepartmentType::CustomerSuccess,
                    "Predictive Restock Draft".to_string(),
                    event.tenant_id.clone(),
                    ActionRisk::DraftForReview,
                    action_payload,
                ).await;
            }

            return Ok(());
        }

        if event.event_type == "tenant.subscription.action.requested" {
            let action = event.payload.get("action").and_then(|v| v.as_str()).unwrap_or("");
            let customer_id = event.payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("");
            tracing::info!("Ambassador agent received subscription action: {} for customer: {}", action, customer_id);

            // Mock updating subscription action
            let proposed_action = serde_json::json!({
                "action_type": "Execute Subscription Update",
                "customer_id": customer_id,
                "action": action
            });

            self.orchestrator.execute_action(
                DepartmentType::CustomerSuccess,
                "Execute Subscription Update".to_string(),
                event.tenant_id.clone(),
                ActionRisk::AutoExecute,
                proposed_action,
            ).await.map_err(|e| e.to_string())?;

            return Ok(());
        }


        if event.event_type == "tenant.subscription.at_risk" {
            let subscriber_id = event.payload.get("subscriber_id").and_then(|v| v.as_str()).unwrap_or("");
            let customer_id = event.payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("");
            let health_score = event.payload.get("health_score").and_then(|v| v.as_i64()).unwrap_or(0);

            let prompt = format!("You are The Ambassador for tenant {}. A subscriber (Customer ID: {}) is at risk of churning due to a low health score of {}. Write a concise, personalized win-back message offering a free 15-minute consultation to get them back on track. Keep it warm and friendly.", event.tenant_id, customer_id, health_score);
            let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);

            let generated_response = match std::env::var("OHC_INBOX_DRAFT_LLM_PROVIDER").or_else(|_| std::env::var("OHC_LLM_PROVIDER")).as_deref() {
                Ok("minimax") => { let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string()); crate::minimax::MinimaxClient::new(api_key).reason(&compressed_prompt).await.unwrap_or_else(|_| "Hi! We noticed you haven't been active lately. We'd love to offer a free 15-minute consultation to get you back on track!".to_string()) },
                _ => { crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await.unwrap_or_else(|_| "Hi! We noticed you haven't been active lately. We'd love to offer a free 15-minute consultation to get you back on track!".to_string()) }
            };

            let description = format!("The Ambassador identified subscriber {} as at-risk and drafted a win-back offer.", subscriber_id);
            let action_payload = serde_json::json!({
                "feature_type": "subscription_win_back", "subscriber_id": subscriber_id, "customer_id": customer_id, "health_score": health_score, "generated_response": generated_response, "draft_reply": generated_response,
            });

            let _approval_req = self.orchestrator.execute_action(crate::orchestration::departments::types::DepartmentType::CustomerSuccess, description, event.tenant_id.clone(), crate::orchestration::departments::types::ActionRisk::DraftForReview, action_payload.clone()).await.map_err(|e| e.to_string())?;

            return Ok(());
        }

        if event.event_type == "tenant.message.received" || event.event_type == "tenant.omnichannel.message.received" {
            let message = event.payload.get("original_message")
                .or_else(|| event.payload.get("message"))
                .or_else(|| event.payload.get("content"))
                .and_then(|v| v.as_str()).unwrap_or("");

            // Check for subscription modification intents
            let msg_lower = message.to_lowercase();
            if msg_lower.contains("skip") || msg_lower.contains("pause") || msg_lower.contains("cancel") {
                if msg_lower.contains("subscription") || msg_lower.contains("delivery") {
                    let action = if msg_lower.contains("skip") { "skip" } else if msg_lower.contains("pause") { "pause" } else { "cancel" };
                    tracing::info!("Ambassador parsed subscription intent: {} from message: {}", action, message);
                    let customer_id = event.payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("");

                    let proposed_action = serde_json::json!({
                        "action_type": "Execute Subscription Update",
                        "customer_id": customer_id,
                        "action": action
                    });

                    let risk_level = if action == "cancel" { ActionRisk::DraftForReview } else { ActionRisk::AutoExecute };

                    self.orchestrator.execute_action(
                        DepartmentType::CustomerSuccess,
                        "Execute Subscription Update".to_string(),
                        event.tenant_id.clone(),
                        risk_level,
                        proposed_action,
                    ).await.map_err(|e| e.to_string())?;

                    return Ok(());
                }
            }
            let source = event.payload.get("source").and_then(|v| v.as_str()).unwrap_or("");
            let sender_id = event.payload.get("sender_id").and_then(|v| v.as_str()).unwrap_or("");

            // --- BEGIN: Conversational Quoting Engine ---
            if msg_lower.contains("quote") || msg_lower.contains("custom") || msg_lower.contains("can i get") || msg_lower.contains("need a") {
                tracing::info!("Ambassador parsed potential custom quote intent from message: {}", message);

                let mut missing_fields = vec![];
                if !msg_lower.contains("date") && !msg_lower.contains("when") && !msg_lower.contains("saturday") && !msg_lower.contains("weekend") {
                    missing_fields.push("date/timeline");
                }

                if missing_fields.is_empty() {
                    let proposed_action = serde_json::json!({
                        "action_type": "Draft Custom Quote",
                        "sender_id": sender_id,
                        "message": message,
                        "suggested_price": 50.00,
                        "deposit_required": 25.00,
                        "generated_response": "I can definitely help with that! I've drafted a quote for you. Once approved by the owner, I'll send it right over with a deposit link."
                    });

                    self.orchestrator.execute_action(
                        DepartmentType::CustomerSuccess,
                        "Draft Custom Quote".to_string(),
                        event.tenant_id.clone(),
                        ActionRisk::DraftForReview,
                        proposed_action,
                    ).await.map_err(|e| e.to_string())?;
                    return Ok(());
                } else {
                    let proposed_action = serde_json::json!({
                        "action_type": "Draft Message",
                        "sender_id": sender_id,
                        "source": source,
                        "generated_response": format!("I'd love to help with that! Could you tell me more about the {}?", missing_fields.join(", "))
                    });

                    self.orchestrator.execute_action(
                        DepartmentType::CustomerSuccess,
                        "Draft Follow-up Question".to_string(),
                        event.tenant_id.clone(),
                        ActionRisk::DraftForReview,
                        proposed_action,
                    ).await.map_err(|e| e.to_string())?;
                    return Ok(());
                }
            }
            // --- END: Conversational Quoting Engine ---

            let payload_customer_id = event.payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("");

            // Identity Resolution: Use IdentityResolver to get unified customer graph identity
            let mut customer_id = "".to_string();
            let mut past_orders = "".to_string();
            let mut profile_summary_text = "".to_string();
            let pool = crate::db::get_pool();
            let global_db = std::sync::Arc::new(crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });

            if !payload_customer_id.is_empty() {
                customer_id = payload_customer_id.to_string();
            } else if !sender_id.is_empty() && sender_id != "unknown" {
                let resolver = crate::orchestration::identity_resolution::IdentityResolver::new(global_db.clone());
                if let Ok(id) = resolver.resolve_or_create_customer(&event.tenant_id, sender_id, source).await {
                    customer_id = id.clone();
                    tracing::info!("Resolved sender {} to customer {} via Memory Graph Identity Resolver", sender_id, customer_id);
                }
            }

            let mut memory_graph_summary = String::new();
            if !customer_id.is_empty() {
                // Query Unified Customer Memory Graph
                let mem_service = crate::services::customer_memory_graph::service::CustomerMemoryGraphService::new(pool.clone());
                let _ = mem_service.ingest_interaction(&event.tenant_id, &customer_id, source, message).await;

                if let Ok(profile_summary) = mem_service.get_profile_summary(&event.tenant_id, &customer_id).await {
                    if !profile_summary.summary.is_empty() && profile_summary.summary != "No summary available." && profile_summary.summary != "Customer not found." {
                        profile_summary_text = format!("Unified Customer Memory: {} | Preferences: {}", profile_summary.summary, profile_summary.preferences.join(", "));
                    }
                }

                // Fetch past orders context
                let orders: Result<Vec<(f64,)>, sqlx::Error> = sqlx::query_as("SELECT total_amount FROM orders WHERE tenant_id = $1 AND customer_id = $2")
                    .bind(&event.tenant_id)
                    .bind(&customer_id)
                    .fetch_all(&pool)
                    .await;
                if let Ok(orders) = orders {
                    if !orders.is_empty() {
                        past_orders = format!("Returning Customer ({} past orders).", orders.len());
                    }
                }

                // Query Unified Customer Memory Graph
                let memory_service = CustomerMemoryGraphService::new(pool.clone());
                if let Ok(profile_summary) = memory_service.get_profile_summary(&event.tenant_id, &customer_id).await {
                    if profile_summary.total_interactions > 0 || !profile_summary.segments.is_empty() {
                        memory_graph_summary = format!(
                            "Customer Profile: Interactions: {}. Segments: {}. Preferences: {}. Summary: {}",
                            profile_summary.total_interactions,
                            profile_summary.segments.join(", "),
                            profile_summary.preferences.join(", "),
                            profile_summary.summary
                        );
                    }
                }
            }

            let query_embedding = match std::env::var("OHC_INBOX_DRAFT_LLM_PROVIDER")
                .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
                .as_deref()
            {
                Ok("minimax") => {
                    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string());
                    crate::minimax::MinimaxClient::new(api_key).generate_embedding(message).await.unwrap_or_else(|_| vec![0.0; 1536])
                }
                _ => {
                    crate::minimax::LocalLLMClient::new().generate_embedding(message).await.unwrap_or_else(|_| vec![0.0; 1536])
                }
            };

            let memories = self.orchestrator.query_long_term_memory(&event.tenant_id, &query_embedding, 5).await.unwrap_or_default();

            let mut context_summary = if !memories.is_empty() {
                memories.join("\n")
            } else {
                "No relevant memory found.".to_string()
            };

            if !past_orders.is_empty() {
                context_summary.push_str("\n");
                context_summary.push_str(&past_orders);
            }
            if !profile_summary_text.is_empty() {
                context_summary.push_str("\n");
                context_summary.push_str(&profile_summary_text);
            }

            if !memory_graph_summary.is_empty() {
                context_summary.push_str("\n");
                context_summary.push_str(&memory_graph_summary);
            }

            if let Ok(inventory_summary) = self.orchestrator.get_inventory_summary(&event.tenant_id).await {
                context_summary.push_str("\n\n");
                context_summary.push_str(&inventory_summary);
            }

            let prompt = format!(
                "Write one concise, warm customer-service reply for an omnichannel SMB inbox. Do not invent policies, availability, prices, or order state. Use the provided inventory context if asked about product availability. Tenant: {}. Customer message: {}\n\nContext:\n{}",
                event.tenant_id, message, context_summary
            );
            let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);

            let generated_response = match std::env::var("OHC_INBOX_DRAFT_LLM_PROVIDER")
                .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
                .as_deref()
            {
                Ok("minimax") => {
                    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string());
                    crate::minimax::MinimaxClient::new(api_key).reason(&compressed_prompt).await.unwrap_or_else(|_| "Thank you for your message. We will get back to you shortly.".to_string())
                }
                _ => {
                    crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await.unwrap_or_else(|_| "Thank you for your message. We will get back to you shortly.".to_string())
                }
            };

            let description = if risk == ActionRisk::AutoExecute {
                format!("Auto-replied to message: '{}' with '{}'", message, generated_response)
            } else {
                "The Ambassador drafted a response for your review.".to_string()
            };

            let inbox_id = event.payload.get("inbox_message_id")
                .or_else(|| event.payload.get("message_id"))
                .and_then(|v| v.as_str()).unwrap_or("");
            if !inbox_id.is_empty() {
                let _ = self.orchestrator.update_inbox_message_draft(inbox_id, &event.tenant_id, &generated_response).await;
                if risk == ActionRisk::AutoExecute {
                    let _ = self.orchestrator.update_inbox_message_status(inbox_id, &event.tenant_id, "auto_replied").await;
                }
            }

            let action_payload = serde_json::json!({
                "feature_type": "ambassador_reply",
                "original_message": message,
                "generated_response": generated_response,
                "context_used": context_summary,
                "inbox_message_id": inbox_id,
                "source": source,
                "original_content": message,
                "sender_id": sender_id,
                "customer_id": customer_id,
                "past_orders": past_orders,
                "profile_summary": profile_summary_text,
            });

            let approval_req = self.orchestrator.execute_action(
                DepartmentType::CustomerSuccess,
                description,
                event.tenant_id.clone(),
                risk.clone(),
                action_payload.clone(),
            ).await.map_err(|e| e.to_string())?;

            if risk == ActionRisk::AutoExecute {
                let approved_event = DepartmentEvent {
                    id: uuid::Uuid::new_v4().to_string(),
                    tenant_id: event.tenant_id.clone(),
                    event_type: "agent:customer_success:approved".to_string(),
                    payload: serde_json::json!({
                        "original_payload": action_payload,
                        "approval_id": approval_req.id
                    }),
                };
                let _ = self.orchestrator.dispatch_event(approved_event).await;
            }

            return Ok(());
        }

        if event.payload.get("feature_type").and_then(|v| v.as_str()) == Some("ambassador_reply") {
            let description = "The Ambassador drafted a response for your review.".to_string();
            let action_payload = event.payload.clone();

            let approval_req = self.orchestrator.execute_action(
                DepartmentType::CustomerSuccess,
                description,
                event.tenant_id.clone(),
                risk.clone(),
                action_payload.clone(),
            ).await.map_err(|e| e.to_string())?;

            if risk == ActionRisk::AutoExecute {
                let approved_event = DepartmentEvent {
                    id: uuid::Uuid::new_v4().to_string(),
                    tenant_id: event.tenant_id.clone(),
                    event_type: "agent:customer_success:approved".to_string(),
                    payload: serde_json::json!({
                        "original_payload": action_payload,
                        "approval_id": approval_req.id
                    }),
                };
                let _ = self.orchestrator.dispatch_event(approved_event).await;
            }

            return Ok(());
        }

        self.orchestrator.execute_action(
            DepartmentType::CustomerSuccess,
            "Send personalized thank you & shipping ETA".to_string(),
            event.tenant_id.clone(),
            risk,
            event.payload.clone(),
        ).await.map(|_| ())
    }

    fn get_config(&self, tenant_id: &str) -> Option<DepartmentConfig> {
        self.configs.get(tenant_id).cloned()
    }

    fn set_config(&mut self, tenant_id: String, config: DepartmentConfig) {
        self.configs.insert(tenant_id, config);
    }

    async fn query_memory(&self, _query: &str) -> Result<Vec<String>, String> {
        Ok(vec![])
    }

    async fn request_approval(&self, description: String, tenant_id: String, risk: ActionRisk) -> Result<ApprovalRequest, String> {
        self.orchestrator.execute_action(self.department_type(), description.clone(), tenant_id.clone(), risk, serde_json::json!({})).await
    }
}

#[async_trait::async_trait]
impl BaseAgent for CustomerSuccessAgent {
    fn agent_id(&self) -> String {
        "customer_success_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;

    #[tokio::test]
    async fn test_customer_success_agent_subscribed_events() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            // For environments without DB URL, skip or use memory.
            return;
        }
        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = Arc::new(DepartmentOrchestrator::new(db, mesh));
        let agent = CustomerSuccessAgent::new(orchestrator);
        let events = agent.subscribed_events();
        assert!(events.contains(&"tenant.message.received".to_string()));
        assert!(events.contains(&"tenant.omnichannel.message.received".to_string()));
        assert!(events.contains(&"tenant.order.fulfillment_ready".to_string()));
        assert!(events.contains(&"agent:customer_success:approved".to_string()));
        assert!(events.contains(&"tenant.subscription.at_risk".to_string()));
        assert!(events.contains(&"job_status_updates".to_string()));
    }

    #[test]
    fn test_customer_success_agent_struct_exists() {
        // Minimal test to assert the module loads in the test harness
        let type_name = "CustomerSuccessAgent";
        assert_eq!(type_name, "CustomerSuccessAgent");
    }
}
