use super::tasks::*;
use axum::{body::Body, http::{Request, StatusCode}, Router};
use tower::ServiceExt;
use std::sync::Arc;
use crate::orchestration::tasks::TaskDecompositionService;
use crate::orchestration::mesh::TeammateMesh;
use crate::db::{DB, DbStore};
use sqlx::sqlite::SqlitePoolOptions;


struct DummyMesh;
#[async_trait::async_trait]
impl TeammateMesh for DummyMesh {
    async fn publish(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn publish_with_ack(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn subscribe(&self, _topic: &str, _handler: Box<dyn Fn(crate::msgbus::Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
    async fn acquire_lock(&self, _resource: &str, _owner: &str, _ttl_seconds: u64) -> Result<bool, String> { Ok(true) }
    async fn release_lock(&self, _resource: &str, _owner: &str) -> Result<(), String> { Ok(()) }
    async fn register_presence(&self, _agent_id: &str, _status: &str, _ttl_seconds: u64) -> Result<(), String> { Ok(()) }
    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> { Ok(vec![]) }
    async fn ping(&self) -> Result<(), String> { Ok(()) }
    async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
    async fn publish_state_handoff(&self, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn subscribe_state_handoff(&self, _handler: Box<dyn Fn(crate::msgbus::Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
}

async fn setup_test_db() -> Arc<DB> {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query(
        "CREATE TABLE shared_tasks_decomposition (id TEXT PRIMARY KEY, status TEXT, dependencies TEXT, assigned_agent_id TEXT, updated_at TEXT, payload TEXT, title TEXT, description TEXT, priority TEXT, locked_until TEXT, ultraplan_phase TEXT, deliberation_log TEXT, depth INTEGER, created_at TEXT, action_risk TEXT, approval_status TEXT, proposed_content TEXT, organization_id TEXT, mission_id TEXT, parent_plan_id TEXT)"
    ).execute(&pool).await.unwrap();

    let db_pg = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap();

    Arc::new(DB {
        pool: db_pg,
        store: DbStore::Sqlite(pool),
    })
}

#[tokio::test]
async fn test_create_task() {
    let db = setup_test_db().await;
    let mesh = Arc::new(DummyMesh);
    let service = Arc::new(TaskDecompositionService::new(db, mesh));

    let app = router(service);

    let payload = serde_json::json!({
        "mission_id": "M-123",
        "title": "Audit Security",
        "description": "Verify tenant isolation in K8s",
        "priority": "P1"
    });

    // We can test simply without authentication middleware, simulating what create_task_handler takes
    // The main router we export does have the middleware, so we test the handler directly or via a mock router.

    // For simplicity, let's create a minimal router without the auth middleware specifically for testing the handler
    let test_app = Router::new()
        .route("/", axum::routing::post(super::tasks::create_task_handler))
        .layer(axum::middleware::from_fn(|mut req: axum::extract::Request, next: axum::middleware::Next| async move {
            let claims = crate::common::Claims {
                sub: "test-user".to_string(),
                role: "admin".to_string(),
                tenant_id: "test-tenant".to_string(),
                exp: 0,
            };
            req.extensions_mut().insert(claims);
            next.run(req).await
        }))
        .with_state(service.clone());

    let response = test_app
        .oneshot(
            Request::builder()
                .method(axum::http::Method::POST)
                .uri("/")
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}
