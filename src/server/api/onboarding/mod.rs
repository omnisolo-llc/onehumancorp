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
        .route("/intake", post(process_intake_handler))
        .route("/state", get(get_state).post(save_state))
        .route("/launch", post(launch_onboarding))
        .route("/draft", get(get_draft).post(save_draft))
        .route("/track", post(track_onboarding_step_handler))
        .with_state(agent);

    // Convert to accept MeshTransport state
    Router::new().merge(r)
}

#[derive(serde::Deserialize)]
pub struct TrackRequest {
    pub step: String,
}

async fn track_onboarding_step_handler(
    headers: axum::http::HeaderMap,
    Json(payload): Json<TrackRequest>,
) -> axum::http::StatusCode {
    let tenant_id = headers.get("X-Tenant-ID").and_then(|v| v.to_str().ok()).unwrap_or("default_tenant");
    crate::telemetry::record_onboarding_step_reached(tenant_id, &payload.step);
    axum::http::StatusCode::OK
}

#[derive(serde::Deserialize)]
pub struct IntakeRequest {
    pub description: String,
}

async fn process_intake_handler(
    State(agent): State<Arc<OnboardingAgent>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<IntakeRequest>,
) -> Result<Json<crate::services::onboarding::onboarding_agent::IntakeData>, axum::http::StatusCode> {
    let tenant_id = headers.get("X-Tenant-ID").and_then(|v| v.to_str().ok()).unwrap_or("default_tenant");
    crate::telemetry::record_onboarding_step_reached(tenant_id, "intake_started");

    let start = std::time::Instant::now();
    match agent.process_intake(&payload.description).await {
        Ok(data) => {
            crate::telemetry::track_onboarding_step(tenant_id, "intake_completed", start.elapsed().as_millis() as u64);
            Ok(Json(data))
        },
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_draft(
    State(agent): State<Arc<OnboardingAgent>>,
    headers: axum::http::HeaderMap,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let tenant_id = headers.get("X-Tenant-ID").and_then(|v| v.to_str().ok()).unwrap_or("default_tenant");
    let user_id = headers.get("X-User-ID").and_then(|v| v.to_str().ok()).unwrap_or("default_user");
    match agent.get_onboarding_state(tenant_id, user_id).await {
        Ok(state) => {
            // For now, extract the bio field if we store it as a general state document
            // If there's no state or bio, returning an empty json is fine
            if let Some(bio) = state.get("bio") {
                Ok(Json(serde_json::json!({ "bio": bio })))
            } else {
                Ok(Json(serde_json::json!({ "bio": "" })))
            }
        },
        Err(_) => Ok(Json(serde_json::json!({ "bio": "" }))), // fallback
    }
}

async fn save_draft(
    State(agent): State<Arc<OnboardingAgent>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tenant_id = headers.get("X-Tenant-ID").and_then(|v| v.to_str().ok()).unwrap_or("default_tenant");
    let user_id = headers.get("X-User-ID").and_then(|v| v.to_str().ok()).unwrap_or("default_user");

    // Persist as step 0 or merge into state. Here we treat step=0 as drafting phase.
    match agent.save_onboarding_state(tenant_id, user_id, 0, &payload).await {
        Ok(_) => Ok(axum::http::StatusCode::OK),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn start_onboarding(
    State(agent): State<Arc<OnboardingAgent>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<StartOnboardingRequest>,
) -> Result<Json<StartOnboardingResponse>, axum::http::StatusCode> {
    let tenant_id = headers.get("X-Tenant-ID").and_then(|v| v.to_str().ok()).unwrap_or("default_tenant");
    crate::telemetry::record_onboarding_step_reached(tenant_id, "start_triggered");

    let start = std::time::Instant::now();
    match agent.start_onboarding(payload).await {
        Ok(res) => {
            crate::telemetry::track_onboarding_step(tenant_id, "provisioning_completed", start.elapsed().as_millis() as u64);
            Ok(Json(res))
        },
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn launch_onboarding(
    State(agent): State<Arc<OnboardingAgent>>,
    headers: axum::http::HeaderMap,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let tenant_id = headers.get("X-Tenant-ID").and_then(|v| v.to_str().ok()).unwrap_or("default_tenant");
    let user_id = headers.get("X-User-ID").and_then(|v| v.to_str().ok()).unwrap_or("default_user");
    let current_step = 5; // Launch step

    let state = serde_json::json!({
        "status": "launched"
    });
    match agent.save_onboarding_state(tenant_id, user_id, current_step, &state).await {
        Ok(_) => Ok(Json(state)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn get_state(
    State(agent): State<Arc<OnboardingAgent>>,
    headers: axum::http::HeaderMap,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let tenant_id = headers.get("X-Tenant-ID").and_then(|v| v.to_str().ok()).unwrap_or("default_tenant");
    let user_id = headers.get("X-User-ID").and_then(|v| v.to_str().ok()).unwrap_or("default_user");
    match agent.get_onboarding_state(tenant_id, user_id).await {
        Ok(state) => Ok(Json(state)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn save_state(
    State(agent): State<Arc<OnboardingAgent>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tenant_id = headers.get("X-Tenant-ID").and_then(|v| v.to_str().ok()).unwrap_or("default_tenant");
    let user_id = headers.get("X-User-ID").and_then(|v| v.to_str().ok()).unwrap_or("default_user");

    let step = payload.get("step").and_then(|s| s.as_i64()).unwrap_or(0) as i32;

    match agent.save_onboarding_state(tenant_id, user_id, step, &payload).await {
        Ok(_) => Ok(axum::http::StatusCode::NO_CONTENT),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
