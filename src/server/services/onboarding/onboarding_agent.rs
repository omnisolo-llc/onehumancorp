use serde_json::json;
use crate::ohc::orchestration::{StartOnboardingRequest, StartOnboardingResponse};

pub struct OnboardingAgent {
    db: std::sync::Arc<crate::db::DB>,
}

use crate::ohc::orchestration::InstantBuildResponse;
use ohc_builtin_agent::types::{ChatRequest, Message};
use ohc_builtin_agent::llm::LlmClient;

impl OnboardingAgent {
    pub fn new(db: std::sync::Arc<crate::db::DB>) -> Self {
        OnboardingAgent { db }
    }

    pub async fn extract_instant_metadata(&self, bio: &str) -> Result<InstantBuildResponse, String> {
        let llm_provider = std::env::var("OHC_LLM_PROVIDER").unwrap_or_else(|_| "gemini".to_string());

        let client: Box<dyn LlmClient> = match llm_provider.as_str() {
            "gemini" => {
                let api_key = std::env::var("GEMINI_API_KEY").map_err(|_| "GEMINI_API_KEY must be set".to_string())?;
                Box::new(ohc_builtin_agent::llm::gemini::GeminiClient::new(api_key))
            },
            "openai" => {
                let api_key = std::env::var("OPENAI_API_KEY").map_err(|_| "OPENAI_API_KEY must be set".to_string())?;
                Box::new(ohc_builtin_agent::llm::openai::OpenAIClient::new(api_key))
            },
            _ => return Err(format!("Unsupported LLM provider: {}", llm_provider)),
        };

        let system_prompt = "You are 'The Advisor', an AI assistant helping small business owners set up their storefronts. \
        Extract the following information from the user's bio paragraph: \
        1. 'company_name': The name of the business. Guess a name if not explicitly stated (e.g., \"Maya's Cakes\" from \"I bake cakes\"). \
        2. 'business_type': The type of business. Choose from 'Online Store', 'Service Business', 'Restaurant / Food', or provide a generic type. \
        3. 'admin_email': An email address for the admin, derived from the bio. If not provided, make up a placeholder like 'admin@<company-name>.com'. \
        4. 'payment_pref': A payment preference, e.g., 'online', 'in-person'. Default to 'online'. \
        Respond ONLY with a valid JSON object containing exactly these four keys. Do not include markdown formatting or extra text.";

        let req = ChatRequest {
            model: std::env::var("OHC_LLM_MODEL").unwrap_or_else(|_| "gemini-1.5-pro-latest".to_string()),
            system: system_prompt.to_string(),
            messages: vec![Message::user(bio)],
            tools: vec![],
            max_tokens: 1024,
            temperature: 0.1,
        };

        let resp = client.chat(req).await.map_err(|e| format!("LLM chat failed: {}", e))?;
        let content = resp.message.content.trim().trim_start_matches("```json").trim_end_matches("```").trim();

        match serde_json::from_str::<serde_json::Value>(content) {
            Ok(json) => {
                Ok(InstantBuildResponse {
                    company_name: json.get("company_name").and_then(|v| v.as_str()).unwrap_or("AI Generated Store").to_string(),
                    business_type: json.get("business_type").and_then(|v| v.as_str()).unwrap_or("Online Store").to_string(),
                    admin_email: json.get("admin_email").and_then(|v| v.as_str()).unwrap_or("admin@ai-generated.test").to_string(),
                    payment_pref: json.get("payment_pref").and_then(|v| v.as_str()).unwrap_or("online").to_string(),
                })
            }
            Err(_) => {
                // Fallback parsing failed
                Ok(InstantBuildResponse {
                    company_name: "AI Generated Store".to_string(),
                    business_type: "Online Store".to_string(),
                    admin_email: "admin@ai-generated.test".to_string(),
                    payment_pref: "online".to_string(),
                })
            }
        }
    }

    pub async fn start_onboarding(&self, req: StartOnboardingRequest) -> Result<StartOnboardingResponse, String> {
        let org_id = format!("org-{}", uuid::Uuid::new_v4());

        let business_type = req.business_type.clone();
        let company_name = req.company_name.clone();

        if !req.first_product_name.is_empty() {
             self.create_product(&org_id, &req.first_product_name, &req.first_product_price, &business_type).await?;
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

    async fn create_product(&self, org_id: &str, name: &str, price_str: &str, business_type: &str) -> Result<(), String> {
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
            .bind("Added during onboarding")
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
    async fn test_start_onboarding_online_store() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };
        let agent = OnboardingAgent::new(db);

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
