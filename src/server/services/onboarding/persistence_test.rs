#[cfg(test)]
mod tests {
    use crate::services::onboarding::onboarding_agent::OnboardingAgent;
    use crate::db::DB;
    use std::sync::Arc;
    use serde_json::json;

    async fn setup_db() -> Arc<DB> {
        let database_url = "sqlite::memory:";
        unsafe { std::env::set_var("DATABASE_URL", database_url); }
        unsafe { std::env::set_var("OHC_SQLITE_KEY", "test-key"); }
        Arc::new(DB::new().await.unwrap())
    }

    #[tokio::test]
    async fn test_state_persistence_roundtrip() {
        let db = setup_db().await;
        db.run_migrations().await.unwrap();
        let (tx, _) = tokio::sync::mpsc::channel(1);
        let hub = Arc::new(crate::hub::Hub::new(tx, db.pool.clone()));
        let agent = OnboardingAgent::new(db, hub);

        let tenant_id = "test-tenant-1";
        let user_id = "test-user-1";
        let state = json!({
            "step": 2,
            "businessName": "Maya Bakes",
            "businessType": "Online Store"
        });

        // Save
        agent.save_state(tenant_id, user_id, 2, state.clone()).await.unwrap();

        // Retrieve
        let retrieved = agent.get_state(tenant_id).await.unwrap().unwrap();
        assert_eq!(retrieved["businessName"], "Maya Bakes");
        assert_eq!(retrieved["step"], 2);
    }

    #[tokio::test]
    async fn test_ai_description_generation_logic() {
        let db = setup_db().await;
        let (tx, _) = tokio::sync::mpsc::channel(1);
        let hub = Arc::new(crate::hub::Hub::new(tx, db.pool.clone()));
        let agent = OnboardingAgent::new(db, hub);

        let desc = agent.generate_ai_description("Super Widget").await.unwrap();
        assert!(desc.contains("Super Widget"));

        let err = agent.generate_ai_description("   ").await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn test_state_merging_behavior() {
        let db = setup_db().await;
        db.run_migrations().await.unwrap();
        let (tx, _) = tokio::sync::mpsc::channel(1);
        let hub = Arc::new(crate::hub::Hub::new(tx, db.pool.clone()));
        let agent = OnboardingAgent::new(db, hub);

        let tenant_id = "merge-tenant";
        let user_id = "merge-user";

        // Step 1: Basic info
        agent.save_state(tenant_id, user_id, 1, json!({"businessName": "Maya"})).await.unwrap();

        // Step 2: More info (Overwrites previous state in our current implementation)
        agent.save_state(tenant_id, user_id, 2, json!({"businessType": "Bakery"})).await.unwrap();

        let retrieved = agent.get_state(tenant_id).await.unwrap().unwrap();
        assert_eq!(retrieved["businessType"], "Bakery");
    }
}
