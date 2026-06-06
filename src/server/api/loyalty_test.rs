use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use std::sync::Arc;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::mesh::TeammateMesh;
use crate::db::{DB, DbStore};
use sqlx::sqlite::SqlitePoolOptions;
use async_trait::async_trait;

use ohc_builtin_agent::mesh::transport::Message;

struct MockMesh {}
#[async_trait]
impl TeammateMesh for MockMesh {
    async fn publish(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn publish_with_ack(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn subscribe(&self, _topic: &str, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        Ok(Box::new(|| {}))
    }
    async fn acquire_lock(&self, _resource: &str, _owner: &str, _ttl_seconds: u64) -> Result<bool, String> { Ok(true) }
    async fn release_lock(&self, _resource: &str, _owner: &str) -> Result<(), String> { Ok(()) }
    async fn register_presence(&self, _agent_id: &str, _status: &str, _ttl_seconds: u64) -> Result<(), String> { Ok(()) }
    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> { Ok(vec![]) }
    async fn ping(&self) -> Result<(), String> { Ok(()) }
    async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
    async fn publish_state_handoff(&self, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn subscribe_state_handoff(&self, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        Ok(Box::new(|| {}))
    }
}

async fn setup_test_db() -> Arc<DB> {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    let schema = r#"
        CREATE TABLE loyalty_ledger (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            customer_id TEXT NOT NULL,
            points_balance INTEGER DEFAULT 0,
            tier_name TEXT,
            last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE department_tasks (
            id TEXT PRIMARY KEY,
            tenant_id TEXT,
            department TEXT,
            event_type TEXT,
            payload TEXT,
            status TEXT,
            locked_until TIMESTAMP,
            updated_at TIMESTAMP
        );
    "#;
    sqlx::query(schema).execute(&pool).await.unwrap();

    let pg_pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
        .unwrap();

    Arc::new(DB {
        pool: pg_pool,
        store: DbStore::Sqlite(pool),
    })
}

#[tokio::test]
async fn test_get_balance_not_found() {
    let db = setup_test_db().await;
    let transport = Arc::new(MockMesh {});
    let orchestrator = Arc::new(DepartmentOrchestrator::new(db, transport));
    let app = crate::api::loyalty::router(orchestrator);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/tenant1/customer/cust1")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_add_and_get_balance() {
    let db = setup_test_db().await;
    let transport = Arc::new(MockMesh {});
    let orchestrator = Arc::new(DepartmentOrchestrator::new(db, transport));
    let app = crate::api::loyalty::router(orchestrator.clone());

    let req_body = serde_json::json!({ "points": 100 }).to_string();
    let response = app.clone()
        .oneshot(
            Request::builder()
                .uri("/tenant1/customer/cust1/add")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(req_body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let get_response = app
        .oneshot(
            Request::builder()
                .uri("/tenant1/customer/cust1")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(get_response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains(r#""points_balance":100"#));
}

#[tokio::test]
async fn test_redeem_points() {
    let db = setup_test_db().await;
    let transport = Arc::new(MockMesh {});
    let orchestrator = Arc::new(DepartmentOrchestrator::new(db, transport));
    let app = crate::api::loyalty::router(orchestrator.clone());

    let add_body = serde_json::json!({ "points": 200 }).to_string();
    app.clone()
        .oneshot(
            Request::builder()
                .uri("/tenant2/customer/cust2/add")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(add_body))
                .unwrap(),
        )
        .await
        .unwrap();

    let redeem_body = serde_json::json!({ "points": 50 }).to_string();
    let response = app.clone()
        .oneshot(
            Request::builder()
                .uri("/tenant2/customer/cust2/redeem")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(redeem_body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let get_response = app
        .oneshot(
            Request::builder()
                .uri("/tenant2/customer/cust2")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(get_response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains(r#""points_balance":150"#));
}
