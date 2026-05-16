use axum::{
    extract::{State, Json, Request},
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
    State(agent): State<Arc<OnboardingAgent>>,
    req: Request,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    // Axum request headers for tenant mapping
    let tenant_id = req.headers().get("X-Tenant-ID").and_then(|h| h.to_str().ok()).unwrap_or("test-tenant").to_string();
    let org_id = req.headers().get("X-Organization-ID").and_then(|h| h.to_str().ok()).unwrap_or("test-org").to_string();

    match agent.get_onboarding_state(&tenant_id, &org_id).await {
        Ok(state_json) => {
            let parsed: serde_json::Value = serde_json::from_str(&state_json).unwrap_or_else(|_| serde_json::json!({}));
            Ok(Json(serde_json::json!({
                "state": parsed.to_string()
            })))
        },
        Err(_) => {
            Ok(Json(serde_json::json!({
                "state": "{}"
            })))
        }
    }
}

async fn save_state(
    State(agent): State<Arc<OnboardingAgent>>,
    req: Request,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tenant_id = req.headers().get("X-Tenant-ID").and_then(|h| h.to_str().ok()).unwrap_or("test-tenant").to_string();
    let org_id = req.headers().get("X-Organization-ID").and_then(|h| h.to_str().ok()).unwrap_or("test-org").to_string();
    let user_id = req.headers().get("X-User-ID").and_then(|h| h.to_str().ok()).unwrap_or("test-user").to_string();

    let (_parts, body) = req.into_parts();
    let bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(b) => b,
        Err(_) => return Err(axum::http::StatusCode::BAD_REQUEST),
    };
    let payload: serde_json::Value = serde_json::from_slice(&bytes).unwrap_or_else(|_| serde_json::json!({}));

    let current_step = payload.get("current_step").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let state_string = serde_json::to_string(&payload).unwrap_or_else(|_| "{}".to_string());

    match agent.save_onboarding_state(&tenant_id, &org_id, &user_id, current_step, &state_string).await {
        Ok(_) => Ok(axum::http::StatusCode::NO_CONTENT),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
