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
    State(_agent): State<Arc<OnboardingAgent>>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    Ok(Json(serde_json::json!({
        "state": "{}"
    })))
}

async fn save_state(
    State(agent): State<Arc<OnboardingAgent>>,
    req: axum::extract::Request,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {

    // We need to parse body manually since we took req
    let (parts, body) = req.into_parts();
    let payload_bytes = axum::body::to_bytes(body, usize::MAX).await.map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    let payload: serde_json::Value = serde_json::from_slice(&payload_bytes).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;

    // Extract auth info
    let auth_info = parts.extensions.get::<::server_auth::orchestration::AuthInfo>();

    let (tenant_id, org_id, user_id) = if let Some(auth) = auth_info {
        (auth.org_id.clone(), auth.org_id.clone(), auth.spiffe_id.clone())
    } else {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    };

    let current_step = payload.get("step").and_then(|s| s.as_i64()).unwrap_or(0) as i32;

    let res = sqlx::query(
        "INSERT INTO onboarding_state (tenant_id, organization_id, user_id, current_step, state_json) \
         VALUES ($1, $2, $3, $4, $5) \
         ON CONFLICT (tenant_id, organization_id) DO UPDATE \
         SET state_json = onboarding_state.state_json || EXCLUDED.state_json, \
             current_step = EXCLUDED.current_step, \
             updated_at = CURRENT_TIMESTAMP"
    )
    .bind(tenant_id)
    .bind(org_id)
    .bind(user_id)
    .bind(current_step)
    .bind(&payload)
    .execute(&agent.db.pool)
    .await;

    match res {
        Ok(_) => Ok(axum::http::StatusCode::NO_CONTENT),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
