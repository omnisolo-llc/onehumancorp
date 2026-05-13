use axum::{
    extract::{State, Json, Extension, Request},
    routing::{post, get},
    Router,
    response::Response,
    middleware::{self, Next},
    http::StatusCode,
};
use std::sync::Arc;
use crate::services::onboarding::onboarding_agent::OnboardingAgent;
use ::server_ohc::orchestration::{StartOnboardingRequest, StartOnboardingResponse};

#[derive(Clone)]
pub struct TenantContextKey {
    pub tenant_id: String,
}

pub async fn tenant_auth_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let claims = req.extensions().get::<::server_common::Claims>().cloned();

    let tenant_id = match claims {
        Some(c) => match c.organization_id {
            Some(org) => {
                if org.trim().is_empty() {
                    return Err(StatusCode::UNAUTHORIZED);
                }
                org
            },
            None => return Err(StatusCode::UNAUTHORIZED),
        },
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let mut req = req;
    req.extensions_mut().insert(TenantContextKey { tenant_id });

    Ok(next.run(req).await)
}

pub fn router(agent: Arc<OnboardingAgent>) -> Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    let r = Router::new()
        .route("/start", post(start_onboarding))
        .route("/state", get(get_state))
        .route("/state", post(save_state))
        .route_layer(middleware::from_fn(tenant_auth_middleware))
        .with_state(agent);

    // Convert to accept MeshTransport state
    Router::new().merge(r)
}

async fn start_onboarding(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(tenant_ctx): Extension<TenantContextKey>,
    Json(mut payload): Json<StartOnboardingRequest>,
) -> Result<Json<StartOnboardingResponse>, axum::http::StatusCode> {
    payload.organization_id = Some(tenant_ctx.tenant_id);
    match agent.start_onboarding(payload).await {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_state(
    State(_agent): State<Arc<OnboardingAgent>>,
    Extension(_tenant_ctx): Extension<TenantContextKey>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    Ok(Json(serde_json::json!({
        "state": "{}"
    })))
}

async fn save_state(
    State(_agent): State<Arc<OnboardingAgent>>,
    Extension(_tenant_ctx): Extension<TenantContextKey>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    Ok(axum::http::StatusCode::NO_CONTENT)
}
