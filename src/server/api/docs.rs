use axum::{routing::{get, post}, Router, Json};
use serde_json::json;

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/tooltips", get(get_tooltips))
        .route("/videos", get(get_videos))
        .route("/chat", post(post_chat))
}

async fn get_tooltips() -> Json<serde_json::Value> {
    Json(json!({
        "bio-input-tooltip": "Describe what you sell, your target audience, and the vibe of your brand.",
        "generate-btn-tooltip": "Our AI agents will analyze your description and build a ready-to-launch store for you.",
        "launch-btn-tooltip": "Launch your storefront immediately to a live URL."
    }))
}

async fn get_videos() -> Json<serde_json::Value> {
    Json(json!([
        { "id": 1, "title": "Set up your store", "duration": "1m 15s" },
        { "id": 2, "title": "Add your first product", "duration": "1m 20s" },
        { "id": 3, "title": "Accepting payments", "duration": "1m 10s" },
        { "id": 4, "title": "Activate your AI Support Agent", "duration": "1m 25s" },
        { "id": 5, "title": "Design your storefront", "duration": "1m 05s" },
        { "id": 6, "title": "Manage your inventory", "duration": "0m 55s" },
        { "id": 7, "title": "Configure shipping rates", "duration": "1m 18s" },
        { "id": 8, "title": "Launch marketing campaigns", "duration": "1m 22s" },
        { "id": 9, "title": "Analyze your sales", "duration": "1m 12s" },
        { "id": 10, "title": "Upgrade to premium", "duration": "0m 45s" }
    ]))
}

async fn post_chat() -> Json<serde_json::Value> {
    Json(json!({ "reply": "I'm your AI assistant! Since you're asking about store setup, check out our Getting Started guide. <br/><br/><a href=\"/help\" class=\"text-blue-600 font-bold hover:underline\">Read the full article →</a>" }))
}
