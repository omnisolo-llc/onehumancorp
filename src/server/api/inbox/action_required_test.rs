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
