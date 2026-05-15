use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub duration: String,
}

pub async fn videos_handler() -> impl IntoResponse {
    let videos = vec![
        Video {
            id: "v1".to_string(),
            title: "How to accept your first payment".to_string(),
            duration: "1:20".to_string(),
        },
        Video {
            id: "v2".to_string(),
            title: "Setting up your AI Agent".to_string(),
            duration: "0:55".to_string(),
        },
        Video {
            id: "v3".to_string(),
            title: "Understanding your billing plan".to_string(),
            duration: "1:45".to_string(),
        },
    ];
    Json(videos)
}

pub async fn openapi_handler() -> impl IntoResponse {
    let openapi = serde_json::json!({
        "openapi": "3.0.0",
        "info": {
            "title": "OneHumanCorp API",
            "version": "1.0.0"
        },
        "paths": {
            "/v1/products": {
                "get": {
                    "summary": "List your products",
                    "responses": {
                        "200": {
                            "description": "A list of products"
                        }
                    }
                }
            },
            "/v1/checkout": {
                "post": {
                    "summary": "Create a checkout session",
                    "responses": {
                        "200": {
                            "description": "Session created"
                        }
                    }
                }
            },
            "/v1/customers": {
                "get": {
                    "summary": "List your customers",
                    "responses": {
                        "200": {
                            "description": "A list of customers"
                        }
                    }
                }
            }
        }
    });
    Json(openapi)
}

#[derive(Deserialize)]
pub struct ChatRequest {
    pub query: String,
}

#[derive(Serialize)]
pub struct ChatResponse {
    pub reply: String,
}

pub async fn chat_handler(Json(req): Json<ChatRequest>) -> impl IntoResponse {
    let val = req.query.to_lowercase();
    let mut reply = "I can help with that! Please check our Setup Wizard to get started.".to_string();

    if val.contains("refund") {
        reply = "To issue a refund, go to your dashboard and select the transaction. <a href='#' onclick='openArticle(\"Issuing Refunds\")'>Read the full article →</a>".to_string();
    } else if val.contains("agent") {
        reply = "You can train your AI agent by giving it your website link! <a href='#' onclick='openArticle(\"Training your AI Agent\")'>Read the full article →</a>".to_string();
    }

    Json(ChatResponse { reply })
}
