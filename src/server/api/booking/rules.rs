use axum::{
    extract::Json,
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use crate::minimax::MinimaxClient;

#[derive(Deserialize)]
pub struct ExtractRulesRequest {
    pub prompt: String,
}

#[derive(Serialize, Deserialize)]
pub struct StructuredRules {
    pub working_days: Vec<String>,
    pub start_time: String,
    pub end_time: String,
    pub buffer_time_minutes: i32,
}

#[derive(Serialize)]
pub struct ExtractRulesResponse {
    pub success: bool,
    pub rules: Option<StructuredRules>,
    pub error_message: Option<String>,
}

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/", post(extract_rules_handler))
}

async fn extract_rules_handler(
    Json(payload): Json<ExtractRulesRequest>,
) -> impl IntoResponse {
    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
    let client = MinimaxClient::new(api_key);

    let llm_prompt = format!(
        "You are an AI assistant that extracts structured scheduling rules from natural language.
        Extract the working days, start time, end time, and buffer time in minutes from the following text:
        \"{}\"

        Return ONLY a raw JSON object (no markdown, no quotes) with these exact keys:
        - working_days: array of strings (e.g. [\"Mon\", \"Tue\", \"Wed\", \"Thu\", \"Fri\", \"Sat\", \"Sun\"])
        - start_time: string (e.g. \"09:00 AM\")
        - end_time: string (e.g. \"05:00 PM\")
        - buffer_time_minutes: integer (e.g. 30)",
        payload.prompt
    );

    match client.reason(&llm_prompt).await {
        Ok(json_str) => {
            let json_str = json_str.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();
            match serde_json::from_str::<StructuredRules>(json_str) {
                Ok(rules) => {
                    (
                        StatusCode::OK,
                        Json(ExtractRulesResponse {
                            success: true,
                            rules: Some(rules),
                            error_message: None,
                        }),
                    ).into_response()
                }
                Err(e) => {
                    tracing::error!("Failed to parse LLM rules output: {}. Raw: {}", e, json_str);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ExtractRulesResponse {
                            success: false,
                            rules: None,
                            error_message: Some("Failed to parse scheduling rules".to_string()),
                        }),
                    ).into_response()
                }
            }
        }
        Err(e) => {
            tracing::error!("LLM extraction failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ExtractRulesResponse {
                    success: false,
                    rules: None,
                    error_message: Some("Failed to communicate with LLM".to_string()),
                }),
            ).into_response()
        }
    }
}
