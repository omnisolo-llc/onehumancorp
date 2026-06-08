#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::post,
        Router,
    };
    use tower::ServiceExt;
    use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use crate::orchestration::mesh::CentrifugeNode;
    use crate::db::DbStore;

    async fn setup_app() -> Router {
        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));

        let tenant_id = "tenant-123";
        match &db.store {
            DbStore::Postgres => {
                let _ = sqlx::query("INSERT INTO tenants (id, name, tier) VALUES ($1, 'Test Tenant', 'starter') ON CONFLICT (id) DO UPDATE SET tier = 'starter'")
                    .bind(&tenant_id)
                    .execute(&db.pool)
                    .await;
            }
            DbStore::Sqlite(pool) => {
                let _ = sqlx::query("INSERT INTO tenants (tenant_id, business_name, tier) VALUES (?, 'Test Tenant', 'starter') ON CONFLICT (tenant_id) DO UPDATE SET tier = 'starter'")
                    .bind(&tenant_id)
                    .execute(pool)
                    .await;
            }
        }

        Router::new()
            .route("/dispatch", post(crate::api::agents::mission::action_dispatcher::dispatch_action_endpoint))
            .with_state(orchestrator)
            .layer(axum::middleware::from_fn(move |req: axum::extract::Request, next: axum::middleware::Next| async move {
                let mut req = req;
                let claims = ::server_common::Claims {
                    sub: "user-123".to_string(),
                    organization_id: Some("tenant-123".to_string()),
                    roles: vec!["admin".to_string()],
                    exp: 10000000000,
                    iat: 0,
                    username: "test".to_string(),
                    email: "test@example.com".to_string(),
                    session_id: None,
                    jti: "jti-123".to_string(),
                };
                req.extensions_mut().insert(claims);
                next.run(req).await
            }))
    }

    #[tokio::test]
    async fn test_dispatch_draft_customer_message() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let app = setup_app().await;

        let payload = serde_json::json!({
            "action_type": "DraftCustomerMessage",
            "action_description": "Draft a message",
            "risk_level": "HIGH",
            "payload": {
                "customer_id": "cust-1",
                "message_body": "Hello!"
            }
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/dispatch")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert!(body["success"].as_bool().unwrap());
        assert!(body["approval_request_id"].is_string());
    }
}
