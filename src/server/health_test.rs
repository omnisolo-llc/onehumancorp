use axum::{body::Body, http::Request, routing::get, Router};
use std::sync::Arc;
use tower::ServiceExt;

use crate::hub::Hub;
use crate::api::health::health_handler;

#[tokio::test]
async fn test_health_handler_success() {
    let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
    if !db_url.starts_with("sqlite") && std::env::var("OHC_DATABASE_URL").is_err() {
        return;
    }

    let _pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect_lazy("sqlite::memory:")
        .unwrap();

    let pg_pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy("postgres://dummy")
        .unwrap();

    let (tx, _) = tokio::sync::mpsc::channel(100);
    let hub = Arc::new(Hub::new(tx, pg_pool));

    let app = Router::new()
        .route("/health", get(health_handler))
        .with_state(hub);

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(body.get("mode").is_some());
    assert!(body.get("status").is_some());
    assert_eq!(body.get("status").unwrap(), "degraded"); // Since db is dummy
    assert!(body.get("db_ping").is_some());
    assert!(body.get("hybrid_mode_ready").is_some());
    assert!(body.get("mesh_active").is_some());
}

#[tokio::test]
async fn test_setup_health_check_endpoint() {
    use crate::services::onboarding::onboarding_agent::OnboardingAgent;

    let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
    if !db_url.starts_with("sqlite") && std::env::var("OHC_DATABASE_URL").is_err() {
        return;
    }

    let pg_pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy("postgres://dummy")
        .unwrap();

    let (tx, _) = tokio::sync::mpsc::channel(100);
    let hub = Arc::new(Hub::new(tx, pg_pool.clone()));

    // Ensure clean state
    if std::path::Path::new(".ohc-local-data").exists() {
        std::fs::remove_dir_all(".ohc-local-data").unwrap();
    }
    if std::path::Path::new(".ohc-cloud-data").exists() {
        std::fs::remove_dir_all(".ohc-cloud-data").unwrap();
    }

    // Set up standalone
    crate::services::onboarding::provisioner::provision_environment(false).unwrap();

    let db = crate::db::DB {
        pool: pg_pool.clone(),
        store: crate::db::DbStore::Postgres,
    };
    let agent = Arc::new(OnboardingAgent::new(Arc::new(db), hub));
    let auth_store = Arc::new(crate::auth::Store::new());
    let now = chrono::Utc::now();
    let token = auth_store
        .issue_token(&crate::auth::User {
            id: "health-user".to_string(),
            username: "health-user".to_string(),
            email: "health@example.com".to_string(),
            password_hash: String::new(),
            roles: vec![crate::auth::ROLE_ADMIN.to_string()],
            active: true,
            organization_id: Some("health-tenant".to_string()),
            created_at: now,
            updated_at: now,
            oidc_subject: None,
        })
        .unwrap();

    let transport: Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport> = Arc::new(ohc_builtin_agent::mesh::transport::InProcessTransport::new());

    // We need to provide the MeshTransport state because the router expects it
    let app = crate::api::onboarding::router(agent, auth_store).with_state(transport);

    // Test standalone (should pass since we provisioned it)
    let response = app.clone()
        .oneshot(Request::builder().uri("/setup-health?mode=standalone").header("authorization", format!("Bearer {token}")).body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body.get("status").unwrap(), "ready");

    // Test cloud (should fail since we didn't provision it)
    let response = app.clone()
        .oneshot(Request::builder().uri("/setup-health?mode=cloud").header("authorization", format!("Bearer {token}")).body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK); // Handler returns 200 OK with error JSON
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body.get("status").unwrap(), "error");

    // Clean up
    std::fs::remove_dir_all(".ohc-local-data").unwrap();
}
