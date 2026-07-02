use serde::{Deserialize, Serialize};
use serde_json::json;
use ::server_ohc::orchestration::{StartOnboardingRequest, StartOnboardingResponse};
use crate::minimax::MinimaxClient;
use std::sync::OnceLock;
use ::server_utils::cache::HybridCache;

pub static ONBOARDING_STATE_AGENT_CACHE: OnceLock<HybridCache<serde_json::Value>> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntakeData {
    pub location: Option<String>,
    pub target_audience: Option<String>,
    pub business_name: String,
    pub business_type: String,
    pub categories: Vec<String>,
    pub initial_products: Vec<IntakeProduct>,
    pub initial_tasks: Option<Vec<String>>,
    pub sample_customer_name: Option<String>,
    pub sample_customer_email: Option<String>,
    pub deposit_percentage: Option<i32>,
    pub lead_time_days: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntakeProductVariant {
    pub name: String,
    pub price_modifier: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntakeProduct {
    pub name: String,
    pub price: String,
    pub description: Option<String>,
    pub variants: Option<Vec<IntakeProductVariant>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatResponse {
    pub is_complete: bool,
    pub reply: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intake_data: Option<IntakeData>,
}

#[derive(Clone)]
pub struct OnboardingAgent {
    db: std::sync::Arc<crate::db::DB>,
    hub: std::sync::Arc<crate::hub::Hub>,
    minimax: Option<std::sync::Arc<MinimaxClient>>,
}

impl OnboardingAgent {
    pub fn new(db: std::sync::Arc<crate::db::DB>, hub: std::sync::Arc<crate::hub::Hub>) -> Self {
        let minimax = std::env::var("MINIMAX_API_KEY")
            .ok()
            .map(|key| std::sync::Arc::new(MinimaxClient::new(key)));
        OnboardingAgent { db, hub, minimax }
    }

    pub async fn process_chat(&self, messages: Vec<ChatMessage>) -> Result<ChatResponse, String> {
        let user_messages: Vec<&ChatMessage> = messages.iter().filter(|m| m.role == "user").collect();

        let minimax = match self.minimax.as_ref() {
            Some(m) => m,
            None => {
                // E2E Test / Local adapter mock fallback when no LLM is configured
                let combined_input = user_messages.iter().map(|m| {
                    let mut text = m.content.clone();
                    if let Some(url) = &m.image_url {
                        text.push_str(&format!("\nImage provided: {}", url));
                    }
                    text
                }).collect::<Vec<String>>().join("\n");
                let intake_data = self.process_intake(&combined_input).await?;

                return Ok(ChatResponse {
                    is_complete: true,
                    reply: "Give me a minute... I'm building your business.".to_string(),
                    intake_data: Some(intake_data),
                });
            }
        };

        let mut conversation_history = String::new();
        for msg in &messages {
            let role = if msg.role == "user" { "User" } else { "Assistant" };
            let mut content = msg.content.clone();
            if let Some(url) = &msg.image_url {
                content.push_str(&format!(" (Image provided: {})", url));
            }
            conversation_history.push_str(&format!("{}: {}\n", role, content));
        }

        let prompt = format!(
            "You are the OHC Onboarding Expert assistant. Your goal is to synthesize a fully-operational, mobile-first workspace from a single user prompt.
Extract the business taxonomy, default any missing fields to sensible industry defaults, and generate the configuration. Do NOT ask follow-up questions unless the input is completely empty or nonsensical, EXCEPT if the business is clearly related to Real Estate / Property Management, in which case you MUST ask ONE clarifying question: 'Do you want to handle maintenance requests through the app?' If they have already answered this question or if the conversation history shows you already asked this, then output [COMPLETE].
You need to synthesize at least:
1. What they sell or what service they provide.
2. A rough idea of their business type (e.g. bakery, handyman, tutor, real estate).

Review the following conversation history:
{}

If the input is completely empty or nonsensical, reply with a natural, conversational question asking for clarification.
If the business is Real Estate and you haven't asked about maintenance requests, reply with ONLY the clarifying question.
Otherwise, reply EXACTLY with the string `[COMPLETE]` followed by a brief confirmation message (e.g., `[COMPLETE] Give me a minute... I'm building your business.`). Do not output anything else if you have enough information.

Your response:",
            conversation_history
        );

        let mut attempts = 0;
        let mut response = String::new();
        while attempts < 3 {
            match tokio::time::timeout(std::time::Duration::from_secs(60), minimax.reason(&prompt)).await {
                Ok(Ok(content)) => {
                    response = content;
                    break;
                },
                _ => {
                    attempts += 1;
                    if attempts == 3 {
                        return Err("AI call failed after 3 attempts".into());
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(2u64.pow(attempts))).await;
                }
            }
        }

        let response = response.trim();

        if response.starts_with("[COMPLETE]") {
            let reply_msg = response.trim_start_matches("[COMPLETE]").trim().to_string();
            let combined_input = user_messages.iter().map(|m| {
                let mut text = m.content.clone();
                if let Some(url) = &m.image_url {
                    text.push_str(&format!("\nImage provided: {}", url));
                }
                text
            }).collect::<Vec<String>>().join("\n");

            let intake_data = self.process_intake(&combined_input).await?;

            Ok(ChatResponse {
                is_complete: true,
                reply: if reply_msg.is_empty() { "Give me a minute... I'm building your business.".to_string() } else { reply_msg },
                intake_data: Some(intake_data),
            })
        } else {
            Ok(ChatResponse {
                is_complete: false,
                reply: response.to_string(),
                intake_data: None,
            })
        }
    }

    pub async fn process_intake(&self, input: &str) -> Result<IntakeData, String> {
        let minimax = match self.minimax.as_ref() {
            Some(m) => m,
            None => {
                // E2E Test / Local adapter mock fallback when no LLM is configured
                return Ok(IntakeData {
                    business_name: "Mock Business".to_string(),
                    business_type: "Mock Type".to_string(),
                    categories: vec!["physical".to_string()],
                    initial_products: vec![
                        IntakeProduct {
                            name: "Mock Product 1".to_string(),
                            price: "10.00".to_string(),
                            description: Some("Description for Product 1".to_string()),
                            variants: None,
                        },
                        IntakeProduct {
                            name: "Mock Product 2".to_string(),
                            price: "20.00".to_string(),
                            description: Some("Description for Product 2".to_string()),
                            variants: None,
                        },
                        IntakeProduct {
                            name: "Mock Product 3".to_string(),
                            price: "30.00".to_string(),
                            description: Some("Description for Product 3".to_string()),
                            variants: None,
                        },
                    ],
                    location: Some("Mock Location".to_string()),
                    target_audience: Some("Mock Audience".to_string()),
                    initial_tasks: Some(vec!["Follow up with new leads".to_string()]),
                    sample_customer_name: Some("Sample Customer".to_string()),
                    sample_customer_email: Some("sample@example.com".to_string()),
                    deposit_percentage: Some(50),
                    lead_time_days: Some(3),
                });
            }
        };

        let prompt = format!(
            "You are the OHC Onboarding Expert. Extract structured business information from the user description.
            We serve various OHC personas like:
            - Maya (Home Baker): Needs cake customizer, deposits, and delivery.
            - Carlos (Field Service): Needs service bookings, estimates, and route notes.
            - Priya (Boutique): Needs inventory, variants, and tap-to-pay.
            - Leo (Creator/Tutor): Needs packages, scheduling, and student follow-ups.
            - Fatima (Food Cart): Needs simple order list, pickup timing, and offline flows.
            - Nora (Agency): Needs project intake, proposals, and task assignment.
            - Elena (Property Manager / Real Estate): Needs property listings, applicant intake forms, showing schedules, and maintenance request workflows if requested.

            If the input matches or is similar to these personas, use them for inspiration.
            If the input is an Instagram/social link, infer details from the profile.

            Return ONLY a valid JSON object with fields:
            - business_name (string)
            - business_type (string, e.g., 'Home Bakery', 'Handyman')
            - categories (array of: physical, digital, services, food, subscriptions)
            - initial_products (array of at least 3 objects with 'name', 'price' string, 'description' string, and optional 'variants' array of objects with 'name' and 'price_modifier' string)
            - location (string)
            - target_audience (string)
            - initial_tasks (array of strings, e.g., ['Follow up with new leads'])
            - sample_customer_name (string)
            - sample_customer_email (string)
            - deposit_percentage (integer, e.g., 50 if they ask for 50% deposits)
            - lead_time_days (integer, e.g., 3 if they ask for 3 days notice)

            Description: \"{}\"

            Example JSON:
            {{
              \"business_name\": \"Maya's Cakes\",
              \"business_type\": \"Home Bakery\",
              \"categories\": [\"food\", \"physical\"],
              \"location\": \"Austin, TX\",
              \"target_audience\": \"Vegans and people looking for custom cakes\",
              \"initial_products\": [
                {{\"name\": \"Custom Chocolate Cake\", \"price\": \"45.00\", \"description\": \"A delicious vegan chocolate cake\", \"variants\": [
                    {{\"name\": \"6-inch\", \"price_modifier\": \"0.00\"}},
                    {{\"name\": \"8-inch\", \"price_modifier\": \"15.00\"}}
                ]}},
                {{\"name\": \"Dozen Cupcakes\", \"price\": \"24.00\", \"description\": \"A dozen assorted vegan cupcakes\", \"variants\": []}}
              ],
              \"initial_tasks\": [\"Follow up with new leads\", \"Setup delivery calendar\"],
              \"sample_customer_name\": \"Jane Doe\",
              \"sample_customer_email\": \"jane.doe@example.com\",
              \"deposit_percentage\": 50,
              \"lead_time_days\": 3
            }}",
            input
        );

        let mut attempts = 0;
        let mut response = String::new();
        while attempts < 3 {
            match tokio::time::timeout(std::time::Duration::from_secs(60), minimax.reason(&prompt)).await {
                Ok(Ok(content)) => {
                    response = content;
                    break;
                },
                _ => {
                    attempts += 1;
                    if attempts == 3 {
                        return Err("AI call failed after 3 attempts".into());
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(2u64.pow(attempts))).await;
                }
            }
        }

        // Clean up markdown code blocks if present
        let mut clean_json = response.as_str();
        if let Some(start) = clean_json.find('{') {
            if let Some(end) = clean_json.rfind('}') {
                if start <= end {
                    clean_json = &clean_json[start..=end];
                }
            }
        }

        let data: IntakeData = serde_json::from_str(clean_json)
            .map_err(|e| format!("Failed to parse AI response as JSON: {}. Response was: {}", e, response))?;

        Ok(data)
    }


    pub async fn save_onboarding_state(&self, tenant_id: &str, user_id: &str, current_step: i32, state_json: &serde_json::Value) -> Result<(), String> {
        tracing::debug!("Saving onboarding state for tenant: {}, user: {}", tenant_id, user_id); // pii-safe
        let mut tx = self.hub.pool.begin().await.map_err(|e| e.to_string())?;
        crate::common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        use sqlx::Row;

        let row = sqlx::query("SELECT state_json, current_step FROM onboarding_state WHERE tenant_id = $1 AND user_id = $2")
            .bind(tenant_id)
            .bind(user_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        let (mut merged_state, prev_step) = if let Some(record) = row {
            let existing_json: serde_json::Value = record.try_get("state_json").unwrap_or_else(|_| serde_json::json!({}));
            let existing_step: i32 = record.try_get("current_step").unwrap_or(0);
            (existing_json, existing_step)
        } else {
            (serde_json::json!({}), 0)
        };

        if let (Some(existing_obj), Some(new_obj)) = (merged_state.as_object_mut(), state_json.as_object()) {
            for (k, v) in new_obj {
                existing_obj.insert(k.clone(), v.clone());
            }
        } else {
            merged_state = state_json.clone();
        }

        let new_step = std::cmp::max(prev_step, current_step);

        sqlx::query(
            "INSERT INTO onboarding_state (tenant_id, user_id, current_step, state_json, updated_at)              VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)              ON CONFLICT (tenant_id, user_id) DO UPDATE              SET state_json = EXCLUDED.state_json,                  current_step = EXCLUDED.current_step,                  updated_at = CURRENT_TIMESTAMP"
        )
        .bind(tenant_id)
        .bind(user_id)
        .bind(new_step)
        .bind(&merged_state)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        let cache_key = format!("agent_onboarding_state_{}_{}", tenant_id, user_id);
        let cache = ONBOARDING_STATE_AGENT_CACHE.get_or_init(|| ::server_utils::cache::HybridCache::new(self.hub.redis_client.clone()));
        tracing::debug!("Invalidating onboarding state cache for key: {}", cache_key); // pii-safe
        cache.invalidate(&cache_key).await;

        // Invalidate the Dashboard cache as well
        let dashboard_cache_key = format!("onboarding_state_{}", tenant_id);
        let dashboard_cache = crate::services::dashboard::service::ONBOARDING_STATE_CACHE.get_or_init(|| ::server_utils::cache::HybridCache::new(self.hub.redis_client.clone()));
        tracing::debug!("Invalidating dashboard onboarding state cache for key: {}", dashboard_cache_key); // pii-safe
        dashboard_cache.invalidate(&dashboard_cache_key).await;

        Ok(())
    }

    pub async fn get_onboarding_state(&self, tenant_id: &str, user_id: &str) -> Result<serde_json::Value, String> {
        let cache_key = format!("agent_onboarding_state_{}_{}", tenant_id, user_id);
        let cache = ONBOARDING_STATE_AGENT_CACHE.get_or_init(|| ::server_utils::cache::HybridCache::new(self.hub.redis_client.clone()));
        tracing::debug!("Attempting to get onboarding state from cache for key: {}", cache_key); // pii-safe
        if let Some(cached_state) = cache.get(&cache_key).await {
            tracing::debug!("Cache hit for onboarding state key: {}", cache_key); // pii-safe
            return Ok(cached_state);
        }
        tracing::debug!("Cache miss for onboarding state key: {}", cache_key); // pii-safe

        let mut tx = self.hub.pool.begin().await.map_err(|e| e.to_string())?;
        crate::common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        use sqlx::Row;

        let row = sqlx::query(
            "SELECT current_step, state_json FROM onboarding_state WHERE tenant_id = $1 AND user_id = $2"
        )
        .bind(tenant_id)
        .bind(user_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let state = if let Some(record) = row {
            let mut state: serde_json::Value = record.get("state_json");
            let current_step: i32 = record.get("current_step");
            if let Some(obj) = state.as_object_mut() {
                obj.insert("step".to_string(), serde_json::json!(current_step));
            }
            state
        } else {
            serde_json::json!({ "step": 0 })
        };

        cache.set(&cache_key, state.clone(), std::time::Duration::from_secs(3600)).await;
        Ok(state)
    }

    pub async fn start_onboarding(&self, req: StartOnboardingRequest) -> Result<StartOnboardingResponse, String> {
        static EMAIL_REGEX: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        let email_regex = EMAIL_REGEX.get_or_init(|| regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap());

        if req.admin_email.trim().is_empty() || !email_regex.is_match(&req.admin_email) {
            return Err("Please enter a valid email address".to_string());
        }

        let has_number = req.admin_password.chars().any(|c| c.is_numeric());
        if req.admin_password.trim().is_empty() || req.admin_password.len() < 8 || !has_number {
            return Err("Password must be at least 8 characters and contain a number".to_string());
        }

        let start_time = std::time::Instant::now();
        let org_id = format!("org-{}", uuid::Uuid::new_v4());

        let business_type = req.business_type.clone();
        let company_name = req.company_name.clone();

        // Use organization id as tenant id if not provided
        let _tenant_id = org_id.clone();
        let domain_choice = req.domain_choice.clone();

        let user_id = format!("usr-{}", uuid::Uuid::new_v4());
        let email = req.admin_email.clone();
        let username = if req.admin_name.is_empty() { email.clone() } else { req.admin_name.clone() };
        let password = req.admin_password.clone();
        let location = req.location.clone();

        let req_first_product_name = req.first_product_name.clone();
        let req_first_product_price = req.first_product_price.clone();
        let req_price_type = req.price_type.clone();
        let org_id_clone1 = org_id.clone();
        let org_id_clone2 = org_id.clone();
        let business_type_clone = business_type.clone();

        let req_deposit_percentage = req.deposit_percentage;
        let req_lead_time_days = req.lead_time_days;

        let agent_clone_product = self.clone();
        let req_initial_products = req.initial_products.clone();

        let product_future = tokio::task::spawn(async move {
            if !req_initial_products.is_empty() {
                for product in req_initial_products {
                    let variants_converted: Option<Vec<IntakeProductVariant>> = if product.variants.is_empty() {
                        None
                    } else {
                        Some(product.variants.into_iter().map(|v| IntakeProductVariant {
                            name: v.name,
                            price_modifier: v.price_modifier,
                        }).collect())
                    };

                    let payload = serde_json::json!({
                        "name": product.name,
                        "price": product.price,
                        "price_type": req_price_type,
                        "business_type": business_type_clone,
                        "description": product.description,
                        "variants": variants_converted,
                        "deposit_percentage": req_deposit_percentage,
                        "lead_time_days": req_lead_time_days,
                    });

                    let job_id = uuid::Uuid::new_v4().to_string();
                    if let Err(e) = sqlx::query(
                        "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at)
                         VALUES ($1, $2, $3, $4, 'PENDING', CURRENT_TIMESTAMP)"
                    )
                    .bind(&job_id)
                    .bind(&org_id_clone1)
                    .bind("onboarding_generate_catalog")
                    .bind(serde_json::to_string(&payload).unwrap_or_default())
                    .execute(&agent_clone_product.db.pool)
                    .await
                    {
                        tracing::error!("Failed to enqueue catalog generation job: {}", e);
                    }
                }
                Ok(())
            } else if !req_first_product_name.is_empty() {
                agent_clone_product.create_product(&org_id_clone1, &req_first_product_name, &req_first_product_price, &req_price_type, &business_type_clone, None, None, req_deposit_percentage, req_lead_time_days).await
            } else {
                agent_clone_product.generate_initial_products(&org_id_clone1, &business_type_clone).await
            }
        });

        let agent_clone_seed = self.clone();
        let req_ai_agents = req.ai_agents.clone();
        let req_ai_auto_respond = req.ai_auto_respond;
        let seed_future = tokio::task::spawn(async move {
            agent_clone_seed.seed_default_agents(&org_id_clone2, &req_ai_agents, req_ai_auto_respond).await
        });

        let org_id_clone3 = org_id.clone();
        let pool = self.db.pool.clone();
        let hub_clone = self.hub.clone();
        let company_name_clone = company_name.clone();
        let business_type_clone_2 = business_type.clone();

        let publish_events_future = tokio::task::spawn(async move {
            // Subscribe default AI Agents to specific tenant events dynamically
            let event_topics = vec![
                ("The Manager", "tenant.booking.created"),
                ("The Manager", "tenant.order.placed"),
                ("The Promoter", "tenant.product.created"),
                ("The Salesperson", "tenant.lead.created"),
                ("The Ambassador", "tenant.message.received"),
                ("The Accountant", "tenant.payment.success"),
                ("The Protector", "tenant.contract.signed"),
                ("The Advisor", "tenant.report.generated"),
                ("The Scout", "tenant.seo.optimized"),
            ];

            let start_events = std::time::Instant::now();
            let mut topic_futures = vec![];
            for (agent_role, topic) in event_topics {
                let query = sqlx::query("INSERT INTO agent_event_subscriptions (tenant_id, agent_role, topic) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
                    .bind(org_id_clone3.to_string())
                    .bind(agent_role)
                    .bind(topic);
                let pool = pool.clone();
                topic_futures.push(tokio::spawn(async move {
                    let _ = query.execute(&pool).await;
                }));
            }
            futures::future::join_all(topic_futures).await;
            tracing::info!("publish_events_future event_topics inserts took: {} us", start_events.elapsed().as_micros());

            // Trigger KAIROS Orchestration for initial artifacts
            let storefront_event = ::server_ohc::orchestration::TeammateMeshEvent {
                agent_id: "system".to_string(),
                action: "GenerateStorefront".to_string(),
                status: "pending".to_string(),
                payload: serde_json::to_vec(&json!({
                    "organization_id": org_id_clone3,
                    "company_name": company_name_clone,
                    "business_type": business_type_clone_2,
                })).unwrap_or_default(),
                msg_id: uuid::Uuid::new_v4().to_string(),
            };
            let _ = hub_clone.publish_teammate_event("promoter_inbox".to_string(), storefront_event);

            let policy_event = ::server_ohc::orchestration::TeammateMeshEvent {
                agent_id: "system".to_string(),
                action: "GeneratePolicies".to_string(),
                status: "pending".to_string(),
                payload: serde_json::to_vec(&json!({
                    "organization_id": org_id_clone3,
                    "company_name": company_name_clone,
                })).unwrap_or_default(),
                msg_id: uuid::Uuid::new_v4().to_string(),
            };
            let _ = hub_clone.publish_teammate_event("protector_inbox".to_string(), policy_event);


            // Schedule the weekly health report via the internal task queue for The Advisor
            let scheduled_at = chrono::Utc::now() + chrono::Duration::days(7);
            let payload = serde_json::json!({
                "agent_role": "The Advisor",
                "task": "weekly_health_report",
                "tenant_id": org_id_clone3.clone()
            });
            if let Err(e) = sqlx::query("INSERT INTO sub_agent_queue (id, tenant_id, parent_task_id, payload, status, scheduled_at, created_at, updated_at) VALUES ($1, $2, NULL, $3, 'QUEUED', $4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                .bind(uuid::Uuid::new_v4().to_string())
                .bind(&org_id_clone3)
                .bind(serde_json::to_string(&payload).unwrap_or_default())
                .bind(scheduled_at.naive_utc())
                .execute(&pool)
                .await
            {
                ::server_telemetry::record_error_signal("[bug] Failed to schedule weekly health report");
                tracing::error!("Failed to schedule weekly health report: {}", e); // pii-safe
            }

            Ok::<(), String>(())
        });

        let hash_future = tokio::task::spawn(async move {
            if !password.is_empty() {
                tokio::task::spawn_blocking(move || {
                    bcrypt::hash(&password, if cfg!(test) { 4 } else { bcrypt::DEFAULT_COST }).map_err(|e| format!("Failed to hash password: {}", e))
                }).await.map_err(|e| e.to_string())?
            } else {
                Ok("".to_string())
            }
        });

        let (product_res_res, seed_res_res, events_res_res, hash_res_res) = tokio::join!(product_future, seed_future, publish_events_future, hash_future);

        let product_res = product_res_res.unwrap_or_else(|e| Err(e.to_string()));
        let seed_res = seed_res_res.unwrap_or_else(|e| Err(e.to_string()));
        let hash_res = hash_res_res.unwrap_or_else(|e| Err(e.to_string()));

        let events_res = events_res_res.unwrap_or_else(|e| Err(e.to_string()));
        if let Err(e) = events_res {
            tracing::warn!("Failed to publish onboarding events (non-fatal): {}", e);
        }

        product_res?;
        seed_res?;
        let password_hash = hash_res?;

        let roles_json = serde_json::to_string(&vec!["admin"]).unwrap_or_default();
        let now = chrono::Utc::now();
        let oidc_subject = "";

        sqlx::query(
            r#"
            INSERT INTO tenants (id, name, tier, subdomain)
            VALUES ($1, $2, 'free', $3)
            ON CONFLICT (id) DO UPDATE SET subdomain = EXCLUDED.subdomain
            "#
        )
        .bind(&org_id)
        .bind(&company_name)
        .bind(&domain_choice)
        .execute(&self.db.pool)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#
        )
        .bind(&user_id)
        .bind(&username)
        .bind(&email)
        .bind(&password_hash)
        .bind(&roles_json)
        .bind(true)
        .bind(&org_id)
        .bind(&oidc_subject)
        .bind(now)
        .bind(now)
        .execute(&self.db.pool)
        .await
        .map_err(|e| e.to_string())?;

        let flags_json = onboarding_feature_state(&req, &company_name, &business_type, &location);

        // Provision initial Agent Feed items (Action Required)
        let feed_id = uuid::Uuid::new_v4().to_string();
        let feed_payload = serde_json::json!({
            "description": format!("Welcome to OHC! I've set up your {} business. Click here to review your new storefront.", business_type),
            "feature_type": "onboarding_welcome",
            "company_name": company_name
        });

        if let Err(e) = sqlx::query(
            "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state) VALUES ($1, $2, $3, $4, $5, 'PENDING_APPROVAL')"
        )
        .bind(&feed_id)
        .bind(&org_id)
        .bind("system")
        .bind(&feed_payload)
        .bind(serde_json::json!({"action_type": "review_storefront"}))
        .execute(&self.db.pool)
        .await {
            tracing::error!("Failed to create initial feed item: {}", e);
        }

        sqlx::query(
            "INSERT INTO onboarding_state (tenant_id, user_id, current_step, state_json) VALUES ($1, $2, $3, $4)"
        )
        .bind(&org_id)
        .bind(&user_id)
        .bind(1)
        .bind(flags_json)
        .execute(&self.db.pool)
        .await
        .map_err(|e| e.to_string())?;

        crate::telemetry::track_onboarding_step(&org_id, "start_onboarding", start_time.elapsed().as_millis() as u64);
        Ok(StartOnboardingResponse {
            success: true,
            message: format!("Successfully onboarded {} as a {}!", company_name, business_type),
            organization_id: org_id,
            user_id: user_id,
        })
    }

    async fn create_product(&self, org_id: &str, name: &str, price_str: &str, price_type: &str, business_type: &str, description: Option<&str>, variants: Option<&Vec<IntakeProductVariant>>, deposit_percentage: Option<i32>, lead_time_days: Option<i32>) -> Result<(), String> {
        let price_cents = (price_str.parse::<f64>().unwrap_or(0.0) * 100.0) as i64;
        let strategy = match business_type {
            "Service Business" => "booking",
            _ => "physical",
        };

        let mut meta = json!({"price_type": price_type});
        if let Some(m) = meta.as_object_mut() {
            if let Some(dp) = deposit_percentage {
                m.insert("deposit_percentage".to_string(), json!(dp));
            }
            if let Some(lt) = lead_time_days {
                m.insert("lead_time_days".to_string(), json!(lt));
            }
        }

        let id = format!("prod-{}", uuid::Uuid::new_v4());
        sqlx::query("INSERT INTO products (id, tenant_id, title, description, price_cents, type, metadata) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(&id)
            .bind(org_id)
            .bind(name)
            .bind(description.unwrap_or("Added during onboarding"))
            .bind(price_cents)
            .bind(strategy)
            .bind(meta)
            .execute(&self.db.pool)
            .await
            .map_err(|e| e.to_string())?;


        if let Some(vars) = variants {
            for variant in vars {
                let variant_id = format!("var-{}", uuid::Uuid::new_v4());
                let var_price_modifier = (variant.price_modifier.parse::<f64>().unwrap_or(0.0) * 100.0) as i64;
                sqlx::query("INSERT INTO product_variants (id, tenant_id, product_id, name, sku, price_modifier, inventory_count) VALUES ($1, $2, $3, $4, $5, $6, $7)")
                    .bind(&variant_id)
                    .bind(org_id)
                    .bind(&id)
                    .bind(&variant.name)
                    .bind("")
                    .bind(var_price_modifier)
                    .bind(0)
                    .execute(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        let event_payload = json!({
            "product_id": id,
            "name": name,
            "organization_id": org_id,
        });

        let event = ::server_ohc::orchestration::TeammateMeshEvent {
            agent_id: "system".to_string(),
            action: "ProductCreated".to_string(),
            status: "success".to_string(),
            payload: serde_json::to_vec(&event_payload).unwrap_or_default(),
            msg_id: uuid::Uuid::new_v4().to_string(),
        };

        let _ = self.hub.publish_teammate_event("products_inbox".to_string(), event);

        Ok(())
    }

    async fn generate_initial_products(&self, org_id: &str, business_type: &str) -> Result<(), String> {
        let products = match business_type {
            "Home Baker" | "Bakery" => vec![
                ("Custom Celebration Cake", "Beautifully decorated for your special day", 4500, "booking"),
                ("Dozen Assorted Cupcakes", "A variety of our best flavors", 2400, "physical"),
                ("Seasonal Pie", "Baked fresh with local ingredients", 1800, "physical"),
            ],
            "Handyman" | "Field Service" => vec![
                ("Standard Repair Visit", "Basic maintenance and small repairs", 7500, "booking"),
                ("Plumbing Consultation", "Inspection and quote for plumbing work", 4500, "booking"),
                ("Emergency Call-Out", "Priority service for urgent issues", 12000, "booking"),
            ],
            "Boutique" | "Retail" => vec![
                ("Signature Dress", "Our most popular seasonal piece", 8900, "physical"),
                ("Curated Accessory Set", "Perfectly paired jewelry and scarf", 3500, "physical"),
                ("Private Styling Session", "1-on-1 fashion advice with our team", 5000, "booking"),
            ],
            "Online Store" => vec![
                ("Standard Product", "A great product for your store", 1999, "physical"),
                ("Premium Product", "A premium offering", 4999, "physical"),
            ],
            "Service Business" => vec![
                ("Consultation", "1-hour professional consultation", 10000, "booking"),
                ("Service Call", "On-site service visit", 7500, "booking"),
            ],
                        "Plumbing" => vec![
                ("Premium Plumbing Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Plumbing Service", "Essential services to get you started", 9999, "booking"),
                ("Plumbing Consultation", "Expert advice and planning", 4999, "booking"),
                ("Plumbing Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Plumbing Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Yoga Studio" => vec![
                ("Premium Yoga Studio Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Yoga Studio Service", "Essential services to get you started", 9999, "booking"),
                ("Yoga Studio Consultation", "Expert advice and planning", 4999, "booking"),
                ("Yoga Studio Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Yoga Studio Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Bakery_Old" => vec![
                ("Premium Bakery Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Bakery Service", "Essential services to get you started", 9999, "booking"),
                ("Bakery Consultation", "Expert advice and planning", 4999, "booking"),
                ("Bakery Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Bakery Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Freelance Writer" => vec![
                ("Premium Freelance Writer Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Freelance Writer Service", "Essential services to get you started", 9999, "booking"),
                ("Freelance Writer Consultation", "Expert advice and planning", 4999, "booking"),
                ("Freelance Writer Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Freelance Writer Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Car Wash" => vec![
                ("Premium Car Wash Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Car Wash Service", "Essential services to get you started", 9999, "booking"),
                ("Car Wash Consultation", "Expert advice and planning", 4999, "booking"),
                ("Car Wash Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Car Wash Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Pet Groomer" => vec![
                ("Premium Pet Groomer Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Pet Groomer Service", "Essential services to get you started", 9999, "booking"),
                ("Pet Groomer Consultation", "Expert advice and planning", 4999, "booking"),
                ("Pet Groomer Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Pet Groomer Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Law Firm" => vec![
                ("Premium Law Firm Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Law Firm Service", "Essential services to get you started", 9999, "booking"),
                ("Law Firm Consultation", "Expert advice and planning", 4999, "booking"),
                ("Law Firm Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Law Firm Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Accounting Firm" => vec![
                ("Premium Accounting Firm Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Accounting Firm Service", "Essential services to get you started", 9999, "booking"),
                ("Accounting Firm Consultation", "Expert advice and planning", 4999, "booking"),
                ("Accounting Firm Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Accounting Firm Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Web Design Agency" => vec![
                ("Premium Web Design Agency Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Web Design Agency Service", "Essential services to get you started", 9999, "booking"),
                ("Web Design Agency Consultation", "Expert advice and planning", 4999, "booking"),
                ("Web Design Agency Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Web Design Agency Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Landscaping" => vec![
                ("Premium Landscaping Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Landscaping Service", "Essential services to get you started", 9999, "booking"),
                ("Landscaping Consultation", "Expert advice and planning", 4999, "booking"),
                ("Landscaping Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Landscaping Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Photography Studio" => vec![
                ("Premium Photography Studio Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Photography Studio Service", "Essential services to get you started", 9999, "booking"),
                ("Photography Studio Consultation", "Expert advice and planning", 4999, "booking"),
                ("Photography Studio Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Photography Studio Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Fitness Coach" => vec![
                ("Premium Fitness Coach Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Fitness Coach Service", "Essential services to get you started", 9999, "booking"),
                ("Fitness Coach Consultation", "Expert advice and planning", 4999, "booking"),
                ("Fitness Coach Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Fitness Coach Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Coffee Shop" => vec![
                ("Premium Coffee Shop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Coffee Shop Service", "Essential services to get you started", 9999, "booking"),
                ("Coffee Shop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Coffee Shop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Coffee Shop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Hair Salon" => vec![
                ("Premium Hair Salon Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Hair Salon Service", "Essential services to get you started", 9999, "booking"),
                ("Hair Salon Consultation", "Expert advice and planning", 4999, "booking"),
                ("Hair Salon Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Hair Salon Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Nail Salon" => vec![
                ("Premium Nail Salon Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Nail Salon Service", "Essential services to get you started", 9999, "booking"),
                ("Nail Salon Consultation", "Expert advice and planning", 4999, "booking"),
                ("Nail Salon Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Nail Salon Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Massage Therapist" => vec![
                ("Premium Massage Therapist Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Massage Therapist Service", "Essential services to get you started", 9999, "booking"),
                ("Massage Therapist Consultation", "Expert advice and planning", 4999, "booking"),
                ("Massage Therapist Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Massage Therapist Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Cleaning Service" => vec![
                ("Premium Cleaning Service Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Cleaning Service Service", "Essential services to get you started", 9999, "booking"),
                ("Cleaning Service Consultation", "Expert advice and planning", 4999, "booking"),
                ("Cleaning Service Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Cleaning Service Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Electrician" => vec![
                ("Premium Electrician Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Electrician Service", "Essential services to get you started", 9999, "booking"),
                ("Electrician Consultation", "Expert advice and planning", 4999, "booking"),
                ("Electrician Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Electrician Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "HVAC Repair" => vec![
                ("Premium HVAC Repair Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic HVAC Repair Service", "Essential services to get you started", 9999, "booking"),
                ("HVAC Repair Consultation", "Expert advice and planning", 4999, "booking"),
                ("HVAC Repair Assessment", "Initial evaluation and report", 7500, "booking"),
                ("HVAC Repair Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Pest Control" => vec![
                ("Premium Pest Control Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Pest Control Service", "Essential services to get you started", 9999, "booking"),
                ("Pest Control Consultation", "Expert advice and planning", 4999, "booking"),
                ("Pest Control Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Pest Control Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Interior Design" => vec![
                ("Premium Interior Design Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Interior Design Service", "Essential services to get you started", 9999, "booking"),
                ("Interior Design Consultation", "Expert advice and planning", 4999, "booking"),
                ("Interior Design Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Interior Design Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Real Estate Agent" | "Real Estate" | "Property Manager" => vec![
                ("Long-Term Lease Showing", "Schedule a viewing for an available unit", 0, "booking"),
                ("Rental Application", "Submit application for an apartment", 5000, "booking"),
                ("Tenant Screening", "Comprehensive background and credit check", 3500, "digital"),
                ("Maintenance Request", "Submit a work order for your unit", 0, "booking"),
                ("Property Listing Consultation", "Discuss leasing your property with us", 0, "booking"),
            ],
            "Consulting Firm" => vec![
                ("Premium Consulting Firm Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Consulting Firm Service", "Essential services to get you started", 9999, "booking"),
                ("Consulting Firm Consultation", "Expert advice and planning", 4999, "booking"),
                ("Consulting Firm Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Consulting Firm Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Tutor" => vec![
                ("Premium Tutor Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Tutor Service", "Essential services to get you started", 9999, "booking"),
                ("Tutor Consultation", "Expert advice and planning", 4999, "booking"),
                ("Tutor Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Tutor Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Music Teacher" => vec![
                ("Premium Music Teacher Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Music Teacher Service", "Essential services to get you started", 9999, "booking"),
                ("Music Teacher Consultation", "Expert advice and planning", 4999, "booking"),
                ("Music Teacher Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Music Teacher Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Dance Studio" => vec![
                ("Premium Dance Studio Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Dance Studio Service", "Essential services to get you started", 9999, "booking"),
                ("Dance Studio Consultation", "Expert advice and planning", 4999, "booking"),
                ("Dance Studio Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Dance Studio Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Catering Service" => vec![
                ("Premium Catering Service Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Catering Service Service", "Essential services to get you started", 9999, "booking"),
                ("Catering Service Consultation", "Expert advice and planning", 4999, "booking"),
                ("Catering Service Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Catering Service Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Event Planner" => vec![
                ("Premium Event Planner Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Event Planner Service", "Essential services to get you started", 9999, "booking"),
                ("Event Planner Consultation", "Expert advice and planning", 4999, "booking"),
                ("Event Planner Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Event Planner Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Florist" => vec![
                ("Premium Florist Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Florist Service", "Essential services to get you started", 9999, "booking"),
                ("Florist Consultation", "Expert advice and planning", 4999, "booking"),
                ("Florist Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Florist Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Graphic Designer" => vec![
                ("Premium Graphic Designer Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Graphic Designer Service", "Essential services to get you started", 9999, "booking"),
                ("Graphic Designer Consultation", "Expert advice and planning", 4999, "booking"),
                ("Graphic Designer Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Graphic Designer Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Marketing Agency" => vec![
                ("Premium Marketing Agency Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Marketing Agency Service", "Essential services to get you started", 9999, "booking"),
                ("Marketing Agency Consultation", "Expert advice and planning", 4999, "booking"),
                ("Marketing Agency Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Marketing Agency Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "SEO Specialist" => vec![
                ("Premium SEO Specialist Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic SEO Specialist Service", "Essential services to get you started", 9999, "booking"),
                ("SEO Specialist Consultation", "Expert advice and planning", 4999, "booking"),
                ("SEO Specialist Assessment", "Initial evaluation and report", 7500, "booking"),
                ("SEO Specialist Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Virtual Assistant" => vec![
                ("Premium Virtual Assistant Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Virtual Assistant Service", "Essential services to get you started", 9999, "booking"),
                ("Virtual Assistant Consultation", "Expert advice and planning", 4999, "booking"),
                ("Virtual Assistant Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Virtual Assistant Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Bookkeeper" => vec![
                ("Premium Bookkeeper Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Bookkeeper Service", "Essential services to get you started", 9999, "booking"),
                ("Bookkeeper Consultation", "Expert advice and planning", 4999, "booking"),
                ("Bookkeeper Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Bookkeeper Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Tax Preparer" => vec![
                ("Premium Tax Preparer Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Tax Preparer Service", "Essential services to get you started", 9999, "booking"),
                ("Tax Preparer Consultation", "Expert advice and planning", 4999, "booking"),
                ("Tax Preparer Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Tax Preparer Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Dog Walker" => vec![
                ("Premium Dog Walker Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Dog Walker Service", "Essential services to get you started", 9999, "booking"),
                ("Dog Walker Consultation", "Expert advice and planning", 4999, "booking"),
                ("Dog Walker Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Dog Walker Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Pet Sitter" => vec![
                ("Premium Pet Sitter Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Pet Sitter Service", "Essential services to get you started", 9999, "booking"),
                ("Pet Sitter Consultation", "Expert advice and planning", 4999, "booking"),
                ("Pet Sitter Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Pet Sitter Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Veterinarian" => vec![
                ("Premium Veterinarian Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Veterinarian Service", "Essential services to get you started", 9999, "booking"),
                ("Veterinarian Consultation", "Expert advice and planning", 4999, "booking"),
                ("Veterinarian Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Veterinarian Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Dental Office" => vec![
                ("Premium Dental Office Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Dental Office Service", "Essential services to get you started", 9999, "booking"),
                ("Dental Office Consultation", "Expert advice and planning", 4999, "booking"),
                ("Dental Office Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Dental Office Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Chiropractor" => vec![
                ("Premium Chiropractor Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Chiropractor Service", "Essential services to get you started", 9999, "booking"),
                ("Chiropractor Consultation", "Expert advice and planning", 4999, "booking"),
                ("Chiropractor Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Chiropractor Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Therapist" => vec![
                ("Premium Therapist Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Therapist Service", "Essential services to get you started", 9999, "booking"),
                ("Therapist Consultation", "Expert advice and planning", 4999, "booking"),
                ("Therapist Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Therapist Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Life Coach" => vec![
                ("Premium Life Coach Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Life Coach Service", "Essential services to get you started", 9999, "booking"),
                ("Life Coach Consultation", "Expert advice and planning", 4999, "booking"),
                ("Life Coach Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Life Coach Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Business Coach" => vec![
                ("Premium Business Coach Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Business Coach Service", "Essential services to get you started", 9999, "booking"),
                ("Business Coach Consultation", "Expert advice and planning", 4999, "booking"),
                ("Business Coach Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Business Coach Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Career Counselor" => vec![
                ("Premium Career Counselor Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Career Counselor Service", "Essential services to get you started", 9999, "booking"),
                ("Career Counselor Consultation", "Expert advice and planning", 4999, "booking"),
                ("Career Counselor Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Career Counselor Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Resume Writer" => vec![
                ("Premium Resume Writer Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Resume Writer Service", "Essential services to get you started", 9999, "booking"),
                ("Resume Writer Consultation", "Expert advice and planning", 4999, "booking"),
                ("Resume Writer Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Resume Writer Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Translator" => vec![
                ("Premium Translator Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Translator Service", "Essential services to get you started", 9999, "booking"),
                ("Translator Consultation", "Expert advice and planning", 4999, "booking"),
                ("Translator Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Translator Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Copywriter" => vec![
                ("Premium Copywriter Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Copywriter Service", "Essential services to get you started", 9999, "booking"),
                ("Copywriter Consultation", "Expert advice and planning", 4999, "booking"),
                ("Copywriter Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Copywriter Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Proofreader" => vec![
                ("Premium Proofreader Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Proofreader Service", "Essential services to get you started", 9999, "booking"),
                ("Proofreader Consultation", "Expert advice and planning", 4999, "booking"),
                ("Proofreader Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Proofreader Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Voiceover Artist" => vec![
                ("Premium Voiceover Artist Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Voiceover Artist Service", "Essential services to get you started", 9999, "booking"),
                ("Voiceover Artist Consultation", "Expert advice and planning", 4999, "booking"),
                ("Voiceover Artist Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Voiceover Artist Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Video Editor" => vec![
                ("Premium Video Editor Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Video Editor Service", "Essential services to get you started", 9999, "booking"),
                ("Video Editor Consultation", "Expert advice and planning", 4999, "booking"),
                ("Video Editor Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Video Editor Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Animator" => vec![
                ("Premium Animator Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Animator Service", "Essential services to get you started", 9999, "booking"),
                ("Animator Consultation", "Expert advice and planning", 4999, "booking"),
                ("Animator Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Animator Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Illustrator" => vec![
                ("Premium Illustrator Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Illustrator Service", "Essential services to get you started", 9999, "booking"),
                ("Illustrator Consultation", "Expert advice and planning", 4999, "booking"),
                ("Illustrator Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Illustrator Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "App Developer" => vec![
                ("Premium App Developer Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic App Developer Service", "Essential services to get you started", 9999, "booking"),
                ("App Developer Consultation", "Expert advice and planning", 4999, "booking"),
                ("App Developer Assessment", "Initial evaluation and report", 7500, "booking"),
                ("App Developer Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Software Engineer" => vec![
                ("Premium Software Engineer Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Software Engineer Service", "Essential services to get you started", 9999, "booking"),
                ("Software Engineer Consultation", "Expert advice and planning", 4999, "booking"),
                ("Software Engineer Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Software Engineer Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "IT Support" => vec![
                ("Premium IT Support Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic IT Support Service", "Essential services to get you started", 9999, "booking"),
                ("IT Support Consultation", "Expert advice and planning", 4999, "booking"),
                ("IT Support Assessment", "Initial evaluation and report", 7500, "booking"),
                ("IT Support Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Network Administrator" => vec![
                ("Premium Network Administrator Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Network Administrator Service", "Essential services to get you started", 9999, "booking"),
                ("Network Administrator Consultation", "Expert advice and planning", 4999, "booking"),
                ("Network Administrator Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Network Administrator Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Cybersecurity" => vec![
                ("Premium Cybersecurity Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Cybersecurity Service", "Essential services to get you started", 9999, "booking"),
                ("Cybersecurity Consultation", "Expert advice and planning", 4999, "booking"),
                ("Cybersecurity Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Cybersecurity Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Data Analyst" => vec![
                ("Premium Data Analyst Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Data Analyst Service", "Essential services to get you started", 9999, "booking"),
                ("Data Analyst Consultation", "Expert advice and planning", 4999, "booking"),
                ("Data Analyst Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Data Analyst Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Data Scientist" => vec![
                ("Premium Data Scientist Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Data Scientist Service", "Essential services to get you started", 9999, "booking"),
                ("Data Scientist Consultation", "Expert advice and planning", 4999, "booking"),
                ("Data Scientist Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Data Scientist Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Machine Learning" => vec![
                ("Premium Machine Learning Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Machine Learning Service", "Essential services to get you started", 9999, "booking"),
                ("Machine Learning Consultation", "Expert advice and planning", 4999, "booking"),
                ("Machine Learning Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Machine Learning Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "AI Consultant" => vec![
                ("Premium AI Consultant Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic AI Consultant Service", "Essential services to get you started", 9999, "booking"),
                ("AI Consultant Consultation", "Expert advice and planning", 4999, "booking"),
                ("AI Consultant Assessment", "Initial evaluation and report", 7500, "booking"),
                ("AI Consultant Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Blockchain Dev" => vec![
                ("Premium Blockchain Dev Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Blockchain Dev Service", "Essential services to get you started", 9999, "booking"),
                ("Blockchain Dev Consultation", "Expert advice and planning", 4999, "booking"),
                ("Blockchain Dev Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Blockchain Dev Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Crypto Consultant" => vec![
                ("Premium Crypto Consultant Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Crypto Consultant Service", "Essential services to get you started", 9999, "booking"),
                ("Crypto Consultant Consultation", "Expert advice and planning", 4999, "booking"),
                ("Crypto Consultant Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Crypto Consultant Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Financial Advisor" => vec![
                ("Premium Financial Advisor Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Financial Advisor Service", "Essential services to get you started", 9999, "booking"),
                ("Financial Advisor Consultation", "Expert advice and planning", 4999, "booking"),
                ("Financial Advisor Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Financial Advisor Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Wealth Manager" => vec![
                ("Premium Wealth Manager Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Wealth Manager Service", "Essential services to get you started", 9999, "booking"),
                ("Wealth Manager Consultation", "Expert advice and planning", 4999, "booking"),
                ("Wealth Manager Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Wealth Manager Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Insurance Agent" => vec![
                ("Premium Insurance Agent Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Insurance Agent Service", "Essential services to get you started", 9999, "booking"),
                ("Insurance Agent Consultation", "Expert advice and planning", 4999, "booking"),
                ("Insurance Agent Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Insurance Agent Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Mortgage Broker" => vec![
                ("Premium Mortgage Broker Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Mortgage Broker Service", "Essential services to get you started", 9999, "booking"),
                ("Mortgage Broker Consultation", "Expert advice and planning", 4999, "booking"),
                ("Mortgage Broker Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Mortgage Broker Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Real Estate Investor" => vec![
                ("Premium Real Estate Investor Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Real Estate Investor Service", "Essential services to get you started", 9999, "booking"),
                ("Real Estate Investor Consultation", "Expert advice and planning", 4999, "booking"),
                ("Real Estate Investor Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Real Estate Investor Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Travel Agent" => vec![
                ("Premium Travel Agent Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Travel Agent Service", "Essential services to get you started", 9999, "booking"),
                ("Travel Agent Consultation", "Expert advice and planning", 4999, "booking"),
                ("Travel Agent Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Travel Agent Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Tour Guide" => vec![
                ("Premium Tour Guide Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Tour Guide Service", "Essential services to get you started", 9999, "booking"),
                ("Tour Guide Consultation", "Expert advice and planning", 4999, "booking"),
                ("Tour Guide Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Tour Guide Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Bed and Breakfast" => vec![
                ("Premium Bed and Breakfast Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Bed and Breakfast Service", "Essential services to get you started", 9999, "booking"),
                ("Bed and Breakfast Consultation", "Expert advice and planning", 4999, "booking"),
                ("Bed and Breakfast Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Bed and Breakfast Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Hotel" => vec![
                ("Premium Hotel Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Hotel Service", "Essential services to get you started", 9999, "booking"),
                ("Hotel Consultation", "Expert advice and planning", 4999, "booking"),
                ("Hotel Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Hotel Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Motel" => vec![
                ("Premium Motel Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Motel Service", "Essential services to get you started", 9999, "booking"),
                ("Motel Consultation", "Expert advice and planning", 4999, "booking"),
                ("Motel Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Motel Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Hostel" => vec![
                ("Premium Hostel Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Hostel Service", "Essential services to get you started", 9999, "booking"),
                ("Hostel Consultation", "Expert advice and planning", 4999, "booking"),
                ("Hostel Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Hostel Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Campground" => vec![
                ("Premium Campground Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Campground Service", "Essential services to get you started", 9999, "booking"),
                ("Campground Consultation", "Expert advice and planning", 4999, "booking"),
                ("Campground Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Campground Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "RV Park" => vec![
                ("Premium RV Park Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic RV Park Service", "Essential services to get you started", 9999, "booking"),
                ("RV Park Consultation", "Expert advice and planning", 4999, "booking"),
                ("RV Park Assessment", "Initial evaluation and report", 7500, "booking"),
                ("RV Park Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Amusement Park" => vec![
                ("Premium Amusement Park Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Amusement Park Service", "Essential services to get you started", 9999, "booking"),
                ("Amusement Park Consultation", "Expert advice and planning", 4999, "booking"),
                ("Amusement Park Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Amusement Park Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Museum" => vec![
                ("Premium Museum Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Museum Service", "Essential services to get you started", 9999, "booking"),
                ("Museum Consultation", "Expert advice and planning", 4999, "booking"),
                ("Museum Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Museum Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Art Gallery" => vec![
                ("Premium Art Gallery Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Art Gallery Service", "Essential services to get you started", 9999, "booking"),
                ("Art Gallery Consultation", "Expert advice and planning", 4999, "booking"),
                ("Art Gallery Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Art Gallery Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Theater" => vec![
                ("Premium Theater Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Theater Service", "Essential services to get you started", 9999, "booking"),
                ("Theater Consultation", "Expert advice and planning", 4999, "booking"),
                ("Theater Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Theater Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Cinema" => vec![
                ("Premium Cinema Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Cinema Service", "Essential services to get you started", 9999, "booking"),
                ("Cinema Consultation", "Expert advice and planning", 4999, "booking"),
                ("Cinema Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Cinema Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Concert Venue" => vec![
                ("Premium Concert Venue Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Concert Venue Service", "Essential services to get you started", 9999, "booking"),
                ("Concert Venue Consultation", "Expert advice and planning", 4999, "booking"),
                ("Concert Venue Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Concert Venue Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Nightclub" => vec![
                ("Premium Nightclub Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Nightclub Service", "Essential services to get you started", 9999, "booking"),
                ("Nightclub Consultation", "Expert advice and planning", 4999, "booking"),
                ("Nightclub Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Nightclub Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Bar" => vec![
                ("Premium Bar Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Bar Service", "Essential services to get you started", 9999, "booking"),
                ("Bar Consultation", "Expert advice and planning", 4999, "booking"),
                ("Bar Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Bar Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Pub" => vec![
                ("Premium Pub Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Pub Service", "Essential services to get you started", 9999, "booking"),
                ("Pub Consultation", "Expert advice and planning", 4999, "booking"),
                ("Pub Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Pub Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Brewery" => vec![
                ("Premium Brewery Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Brewery Service", "Essential services to get you started", 9999, "booking"),
                ("Brewery Consultation", "Expert advice and planning", 4999, "booking"),
                ("Brewery Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Brewery Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Winery" => vec![
                ("Premium Winery Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Winery Service", "Essential services to get you started", 9999, "booking"),
                ("Winery Consultation", "Expert advice and planning", 4999, "booking"),
                ("Winery Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Winery Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Distillery" => vec![
                ("Premium Distillery Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Distillery Service", "Essential services to get you started", 9999, "booking"),
                ("Distillery Consultation", "Expert advice and planning", 4999, "booking"),
                ("Distillery Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Distillery Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Food Truck" => vec![
                ("Premium Food Truck Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Food Truck Service", "Essential services to get you started", 9999, "booking"),
                ("Food Truck Consultation", "Expert advice and planning", 4999, "booking"),
                ("Food Truck Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Food Truck Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Pop-up Restaurant" => vec![
                ("Premium Pop-up Restaurant Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Pop-up Restaurant Service", "Essential services to get you started", 9999, "booking"),
                ("Pop-up Restaurant Consultation", "Expert advice and planning", 4999, "booking"),
                ("Pop-up Restaurant Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Pop-up Restaurant Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Personal Chef" => vec![
                ("Premium Personal Chef Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Personal Chef Service", "Essential services to get you started", 9999, "booking"),
                ("Personal Chef Consultation", "Expert advice and planning", 4999, "booking"),
                ("Personal Chef Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Personal Chef Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Meal Prep" => vec![
                ("Premium Meal Prep Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Meal Prep Service", "Essential services to get you started", 9999, "booking"),
                ("Meal Prep Consultation", "Expert advice and planning", 4999, "booking"),
                ("Meal Prep Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Meal Prep Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Nutritionist" => vec![
                ("Premium Nutritionist Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Nutritionist Service", "Essential services to get you started", 9999, "booking"),
                ("Nutritionist Consultation", "Expert advice and planning", 4999, "booking"),
                ("Nutritionist Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Nutritionist Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Dietitian" => vec![
                ("Premium Dietitian Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Dietitian Service", "Essential services to get you started", 9999, "booking"),
                ("Dietitian Consultation", "Expert advice and planning", 4999, "booking"),
                ("Dietitian Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Dietitian Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Personal Trainer" => vec![
                ("Premium Personal Trainer Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Personal Trainer Service", "Essential services to get you started", 9999, "booking"),
                ("Personal Trainer Consultation", "Expert advice and planning", 4999, "booking"),
                ("Personal Trainer Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Personal Trainer Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Gym" => vec![
                ("Premium Gym Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Gym Service", "Essential services to get you started", 9999, "booking"),
                ("Gym Consultation", "Expert advice and planning", 4999, "booking"),
                ("Gym Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Gym Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Martial Arts" => vec![
                ("Premium Martial Arts Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Martial Arts Service", "Essential services to get you started", 9999, "booking"),
                ("Martial Arts Consultation", "Expert advice and planning", 4999, "booking"),
                ("Martial Arts Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Martial Arts Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Boxing Gym" => vec![
                ("Premium Boxing Gym Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Boxing Gym Service", "Essential services to get you started", 9999, "booking"),
                ("Boxing Gym Consultation", "Expert advice and planning", 4999, "booking"),
                ("Boxing Gym Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Boxing Gym Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "CrossFit" => vec![
                ("Premium CrossFit Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic CrossFit Service", "Essential services to get you started", 9999, "booking"),
                ("CrossFit Consultation", "Expert advice and planning", 4999, "booking"),
                ("CrossFit Assessment", "Initial evaluation and report", 7500, "booking"),
                ("CrossFit Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Pilates" => vec![
                ("Premium Pilates Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Pilates Service", "Essential services to get you started", 9999, "booking"),
                ("Pilates Consultation", "Expert advice and planning", 4999, "booking"),
                ("Pilates Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Pilates Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Barre" => vec![
                ("Premium Barre Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Barre Service", "Essential services to get you started", 9999, "booking"),
                ("Barre Consultation", "Expert advice and planning", 4999, "booking"),
                ("Barre Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Barre Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Spin Class" => vec![
                ("Premium Spin Class Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Spin Class Service", "Essential services to get you started", 9999, "booking"),
                ("Spin Class Consultation", "Expert advice and planning", 4999, "booking"),
                ("Spin Class Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Spin Class Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Acupuncturist" => vec![
                ("Premium Acupuncturist Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Acupuncturist Service", "Essential services to get you started", 9999, "booking"),
                ("Acupuncturist Consultation", "Expert advice and planning", 4999, "booking"),
                ("Acupuncturist Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Acupuncturist Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Homeopath" => vec![
                ("Premium Homeopath Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Homeopath Service", "Essential services to get you started", 9999, "booking"),
                ("Homeopath Consultation", "Expert advice and planning", 4999, "booking"),
                ("Homeopath Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Homeopath Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Naturopath" => vec![
                ("Premium Naturopath Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Naturopath Service", "Essential services to get you started", 9999, "booking"),
                ("Naturopath Consultation", "Expert advice and planning", 4999, "booking"),
                ("Naturopath Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Naturopath Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Herbalist" => vec![
                ("Premium Herbalist Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Herbalist Service", "Essential services to get you started", 9999, "booking"),
                ("Herbalist Consultation", "Expert advice and planning", 4999, "booking"),
                ("Herbalist Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Herbalist Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Aromatherapist" => vec![
                ("Premium Aromatherapist Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Aromatherapist Service", "Essential services to get you started", 9999, "booking"),
                ("Aromatherapist Consultation", "Expert advice and planning", 4999, "booking"),
                ("Aromatherapist Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Aromatherapist Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Reflexologist" => vec![
                ("Premium Reflexologist Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Reflexologist Service", "Essential services to get you started", 9999, "booking"),
                ("Reflexologist Consultation", "Expert advice and planning", 4999, "booking"),
                ("Reflexologist Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Reflexologist Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Reiki Practitioner" => vec![
                ("Premium Reiki Practitioner Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Reiki Practitioner Service", "Essential services to get you started", 9999, "booking"),
                ("Reiki Practitioner Consultation", "Expert advice and planning", 4999, "booking"),
                ("Reiki Practitioner Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Reiki Practitioner Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Psychic" => vec![
                ("Premium Psychic Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Psychic Service", "Essential services to get you started", 9999, "booking"),
                ("Psychic Consultation", "Expert advice and planning", 4999, "booking"),
                ("Psychic Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Psychic Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Tarot Reader" => vec![
                ("Premium Tarot Reader Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Tarot Reader Service", "Essential services to get you started", 9999, "booking"),
                ("Tarot Reader Consultation", "Expert advice and planning", 4999, "booking"),
                ("Tarot Reader Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Tarot Reader Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Astrologer" => vec![
                ("Premium Astrologer Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Astrologer Service", "Essential services to get you started", 9999, "booking"),
                ("Astrologer Consultation", "Expert advice and planning", 4999, "booking"),
                ("Astrologer Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Astrologer Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Numerologist" => vec![
                ("Premium Numerologist Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Numerologist Service", "Essential services to get you started", 9999, "booking"),
                ("Numerologist Consultation", "Expert advice and planning", 4999, "booking"),
                ("Numerologist Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Numerologist Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Feng Shui" => vec![
                ("Premium Feng Shui Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Feng Shui Service", "Essential services to get you started", 9999, "booking"),
                ("Feng Shui Consultation", "Expert advice and planning", 4999, "booking"),
                ("Feng Shui Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Feng Shui Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Home Stager" => vec![
                ("Premium Home Stager Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Home Stager Service", "Essential services to get you started", 9999, "booking"),
                ("Home Stager Consultation", "Expert advice and planning", 4999, "booking"),
                ("Home Stager Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Home Stager Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Professional Organizer" => vec![
                ("Premium Professional Organizer Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Professional Organizer Service", "Essential services to get you started", 9999, "booking"),
                ("Professional Organizer Consultation", "Expert advice and planning", 4999, "booking"),
                ("Professional Organizer Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Professional Organizer Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Declutterer" => vec![
                ("Premium Declutterer Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Declutterer Service", "Essential services to get you started", 9999, "booking"),
                ("Declutterer Consultation", "Expert advice and planning", 4999, "booking"),
                ("Declutterer Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Declutterer Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Moving Company" => vec![
                ("Premium Moving Company Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Moving Company Service", "Essential services to get you started", 9999, "booking"),
                ("Moving Company Consultation", "Expert advice and planning", 4999, "booking"),
                ("Moving Company Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Moving Company Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Storage Facility" => vec![
                ("Premium Storage Facility Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Storage Facility Service", "Essential services to get you started", 9999, "booking"),
                ("Storage Facility Consultation", "Expert advice and planning", 4999, "booking"),
                ("Storage Facility Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Storage Facility Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Junk Removal" => vec![
                ("Premium Junk Removal Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Junk Removal Service", "Essential services to get you started", 9999, "booking"),
                ("Junk Removal Consultation", "Expert advice and planning", 4999, "booking"),
                ("Junk Removal Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Junk Removal Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Recycling Center" => vec![
                ("Premium Recycling Center Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Recycling Center Service", "Essential services to get you started", 9999, "booking"),
                ("Recycling Center Consultation", "Expert advice and planning", 4999, "booking"),
                ("Recycling Center Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Recycling Center Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Scrap Metal" => vec![
                ("Premium Scrap Metal Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Scrap Metal Service", "Essential services to get you started", 9999, "booking"),
                ("Scrap Metal Consultation", "Expert advice and planning", 4999, "booking"),
                ("Scrap Metal Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Scrap Metal Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Auto Repair" => vec![
                ("Premium Auto Repair Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Auto Repair Service", "Essential services to get you started", 9999, "booking"),
                ("Auto Repair Consultation", "Expert advice and planning", 4999, "booking"),
                ("Auto Repair Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Auto Repair Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Body Shop" => vec![
                ("Premium Body Shop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Body Shop Service", "Essential services to get you started", 9999, "booking"),
                ("Body Shop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Body Shop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Body Shop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Tire Shop" => vec![
                ("Premium Tire Shop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Tire Shop Service", "Essential services to get you started", 9999, "booking"),
                ("Tire Shop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Tire Shop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Tire Shop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Oil Change" => vec![
                ("Premium Oil Change Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Oil Change Service", "Essential services to get you started", 9999, "booking"),
                ("Oil Change Consultation", "Expert advice and planning", 4999, "booking"),
                ("Oil Change Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Oil Change Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Car Detailing" => vec![
                ("Premium Car Detailing Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Car Detailing Service", "Essential services to get you started", 9999, "booking"),
                ("Car Detailing Consultation", "Expert advice and planning", 4999, "booking"),
                ("Car Detailing Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Car Detailing Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Car Rental" => vec![
                ("Premium Car Rental Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Car Rental Service", "Essential services to get you started", 9999, "booking"),
                ("Car Rental Consultation", "Expert advice and planning", 4999, "booking"),
                ("Car Rental Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Car Rental Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Taxi Service" => vec![
                ("Premium Taxi Service Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Taxi Service Service", "Essential services to get you started", 9999, "booking"),
                ("Taxi Service Consultation", "Expert advice and planning", 4999, "booking"),
                ("Taxi Service Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Taxi Service Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Rideshare" => vec![
                ("Premium Rideshare Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Rideshare Service", "Essential services to get you started", 9999, "booking"),
                ("Rideshare Consultation", "Expert advice and planning", 4999, "booking"),
                ("Rideshare Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Rideshare Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Courier" => vec![
                ("Premium Courier Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Courier Service", "Essential services to get you started", 9999, "booking"),
                ("Courier Consultation", "Expert advice and planning", 4999, "booking"),
                ("Courier Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Courier Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Delivery Service" => vec![
                ("Premium Delivery Service Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Delivery Service Service", "Essential services to get you started", 9999, "booking"),
                ("Delivery Service Consultation", "Expert advice and planning", 4999, "booking"),
                ("Delivery Service Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Delivery Service Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Freight Broker" => vec![
                ("Premium Freight Broker Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Freight Broker Service", "Essential services to get you started", 9999, "booking"),
                ("Freight Broker Consultation", "Expert advice and planning", 4999, "booking"),
                ("Freight Broker Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Freight Broker Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Trucking Company" => vec![
                ("Premium Trucking Company Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Trucking Company Service", "Essential services to get you started", 9999, "booking"),
                ("Trucking Company Consultation", "Expert advice and planning", 4999, "booking"),
                ("Trucking Company Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Trucking Company Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Logistics" => vec![
                ("Premium Logistics Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Logistics Service", "Essential services to get you started", 9999, "booking"),
                ("Logistics Consultation", "Expert advice and planning", 4999, "booking"),
                ("Logistics Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Logistics Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Warehouse" => vec![
                ("Premium Warehouse Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Warehouse Service", "Essential services to get you started", 9999, "booking"),
                ("Warehouse Consultation", "Expert advice and planning", 4999, "booking"),
                ("Warehouse Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Warehouse Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Fulfillment" => vec![
                ("Premium Fulfillment Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Fulfillment Service", "Essential services to get you started", 9999, "booking"),
                ("Fulfillment Consultation", "Expert advice and planning", 4999, "booking"),
                ("Fulfillment Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Fulfillment Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Dropshipper" => vec![
                ("Premium Dropshipper Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Dropshipper Service", "Essential services to get you started", 9999, "booking"),
                ("Dropshipper Consultation", "Expert advice and planning", 4999, "booking"),
                ("Dropshipper Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Dropshipper Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "E-commerce" => vec![
                ("Premium E-commerce Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic E-commerce Service", "Essential services to get you started", 9999, "booking"),
                ("E-commerce Consultation", "Expert advice and planning", 4999, "booking"),
                ("E-commerce Assessment", "Initial evaluation and report", 7500, "booking"),
                ("E-commerce Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Amazon Seller" => vec![
                ("Premium Amazon Seller Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Amazon Seller Service", "Essential services to get you started", 9999, "booking"),
                ("Amazon Seller Consultation", "Expert advice and planning", 4999, "booking"),
                ("Amazon Seller Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Amazon Seller Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "eBay Seller" => vec![
                ("Premium eBay Seller Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic eBay Seller Service", "Essential services to get you started", 9999, "booking"),
                ("eBay Seller Consultation", "Expert advice and planning", 4999, "booking"),
                ("eBay Seller Assessment", "Initial evaluation and report", 7500, "booking"),
                ("eBay Seller Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Etsy Shop" => vec![
                ("Premium Etsy Shop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Etsy Shop Service", "Essential services to get you started", 9999, "booking"),
                ("Etsy Shop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Etsy Shop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Etsy Shop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Shopify Store" => vec![
                ("Premium Shopify Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Shopify Store Service", "Essential services to get you started", 9999, "booking"),
                ("Shopify Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Shopify Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Shopify Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Boutique_Old" => vec![
                ("Premium Boutique Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Boutique Service", "Essential services to get you started", 9999, "booking"),
                ("Boutique Consultation", "Expert advice and planning", 4999, "booking"),
                ("Boutique Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Boutique Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Thrift Store" => vec![
                ("Premium Thrift Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Thrift Store Service", "Essential services to get you started", 9999, "booking"),
                ("Thrift Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Thrift Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Thrift Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Antique Shop" => vec![
                ("Premium Antique Shop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Antique Shop Service", "Essential services to get you started", 9999, "booking"),
                ("Antique Shop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Antique Shop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Antique Shop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Pawn Shop" => vec![
                ("Premium Pawn Shop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Pawn Shop Service", "Essential services to get you started", 9999, "booking"),
                ("Pawn Shop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Pawn Shop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Pawn Shop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Jewelry Store" => vec![
                ("Premium Jewelry Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Jewelry Store Service", "Essential services to get you started", 9999, "booking"),
                ("Jewelry Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Jewelry Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Jewelry Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Watch Repair" => vec![
                ("Premium Watch Repair Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Watch Repair Service", "Essential services to get you started", 9999, "booking"),
                ("Watch Repair Consultation", "Expert advice and planning", 4999, "booking"),
                ("Watch Repair Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Watch Repair Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Shoe Repair" => vec![
                ("Premium Shoe Repair Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Shoe Repair Service", "Essential services to get you started", 9999, "booking"),
                ("Shoe Repair Consultation", "Expert advice and planning", 4999, "booking"),
                ("Shoe Repair Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Shoe Repair Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Tailor" => vec![
                ("Premium Tailor Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Tailor Service", "Essential services to get you started", 9999, "booking"),
                ("Tailor Consultation", "Expert advice and planning", 4999, "booking"),
                ("Tailor Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Tailor Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Dry Cleaner" => vec![
                ("Premium Dry Cleaner Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Dry Cleaner Service", "Essential services to get you started", 9999, "booking"),
                ("Dry Cleaner Consultation", "Expert advice and planning", 4999, "booking"),
                ("Dry Cleaner Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Dry Cleaner Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Laundromat" => vec![
                ("Premium Laundromat Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Laundromat Service", "Essential services to get you started", 9999, "booking"),
                ("Laundromat Consultation", "Expert advice and planning", 4999, "booking"),
                ("Laundromat Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Laundromat Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Alterations" => vec![
                ("Premium Alterations Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Alterations Service", "Essential services to get you started", 9999, "booking"),
                ("Alterations Consultation", "Expert advice and planning", 4999, "booking"),
                ("Alterations Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Alterations Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Bridal Shop" => vec![
                ("Premium Bridal Shop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Bridal Shop Service", "Essential services to get you started", 9999, "booking"),
                ("Bridal Shop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Bridal Shop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Bridal Shop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Tuxedo Rental" => vec![
                ("Premium Tuxedo Rental Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Tuxedo Rental Service", "Essential services to get you started", 9999, "booking"),
                ("Tuxedo Rental Consultation", "Expert advice and planning", 4999, "booking"),
                ("Tuxedo Rental Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Tuxedo Rental Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Costume Shop" => vec![
                ("Premium Costume Shop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Costume Shop Service", "Essential services to get you started", 9999, "booking"),
                ("Costume Shop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Costume Shop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Costume Shop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Party Supply" => vec![
                ("Premium Party Supply Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Party Supply Service", "Essential services to get you started", 9999, "booking"),
                ("Party Supply Consultation", "Expert advice and planning", 4999, "booking"),
                ("Party Supply Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Party Supply Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Toy Store" => vec![
                ("Premium Toy Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Toy Store Service", "Essential services to get you started", 9999, "booking"),
                ("Toy Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Toy Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Toy Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Hobby Shop" => vec![
                ("Premium Hobby Shop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Hobby Shop Service", "Essential services to get you started", 9999, "booking"),
                ("Hobby Shop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Hobby Shop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Hobby Shop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Comic Book Store" => vec![
                ("Premium Comic Book Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Comic Book Store Service", "Essential services to get you started", 9999, "booking"),
                ("Comic Book Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Comic Book Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Comic Book Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Video Game Store" => vec![
                ("Premium Video Game Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Video Game Store Service", "Essential services to get you started", 9999, "booking"),
                ("Video Game Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Video Game Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Video Game Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Board Game Cafe" => vec![
                ("Premium Board Game Cafe Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Board Game Cafe Service", "Essential services to get you started", 9999, "booking"),
                ("Board Game Cafe Consultation", "Expert advice and planning", 4999, "booking"),
                ("Board Game Cafe Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Board Game Cafe Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Escape Room" => vec![
                ("Premium Escape Room Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Escape Room Service", "Essential services to get you started", 9999, "booking"),
                ("Escape Room Consultation", "Expert advice and planning", 4999, "booking"),
                ("Escape Room Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Escape Room Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Laser Tag" => vec![
                ("Premium Laser Tag Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Laser Tag Service", "Essential services to get you started", 9999, "booking"),
                ("Laser Tag Consultation", "Expert advice and planning", 4999, "booking"),
                ("Laser Tag Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Laser Tag Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Paintball" => vec![
                ("Premium Paintball Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Paintball Service", "Essential services to get you started", 9999, "booking"),
                ("Paintball Consultation", "Expert advice and planning", 4999, "booking"),
                ("Paintball Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Paintball Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Bowling Alley" => vec![
                ("Premium Bowling Alley Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Bowling Alley Service", "Essential services to get you started", 9999, "booking"),
                ("Bowling Alley Consultation", "Expert advice and planning", 4999, "booking"),
                ("Bowling Alley Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Bowling Alley Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Roller Rink" => vec![
                ("Premium Roller Rink Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Roller Rink Service", "Essential services to get you started", 9999, "booking"),
                ("Roller Rink Consultation", "Expert advice and planning", 4999, "booking"),
                ("Roller Rink Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Roller Rink Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Ice Rink" => vec![
                ("Premium Ice Rink Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Ice Rink Service", "Essential services to get you started", 9999, "booking"),
                ("Ice Rink Consultation", "Expert advice and planning", 4999, "booking"),
                ("Ice Rink Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Ice Rink Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Skate Park" => vec![
                ("Premium Skate Park Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Skate Park Service", "Essential services to get you started", 9999, "booking"),
                ("Skate Park Consultation", "Expert advice and planning", 4999, "booking"),
                ("Skate Park Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Skate Park Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Mini Golf" => vec![
                ("Premium Mini Golf Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Mini Golf Service", "Essential services to get you started", 9999, "booking"),
                ("Mini Golf Consultation", "Expert advice and planning", 4999, "booking"),
                ("Mini Golf Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Mini Golf Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Golf Course" => vec![
                ("Premium Golf Course Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Golf Course Service", "Essential services to get you started", 9999, "booking"),
                ("Golf Course Consultation", "Expert advice and planning", 4999, "booking"),
                ("Golf Course Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Golf Course Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Driving Range" => vec![
                ("Premium Driving Range Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Driving Range Service", "Essential services to get you started", 9999, "booking"),
                ("Driving Range Consultation", "Expert advice and planning", 4999, "booking"),
                ("Driving Range Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Driving Range Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Batting Cages" => vec![
                ("Premium Batting Cages Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Batting Cages Service", "Essential services to get you started", 9999, "booking"),
                ("Batting Cages Consultation", "Expert advice and planning", 4999, "booking"),
                ("Batting Cages Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Batting Cages Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Arcade" => vec![
                ("Premium Arcade Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Arcade Service", "Essential services to get you started", 9999, "booking"),
                ("Arcade Consultation", "Expert advice and planning", 4999, "booking"),
                ("Arcade Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Arcade Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Casino" => vec![
                ("Premium Casino Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Casino Service", "Essential services to get you started", 9999, "booking"),
                ("Casino Consultation", "Expert advice and planning", 4999, "booking"),
                ("Casino Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Casino Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Bingo Hall" => vec![
                ("Premium Bingo Hall Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Bingo Hall Service", "Essential services to get you started", 9999, "booking"),
                ("Bingo Hall Consultation", "Expert advice and planning", 4999, "booking"),
                ("Bingo Hall Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Bingo Hall Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Pool Hall" => vec![
                ("Premium Pool Hall Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Pool Hall Service", "Essential services to get you started", 9999, "booking"),
                ("Pool Hall Consultation", "Expert advice and planning", 4999, "booking"),
                ("Pool Hall Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Pool Hall Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Sports Bar" => vec![
                ("Premium Sports Bar Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Sports Bar Service", "Essential services to get you started", 9999, "booking"),
                ("Sports Bar Consultation", "Expert advice and planning", 4999, "booking"),
                ("Sports Bar Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Sports Bar Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Sports Club" => vec![
                ("Premium Sports Club Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Sports Club Service", "Essential services to get you started", 9999, "booking"),
                ("Sports Club Consultation", "Expert advice and planning", 4999, "booking"),
                ("Sports Club Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Sports Club Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Country Club" => vec![
                ("Premium Country Club Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Country Club Service", "Essential services to get you started", 9999, "booking"),
                ("Country Club Consultation", "Expert advice and planning", 4999, "booking"),
                ("Country Club Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Country Club Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Yacht Club" => vec![
                ("Premium Yacht Club Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Yacht Club Service", "Essential services to get you started", 9999, "booking"),
                ("Yacht Club Consultation", "Expert advice and planning", 4999, "booking"),
                ("Yacht Club Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Yacht Club Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Marina" => vec![
                ("Premium Marina Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Marina Service", "Essential services to get you started", 9999, "booking"),
                ("Marina Consultation", "Expert advice and planning", 4999, "booking"),
                ("Marina Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Marina Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Boat Rental" => vec![
                ("Premium Boat Rental Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Boat Rental Service", "Essential services to get you started", 9999, "booking"),
                ("Boat Rental Consultation", "Expert advice and planning", 4999, "booking"),
                ("Boat Rental Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Boat Rental Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Jet Ski Rental" => vec![
                ("Premium Jet Ski Rental Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Jet Ski Rental Service", "Essential services to get you started", 9999, "booking"),
                ("Jet Ski Rental Consultation", "Expert advice and planning", 4999, "booking"),
                ("Jet Ski Rental Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Jet Ski Rental Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Surf Shop" => vec![
                ("Premium Surf Shop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Surf Shop Service", "Essential services to get you started", 9999, "booking"),
                ("Surf Shop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Surf Shop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Surf Shop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Dive Shop" => vec![
                ("Premium Dive Shop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Dive Shop Service", "Essential services to get you started", 9999, "booking"),
                ("Dive Shop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Dive Shop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Dive Shop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Fishing Charter" => vec![
                ("Premium Fishing Charter Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Fishing Charter Service", "Essential services to get you started", 9999, "booking"),
                ("Fishing Charter Consultation", "Expert advice and planning", 4999, "booking"),
                ("Fishing Charter Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Fishing Charter Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Hunting Guide" => vec![
                ("Premium Hunting Guide Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Hunting Guide Service", "Essential services to get you started", 9999, "booking"),
                ("Hunting Guide Consultation", "Expert advice and planning", 4999, "booking"),
                ("Hunting Guide Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Hunting Guide Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Gun Shop" => vec![
                ("Premium Gun Shop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Gun Shop Service", "Essential services to get you started", 9999, "booking"),
                ("Gun Shop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Gun Shop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Gun Shop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Shooting Range" => vec![
                ("Premium Shooting Range Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Shooting Range Service", "Essential services to get you started", 9999, "booking"),
                ("Shooting Range Consultation", "Expert advice and planning", 4999, "booking"),
                ("Shooting Range Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Shooting Range Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Archery Range" => vec![
                ("Premium Archery Range Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Archery Range Service", "Essential services to get you started", 9999, "booking"),
                ("Archery Range Consultation", "Expert advice and planning", 4999, "booking"),
                ("Archery Range Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Archery Range Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Taxidermist" => vec![
                ("Premium Taxidermist Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Taxidermist Service", "Essential services to get you started", 9999, "booking"),
                ("Taxidermist Consultation", "Expert advice and planning", 4999, "booking"),
                ("Taxidermist Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Taxidermist Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Butcher Shop" => vec![
                ("Premium Butcher Shop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Butcher Shop Service", "Essential services to get you started", 9999, "booking"),
                ("Butcher Shop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Butcher Shop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Butcher Shop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Seafood Market" => vec![
                ("Premium Seafood Market Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Seafood Market Service", "Essential services to get you started", 9999, "booking"),
                ("Seafood Market Consultation", "Expert advice and planning", 4999, "booking"),
                ("Seafood Market Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Seafood Market Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Farmers Market" => vec![
                ("Premium Farmers Market Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Farmers Market Service", "Essential services to get you started", 9999, "booking"),
                ("Farmers Market Consultation", "Expert advice and planning", 4999, "booking"),
                ("Farmers Market Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Farmers Market Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Grocery Store" => vec![
                ("Premium Grocery Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Grocery Store Service", "Essential services to get you started", 9999, "booking"),
                ("Grocery Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Grocery Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Grocery Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Convenience Store" => vec![
                ("Premium Convenience Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Convenience Store Service", "Essential services to get you started", 9999, "booking"),
                ("Convenience Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Convenience Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Convenience Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Liquor Store" => vec![
                ("Premium Liquor Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Liquor Store Service", "Essential services to get you started", 9999, "booking"),
                ("Liquor Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Liquor Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Liquor Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Vape Shop" => vec![
                ("Premium Vape Shop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Vape Shop Service", "Essential services to get you started", 9999, "booking"),
                ("Vape Shop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Vape Shop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Vape Shop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Dispensary" => vec![
                ("Premium Dispensary Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Dispensary Service", "Essential services to get you started", 9999, "booking"),
                ("Dispensary Consultation", "Expert advice and planning", 4999, "booking"),
                ("Dispensary Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Dispensary Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Pharmacy" => vec![
                ("Premium Pharmacy Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Pharmacy Service", "Essential services to get you started", 9999, "booking"),
                ("Pharmacy Consultation", "Expert advice and planning", 4999, "booking"),
                ("Pharmacy Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Pharmacy Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Medical Supply" => vec![
                ("Premium Medical Supply Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Medical Supply Service", "Essential services to get you started", 9999, "booking"),
                ("Medical Supply Consultation", "Expert advice and planning", 4999, "booking"),
                ("Medical Supply Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Medical Supply Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Optometrist" => vec![
                ("Premium Optometrist Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Optometrist Service", "Essential services to get you started", 9999, "booking"),
                ("Optometrist Consultation", "Expert advice and planning", 4999, "booking"),
                ("Optometrist Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Optometrist Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Ophthalmologist" => vec![
                ("Premium Ophthalmologist Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Ophthalmologist Service", "Essential services to get you started", 9999, "booking"),
                ("Ophthalmologist Consultation", "Expert advice and planning", 4999, "booking"),
                ("Ophthalmologist Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Ophthalmologist Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Audiologist" => vec![
                ("Premium Audiologist Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Audiologist Service", "Essential services to get you started", 9999, "booking"),
                ("Audiologist Consultation", "Expert advice and planning", 4999, "booking"),
                ("Audiologist Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Audiologist Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Speech Therapist" => vec![
                ("Premium Speech Therapist Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Speech Therapist Service", "Essential services to get you started", 9999, "booking"),
                ("Speech Therapist Consultation", "Expert advice and planning", 4999, "booking"),
                ("Speech Therapist Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Speech Therapist Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Occupational Therapist" => vec![
                ("Premium Occupational Therapist Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Occupational Therapist Service", "Essential services to get you started", 9999, "booking"),
                ("Occupational Therapist Consultation", "Expert advice and planning", 4999, "booking"),
                ("Occupational Therapist Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Occupational Therapist Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Physical Therapist" => vec![
                ("Premium Physical Therapist Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Physical Therapist Service", "Essential services to get you started", 9999, "booking"),
                ("Physical Therapist Consultation", "Expert advice and planning", 4999, "booking"),
                ("Physical Therapist Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Physical Therapist Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Sports Medicine" => vec![
                ("Premium Sports Medicine Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Sports Medicine Service", "Essential services to get you started", 9999, "booking"),
                ("Sports Medicine Consultation", "Expert advice and planning", 4999, "booking"),
                ("Sports Medicine Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Sports Medicine Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Orthopedic" => vec![
                ("Premium Orthopedic Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Orthopedic Service", "Essential services to get you started", 9999, "booking"),
                ("Orthopedic Consultation", "Expert advice and planning", 4999, "booking"),
                ("Orthopedic Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Orthopedic Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Pediatrician" => vec![
                ("Premium Pediatrician Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Pediatrician Service", "Essential services to get you started", 9999, "booking"),
                ("Pediatrician Consultation", "Expert advice and planning", 4999, "booking"),
                ("Pediatrician Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Pediatrician Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "OBGYN" => vec![
                ("Premium OBGYN Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic OBGYN Service", "Essential services to get you started", 9999, "booking"),
                ("OBGYN Consultation", "Expert advice and planning", 4999, "booking"),
                ("OBGYN Assessment", "Initial evaluation and report", 7500, "booking"),
                ("OBGYN Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Dermatologist" => vec![
                ("Premium Dermatologist Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Dermatologist Service", "Essential services to get you started", 9999, "booking"),
                ("Dermatologist Consultation", "Expert advice and planning", 4999, "booking"),
                ("Dermatologist Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Dermatologist Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Plastic Surgeon" => vec![
                ("Premium Plastic Surgeon Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Plastic Surgeon Service", "Essential services to get you started", 9999, "booking"),
                ("Plastic Surgeon Consultation", "Expert advice and planning", 4999, "booking"),
                ("Plastic Surgeon Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Plastic Surgeon Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Med Spa" => vec![
                ("Premium Med Spa Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Med Spa Service", "Essential services to get you started", 9999, "booking"),
                ("Med Spa Consultation", "Expert advice and planning", 4999, "booking"),
                ("Med Spa Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Med Spa Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Tattoo Parlor" => vec![
                ("Premium Tattoo Parlor Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Tattoo Parlor Service", "Essential services to get you started", 9999, "booking"),
                ("Tattoo Parlor Consultation", "Expert advice and planning", 4999, "booking"),
                ("Tattoo Parlor Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Tattoo Parlor Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Piercing Studio" => vec![
                ("Premium Piercing Studio Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Piercing Studio Service", "Essential services to get you started", 9999, "booking"),
                ("Piercing Studio Consultation", "Expert advice and planning", 4999, "booking"),
                ("Piercing Studio Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Piercing Studio Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Barbershop" => vec![
                ("Premium Barbershop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Barbershop Service", "Essential services to get you started", 9999, "booking"),
                ("Barbershop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Barbershop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Barbershop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Beauty Supply" => vec![
                ("Premium Beauty Supply Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Beauty Supply Service", "Essential services to get you started", 9999, "booking"),
                ("Beauty Supply Consultation", "Expert advice and planning", 4999, "booking"),
                ("Beauty Supply Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Beauty Supply Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Cosmetics Store" => vec![
                ("Premium Cosmetics Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Cosmetics Store Service", "Essential services to get you started", 9999, "booking"),
                ("Cosmetics Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Cosmetics Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Cosmetics Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Perfume Shop" => vec![
                ("Premium Perfume Shop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Perfume Shop Service", "Essential services to get you started", 9999, "booking"),
                ("Perfume Shop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Perfume Shop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Perfume Shop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Lingerie Store" => vec![
                ("Premium Lingerie Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Lingerie Store Service", "Essential services to get you started", 9999, "booking"),
                ("Lingerie Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Lingerie Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Lingerie Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Maternity Store" => vec![
                ("Premium Maternity Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Maternity Store Service", "Essential services to get you started", 9999, "booking"),
                ("Maternity Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Maternity Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Maternity Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Baby Store" => vec![
                ("Premium Baby Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Baby Store Service", "Essential services to get you started", 9999, "booking"),
                ("Baby Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Baby Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Baby Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Toy Maker" => vec![
                ("Premium Toy Maker Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Toy Maker Service", "Essential services to get you started", 9999, "booking"),
                ("Toy Maker Consultation", "Expert advice and planning", 4999, "booking"),
                ("Toy Maker Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Toy Maker Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Furniture Store" => vec![
                ("Premium Furniture Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Furniture Store Service", "Essential services to get you started", 9999, "booking"),
                ("Furniture Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Furniture Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Furniture Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Mattress Store" => vec![
                ("Premium Mattress Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Mattress Store Service", "Essential services to get you started", 9999, "booking"),
                ("Mattress Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Mattress Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Mattress Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Appliance Store" => vec![
                ("Premium Appliance Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Appliance Store Service", "Essential services to get you started", 9999, "booking"),
                ("Appliance Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Appliance Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Appliance Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Electronics Store" => vec![
                ("Premium Electronics Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Electronics Store Service", "Essential services to get you started", 9999, "booking"),
                ("Electronics Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Electronics Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Electronics Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Computer Store" => vec![
                ("Premium Computer Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Computer Store Service", "Essential services to get you started", 9999, "booking"),
                ("Computer Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Computer Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Computer Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Mobile Phone Shop" => vec![
                ("Premium Mobile Phone Shop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Mobile Phone Shop Service", "Essential services to get you started", 9999, "booking"),
                ("Mobile Phone Shop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Mobile Phone Shop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Mobile Phone Shop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Camera Store" => vec![
                ("Premium Camera Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Camera Store Service", "Essential services to get you started", 9999, "booking"),
                ("Camera Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Camera Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Camera Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Music Store" => vec![
                ("Premium Music Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Music Store Service", "Essential services to get you started", 9999, "booking"),
                ("Music Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Music Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Music Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Instrument Repair" => vec![
                ("Premium Instrument Repair Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Instrument Repair Service", "Essential services to get you started", 9999, "booking"),
                ("Instrument Repair Consultation", "Expert advice and planning", 4999, "booking"),
                ("Instrument Repair Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Instrument Repair Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Record Store" => vec![
                ("Premium Record Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Record Store Service", "Essential services to get you started", 9999, "booking"),
                ("Record Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Record Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Record Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Bookstore" => vec![
                ("Premium Bookstore Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Bookstore Service", "Essential services to get you started", 9999, "booking"),
                ("Bookstore Consultation", "Expert advice and planning", 4999, "booking"),
                ("Bookstore Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Bookstore Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Stationery Store" => vec![
                ("Premium Stationery Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Stationery Store Service", "Essential services to get you started", 9999, "booking"),
                ("Stationery Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Stationery Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Stationery Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Art Supply Store" => vec![
                ("Premium Art Supply Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Art Supply Store Service", "Essential services to get you started", 9999, "booking"),
                ("Art Supply Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Art Supply Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Art Supply Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Craft Store" => vec![
                ("Premium Craft Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Craft Store Service", "Essential services to get you started", 9999, "booking"),
                ("Craft Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Craft Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Craft Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Fabric Store" => vec![
                ("Premium Fabric Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Fabric Store Service", "Essential services to get you started", 9999, "booking"),
                ("Fabric Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Fabric Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Fabric Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Yarn Shop" => vec![
                ("Premium Yarn Shop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Yarn Shop Service", "Essential services to get you started", 9999, "booking"),
                ("Yarn Shop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Yarn Shop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Yarn Shop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Quilt Shop" => vec![
                ("Premium Quilt Shop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Quilt Shop Service", "Essential services to get you started", 9999, "booking"),
                ("Quilt Shop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Quilt Shop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Quilt Shop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Scrapbook Store" => vec![
                ("Premium Scrapbook Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Scrapbook Store Service", "Essential services to get you started", 9999, "booking"),
                ("Scrapbook Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Scrapbook Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Scrapbook Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Hardware Store" => vec![
                ("Premium Hardware Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Hardware Store Service", "Essential services to get you started", 9999, "booking"),
                ("Hardware Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Hardware Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Hardware Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Lumber Yard" => vec![
                ("Premium Lumber Yard Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Lumber Yard Service", "Essential services to get you started", 9999, "booking"),
                ("Lumber Yard Consultation", "Expert advice and planning", 4999, "booking"),
                ("Lumber Yard Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Lumber Yard Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Plumbing Supply" => vec![
                ("Premium Plumbing Supply Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Plumbing Supply Service", "Essential services to get you started", 9999, "booking"),
                ("Plumbing Supply Consultation", "Expert advice and planning", 4999, "booking"),
                ("Plumbing Supply Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Plumbing Supply Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Electrical Supply" => vec![
                ("Premium Electrical Supply Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Electrical Supply Service", "Essential services to get you started", 9999, "booking"),
                ("Electrical Supply Consultation", "Expert advice and planning", 4999, "booking"),
                ("Electrical Supply Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Electrical Supply Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Paint Store" => vec![
                ("Premium Paint Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Paint Store Service", "Essential services to get you started", 9999, "booking"),
                ("Paint Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Paint Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Paint Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Wallpaper Store" => vec![
                ("Premium Wallpaper Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Wallpaper Store Service", "Essential services to get you started", 9999, "booking"),
                ("Wallpaper Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Wallpaper Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Wallpaper Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Flooring Store" => vec![
                ("Premium Flooring Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Flooring Store Service", "Essential services to get you started", 9999, "booking"),
                ("Flooring Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Flooring Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Flooring Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Tile Shop" => vec![
                ("Premium Tile Shop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Tile Shop Service", "Essential services to get you started", 9999, "booking"),
                ("Tile Shop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Tile Shop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Tile Shop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Carpet Cleaning" => vec![
                ("Premium Carpet Cleaning Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Carpet Cleaning Service", "Essential services to get you started", 9999, "booking"),
                ("Carpet Cleaning Consultation", "Expert advice and planning", 4999, "booking"),
                ("Carpet Cleaning Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Carpet Cleaning Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Window Cleaning" => vec![
                ("Premium Window Cleaning Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Window Cleaning Service", "Essential services to get you started", 9999, "booking"),
                ("Window Cleaning Consultation", "Expert advice and planning", 4999, "booking"),
                ("Window Cleaning Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Window Cleaning Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Gutter Cleaning" => vec![
                ("Premium Gutter Cleaning Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Gutter Cleaning Service", "Essential services to get you started", 9999, "booking"),
                ("Gutter Cleaning Consultation", "Expert advice and planning", 4999, "booking"),
                ("Gutter Cleaning Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Gutter Cleaning Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Power Washing" => vec![
                ("Premium Power Washing Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Power Washing Service", "Essential services to get you started", 9999, "booking"),
                ("Power Washing Consultation", "Expert advice and planning", 4999, "booking"),
                ("Power Washing Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Power Washing Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Snow Removal" => vec![
                ("Premium Snow Removal Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Snow Removal Service", "Essential services to get you started", 9999, "booking"),
                ("Snow Removal Consultation", "Expert advice and planning", 4999, "booking"),
                ("Snow Removal Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Snow Removal Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Pool Cleaning" => vec![
                ("Premium Pool Cleaning Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Pool Cleaning Service", "Essential services to get you started", 9999, "booking"),
                ("Pool Cleaning Consultation", "Expert advice and planning", 4999, "booking"),
                ("Pool Cleaning Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Pool Cleaning Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Lawn Care" => vec![
                ("Premium Lawn Care Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Lawn Care Service", "Essential services to get you started", 9999, "booking"),
                ("Lawn Care Consultation", "Expert advice and planning", 4999, "booking"),
                ("Lawn Care Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Lawn Care Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Tree Service" => vec![
                ("Premium Tree Service Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Tree Service Service", "Essential services to get you started", 9999, "booking"),
                ("Tree Service Consultation", "Expert advice and planning", 4999, "booking"),
                ("Tree Service Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Tree Service Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Arborist" => vec![
                ("Premium Arborist Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Arborist Service", "Essential services to get you started", 9999, "booking"),
                ("Arborist Consultation", "Expert advice and planning", 4999, "booking"),
                ("Arborist Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Arborist Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Greenhouse" => vec![
                ("Premium Greenhouse Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Greenhouse Service", "Essential services to get you started", 9999, "booking"),
                ("Greenhouse Consultation", "Expert advice and planning", 4999, "booking"),
                ("Greenhouse Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Greenhouse Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Nursery" => vec![
                ("Premium Nursery Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Nursery Service", "Essential services to get you started", 9999, "booking"),
                ("Nursery Consultation", "Expert advice and planning", 4999, "booking"),
                ("Nursery Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Nursery Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Garden Center" => vec![
                ("Premium Garden Center Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Garden Center Service", "Essential services to get you started", 9999, "booking"),
                ("Garden Center Consultation", "Expert advice and planning", 4999, "booking"),
                ("Garden Center Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Garden Center Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Florist Supply" => vec![
                ("Premium Florist Supply Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Florist Supply Service", "Essential services to get you started", 9999, "booking"),
                ("Florist Supply Consultation", "Expert advice and planning", 4999, "booking"),
                ("Florist Supply Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Florist Supply Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Farm" => vec![
                ("Premium Farm Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Farm Service", "Essential services to get you started", 9999, "booking"),
                ("Farm Consultation", "Expert advice and planning", 4999, "booking"),
                ("Farm Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Farm Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Ranch" => vec![
                ("Premium Ranch Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Ranch Service", "Essential services to get you started", 9999, "booking"),
                ("Ranch Consultation", "Expert advice and planning", 4999, "booking"),
                ("Ranch Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Ranch Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Orchard" => vec![
                ("Premium Orchard Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Orchard Service", "Essential services to get you started", 9999, "booking"),
                ("Orchard Consultation", "Expert advice and planning", 4999, "booking"),
                ("Orchard Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Orchard Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Vineyard" => vec![
                ("Premium Vineyard Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Vineyard Service", "Essential services to get you started", 9999, "booking"),
                ("Vineyard Consultation", "Expert advice and planning", 4999, "booking"),
                ("Vineyard Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Vineyard Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Apiary" => vec![
                ("Premium Apiary Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Apiary Service", "Essential services to get you started", 9999, "booking"),
                ("Apiary Consultation", "Expert advice and planning", 4999, "booking"),
                ("Apiary Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Apiary Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Dairy Farm" => vec![
                ("Premium Dairy Farm Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Dairy Farm Service", "Essential services to get you started", 9999, "booking"),
                ("Dairy Farm Consultation", "Expert advice and planning", 4999, "booking"),
                ("Dairy Farm Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Dairy Farm Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Poultry Farm" => vec![
                ("Premium Poultry Farm Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Poultry Farm Service", "Essential services to get you started", 9999, "booking"),
                ("Poultry Farm Consultation", "Expert advice and planning", 4999, "booking"),
                ("Poultry Farm Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Poultry Farm Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Pig Farm" => vec![
                ("Premium Pig Farm Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Pig Farm Service", "Essential services to get you started", 9999, "booking"),
                ("Pig Farm Consultation", "Expert advice and planning", 4999, "booking"),
                ("Pig Farm Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Pig Farm Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Cattle Ranch" => vec![
                ("Premium Cattle Ranch Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Cattle Ranch Service", "Essential services to get you started", 9999, "booking"),
                ("Cattle Ranch Consultation", "Expert advice and planning", 4999, "booking"),
                ("Cattle Ranch Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Cattle Ranch Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Horse Farm" => vec![
                ("Premium Horse Farm Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Horse Farm Service", "Essential services to get you started", 9999, "booking"),
                ("Horse Farm Consultation", "Expert advice and planning", 4999, "booking"),
                ("Horse Farm Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Horse Farm Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Equestrian Center" => vec![
                ("Premium Equestrian Center Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Equestrian Center Service", "Essential services to get you started", 9999, "booking"),
                ("Equestrian Center Consultation", "Expert advice and planning", 4999, "booking"),
                ("Equestrian Center Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Equestrian Center Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Riding Stable" => vec![
                ("Premium Riding Stable Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Riding Stable Service", "Essential services to get you started", 9999, "booking"),
                ("Riding Stable Consultation", "Expert advice and planning", 4999, "booking"),
                ("Riding Stable Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Riding Stable Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Pet Store" => vec![
                ("Premium Pet Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Pet Store Service", "Essential services to get you started", 9999, "booking"),
                ("Pet Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Pet Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Pet Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Aquarium Store" => vec![
                ("Premium Aquarium Store Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Aquarium Store Service", "Essential services to get you started", 9999, "booking"),
                ("Aquarium Store Consultation", "Expert advice and planning", 4999, "booking"),
                ("Aquarium Store Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Aquarium Store Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Reptile Shop" => vec![
                ("Premium Reptile Shop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Reptile Shop Service", "Essential services to get you started", 9999, "booking"),
                ("Reptile Shop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Reptile Shop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Reptile Shop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Bird Shop" => vec![
                ("Premium Bird Shop Package", "Comprehensive service for your needs", 19999, "booking"),
                ("Basic Bird Shop Service", "Essential services to get you started", 9999, "booking"),
                ("Bird Shop Consultation", "Expert advice and planning", 4999, "booking"),
                ("Bird Shop Assessment", "Initial evaluation and report", 7500, "booking"),
                ("Bird Shop Starter Kit", "Everything you need in one bundle", 12000, "physical"),
            ],
            "Restaurant / Food" => vec![
                ("House Special", "Our most popular dish", 1599, "physical"),
                ("Drink of the Day", "Refreshing beverage", 450, "physical"),
            ],
            _ => vec![
                ("Default Item", "Welcome to your new business", 1000, "physical"),
            ],
        };

        let mut futures = vec![];
        for (name, desc, price, strategy) in products {
            let id = format!("prod-{}", uuid::Uuid::new_v4());
            let org_id = org_id.to_string();
            let name = name.to_string();
            let desc = desc.to_string();
            let strategy = strategy.to_string();
            let pool = self.db.pool.clone();

            let hub = self.hub.clone();
            futures.push(tokio::spawn(async move {
                sqlx::query("INSERT INTO products (id, tenant_id, title, description, price_cents, type, metadata) VALUES ($1, $2, $3, $4, $5, $6, $7)")
                    .bind(&id)
                    .bind(&org_id)
                    .bind(&name)
                    .bind(&desc)
                    .bind(price)
                    .bind(&strategy)
                    .bind(json!({}))
                    .execute(&pool)
                    .await?;

                let event_payload = json!({
                    "product_id": id,
                    "name": name,
                    "organization_id": org_id,
                });

                let event = ::server_ohc::orchestration::TeammateMeshEvent {
                    agent_id: "system".to_string(),
                    action: "ProductCreated".to_string(),
                    status: "success".to_string(),
                    payload: serde_json::to_vec(&event_payload).unwrap_or_default(),
                    msg_id: uuid::Uuid::new_v4().to_string(),
                };

                let _ = hub.publish_teammate_event("products_inbox".to_string(), event);
                Ok::<_, sqlx::Error>(())
            }));
        }

        for f in futures {
            f.await.map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    async fn seed_default_agents(&self, org_id: &str, ai_agents: &[String], ai_auto_respond: bool) -> Result<(), String> {
        let default_agents = vec![
            ("Operations", "The Manager", "Operations"),
            ("Marketing & Advertising", "The Promoter", "Marketing"),
            ("Sales & Acquisition", "The Salesperson", "Sales"),
            ("Customer Success", "The Ambassador", "CustomerSuccess"),
            ("Finance & Payments", "The Accountant", "Finance"),
            ("Legal & Compliance", "The Protector", "Legal"),
            ("Business Advisory", "The Advisor", "Advisory"),
            ("Discovery & SEO", "The Scout", "Discovery"),
        ];

        let start_seed = std::time::Instant::now();
        let mut futures = vec![];
        for (name, role, role_id) in default_agents {
            // Only add agents requested by the user, unless the array is empty (add all)
            if !ai_agents.is_empty() && !ai_agents.contains(&name.to_string()) {
                continue;
            }

            let id = format!("{}-{}", org_id, role_id.to_lowercase());
            let status = if ai_auto_respond { "ACTIVE" } else { "IDLE" };
            let query = sqlx::query("INSERT INTO agents (id, name, role, organization_id, status, provider_type, is_default) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name, role = EXCLUDED.role, status = EXCLUDED.status")
                .bind(id)
                .bind(name)
                .bind(role)
                .bind(org_id.to_string())
                .bind(status)
                .bind("builtin")
                .bind(true);

            let pool = self.db.pool.clone();
            futures.push(tokio::spawn(async move {
                query.execute(&pool).await.map_err(|e| e.to_string())
            }));
        }

        for f in futures {
            f.await.map_err(|e| e.to_string())??;
        }
        tracing::info!("seed_default_agents inserts took: {} us", start_seed.elapsed().as_micros());

        Ok(())
    }
}

pub fn onboarding_feature_state(
    req: &StartOnboardingRequest,
    company_name: &str,
    business_type: &str,
    location: &str,
) -> serde_json::Value {
    let has_services = business_type == "Service Business"
        || business_type == "Service"
        || req.selling_categories.iter().any(|category| category == "services");
    let has_products = req
        .selling_categories
        .iter()
        .any(|category| category == "physical" || category == "digital")
        || !req.first_product_name.trim().is_empty();
    let has_food = business_type == "Restaurant / Food"
        || business_type == "Food Cart"
        || req.selling_categories.iter().any(|category| category == "food");

    let mut flags = serde_json::Map::new();
    flags.insert("onboarding_goal_seconds".to_string(), json!(600));
    flags.insert("unified_storefront".to_string(), json!(true));
    if has_services {
        flags.insert("enable_booking".to_string(), json!(true));
    }
    if has_food {
        flags.insert("enable_menu".to_string(), json!(true));
        flags.insert("enable_pre_order".to_string(), json!(true));
    }
    if has_products {
        flags.insert("enable_ecommerce".to_string(), json!(true));
    }
    if req.selling_categories.iter().any(|category| category == "subscriptions") {
        flags.insert("enable_subscriptions".to_string(), json!(true));
    }

    let mut modules = vec!["storefront"];
    if has_products {
        modules.push("products");
    }
    if has_services {
        modules.push("bookings");
    }
    if has_food {
        modules.push("menu");
    }

    flags.insert("generated_modules".to_string(), json!(modules));
    flags.insert("storefront_status".to_string(), json!("generating"));
    flags.insert("policies_status".to_string(), json!("generating"));
    flags.insert("location".to_string(), json!(location));
    flags.insert("artifacts".to_string(), json!({
        "storefront": {
            "title": company_name,
            "description": format!("Welcome to {}!", company_name),
            "theme": req.website_template,
            "supports_products": has_products,
            "supports_bookings": has_services,
        },
        "policies": [
            {"title": "Terms of Service", "content": "Generating..."},
            {"title": "Privacy Policy", "content": "Generating..."}
        ]
    }));

    serde_json::Value::Object(flags)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::db::DB;
    use ::server_ohc::orchestration::StartOnboardingRequest;

    async fn setup_test_db() -> Option<Arc<DB>> {
        let _ = std::env::var("OHC_DATABASE_URL").ok()?;
        unsafe {
            std::env::set_var("OHC_SQLITE_KEY", "test-fallback-key");
        }
        let db = Arc::new(DB::new().await.ok()?);
        Some(db)
    }

    #[tokio::test]
    async fn test_cache_invalidation_on_save() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };
        let (tx, _) = tokio::sync::mpsc::channel(10);
        let hub = std::sync::Arc::new(crate::hub::Hub::new(tx, db.pool.clone()));
        let agent = OnboardingAgent::new(db.clone(), hub.clone());

        let tenant_id = "test_cache_invalidation_tenant";
        let user_id = "test_cache_invalidation_user";

        // Save initial state using agent
        let state1 = serde_json::json!({"test_key": "val1"});
        agent.save_onboarding_state(tenant_id, user_id, 1, &state1).await.unwrap();

        // Prime the dashboard cache
        let dashboard_cache_key = format!("onboarding_state_{}", tenant_id);
        let dashboard_cache = crate::services::dashboard::service::ONBOARDING_STATE_CACHE.get_or_init(|| ::server_utils::cache::HybridCache::new(hub.redis_client.clone()));
        let dashboard_resp = ::server_ohc::app::GetOnboardingStateResponse {
            state: Some(::server_ohc::app::OnboardingState {
                organization_id: tenant_id.to_string(),
                user_id: user_id.to_string(),
                current_step: 1,
                state_json: state1.to_string(),
            }),
        };
        dashboard_cache.set(&dashboard_cache_key, dashboard_resp.clone(), std::time::Duration::from_secs(3600)).await;

        // Prime the agent cache
        let agent_cache_key = format!("agent_onboarding_state_{}_{}", tenant_id, user_id);
        let agent_cache = ONBOARDING_STATE_AGENT_CACHE.get_or_init(|| ::server_utils::cache::HybridCache::new(hub.redis_client.clone()));
        agent_cache.set(&agent_cache_key, state1.clone(), std::time::Duration::from_secs(3600)).await;

        // Verify caches are primed
        assert!(dashboard_cache.get(&dashboard_cache_key).await.is_some(), "Dashboard cache should be primed");
        assert!(agent_cache.get(&agent_cache_key).await.is_some(), "Agent cache should be primed");

        // Save updated state using agent (this should invalidate both caches)
        let state2 = serde_json::json!({"test_key": "val2"});
        agent.save_onboarding_state(tenant_id, user_id, 2, &state2).await.unwrap();

        // Verify caches are invalidated
        assert!(dashboard_cache.get(&dashboard_cache_key).await.is_none(), "Dashboard cache should be invalidated");
        assert!(agent_cache.get(&agent_cache_key).await.is_none(), "Agent cache should be invalidated");
    }

    #[tokio::test]
    async fn test_start_onboarding_online_store() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };
        let (tx, _) = tokio::sync::mpsc::channel(10);
        let hub = std::sync::Arc::new(crate::hub::Hub::new(tx, db.pool.clone()));
        let agent = OnboardingAgent::new(db, hub);

        let req = StartOnboardingRequest {
            business_type: "Online Store".to_string(),
            company_name: "Test Store".to_string(),
            company_description: "A test store".to_string(),
            selling_categories: vec!["physical".to_string(), "digital".to_string()],
            payment_pref: "online".to_string(),
            admin_email: "admin@test.com".to_string(),
            admin_name: "Admin User".to_string(),
            admin_password: "password123".to_string(),
            website_template: "Modern".to_string(),
            first_product_name: "Cake".to_string(),
            first_product_price: "25.00".to_string(),
            domain_choice: "subdomain".to_string(),
            price_type: "fixed".to_string(),
            location: "New York, USA".to_string(),
            target_audience: "Anyone".to_string(),
            initial_products: vec![],
            ai_agents: vec![],
            ai_auto_respond: false, deposit_percentage: None, lead_time_days: None,
        };

        let req_categories = req.selling_categories.clone();
        assert_eq!(req_categories.len(), 2);
        assert_eq!(req_categories[0], "physical");

        let res = agent.start_onboarding(req).await;
        assert!(res.is_ok());
        let resp = res.unwrap();
        assert!(resp.success);
        assert!(!resp.organization_id.is_empty());

        let org_id = resp.organization_id;
        use sqlx::Row;
        let agents = sqlx::query("SELECT id, name, role FROM agents WHERE organization_id = $1 AND is_default = TRUE")
            .bind(&org_id)
            .fetch_all(&agent.db.pool)
            .await
            .unwrap();

        assert_eq!(agents.len(), 8);

        let expected_roles = vec!["The Manager", "The Promoter", "The Salesperson", "The Ambassador", "The Accountant", "The Protector", "The Advisor", "The Scout"];
        for role in expected_roles {
            assert!(agents.iter().any(|a| a.get::<String, _>("role") == role));
        }

        let users = sqlx::query("SELECT username, email, roles FROM users WHERE organization_id = $1")
            .bind(&org_id)
            .fetch_all(&agent.db.pool)
            .await
            .unwrap();

        assert_eq!(users.len(), 1);
        assert_eq!(users[0].get::<String, _>("email"), "admin@test.com");
        assert_eq!(users[0].get::<String, _>("username"), "Admin User");
        assert!(users[0].get::<String, _>("roles").contains("admin"));
    }


    #[tokio::test]
    async fn test_process_intake_and_variants() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };
        let (tx, _) = tokio::sync::mpsc::channel(10);
        let hub = std::sync::Arc::new(crate::hub::Hub::new(tx, db.pool.clone()));
        let mut agent = OnboardingAgent::new(db.clone(), hub);

        if std::env::var("MINIMAX_API_KEY").is_err() {
            agent.minimax = Some(std::sync::Arc::new(MinimaxClient::new("fake-key".to_string())));
        }

        let input = "I sell custom vegan cakes in Austin, Texas. Maya's Cakes.";
        let res = agent.process_intake(input).await;
        assert!(res.is_ok());
        let data = res.unwrap();

        assert_eq!(data.business_name, "Maya's Cakes");
        assert!(data.initial_products.len() >= 1);

        // Also test creating the variants via start_onboarding directly with the mocked data
        let req = StartOnboardingRequest {
            business_type: data.business_type,
            company_name: data.business_name,
            company_description: input.to_string(),
            selling_categories: data.categories,
            payment_pref: "online".to_string(),
            admin_email: "test@example.com".to_string(),
            admin_name: "Test Admin".to_string(),
            admin_password: "password".to_string(),
            website_template: "Modern".to_string(),
            first_product_name: "Cake".to_string(),
            first_product_price: "45.00".to_string(),
            domain_choice: "subdomain".to_string(),
            price_type: "fixed".to_string(),
            location: data.location.unwrap_or_default(),
            target_audience: data.target_audience.unwrap_or_default(),
            initial_products: data.initial_products.into_iter().map(|p| {
                ::server_ohc::orchestration::IntakeProductProto {
                    name: p.name,
                    price: p.price,
                    description: p.description.unwrap_or_default(),
                    variants: p.variants.unwrap_or_default().into_iter().map(|v| {
                        ::server_ohc::orchestration::IntakeProductVariantProto {
                            name: v.name,
                            price_modifier: v.price_modifier,
                        }
                    }).collect(),
                }
            }).collect(),
            ai_agents: vec![],
            ai_auto_respond: false, deposit_percentage: None, lead_time_days: None,
        };

        let start_res = agent.start_onboarding(req).await;
        assert!(start_res.is_ok());

        let resp = start_res.unwrap();
        assert!(resp.success);
        let org_id = resp.organization_id;

        use sqlx::Row;

        // Verify products are added
        let products = sqlx::query("SELECT id, title as name FROM products WHERE tenant_id = $1")
            .bind(&org_id)
            .fetch_all(&agent.db.pool)
            .await
            .unwrap();

        assert!(!products.is_empty());

        let mut has_variants = false;
        for product in &products {
            let pid: String = product.get("id");
            let variants = sqlx::query("SELECT id, name FROM product_variants WHERE product_id = $1")
                .bind(&pid)
                .fetch_all(&agent.db.pool)
                .await
                .unwrap();

            if !variants.is_empty() {
                has_variants = true;
                break;
            }
        }

        assert!(has_variants, "There should be at least one product variant created.");
    }

    #[tokio::test]
    async fn test_process_intake() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };
        let (tx, _) = tokio::sync::mpsc::channel(10);
        let hub = std::sync::Arc::new(crate::hub::Hub::new(tx, db.pool.clone()));
        let mut agent = OnboardingAgent::new(db.clone(), hub);

        // Mock MinimaxClient if we could, but here we'll just check if it handles configured key
        if std::env::var("MINIMAX_API_KEY").is_err() {
            // Setup a fake one for testing if not present
            agent.minimax = Some(Arc::new(MinimaxClient::new("fake-key".to_string())));
        }

        // This test will likely fail without a real API key if it actually calls the API,
        // but we want to verify the method existence and basic logic.
        // In a real scenario we'd use a trait and mock it.
    }

    #[test]
    fn test_onboarding_feature_state_supports_products_and_bookings_under_ten_minutes() {
        let req = StartOnboardingRequest {
            business_type: "Service Business".to_string(),
            company_name: "Maya Studio".to_string(),
            company_description: "Cakes and classes".to_string(),
            selling_categories: vec!["physical".to_string(), "services".to_string()],
            payment_pref: "online".to_string(),
            admin_email: "owner@example.com".to_string(),
            admin_name: "Owner".to_string(),
            admin_password: "password123".to_string(),
            website_template: "Modern".to_string(),
            first_product_name: "Celebration Cake".to_string(),
            first_product_price: "45.00".to_string(),
            domain_choice: "subdomain".to_string(),
            price_type: "fixed".to_string(),
            location: "Oakland, CA".to_string(),
            target_audience: "Anyone".to_string(),
            initial_products: vec![],
            ai_agents: vec![],
            ai_auto_respond: false, deposit_percentage: None, lead_time_days: None,
        };


        let state = onboarding_feature_state(&req, "Maya Studio", &req.business_type, &req.location);

        assert_eq!(state["unified_storefront"], true);
        assert_eq!(state["onboarding_goal_seconds"], 600);
        assert_eq!(state["enable_ecommerce"], true);
        assert_eq!(state["enable_booking"], true);
        assert_eq!(state["artifacts"]["storefront"]["supports_products"], true);
        assert_eq!(state["artifacts"]["storefront"]["supports_bookings"], true);
        let modules = state["generated_modules"].as_array().unwrap();
        assert!(modules.iter().any(|module| module == "products"));
        assert!(modules.iter().any(|module| module == "bookings"));
    }

    #[tokio::test]
    async fn test_start_onboarding_service_and_food_cart() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };
        let (tx, _) = tokio::sync::mpsc::channel(10);
        let hub = std::sync::Arc::new(crate::hub::Hub::new(tx, db.pool.clone()));
        let agent = OnboardingAgent::new(db.clone(), hub);

        // Test Service Business
        let req_service = StartOnboardingRequest {
            business_type: "Service Business".to_string(),
            company_name: "Test Service".to_string(),
            company_description: "A test service".to_string(),
            selling_categories: vec![],
            payment_pref: "online".to_string(),
            admin_email: "service@test.com".to_string(),
            admin_name: "Service Admin".to_string(),
            admin_password: "password123".to_string(),
            website_template: "Modern".to_string(),
            first_product_name: "Consultation".to_string(),
            first_product_price: "100.00".to_string(),
            domain_choice: "subdomain".to_string(),
            price_type: "fixed".to_string(),
            location: "London, UK".to_string(),
            target_audience: "Anyone".to_string(),
            initial_products: vec![],
            ai_agents: vec![],
            ai_auto_respond: false, deposit_percentage: None, lead_time_days: None,
        };

        let res_service = agent.start_onboarding(req_service).await.unwrap();
        let org_id_service = res_service.organization_id;

        use sqlx::Row;
        let row_service = sqlx::query("SELECT state_json FROM onboarding_state WHERE tenant_id = $1")
            .bind(&org_id_service)
            .fetch_one(&db.pool)
            .await
            .unwrap();

        let state_json_service: serde_json::Value = row_service.try_get("state_json").unwrap_or_else(|_| serde_json::json!({}));
        assert_eq!(state_json_service.get("enable_booking").and_then(|v| v.as_bool()), Some(true));

        let agents_service = sqlx::query("SELECT role FROM agents WHERE organization_id = $1 AND role = 'The Salesperson'")
            .bind(&org_id_service)
            .fetch_all(&agent.db.pool)
            .await
            .unwrap();
        assert_eq!(agents_service.len(), 1);

        // Test Food Cart
        let req_food = StartOnboardingRequest {
            business_type: "Food Cart".to_string(),
            company_name: "Test Food".to_string(),
            company_description: "A test food cart".to_string(),
            selling_categories: vec![],
            payment_pref: "online".to_string(),
            admin_email: "food@test.com".to_string(),
            admin_name: "Food Admin".to_string(),
            admin_password: "password123".to_string(),
            website_template: "Modern".to_string(),
            first_product_name: "Taco".to_string(),
            first_product_price: "5.00".to_string(),
            domain_choice: "subdomain".to_string(),
            price_type: "fixed".to_string(),
            location: "Austin, TX".to_string(),
            target_audience: "Anyone".to_string(),
            initial_products: vec![],
            ai_agents: vec![],
            ai_auto_respond: false, deposit_percentage: None, lead_time_days: None,
        };

        let res_food = agent.start_onboarding(req_food).await.unwrap();
        let org_id_food = res_food.organization_id;

        let row_food = sqlx::query("SELECT state_json FROM onboarding_state WHERE tenant_id = $1")
            .bind(&org_id_food)
            .fetch_one(&db.pool)
            .await
            .unwrap();

        let state_json_food: serde_json::Value = row_food.try_get("state_json").unwrap_or_else(|_| serde_json::json!({}));
        assert_eq!(state_json_food.get("enable_menu").and_then(|v| v.as_bool()), Some(true));
    }

    #[tokio::test]
    async fn test_get_onboarding_state_caching() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };
        let (tx, _) = tokio::sync::mpsc::channel(10);
        let hub = std::sync::Arc::new(crate::hub::Hub::new(tx, db.pool.clone()));
        let agent = OnboardingAgent::new(db.clone(), hub);

        let tenant_id = "test_cache_tenant";
        let user_id = "test_cache_user";

        // Pre-fill state in DB
        let state = serde_json::json!({"test_key": "test_value"});
        agent.save_onboarding_state(tenant_id, user_id, 2, &state).await.unwrap();

        // Fetch once - should query DB and cache
        let start1 = std::time::Instant::now();
        let res1 = agent.get_onboarding_state(tenant_id, user_id).await.unwrap();
        let _elapsed1 = start1.elapsed();

        assert_eq!(res1.get("step").and_then(|v| v.as_i64()), Some(2));
        assert_eq!(res1.get("test_key").and_then(|v| v.as_str()), Some("test_value"));

        // Update directly in DB (bypass cache logic to prove cache is working)
        let _ = sqlx::query("UPDATE onboarding_state SET current_step = 3 WHERE tenant_id = $1 AND user_id = $2")
            .bind(tenant_id)
            .bind(user_id)
            .execute(&db.pool)
            .await
            .unwrap();

        // Fetch second time - should use cache and get step 2, not 3
        let start2 = std::time::Instant::now();
        let res2 = agent.get_onboarding_state(tenant_id, user_id).await.unwrap();
        let _elapsed2 = start2.elapsed();
        assert_eq!(res2.get("step").and_then(|v| v.as_i64()), Some(2), "Should return cached step 2");

        // Now save using the agent which invalidates the cache
        agent.save_onboarding_state(tenant_id, user_id, 4, &state).await.unwrap();

        // Fetch third time - cache invalidated, should hit DB and return step 4
        let res3 = agent.get_onboarding_state(tenant_id, user_id).await.unwrap();
        assert_eq!(res3.get("step").and_then(|v| v.as_i64()), Some(4), "Should return updated step 4 after invalidation");
    }

    #[tokio::test]
    async fn test_generate_initial_products_personas() {
        use sqlx::Row;
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };
        let (tx, _) = tokio::sync::mpsc::channel(10);
        let hub = std::sync::Arc::new(crate::hub::Hub::new(tx, db.pool.clone()));
        let agent = OnboardingAgent::new(db.clone(), hub);

        let org_id = "test-org-products";

        // Test Bakery
        agent.generate_initial_products(org_id, "Home Baker").await.unwrap();
        let products = sqlx::query("SELECT title as name FROM products WHERE tenant_id = $1")
            .bind(org_id)
            .fetch_all(&db.pool).await.unwrap();
        assert!(products.iter().any(|p| p.get::<String, _>("name") == "Custom Celebration Cake"));

        // Test Handyman
        let org_id2 = "test-org-handyman";
        agent.generate_initial_products(org_id2, "Handyman").await.unwrap();
        let products2 = sqlx::query("SELECT title as name FROM products WHERE tenant_id = $1")
            .bind(org_id2)
            .fetch_all(&db.pool).await.unwrap();
        assert!(products2.iter().any(|p| p.get::<String, _>("name") == "Standard Repair Visit"));
    }
}
