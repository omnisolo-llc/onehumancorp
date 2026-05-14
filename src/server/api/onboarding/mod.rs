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
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let row = sqlx::query("SELECT state_json FROM onboarding_state LIMIT 1")
        .fetch_optional(&agent.db.pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(r) = row {
        use sqlx::Row;
        let json: serde_json::Value = r.get("state_json");
        return Ok(Json(serde_json::json!({ "state": json.to_string() })));
    }

    Ok(Json(serde_json::json!({
        "state": "{}"
    })))
}

async fn save_state(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let step = payload.get("step").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    sqlx::query("INSERT INTO onboarding_state (tenant_id, organization_id, user_id, current_step, state_json) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (tenant_id, organization_id) DO UPDATE SET state_json = EXCLUDED.state_json, current_step = EXCLUDED.current_step")
        .bind("default")
        .bind("org-default")
        .bind("usr-default")
        .bind(step)
        .bind(&payload)
        .execute(&agent.db.pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}
