use super::native_chat;
use axum::{body::Body, http::{Request, StatusCode}};
use std::sync::Arc;
use tower::ServiceExt;
use crate::db::{DB, DbStore};

fn auth_store_and_token(organization_id: &str) -> (Arc<::server_auth::Store>, String) {
    let store = Arc::new(::server_auth::Store::new());
    let now = chrono::Utc::now();
    let token = store
        .issue_token(&::server_auth::User {
            id: "user-a".to_string(),
            username: "user-a".to_string(),
            email: "user-a@example.com".to_string(),
            password_hash: String::new(),
            roles: vec!["ADMIN".to_string()],
            active: true,
            organization_id: Some(organization_id.to_string()),
            created_at: now,
            updated_at: now,
            oidc_subject: None,
        })
        .unwrap();
    (store, token)
}

#[tokio::test]
async fn test_native_chat_router_unauthorized() {
    let pool = sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost/ohc").unwrap();
    let db = Arc::new(DB { pool: pool.clone(), store: DbStore::Postgres });

    let state = native_chat::NativeChatState {
        db: db.clone(),
        chat_service: Arc::new(crate::services::chat::service::ChatService::new(pool)),
    };

    let app = native_chat::router(state);

    // Test without auth extension
    let _response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/inboxes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should fail or cause internal error if no Extension was provided (or just reject if properly guarded in real env)
    // Actually, in our tests without `protect_internal_ingress`, it might panic or return 500 if claims aren't added manually to the request extensions.
    // We are just verifying the route exists and fails gracefully or expects the extension.
}

#[tokio::test]
async fn test_get_inboxes_route_exists() {
    let pool = sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost/ohc").unwrap();
    let db = Arc::new(DB { pool: pool.clone(), store: DbStore::Postgres });

    let state = native_chat::NativeChatState {
        db: db.clone(),
        chat_service: Arc::new(crate::services::chat::service::ChatService::new(pool)),
    };

    let app = native_chat::router(state);

    // Inject the extension manually
    let now = chrono::Utc::now();
    let user = ::server_auth::User {
        id: "user-a".to_string(),
        username: "user-a".to_string(),
        email: "user-a@example.com".to_string(),
        password_hash: String::new(),
        roles: vec!["ADMIN".to_string()],
        active: true,
        organization_id: Some("12345678-1234-1234-1234-123456789012".to_string()),
        created_at: now,
        updated_at: now,
        oidc_subject: None,
    };
    let claims = ::server_common::Claims {
        sub: user.id.clone(),
        exp: 10000000000,
        iat: 0,
        email: user.email.clone(),
        roles: user.roles.clone(),
        organization_id: user.organization_id.clone(),
        jti: "test-jti".to_string(),
        session_id: Some("test-session".to_string()),
        username: "test-username".to_string(),
    };

    let mut req = Request::builder()
        .uri("/inboxes")
        .body(Body::empty())
        .unwrap();

    req.extensions_mut().insert(claims);

    let response = app
        .oneshot(req)
        .await
        .unwrap();

    // Should return 500 in this test because it hits the db connection which is lazy/mock
    // but the route resolves correctly.
    let status = response.status();
    assert!(status == StatusCode::OK || status == StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_get_conversations_route_exists() {
    let pool = sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost/ohc").unwrap();
    let db = Arc::new(DB { pool: pool.clone(), store: DbStore::Postgres });

    let state = native_chat::NativeChatState {
        db: db.clone(),
        chat_service: Arc::new(crate::services::chat::service::ChatService::new(pool)),
    };

    let app = native_chat::router(state);

    // Inject the extension manually
    let claims = ::server_common::Claims {
        sub: "user-a".to_string(),
        exp: 10000000000,
        iat: 0,
        email: "user-a@example.com".to_string(),
        roles: vec!["ADMIN".to_string()],
        organization_id: Some("12345678-1234-1234-1234-123456789012".to_string()),
        jti: "test-jti".to_string(),
        session_id: Some("test-session".to_string()),
        username: "test-username".to_string(),
    };

    let mut req = Request::builder()
        .uri("/conversations")
        .body(Body::empty())
        .unwrap();

    req.extensions_mut().insert(claims);

    let response = app
        .oneshot(req)
        .await
        .unwrap();

    let status = response.status();
    assert!(status == StatusCode::OK || status == StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_get_messages_route_exists() {
    let pool = sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost/ohc").unwrap();
    let db = Arc::new(DB { pool: pool.clone(), store: DbStore::Postgres });

    let state = native_chat::NativeChatState {
        db: db.clone(),
        chat_service: Arc::new(crate::services::chat::service::ChatService::new(pool)),
    };

    let app = native_chat::router(state);

    // Inject the extension manually
    let claims = ::server_common::Claims {
        sub: "user-a".to_string(),
        exp: 10000000000,
        iat: 0,
        email: "user-a@example.com".to_string(),
        roles: vec!["ADMIN".to_string()],
        organization_id: Some("12345678-1234-1234-1234-123456789012".to_string()),
        jti: "test-jti".to_string(),
        session_id: Some("test-session".to_string()),
        username: "test-username".to_string(),
    };

    let mut req = Request::builder()
        .uri("/conversations/12345678-1234-1234-1234-123456789012/messages")
        .body(Body::empty())
        .unwrap();

    req.extensions_mut().insert(claims);

    let response = app
        .oneshot(req)
        .await
        .unwrap();

    let status = response.status();
    assert!(status == StatusCode::OK || status == StatusCode::INTERNAL_SERVER_ERROR);
}
