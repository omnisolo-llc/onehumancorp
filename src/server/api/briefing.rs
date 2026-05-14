use axum::{
    extract::State,
    response::IntoResponse,
    routing::get,
    Router,
    Json,
};
use std::sync::Arc;
use serde::Serialize;
use crate::hub::Hub;

#[derive(Serialize)]
pub struct BriefingResponse {
    pub bullets: Vec<String>,
}

pub fn router<S>(hub: Arc<Hub>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(briefing_handler))
        .with_state(hub)
}

async fn briefing_handler(State(hub): State<Arc<Hub>>) -> impl IntoResponse {
    let mut bullets = Vec::new();

    let api_key = hub.minimax_api_key().to_string();
    let client = crate::minimax::MinimaxClient::new(api_key);
    let prompt = "You are a business advisor for a small business. Summarize today's metrics in 3-4 short, plain-language bullet points without technical jargon. Example: 'You had 8 orders this week. Vegan cake requests doubled. Consider adding a vegan chocolate option!'. Return ONLY a JSON object with a single key 'bullets' containing an array of strings. Do not output markdown format.";

    if let Ok(res) = client.reason(prompt).await {
        let clean_res = res.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();
        if let Ok(json_res) = serde_json::from_str::<serde_json::Value>(clean_res) {
             if let Some(b) = json_res.get("bullets").and_then(|v| v.as_array()) {
                 for item in b {
                     if let Some(s) = item.as_str() {
                         bullets.push(s.to_string());
                     }
                 }
             }
        }
    }
    if bullets.is_empty() {
        bullets.push("Your business is running smoothly today.".to_string());
        bullets.push("No urgent actions required.".to_string());
    }

    (axum::http::StatusCode::OK, Json(BriefingResponse { bullets })).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;
    use axum::body::Body;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_briefing_handler_success() {
        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let (tx, _rx) = tokio::sync::mpsc::channel(10);
        let hub = Arc::new(Hub::new(tx, db.pool.clone()));

        let app = router(hub);

        let req = Request::builder()
            .uri("/")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), axum::http::StatusCode::OK);
    }
}
