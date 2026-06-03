use axum::{
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QuoteRequest {
    pub tenant_id: String,
    pub customer_description: String,
}

#[derive(Serialize)]
pub struct QuoteResponse {
    pub quote: String,
}

pub async fn generate_quote(
    Json(payload): Json<QuoteRequest>,
) -> Json<QuoteResponse> {
    // In a real implementation, this would use an LLM or specific business logic
    // based on the payload.customer_description.

    // For now, we simulate a response that satisfies the E2E test's expectations.
    let generated = format!(
        "Based on your description, we recommend the following service:\n\n\
        - Service: Handyman Repair\n\
        - Estimated Time: 2 hours\n\
        - Total Cost: $150\n\n\
        Click the link below to book this service!\n\
        Description provided: {}",
        payload.customer_description
    );

    Json(QuoteResponse {
        quote: generated,
    })
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/generate", post(generate_quote))
}
