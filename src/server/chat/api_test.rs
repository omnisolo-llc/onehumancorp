use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use super::api::chat_router;
use sqlx::PgPool;
use std::env;

#[tokio::test]
async fn test_chat_api_routes() {
    let pool = crate::harness::db::setup_test_db().await;
    let app = chat_router(pool.clone());

    let tenant_id = uuid::Uuid::new_v4().to_string();

    // Create an inbox
    let inbox_payload = serde_json::json!({
        "name": "Test Inbox",
        "channel_type": "email"
    });

    let req = Request::builder()
        .method("POST")
        .uri(format!("/api/v1/chat/{}/inboxes", tenant_id))
        .header("Content-Type", "application/json")
        .body(Body::from(inbox_payload.to_string()))
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let inbox: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let inbox_id = inbox["id"].as_str().unwrap().to_string();

    // Create a conversation using that inbox
    let conv_payload = serde_json::json!({
        "inbox_id": inbox_id,
        "contact_id": null
    });

    let req = Request::builder()
        .method("POST")
        .uri(format!("/api/v1/chat/{}/conversations", tenant_id))
        .header("Content-Type", "application/json")
        .body(Body::from(conv_payload.to_string()))
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Fetch conversations
    let req = Request::builder()
        .method("GET")
        .uri(format!("/api/v1/chat/{}/conversations", tenant_id))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
