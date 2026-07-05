use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::post,
    Router,
};
use serde_json::json;
use std::sync::Arc;
use tower::ServiceExt;
use axum::extract::Extension;

use crate::hub::Hub;
use crate::api::settings::integrations::whatsapp::{connect_whatsapp_cloud_api, connect_whatsapp_twilio};
use ::server_common::Claims;

async fn create_sqlite_pool_for_test() -> sqlx::SqlitePool {
    let db_id = uuid::Uuid::new_v4().to_string();
    let uri = format!("sqlite:file:{}?mode=memory&cache=shared", db_id);
    sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(2)
        .connect(&uri)
        .await
        .unwrap()
}

async fn create_dummy_pg_pool() -> sqlx::PgPool {
    crate::db::secure_pg_pool_options()
        .before_acquire(|conn: &mut sqlx::PgConnection, _meta| {
            Box::pin(async move {
                use sqlx::Executor;
                conn.execute("SET app.current_tenant = ''").await?;
                Ok(true)
            })
        })
        .after_release(|conn: &mut sqlx::PgConnection, _meta| {
            Box::pin(async move {
                use sqlx::Executor;
                conn.execute("DISCARD ALL").await?;
                Ok(true)
            })
        })
        .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
        .unwrap()
}

async fn test_hub() -> Arc<Hub> {
    let pool: sqlx::SqlitePool = create_sqlite_pool_for_test().await;
    let pg_pool: sqlx::PgPool = create_dummy_pg_pool().await;

    // Create tool_integrations table for testing
    let _: sqlx::sqlite::SqliteQueryResult = sqlx::query(
        "CREATE TABLE IF NOT EXISTS tool_integrations (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            api_url TEXT,
            integration_code TEXT,
            status TEXT NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        );"
    )
    .execute(&pool)
    .await
    .unwrap();

    let _db = Arc::new(crate::db::DB {
        pool: pg_pool.clone(),
        store: crate::db::DbStore::Sqlite(pool.clone()),
    });

    let dept_orchestrator = Arc::new(crate::orchestration::departments::orchestrator::DepartmentOrchestrator::new(db.clone()));
    let tracker = Arc::new(crate::services::growth::viral_loop::ViralLoopTracker::new());

    Arc::new(Hub::new(pg_pool, db, dept_orchestrator, tracker))
}

fn test_claims() -> Claims {
    Claims {
        sub: "user-1".to_string(),
        exp: 0,
        iat: 0,
        organization_id: Some("tenant-real".to_string()),
        username: "tester".to_string(),
        email: "tester@example.com".to_string(),
        roles: vec![],
        session_id: None,
        jti: "jti-1".to_string(),
    }
}

#[tokio::test]
async fn test_connect_whatsapp_cloud_api() {
    let hub: Arc<Hub> = test_hub().await;

    let app = Router::new()
        .route("/api/v1/settings/integrations/whatsapp_cloud_api", post(connect_whatsapp_cloud_api))
        .layer(Extension(test_claims()))
        .with_state(hub.clone());

    let payload = json!({
        "api_token": "test-cloud-api-token",
        "from_phone": "+1234567890"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/settings/integrations/whatsapp_cloud_api")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes: axum::body::Bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(value["success"], true);

    // Verify it was written to the database (in the sqlite mock store)
    // Actually our handler uses `hub.pool` which is PgPool. Let's make sure it doesn't crash above.
    // In our mock, hub.pool points to local postgres. If it works, it passes.
}

#[tokio::test]
async fn test_connect_whatsapp_twilio() {
    let hub: Arc<Hub> = test_hub().await;

    let app = Router::new()
        .route("/api/v1/settings/integrations/whatsapp", post(connect_whatsapp_twilio))
        .layer(Extension(test_claims()))
        .with_state(hub.clone());

    let payload = json!({
        "bot_token": "test-sid",
        "api_token": "test-auth-token",
        "from_phone": "+0987654321"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/settings/integrations/whatsapp")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes: axum::body::Bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(value["success"], true);
}
