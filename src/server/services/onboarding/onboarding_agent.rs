use std::collections::HashMap;
use serde_json::json;
use crate::ohc::orchestration::{StartOnboardingRequest, StartOnboardingResponse};

pub struct OnboardingAgent {
    db: std::sync::Arc<crate::db::DB>,
    llm: Option<std::sync::Arc<dyn ohc_builtin_agent::llm::LlmClient>>,
}

impl OnboardingAgent {
    pub fn new(db: std::sync::Arc<crate::db::DB>, llm: Option<std::sync::Arc<dyn ohc_builtin_agent::llm::LlmClient>>) -> Self {
        OnboardingAgent { db, llm }
    }

    pub async fn start_onboarding(&self, req: StartOnboardingRequest) -> Result<StartOnboardingResponse, String> {
        let org_id = format!("org-{}", uuid::Uuid::new_v4());

        let (business_type, company_name) = if !req.instant_prompt.is_empty() {
            let mut extracted_b_type = "Online Store".to_string();
            let mut extracted_c_name = "My Instant Business".to_string();

            let prompt = format!(
                "Extract the business type and business name from this description: '{}'.
                Respond EXACTLY in this format: TYPE|NAME
                where TYPE is one of 'Online Store', 'Service Business', 'Restaurant / Food', or 'Other'.
                Example: Online Store|Maya's Bakery",
                req.instant_prompt
            );

            let (mut response_text, mut api_key_missing) = (String::new(), true);

            if let Some(llm) = &self.llm {
                let chat_req = ohc_builtin_agent::types::ChatRequest {
                    messages: vec![ohc_builtin_agent::types::Message {
                        role: ohc_builtin_agent::types::Role::User,
                        content: prompt.clone(),
                        tool_calls: vec![],
                        tool_results: vec![],
                    }],
                    model: "gemini-1.5-pro".to_string(), // Fallback standard model
                    temperature: 0.1,
                    tools: vec![],
                    max_tokens: 1024,
                    system: String::new(),
                };

                if let Ok(res) = llm.chat(chat_req).await {
                    response_text = res.message.content;
                    api_key_missing = false;
                }
            }

            if api_key_missing || response_text.is_empty() {
                // Fallback heuristic for local testing without an API key
                let p = req.instant_prompt.to_lowercase();

                if p.contains("bakery") || p.contains("cake") || p.contains("food") {
                    extracted_b_type = "Restaurant / Food".to_string();
                } else if p.contains("handyman") || p.contains("repair") || p.contains("service") || p.contains("tutor") || p.contains("consult") {
                    extracted_b_type = "Service Business".to_string();
                }

                if p.chars().count() < 30 {
                    extracted_c_name = req.instant_prompt.clone();
                } else if let Some(idx) = p.find("called") {
                    let char_idx = p[..idx].chars().count();
                    let substr = req.instant_prompt.chars().skip(char_idx + 6).collect::<String>();
                    let substr = substr.trim();
                    let parts: Vec<&str> = substr.split(&[' ', '.', ','][..]).collect();
                    if parts.len() > 0 && !parts[0].is_empty() {
                        extracted_c_name = parts[0].to_string();
                        if parts.len() > 1 && !parts[1].is_empty() {
                            extracted_c_name = format!("{} {}", parts[0], parts[1]);
                        }
                    }
                } else if let Some(idx) = p.find("named") {
                    let char_idx = p[..idx].chars().count();
                    let substr = req.instant_prompt.chars().skip(char_idx + 5).collect::<String>();
                    let substr = substr.trim();
                    let parts: Vec<&str> = substr.split(&[' ', '.', ','][..]).collect();
                    if parts.len() > 0 && !parts[0].is_empty() {
                        extracted_c_name = parts[0].to_string();
                        if parts.len() > 1 && !parts[1].is_empty() {
                            extracted_c_name = format!("{} {}", parts[0], parts[1]);
                        }
                    }
                }
            } else {
                let parts: Vec<&str> = response_text.split('|').collect();
                if parts.len() >= 2 {
                    let parsed_type = parts[0].trim();
                    if parsed_type == "Online Store" || parsed_type == "Service Business" || parsed_type == "Restaurant / Food" || parsed_type == "Other" {
                        extracted_b_type = parsed_type.to_string();
                    }
                    let parsed_name = parts[1].trim();
                    if !parsed_name.is_empty() {
                        extracted_c_name = parsed_name.to_string();
                    }
                }
            }

            (extracted_b_type, extracted_c_name)
        } else {
            (req.business_type.clone(), req.company_name.clone())
        };

        if !req.first_product_name.is_empty() {
             let desc = if req.first_product_description.is_empty() {
                 "Added during onboarding".to_string()
             } else {
                 req.first_product_description.clone()
             };
             self.create_product(&org_id, &req.first_product_name, &req.first_product_price, &business_type, &desc).await?;
        } else {
             self.generate_initial_products(&org_id, &business_type).await?;
        }

        self.seed_default_agents(&org_id).await?;

        let user_id = format!("usr-{}", uuid::Uuid::new_v4());
        let email = req.admin_email.clone();
        let username = if req.admin_name.is_empty() { email.clone() } else { req.admin_name.clone() };
        let password = req.admin_password.clone();

        let password_hash = if !password.is_empty() {
            tokio::task::spawn_blocking(move || {
                bcrypt::hash(&password, bcrypt::DEFAULT_COST).map_err(|e| format!("Failed to hash password: {}", e))
            }).await.map_err(|e| e.to_string())??
        } else {
            "".to_string()
        };

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

        Ok(StartOnboardingResponse {
            success: true,
            message: format!("Successfully onboarded {} as a {}!", company_name, business_type),
            organization_id: org_id,
        })
    }

