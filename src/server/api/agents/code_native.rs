use axum::{Json, routing::post, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CodeNativeRequest {
    // any configuration
}

#[derive(Serialize)]
pub struct CodeNativeResponse {
    pub results: Vec<String>,
}

pub async fn execute_code_native(_req: Json<CodeNativeRequest>) -> Json<CodeNativeResponse> {
    // Simulated invocation of the code-native pipeline logic for UI
    let results = vec![
        "Generated rich data with ID: test_id".to_string(),
        "Processed data natively. New record count: 2".to_string(),
    ];
    Json(CodeNativeResponse { results })
}

pub fn routes() -> Router {
    Router::new().route("/api/agents/code-native", post(execute_code_native))
}
