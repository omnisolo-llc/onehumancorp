use axum::{routing::get, Json, Router};
use std::sync::Arc;
use crate::services::docs::service::MyDocsService;
use crate::ohc::docs::v1::docs_service_server::DocsService;
use crate::ohc::docs::v1::{GetHelpArticleRequest, SearchHelpArticlesRequest, GetTooltipRequest};

pub fn router() -> Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    Router::new()
        .route("/help-center", get(help_center))
        .route("/tooltip", get(tooltip))
        .route("/article/:id", get(article))
        .route("/videos", get(videos))
}

async fn help_center() -> Json<serde_json::Value> {
    let service = MyDocsService::new();
    let req = tonic::Request::new(SearchHelpArticlesRequest {
        query: "".to_string(),
        topic_filter: "".to_string(),
    });
    match service.search_help_articles(req).await {
        Ok(res) => {
            let mut articles_json = vec![];
            for article in res.into_inner().articles {
                articles_json.push(serde_json::json!({
                    "id": article.id,
                    "topic": article.topic,
                    "title": article.title,
                    "content_markdown": article.content_markdown
                }));
            }
            Json(serde_json::json!({
                "status": "ok",
                "articles": articles_json
            }))
        }
        Err(e) => {
            Json(serde_json::json!({
                "status": "error",
                "message": e.message()
            }))
        }
    }
}

async fn tooltip(axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>) -> Json<serde_json::Value> {
    let service = MyDocsService::new();
    let element_id = params.get("element_id").cloned().unwrap_or_default();
    let req = tonic::Request::new(GetTooltipRequest { element_id });

    match service.get_tooltip(req).await {
        Ok(res) => {
            if let Some(tooltip) = res.into_inner().tooltip {
                Json(serde_json::json!({
                    "status": "ok",
                    "tooltip": {
                        "title": tooltip.title,
                        "description": tooltip.plain_language_description
                    }
                }))
            } else {
                 Json(serde_json::json!({"status": "error", "message": "not found"}))
            }
        },
        Err(e) => Json(serde_json::json!({"status": "error", "message": e.message()}))
    }
}

async fn article(axum::extract::Path(id): axum::extract::Path<String>) -> Json<serde_json::Value> {
    let service = MyDocsService::new();
    let req = tonic::Request::new(GetHelpArticleRequest { id });

    match service.get_help_article(req).await {
        Ok(res) => {
            if let Some(article) = res.into_inner().article {
                Json(serde_json::json!({
                    "status": "ok",
                    "article": {
                        "id": article.id,
                        "topic": article.topic,
                        "title": article.title,
                        "content_markdown": article.content_markdown
                    }
                }))
            } else {
                Json(serde_json::json!({"status": "error", "message": "not found"}))
            }
        },
        Err(e) => Json(serde_json::json!({"status": "error", "message": e.message()}))
    }
}

async fn videos() -> Json<serde_json::Value> {
    // Return video metadata from the backend
    Json(serde_json::json!({
        "status": "ok",
        "videos": [
            { "id": "1", "title": "Set up your store", "url": "https://example.com/video1.mp4" },
            { "id": "2", "title": "Accept your first payment", "url": "https://example.com/video2.mp4" },
            { "id": "3", "title": "Activate your AI Support Agent", "url": "https://example.com/video3.mp4" }
        ]
    }))
}
