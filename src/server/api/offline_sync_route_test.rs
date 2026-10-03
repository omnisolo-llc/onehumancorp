use super::*;
use axum::http::HeaderMap;
use omnisolo_builtin_agent::mesh::transport::{InProcessTransport, MeshTransport};
use serde_json::json;

async fn fixture() -> Option<(sqlx::PgPool, HeaderMap)> {
    let url = std::env::var("OHC_SYNC_TEST_DATABASE_URL")
        .or_else(|_| std::env::var("OMNISOLO_DATABASE_URL"))
        .ok()?;
    if !url.contains("test") {
        return None;
    }
    let schema = format!("offline_route_test_{}", uuid::Uuid::new_v4().simple());
    let admin = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&url)
        .await
        .unwrap();
    sqlx::query(&format!("CREATE SCHEMA {schema}"))
        .execute(&admin)
        .await
        .unwrap();
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .after_connect(move |connection, _| {
            let schema = schema.clone();
            Box::pin(async move {
                sqlx::query(&format!("SET search_path TO {schema}"))
                    .execute(connection)
                    .await?;
                Ok(())
            })
        })
        .connect(&url)
        .await
        .unwrap();
    sqlx::raw_sql(include_str!("durable_sync_test_schema.sql"))
        .execute(&pool)
        .await
        .unwrap();
    sqlx::raw_sql(include_str!("../migrations/234_sync_durable_receipts.sql"))
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("CREATE TABLE revoked_tokens(jti TEXT,tenant_id TEXT,expires_at TIMESTAMPTZ,UNIQUE(jti,tenant_id))").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO products(id,tenant_id) VALUES ('p','tenant-a')")
        .execute(&pool)
        .await
        .unwrap();
    let store = crate::auth::Store::with_repo(Arc::new(
        crate::auth::postgres_store::PgUserRepository::new(pool.clone()),
    ));
    let user = crate::auth::User {
        id: "owner".into(),
        username: "owner".into(),
        email: "owner@example.test".into(),
        password_hash: String::new(),
        roles: vec!["owner".into()],
        active: true,
        organization_id: Some("tenant-a".into()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        oidc_subject: None,
    };
    let token = store.issue_token(&user).unwrap();
    let mut headers = HeaderMap::new();
    headers.insert("authorization", format!("Bearer {token}").parse().unwrap());
    Some((pool, headers))
}
async fn body(response: impl IntoResponse) -> serde_json::Value {
    serde_json::from_slice(
        &axum::body::to_bytes(response.into_response().into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap()
}
fn event(id: &str, expected: bool) -> SyncEvent {
    SyncEvent {
        id: id.into(),
        entity_id: "p".into(),
        entity_type: "product".into(),
        action_type: "ToggleSoldOut".into(),
        payload: json!({"is_sold_out":true,"expected_is_sold_out":expected,"expected_updated_at":"2026-01-01T00:00:00Z"}),
        base_version: 1,
    }
}
fn mutation(id: &str, quantity: i32) -> OfflineMutation {
    OfflineMutation {
        transaction_id: id.into(),
        timestamp: None,
        product_id: "p".into(),
        quantity_deducted: quantity,
        amount: Some(100),
        payment_method: None,
        payment_intent_id: None,
        currency: Some("USD".into()),
        mutation_type: None,
        payload: None,
        client_mutation_id: Some(id.into()),
    }
}
fn mesh() -> Arc<dyn MeshTransport> {
    Arc::new(InProcessTransport::new())
}

#[tokio::test]
async fn test_sync_events_success() {
    let Some((pool, headers)) = fixture().await else {
        return;
    };
    let value = body(
        sync_events_handler(
            State(pool.clone()),
            headers,
            Json(SyncEventsRequest {
                events: vec![event("success", false)],
            }),
        )
        .await,
    )
    .await;
    assert_eq!(value["applied_count"], 1);
    assert_eq!(value["outcomes"][0]["status"], "acknowledged");
    let sold: bool = sqlx::query_scalar("SELECT is_sold_out FROM products WHERE id='p'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(sold);
}
#[tokio::test]
async fn test_sync_events_idempotency() {
    let Some((pool, headers)) = fixture().await else {
        return;
    };
    for _ in 0..2 {
        let value = body(
            sync_events_handler(
                State(pool.clone()),
                headers.clone(),
                Json(SyncEventsRequest {
                    events: vec![event("replay", false)],
                }),
            )
            .await,
        )
        .await;
        assert_eq!(value["outcomes"][0]["status"], "acknowledged");
    }
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM sync_events")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1);
}
#[tokio::test]
async fn test_sync_events_conflict() {
    let Some((pool, headers)) = fixture().await else {
        return;
    };
    for _ in 0..2 {
        let value = body(
            sync_events_handler(
                State(pool.clone()),
                headers.clone(),
                Json(SyncEventsRequest {
                    events: vec![event("conflict", true)],
                }),
            )
            .await,
        )
        .await;
        assert_eq!(value["conflict_count"], 1);
        assert_eq!(value["outcomes"][0]["status"], "reconciliation");
    }
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM sync_conflict_queue")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1);
}
#[tokio::test]
async fn test_offline_sync_unauthorized() {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_millis(10))
        .connect_lazy("postgres://localhost/dummy")
        .unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(
        "x-spiffe-id",
        "spiffe://ohc/org/tenant-a/agent/owner".parse().unwrap(),
    );
    let response = offline_sync_handler(
        State((pool, mesh())),
        headers,
        Json(OfflineSyncRequest { mutations: vec![] }),
    )
    .await
    .into_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
#[tokio::test]
async fn test_offline_sync_success_and_negative_guard() {
    let Some((pool, headers)) = fixture().await else {
        return;
    };
    let mut missing = mutation("missing", 2);
    missing.product_id = "nonexistent".into();
    let value = body(
        offline_sync_handler(
            State((pool.clone(), mesh())),
            headers,
            Json(OfflineSyncRequest {
                mutations: vec![mutation("valid", 3), mutation("negative", -1), missing],
            }),
        )
        .await,
    )
    .await;
    assert_eq!(value["applied_count"], 1);
    assert_eq!(value["failed_count"], 2);
    assert_eq!(value["outcomes"][0]["status"], "acknowledged");
    assert_eq!(value["outcomes"][1]["status"], "blocked");
    let stock: i32 = sqlx::query_scalar("SELECT inventory_count FROM products WHERE id='p'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(stock, 7);
}
#[tokio::test]
async fn test_offline_sync_field_service_mutations() {
    let Some((pool, headers)) = fixture().await else {
        return;
    };
    let mut quote = mutation("quote", 0);
    quote.mutation_type = Some("draft_quote".into());
    quote.payload = Some("Prepare plumbing quote".into());
    let mut intent = mutation("intent", 0);
    intent.mutation_type = Some("agent_intent".into());
    intent.payload = Some(json!({"message":"Prepare schedule"}).to_string());
    for _ in 0..2 {
        let value = body(
            offline_sync_handler(
                State((pool.clone(), mesh())),
                headers.clone(),
                Json(OfflineSyncRequest {
                    mutations: vec![quote.clone(), intent.clone()],
                }),
            )
            .await,
        )
        .await;
        assert_eq!(value["applied_count"], 2);
    }
    let tasks: i64 = sqlx::query_scalar("SELECT count(*) FROM department_tasks")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(tasks, 1);
    let jobs: i64 = sqlx::query_scalar("SELECT count(*) FROM ohc_job_queue")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(jobs, 1);
}
