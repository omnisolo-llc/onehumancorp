use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct WalkupPayload {
    pub tenant_id: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct WalkupResponse {
    pub success: bool,
    pub structured_order: Option<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<crate::db::DB>,
}

pub fn walkup_routes<S: Clone + Send + Sync + 'static>(state: AppState) -> Router<S> {
    Router::new()
        .route("/", post(handle_walkup))
        .with_state(state)
}

pub async fn handle_walkup(
    State(state): State<AppState>,
    Json(payload): Json<WalkupPayload>,
) -> impl IntoResponse {
    let tenant_id = &payload.tenant_id;
    let message = &payload.message;
    let pool = &state.db.pool;

    let target_language: String = {
        let prefs_row = sqlx::query(
            "SELECT language_preference FROM tenants WHERE id = $1"
        )
        .bind(tenant_id)
        .fetch_optional(pool)
        .await
        .unwrap_or(None);

        match prefs_row {
            Some(r) => {
                use sqlx::Row;
                let lang: Option<String> = r.get("language_preference");
                lang.unwrap_or_else(|| "en".to_string())
            }
            None => "en".to_string(),
        }
    };

    let prompt = format!(
        "You are a Multilingual Order Interceptor for a small business. Detect the language of the following input, translate it to {}, extract the intent (Order, Query, Status Check, etc.), and extract any items and quantities.\nInput: {}\nReturn JSON format exactly like: {{\"intent\": \"Order\", \"translated_text\": \"3x Chicken Tacos\", \"items\": [\"3x Chicken Tacos\"]}}",
        target_language, message
    );

    let raw_response = match std::env::var("OHC_TRANSLATION_LLM_PROVIDER")
        .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
        .as_deref()
    {
        Ok("minimax") => {
            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
            if api_key.trim().is_empty() {
                crate::minimax::LocalLLMClient::new().reason(&crate::pricing::compression::reduce_tokens(&prompt)).await.unwrap_or_default()
            } else {
                crate::minimax::MinimaxClient::new(api_key).reason(&crate::pricing::compression::reduce_tokens(&prompt)).await.unwrap_or_default()
            }
        },
        _ => crate::minimax::LocalLLMClient::new().reason(&crate::pricing::compression::reduce_tokens(&prompt)).await.unwrap_or_default(),
    };

    let clean_res = raw_response.trim_matches('`').trim_start_matches("json\n").trim_end();
    if let Ok(translated_json) = serde_json::from_str::<serde_json::Value>(clean_res) {
        let intent = translated_json.get("intent").and_then(|v| v.as_str()).unwrap_or("Query");
        let translated_text = translated_json.get("translated_text").and_then(|v| v.as_str()).unwrap_or(message);

        if intent == "Order" {
            let _ = sqlx::query(
                "INSERT INTO triage_items (id, tenant_id, source, priority, context, status) VALUES ($1, $2, 'Multilingual Interceptor Agent', 'high', $3, 'pending')"
            )
            .bind(uuid::Uuid::new_v4().to_string())
            .bind(tenant_id)
            .bind(translated_text)
            .execute(pool)
            .await;

            return (StatusCode::OK, Json(WalkupResponse { success: true, structured_order: Some(translated_text.to_string()) })).into_response();
        }
    }

    (StatusCode::OK, Json(WalkupResponse { success: true, structured_order: None })).into_response()
}
