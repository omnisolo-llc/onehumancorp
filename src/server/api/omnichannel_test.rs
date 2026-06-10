use super::omnichannel::{resolve_identity, OmnichannelWebhookState, omnichannel_webhook_handler};
use crate::db::{DB, DbStore};
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::post,
    Router,
};
use tower::ServiceExt;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::Hub;

#[tokio::test]
async fn test_resolve_identity_empty_or_unknown() {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    let db = Arc::new(DB { store: DbStore::Sqlite(pool.clone()), pool });

    assert_eq!(resolve_identity(&db, "t1", "email", "").await, None);
    assert_eq!(resolve_identity(&db, "t1", "email", "unknown").await, None);
}

#[tokio::test]
async fn test_resolve_identity_email_match() {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    sqlx::query("CREATE TABLE customers (id TEXT, tenant_id TEXT, email TEXT, phone TEXT, name TEXT)")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO customers (id, tenant_id, email, phone, name) VALUES ('c1', 't1', 'test@example.com', '123', 'John')")
        .execute(&pool)
        .await
        .unwrap();

    let db = Arc::new(DB { store: DbStore::Sqlite(pool.clone()), pool });

    let result = resolve_identity(&db, "t1", "email", "test@example.com").await;
    assert_eq!(result, Some("c1".to_string()));

    let no_match = resolve_identity(&db, "t1", "email", "other@example.com").await;
    assert_eq!(no_match, None);

    let wrong_tenant = resolve_identity(&db, "t2", "email", "test@example.com").await;
    assert_eq!(wrong_tenant, None);
}

#[tokio::test]
async fn test_resolve_identity_whatsapp_match() {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    sqlx::query("CREATE TABLE customers (id TEXT, tenant_id TEXT, email TEXT, phone TEXT, name TEXT)")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO customers (id, tenant_id, email, phone, name) VALUES ('c2', 't1', 'test@example.com', '+1234567890', 'Jane')")
        .execute(&pool)
        .await
        .unwrap();

    let db = Arc::new(DB { store: DbStore::Sqlite(pool.clone()), pool });

    let result = resolve_identity(&db, "t1", "whatsapp", "+1234567890").await;
    assert_eq!(result, Some("c2".to_string()));
}

#[tokio::test]
async fn test_resolve_identity_instagram_match() {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    sqlx::query("CREATE TABLE customers (id TEXT, tenant_id TEXT, email TEXT, phone TEXT, name TEXT)")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO customers (id, tenant_id, email, phone, name) VALUES ('c3', 't1', 'test@example.com', '+1234567890', 'jane_doe')")
        .execute(&pool)
        .await
        .unwrap();

    let db = Arc::new(DB { store: DbStore::Sqlite(pool.clone()), pool });

    let result = resolve_identity(&db, "t1", "instagram", "jane_doe").await;
    assert_eq!(result, Some("c3".to_string()));
}

#[tokio::test]
async fn test_resolve_identity_unsupported_source() {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    let db = Arc::new(DB { store: DbStore::Sqlite(pool.clone()), pool });

    let result = resolve_identity(&db, "t1", "sms", "+123").await;
    assert_eq!(result, None);
}

#[tokio::test]
async fn test_omnichannel_webhook_success() {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    let db = Arc::new(DB { store: DbStore::Sqlite(pool.clone()), pool: pool.clone() });

    sqlx::query("CREATE TABLE omni_inbox_messages (id TEXT, tenant_id TEXT, source TEXT, original_content TEXT, translated_content TEXT, source_language TEXT, target_language TEXT, status TEXT, sender_id TEXT, customer_id TEXT, draft_reply TEXT, created_at TEXT, updated_at TEXT)")
        .execute(&pool)
        .await
        .unwrap();

    let redis = Arc::new(crate::orchestration::queue::redis_queue::RedisQueue::new("dummy".to_string()));
    let hub = Arc::new(Hub::new_test(redis).await);
    let orchestrator = Arc::new(DepartmentOrchestrator::new(hub, db.clone()));

    let state = OmnichannelWebhookState {
        db,
        orchestrator,
    };

    let app = Router::new()
        .route("/webhook", post(omnichannel_webhook_handler))
        .with_state(state);

    let payload = serde_json::json!({
        "tenant_id": "test_tenant",
        "source": "whatsapp",
        "sender_id": "+123456789",
        "message": "Hello World"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/webhook")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_omnichannel_webhook_bad_request() {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    let db = Arc::new(DB { store: DbStore::Sqlite(pool.clone()), pool: pool.clone() });

    let redis = Arc::new(crate::orchestration::queue::redis_queue::RedisQueue::new("dummy".to_string()));
    let hub = Arc::new(Hub::new_test(redis).await);
    let orchestrator = Arc::new(DepartmentOrchestrator::new(hub, db.clone()));

    let state = OmnichannelWebhookState {
        db,
        orchestrator,
    };

    let app = Router::new()
        .route("/webhook", post(omnichannel_webhook_handler))
        .with_state(state);

    let payload = serde_json::json!({
        "tenant_id": "test_tenant",
        "source": "whatsapp",
        "sender_id": "", // Empty sender_id should fail
        "message": "Hello"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/webhook")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
