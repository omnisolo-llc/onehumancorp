use crate::services::onboarding::onboarding_agent::OnboardingAgent;
use ::server_ohc::orchestration::{StartOnboardingRequest, StartOnboardingResponse};
use axum::{
    Router,
    extract::{Extension, Json, State},
    routing::{get, post},
};
use std::sync::Arc;

const MAX_ONBOARDING_INPUT_CHARS: usize = 4_000;
const MAX_ONBOARDING_IMAGE_URL_CHARS: usize = 2_048;
const MAX_ONBOARDING_CHAT_MESSAGES: usize = 20;
const MAX_ONBOARDING_CHAT_CHARS: usize = 12_000;

fn valid_required_text(value: &str, max_chars: usize) -> bool {
    !value.trim().is_empty() && value.chars().count() <= max_chars
}

fn valid_optional_url(value: Option<&str>) -> bool {
    value.is_none_or(|url| {
        let url = url.trim();
        !url.is_empty() && url.chars().count() <= MAX_ONBOARDING_IMAGE_URL_CHARS
    })
}

pub fn router(
    agent: Arc<OnboardingAgent>,
    auth_store: Arc<::server_auth::Store>,
) -> Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    let r = Router::new()
        .route("/start", post(start_onboarding))
        .route("/start_zero_click", post(start_zero_click))
        .route("/intake", post(process_intake_handler))
        .route("/chat", post(process_chat_handler))
        .route("/state", get(get_state).post(save_state))
        .route("/launch", post(launch_onboarding))
        .route("/draft", get(get_draft).post(save_draft))
        .route("/setup-health", get(setup_health_check))
        .layer(axum::middleware::from_fn(require_onboarding_admin))
        .layer(axum::middleware::from_fn_with_state(
            auth_store,
            ::server_auth::strict_bearer_auth_middleware,
        ))
        .with_state(agent.clone());

    let gateway_r = Router::new()
        .route("/api/v1/gateway/run", post(gateway_run_handler))
        .layer(axum::middleware::from_fn(::server_auth::api_key_auth_middleware))
        .with_state(agent);

    // Convert to accept MeshTransport state
    Router::new().merge(r).merge(gateway_r)
}

fn is_onboarding_admin(claims: &::server_common::Claims) -> bool {
    claims
        .roles
        .iter()
        .any(|role| role.eq_ignore_ascii_case("admin") || role.eq_ignore_ascii_case("owner"))
}

async fn require_onboarding_admin(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    use axum::response::IntoResponse;

    if req
        .extensions()
        .get::<::server_common::Claims>()
        .is_some_and(is_onboarding_admin)
    {
        next.run(req).await
    } else {
        axum::http::StatusCode::FORBIDDEN.into_response()
    }
}