    async fn create_product(&self, org_id: &str, name: &str, price_str: &str, business_type: &str, description: &str) -> Result<(), String> {
        let price_cents = (price_str.parse::<f64>().unwrap_or(0.0) * 100.0) as i64;
        let strategy = match business_type {
            "Service Business" => "booking",
            _ => "physical",
        };

        let id = format!("prod-{}", uuid::Uuid::new_v4());
        sqlx::query("INSERT INTO products (id, organization_id, name, description, price_cents, fulfillment_strategy, metadata) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(id)
            .bind(org_id)
            .bind(name)
            .bind(description)
            .bind(price_cents)
            .bind(strategy)
            .bind(json!({}))
            .execute(&self.db.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn generate_initial_products(&self, org_id: &str, business_type: &str) -> Result<(), String> {
        let products = match business_type {
            "Online Store" => vec![
                ("Standard Product", "A great product for your store", 1999, "physical"),
                ("Premium Product", "A premium offering", 4999, "physical"),
            ],
            "Service Business" => vec![
                ("Consultation", "1-hour professional consultation", 10000, "booking"),
                ("Service Call", "On-site service visit", 7500, "booking"),
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

            futures.push(tokio::spawn(async move {
                sqlx::query("INSERT INTO products (id, organization_id, name, description, price_cents, fulfillment_strategy, metadata) VALUES ($1, $2, $3, $4, $5, $6, $7)")
                    .bind(id)
                    .bind(org_id)
                    .bind(name)
                    .bind(desc)
                    .bind(price)
                    .bind(strategy)
                    .bind(json!({}))
                    .execute(&pool)
                    .await
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
    use crate::ohc::orchestration::StartOnboardingRequest;

    async fn setup_test_db() -> Option<Arc<DB>> {
        let _ = std::env::var("DATABASE_URL").ok()?;
        let db = Arc::new(DB::new().await.ok()?);
        Some(db)
    }


    #[tokio::test]
    async fn test_start_onboarding_instant_build_heuristic_fallback() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };
        let agent = OnboardingAgent::new(db, None);

        let req = StartOnboardingRequest {
            business_type: "".to_string(),
            company_name: "".to_string(),
            company_description: "".to_string(),
            selling_categories: vec![],
            payment_pref: "online".to_string(),
            admin_email: "test@example.com".to_string(),
            admin_name: "Test Admin".to_string(),
            admin_password: "password".to_string(),
            website_template: "Modern".to_string(),
            first_product_name: "".to_string(),
            first_product_price: "0.00".to_string(),
            first_product_description: "".to_string(),
            domain_choice: "subdomain".to_string(),
            instant_prompt: "I run a successful home bakery called Maya's Sweets in Austin.".to_string(),
        };

        let res = agent.start_onboarding(req).await;
        assert!(res.is_ok());
        let resp = res.unwrap();
        assert!(resp.success);

        let org_id = resp.organization_id;
        use sqlx::Row;

        let products = sqlx::query("SELECT name, description, fulfillment_strategy FROM products WHERE organization_id = $1")
            .bind(&org_id)
            .fetch_all(&agent.db.pool)
            .await
            .unwrap();

        // We know it fell back to generating initial products
        assert!(!products.is_empty());
        assert_eq!(products[0].get::<String, _>("fulfillment_strategy"), "physical"); // Restaurant / Food defaults to physical
    }

    #[tokio::test]
    async fn test_start_onboarding_instant_build_heuristic_named() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };
        let agent = OnboardingAgent::new(db, None);

        let req = StartOnboardingRequest {
            business_type: "".to_string(),
            company_name: "".to_string(),
            company_description: "".to_string(),
            selling_categories: vec![],
            payment_pref: "online".to_string(),
            admin_email: "test@example.com".to_string(),
            admin_name: "Test Admin".to_string(),
            admin_password: "password".to_string(),
            website_template: "Modern".to_string(),
            first_product_name: "".to_string(),
            first_product_price: "0.00".to_string(),
            first_product_description: "".to_string(),
            domain_choice: "subdomain".to_string(),
            instant_prompt: "I am a freelance handyman named Bob Builder.".to_string(),
        };

        let res = agent.start_onboarding(req).await;
        assert!(res.is_ok());
        let resp = res.unwrap();
        assert!(resp.success);
    }
#[tokio::test]
    #[ignore]
    async fn test_start_onboarding_online_store() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };
        let agent = OnboardingAgent::new(db, None);

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
            first_product_description: "A delicious test cake".to_string(),
            domain_choice: "subdomain".to_string(),
            instant_prompt: "".to_string(),
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

        assert_eq!(agents.len(), 7);

        let expected_roles = vec!["The Manager", "The Promoter", "The Salesperson", "The Ambassador", "The Accountant", "The Protector", "The Advisor"];
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
}
