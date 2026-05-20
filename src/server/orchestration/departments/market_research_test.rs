#[cfg(test)]
mod market_research_tests {
    use crate::orchestration::departments::orchestrator::{DepartmentOrchestrator};
    use crate::orchestration::departments::types::{DepartmentEvent, DepartmentType, ActionRisk};
    use crate::orchestration::departments::customer_success_agent::CustomerSuccessAgent;
    use crate::orchestration::departments::marketing_agent::MarketingAgent;
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use uuid::Uuid;
    use crate::db::DbStore;

    async fn setup_orchestrator() -> (Arc<DepartmentOrchestrator>, Arc<crate::db::DB>) {
        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));
        (orchestrator, db)
    }

    #[tokio::test]
    async fn test_auto_responder_intent_detection() {
        if std::env::var("DATABASE_URL").is_err() { return; }
        let (orchestrator, db) = setup_orchestrator().await;
        let cs_agent = Arc::new(RwLock::new(CustomerSuccessAgent::new(orchestrator.clone())));
        orchestrator.register_department(cs_agent).await;

        let tenant_id = "test-tenant-cs".to_string();
        let _ = sqlx::query("INSERT INTO tenants (tenant_id, ai_budget) VALUES ($1, 100) ON CONFLICT DO NOTHING").bind(&tenant_id).execute(&db.pool).await;

        // Test Case 1: Order Status Intent
        let event = DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.message.received".to_string(),
            payload: serde_json::json!({"message": "Where is my order?"}),
        };
        orchestrator.dispatch_event(event).await.unwrap();

        let mut success = false;
        for _ in 0..20 {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            // Note: AutoExecute still creates an entry in agent_approvals but with status APPROVED
            // and description formatted with " | Payload: ..."
            // We need to check the DB directly or use a helper that sees all approvals.
            // DepartmentOrchestrator::get_pending_approvals only returns PENDING.
            // Let's check the DB.
            let row: Option<(String,)> = sqlx::query_as("SELECT status FROM agent_approvals WHERE tenant_id = $1 AND description LIKE '%checked your order status%'")
                .bind(&tenant_id)
                .fetch_optional(&db.pool)
                .await
                .unwrap();
            if let Some((status,)) = row {
                if status == "APPROVED" {
                    success = true;
                    break;
                }
            }
        }
        assert!(success, "Order status should be auto-responded and approved");
    }

    #[tokio::test]
    async fn test_social_media_manager_automation() {
        if std::env::var("DATABASE_URL").is_err() { return; }
        let (orchestrator, db) = setup_orchestrator().await;
        let marketing_agent = Arc::new(RwLock::new(MarketingAgent::new(orchestrator.clone())));
        orchestrator.register_department(marketing_agent).await;

        let tenant_id = "test-tenant-marketing".to_string();
        let _ = sqlx::query("INSERT INTO tenants (tenant_id, ai_budget) VALUES ($1, 100) ON CONFLICT DO NOTHING").bind(&tenant_id).execute(&db.pool).await;

        let event = DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.product.created".to_string(),
            payload: serde_json::json!({"id": "prod_123", "name": "Delicious Vegan Cake"}),
        };
        orchestrator.dispatch_event(event).await.unwrap();

        let mut success = false;
        for _ in 0..20 {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            let pending = orchestrator.get_pending_approvals(&tenant_id).await;
            if pending.iter().any(|req| req.description.contains("Draft social media post for new product: Delicious Vegan Cake")) {
                success = true;
                break;
            }
        }
        assert!(success, "Marketing agent should generate a pending approval for the new product");
    }
}
