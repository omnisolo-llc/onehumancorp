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
        .route("/intake", post(process_intake_handler))
        .route("/chat", post(process_chat_handler))
        .route("/zero-click-intake", post(process_zero_click_intake_handler))
        .route("/state", get(get_state).post(save_state))
        .route("/launch", post(launch_onboarding))
        .route("/draft", get(get_draft).post(save_draft))
        .route("/setup-health", get(setup_health_check))
        .layer(axum::middleware::from_fn(::server_auth::guest_auth_middleware))
        .with_state(agent);

    // Convert to accept MeshTransport state
    Router::new().merge(r)
}

#[derive(serde::Deserialize)]
pub struct HealthCheckQuery {
    pub mode: Option<String>,
}

async fn setup_health_check(
    axum::extract::Query(query): axum::extract::Query<HealthCheckQuery>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let is_cloud = query.mode.as_deref() == Some("cloud");
    match crate::services::onboarding::provisioner::check_environment(is_cloud) {
        Ok(_) => Ok(Json(serde_json::json!({ "status": "ready" }))),
        Err(e) => Ok(Json(serde_json::json!({ "status": "error", "message": e }))),
    }
}

#[derive(serde::Deserialize)]
pub struct IntakeRequest {
    pub description: String,
    pub image_url: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<crate::services::onboarding::onboarding_agent::ChatMessage>,
}

