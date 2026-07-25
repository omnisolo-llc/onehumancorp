use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; // for `oneshot`
use serde_json::json;
use crate::api::inbox::core::{InboxCoreState, router};
use crate::db::DB;
use std::sync::Arc;
use sqlx::PgPool;

async fn setup_db() -> Arc<DB> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
    let pool = PgPool::connect(&database_url).await.unwrap();
    Arc::new(DB { pool, store: crate::db::DbStore::Postgres })
}

#[tokio::test]
async fn test_create_inbox() {
    let db = setup_db().await;
    let state = InboxCoreState { db: db.clone() };
    let app = router(state);

    let payload = json!({
        "name": "Test Inbox",
        "channel_type": "email"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/inbox/test-tenant")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_create_contact() {
    let db = setup_db().await;
    let state = InboxCoreState { db: db.clone() };
    let app = router(state);

    let payload = json!({
        "name": "Test User",
        "identifier": "test@example.com"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/contact/test-tenant")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
}
