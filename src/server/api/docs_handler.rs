use axum::{response::{Html, IntoResponse, Json}, extract};
use serde::{Deserialize, Serialize};
use crate::docs::DocRegistry;
use std::sync::Arc;
use tokio::sync::OnceCell;
static DOC_REGISTRY: OnceCell<Arc<DocRegistry>> = OnceCell::const_new();
async fn get_registry() -> Arc<DocRegistry> {
    DOC_REGISTRY.get_or_init(|| async { Arc::new(DocRegistry::new()) }).await.clone()
}

pub async fn articles_handler() -> impl IntoResponse {
    let reg = get_registry().await;
    Json(reg.articles.clone())
}

pub async fn videos_handler() -> impl IntoResponse { Json(get_registry().await.videos.clone()) }
pub async fn tooltips_handler() -> impl IntoResponse { Json(get_registry().await.tooltips.clone()) }
#[derive(Deserialize)] pub struct ChatRequest { pub message: String }
#[derive(Serialize)] pub struct ChatResponse { pub response: String }
pub async fn chat_handler(extract::Json(payload): extract::Json<ChatRequest>) -> impl IntoResponse {
    Json(ChatResponse { response: get_registry().await.search_chat_query(&payload.message) })
}
pub async fn api_docs_handler() -> impl IntoResponse { Html(r#"<!DOCTYPE html><html><head><title>API</title></head><body>API</body></html>"#) }
