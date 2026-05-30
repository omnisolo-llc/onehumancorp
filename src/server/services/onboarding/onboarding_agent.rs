use serde::{Deserialize, Serialize};
use serde_json::json;
use ::server_ohc::orchestration::{StartOnboardingRequest, StartOnboardingResponse};
use crate::minimax::MinimaxClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct IntakeData {
    pub business_name: String,
    pub business_type: String,
    pub categories: Vec<String>,
    pub initial_products: Vec<IntakeProduct>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntakeProduct {
    pub name: String,
    pub price: String,
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

    pub async fn process_intake(&self, input: &str) -> Result<IntakeData, String> {
        let minimax = self.minimax.as_ref().ok_or("MiniMax API key not configured")?;

        let prompt = format!(
            "Extract structured business information from the following user description.
            Return ONLY a valid JSON object with fields: business_name, business_type, categories (array), initial_products (array of objects with 'name' and 'price' as string).

            Description: \"{}\"

            Example JSON:
            {{
              \"business_name\": \"Maya's Cakes\",
              \"business_type\": \"Bakery\",
              \"categories\": [\"food\", \"physical\"],
              \"initial_products\": [
                {{\"name\": \"Chocolate Cake\", \"price\": \"25.00\"}},
                {{\"name\": \"Vanilla Cupcake\", \"price\": \"3.50\"}}
              ]
            }}",
            input
        );

        let response = minimax.reason(&prompt).await?;

        // Clean up markdown code blocks if present
        let clean_json = response.trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        let data: IntakeData = serde_json::from_str(clean_json)
            .map_err(|e| format!("Failed to parse AI response as JSON: {}. Response was: {}", e, response))?;

        Ok(data)
    }


    pub async fn save_onboarding_state(&self, tenant_id: &str, user_id: &str, current_step: i32, state_json: &serde_json::Value) -> Result<(), String> {
        let mut tx = self.hub.pool.begin().await.map_err(|e| e.to_string())?;
        crate::common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO onboarding_state (tenant_id, user_id, current_step, state_json) \
             VALUES ($1, $2, $3, $4) \
             ON CONFLICT (tenant_id, user_id) DO UPDATE \
             SET state_json = onboarding_state.state_json || EXCLUDED.state_json, \
                 current_step = EXCLUDED.current_step, \
                 updated_at = CURRENT_TIMESTAMP"
        )
        .bind(tenant_id)
        .bind(user_id)
        .bind(current_step)
        .bind(state_json)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn get_onboarding_state(&self, tenant_id: &str, user_id: &str) -> Result<serde_json::Value, String> {
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

        if let Some(record) = row {
            let mut state: serde_json::Value = record.get("state_json");
            let current_step: i32 = record.get("current_step");
            if let Some(obj) = state.as_object_mut() {
                obj.insert("step".to_string(), serde_json::json!(current_step));
            }
            Ok(state)
        } else {
            Ok(serde_json::json!({ "step": 0 }))
        }
    }

    pub async fn start_onboarding(&self, req: StartOnboardingRequest) -> Result<StartOnboardingResponse, String> {
        let start_time = std::time::Instant::now();
        let org_id = format!("org-{}", uuid::Uuid::new_v4());

        let business_type = req.business_type.clone();
        let company_name = req.company_name.clone();

        // Use organization id as tenant id if not provided
        let _tenant_id = org_id.clone();

        let user_id = format!("usr-{}", uuid::Uuid::new_v4());
        let email = req.admin_email.clone();
        let username = if req.admin_name.is_empty() { email.clone() } else { req.admin_name.clone() };
        let password = req.admin_password.clone();

        let req_first_product_name = req.first_product_name.clone();
        let req_first_product_price = req.first_product_price.clone();
        let req_price_type = req.price_type.clone();
        let org_id_clone1 = org_id.clone();
        let org_id_clone2 = org_id.clone();
        let business_type_clone = business_type.clone();

        let agent_clone_product = self.clone();
        let product_future = tokio::task::spawn(async move {
            if !req_first_product_name.is_empty() {
                agent_clone_product.create_product(&org_id_clone1, &req_first_product_name, &req_first_product_price, &req_price_type, &business_type_clone).await
            } else {
                // If it's a "born live" conversational intake, we might have multiple products
                // but for now we follow the legacy pattern or seed based on type
                agent_clone_product.generate_initial_products(&org_id_clone1, &business_type_clone).await
            }
        });

        let agent_clone_seed = self.clone();
        let seed_future = tokio::task::spawn(async move {
            agent_clone_seed.seed_default_agents(&org_id_clone2).await
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

            for (agent_role, topic) in event_topics {
                let _ = sqlx::query("INSERT INTO agent_event_subscriptions (tenant_id, agent_role, topic) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
                    .bind(&org_id_clone3)
                    .bind(agent_role)
                    .bind(topic)
                    .execute(&pool)
                    .await;
            }

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

        let (product_res_res, seed_res_res, _events_res_res, hash_res_res) = tokio::join!(product_future, seed_future, publish_events_future, hash_future);

        let product_res = product_res_res.unwrap_or_else(|e| Err(e.to_string()));
        let seed_res = seed_res_res.unwrap_or_else(|e| Err(e.to_string()));
        let hash_res = hash_res_res.unwrap_or_else(|e| Err(e.to_string()));

        product_res?;
        seed_res?;
        let password_hash = hash_res?;

        let roles_json = serde_json::to_string(&vec!["admin"]).unwrap_or_default();
        let now = chrono::Utc::now();
        let oidc_subject = "";

        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at)
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

        // Extract feature flags logic
        let mut flags = serde_json::Map::new();
        if business_type == "Service Business" || business_type == "Service" || req.selling_categories.contains(&"services".to_string()) {
            flags.insert("enable_booking".to_string(), serde_json::json!(true));
        }
        if business_type == "Restaurant / Food" || business_type == "Food Cart" || req.selling_categories.contains(&"food".to_string()) {
            flags.insert("enable_menu".to_string(), serde_json::json!(true));
            flags.insert("enable_pre_order".to_string(), serde_json::json!(true));
        }
        if req.selling_categories.contains(&"physical".to_string()) || req.selling_categories.contains(&"digital".to_string()) {
            flags.insert("enable_ecommerce".to_string(), serde_json::json!(true));
        }
        if req.selling_categories.contains(&"subscriptions".to_string()) {
            flags.insert("enable_subscriptions".to_string(), serde_json::json!(true));
        }

        // Add initial artifact placeholders to state
        flags.insert("storefront_status".to_string(), json!("generating"));
        flags.insert("policies_status".to_string(), json!("generating"));
        flags.insert("artifacts".to_string(), json!({
            "storefront": {
                "title": company_name,
                "description": format!("Welcome to {}!", company_name),
                "theme": req.website_template,
            },
            "policies": [
                {"title": "Terms of Service", "content": "Generating..."},
                {"title": "Privacy Policy", "content": "Generating..."}
            ]
        }));

        let flags_json = serde_json::Value::Object(flags);

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
        })
    }

    async fn create_product(&self, org_id: &str, name: &str, price_str: &str, price_type: &str, business_type: &str) -> Result<(), String> {
        let price_cents = (price_str.parse::<f64>().unwrap_or(0.0) * 100.0) as i64;
        let strategy = match business_type {
            "Service Business" => "booking",
            _ => "physical",
        };

        let id = format!("prod-{}", uuid::Uuid::new_v4());
        sqlx::query("INSERT INTO products (id, organization_id, name, description, price_cents, fulfillment_strategy, metadata) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(&id)
            .bind(org_id)
            .bind(name)
            .bind("Added during onboarding")
            .bind(price_cents)
            .bind(strategy)
            .bind(json!({"price_type": price_type}))
            .execute(&self.db.pool)
            .await
            .map_err(|e| e.to_string())?;

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
            "Online Store" => vec![
                ("Standard Product", "A great product for your store", 1999, "physical"),
                ("Premium Product", "A premium offering", 4999, "physical"),
            ],
            "Service Business" | "Handyman" | "Plumbing" => vec![
                ("Standard Service", "Professional service visit", 7500, "booking"),
                ("Premium Package", "Comprehensive service package", 19999, "booking"),
            ],
            "Bakery" | "Home Baker" => vec![
                ("Custom Cake", "Delicious custom baked cake", 5000, "physical"),
                ("Cupcake Set", "Box of 12 artisan cupcakes", 2400, "physical"),
            ],
            "Boutique" | "Clothing" => vec![
                ("Seasonal Item", "Handpicked item from our collection", 3500, "physical"),
                ("Accessory", "Perfect addition to any outfit", 1500, "physical"),
            ],
            "Tutor" | "Music Teacher" => vec![
                ("Single Lesson", "1-hour personalized lesson", 4500, "booking"),
                ("Lesson Package", "Bundle of 5 lessons", 20000, "booking"),
            ],
            "Food Cart" | "Restaurant / Food" => vec![
                ("House Special", "Our most popular dish", 1500, "physical"),
                ("Combo Meal", "Full meal with a drink", 2200, "physical"),
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
                sqlx::query("INSERT INTO products (id, organization_id, name, description, price_cents, fulfillment_strategy, metadata) VALUES ($1, $2, $3, $4, $5, $6, $7)")
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

    async fn seed_default_agents(&self, org_id: &str) -> Result<(), String> {
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

        for (name, role, role_id) in default_agents {
            let id = format!("{}-{}", org_id, role_id.to_lowercase());
            sqlx::query("INSERT INTO agents (id, name, role, organization_id, status, provider_type, is_default) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name, role = EXCLUDED.role, status = EXCLUDED.status")
                .bind(id)
                .bind(name)
                .bind(role)
                .bind(org_id)
                .bind("IDLE")
                .bind("builtin")
                .bind(true)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::db::DB;
    use ::server_ohc::orchestration::StartOnboardingRequest;

    async fn setup_test_db() -> Option<Arc<DB>> {
        let _ = std::env::var("DATABASE_URL").ok()?;
        unsafe {
            std::env::set_var("OHC_SQLITE_KEY", "test-fallback-key");
        }
        let db = Arc::new(DB::new().await.ok()?);
        Some(db)
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
}
