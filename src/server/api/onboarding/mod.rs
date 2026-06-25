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
        .route("/generate", post(handle_zero_click_generate))
        .route("/start", post(start_onboarding))
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

#[derive(Debug, serde::Deserialize)]
pub struct ZeroClickGenerateRequest {
    pub prompt: String,
}
#[derive(Debug, serde::Serialize)]
pub struct ZeroClickGenerateResponse {
    pub organization_id: String,
    pub user_id: String,
    pub message: String,
}

pub async fn handle_zero_click_generate(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Json(req): Json<ZeroClickGenerateRequest>,
) -> Result<Json<ZeroClickGenerateResponse>, axum::http::StatusCode> {
    let intake_data = agent.process_intake(&req.prompt).await.map_err(|e| {
        tracing::error!("Intake error: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let first_product = intake_data.initial_products.first();
    let first_product_name = first_product.map(|p| p.name.clone()).unwrap_or_else(|| "Standard Product".to_string());
    let first_product_price = first_product.map(|p| p.price.clone()).unwrap_or_else(|| "10.00".to_string());

    let start_req = ::server_ohc::orchestration::StartOnboardingRequest {
        business_type: intake_data.business_type,
        company_name: intake_data.business_name.clone(),
        company_description: req.prompt.clone(),
        selling_categories: intake_data.categories,
        payment_pref: "online".to_string(),
        admin_email: if !auth_info.agent_id.is_empty() { auth_info.agent_id.clone() } else { format!("owner_{}@ohc.app", uuid::Uuid::new_v4().simple()) },
        admin_name: "Owner".to_string(),
        admin_password: uuid::Uuid::new_v4().to_string(),
        website_template: "Modern".to_string(),
        first_product_name,
        first_product_price,
        domain_choice: "subdomain".to_string(),
        price_type: "fixed".to_string(),
        location: intake_data.location.unwrap_or_else(|| "Global".to_string()),
        target_audience: intake_data.target_audience.unwrap_or_else(|| "Everyone".to_string()),
        ai_agents: vec![],
        ai_auto_respond: true,
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
    };

    let _start_res = agent.start_onboarding(start_req).await.map_err(|e| {
        tracing::error!("Start onboarding error: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(ZeroClickGenerateResponse {
        organization_id: _start_res.organization_id,
        user_id: _start_res.user_id,
        message: "Storefront generated successfully".to_string()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DbStore;

    async fn setup_db() -> sqlx::PgPool {
        let database_url = std::env::var("OHC_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        crate::db::secure_pg_pool_options()
            .acquire_timeout(std::time::Duration::from_millis(500))
            .max_connections(1)
            .connect_lazy(&database_url)
            .expect("Failed to connect to DB")
    }

    #[tokio::test]
    async fn test_handle_zero_click_generate() {
        let pool = setup_db().await;
        let (tx, _) = tokio::sync::mpsc::channel(10);
        let hub = Arc::new(crate::hub::Hub::new(tx, pool.clone()));
        let db = Arc::new(crate::db::DB {
            pool: pool.clone(),
            store: DbStore::Postgres,
        });
        let agent = Arc::new(crate::services::onboarding::onboarding_agent::OnboardingAgent::new(db, hub));

        let req = ZeroClickGenerateRequest {
            prompt: "I sell coffee".to_string(),
        };

        let auth_info = ::server_auth::orchestration::AuthInfo {
            spiffe_id: format!("spiffe://ohc.app/{}/agent1", "test-tenant-zero"),
            org_id: "test-tenant-zero".to_string(),
            agent_id: "owner@test.com".to_string(),
        };
        let _ = handle_zero_click_generate(State(agent), axum::extract::Extension(auth_info.clone()), Json(req)).await;
    }

    #[tokio::test]
    async fn test_zero_click_generate() {
        let pool = setup_db().await;
        if sqlx::query("SELECT 1").execute(&pool).await.is_err() {
            return;
        }
        let (event_tx, _) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(crate::hub::Hub::new(event_tx, pool.clone()));
        let db = Arc::new(crate::db::DB {
            pool: pool.clone(),
            store: DbStore::Postgres,
        });
        let agent = Arc::new(crate::services::onboarding::onboarding_agent::OnboardingAgent::new(db, hub));

        let req = ZeroClickGenerateRequest {
            prompt: "I am a home baker selling cakes.".to_string(),
        };

        let auth_info = ::server_auth::orchestration::AuthInfo {
            spiffe_id: format!("spiffe://ohc.app/{}/agent1", "test-tenant-zero"),
            org_id: "test-tenant-zero".to_string(),
            agent_id: "owner@test.com".to_string(),
        };

        let res = handle_zero_click_generate(State(agent), axum::extract::Extension(auth_info.clone()), Json(req)).await;

        assert!(res.is_ok());
        let response = res.unwrap().0;
        assert!(!response.organization_id.is_empty());
        assert!(!response.user_id.is_empty());
    }
}
