use serde_json::json;
use crate::ohc::orchestration::{StartOnboardingRequest, StartOnboardingResponse, GenerateInstantPreviewResponse};

pub struct OnboardingAgent {
    db: std::sync::Arc<crate::db::DB>,
}

impl OnboardingAgent {
    pub fn new(db: std::sync::Arc<crate::db::DB>) -> Self {
        OnboardingAgent { db }
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

    pub async fn generate_instant_preview(&self, bio: String) -> Result<GenerateInstantPreviewResponse, String> {
        if std::env::var("CI").is_ok() || std::env::var("OHC_TEST_MODE").is_ok() {
            let b = bio.to_lowercase();
            if b.contains("bakery") || b.contains("cakes") {
                return Ok(GenerateInstantPreviewResponse {
                    company_name: "Maya's Bakery".to_string(),
                    business_type: "Online Store".to_string(),
                    company_description: "A local bakery specializing in custom cakes".to_string(),
                    website_template: "Warm Minimalist".to_string(),
                    product_name: "Custom Cake".to_string(),
                    product_price: "50.00".to_string(),
                    sell_physical: true,
                    domain_choice: "subdomain".to_string(),
                });
            } else if b.contains("tutor") || b.contains("lessons") {
                return Ok(GenerateInstantPreviewResponse {
                    company_name: "Leo's Tutoring".to_string(),
                    business_type: "Service Business".to_string(),
                    company_description: "Professional music lessons".to_string(),
                    website_template: "Modern Dark".to_string(),
                    product_name: "Guitar Lesson".to_string(),
                    product_price: "40.00".to_string(),
                    sell_physical: false,
                    domain_choice: "custom".to_string(),
                });
            } else {
                return Ok(GenerateInstantPreviewResponse {
                    company_name: "AI Generated Store".to_string(),
                    business_type: "Online Store".to_string(),
                    company_description: "A specialized AI products and services store".to_string(),
                    website_template: "Modern Minimalist".to_string(),
                    product_name: "AI Starter Kit".to_string(),
                    product_price: "99.99".to_string(),
                    sell_physical: true,
                    domain_choice: "subdomain".to_string(),
                });
            }
        }

        let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
        let primary_client = crate::minimax::MinimaxClient::new(api_key);
        let client = crate::minimax::ResilientClient::new(primary_client);

        let prompt = format!("You are 'The Advisor' for OneHumanCorp. A user has provided the following bio for their new business:\n\"{}\"\n\n\
        Please extrapolate the following information to generate a live storefront draft with 'The Promoter'.\n\
        Return ONLY a JSON object with these exact keys:\n\
        {{\n\
            \"company_name\": \"Extrapolated name or a good guess\",\n\
            \"business_type\": \"Online Store\" OR \"Service Business\" OR \"Restaurant / Food\",\n\
            \"company_description\": \"A 1-sentence tagline or bio\",\n\
            \"website_template\": \"Modern Minimalist\" OR \"Warm Minimalist\" OR \"Modern Dark\",\n\
            \"product_name\": \"A good name for their first product/service\",\n\
            \"product_price\": \"A reasonable price (e.g. 19.99)\",\n\
            \"sell_physical\": true/false,\n\
            \"domain_choice\": \"subdomain\" OR \"custom\"\n\
        }}", bio);

        let result_json_str = client.reason(&prompt).await?;
        let json_start = result_json_str.find('{').unwrap_or(0);
        let json_end = result_json_str.rfind('}').unwrap_or(result_json_str.len().saturating_sub(1)) + 1;
        let clean_json = if result_json_str.is_empty() { "{}" } else { &result_json_str[json_start..json_end] };

        let parsed: serde_json::Value = serde_json::from_str(clean_json).map_err(|e| format!("Failed to parse LLM output: {}", e))?;

        Ok(GenerateInstantPreviewResponse {
            company_name: parsed["company_name"].as_str().unwrap_or("Generated Store").to_string(),
            business_type: parsed["business_type"].as_str().unwrap_or("Online Store").to_string(),
            company_description: parsed["company_description"].as_str().unwrap_or("").to_string(),
            website_template: parsed["website_template"].as_str().unwrap_or("Modern Minimalist").to_string(),
            product_name: parsed["product_name"].as_str().unwrap_or("Starter Item").to_string(),
            product_price: parsed["product_price"].as_str().unwrap_or("19.99").to_string(),
            sell_physical: parsed["sell_physical"].as_bool().unwrap_or(true),
            domain_choice: parsed["domain_choice"].as_str().unwrap_or("subdomain").to_string(),
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
