#[cfg(test)]
mod tests {
    use crate::orchestration::departments::orchestrator::{DepartmentOrchestrator, Department, BaseAgent};
    use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ActionRisk};
    use crate::orchestration::departments::operations_agent::OperationsAgent;
    use crate::orchestration::departments::marketing_agent::MarketingAgent;
    use crate::orchestration::departments::sales_agent::SalesAgent;
    use crate::orchestration::departments::customer_success_agent::CustomerSuccessAgent;
    use crate::orchestration::departments::finance_agent::FinanceAgent;
    use crate::orchestration::departments::legal_agent::LegalAgent;
    use crate::orchestration::departments::business_advisory_agent::BusinessAdvisoryAgent;
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use std::sync::Arc;
    use crate::db::DbStore;

    async fn setup_orchestrator() -> (Arc<DepartmentOrchestrator>, String) {
        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));

        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));
        let tenant_id = "test-agents-tenant-123".to_string();

        match &db.store {
            DbStore::Postgres => {
                let _ = sqlx::query("INSERT INTO tenants (tenant_id, ai_budget) VALUES ($1, 100) ON CONFLICT (tenant_id) DO UPDATE SET ai_budget = 100")
                    .bind(&tenant_id)
                    .execute(&db.pool)
                    .await;
            }
            DbStore::Sqlite(pool) => {
                let _ = sqlx::query("INSERT INTO tenants (tenant_id, ai_budget) VALUES (?, 100) ON CONFLICT (tenant_id) DO UPDATE SET ai_budget = 100")
                    .bind(&tenant_id)
                    .execute(pool)
                    .await;
            }
        }
        (orchestrator, tenant_id)
    }

    #[tokio::test]
    async fn test_operations_agent() {
        if std::env::var("OHC_DATABASE_URL").is_err() { return; }
        let (orchestrator, tenant_id) = setup_orchestrator().await;
        let mut agent = OperationsAgent::new(orchestrator.clone());
        assert_eq!(agent.department_type(), DepartmentType::Operations);

        // Test Config
        agent.set_config(tenant_id.clone(), DepartmentConfig { tone_of_voice: "formal".to_string(), auto_approve_limits: 100.0 });
        let config = agent.get_config(&tenant_id).unwrap();
        assert_eq!(config.tone_of_voice, "formal");
        assert_eq!(config.auto_approve_limits, 100.0);

        // Test Handle Event
        let event = DepartmentEvent {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.order.created".to_string(),
            payload: serde_json::json!({}),
        };
        assert!(agent.handle_event(&event).await.is_ok());

        // Test memory & base agent
        assert!(agent.query_memory("test").await.is_ok());
        assert_eq!(agent.agent_id(), "operations_agent");
    }

    #[tokio::test]
    async fn test_marketing_agent() {
        if std::env::var("OHC_DATABASE_URL").is_err() { return; }
        let (orchestrator, tenant_id) = setup_orchestrator().await;
        let mut agent = MarketingAgent::new(orchestrator.clone());
        assert_eq!(agent.department_type(), DepartmentType::Marketing);

        agent.set_config(tenant_id.clone(), DepartmentConfig { tone_of_voice: "exciting".to_string(), auto_approve_limits: 0.0 });
        let config = agent.get_config(&tenant_id).unwrap();
        assert_eq!(config.tone_of_voice, "exciting");

        let event = DepartmentEvent {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.product.created".to_string(),
            payload: serde_json::json!({}),
        };
        assert!(agent.handle_event(&event).await.is_ok());

        assert!(agent.query_memory("test").await.is_ok());
        assert_eq!(agent.agent_id(), "marketing_agent");
    }

    #[tokio::test]
    async fn test_sales_agent() {
        if std::env::var("OHC_DATABASE_URL").is_err() { return; }
        let (orchestrator, tenant_id) = setup_orchestrator().await;
        let mut agent = SalesAgent::new(orchestrator.clone());
        assert_eq!(agent.department_type(), DepartmentType::Sales);

        agent.set_config(tenant_id.clone(), DepartmentConfig { tone_of_voice: "friendly".to_string(), auto_approve_limits: 10.0 });
        let config = agent.get_config(&tenant_id).unwrap();
        assert_eq!(config.tone_of_voice, "friendly");

        let event = DepartmentEvent {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.quote.requested".to_string(),
            payload: serde_json::json!({}),
        };
        assert!(agent.handle_event(&event).await.is_ok());

        assert!(agent.query_memory("test").await.is_ok());
        assert_eq!(agent.agent_id(), "sales_agent");
    }

    #[tokio::test]
    async fn test_customer_success_agent() {
        if std::env::var("OHC_DATABASE_URL").is_err() { return; }
        let (orchestrator, tenant_id) = setup_orchestrator().await;
        let mut agent = CustomerSuccessAgent::new(orchestrator.clone());
        assert_eq!(agent.department_type(), DepartmentType::CustomerSuccess);

        agent.set_config(tenant_id.clone(), DepartmentConfig { tone_of_voice: "caring".to_string(), auto_approve_limits: 0.0 });
        let config = agent.get_config(&tenant_id).unwrap();
        assert_eq!(config.tone_of_voice, "caring");

        let event = DepartmentEvent {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.message.received".to_string(),
            payload: serde_json::json!({}),
        };
        assert!(agent.handle_event(&event).await.is_ok());

        assert!(agent.query_memory("test").await.is_ok());
        assert_eq!(agent.agent_id(), "customer_success_agent");
    }

    #[tokio::test]
    async fn test_finance_agent() {
        if std::env::var("OHC_DATABASE_URL").is_err() { return; }
        let (orchestrator, tenant_id) = setup_orchestrator().await;
        let mut agent = FinanceAgent::new(orchestrator.clone());
        assert_eq!(agent.department_type(), DepartmentType::Finance);

        agent.set_config(tenant_id.clone(), DepartmentConfig { tone_of_voice: "formal".to_string(), auto_approve_limits: 0.0 });
        let config = agent.get_config(&tenant_id).unwrap();
        assert_eq!(config.tone_of_voice, "formal");

        let event = DepartmentEvent {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.payment.received".to_string(),
            payload: serde_json::json!({}),
        };
        assert!(agent.handle_event(&event).await.is_ok());

        assert!(agent.query_memory("test").await.is_ok());
        assert_eq!(agent.agent_id(), "finance_agent");
    }

    #[tokio::test]
    async fn test_legal_agent() {
        if std::env::var("OHC_DATABASE_URL").is_err() { return; }
        let (orchestrator, tenant_id) = setup_orchestrator().await;
        let mut agent = LegalAgent::new(orchestrator.clone());
        assert_eq!(agent.department_type(), DepartmentType::Legal);

        agent.set_config(tenant_id.clone(), DepartmentConfig { tone_of_voice: "professional".to_string(), auto_approve_limits: 0.0 });
        let config = agent.get_config(&tenant_id).unwrap();
        assert_eq!(config.tone_of_voice, "professional");

        let event = DepartmentEvent {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.contract.requested".to_string(),
            payload: serde_json::json!({}),
        };
        assert!(agent.handle_event(&event).await.is_ok());

        assert!(agent.query_memory("test").await.is_ok());
        assert_eq!(agent.agent_id(), "legal_agent");
    }

    #[tokio::test]
    async fn test_business_advisory_agent() {
        if std::env::var("OHC_DATABASE_URL").is_err() { return; }
        let (orchestrator, tenant_id) = setup_orchestrator().await;
        let mut agent = BusinessAdvisoryAgent::new(orchestrator.clone());
        assert_eq!(agent.department_type(), DepartmentType::BusinessAdvisory);

        agent.set_config(tenant_id.clone(), DepartmentConfig { tone_of_voice: "supportive".to_string(), auto_approve_limits: 0.0 });
        let config = agent.get_config(&tenant_id).unwrap();
        assert_eq!(config.tone_of_voice, "supportive");

        let event = DepartmentEvent {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "system.weekly_review".to_string(),
            payload: serde_json::json!({}),
        };
        assert!(agent.handle_event(&event).await.is_ok());

        assert!(agent.query_memory("test").await.is_ok());
        assert_eq!(agent.agent_id(), "business_advisory_agent");
    }
}
