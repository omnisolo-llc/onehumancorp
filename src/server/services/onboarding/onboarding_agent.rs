use std::collections::HashMap;
use serde_json::json;
use crate::ohc::orchestration::{StartOnboardingRequest, StartOnboardingResponse};

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

        // 1. Generate initial products based on business type
        self.generate_initial_products(&org_id, &business_type).await?;

        // 2. Persist the first product if provided
        if !req.first_product_name.is_empty() {
            let price_cents = parse_price_cents(&req.first_product_price);
            let strategy = match business_type.as_str() {
                "Service Business" => "booking",
                _ => "physical",
            };

            // AI auto-generates description from name
            let description = format!("A high-quality {} from {}, designed for the modern {}.",
                req.first_product_name, company_name, business_type);

            self.insert_product(&org_id, &req.first_product_name, &description, price_cents, strategy).await?;
        }

        Ok(StartOnboardingResponse {
            success: true,
            message: format!("Successfully onboarded {} as a {}!", company_name, business_type),
            organization_id: org_id,
        })
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

        for (name, desc, price, strategy) in products {
            self.insert_product(org_id, name, desc, price, strategy).await?;
        }

        Ok(())
    }

    async fn insert_product(&self, org_id: &str, name: &str, desc: &str, price: i64, strategy: &str) -> Result<(), String> {
        let id = format!("prod-{}", uuid::Uuid::new_v4());
        sqlx::query("INSERT INTO products (id, organization_id, name, description, price_cents, fulfillment_strategy, metadata) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(id)
            .bind(org_id)
            .bind(name)
            .bind(desc)
            .bind(price)
            .bind(strategy)
            .bind(json!({}))
            .execute(&self.db.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

fn parse_price_cents(price_str: &str) -> i64 {
    if price_str.is_empty() {
        return 0;
    }

    // Remove any currency symbols or commas
    let clean_price = price_str.replace('$', "").replace(',', "");

    if let Some(pos) = clean_price.find('.') {
        let parts: Vec<&str> = clean_price.split('.').collect();
        let dollars = parts[0].parse::<i64>().unwrap_or(0);
        let cents_str = parts[1];
        let cents = if cents_str.len() >= 2 {
            cents_str[..2].parse::<i64>().unwrap_or(0)
        } else if cents_str.len() == 1 {
            cents_str.parse::<i64>().unwrap_or(0) * 10
        } else {
            0
        };
        dollars * 100 + cents
    } else {
        clean_price.parse::<i64>().unwrap_or(0) * 100
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::db::DB;
    use crate::ohc::orchestration::StartOnboardingRequest;

    #[test]
    fn test_parse_price_cents() {
        assert_eq!(parse_price_cents("50.00"), 5000);
        assert_eq!(parse_price_cents("50"), 5000);
        assert_eq!(parse_price_cents("50.5"), 5050);
        assert_eq!(parse_price_cents("50.55"), 5055);
        assert_eq!(parse_price_cents("$50.00"), 5000);
        assert_eq!(parse_price_cents("1,234.56"), 123456);
    }

    async fn setup_test_db() -> Option<Arc<DB>> {
        let _ = std::env::var("DATABASE_URL").ok()?;
        let db = Arc::new(DB::new().await.ok()?);
        Some(db)
    }

    #[tokio::test]
    #[ignore]
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
            selling_categories: vec![],
            payment_pref: "online".to_string(),
            admin_email: "admin@test.com".to_string(),
            website_template: "Modern".to_string(),
            first_product_name: "Cake".to_string(),
            first_product_price: "25.00".to_string(),
            domain_choice: "subdomain".to_string(),
        };

        let res = agent.start_onboarding(req).await;
        assert!(res.is_ok());
        let resp = res.unwrap();
        assert!(resp.success);
        assert!(!resp.organization_id.is_empty());
    }
}