async fn process_zero_click_intake_handler(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<IntakeRequest>,
) -> Result<Json<StartOnboardingResponse>, axum::http::StatusCode> {
    let mut combined_input = payload.description.clone();
    if let Some(image_url) = &payload.image_url {
        combined_input.push_str(&format!("\nImage provided: {}", image_url));
    }
    match agent.process_intake(&combined_input).await {
        Ok(data) => {
            let req = StartOnboardingRequest {
                business_type: data.business_type.clone(),
                company_name: data.business_name.clone(),
                company_description: combined_input,
                selling_categories: data.categories.clone(),
                payment_pref: "online".to_string(),
                admin_email: data.sample_customer_email.unwrap_or_else(|| "admin@example.com".to_string()),
                admin_name: data.sample_customer_name.unwrap_or_else(|| "Admin User".to_string()),
                admin_password: "password123".to_string(),
                website_template: "Modern".to_string(),
                first_product_name: data.initial_products.get(0).map(|p| p.name.clone()).unwrap_or_else(|| "Product".to_string()),
                first_product_price: data.initial_products.get(0).map(|p| p.price.clone()).unwrap_or_else(|| "10.00".to_string()),
                domain_choice: "subdomain".to_string(),
                price_type: "fixed".to_string(),
                location: data.location.unwrap_or_else(|| "Unknown".to_string()),
                target_audience: data.target_audience.unwrap_or_else(|| "Everyone".to_string()),
                ai_agents: vec!["onboarding".to_string(), "operations".to_string()],
                ai_auto_respond: true,
                initial_products: data.initial_products.into_iter().map(|p| {
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
            };
            match agent.start_onboarding(req).await {
                Ok(res) => Ok(Json(res)),
                Err(e) => {
                    tracing::error!("zero click start_onboarding error: {}", e);
                    Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        },
        Err(error) => {
            tracing::error!("zero click intake agent error: {}", error);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn process_intake_handler(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<IntakeRequest>,
) -> Result<Json<crate::services::onboarding::onboarding_agent::IntakeData>, axum::http::StatusCode> {
    let mut combined_input = payload.description.clone();
    if let Some(image_url) = &payload.image_url {
        combined_input.push_str(&format!("\nImage provided: {}", image_url));
    }
    match agent.process_intake(&combined_input).await {
        Ok(data) => Ok(Json(data)),
        Err(error) => {
            tracing::error!("onboarding intake agent error: {}", error);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn process_chat_handler(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<ChatRequest>,
) -> Result<Json<crate::services::onboarding::onboarding_agent::ChatResponse>, axum::http::StatusCode> {
    match agent.process_chat(payload.messages).await {
        Ok(data) => Ok(Json(data)),
        Err(error) => {
            tracing::error!("onboarding chat agent error: {}", error);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_draft(
    headers: axum::http::HeaderMap,
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let tenant_id = auth_info.org_id.clone();
    let user_id = auth_info.agent_id.clone(); // In this context agent_id refers to the user

    let mut tid = tenant_id;
    let mut uid = user_id;

    if tid.is_empty() {
        if let Some(val) = headers.get("X-Tenant-ID") {
            if let Ok(val_str) = val.to_str() {
                tid = val_str.to_string();
            }
        }
    }
    if uid.is_empty() {
        if let Some(val) = headers.get("X-User-ID") {
            if let Ok(val_str) = val.to_str() {
                uid = val_str.to_string();
            }
        }
    }

    let tid = if tid.is_empty() { "default".to_string() } else { tid };
    let uid = if uid.is_empty() { "default".to_string() } else { uid };

    match agent.get_onboarding_state(&tid, &uid).await {
        Ok(state) => Ok(Json(state)),
        Err(_) => Ok(Json(serde_json::json!({}))), // fallback
    }
}

async fn save_draft(
    headers: axum::http::HeaderMap,
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tenant_id = auth_info.org_id.clone();
    let user_id = auth_info.agent_id.clone();

    let mut tid = tenant_id;
    let mut uid = user_id;

    if tid.is_empty() {
        if let Some(val) = headers.get("X-Tenant-ID") {
            if let Ok(val_str) = val.to_str() {
                tid = val_str.to_string();
            }
        }
    }
    if uid.is_empty() {
        if let Some(val) = headers.get("X-User-ID") {
            if let Ok(val_str) = val.to_str() {
                uid = val_str.to_string();
            }
        }
    }

    let tid = if tid.is_empty() { "default".to_string() } else { tid };
    let uid = if uid.is_empty() { "default".to_string() } else { uid };

    let step = payload.get("step")
        .and_then(|s| s.as_i64())
        .unwrap_or(0) as i32;

    match agent.save_onboarding_state(&tid, &uid, step, &payload).await {
        Ok(_) => Ok(axum::http::StatusCode::OK),
        Err(e) => {
            tracing::error!("Failed to save onboarding draft: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

async fn start_onboarding(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<StartOnboardingRequest>,
) -> Result<Json<StartOnboardingResponse>, axum::http::StatusCode> {
    match agent.start_onboarding(payload).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => {
            tracing::error!("Failed to start onboarding: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        },
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
    headers: axum::http::HeaderMap,
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let tenant_id = auth_info.org_id.clone();
    let user_id = auth_info.agent_id.clone();

    // Support X- headers if auth_info is empty (for setup phase)
    let mut tid = tenant_id;
    let mut uid = user_id;

    if tid.is_empty() {
        if let Some(val) = headers.get("X-Tenant-ID") {
            if let Ok(val_str) = val.to_str() {
                tid = val_str.to_string();
            }
        }
    }
    if uid.is_empty() {
        if let Some(val) = headers.get("X-User-ID") {
            if let Ok(val_str) = val.to_str() {
                uid = val_str.to_string();
            }
        }
    }

    let tid = if tid.is_empty() { "default".to_string() } else { tid };
    let uid = if uid.is_empty() { "default".to_string() } else { uid };

    match agent.get_onboarding_state(&tid, &uid).await {
        Ok(state) => Ok(Json(state)),
        Err(e) => {
            tracing::error!("Failed to get onboarding state: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

async fn save_state(
    headers: axum::http::HeaderMap,
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tenant_id = auth_info.org_id.clone();
    let user_id = auth_info.agent_id.clone();

    let mut tid = tenant_id;
    let mut uid = user_id;

    if tid.is_empty() {
        if let Some(val) = headers.get("X-Tenant-ID") {
            if let Ok(val_str) = val.to_str() {
                tid = val_str.to_string();
            }
        }
    }
    if uid.is_empty() {
        if let Some(val) = headers.get("X-User-ID") {
            if let Ok(val_str) = val.to_str() {
                uid = val_str.to_string();
            }
        }
    }

    let tid = if tid.is_empty() { "default".to_string() } else { tid };
    let uid = if uid.is_empty() { "default".to_string() } else { uid };

    let step = payload.get("step")
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
