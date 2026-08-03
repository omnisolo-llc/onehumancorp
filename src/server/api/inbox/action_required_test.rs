use axum::{body::Body, http::{Request, StatusCode}};
use std::sync::Arc;
use tower::ServiceExt;

use crate::db::{DB, DbStore};

fn auth_store_and_token(organization_id: &str) -> (Arc<::server_auth::Store>, String) {
    auth_store_and_token_with_roles(organization_id, vec!["ADMIN".to_string()])
}

fn auth_store_and_token_with_roles(
    organization_id: &str,
    roles: Vec<String>,
) -> (Arc<::server_auth::Store>, String) {
    let store = Arc::new(::server_auth::Store::new());
    let now = chrono::Utc::now();
    let token = store
        .issue_token(&::server_auth::User {
            id: "user-a".to_string(),
            username: "user-a".to_string(),
            email: "user-a@example.com".to_string(),
            password_hash: String::new(),
            roles,
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
async fn test_viewer_cannot_manage_action_required_drafts() {
    let db = Arc::new(DB {
        pool: sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost/ohc").unwrap(),
        store: DbStore::Postgres,
    });
    let (auth_store, token) = auth_store_and_token_with_roles(
        "12345678-1234-1234-1234-123456789012",
        vec!["VIEWER".to_string()],
    );
    let app = super::action_required::router(db, auth_store);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_list_pending_drafts_unauthorized() {
    let db = Arc::new(DB {
        pool: sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost/ohc").unwrap(),
        store: DbStore::Postgres,
    });

    let app = super::action_required::router(db, Arc::new(::server_auth::Store::new()));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_approve_draft_unauthorized() {
    let db = Arc::new(DB {
        pool: sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost/ohc").unwrap(),
        store: DbStore::Postgres,
    });

    let app = super::action_required::router(db, Arc::new(::server_auth::Store::new()));

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/12345678-1234-1234-1234-123456789012/approve")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_edit_draft_unauthorized() {
    let db = Arc::new(DB {
        pool: sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost/ohc").unwrap(),
        store: DbStore::Postgres,
    });

    let app = super::action_required::router(db, Arc::new(::server_auth::Store::new()));

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/12345678-1234-1234-1234-123456789012/edit")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"response": "Test"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_list_pending_drafts_rejects_forged_tenant_header() {
    let db = Arc::new(DB {
        pool: sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost/ohc").unwrap(),
        store: DbStore::Postgres,
    });

    let app = super::action_required::router(db, Arc::new(::server_auth::Store::new()));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .header("x-tenant-id", "invalid-uuid")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_approve_draft_invalid_draft_id() {
    let db = Arc::new(DB {
        pool: sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost/ohc").unwrap(),
        store: DbStore::Postgres,
    });

    let (auth_store, token) = auth_store_and_token("12345678-1234-1234-1234-123456789012");
    let app = super::action_required::router(db, auth_store);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/invalid-draft-id/approve")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_edit_draft_invalid_draft_id() {
    let db = Arc::new(DB {
        pool: sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost/ohc").unwrap(),
        store: DbStore::Postgres,
    });

    let (auth_store, token) = auth_store_and_token("12345678-1234-1234-1234-123456789012");
    let app = super::action_required::router(db, auth_store);

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/invalid-draft-id/edit")
                .header("authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(r#"{"response": "Test"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
