use super::*;
use crate::db::{DB, DbStore};
use crate::orchestration::mesh::CentrifugeNode;
use axum::{
    body::{Body, to_bytes},
    http::Request,
};
use omnisolo_builtin_agent::mesh::transport::InProcessTransport;
use serde_json::{Value, json};
use tower::ServiceExt;

async fn fixture() -> (Router, sqlx::SqlitePool, Arc<DepartmentOrchestrator>) {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::raw_sql("CREATE TABLE tenants(id TEXT PRIMARY KEY,plan_tier TEXT); INSERT INTO tenants VALUES('tenant-a','free');
      CREATE TABLE tenant_ai_budgets(tenant_id TEXT,year_month TEXT,actions_used INTEGER,updated_at TEXT,PRIMARY KEY(tenant_id,year_month));
      CREATE TABLE agent_feed_items(id TEXT PRIMARY KEY,tenant_id TEXT,event_source TEXT,context_payload TEXT,proposed_action TEXT,lifecycle_state TEXT,created_at TEXT,updated_at TEXT);
      CREATE TABLE agent_action_requests(id TEXT PRIMARY KEY,tenant_id TEXT,action_type TEXT,status TEXT,department_type TEXT,description TEXT,payload TEXT,created_at TEXT,updated_at TEXT);")
        .execute(&pool)
        .await
        .unwrap();
    let db = Arc::new(DB {
        pool: sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://unused:unused@127.0.0.1:1/unused")
            .unwrap(),
        store: DbStore::Sqlite(pool.clone()),
    });
    let orchestrator = Arc::new(DepartmentOrchestrator::new(
        db,
        Arc::new(CentrifugeNode::new(Arc::new(InProcessTransport::new()))),
    ));
    (
        Router::new().nest(
            "/api/v1/agents/chat",
            router(orchestrator.clone(), Arc::new(SemanticRouter::new())),
        ),
        pool,
        orchestrator,
    )
}

fn request(message: &str, tenant: Option<&str>, expected: Option<&str>) -> Request<Body> {
    let claims = Claims {
        sub: "owner-a".into(),
        exp: i64::MAX,
        iat: 0,
        organization_id: tenant.map(str::to_owned),
        username: String::new(),
        email: String::new(),
        roles: vec!["OWNER".into()],
        session_id: None,
        jti: "test-session".into(),
    };
    let mut request = Request::builder()
        .method("POST")
        .uri("/api/v1/agents/chat")
        .header("content-type", "application/json")
        .extension(claims);
    if let Some(expected) = expected {
        request = request
            .header("x-ohc-expected-user", "owner-a")
            .header("x-ohc-expected-tenant", expected);
    }
    request
        .body(Body::from(json!({"message":message}).to_string()))
        .unwrap()
}

#[tokio::test]
async fn chat_returns_the_persisted_tenant_scoped_approval() {
    let (app, pool, orchestrator) = fixture().await;
    let response = app
        .oneshot(request(
            "Draft a sales quote",
            Some("tenant-a"),
            Some("tenant-a"),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 65536).await.unwrap()).unwrap();
    let rows = orchestrator
        .get_pending_approvals("tenant-a", None, 100)
        .await;
    assert_eq!(rows.len(), 1);
    assert_eq!(body["approval"], serde_json::to_value(&rows[0]).unwrap());
    assert_eq!(
        body["approval"]["payload"]["original_request"],
        "Draft a sales quote"
    );
    assert_eq!(body["approval"]["status"], "PendingApproval");
    assert_eq!(body["approval"]["tenant_id"], "tenant-a");
    assert!(
        orchestrator
            .get_pending_approvals("tenant-b", None, 100)
            .await
            .is_empty()
    );
    pool.close().await;
}

#[tokio::test]
async fn chat_rejects_empty_input_missing_tenant_and_changed_owner() {
    let (app, pool, _) = fixture().await;
    for (message, tenant, expected, status) in [
        (
            "   ",
            Some("tenant-a"),
            Some("tenant-a"),
            StatusCode::BAD_REQUEST,
        ),
        ("Request", None, None, StatusCode::UNAUTHORIZED),
        (
            "Request",
            Some("tenant-a"),
            Some("tenant-b"),
            StatusCode::CONFLICT,
        ),
    ] {
        assert_eq!(
            app.clone()
                .oneshot(request(message, tenant, expected))
                .await
                .unwrap()
                .status(),
            status
        );
    }
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM agent_feed_items")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
}

#[tokio::test]
async fn failed_storage_never_returns_an_approval_identity() {
    let (app, pool, _) = fixture().await;
    pool.close().await;
    let response = app
        .oneshot(request("Request", Some("tenant-a"), None))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let body: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 65536).await.unwrap()).unwrap();
    assert_eq!(body["success"], false);
    assert!(body.get("approval").is_none_or(Value::is_null));
}
