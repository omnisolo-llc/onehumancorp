use super::help::{chat_handler, openapi_handler, videos_handler, ChatRequest};
use axum::{body::Body, response::IntoResponse};
use axum::Json;
use serde_json::Value;


async fn get_body_json(response: axum::response::Response) -> Value {
    let body = response.into_body();
    // Using axum::body::to_bytes in axum 0.7
    let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn test_videos_handler() {
    let response = videos_handler().await.into_response();
    assert_eq!(response.status(), 200);

    let json = get_body_json(response).await;
    let videos = json.as_array().expect("Should return an array");

    assert_eq!(videos.len(), 3);
    assert_eq!(videos[0]["id"], "v1");
    assert_eq!(videos[0]["title"], "How to accept your first payment");
}

#[tokio::test]
async fn test_openapi_handler() {
    let response = openapi_handler().await.into_response();
    assert_eq!(response.status(), 200);

    let json = get_body_json(response).await;

    assert_eq!(json["openapi"], "3.0.0");
    assert_eq!(json["info"]["title"], "OneHumanCorp API");

    // Check paths
    let paths = json["paths"].as_object().expect("Paths should be an object");
    assert!(paths.contains_key("/v1/products"));
    assert!(paths.contains_key("/v1/checkout"));
    assert!(paths.contains_key("/v1/customers"));
}

#[tokio::test]
async fn test_chat_handler_default_reply() {
    let req = ChatRequest { query: "hello there".to_string() };
    let response = chat_handler(Json(req)).await.into_response();

    let json = get_body_json(response).await;
    let reply = json["reply"].as_str().unwrap();

    assert!(reply.contains("Setup Wizard to get started"));
}

#[tokio::test]
async fn test_chat_handler_refund_routing() {
    let req = ChatRequest { query: "How do I process a refund?".to_string() };
    let response = chat_handler(Json(req)).await.into_response();

    let json = get_body_json(response).await;
    let reply = json["reply"].as_str().unwrap();

    assert!(reply.contains("go to your dashboard and select the transaction"));
    assert!(reply.contains("Read the full article"));
    assert!(reply.contains("openArticle(\"Issuing Refunds\")"));
}

#[tokio::test]
async fn test_chat_handler_agent_routing() {
    let req = ChatRequest { query: "I want to train my agent".to_string() };
    let response = chat_handler(Json(req)).await.into_response();

    let json = get_body_json(response).await;
    let reply = json["reply"].as_str().unwrap();

    assert!(reply.contains("giving it your website link"));
    assert!(reply.contains("Read the full article"));
    assert!(reply.contains("openArticle(\"Training your AI Agent\")"));
}
