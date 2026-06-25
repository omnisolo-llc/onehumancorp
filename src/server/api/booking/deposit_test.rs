use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use std::sync::Arc;
use server_lib::db::{DB, DbStore};
use server_lib::api::booking::deposit;

#[tokio::test]
async fn test_deposit_booking_missing_tenant() {
    let pool = sqlx::PgPool::connect_lazy("postgres://dummy").unwrap();
    let db = Arc::new(DB { pool, store: DbStore::Postgres });
    let app = deposit::router(db);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/?booking_id=123")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
