use axum::{body::Body, http::{Request, StatusCode}};
use std::sync::Arc;
use tower::ServiceExt;

use crate::db::{DB, DbStore};

#[tokio::test]
async fn test_list_pending_drafts_unauthorized() {
    let db = Arc::new(DB {
        pool: sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost/ohc").unwrap(),
        store: DbStore::Postgres,
    });

    let app = super::action_required::router(db);

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

    let app = super::action_required::router(db);

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

    let app = super::action_required::router(db);

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
async fn test_list_pending_drafts_invalid_tenant() {
    let db = Arc::new(DB {
        pool: sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost/ohc").unwrap(),
        store: DbStore::Postgres,
    });

    let app = super::action_required::router(db);

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

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_approve_draft_invalid_draft_id() {
    let db = Arc::new(DB {
        pool: sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost/ohc").unwrap(),
        store: DbStore::Postgres,
    });

    let app = super::action_required::router(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/invalid-draft-id/approve")
                .header("x-tenant-id", "12345678-1234-1234-1234-123456789012")
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

    let app = super::action_required::router(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/invalid-draft-id/edit")
                .header("x-tenant-id", "12345678-1234-1234-1234-123456789012")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"response": "Test"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
