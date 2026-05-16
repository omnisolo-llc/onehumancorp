#[cfg(test)]
mod tests {
    use crate::api::agents::approvals::{router, DecisionRequest, ApprovalsResponse};
    use crate::orchestration::departments::orchestrator::{DepartmentOrchestrator, ActionRisk};
    use crate::orchestration::departments::types::DepartmentType;
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::MemoryTransport;
    use std::sync::Arc;
    use axum::http::{StatusCode, Request, header};
    use axum::body::Body;
    use server_common::Claims;
    use crate::db::DbStore;

    // A helper to quickly consume responses since we can't easily rely on tower/http-body-util locally
    async fn read_body(body: axum::body::Body) -> String {
        let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    #[tokio::test]
    async fn test_approvals_api_endpoints() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(MemoryTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));

        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));

        let tenant_id = "test-tenant-api-123".to_string();

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

        // Add a pending approval request via internal method to set up the GET test
        let approval = orchestrator.execute_action(
            DepartmentType::Marketing,
            "Draft social media campaign for trending item".to_string(),
            tenant_id.clone(),
            ActionRisk::DraftForReview,
            serde_json::json!({}),
        ).await;

        assert!(approval.is_ok());

        // Implement Full Integration Test utilizing the router logic directly.
        // We will invoke the handler functions directly since we don't have tower available for routing oneshots.
        use axum::extract::{State, Query, Extension, Path};
        use crate::api::agents::approvals::{list_approvals, decide_approval, PaginationQuery};
        use axum::response::IntoResponse;

        let claims = Claims {
            sub: "test-user".to_string(),
            exp: 9999999999,
            iat: 0,
            organization_id: Some("test-tenant-api-123".to_string()),
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            roles: vec![],
            jti: "test-jti".to_string(),
            session_id: Some("test-session".to_string()),
        };

        let pagination = Query(PaginationQuery { cursor: None, limit: None });
        let ext = Extension(claims.clone());
        let state = State(orchestrator.clone());

        // Test GET Request directly against handler
        let get_response = list_approvals(state.clone(), pagination, ext.clone()).await.into_response();
        assert_eq!(get_response.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(get_response.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        let parsed: ApprovalsResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(parsed.pending_approvals.len(), 1);
        assert_eq!(parsed.pending_approvals[0].description, "Draft social media campaign for trending item | Payload: {}");

        let approval_id = &parsed.pending_approvals[0].id;

        // Test POST Decision directly against handler
        let decision_payload = axum::Json(DecisionRequest { approved: true });
        let path = Path(approval_id.clone());

        let post_res = decide_approval(state, path, ext, decision_payload).await.into_response();
        assert_eq!(post_res.status(), StatusCode::OK);

        // Verify status changed
        let pending_after = orchestrator.get_pending_approvals(&tenant_id).await;
        assert_eq!(pending_after.len(), 0);
    }
}
