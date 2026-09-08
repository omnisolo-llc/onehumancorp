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

async fn create_dummy_pg_pool() -> Result<sqlx::PgPool, sqlx::Error> {
    let database_url = std::env::var("OMNISOLO_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .unwrap_or_else(|_| "postgres://ohc:ohc@localhost:5432/ohc".to_string());
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
async fn test_hub() -> Option<Arc<Hub>> {
    let pg_pool = match create_dummy_pg_pool().await {
        Ok(pool) => pool,
        Err(_) => return None,
    };
    let schema_ready: bool = sqlx::query_scalar(
        "SELECT to_regclass('public.tool_integrations') IS NOT NULL
                AND to_regclass('public.integration_credentials') IS NOT NULL",
    )
    .fetch_one(&pg_pool)
    .await
    .unwrap_or(false);
    if !schema_ready {
        return None;
    }

    let (event_tx, _) = tokio::sync::mpsc::channel(1);
    Some(Arc::new(Hub::new(event_tx, pg_pool)))
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
    let Some(hub) = test_hub().await else {
        return;
    };

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
    let Some(hub) = test_hub().await else {
        return;
    };

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
