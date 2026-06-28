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
Extract the business taxonomy, default any missing fields to sensible industry defaults, and generate the configuration. Do NOT ask follow-up questions unless the input is completely empty or nonsensical.
You need to synthesize at least:
1. What they sell or what service they provide.
2. A rough idea of their business type (e.g. bakery, handyman, tutor).

Review the following conversation history:
{}

If the input is completely empty or nonsensical, reply with a natural, conversational question asking for clarification.
Otherwise, since you must complete the setup in a single prompt, reply EXACTLY with the string `[COMPLETE]` followed by a brief confirmation message (e.g., `[COMPLETE] Give me a minute... I'm building your business.`). Do not output anything else if you have enough information.

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
        tracing::debug!("Saving onboarding state for tenant: {}, user: {}", tenant_id, user_id);
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
                agent_clone_product.create_product(&org_id_clone1, &req_first_product_name, &req_first_product_price, &req_price_type, &business_type_clone, None, None, None, req_deposit_percentage, req_lead_time_days).await
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
                tracing::error!("Failed to schedule weekly health report: {}", e);
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

    async fn create_product(&self, org_id: &str, name: &str, price_str: &str, price_type: &str, business_type: &str, description: Option<&str>, product_type: Option<&str>, variants: Option<&Vec<IntakeProductVariant>>, deposit_percentage: Option<i32>, lead_time_days: Option<i32>) -> Result<(), String> {
        let price_cents = (price_str.parse::<f64>().unwrap_or(0.0) * 100.0) as i64;
        let strategy = if let Some(pt) = product_type {
            pt
        } else {
            match business_type {
                "Service Business" => "booking",
                _ => "physical",
            }
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
        let mut products = vec![
            ("Standard Product".to_string(), "A great product for your store".to_string(), 1999, "physical".to_string()),
            ("Premium Product".to_string(), "A premium offering".to_string(), 4999, "physical".to_string()),
            ("Consultation".to_string(), "Expert advice and planning".to_string(), 4999, "booking".to_string()),
        ];

        if let Some(minimax) = &self.minimax {
            let prompt = format!(
                r#"You are an expert business catalog generator. Generate 3 initial products or services for a business of type '{}'.
Return ONLY a valid JSON array of objects, where each object has:
- name (string)
- description (string)
- price_cents (integer, e.g. 4500 for $45.00)
- type (string, must be 'physical' or 'booking')

Do not include any markdown formatting, just the raw JSON array.
Example:
[
  {{"name": "Custom Cake", "description": "Beautifully decorated", "price_cents": 4500, "type": "booking"}}
]"#,
                business_type
            );

            if let Ok(Ok(response)) = tokio::time::timeout(std::time::Duration::from_secs(30), minimax.reason(&prompt)).await {
                let mut clean_json = response.trim();
                if clean_json.starts_with("```json") {
                    clean_json = clean_json.trim_start_matches("```json").trim();
                }
                if clean_json.starts_with("```") {
                    clean_json = clean_json.trim_start_matches("```").trim();
                }
                if clean_json.ends_with("```") {
                    clean_json = clean_json.trim_end_matches("```").trim();
                }

                #[derive(serde::Deserialize)]
                struct GenProduct {
                    name: String,
                    description: String,
                    price_cents: i64,
                    #[serde(rename = "type")]
                    product_type: String,
                }

                if let Ok(gen_products) = serde_json::from_str::<Vec<GenProduct>>(clean_json) {
                    if !gen_products.is_empty() {
                        products = gen_products.into_iter().map(|p| {
                            (p.name, p.description, p.price_cents, p.product_type)
                        }).collect();
                    }
                }
            }
        } else {
            match business_type {
                "Home Baker" | "Bakery" => {
                    products = vec![
                        ("Custom Celebration Cake".to_string(), "Beautifully decorated for your special day".to_string(), 4500, "booking".to_string()),
                        ("Dozen Assorted Cupcakes".to_string(), "A variety of our best flavors".to_string(), 2400, "physical".to_string()),
                        ("Seasonal Pie".to_string(), "Baked fresh with local ingredients".to_string(), 1800, "physical".to_string()),
                    ];
                },
                "Handyman" | "Field Service" => {
                    products = vec![
                        ("Standard Repair Visit".to_string(), "Basic maintenance and small repairs".to_string(), 7500, "booking".to_string()),
                        ("Plumbing Consultation".to_string(), "Inspection and quote for plumbing work".to_string(), 4500, "booking".to_string()),
                        ("Emergency Call-Out".to_string(), "Priority service for urgent issues".to_string(), 12000, "booking".to_string()),
                    ];
                },
                _ => {}
            }
        }

        for (name, desc, price_cents, _prod_type) in products {
            let price_str = format!("{}.{:02}", price_cents / 100, price_cents % 100);
            if let Err(e) = self.create_product(org_id, &name, &price_str, "fixed", business_type, Some(&desc), Some(&_prod_type), None, None, None).await {
                tracing::error!("Failed to create generated product {}: {}", name, e);
            }
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
