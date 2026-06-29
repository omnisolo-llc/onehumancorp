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
        .route("/start_zero_click", post(start_zero_click))
        .route("/intake", post(process_intake_handler))
        .route("/chat", post(process_chat_handler))
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
#[derive(serde::Deserialize)]
pub struct ZeroClickGenerateRequest {
    pub prompt: String,
    #[serde(default)]
    pub image_url: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct ZeroClickGenerateResponse {
    pub organization_id: String,
    pub user_id: String,
    pub message: String,
}

async fn start_zero_click(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Json(req): Json<ZeroClickGenerateRequest>,
) -> Result<Json<ZeroClickGenerateResponse>, axum::http::StatusCode> {
    let mut combined_prompt = req.prompt.clone();
    if let Some(image_url) = &req.image_url {
        combined_prompt.push_str(&format!("\nImage provided: {}", image_url));
    }

    let intake_data = agent.process_intake(&combined_prompt).await.map_err(|e| {
        tracing::error!("Intake error: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let first_product = intake_data.initial_products.first();
    let first_product_name = first_product.map(|p| p.name.clone()).unwrap_or_else(|| "Standard Product".to_string());
    let first_product_price = first_product.map(|p| p.price.clone()).unwrap_or_else(|| "10.00".to_string());

    let start_req = ::server_ohc::orchestration::StartOnboardingRequest {
        business_type: if intake_data.business_type.is_empty() { "Other".to_string() } else { intake_data.business_type },
        company_name: if intake_data.business_name.is_empty() { "My Store".to_string() } else { intake_data.business_name.clone() },
        company_description: req.prompt.clone(),
        selling_categories: if intake_data.categories.is_empty() { vec!["Other".to_string()] } else { intake_data.categories },
        payment_pref: "online".to_string(),
        admin_email: if !auth_info.agent_id.is_empty() { auth_info.agent_id.clone() } else { format!("owner_{}@ohc.app", uuid::Uuid::new_v4().simple()) },
        admin_name: "Owner".to_string(),
        admin_password: format!("{}!", uuid::Uuid::new_v4().to_string()),
        website_template: "Modern".to_string(),
        first_product_name,
        first_product_price,
        domain_choice: "subdomain".to_string(),
        price_type: "fixed".to_string(),
        location: intake_data.location.unwrap_or_else(|| "Global".to_string()),
        target_audience: intake_data.target_audience.unwrap_or_else(|| "Everyone".to_string()),
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
        ai_auto_respond: false, deposit_percentage: intake_data.deposit_percentage, lead_time_days: intake_data.lead_time_days,
    };

    let start_res = agent.start_onboarding(start_req).await.map_err(|e| {
        tracing::error!("Start onboarding error: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(ZeroClickGenerateResponse {
        organization_id: start_res.organization_id,
        user_id: start_res.user_id,
        message: "Storefront generated successfully".to_string()
    }))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_zero_click_generate_request_deserialization() {
        let json = r#"{"prompt": "I am a baker"}"#;
        let req: ZeroClickGenerateRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.prompt, "I am a baker");
        assert_eq!(req.image_url, None);

        let json2 = r#"{"prompt": "I am a baker", "image_url": "http://example.com/img.png"}"#;
        let req2: ZeroClickGenerateRequest = serde_json::from_str(json2).unwrap();
        assert_eq!(req2.prompt, "I am a baker");
        assert_eq!(req2.image_url, Some("http://example.com/img.png".to_string()));
    }
}