fn onboarding_identity(
    auth_info: &::server_auth::orchestration::AuthInfo,
) -> Result<(String, String), axum::http::StatusCode> {
    let tenant_id = auth_info.org_id.trim();
    let user_id = auth_info.agent_id.trim();
    if tenant_id.is_empty() || user_id.is_empty() {
        Err(axum::http::StatusCode::UNAUTHORIZED)
    } else {
        Ok((tenant_id.to_string(), user_id.to_string()))
    }
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
        Err(e) => {
            tracing::info!(
                "Health check failed for environment (cloud: {}): {}. Attempting to provision...",
                is_cloud,
                e
            ); // pii-safe
            match crate::services::onboarding::provisioner::provision_environment(is_cloud) {
                Ok(_) => {
                    tracing::info!("Successfully provisioned environment (cloud: {})", is_cloud);
                    Ok(Json(serde_json::json!({ "status": "ready" })))
                }
                Err(provision_err) => {
                    tracing::error!(
                        "Failed to provision environment (cloud: {}): {}",
                        is_cloud,
                        provision_err
                    );
                    Ok(Json(
                        serde_json::json!({ "status": "error", "message": provision_err }),
                    ))
                }
            }
        }
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

fn validate_chat_request(request: &ChatRequest) -> bool {
    if request.messages.is_empty() || request.messages.len() > MAX_ONBOARDING_CHAT_MESSAGES {
        return false;
    }

    let mut total_chars = 0usize;
    let mut has_user_input = false;
    let valid = request.messages.iter().all(|message| {
        let content_chars = message.content.chars().count();
        total_chars = total_chars.saturating_add(content_chars);
        if message.role == "user"
            && (!message.content.trim().is_empty() || message.image_url.is_some())
        {
            has_user_input = true;
        }
        matches!(message.role.as_str(), "user" | "assistant")
            && content_chars <= MAX_ONBOARDING_INPUT_CHARS
            && total_chars <= MAX_ONBOARDING_CHAT_CHARS
            && valid_optional_url(message.image_url.as_deref())
    });
    valid && has_user_input
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthenticatedStartOnboardingRequest {
    business_type: String,
    company_name: String,
    company_description: String,
    selling_categories: Vec<String>,
    payment_pref: String,
    website_template: String,
    first_product_name: String,
    first_product_price: String,
    domain_choice: String,
    price_type: String,
    location: String,
    target_audience: String,
    #[serde(default)]
    initial_products: Vec<::server_ohc::orchestration::IntakeProductProto>,
    #[serde(default)]
    ai_agents: Vec<String>,
    #[serde(default)]
    ai_auto_respond: bool,
    deposit_percentage: Option<i32>,
    lead_time_days: Option<i32>,
}

impl From<AuthenticatedStartOnboardingRequest> for StartOnboardingRequest {
    fn from(request: AuthenticatedStartOnboardingRequest) -> Self {
        Self {
            business_type: request.business_type,
            company_name: request.company_name,
            company_description: request.company_description,
            selling_categories: request.selling_categories,
            payment_pref: request.payment_pref,
            website_template: request.website_template,
            first_product_name: request.first_product_name,
            first_product_price: request.first_product_price,
            domain_choice: request.domain_choice,
            price_type: request.price_type,
            location: request.location,
            target_audience: request.target_audience,
            initial_products: request.initial_products,
            ai_agents: request.ai_agents,
            ai_auto_respond: request.ai_auto_respond,
            deposit_percentage: request.deposit_percentage,
            lead_time_days: request.lead_time_days,
        }
    }
}

fn validate_start_request(request: &AuthenticatedStartOnboardingRequest) -> bool {
    let required_fields = [
        request.business_type.as_str(),
        request.company_name.as_str(),
        request.company_description.as_str(),
        request.payment_pref.as_str(),
        request.website_template.as_str(),
        request.domain_choice.as_str(),
        request.price_type.as_str(),
        request.location.as_str(),
        request.target_audience.as_str(),
    ];
    if required_fields
        .iter()
        .any(|value| !valid_required_text(value, MAX_ONBOARDING_INPUT_CHARS))
        || request.selling_categories.len() > 50
        || request.ai_agents.len() > 20
        || request.initial_products.len() > 50
        || request.first_product_name.chars().count() > 200
        || request.first_product_price.chars().count() > 50
        || request
            .deposit_percentage
            .is_some_and(|percentage| !(0..=100).contains(&percentage))
        || request
            .lead_time_days
            .is_some_and(|days| !(0..=3_650).contains(&days))
    {
        return false;
    }

    request
        .selling_categories
        .iter()
        .chain(request.ai_agents.iter())
        .all(|value| valid_required_text(value, 200))
        && request.initial_products.iter().all(|product| {
            valid_required_text(&product.name, 200)
                && product.price.chars().count() <= 50
                && product.description.chars().count() <= MAX_ONBOARDING_INPUT_CHARS
                && product.variants.len() <= 50
                && product.variants.iter().all(|variant| {
                    valid_required_text(&variant.name, 200)
                        && variant.price_modifier.chars().count() <= 50
                })
        })
}

async fn process_intake_handler(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<IntakeRequest>,
) -> Result<Json<crate::services::onboarding::onboarding_agent::IntakeData>, axum::http::StatusCode>
{
    if !valid_required_text(&payload.description, MAX_ONBOARDING_INPUT_CHARS)
        || !valid_optional_url(payload.image_url.as_deref())
    {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }
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
) -> Result<Json<crate::services::onboarding::onboarding_agent::ChatResponse>, axum::http::StatusCode>
{
    if !validate_chat_request(&payload) {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }
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
    let (tenant_id, user_id) = onboarding_identity(&auth_info)?;

    match agent.get_onboarding_state(&tenant_id, &user_id).await {
        Ok(state) => Ok(Json(state)),
        Err(error) => {
            tracing::error!("Failed to get onboarding draft: {}", error);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn save_draft(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let (tenant_id, user_id) = onboarding_identity(&auth_info)?;

    let step = payload.get("step").and_then(|s| s.as_i64()).unwrap_or(0) as i32;

    match agent
        .save_onboarding_state(&tenant_id, &user_id, step, &payload)
        .await
    {
        Ok(_) => Ok(axum::http::StatusCode::OK),
        Err(e) => {
            tracing::error!("Failed to save onboarding draft: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn start_onboarding(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Json(payload): Json<AuthenticatedStartOnboardingRequest>,
) -> Result<Json<StartOnboardingResponse>, axum::http::StatusCode> {
    if !validate_start_request(&payload) {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }
    let (organization_id, user_id) = onboarding_identity(&auth_info)?;
    match agent
        .start_onboarding_for_identity(payload.into(), &organization_id, &user_id)
        .await
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => {
            tracing::error!("Failed to start onboarding: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
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
    if !valid_required_text(&req.prompt, MAX_ONBOARDING_INPUT_CHARS)
        || !valid_optional_url(req.image_url.as_deref())
    {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }
    let mut combined_prompt = req.prompt.clone();
    if let Some(image_url) = &req.image_url {
        combined_prompt.push_str(&format!("\nImage provided: {}", image_url));
    }

    let intake_data = agent.process_intake(&combined_prompt).await.map_err(|e| {
        tracing::error!("Intake error: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let first_product = intake_data.initial_products.first();
    let first_product_name = first_product
        .map(|p| p.name.clone())
        .unwrap_or_else(|| "Standard Product".to_string());
    let first_product_price = first_product
        .map(|p| p.price.clone())
        .unwrap_or_else(|| "10.00".to_string());

    let start_req = ::server_ohc::orchestration::StartOnboardingRequest {
        business_type: if intake_data.business_type.is_empty() {
            "Other".to_string()
        } else {
            intake_data.business_type
        },
        company_name: if intake_data.business_name.is_empty() {
            "My Store".to_string()
        } else {
            intake_data.business_name.clone()
        },
        company_description: req.prompt.clone(),
        selling_categories: if intake_data.categories.is_empty() {
            vec!["Other".to_string()]
        } else {
            intake_data.categories
        },
        payment_pref: "online".to_string(),
        website_template: "Modern".to_string(),
        first_product_name,
        first_product_price,
        domain_choice: "subdomain".to_string(),
        price_type: "fixed".to_string(),
        location: intake_data.location.unwrap_or_else(|| "Global".to_string()),
        target_audience: intake_data
            .target_audience
            .unwrap_or_else(|| "Everyone".to_string()),
        initial_products: intake_data
            .initial_products
            .into_iter()
            .map(|p| ::server_ohc::orchestration::IntakeProductProto {
                name: p.name,
                price: p.price,
                description: p.description.unwrap_or_default(),
                variants: p
                    .variants
                    .unwrap_or_default()
                    .into_iter()
                    .map(|v| ::server_ohc::orchestration::IntakeProductVariantProto {
                        name: v.name,
                        price_modifier: v.price_modifier,
                    })
                    .collect(),
            })
            .collect(),
        ai_agents: vec![],
        ai_auto_respond: false,
        deposit_percentage: intake_data.deposit_percentage,
        lead_time_days: intake_data.lead_time_days,
    };

    let (organization_id, user_id) = onboarding_identity(&auth_info)?;
    let start_res = agent
        .start_onboarding_for_identity(start_req, &organization_id, &user_id)
        .await
        .map_err(|e| {
            tracing::error!("Start onboarding error: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ZeroClickGenerateResponse {
        organization_id: start_res.organization_id,
        user_id: start_res.user_id,
        message: "Storefront generated successfully".to_string(),
    }))
}

pub async fn gateway_run_handler(
    state: State<Arc<OnboardingAgent>>,
    extension: Extension<::server_auth::orchestration::AuthInfo>,
    json: Json<ZeroClickGenerateRequest>,
) -> Result<Json<ZeroClickGenerateResponse>, axum::http::StatusCode> {
    // Record usage for gateway execution (simulating token usage and cost for zero-click generation)
    // Values are hardcoded or estimated for now
    let auth_info = &extension.0;
    ::server_auth::record_usage(
        &auth_info.agent_id,
        &auth_info.org_id,
        "gateway_run",
        15200,
        0.0304,
    )
    .await;

    start_zero_click(state, extension, json).await
}

async fn launch_onboarding(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let (tenant_id, user_id) = onboarding_identity(&auth_info)?;
    let current_step = 5; // Launch step

    let state = serde_json::json!({
        "status": "launched"
    });
    match agent
        .save_onboarding_system_state(&tenant_id, &user_id, current_step, &state)
        .await
    {
        Ok(_) => Ok(Json(state)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_state(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let (tenant_id, user_id) = onboarding_identity(&auth_info)?;

    match agent.get_onboarding_state(&tenant_id, &user_id).await {
        Ok(state) => Ok(Json(state)),
        Err(e) => {
            tracing::error!("Failed to get onboarding state: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn save_state(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let (tenant_id, user_id) = onboarding_identity(&auth_info)?;

    let step = payload.get("step").and_then(|s| s.as_i64()).unwrap_or(0) as i32;

    match agent
        .save_onboarding_state(&tenant_id, &user_id, step, &payload)
        .await
    {
        Ok(_) => Ok(axum::http::StatusCode::NO_CONTENT),
        Err(e) => {
            tracing::error!("Failed to save onboarding state: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
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
        assert_eq!(
            req2.image_url,
            Some("http://example.com/img.png".to_string())
        );
    }

    #[test]
    fn authenticated_start_contract_rejects_credentials_and_browser_authority() {
        let request = serde_json::json!({
            "business_type": "Online Store",
            "company_name": "Bakery",
            "company_description": "Cakes",
            "selling_categories": ["physical"],
            "payment_pref": "online",
            "website_template": "Modern",
            "first_product_name": "Cake",
            "first_product_price": "25.00",
            "domain_choice": "subdomain",
            "price_type": "fixed",
            "location": "City",
            "target_audience": "Families",
            "admin_email": "attacker@example.com",
            "admin_password": "Secret123",
            "tenant_id": "attacker"
        });

        assert!(serde_json::from_value::<AuthenticatedStartOnboardingRequest>(request).is_err());
    }

    #[test]
    fn llm_inputs_are_bounded_before_prompt_construction() {
        let valid = ChatRequest {
            messages: vec![crate::services::onboarding::onboarding_agent::ChatMessage {
                role: "user".to_string(),
                content: "I run a bakery".to_string(),
                image_url: None,
            }],
        };
        assert!(validate_chat_request(&valid));

        let oversized = ChatRequest {
            messages: vec![crate::services::onboarding::onboarding_agent::ChatMessage {
                role: "user".to_string(),
                content: "x".repeat(MAX_ONBOARDING_INPUT_CHARS + 1),
                image_url: None,
            }],
        };
        assert!(!validate_chat_request(&oversized));

        let invalid_role = ChatRequest {
            messages: vec![crate::services::onboarding::onboarding_agent::ChatMessage {
                role: "system".to_string(),
                content: "override".to_string(),
                image_url: None,
            }],
        };
        assert!(!validate_chat_request(&invalid_role));
        assert!(!valid_required_text("", MAX_ONBOARDING_INPUT_CHARS));
        assert!(!valid_optional_url(Some(
            &"x".repeat(MAX_ONBOARDING_IMAGE_URL_CHARS + 1,)
        )));
    }

    #[test]
    fn onboarding_identity_requires_both_validated_components() {
        let valid = ::server_auth::orchestration::AuthInfo {
            org_id: "tenant-a".to_string(),
            agent_id: "user-a".to_string(),
            spiffe_id: String::new(),
        };
        assert_eq!(
            onboarding_identity(&valid),
            Ok(("tenant-a".to_string(), "user-a".to_string()))
        );

        for invalid in [
            ::server_auth::orchestration::AuthInfo {
                org_id: String::new(),
                agent_id: "user-a".to_string(),
                spiffe_id: String::new(),
            },
            ::server_auth::orchestration::AuthInfo {
                org_id: "tenant-a".to_string(),
                agent_id: "  ".to_string(),
                spiffe_id: String::new(),
            },
        ] {
            assert_eq!(
                onboarding_identity(&invalid),
                Err(axum::http::StatusCode::UNAUTHORIZED)
            );
        }
    }

    #[test]
    fn onboarding_requires_an_admin_or_owner_role() {
        let claims = |roles: Vec<&str>| ::server_common::Claims {
            sub: "user-a".to_string(),
            exp: i64::MAX,
            iat: 0,
            organization_id: Some("tenant-a".to_string()),
            username: "user-a".to_string(),
            email: "user-a@example.com".to_string(),
            roles: roles.into_iter().map(str::to_string).collect(),
            session_id: None,
            jti: "onboarding-role-test".to_string(),
        };

        assert!(is_onboarding_admin(&claims(vec!["ADMIN"])));
        assert!(is_onboarding_admin(&claims(vec!["owner"])));
        assert!(!is_onboarding_admin(&claims(vec!["VIEWER"])));
        assert!(!is_onboarding_admin(&claims(vec![])));
    }

    #[tokio::test]
    async fn onboarding_router_rejects_forged_headers_and_non_admin_tokens() {
        use axum::{body::Body, http::Request};
        use tower::ServiceExt;

        let pool = sqlx::PgPool::connect_lazy("postgres://localhost/unused").unwrap();
        let db = Arc::new(crate::db::DB {
            pool: pool.clone(),
            store: crate::db::DbStore::Postgres,
        });
        let (event_tx, _) = tokio::sync::mpsc::channel(1);
        let hub = Arc::new(crate::hub::Hub::new(event_tx, pool));
        let agent = Arc::new(OnboardingAgent::new(db, hub));
        let auth_store = Arc::new(::server_auth::Store::new());
        let now = chrono::Utc::now();
        let token = auth_store
            .issue_token(&::server_auth::User {
                id: "viewer-a".to_string(),
                username: "viewer-a".to_string(),
                email: "viewer-a@example.com".to_string(),
                password_hash: String::new(),
                roles: vec!["VIEWER".to_string()],
                active: true,
                organization_id: Some("tenant-a".to_string()),
                created_at: now,
                updated_at: now,
                oidc_subject: None,
            })
            .unwrap();
        let transport: Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport> =
            Arc::new(ohc_builtin_agent::mesh::transport::InProcessTransport::new());
        let app = router(agent, auth_store).with_state(transport);

        let forged = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/state")
                    .header("x-tenant-id", "attacker")
                    .header("x-user-id", "attacker")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(forged.status(), axum::http::StatusCode::UNAUTHORIZED);

        let viewer = app
            .oneshot(
                Request::builder()
                    .uri("/state")
                    .header("authorization", format!("Bearer {token}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(viewer.status(), axum::http::StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_gateway_run_auth_and_execution() {
        use axum::{body::Body, http::Request};
        use tower::ServiceExt;
        use sha2::Digest;

        let pool = sqlx::PgPool::connect_lazy("postgres://localhost/unused").unwrap();
        let db = Arc::new(crate::db::DB {
            pool: pool.clone(),
            store: crate::db::DbStore::Postgres,
        });
        let (event_tx, _) = tokio::sync::mpsc::channel(1);
        let hub = Arc::new(crate::hub::Hub::new(event_tx, pool));
        let agent = Arc::new(OnboardingAgent::new(db, hub));
        let auth_store = Arc::new(::server_auth::Store::new());

        let transport: Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport> =
            Arc::new(ohc_builtin_agent::mesh::transport::InProcessTransport::new());
        let app = router(agent, auth_store).with_state(transport);

        // 1. Request without key is rejected with 401
        let no_key_req = Request::builder()
            .method("POST")
            .uri("/api/v1/gateway/run")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"prompt":"baker"}"#))
            .unwrap();
        let response = app.clone().oneshot(no_key_req).await.unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);

        // 2. Request with invalid key is rejected with 401
        let invalid_key_req = Request::builder()
            .method("POST")
            .uri("/api/v1/gateway/run")
            .header("authorization", "Bearer ohc_gwy_invalidkey")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"prompt":"baker"}"#))
            .unwrap();
        let response = app.clone().oneshot(invalid_key_req).await.unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);

        // 3. Request with valid key passes authentication
        let raw_key = "ohc_gwy_test_gateway_key_123";
        let key_hash = format!("{:x}", sha2::Sha256::digest(raw_key.as_bytes()));

        {
            let mut keys = ::server_auth::http::get_in_memory_keys().lock().unwrap();
            keys.push(::server_auth::http::InMemoryApiKey {
                id: "key-test-1".to_string(),
                key_hash,
                name: "Test Gateway Key".to_string(),
                member_id: "user-a".to_string(),
                organization_id: "tenant-a".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
            });
        }

        let valid_key_req = Request::builder()
            .method("POST")
            .uri("/api/v1/gateway/run")
            .header("authorization", format!("Bearer {raw_key}"))
            .header("content-type", "application/json")
            .body(Body::from(r#"{"prompt":"baker"}"#))
            .unwrap();
        let response = app.oneshot(valid_key_req).await.unwrap();

        // Since no real database is set up or seeded, this may return 500 Internal Server Error,
        // but getting past 401 proves the api_key_auth_middleware successfully authenticated the key.
        let status = response.status();
        assert!(
            status == axum::http::StatusCode::OK || status == axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Expected status OK or INTERNAL_SERVER_ERROR, got {:?}",
            status
        );
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;

    #[test]
    fn test_zero_click_generate_response_serialization() {
        let resp = ZeroClickGenerateResponse {
            organization_id: "org_123".to_string(),
            user_id: "user_456".to_string(),
            message: "Success".to_string(),
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("org_123"));
        assert!(json.contains("user_456"));
        assert!(json.contains("Success"));
    }
}
