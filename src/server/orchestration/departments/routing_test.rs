#[cfg(test)]
mod tests {
    use crate::orchestration::departments::orchestrator::{DepartmentOrchestrator};
    use crate::orchestration::departments::types::DepartmentEvent;
    use crate::orchestration::departments::sales_agent::SalesAgent;
    use crate::orchestration::departments::finance_agent::FinanceAgent;
    use crate::orchestration::departments::legal_agent::LegalAgent;
    use crate::orchestration::departments::business_advisory_agent::BusinessAdvisoryAgent;
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use uuid::Uuid;
    use crate::db::DbStore;

    #[tokio::test]
    async fn test_new_departments_routing() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));

        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));

        let sales_agent = Arc::new(RwLock::new(SalesAgent::new(orchestrator.clone())));
        let finance_agent = Arc::new(RwLock::new(FinanceAgent::new(orchestrator.clone())));
        let legal_agent = Arc::new(RwLock::new(LegalAgent::new(orchestrator.clone())));
        let business_advisory_agent = Arc::new(RwLock::new(BusinessAdvisoryAgent::new(orchestrator.clone())));

        orchestrator.register_department(sales_agent).await;
        orchestrator.register_department(finance_agent).await;
        orchestrator.register_department(legal_agent).await;
        orchestrator.register_department(business_advisory_agent).await;

        let tenant_id = "test-routing-tenant-123".to_string();

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

        let events = vec![
            ("tenant.quote.requested", "Quote generated for review"),
            ("tenant.payment.received", "Record deposit and track payment"),
            ("tenant.compliance.check_needed", "Draft compliance terms and policy update"),
            ("tenant.report.weekly_health", "Draft weekly business health report and next-action suggestions"),
        ];

        for (event_type, expected_desc) in events {
            let event = DepartmentEvent {
                id: Uuid::new_v4().to_string(),
                tenant_id: tenant_id.clone(),
                event_type: event_type.to_string(),
                payload: serde_json::json!({}),
            };

            let res = orchestrator.dispatch_event(event).await;
            assert!(res.is_ok());

            let mut found = false;
            for _ in 0..10 {
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                let pending = orchestrator.get_pending_approvals(&tenant_id, None, 100).await;
                if pending.iter().any(|req| req.description.contains(expected_desc)) {
                    found = true;
                    break;
                }
            }
            assert!(found, "Should generate a pending approval for {}", event_type);
        }
    }
}
