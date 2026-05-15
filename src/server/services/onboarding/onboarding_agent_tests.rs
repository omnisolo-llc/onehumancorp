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
        let row_service = sqlx::query("SELECT state_json FROM onboarding_state WHERE organization_id = $1")
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

        let row_food = sqlx::query("SELECT state_json FROM onboarding_state WHERE organization_id = $1")
            .bind(&org_id_food)
            .fetch_one(&db.pool)
            .await
            .unwrap();

        let state_json_food: serde_json::Value = row_food.try_get("state_json").unwrap_or_else(|_| serde_json::json!({}));
        assert_eq!(state_json_food.get("enable_menu").and_then(|v| v.as_bool()), Some(true));
    }
