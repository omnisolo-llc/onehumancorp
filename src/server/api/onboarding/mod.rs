use axum::{
    extract::{State, Json},
    routing::{post, get},
    Router,
};
use std::sync::Arc;
use crate::services::onboarding::onboarding_agent::OnboardingAgent;
use ::server_ohc::orchestration::{StartOnboardingRequest, StartOnboardingResponse};

pub fn router(agent: Arc<OnboardingAgent>) -> Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    let r = Router::new()
        .route("/start", post(start_onboarding))
        .route("/state", get(get_state))
        .route("/state", post(save_state))
        .route("/diagnostics", get(get_diagnostics))
        .route("/growth-tips", get(get_growth_tips))
        .route("/generate-description", post(generate_description))
        .route("/generate-tagline", post(generate_tagline))
        .with_state(agent);

    // Convert to accept MeshTransport state
    Router::new().merge(r)
}

async fn start_onboarding(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<StartOnboardingRequest>,
) -> Result<Json<StartOnboardingResponse>, axum::http::StatusCode> {
    match agent.start_onboarding(payload).await {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_state(
    State(_agent): State<Arc<OnboardingAgent>>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    Ok(Json(serde_json::json!({
        "state": "{}"
    })))
}

async fn save_state(
    State(_agent): State<Arc<OnboardingAgent>>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    Ok(axum::http::StatusCode::NO_CONTENT)
}

async fn get_diagnostics(
    State(_agent): State<Arc<OnboardingAgent>>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "warning",
        "issues": [
            {
                "id": "AUTH_EXPIRED",
                "agent": "The Manager",
                "message": "Connection Key has expired"
            }
        ]
    }))
}

async fn get_growth_tips(
    State(_agent): State<Arc<OnboardingAgent>>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "stage": "Early Launch",
        "tips": [
            { "title": "Add 5 more products", "impact": "high" },
            { "title": "Connect Instagram", "impact": "medium" },
            { "title": "Run Email Campaign", "impact": "medium" }
        ]
    }))
}

#[derive(serde::Deserialize)]
pub struct GenerateRequest {
    pub name: String,
    pub business_type: String,
}

#[derive(serde::Serialize)]
pub struct GenerateResponse {
    pub suggestion: String,
}

async fn generate_description(
    State(_agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<GenerateRequest>,
) -> Json<GenerateResponse> {
    let suggestion = format!("Welcome to {}! We are a premier {} dedicated to providing exceptional quality and service to our community. Our passion drives us to deliver the best possible experience for every customer.", payload.name, payload.business_type);
    Json(GenerateResponse { suggestion })
}

async fn generate_tagline(
    State(_agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<GenerateRequest>,
) -> Json<GenerateResponse> {
    let suggestion = format!("{}: Excellence in {} services.", payload.name, payload.business_type);
    Json(GenerateResponse { suggestion })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_logic_placeholder() {
        let payload = GenerateRequest {
            name: "Maya Cakes".to_string(),
            business_type: "Bakery".to_string(),
        };
        assert_eq!(payload.name, "Maya Cakes");
        assert_eq!(payload.business_type, "Bakery");
    }
}
