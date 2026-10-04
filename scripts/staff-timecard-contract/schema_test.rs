//! These assertions fail if fresh storage loses receipts, the real upgrade is
//! disconnected, or an additive upgrade invents identity for a legacy row.
use sqlx::Row;

async fn pool() -> sqlx::SqlitePool {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::raw_sql(crate::ACTUAL_SQLITE_BUSINESS)
        .execute(&pool)
        .await
        .unwrap();
    pool
}

#[tokio::test]
async fn fresh_sqlite_bootstrap_has_nullable_receipt_identity() {
    let pool = pool().await;
    sqlx::query("INSERT INTO ohc_timecard_event(id,tenant_id,staff_id,event_type) VALUES('fresh','clock-tenant','clock-owner','CLOCK_IN')")
        .execute(&pool).await.unwrap();
    let identity: Option<String> =
        sqlx::query_scalar("SELECT request_identity FROM ohc_timecard_event WHERE id='fresh'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(
        identity, None,
        "fresh schema must not fabricate legacy receipts"
    );
    pool.close().await;
}

#[tokio::test]
async fn actual_sqlite_upgrade_is_idempotent_and_never_backfills_receipts() {
    let pool = pool().await;
    sqlx::query("ALTER TABLE ohc_timecard_event DROP COLUMN request_identity")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO ohc_timecard_event(id,tenant_id,staff_id,event_type) VALUES('legacy','clock-tenant','clock-owner','CLOCK_IN')")
        .execute(&pool).await.unwrap();
    crate::db::actual_sqlite_clock_upgrade(&pool).await.unwrap();
    crate::db::actual_sqlite_clock_upgrade(&pool).await.unwrap();
    let row = sqlx::query("SELECT id,request_identity FROM ohc_timecard_event")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(row.get::<String, _>("id"), "legacy");
    assert_eq!(row.get::<Option<String>, _>("request_identity"), None);
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM ohc_timecard_event")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1);
    pool.close().await;
}

// Compile and exercise the exact production parent mount, TimecardAccess
// configuration and protected bearer middleware, in addition to native cases.
#[tokio::test]
async fn actual_parent_mount_requires_authentication() {
    use crate::api::staff_timecards_test::fixture::{Backend, Fixture, ROUTE, clock};
    use tower::ServiceExt;
    let fixture = Fixture::new(Backend::Sqlite).await;
    let app = crate::actual_parent_timecard_app(fixture.db.clone(), fixture.auth.clone()).await;
    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method("POST")
                .uri(ROUTE)
                .header("content-type", "application/json")
                .body(axum::body::Body::from(
                    serde_json::json!({"events":[clock("unauthed-clock")]}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let count = fixture.count().await;
    fixture.finish().await;
    assert_eq!(status, axum::http::StatusCode::UNAUTHORIZED);
    assert_eq!(count, 0);
}
#[tokio::test]
async fn actual_parent_mount_commits_a_verified_clock_receipt() {
    use crate::api::staff_timecards_test::fixture::{Backend, Fixture, ROUTE, assert_ack, clock};
    use tower::ServiceExt;
    let fixture = Fixture::new(Backend::Sqlite).await;
    let app = crate::actual_parent_timecard_app(fixture.db.clone(), fixture.auth.clone()).await;
    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method("POST")
                .uri(ROUTE)
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", fixture.token))
                .body(axum::body::Body::from(
                    serde_json::json!({"events":[clock("mounted-clock")]}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
        .await
        .unwrap();
    let count = fixture.count().await;
    fixture.finish().await;
    assert_ack(
        &(status, serde_json::from_slice(&body).unwrap()),
        &["mounted-clock"],
    );
    assert_eq!(count, 1);
}
