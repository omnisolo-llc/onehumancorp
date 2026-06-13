use axum::{
    extract::{State, Json, Extension},
    routing::{post, get},
    Router,
};
use std::sync::Arc;
use crate::services::onboarding::onboarding_agent::OnboardingAgent;
use ::server_ohc::orchestration::{StartOnboardingRequest, StartOnboardingResponse};

pub fn router(agent: Arc<OnboardingAgent>) -> Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    let r = Router::new()
        .route("/start", post(start_onboarding))
        .route("/zero_click", post(zero_click_handler))

        .route("/intake", post(process_intake_handler))
        .route("/state", get(get_state).post(save_state))
        .route("/launch", post(launch_onboarding))
        .route("/draft", get(get_draft).post(save_draft))
        .layer(axum::middleware::from_fn(::server_auth::guest_auth_middleware))
        .with_state(agent);

    // Convert to accept MeshTransport state
    Router::new().merge(r)
}

#[derive(serde::Deserialize)]
pub struct IntakeRequest {
    pub description: String,
}

async fn process_intake_handler(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<IntakeRequest>,
) -> Result<Json<crate::services::onboarding::onboarding_agent::IntakeData>, axum::http::StatusCode> {
    match agent.process_intake(&payload.description).await {
        Ok(data) => Ok(Json(data)),
        Err(error) => {
            tracing::error!("onboarding intake agent error: {}", error);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_draft(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let tenant_id = auth_info.org_id.clone();
    let user_id = auth_info.agent_id.clone(); // In this context agent_id refers to the user

    let tid = if tenant_id.is_empty() { "default".to_string() } else { tenant_id };
    let uid = if user_id.is_empty() { "default".to_string() } else { user_id };

    match agent.get_onboarding_state(&tid, &uid).await {
        Ok(state) => Ok(Json(state)),
        Err(_) => Ok(Json(serde_json::json!({}))), // fallback
    }
}

async fn save_draft(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tenant_id = auth_info.org_id.clone();
    let user_id = auth_info.agent_id.clone();

    let tid = if tenant_id.is_empty() { "default".to_string() } else { tenant_id };
    let uid = if user_id.is_empty() { "default".to_string() } else { user_id };

    let step = payload.get("wizardState")
        .and_then(|w| w.get("step"))
        .or_else(|| payload.get("step"))
        .and_then(|s| s.as_i64())
        .unwrap_or(0) as i32;

    match agent.save_onboarding_state(&tid, &uid, step, &payload).await {
        Ok(_) => Ok(axum::http::StatusCode::OK),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
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

async fn launch_onboarding(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let tenant_id = auth_info.org_id.clone();
    let user_id = auth_info.agent_id.clone();
    let current_step = 5; // Launch step

    let tid = if tenant_id.is_empty() { "default".to_string() } else { tenant_id };
    let uid = if user_id.is_empty() { "default".to_string() } else { user_id };

    let state = serde_json::json!({
        "status": "launched"
    });
    match agent.save_onboarding_state(&tid, &uid, current_step, &state).await {
        Ok(_) => Ok(Json(state)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn get_state(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let tenant_id = auth_info.org_id.clone();
    let user_id = auth_info.agent_id.clone();

    // Support X- headers if auth_info is empty (for setup phase)
    let tid = if tenant_id.is_empty() { "default".to_string() } else { tenant_id };
    let uid = if user_id.is_empty() { "default".to_string() } else { user_id };

    match agent.get_onboarding_state(&tid, &uid).await {
        Ok(state) => Ok(Json(state)),
        Err(e) => {
            tracing::error!("Failed to get onboarding state: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

async fn save_state(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tenant_id = auth_info.org_id.clone();
    let user_id = auth_info.agent_id.clone();

    let tid = if tenant_id.is_empty() { "default".to_string() } else { tenant_id };
    let uid = if user_id.is_empty() { "default".to_string() } else { user_id };

    let step = payload.get("wizardState")
        .and_then(|w| w.get("step"))
        .or_else(|| payload.get("step"))
        .and_then(|s| s.as_i64())
        .unwrap_or(0) as i32;

    match agent.save_onboarding_state(&tid, &uid, step, &payload).await {
        Ok(_) => Ok(axum::http::StatusCode::NO_CONTENT),
        Err(e) => {
            tracing::error!("Failed to save onboarding state: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

#[derive(serde::Deserialize)]
pub struct ZeroClickRequest {
    pub description: String,
}

async fn zero_click_handler(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Json(payload): Json<ZeroClickRequest>,
) -> Result<Json<StartOnboardingResponse>, axum::http::StatusCode> {
    let _tenant_id = auth_info.org_id.clone();
    let _user_id = auth_info.agent_id.clone();

    // 1. Process Intake
    let intake_data = match agent.process_intake(&payload.description).await {
        Ok(data) => data,
        Err(error) => {
            tracing::error!("onboarding zero_click intake error: {}", error);
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // 2. Start Onboarding
    let start_req = StartOnboardingRequest {
        business_type: intake_data.business_type.clone(),
        company_name: intake_data.business_name.clone(),
        company_description: payload.description.clone(),
        selling_categories: intake_data.categories.clone(),
        payment_pref: "online".to_string(),
        admin_email: "admin@example.com".to_string(),
        admin_name: "Admin".to_string(),
        admin_password: "password123".to_string(),
        website_template: "Modern".to_string(),
        first_product_name: intake_data.initial_products.first().map(|p| p.name.clone()).unwrap_or_default(),
        first_product_price: intake_data.initial_products.first().map(|p| p.price.clone()).unwrap_or_default(),
        domain_choice: "subdomain".to_string(),
        price_type: "fixed".to_string(),
        location: intake_data.location.clone().unwrap_or_default(),
        target_audience: intake_data.target_audience.clone().unwrap_or_default(),
        initial_products: intake_data.initial_products.into_iter().map(|p| {
            ::server_ohc::orchestration::IntakeProductProto {
                name: p.name,
                price: p.price,
                description: p.description.unwrap_or_default(),
                variants: p.variants.unwrap_or_default().into_iter().map(|v| {
                    ::server_ohc::orchestration::IntakeProductVariantProto {
                        name: v.name,
                        price_modifier: v.price_modifier,
                    }
                }).collect(),
            }
        }).collect(),
        ai_agents: vec![],
        ai_auto_respond: true,
    };

    match agent.start_onboarding(start_req).await {
        Ok(res) => Ok(Json(res)),
        Err(error) => {
            tracing::error!("onboarding zero_click start error: {}", error);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
