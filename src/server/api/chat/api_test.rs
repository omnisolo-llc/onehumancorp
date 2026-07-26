use super::api::*;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::Router,
};
use tower::ServiceExt;
use sqlx::PgPool;
use std::sync::Arc;
use crate::services::chat::service::ChatService;
use uuid::Uuid;

async fn setup_db(pool: &PgPool) {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS chat_inboxes (
            id UUID PRIMARY KEY,
            tenant_id UUID NOT NULL,
            name TEXT NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        );"
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS chat_channels (
            id UUID PRIMARY KEY,
            tenant_id UUID NOT NULL,
            inbox_id UUID NOT NULL REFERENCES chat_inboxes(id) ON DELETE CASCADE,
            channel_type TEXT NOT NULL,
            config JSONB DEFAULT '{}'::jsonb,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        );"
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS chat_contacts (
            id UUID PRIMARY KEY,
            tenant_id UUID NOT NULL,
            name TEXT,
            email TEXT,
            phone TEXT,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        );"
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS chat_conversations (
            id UUID PRIMARY KEY,
            tenant_id UUID NOT NULL,
            inbox_id UUID NOT NULL REFERENCES chat_inboxes(id) ON DELETE CASCADE,
            contact_id UUID NOT NULL REFERENCES chat_contacts(id) ON DELETE CASCADE,
            assignee_id UUID,
            status TEXT NOT NULL DEFAULT 'open',
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        );"
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS chat_messages (
            id UUID PRIMARY KEY,
            tenant_id UUID NOT NULL,
            conversation_id UUID NOT NULL REFERENCES chat_conversations(id) ON DELETE CASCADE,
            sender_type TEXT NOT NULL,
            sender_id UUID,
            content TEXT NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        );"
    )
    .execute(pool)
    .await
    .unwrap();
}

#[tokio::test]
async fn test_full_chat_api_flow() {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
    let pool = match PgPool::connect(&database_url).await {
        Ok(pool) => pool,
        Err(_) => panic!("Failed to connect to DB for tests"),
    };
    setup_db(&pool).await;

    let app = router(pool.clone());
    let tenant_id = Uuid::new_v4();

    // 1. Create Inbox
    let req = Request::builder()
        .method("POST")
        .uri(format!("/api/v1/chat/{}/inboxes", tenant_id))
        .header("content-type", "application/json")
        .body(Body::from(r#"{"name": "Support Inbox"}"#))
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let inbox: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let inbox_id = inbox["id"].as_str().unwrap();

    // 2. Create Channel
    let req = Request::builder()
        .method("POST")
        .uri(format!("/api/v1/chat/{}/inboxes/{}/channels", tenant_id, inbox_id))
        .header("content-type", "application/json")
        .body(Body::from(r#"{"channel_type": "whatsapp", "config": {}}"#))
        .unwrap();
    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 3. Create Contact
    let req = Request::builder()
        .method("POST")
        .uri(format!("/api/v1/chat/{}/contacts", tenant_id))
        .header("content-type", "application/json")
        .body(Body::from(r#"{"name": "John Doe", "email": "john@example.com", "phone": "1234567890"}"#))
        .unwrap();
    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let contact: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let contact_id = contact["id"].as_str().unwrap();

    // 4. Start Conversation
    let req = Request::builder()
        .method("POST")
        .uri(format!("/api/v1/chat/{}/conversations", tenant_id))
        .header("content-type", "application/json")
        .body(Body::from(format!(
            r#"{{"inbox_id": "{}", "contact_id": "{}"}}"#,
            inbox_id, contact_id
        )))
        .unwrap();
    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let conv: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let conv_id = conv["id"].as_str().unwrap();

    // 5. Send Message
    let req = Request::builder()
        .method("POST")
        .uri(format!("/api/v1/chat/{}/conversations/{}/messages", tenant_id, conv_id))
        .header("content-type", "application/json")
        .body(Body::from(r#"{"sender_type": "agent", "content": "Hello, how can I help?"}"#))
        .unwrap();
    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
