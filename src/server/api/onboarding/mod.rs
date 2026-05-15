use axum::{
    extract::{State, Json},
    routing::{post, get},
    Router,
};
use std::sync::Arc;
use crate::services::onboarding::onboarding_agent::OnboardingAgent;
use ::server_ohc::orchestration::{StartOnboardingRequest, StartOnboardingResponse};
use serde_json::{Value, json};
use tracing::error;

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
) -> Result<Json<Value>, axum::http::StatusCode> {
    let db = agent.db();
    match sqlx::query("SELECT state_json FROM onboarding_state WHERE organization_id = $1 LIMIT 1")
        .bind("system")
        .fetch_optional(&db.pool)
        .await
    {
        Ok(Some(row)) => {
            let state: Value = sqlx::Row::try_get(&row, "state_json").unwrap_or_else(|_| json!({}));
            Ok(Json(state))
        }
        Ok(None) => Ok(Json(json!({}))),
        Err(e) => {
            error!("Failed to fetch onboarding state: {}", e);
            Ok(Json(json!({}))) // Fallback to empty on error
        }
    }
}

async fn save_state(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let db = agent.db();
    let step = payload.get("step").and_then(|v| v.as_i64()).unwrap_or(0);

    // SQLite compatible upsert
    let query_str = if db.is_sqlite() {
        r#"
        INSERT INTO onboarding_state (tenant_id, organization_id, user_id, current_step, state_json)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT(organization_id, user_id) DO UPDATE SET current_step=excluded.current_step, state_json=excluded.state_json, updated_at=CURRENT_TIMESTAMP
        "#
    } else {
        r#"
        INSERT INTO onboarding_state (tenant_id, organization_id, user_id, current_step, state_json)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (organization_id, user_id)
        DO UPDATE SET current_step = $4, state_json = $5, updated_at = CURRENT_TIMESTAMP
        "#
    };

    let res = sqlx::query(query_str)
    .bind("system")
    .bind("system")
    .bind("test_user")
    .bind(step as i32)
    .bind(&payload)
    .execute(&db.pool)
    .await;

    match res {
        Ok(_) => Ok(axum::http::StatusCode::NO_CONTENT),
        Err(e) => {
            error!("Failed to save onboarding state: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
