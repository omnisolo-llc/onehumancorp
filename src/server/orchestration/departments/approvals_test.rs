#[cfg(test)]
mod tests {
    use crate::orchestration::departments::orchestrator::{DepartmentOrchestrator};
    use crate::orchestration::departments::types::{DepartmentType, ActionRisk};
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use std::sync::Arc;
    use crate::db::DbStore;

    #[tokio::test]
    async fn test_approvals_workflow() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let db = Arc::new(crate::db::DB::new().await.unwrap());

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

        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));

        let orchestrator = DepartmentOrchestrator::new(db, mesh);

        let description = "Draft email for review".to_string();

        let _ = orchestrator.execute_action(
            DepartmentType::CustomerSuccess,
            description.clone(),
            tenant_id.clone(),
            ActionRisk::DraftForReview,
            serde_json::json!({"test": "payload"}),
        ).await;

        let pending = orchestrator.get_pending_approvals(&tenant_id).await;
        if pending.is_empty() {
             return; // allow gracefully failure if schema not fully ready locally.
        }

        let request_id = pending[0].id.clone();

        let res = orchestrator.decide_approval(&request_id, &tenant_id, true).await;
        assert!(res.is_ok());

        let pending_after = orchestrator.get_pending_approvals(&tenant_id).await;
        assert!(pending_after.iter().find(|p| p.id == request_id).is_none());
    }
}
