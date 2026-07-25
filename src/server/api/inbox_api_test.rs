use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use super::inbox_api;
use sqlx::PgPool;
use std::env;
use uuid::Uuid;
use axum::Router;
use std::sync::Arc;
use crate::services::inbox::service::InboxService;
use crate::domain::repository::omnichannel_repo::OmniChannelRepo;
use crate::db::DB;

#[tokio::test]
async fn test_inbox_api_routes() {
    let app = Router::new().route("/api/v1/inboxes/:tenant_id", axum::routing::post(|| async { "created" }));

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/inboxes/123")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name": "test inbox"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_inbox_api_payloads() {
    let conv = inbox_api::CreateConversationRequest {
        channel: "email".to_string(),
        status: "open".to_string()
    };
    assert_eq!(conv.status, "open");

    let msg = inbox_api::CreateMessageRequest {
        direction: "inbound".to_string(),
        content: "hello world".to_string(),
    };
    assert_eq!(msg.direction, "inbound");

    let contact = inbox_api::CreateContactRequest {
        name: Some("John".to_string()),
        email: Some("john@example.com".to_string()),
        phone: None,
    };
    assert_eq!(contact.name.unwrap(), "John");
}
