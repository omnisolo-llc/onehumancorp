#[cfg(test)]
mod tests {
    use crate::orchestration::departments::orchestrator::{DepartmentOrchestrator};
    use crate::orchestration::departments::types::{DepartmentEvent};
    use crate::orchestration::departments::customer_success_agent::CustomerSuccessAgent;
    use crate::orchestration::departments::sales_agent::SalesAgent;
    use crate::orchestration::departments::finance_agent::FinanceAgent;
    use crate::orchestration::departments::legal_agent::LegalAgent;
    use crate::orchestration::departments::business_advisory_agent::BusinessAdvisoryAgent;
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::MemoryTransport;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use uuid::Uuid;
    use crate::db::DbStore;

    #[tokio::test]
    async fn test_cross_department_flow() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(MemoryTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));

        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));

        let cs_agent = Arc::new(RwLock::new(CustomerSuccessAgent::new(orchestrator.clone())));
        let sales_agent = Arc::new(RwLock::new(SalesAgent::new(orchestrator.clone())));
        let finance_agent = Arc::new(RwLock::new(FinanceAgent::new(orchestrator.clone())));
        let legal_agent = Arc::new(RwLock::new(LegalAgent::new(orchestrator.clone())));
        let business_advisory_agent = Arc::new(RwLock::new(BusinessAdvisoryAgent::new(orchestrator.clone())));

        orchestrator.register_department(cs_agent).await;
        orchestrator.register_department(sales_agent).await;
        orchestrator.register_department(finance_agent).await;
        orchestrator.register_department(legal_agent).await;
        orchestrator.register_department(business_advisory_agent).await;

        let tenant_id = "test-tenant-123".to_string();

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

        let event = DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.message.received".to_string(),
            payload: serde_json::json!({"message": "Do you make vegan cakes? How much?"}),
        };

        let res = orchestrator.dispatch_event(event).await;
        assert!(res.is_ok());

        // Test Finance Agent
        let finance_event = DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.payment.disputed".to_string(),
            payload: serde_json::json!({"amount": 100.0}),
        };
        let _ = orchestrator.dispatch_event(finance_event).await;

        // Test Legal Agent
        let legal_event = DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.policy.update_required".to_string(),
            payload: serde_json::json!({"policy": "privacy"}),
        };
        let _ = orchestrator.dispatch_event(legal_event).await;

        // Test Business Advisory Agent
        let advisory_event = DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.report.weekly_generated".to_string(),
            payload: serde_json::json!({"revenue": 5000.0}),
        };
        let _ = orchestrator.dispatch_event(advisory_event).await;


        // Poll to allow async event handling to complete instead of sleep
        let mut has_quote = false;
        let mut has_finance = false;
        let mut has_legal = false;
        let mut has_advisory = false;

        for _ in 0..15 {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            let pending = orchestrator.get_pending_approvals(&tenant_id).await;
            if pending.iter().any(|req| req.description.contains("Quote generated for review")) {
                has_quote = true;
            }
            if pending.iter().any(|req| req.description.contains("Draft response for payment dispute or invoice")) {
                has_finance = true;
            }
            if pending.iter().any(|req| req.description.contains("Draft policy update for review")) {
                has_legal = true;
            }
            if pending.iter().any(|req| req.description.contains("Draft weekly business advisory report")) {
                has_advisory = true;
            }

            if has_quote && has_finance && has_legal && has_advisory {
                break;
            }
        }

        assert!(has_quote, "Cross-department flow should result in a pending quote approval");
        assert!(has_finance, "Finance agent should handle event and result in pending approval");
        assert!(has_legal, "Legal agent should handle event and result in pending approval");
        assert!(has_advisory, "Business Advisory agent should handle event and result in pending approval");
    }
}
