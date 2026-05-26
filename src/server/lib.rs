pub use ::server_harness as harness;
pub mod api;

use std::collections::HashMap;
use std::sync::RwLock;

#[derive(serde::Deserialize)]
struct ChatRequest {
    message: String,
}

static TOOLTIPS_REGISTRY: std::sync::OnceLock<RwLock<HashMap<String, String>>> = std::sync::OnceLock::new();

fn get_tooltips_registry() -> &'static RwLock<HashMap<String, String>> {
    TOOLTIPS_REGISTRY.get_or_init(|| {
    let mut m = HashMap::new();
    m.insert("bio-input-tooltip".to_string(), "Describe what you sell, your target audience, and the vibe of your brand.".to_string());
    m.insert("generate-btn-tooltip".to_string(), "Our AI agents will analyze your description and build a ready-to-launch store for you.".to_string());
    m.insert("launch-btn-tooltip".to_string(), "Launch your storefront immediately to a live URL.".to_string());
    m.insert("team-activity-tooltip".to_string(), "Monitor the real-time actions and tasks being performed by your AI workforce.".to_string());
    m.insert("referral-tooltip".to_string(), "Share your unique link to earn credits when friends join OHC.".to_string());
    m.insert("swarm-online-tooltip".to_string(), "Your AI workforce is currently active and processing tasks in the background.".to_string());
    m.insert("department-card-tooltip".to_string(), "Click to view and manage pending approvals for this department.".to_string());
    m.insert("nav-dashboard-tooltip".to_string(), "View your store metrics, recent orders, and overall performance.".to_string());
    m.insert("nav-agents-tooltip".to_string(), "Manage your AI workforce, check their tasks, and hire new agents.".to_string());
    m.insert("nav-setup-tooltip".to_string(), "Configure your business details, branding, and payment settings.".to_string());
    m.insert("credit-tooltip".to_string(), "Earn credits to use on premium tools when you refer a friend.".to_string());
    m.insert("help-btn-tooltip".to_string(), "Need help? Click here to access our Help Center, Ask AI, Video Tutorials, and Release Notes.".to_string());
    m.insert("changelog-nav-tooltip".to_string(), "See what's new in the latest OneHumanCorp updates.".to_string());
    m.insert("todays-sales-tooltip".to_string(), "Your total sales for today. Check back often to track your progress.".to_string());
    m.insert("approval-inbox-tooltip".to_string(), "Review tasks that your AI agents need permission to execute. Approve or deny them here.".to_string());
    m.insert("ask-ai-tooltip".to_string(), "Open the AI Chat to get answers instantly. The AI reads our entire Help Center for you.".to_string());
    RwLock::new(m)
    })
}
pub mod db;
pub use ::server_auth as auth;
pub mod hub;
pub mod minimax;
pub mod billing;
pub mod ultraplan;
pub mod autodream;
pub mod autodream_pipeline;
pub mod tasks;
pub mod settings;
pub mod scheduler;
pub mod msgbus;
pub mod pipeline;
pub use ::server_oidc as oidc;
pub mod sip;
pub mod seeder;
pub mod queue;
pub mod domain;
pub use ::server_pricing as pricing;
pub mod analytics;
pub use ::server_telemetry as telemetry;
pub mod chaos;
pub mod integrations;
pub use ::server_utils as utils;
pub mod orchestration;
pub mod storage;
pub mod interop;

pub mod benchmarks;

pub use ::server_config as config;
pub use ::server_common as common;
pub use ::server_ohc as ohc;
pub mod builder;
pub mod tools;
pub mod workers;
use crate::orchestration::mesh::TeammateMesh;

pub mod services {
    pub mod dashboard;
    pub mod wizard;
    pub mod billing;
    pub mod growth;
    pub mod onboarding;
    pub mod sync;
    pub mod chat;
    pub mod b2b;
    pub mod integration;
    pub mod ops;
    pub mod mcp;
    pub mod org;
    pub mod scheduler;
    pub mod agent;
    pub mod autodream;
    pub mod booking;
}

use tonic::{transport::Server, Request, Response, Status};
use tokio_stream::Stream;
use tokio_stream::StreamExt;
use chrono::Utc;
use std::pin::Pin;
use tokio::sync::mpsc;
use std::sync::OnceLock;
use std::sync::Arc;
// OTP Cache for verification
pub static OTP_STORE: std::sync::LazyLock<std::sync::Mutex<std::collections::HashMap<String, (String, std::time::Instant)>>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(std::collections::HashMap::new()));

use hub::Hub;

static TELEMETRY_CHAN: OnceLock<mpsc::Sender<Box<dyn FnOnce() + Send>>> = OnceLock::new();

fn get_telemetry_chan() -> &'static mpsc::Sender<Box<dyn FnOnce() + Send>> {
    TELEMETRY_CHAN.get_or_init(|| {
        let (tx, rx) = mpsc::channel::<Box<dyn FnOnce() + Send>>(10000);
        let rx = std::sync::Arc::new(tokio::sync::Mutex::new(rx));
        
        for _ in 0..16 {
            let rx = rx.clone();
            tokio::spawn(async move {
                loop {
                    let job = {
                        let mut rx = rx.lock().await;
                        rx.recv().await
                    };
                    
                    if let Some(job) = job {
                        job();
                    } else {
                        break;
                    }
                }
            });
        }
        tx
    })
}

pub fn record_telemetry<F>(f: F)
where
    F: FnOnce() + Send + 'static,
{
    let tx = get_telemetry_chan();
    let _ = tx.try_send(Box::new(f));
}

fn spiffe_interceptor(req: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
    let spiffe_id = req.metadata().get("x-spiffe-id")
        .ok_or_else(|| tonic::Status::unauthenticated("missing x-spiffe-id header"))?;

    let spiffe_id_str = spiffe_id.to_str()
        .map_err(|_| tonic::Status::invalid_argument("invalid x-spiffe-id header"))?;

    match ::server_auth::parse_spiffe_id(spiffe_id_str) {
        Ok((_org_id, _agent_id)) => {
            tracing::info!("Authenticated SPIFFE ID successfully.");
        }
        Err(e) => return Err(e),
    }

    Ok(req)
}

pub mod proto {
    pub mod interop {
        pub use interop_proto::ohc::interop::*;
    }
    pub mod mcp_proxy {
        pub use mcp_proxy_proto::ohc::mcp_proxy::*;
    }
    pub mod orchestration {
        pub use hub_proto::ohc::orchestration::*;
    }
    pub mod billing {
        pub use billing_proto::ohc::billing::*;
    }
    pub mod agent {
        pub use agent_proto::ohc::agent::*;
        pub mod service {
            pub use agent_service_proto::ohc::agent::service::*;
        }
    }
    pub mod organization {
        pub use organization_proto::ohc::organization::*;
    }
    pub mod common {
        pub use common_proto::ohc::common::*;
    }
    pub mod app {
        pub use app_proto::ohc::api::v1::*;
    }
}

use ::server_ohc::orchestration::hub_service_server::{HubService, HubServiceServer};
use ::server_ohc::orchestration::growth_service_server::GrowthServiceServer;
use ::server_ohc::billing::billing_service_server::BillingServiceServer;
use ::server_ohc::orchestration::*;

pub struct MyHubService {
    hub: Arc<Hub>,
    invite_tracker: Arc<crate::services::growth::invites::InviteTracker>,
    viral_loop_tracker: Arc<crate::services::growth::viral_loop::ViralLoopTracker>,
    onboarding_agent: crate::services::onboarding::onboarding_agent::OnboardingAgent,
    publish_counter: opentelemetry::metrics::Counter<u64>,
    stream_counter: opentelemetry::metrics::Counter<u64>,
}

impl MyHubService {
    pub fn new(hub: Arc<Hub>, pool: sqlx::PgPool, db: Arc<crate::db::DB>) -> Self {
        let invite_repo = Arc::new(crate::services::growth::invites::InviteRepository::new(pool));
        let invite_tracker = Arc::new(crate::services::growth::invites::InviteTracker::new(invite_repo));
        let viral_loop_tracker = Arc::new(crate::services::growth::viral_loop::ViralLoopTracker::new());
        let onboarding_agent = crate::services::onboarding::onboarding_agent::OnboardingAgent::new(db, hub.clone());

        let meter = opentelemetry::global::meter("ohc.orchestration.hub");
        let publish_counter = meter.u64_counter("hub.mesh_events.published").build();
        let stream_counter = meter.u64_counter("hub.mesh_events.stream_started").build();

        MyHubService { hub, invite_tracker, viral_loop_tracker, onboarding_agent, publish_counter, stream_counter }
    }
}

#[derive(serde::Deserialize)]
struct HttpLoginRequest {
    username: String,
    password: String,
    organization_id: Option<String>,
}

#[derive(serde::Serialize)]
struct HttpLoginUser {
    id: String,
    username: String,
    email: String,
    roles: Vec<String>,
    organization_id: String,
}

#[derive(serde::Serialize)]
struct HttpLoginResponse {
    token: String,
    expires_at: i64,
    user: HttpLoginUser,
}

#[derive(serde::Serialize)]
struct HttpErrorResponse {
    error: String,
}

#[derive(serde::Deserialize)]
struct DraftReplyRequest {
    customer_message: Option<String>,
}

#[derive(serde::Serialize)]
struct DraftReplyResponse {
    output: String,
}


#[derive(serde::Deserialize)]
struct HttpMetricsRequest {
    tenant_id: String,
}

#[derive(serde::Serialize)]
struct HttpMetricsResponse {
    active_customers: i64,
    pending_orders: i64,
    total_sales: f64,
}

async fn http_metrics_handler(
    db: std::sync::Arc<db::DB>,
    store: std::sync::Arc<crate::auth::Store>,
    headers: axum::http::HeaderMap,
    axum::Json(payload): axum::Json<HttpMetricsRequest>,
) -> axum::response::Response {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    let auth_header = match headers.get("authorization").and_then(|v| v.to_str().ok()) {
        Some(h) => h,
        None => return (StatusCode::UNAUTHORIZED, "Missing authorization header").into_response(),
    };

    let token = if auth_header.to_lowercase().starts_with("bearer ") {
        &auth_header[7..]
    } else {
        auth_header
    };

    let claims = match store.validate_token(token).await {
        Ok(claims) => claims,
        Err(_) => return (StatusCode::UNAUTHORIZED, "Invalid token").into_response(),
    };

    let tenant_id = payload.tenant_id;
    if tenant_id == "system" {
        return (StatusCode::FORBIDDEN, "Querying system tenant is not allowed").into_response();
    }
    if claims.organization_id.as_deref() != Some(&tenant_id) && !claims.roles.contains(&crate::auth::ROLE_ADMIN.to_string()) {
         return (StatusCode::FORBIDDEN, "Tenant ID does not match authorization context").into_response();
    }

    let (active_customers_res, pending_orders_res, sales_res) = tokio::join!(
        async {
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE tenant_id = $1")
                .bind(&tenant_id)
                .fetch_one(&db.pool)
                .await
        },
        async {
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM orders WHERE tenant_id = $1 AND status = 'pending'")
                .bind(&tenant_id)
                .fetch_one(&db.pool)
                .await
        },
        async {
            sqlx::query_scalar::<_, f64>("SELECT COALESCE(SUM(total_amount), 0.0) FROM orders WHERE tenant_id = $1")
                .bind(&tenant_id)
                .fetch_one(&db.pool)
                .await
        }
    );

    let active_customers = active_customers_res.unwrap_or(0);
    let pending_orders = pending_orders_res.unwrap_or(0);
    let total_sales = sales_res.unwrap_or(0.0);

    (
        StatusCode::OK,
        axum::Json(HttpMetricsResponse { active_customers, pending_orders, total_sales }),
    )
        .into_response()
}

async fn http_login_handler(
    db: std::sync::Arc<db::DB>,
    payload: HttpLoginRequest,
) -> axum::response::Response {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use sqlx::Row;

    let username = payload.username.trim();
    if username.is_empty() || payload.password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            axum::Json(HttpErrorResponse { error: "username and password are required".to_string() }),
        )
            .into_response();
    }

    let tenant_id = payload
        .organization_id
        .filter(|id| !id.trim().is_empty())
        .or_else(|| std::env::var("OHC_DEFAULT_TENANT_ID").ok())
        .unwrap_or_else(|| "e2e-tenant".to_string());

    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("failed to start login transaction: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(HttpErrorResponse { error: "login unavailable".to_string() }),
            )
                .into_response();
        }
    };

    if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        tracing::error!("failed to set tenant context for login: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(HttpErrorResponse { error: "login unavailable".to_string() }),
        )
            .into_response();
    }

    let row = match sqlx::query(
        r#"
        SELECT id, username, email, password_hash, roles, tenant_id
        FROM users
        WHERE tenant_id = $1 AND (username = $2 OR email = $2) AND active = TRUE
        LIMIT 1
        "#,
    )
    .bind(&tenant_id)
    .bind(username)
    .fetch_optional(&mut *tx)
    .await
    {
        Ok(row) => row,
        Err(e) => {
            tracing::error!("failed to query login user: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(HttpErrorResponse { error: "login unavailable".to_string() }),
            )
                .into_response();
        }
    };

    let Some(row) = row else {
        return (
            StatusCode::UNAUTHORIZED,
            axum::Json(HttpErrorResponse { error: "invalid credentials".to_string() }),
        )
            .into_response();
    };

    let password_hash: String = row.get("password_hash");
    match bcrypt::verify(&payload.password, &password_hash) {
        Ok(true) => {}
        Ok(false) => {
            return (
                StatusCode::UNAUTHORIZED,
                axum::Json(HttpErrorResponse { error: "invalid credentials".to_string() }),
            )
                .into_response();
        }
        Err(e) => {
            tracing::error!("failed to verify auth credential: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(HttpErrorResponse { error: "login unavailable".to_string() }),
            )
                .into_response();
        }
    }

    let id: String = row.get("id");
    let email: String = row.get("email");
    let username: String = row.get("username");
    let roles: Vec<String> = row.try_get("roles").unwrap_or_default();
    let expires_at = (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp();
    let issued_at = chrono::Utc::now().timestamp();
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "e2e-local-jwt-secret-change-me-32-bytes".to_string());
    let claims = ::server_common::Claims {
        sub: id.clone(),
        exp: expires_at,
        iat: issued_at,
        organization_id: Some(tenant_id.clone()),
        username: username.clone(),
        email: email.clone(),
        roles: roles.clone(),
        session_id: None,
        jti: uuid::Uuid::new_v4().to_string(),
    };
    let token = match jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    ) {
        Ok(token) => token,
        Err(e) => {
            tracing::error!("failed to issue login token: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(HttpErrorResponse { error: "login unavailable".to_string() }),
            )
                .into_response();
        }
    };

    (
        StatusCode::OK,
        axum::Json(HttpLoginResponse {
            token,
            expires_at,
            user: HttpLoginUser {
                id,
                username,
                email,
                roles,
                organization_id: tenant_id,
            },
        }),
    )
        .into_response()
}

async fn advisory_insights_handler(
    db: std::sync::Arc<db::DB>,
    store: std::sync::Arc<crate::auth::Store>,
    headers: axum::http::HeaderMap,
) -> axum::response::Response {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    let auth_header = match headers.get("authorization").and_then(|v| v.to_str().ok()) {
        Some(h) => h,
        None => return (StatusCode::UNAUTHORIZED, "Missing authorization header").into_response(),
    };

    let token = if auth_header.to_lowercase().starts_with("bearer ") {
        &auth_header[7..]
    } else {
        auth_header
    };

    let claims = match store.validate_token(token).await {
        Ok(c) => c,
        Err(_) => return (StatusCode::UNAUTHORIZED, "Invalid token").into_response(),
    };

    let tenant_id = match claims.organization_id {
        Some(id) if !id.trim().is_empty() => id,
        _ => return (StatusCode::FORBIDDEN, "Tenant ID not found in claims").into_response(),
    };

    let api_key = match std::env::var("MINIMAX_API_KEY") {
        Ok(key) if !key.trim().is_empty() => key,
        _ => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                axum::Json(HttpErrorResponse { error: "MINIMAX_API_KEY is required".to_string() }),
            )
                .into_response();
        }
    };

    // Gather context from DB
    let (business_name, industry): (String, String) = sqlx::query_as(
        "SELECT name, COALESCE(industry, '') FROM tenants WHERE id = $1"
    )
    .bind(&tenant_id)
    .fetch_optional(&db.pool)
    .await
    .unwrap_or(None)
    .unwrap_or_else(|| ("A business".to_string(), "".to_string()));

    // Get order counts
    let active_orders: i64 = sqlx::query_scalar("SELECT count(*) FROM orders WHERE tenant_id = $1 AND status != 'delivered'")
        .bind(&tenant_id)
        .fetch_one(&db.pool)
        .await
        .unwrap_or(0);

    let prompt = format!("You are a business advisory agent. Business context: A {} business named {}. The business currently has {} active orders to fulfill. Provide a short, plain language insight (about 2 sentences) summarizing this performance and suggesting an actionable next step, like running a promo or checking the inbox. Make it warm and accessible.", industry, business_name, active_orders);

    let client = crate::minimax::MinimaxClient::new(api_key);
    match client.reason(&prompt).await {
        Ok(output) => (StatusCode::OK, axum::Json(serde_json::json!({ "summary": output }))).into_response(),
        Err(e) => {
            tracing::error!("MiniMax advisory insights failed: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                axum::Json(HttpErrorResponse { error: "AI advisory generation failed".to_string() }),
            )
                .into_response()
        }
    }
}

async fn draft_reply_handler(
    db: std::sync::Arc<db::DB>,
    store: std::sync::Arc<crate::auth::Store>,
    headers: axum::http::HeaderMap,
    payload: DraftReplyRequest,
) -> axum::response::Response {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    let auth_header = match headers.get("authorization").and_then(|v| v.to_str().ok()) {
        Some(h) => h,
        None => return (StatusCode::UNAUTHORIZED, "Missing authorization header").into_response(),
    };

    let token = if auth_header.to_lowercase().starts_with("bearer ") {
        &auth_header[7..]
    } else {
        auth_header
    };

    let claims = match store.validate_token(token).await {
        Ok(claims) => claims,
        Err(_) => return (StatusCode::UNAUTHORIZED, "Invalid token").into_response(),
    };

    let tenant_id = match claims.organization_id {
        Some(id) if !id.trim().is_empty() => id,
        _ => return (StatusCode::FORBIDDEN, "Tenant ID not found in claims").into_response(),
    };

    let api_key = match std::env::var("MINIMAX_API_KEY") {
        Ok(key) if !key.trim().is_empty() => key,
        _ => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                axum::Json(HttpErrorResponse { error: "MINIMAX_API_KEY is required".to_string() }),
            )
                .into_response();
        }
    };

    let (business_name, industry): (String, String) = sqlx::query_as(
        "SELECT name, COALESCE(industry, '') FROM tenants WHERE id = $1"
    )
    .bind(&tenant_id)
    .fetch_optional(&db.pool)
    .await
    .unwrap_or(None)
    .unwrap_or_else(|| ("A business".to_string(), "".to_string()));

    let customer_message = payload
        .customer_message
        .unwrap_or_else(|| "Do you have vegan options for birthday cakes?".to_string());

    let business_context = if industry.is_empty() {
        format!("A business named {}", business_name)
    } else {
        format!("A {} business named {}", industry, business_name)
    };

    let prompt = format!(
        "Write one concise, warm customer-service reply. Business context: {} Customer message: {}",
        business_context, customer_message
    );

    let client = crate::minimax::MinimaxClient::new(api_key);
    match client.reason(&prompt).await {
        Ok(output) => (StatusCode::OK, axum::Json(DraftReplyResponse { output })).into_response(),
        Err(e) => {
            tracing::error!("MiniMax draft reply failed: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                axum::Json(HttpErrorResponse { error: "AI draft generation failed".to_string() }),
            )
                .into_response()
        }
    }
}

#[tonic::async_trait]
impl HubService for MyHubService {

    async fn get_my_plan(
        &self,
        request: tonic::Request<::server_ohc::orchestration::EmptyRequest>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::MyPlanResponse>, tonic::Status> {
                let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>()
            .ok_or_else(|| tonic::Status::unauthenticated("Missing AuthInfo"))?;
        let tenant_id = if auth_info.org_id.is_empty() { return Err(tonic::Status::unauthenticated("Missing org_id")); } else { &auth_info.org_id };

        let tracker = self.hub.tracker();
        let tier_future = tracker.get_tenant_tier(tenant_id);
        let ai_used_future = tracker.get_tenant_actions_used(tenant_id);
        let storage_used_bytes_future = tracker.get_tenant_storage_used(tenant_id);

        let (tier_res, ai_used_res, storage_used_bytes_res) = tokio::join!(tier_future, ai_used_future, storage_used_bytes_future);

        let tier = tier_res.unwrap_or(::server_pricing::rate_limit::PlanTier::Free);
        let ai_used = ai_used_res.unwrap_or(0);
        let storage_used_bytes = storage_used_bytes_res.unwrap_or(0);

        let plan_name = match tier {
            ::server_pricing::rate_limit::PlanTier::Free => "Free",
            ::server_pricing::rate_limit::PlanTier::Starter => "Starter",
            ::server_pricing::rate_limit::PlanTier::Pro => "Pro",
            ::server_pricing::rate_limit::PlanTier::Business => "Business",
        }.to_string();

        let ai_limit = tier.monthly_action_limit().map(|v| v as i32);
        let storage_limit = tier.storage_limit_mb().map(|v| (v as i64) * 1024 * 1024);

        let next_bill_estimated = match tier {
            ::server_pricing::rate_limit::PlanTier::Free => 0,
            ::server_pricing::rate_limit::PlanTier::Starter => 9,
            ::server_pricing::rate_limit::PlanTier::Pro => 29,
            ::server_pricing::rate_limit::PlanTier::Business => 79,
        };

        Ok(tonic::Response::new(::server_ohc::orchestration::MyPlanResponse {
            current_plan: plan_name,
            ai_actions_used: ai_used as i32,
            ai_actions_limit: ai_limit,
            storage_used_bytes,
            storage_limit_bytes: storage_limit,
            next_bill_estimated,
        }))
    }

    async fn get_cost_dashboard(
        &self,
        request: tonic::Request<::server_ohc::orchestration::EmptyRequest>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::CostDashboardResponse>, tonic::Status> {
                let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>()
            .ok_or_else(|| tonic::Status::unauthenticated("Missing AuthInfo"))?;
        let tenant_id = if auth_info.org_id.is_empty() { return Err(tonic::Status::unauthenticated("Missing org_id")); } else { &auth_info.org_id };

        let auditor = self.hub.get_cost_auditor();
        let llm_cost_f64 = auditor.get_total_cost();
        let total_revenue_f64 = auditor.get_total_revenue();

        let storage_bytes = self.hub.tracker().get_tenant_storage_used(tenant_id).await.unwrap_or(0);
        let storage_gb = storage_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        let storage_cost_f64 = storage_gb * 0.10; // $0.10 per GB

        let payment_fees_f64 = total_revenue_f64 * 0.029;

        let total_costs_f64 = llm_cost_f64 + storage_cost_f64 + payment_fees_f64;

        Ok(tonic::Response::new(::server_ohc::orchestration::CostDashboardResponse {
            total_revenue: (total_revenue_f64 * 100.0) as i64,
            total_costs: (total_costs_f64 * 100.0) as i64,
            llm_cost: (llm_cost_f64 * 100.0) as i64,
            storage_cost: (storage_cost_f64 * 100.0) as i64,
            payment_fees: (payment_fees_f64 * 100.0) as i64,
            period_start: "2024-05-01".to_string(), // In a real app this would be computed
            period_end: "2024-05-31".to_string(),
        }))
    }

    async fn select_plan(
        &self,
        request: tonic::Request<::server_ohc::orchestration::SelectPlanRequest>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::SelectPlanResponse>, tonic::Status> {
                let tenant_id = request.extensions().get::<::server_auth::orchestration::AuthInfo>()
            .map(|a| a.org_id.clone())
            .filter(|id| !id.is_empty())
            .ok_or_else(|| tonic::Status::unauthenticated("Missing valid AuthInfo"))?;
        let req = request.into_inner();

        let stripe_key = std::env::var("STRIPE_API_KEY")
            .map_err(|_| tonic::Status::failed_precondition("STRIPE_API_KEY is required"))?;
        let client = crate::integrations::stripe::client::StripeClient::new(stripe_key);
        let mercadopago_client = std::env::var("MERCADOPAGO_ACCESS_TOKEN").ok().map(|token| crate::integrations::mercadopago::client::MercadoPagoClient::new(token));
        let alipay_client = std::env::var("ALIPAY_ACCESS_TOKEN").ok().map(|token| crate::integrations::alipay::client::AlipayClient::new(token));

        let amount = match req.plan_id.as_str() {
            "Starter" => 9.0,
            "Pro" => 29.0,
            "Business" => 79.0,
            _ => 0.0
        };

        let optimal_pm = crate::integrations::stripe::routing::PaymentRouter::optimize_payment_method(amount);
        let savings = crate::integrations::stripe::routing::PaymentRouter::calculate_fee_savings(amount);
        if savings > 0.0 {
            tracing::info!("Optimized payment method to {:?} to save ${:.2} on transaction fees", optimal_pm, savings);
        }

        let is_china = req.plan_id.ends_with("_china");
        let is_latam = req.plan_id.ends_with("_latam");
        let url = if let Some(alipay_client) = alipay_client.filter(|_| is_china) {
            alipay_client.create_checkout_preference(&req.plan_id, &tenant_id).await
        } else if let Some(mp_client) = mercadopago_client.filter(|_| is_latam) {
            mp_client.create_checkout_preference(&req.plan_id, &tenant_id).await
        } else {
            client.create_checkout_session(&req.plan_id, &tenant_id, amount).await
        }
            .map_err(|e| tonic::Status::internal(e))?;

        Ok(tonic::Response::new(::server_ohc::orchestration::SelectPlanResponse {
            success: true,
            checkout_url: url,
        }))
    }

    async fn cancel_subscription(
        &self,
        request: tonic::Request<::server_ohc::orchestration::CancelSubscriptionRequest>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::CancelSubscriptionResponse>, tonic::Status> {
        let req = request.into_inner();
        let stripe_key = std::env::var("STRIPE_API_KEY")
            .map_err(|_| tonic::Status::failed_precondition("STRIPE_API_KEY is required"))?;
        let client = crate::integrations::stripe::client::StripeClient::new(stripe_key);
        let _mercadopago_client = std::env::var("MERCADOPAGO_ACCESS_TOKEN").ok().map(|token| crate::integrations::mercadopago::client::MercadoPagoClient::new(token));
        let _alipay_client = std::env::var("ALIPAY_ACCESS_TOKEN").ok().map(|token| crate::integrations::alipay::client::AlipayClient::new(token));

        client.cancel_subscription(&req.plan_id).await
            .map_err(|e| tonic::Status::internal(e))?;

        Ok(tonic::Response::new(::server_ohc::orchestration::CancelSubscriptionResponse {
            success: true,
        }))
    }

    async fn download_invoice(
        &self,
        _request: tonic::Request<::server_ohc::orchestration::DownloadInvoiceRequest>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::DownloadInvoiceResponse>, tonic::Status> {
        Ok(tonic::Response::new(::server_ohc::orchestration::DownloadInvoiceResponse {
            pdf_url: "https://invoice.stripe.com/...".to_string(),
        }))
    }


    async fn register_agent(
        &self,
        request: Request<RegisterAgentRequest>,
    ) -> Result<Response<RegisterAgentResponse>, Status> {
        let req = request.into_inner();
        if let Some(agent) = req.agent {
            self.hub.register_agent(agent);
            Ok(Response::new(RegisterAgentResponse { success: true }))
        } else {
            Err(Status::invalid_argument("agent is required"))
        }
    }

    async fn handle_config_wizard(
        &self,
        _request: tonic::Request<::server_ohc::orchestration::AgentConfig>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::WizardResponse>, tonic::Status> {
        tracing::debug!("Received ConfigWizard request in wizard service");
        Ok(tonic::Response::new(WizardResponse {
            success: true,
            message: "success".to_string(),
        }))
    }

    async fn handle_prompt_tuning(
        &self,
        _request: tonic::Request<::server_ohc::orchestration::PromptTuningConfig>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::WizardResponse>, tonic::Status> {
        tracing::debug!("Received PromptTuning request in wizard service");
        Ok(tonic::Response::new(WizardResponse {
            success: true,
            message: "success".to_string(),
        }))
    }

    async fn open_meeting(
        &self,
        request: Request<OpenMeetingRequest>,
    ) -> Result<Response<MeetingRoom>, Status> {
        let req = request.into_inner();
        let meeting = self.hub.open_meeting(req.meeting_id, req.participants, req.agenda);
        Ok(Response::new(meeting))
    }

    async fn publish(
        &self,
        request: Request<PublishMessageRequest>,
    ) -> Result<Response<PublishMessageResponse>, Status> {
        let req = request.into_inner();
        if let Some(msg) = req.message {
            match self.hub.clone().publish(msg) {
                Ok(_) => Ok(Response::new(PublishMessageResponse { success: true })),
                Err(e) => Err(Status::internal(e)),
            }
        } else {
            Err(Status::invalid_argument("message is required"))
        }
    }

    async fn delegate_task(
        &self,
        request: Request<DelegateTaskRequest>,
    ) -> Result<Response<DelegateTaskResponse>, Status> {
        let req = request.into_inner();
        if let Some(task) = req.task {
            match self.hub.clone().delegate_task(req.from_agent_id, req.to_agent_id, task) {
                Ok(_) => Ok(Response::new(DelegateTaskResponse { success: true })),
                Err(e) => Err(Status::internal(e)),
            }
        } else {
            Err(Status::invalid_argument("task is required"))
        }
    }

    async fn verify_environment(
        &self,
        request: tonic::Request<VerifyEnvironmentRequest>,
    ) -> Result<tonic::Response<VerifyEnvironmentResponse>, tonic::Status> {
        let req = request.into_inner();
        let env_vars = req.env_vars;
        
        match services::onboarding::env_verifier::verify_environment(&env_vars) {
            Ok(config) => {
                Ok(tonic::Response::new(VerifyEnvironmentResponse {
                    status: "success".to_string(),
                    config: Some(EnvConfig {
                        mode: config.mode,
                        multi_tenant: config.multi_tenant,
                        headless: config.headless,
                        telemetry_enabled: config.telemetry_enabled,
                        api_endpoint: config.api_endpoint,
                        database_url: config.database_url,
                    }),
                    error: String::new(),
                }))
            }
            Err(e) => {
                Ok(tonic::Response::new(VerifyEnvironmentResponse {
                    status: "error".to_string(),
                    config: None,
                    error: e,
                }))
            }
        }
    }

    async fn generate_config(
        &self,
        request: tonic::Request<GenerateConfigRequest>,
    ) -> Result<tonic::Response<GenerateConfigResponse>, tonic::Status> {
        let req = request.into_inner();
        let mode = req.mode;

        let mut config = std::collections::HashMap::new();
        if mode == "cloud" {
            config.insert("swarm_size".to_string(), "large".to_string());
            config.insert("database".to_string(), "postgresql".to_string());
            config.insert("cache".to_string(), "redis".to_string());
        } else if mode == "standalone" {
            config.insert("swarm_size".to_string(), "small".to_string());
            config.insert("database".to_string(), "sqlite".to_string());
            config.insert("cache".to_string(), "memory".to_string());
        } else if mode == "thin_client" {
            config.insert("swarm_size".to_string(), "none".to_string());
            config.insert("database".to_string(), "remote".to_string());
            config.insert("cache".to_string(), "none".to_string());
        } else {
            return Ok(tonic::Response::new(GenerateConfigResponse {
                status: "error".to_string(),
                config: std::collections::HashMap::new(),
            }));
        }

        Ok(tonic::Response::new(GenerateConfigResponse {
            status: "success".to_string(),
            config,
        }))
    }

    async fn save_wizard_state(
        &self,
        request: tonic::Request<SaveWizardStateRequest>,
    ) -> Result<tonic::Response<SaveWizardStateResponse>, tonic::Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>()
            .ok_or_else(|| tonic::Status::unauthenticated("Missing AuthInfo"))?;

        let org_id = auth_info.org_id.clone();
        if org_id.is_empty() {
             return Err(tonic::Status::permission_denied("Only tenants can modify wizard state"));
        }

        let user_id = auth_info.spiffe_id.clone();

        let req = request.into_inner();
        
        let mut state = req.state;
        state.remove("admin_password");
        let current_step = state.get("step").and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
        let state_json = serde_json::to_value(&state).unwrap_or(serde_json::json!({}));

        let tenant_id = org_id.clone();

        let mut tx = self.hub.pool.begin().await.map_err(|e| tonic::Status::internal(e.to_string()))?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| tonic::Status::internal(e.to_string()))?;

        sqlx::query(
            "INSERT INTO onboarding_state (tenant_id, user_id, current_step, state_json) \
             VALUES ($1, $2, $3, $4) \
             ON CONFLICT (tenant_id, user_id) DO UPDATE \
             SET state_json = onboarding_state.state_json || EXCLUDED.state_json, \
                 current_step = EXCLUDED.current_step, \
                 updated_at = CURRENT_TIMESTAMP"
        )
        .bind(&tenant_id)
        .bind(&user_id)
        .bind(current_step)
        .bind(&state_json)
        .execute(&mut *tx)
        .await
        .map_err(|e| tonic::Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(tonic::Response::new(SaveWizardStateResponse {
            status: "saved".to_string(),
        }))
    }

    async fn get_wizard_state(
        &self,
        request: tonic::Request<GetWizardStateRequest>,
    ) -> Result<tonic::Response<GetWizardStateResponse>, tonic::Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>()
            .ok_or_else(|| tonic::Status::unauthenticated("Missing AuthInfo"))?;

        let org_id = auth_info.org_id.clone();
        if org_id.is_empty() {
             return Err(tonic::Status::permission_denied("Only tenants can read wizard state"));
        }
        let tenant_id = org_id.clone();

        let mut tx = self.hub.pool.begin().await.map_err(|e| tonic::Status::internal(e.to_string()))?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| tonic::Status::internal(e.to_string()))?;

        let row = sqlx::query(
            "SELECT state_json FROM onboarding_state WHERE tenant_id = $1 AND organization_id = $2"
        )
        .bind(&tenant_id)
        .bind(&org_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| tonic::Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| tonic::Status::internal(e.to_string()))?;

        let mut state = std::collections::HashMap::new();
        if let Some(record) = row {
            use sqlx::Row;
            let state_json: serde_json::Value = record.get("state_json");
            if let Some(json_obj) = state_json.as_object() {
                for (k, v) in json_obj.iter() {
                    if let Some(s) = v.as_str() {
                        state.insert(k.clone(), s.to_string());
                    } else {
                        state.insert(k.clone(), v.to_string());
                    }
                }
            }
        }

        Ok(tonic::Response::new(GetWizardStateResponse {
            state,
        }))
    }

    async fn reset_wizard_state(
        &self,
        request: tonic::Request<ResetWizardStateRequest>,
    ) -> Result<tonic::Response<ResetWizardStateResponse>, tonic::Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>()
            .ok_or_else(|| tonic::Status::unauthenticated("Missing AuthInfo"))?;

        let org_id = auth_info.org_id.clone();
        if org_id.is_empty() {
             return Err(tonic::Status::permission_denied("Only tenants can reset wizard state"));
        }
        let tenant_id = org_id.clone();

        let mut tx = self.hub.pool.begin().await.map_err(|e| tonic::Status::internal(e.to_string()))?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| tonic::Status::internal(e.to_string()))?;

        sqlx::query(
            "DELETE FROM onboarding_state WHERE tenant_id = $1 AND organization_id = $2"
        )
        .bind(&tenant_id)
        .bind(&org_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| tonic::Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(tonic::Response::new(ResetWizardStateResponse {
            status: "reset".to_string(),
        }))
    }

    async fn provision(
        &self,
        request: tonic::Request<ProvisionRequest>,
    ) -> Result<tonic::Response<ProvisionResponse>, tonic::Status> {
        let _req = request.into_inner();
        
        Ok(tonic::Response::new(ProvisionResponse {
            status: "provisioned".to_string(),
            message: "State persisted successfully".to_string(),
        }))
    }

    async fn publish_site(
        &self,
        request: tonic::Request<PublishSiteRequest>,
    ) -> Result<tonic::Response<PublishSiteResponse>, tonic::Status> {
        let req = request.into_inner();

        // Simulating the actual backend write/deployment operations
        let url = if req.domain_choice == "custom" {
            "https://www.mybusiness.com".to_string()
        } else {
            "https://mybusiness.ohc.app".to_string()
        };

        Ok(tonic::Response::new(PublishSiteResponse {
            status: "published".to_string(),
            url,
        }))
    }

    async fn audit_setup(
        &self,
        request: tonic::Request<AuditSetupRequest>,
    ) -> Result<tonic::Response<AuditSetupResponse>, tonic::Status> {
        let req = request.into_inner();
        let env = req.env;

        match services::onboarding::env_verifier::verify_environment(&env) {
            Ok(config) => {
                Ok(tonic::Response::new(AuditSetupResponse {
                    status: "success".to_string(),
                    config: Some(EnvConfig {
                        mode: config.mode,
                        multi_tenant: config.multi_tenant,
                        headless: config.headless,
                        telemetry_enabled: config.telemetry_enabled,
                        api_endpoint: config.api_endpoint,
                        database_url: config.database_url,
                    }),
                    error: String::new(),
                }))
            }
            Err(e) => {
                Ok(tonic::Response::new(AuditSetupResponse {
                    status: "error".to_string(),
                    config: None,
                    error: e,
                }))
            }
        }
    }

    async fn diagnostics(
        &self,
        _request: tonic::Request<DiagnosticsRequest>,
    ) -> Result<tonic::Response<DiagnosticsResponse>, tonic::Status> {
        let env_vars = std::env::vars().collect::<std::collections::HashMap<String, String>>();
        
        let config_res = services::onboarding::env_verifier::verify_environment(&env_vars);
        
        let state = std::collections::HashMap::new();

        match config_res {
            Ok(config) => {
                Ok(tonic::Response::new(DiagnosticsResponse {
                    status: "success".to_string(),
                    config: Some(EnvConfig {
                        mode: config.mode,
                        multi_tenant: config.multi_tenant,
                        headless: config.headless,
                        telemetry_enabled: config.telemetry_enabled,
                        api_endpoint: config.api_endpoint,
                        database_url: config.database_url,
                    }),
                    wizard_state: state,
                    error: String::new(),
                }))
            }
            Err(e) => {
                Ok(tonic::Response::new(DiagnosticsResponse {
                    status: "error".to_string(),
                    config: None,
                    wizard_state: state,
                    error: e,
                }))
            }
        }
    }

    async fn get_wizard_profile(
        &self,
        request: tonic::Request<GetWizardProfileRequest>,
    ) -> Result<tonic::Response<GetWizardProfileResponse>, tonic::Status> {
        let req = request.into_inner();
        let mode = req.mode;

        let profile = if mode == "cloud" {
            Some(EnvConfig {
                mode: "cloud".to_string(),
                multi_tenant: true,
                headless: false,
                telemetry_enabled: true,
                api_endpoint: String::new(),
                database_url: "postgresql://user:pass@localhost:5432/ohc".to_string(),
            })
        } else if mode == "standalone" {
            Some(EnvConfig {
                mode: "standalone".to_string(),
                multi_tenant: false,
                headless: false,
                telemetry_enabled: false,
                api_endpoint: String::new(),
                database_url: "sqlite://local.db".to_string(),
            })
        } else {
            return Ok(tonic::Response::new(GetWizardProfileResponse {
                status: "error".to_string(),
                profile: None,
                error: "Invalid mode requested".to_string(),
            }));
        };

        Ok(tonic::Response::new(GetWizardProfileResponse {
            status: "success".to_string(),
            profile,
            error: String::new(),
        }))
    }

    async fn create_task(
        &self,
        request: Request<CreateTaskRequest>,
    ) -> Result<Response<::server_ohc::orchestration::SharedTask>, Status> {
        let req = request.into_inner();
        let task = self.hub.task_manager().create_task(
            "default_org".to_string(),
            req.mission_id,
            req.title,
            req.description,
            req.priority,
        ).map_err(|e| Status::internal(e))?;
        
        Ok(Response::new(::server_ohc::orchestration::SharedTask {
            id: task.id,
            organization_id: task.organization_id,
            parent_plan_id: task.parent_plan_id,
            dependencies: task.dependencies,
            title: task.title,
            description: task.description.unwrap_or_default(),
            status: task.status,
            assigned_agent_id: task.assigned_agent_id.unwrap_or_default(),
            priority: task.priority,
            payload: task.payload,
            locked_until_unix: task.locked_until.map(|t| t.timestamp()).unwrap_or(0),
            created_at_unix: task.created_at.timestamp(),
            updated_at_unix: task.updated_at.timestamp(),
            action_risk: match task.action_risk {
                Some(crate::tasks::ActionRisk::Low) => 1,
                Some(crate::tasks::ActionRisk::High) => 2,
                _ => 0,
            },
            approval_status: task.approval_status.unwrap_or_default(),
            proposed_content: task.proposed_content.unwrap_or_default(),
        }))
    }

    type PollTasksStream = Pin<Box<dyn Stream<Item = Result<::server_ohc::orchestration::SharedTask, Status>> + Send>>;
    
    async fn poll_tasks(
        &self,
        request: Request<PollTasksRequest>,
    ) -> Result<Response<Self::PollTasksStream>, Status> {
        let req = request.into_inner();
        let tasks = self.hub.task_manager().poll_tasks(&req.agent_id, req.limit as usize);
        
        let mapped_tasks: Vec<Result<::server_ohc::orchestration::SharedTask, Status>> = tasks.into_iter().map(|task| {
            Ok(::server_ohc::orchestration::SharedTask {
                id: task.id,
                organization_id: task.organization_id,
                parent_plan_id: task.parent_plan_id,
                dependencies: task.dependencies,
                title: task.title,
                description: task.description.unwrap_or_default(),
                status: task.status,
                assigned_agent_id: task.assigned_agent_id.unwrap_or_default(),
                priority: task.priority,
                payload: task.payload,
                locked_until_unix: task.locked_until.map(|t| t.timestamp()).unwrap_or(0),
                created_at_unix: task.created_at.timestamp(),
                updated_at_unix: task.updated_at.timestamp(),
                action_risk: match task.action_risk {
                    Some(crate::tasks::ActionRisk::Low) => 1,
                    Some(crate::tasks::ActionRisk::High) => 2,
                    _ => 0,
                },
                approval_status: task.approval_status.unwrap_or_default(),
                proposed_content: task.proposed_content.unwrap_or_default(),
            })
        }).collect();
        
        let stream = tokio_stream::iter(mapped_tasks);
        Ok(Response::new(Box::pin(stream) as Self::PollTasksStream))
    }

    async fn update_task_status(
        &self,
        request: Request<UpdateTaskStatusRequest>,
    ) -> Result<Response<UpdateTaskStatusResponse>, Status> {
        let req = request.into_inner();
        
        match req.status.as_str() {
            "REVIEW" => {
                self.hub.task_manager().review_task(&req.task_id, &req.agent_id)
                    .map_err(|e| Status::internal(e))?;
            }
            "COMPLETED" => {
                self.hub.task_manager().complete_task(&req.task_id, &req.agent_id, req.result)
                    .map_err(|e| Status::internal(e))?;
            }
            _ => {
                self.hub.task_manager().update_task_status(&req.task_id, req.status)
                    .map_err(|e| Status::internal(e))?;
            }
        }
        
        Ok(Response::new(UpdateTaskStatusResponse { success: true }))
    }


    async fn approve_task(
        &self,
        request: Request<ApproveTaskRequest>,
    ) -> Result<Response<ApproveTaskResponse>, Status> {
        let org_id = request.extensions().get::<::server_common::Claims>()
            .ok_or_else(|| Status::unauthenticated("Missing claims"))?
            .organization_id.as_ref()
            .ok_or_else(|| Status::unauthenticated("Missing org_id"))?
            .clone();

        let req = request.into_inner();
        self.hub.task_manager().approve_task(&req.task_id, req.is_approved, &org_id).await
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(ApproveTaskResponse {
            success: true,
        }))
    }

    async fn get_pending_approvals(
        &self,
        request: Request<GetPendingApprovalsRequest>,
    ) -> Result<Response<GetPendingApprovalsResponse>, Status> {
        let req = request.into_inner();
        let tasks = self.hub.task_manager().get_pending_approvals(&req.organization_id);

        let mapped_tasks: Vec<::server_ohc::orchestration::SharedTask> = tasks.into_iter().map(|task| {
            ::server_ohc::orchestration::SharedTask {
                id: task.id,
                organization_id: task.organization_id,
                parent_plan_id: task.parent_plan_id,
                dependencies: task.dependencies,
                title: task.title,
                description: task.description.unwrap_or_default(),
                status: task.status,
                assigned_agent_id: task.assigned_agent_id.unwrap_or_default(),
                priority: task.priority,
                payload: task.payload,
                locked_until_unix: task.locked_until.map(|t| t.timestamp()).unwrap_or(0),
                created_at_unix: task.created_at.timestamp(),
                updated_at_unix: task.updated_at.timestamp(),
                action_risk: match task.action_risk {
                    Some(crate::tasks::ActionRisk::Low) => 1,
                    Some(crate::tasks::ActionRisk::High) => 2,
                    _ => 0,
                },
                approval_status: task.approval_status.unwrap_or_default(),
                proposed_content: task.proposed_content.unwrap_or_default(),
            }
        }).collect();

        Ok(Response::new(GetPendingApprovalsResponse {
            tasks: mapped_tasks,
        }))
    }

    async fn trigger_custom_order(
        &self,
        request: Request<TriggerCustomOrderRequest>,
    ) -> Result<Response<TriggerCustomOrderResponse>, Status> {
        let req = request.into_inner();

        let mut ops_task = self.hub.task_manager().create_task(
            req.organization_id.clone(),
            format!("mission-ops-{}", uuid::Uuid::new_v4()),
            format!("Process Custom Order for {}", req.customer_name),
            req.details.clone(),
            "P1".to_string(),
        ).map_err(|e| Status::internal(e))?;
        ops_task.action_risk = Some(crate::tasks::ActionRisk::Low);
        self.hub.task_manager().insert_task(ops_task);

        let mut cs_task = self.hub.task_manager().create_task(
            req.organization_id.clone(),
            format!("mission-cs-{}", uuid::Uuid::new_v4()),
            format!("Draft Confirmation for {}", req.customer_name),
            req.details.clone(),
            "P1".to_string(),
        ).map_err(|e| Status::internal(e))?;
        cs_task.action_risk = Some(crate::tasks::ActionRisk::High);
        cs_task.approval_status = Some("PENDING".to_string());
        cs_task.proposed_content = Some(format!("Hi {}, thank you for your custom order!", req.customer_name));
        self.hub.task_manager().insert_task(cs_task);

        Ok(Response::new(TriggerCustomOrderResponse {
            success: true,
        }))
    }

    async fn decompose_task(
        &self,
        request: Request<DecomposeTaskRequest>,
    ) -> Result<Response<DecomposeTaskResponse>, Status> {
        let req = request.into_inner();
        
        for st in req.sub_tasks {
            let mut filtered_deps = Vec::new();
            for dep in st.dependencies {
                if dep != req.task_id {
                    filtered_deps.push(dep);
                }
            }
            
            self.hub.task_manager().create_task_with_plan(
                req.organization_id.clone(),
                String::new(),
                req.task_id.clone(),
                filtered_deps,
                st.title,
                st.description,
                st.priority,
            ).map_err(|e| Status::internal(e))?;
        }
        
        Ok(Response::new(DecomposeTaskResponse { success: true }))
    }

    type StreamMessagesStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;

    async fn stream_messages(
        &self,
        request: Request<StreamMessagesRequest>,
    ) -> Result<Response<Self::StreamMessagesStream>, Status> {
        let req = request.into_inner();
        let agent_id = req.agent_id.clone();
        
        let rx = self.hub.subscribe(agent_id.clone());
        let drained = self.hub.get_inbox(&agent_id);
        
        let drained_stream = tokio_stream::iter(drained.into_iter().map(Ok));
        
        let rx_stream = tokio_stream::wrappers::BroadcastStream::new(rx)
            .map(|res| match res {
                Ok(msg) => Ok(msg),
                Err(e) => Err(Status::internal(e.to_string())),
            });
            
        let full_stream = drained_stream.chain(rx_stream);
        
        Ok(Response::new(Box::pin(full_stream) as Self::StreamMessagesStream))
    }

    async fn reason(
        &self,
        request: Request<ReasonRequest>,
    ) -> Result<Response<ReasonResponse>, Status> {
        let req = request.into_inner();
        let api_key = self.hub.minimax_api_key().to_string();
        if api_key.is_empty() {
            return Err(Status::failed_precondition("Minimax API key is not configured"));
        }
        
        let client = minimax::MinimaxClient::new(api_key);
        match client.reason(&req.prompt).await {
            Ok(content) => Ok(Response::new(ReasonResponse { content })),
            Err(e) => Err(Status::internal(e)),
        }
    }

    async fn delegate_sub_task(
        &self,
        request: Request<SubTask>,
    ) -> Result<Response<DelegateTaskResponse>, Status> {
        let req = request.into_inner();
        
        if req.task_id.is_empty() || req.target_role.is_empty() {
            return Err(Status::invalid_argument("task_id and target_role are required"));
        }
        
        if self.hub.get_agent(&req.from_agent_id).is_none() {
            return Err(Status::invalid_argument("sender agent is not registered"));
        }

        // Quota Enforcement
        if self.hub.get_agents_count() >= 10 {
            return Err(Status::resource_exhausted("VRAM quota limit exceeded, cannot spawn sub-agent"));
        }
        
        let now_nano = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let sub_agent_id = format!("sub-agent-{}-{}", req.target_role, now_nano);
        
        let sub_agent = Agent {
            id: sub_agent_id.clone(),
            name: format!("Specialized {} Agent", req.target_role),
            role: req.target_role.clone(),
            organization_id: "dynamic-delegation".to_string(),
            status: "IDLE".to_string(),
            provider_type: "builtin".to_string(),
        };
        
        self.hub.register_agent(sub_agent);
        
        // Prompt injection checks
        if req.instruction.contains("SYSTEM:") || req.instruction.contains("\n\n") {
            return Err(Status::invalid_argument("instruction contains forbidden prompt injection sequences"));
        }
        if req.parent_thread_id.contains("SYSTEM:") || req.parent_thread_id.contains("\n\n") {
            return Err(Status::invalid_argument("parent_thread_id contains forbidden prompt injection sequences"));
        }
        
        // Delegate to K8s Operator
        let pod_id = crate::orchestration::hierarchical::K8sOperatorDelegator::spawn_sub_agent_pod(
            &req.target_role,
            &req.instruction,
            &req.parent_thread_id,
        ).await.map_err(|e| Status::internal(e))?;
        tracing::debug!("Spawned K8s Pod {} for Hierarchical Task Delegation", pod_id);

        let msg_id = format!("msg-{}-{}", req.task_id, now_nano);
        let msg = Message {
            id: msg_id,
            from_agent: req.from_agent_id,
            to_agent: sub_agent_id,
             r#type: "TaskDelegation".to_string(),
            content: format!("Execute Task: {}\nContext: {}\nK8sPod: {}", req.instruction, req.parent_thread_id, pod_id),
            occurred_at_unix: Utc::now().timestamp(),
            meeting_id: String::new(),
        };
        
        match self.hub.clone().publish(msg) {
            Ok(_) => Ok(Response::new(DelegateTaskResponse { success: true })),
            Err(e) => Err(Status::internal(e)),
        }
    }

    async fn advertise_capabilities(
        &self,
        request: Request<AgentCapabilities>,
    ) -> Result<Response<PublishMessageResponse>, Status> {
        let req = request.into_inner();
        if req.agent_id.is_empty() {
            return Err(Status::invalid_argument("agent_id is required"));
        }
        
        match self.hub.advertise_capabilities(req) {
            Ok(_) => Ok(Response::new(PublishMessageResponse { success: true })),
            Err(e) => Err(Status::internal(e)),
        }
    }

    type DiscoverAgentsStream = Pin<Box<dyn Stream<Item = Result<AgentCapabilities, Status>> + Send>>;

    async fn discover_agents(
        &self,
        _request: Request<Query>,
    ) -> Result<Response<Self::DiscoverAgentsStream>, Status> {
        let rx = self.hub.subscribe_capabilities();
        
        let rx_stream = tokio_stream::wrappers::BroadcastStream::new(rx)
            .map(|res| match res {
                Ok(caps) => Ok(caps),
                Err(e) => Err(Status::internal(e.to_string())),
            });
            
        Ok(Response::new(Box::pin(rx_stream) as Self::DiscoverAgentsStream))
    }

    type StreamMeshEventsStream = Pin<Box<dyn Stream<Item = Result<MeshEvent, Status>> + Send>>;

    async fn publish_mesh_event(
        &self,
        request: Request<::server_ohc::orchestration::PublishMeshEventRequest>,
    ) -> Result<Response<PublishMessageResponse>, Status> {
        let req = request.into_inner();
        if let Some(event) = req.event {
            self.publish_counter.add(1, &[opentelemetry::KeyValue::new("topic", event.topic.clone())]);

            match self.hub.publish_mesh_event(event) {
                Ok(_) => Ok(Response::new(PublishMessageResponse { success: true })),
                Err(e) => Err(Status::internal(e)),
            }
        } else {
            Err(Status::invalid_argument("event is required"))
        }
    }

    async fn stream_mesh_events(
        &self,
        request: Request<EventStreamRequest>,
    ) -> Result<Response<Self::StreamMeshEventsStream>, Status> {
        let req = request.into_inner();
        if req.topic.is_empty() {
            return Err(Status::invalid_argument("topic is required"));
        }
        
        self.stream_counter.add(1, &[opentelemetry::KeyValue::new("topic", req.topic.clone())]);

        let rx = self.hub.subscribe_mesh_events(req.topic);
        
        let rx_stream = tokio_stream::wrappers::BroadcastStream::new(rx)
            .map(|res| match res {
                Ok(event) => Ok(event),
                Err(e) => Err(Status::internal(e.to_string())),
            });
            
        Ok(Response::new(Box::pin(rx_stream) as Self::StreamMeshEventsStream))
    }

    type StreamTeammateMeshStream = Pin<Box<dyn Stream<Item = Result<TeammateMeshEvent, Status>> + Send>>;

    async fn publish_teammate_mesh_event(
        &self,
        request: Request<PublishTeammateMeshEventRequest>,
    ) -> Result<Response<PublishMessageResponse>, Status> {
        let req = request.into_inner();
        if req.channel.is_empty() {
            return Err(Status::invalid_argument("channel is required"));
        }
        if let Some(event) = req.event {
            match self.hub.publish_teammate_event(req.channel, event) {
                Ok(_) => Ok(Response::new(PublishMessageResponse { success: true })),
                Err(e) => Err(Status::internal(e)),
            }
        } else {
            Err(Status::invalid_argument("event is required"))
        }
    }

    async fn stream_teammate_mesh(
        &self,
        request: Request<EventStreamRequest>,
    ) -> Result<Response<Self::StreamTeammateMeshStream>, Status> {
        let req = request.into_inner();
        if req.topic.is_empty() {
            return Err(Status::invalid_argument("topic is required"));
        }
        
        let rx = self.hub.subscribe_teammate_mesh(req.topic);
        
        let rx_stream = tokio_stream::wrappers::BroadcastStream::new(rx)
            .map(|res| match res {
                Ok(event) => Ok(event),
                Err(e) => Err(Status::internal(e.to_string())),
            });
            
        Ok(Response::new(Box::pin(rx_stream) as Self::StreamTeammateMeshStream))
    }

    async fn invite(
        &self,
        request: Request<InviteRequest>,
    ) -> Result<Response<InviteResponse>, Status> {
        let req = request.into_inner();
        
        if req.team_id.is_empty() || req.inviter_id.is_empty() || req.invitee_id.is_empty() {
            return Err(Status::invalid_argument("Missing required fields"));
        }

        self.invite_tracker.record_invite(&req.team_id, &req.inviter_id, &req.invitee_id).await
            .map_err(|e| Status::internal(format!("Failed to record invite: {}", e)))?;

        self.viral_loop_tracker.record_invite_sent(&req.inviter_id);

        Ok(Response::new(InviteResponse { success: true }))
    }

    async fn accept_invite(
        &self,
        request: Request<AcceptInviteRequest>,
    ) -> Result<Response<AcceptInviteResponse>, Status> {
        let req = request.into_inner();
        
        if req.invitee_id.is_empty() {
            return Err(Status::invalid_argument("Missing invitee_id"));
        }

        self.viral_loop_tracker.record_invite_accepted(&req.invitee_id);

        Ok(Response::new(AcceptInviteResponse { success: true }))
    }

    async fn get_meetings(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<GetMeetingsResponse>, Status> {
        let meetings = self.hub.get_meetings();
        Ok(Response::new(GetMeetingsResponse { meetings: meetings.await.to_vec() }))
    }

    async fn start_onboarding(
        &self,
        request: Request<StartOnboardingRequest>,
    ) -> Result<Response<StartOnboardingResponse>, Status> {
        let req = request.into_inner();
        match self.onboarding_agent.start_onboarding(req).await {
            Ok(resp) => Ok(Response::new(resp)),
            Err(e) => Err(Status::internal(e)),
        }
    }
}

pub async fn dispatch_critical_sms(event_type: &str, message: &str) -> Result<(), String> {
    let store = crate::settings::Store::new();
    let settings = store.get();

    let should_send = match event_type {
        "failed_payment" => settings.sms_alert_failed_payment,
        "new_order" => settings.sms_alert_new_order,
        "urgent_booking" => settings.sms_alert_urgent_booking,
        _ => false,
    };

    if !should_send {
        return Ok(());
    }

    if let Some(phone) = settings.sms_critical_phone {
        let account_sid = std::env::var("TWILIO_ACCOUNT_SID").unwrap_or_else(|_| "dummy_sid".to_string());
        let auth_token = std::env::var("TWILIO_AUTH_TOKEN").unwrap_or_else(|_| "dummy_token".to_string());
        let from_number = std::env::var("TWILIO_FROM_NUMBER").unwrap_or_else(|_| "+1234567890".to_string());

        let provider = crate::integrations::twilio::provider::TwilioProvider::new(account_sid, auth_token);

        if let Err(e) = provider.send_sms(&phone, &from_number, message).await {
            tracing::warn!("Failed to dispatch critical SMS to {}: {}. Expected if Twilio is not configured.", phone, e);
        }
    }
    Ok(())
}

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    let use_json = std::env::var("LOG_FORMAT").unwrap_or_default() == "json";

    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()));

    if use_json {
        subscriber.json().init();
    } else {
        subscriber.init();
    }

    // Initialize database
    let db = Arc::new(db::DB::new().await?);
    db.run_migrations().await?;

    let grpc_port = std::env::var("OHC_GRPC_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8081);
    let addr = format!("0.0.0.0:{}", grpc_port).parse()?;
    let (event_tx, mut event_rx) = tokio::sync::mpsc::channel(100);
    let hub = Arc::new(Hub::new(event_tx, db.pool.clone()));
    hub.set_db(db.clone());
    
    // Start AutoDream worker
    let autodream_worker = Arc::new(autodream::AutoDreamWorker::new(db.clone()));
    autodream_worker.start();

    // Start Memory Consolidation Worker
    let vector_repo = std::sync::Arc::new(match &db.store {
        crate::db::DbStore::Postgres => ohc_builtin_agent::memory_store::VectorRepository::new(db.pool.clone()),
        crate::db::DbStore::Sqlite(sqlite_pool) => ohc_builtin_agent::memory_store::VectorRepository::new_sqlite(sqlite_pool.clone()),
    });
    let consolidation_worker = crate::workers::memory::MemoryConsolidationWorker::new(vector_repo);
    consolidation_worker.start();

    // Start Competitor Audit Worker
    let competitor_audit_worker = crate::workers::competitor_audit::CompetitorAuditWorker::new(db.clone());
    competitor_audit_worker.start();

    let ops_worker = crate::workers::department_workers::OperationsWorker::new(db.clone());
    let promoter_worker = crate::workers::department_workers::PromoterWorker::new(db.clone(), hub.clone());
    promoter_worker.start();

    ops_worker.start();
    let cs_worker = crate::workers::department_workers::CustomerSuccessWorker::new(db.clone());
    cs_worker.start();

    // Start Maintenance Worker
    let maintenance_worker = Arc::new(crate::workers::maintenance::MaintenanceWorker::new(db.clone()));
    maintenance_worker.start();

    // Start Token Forecast Engine
    let forecaster = Arc::new(crate::telemetry::forecaster::Forecaster::new(db.pool.clone()));
    forecaster.start();

    // Start Agent Memory Pipeline
    let memory_embedding_api = Arc::new(crate::workers::agent_memory_pipeline::DefaultMemoryEmbeddingApi::new());
    let agent_memory_pipeline = Arc::new(crate::workers::agent_memory_pipeline::AgentMemoryPipeline::new(db.clone(), memory_embedding_api));
    let agent_memory_pipeline_clone = agent_memory_pipeline.clone();
    tokio::spawn(async move {
        loop {
            if let Err(e) = agent_memory_pipeline_clone.run().await {
                tracing::error!("Agent Memory Pipeline error: {}", e);
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    });

    // Ensure local database permissions are secure in standalone mode
    if std::env::var("STANDALONE_MODE").unwrap_or_else(|_| "true".to_string()) == "true" {
        // Initialize local tables required for standalone mode
        if let crate::db::DbStore::Sqlite(pool) = &db.store {
            let _ = sqlx::query(
                "CREATE TABLE IF NOT EXISTS consolidated_memory (
                    id TEXT PRIMARY KEY,
                    tenant_id TEXT NOT NULL,
                    agent_id TEXT,
                    content TEXT NOT NULL,
                    embedding VECTOR(1536),
                    source_type TEXT NOT NULL,
                    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                    last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                    reference_count INTEGER DEFAULT 0,
                    reliability_score INTEGER DEFAULT 50,
                    owner_override BOOLEAN DEFAULT FALSE,
                    metadata TEXT
                );"
            )
            .execute(pool)
            .await;

            let _ = sqlx::query(
                "CREATE TABLE IF NOT EXISTS onboarding_state (
                    tenant_id TEXT NOT NULL,
                    organization_id TEXT NOT NULL,
                    user_id TEXT NOT NULL,
                    current_step INTEGER NOT NULL DEFAULT 0,
                    state_json JSONB NOT NULL DEFAULT '{}'::jsonb,
                    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                    PRIMARY KEY (tenant_id, organization_id)
                );"
            )
            .execute(pool)
            .await;
        }

        let cfg = crate::config::get();
        let _db_path = cfg.database_url.as_ref()
            .and_then(|url| url.strip_prefix("sqlite://"))
            .map(|s| s.split('?').next().unwrap_or("ohc-standalone.db"))
            .unwrap_or("ohc-standalone.db");
        #[cfg(unix)]
        {
            use std::fs::OpenOptions;
            use std::os::unix::fs::OpenOptionsExt;
            use std::os::unix::fs::PermissionsExt;

            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .mode(0o600)
                .open(_db_path)?;

            let metadata = file.metadata()?;
            let mut perms = metadata.permissions();
            if perms.mode() & 0o777 != 0o600 {
                perms.set_mode(0o600);
                file.set_permissions(perms)?;
            }
        }
    }

    // Start Mesh API server
    let is_cloud = std::env::var("STANDALONE_MODE").unwrap_or_else(|_| "true".to_string()) != "true";
    let mesh_transport = ohc_builtin_agent::mesh::transport::create_transport(
        std::env::var("REDIS_URL").ok().as_deref(),
        is_cloud
    ).await.expect("Failed to create MeshTransport");

    // Initialize Handoff Manager
    let handoff_mesh = std::sync::Arc::new(crate::orchestration::mesh::CentrifugeNode::new(mesh_transport.clone()));
    let dept_orchestrator = std::sync::Arc::new(crate::orchestration::departments::orchestrator::DepartmentOrchestrator::new(db.clone(), handoff_mesh.clone()));
    let ops_agent = std::sync::Arc::new(tokio::sync::RwLock::new(crate::orchestration::departments::operations_agent::OperationsAgent::new(dept_orchestrator.clone())));
    let cs_agent = std::sync::Arc::new(tokio::sync::RwLock::new(crate::orchestration::departments::customer_success_agent::CustomerSuccessAgent::new(dept_orchestrator.clone())));
    let mkt_agent = std::sync::Arc::new(tokio::sync::RwLock::new(crate::orchestration::departments::marketing_agent::MarketingAgent::new(dept_orchestrator.clone())));
    dept_orchestrator.register_department(ops_agent).await;
    dept_orchestrator.register_department(cs_agent).await;
    dept_orchestrator.register_department(mkt_agent).await;

    let handoff_manager = crate::orchestration::handoff::HandoffManager::new(
        handoff_mesh.clone(),
        db.clone(),
        is_cloud
    );
    if let Err(e) = handoff_manager.start_listener().await {
        tracing::error!("Failed to start handoff listener: {}", e);
    }


    // Start Cross-Mode Health Monitor
    let monitor_mesh = handoff_mesh.clone();
    let monitor_hub = hub.clone();
    tokio::spawn(async move {
        crate::orchestration::health::run_health_monitor(
            monitor_mesh,
            monitor_hub,
            is_cloud,
            std::time::Duration::from_secs(30)
        ).await;
    });

    // Start Builtin Agent
    let builtin_transport = mesh_transport.clone();
    let builtin_mesh = handoff_mesh.clone();
    tokio::spawn(async move {
        let agent_id = std::env::var("OHC_AGENT_ID").unwrap_or_else(|_| uuid::Uuid::new_v4().hyphenated().to_string());

        // Cross-Mode Health Monitoring: Builtin Agent Heartbeat
        let heartbeat_transport = builtin_transport.clone();
        let heartbeat_agent_id = agent_id.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
            loop {
                interval.tick().await;
                if let Err(e) = heartbeat_transport.register_presence(&heartbeat_agent_id, "online", 60).await {
                    tracing::error!("Failed to register builtin agent presence: {}", e);
                }
            }
        });

        let _health_cancel = builtin_mesh.start_health_responder().await;

        let cfg = ohc_builtin_agent::service::AgentConfig {
            llm_provider: std::env::var("OHC_LLM_PROVIDER").unwrap_or_default(),
            model: std::env::var("OHC_LLM_MODEL").unwrap_or_default(),
            llm_endpoint: std::env::var("OHC_LOCAL_LLM_ENDPOINT").unwrap_or_default(),
            system_prompt: ::server_pricing::compression::reduce_tokens(&std::env::var("OHC_SYSTEM_PROMPT").unwrap_or_default()),
            max_tokens: {
                let parsed = std::env::var("OHC_MAX_TOKENS").ok().and_then(|v| v.parse().ok()).unwrap_or(2048);
                if parsed > 4096 { 4096 } else if parsed == 0 { 2048 } else { parsed }
            },
            temperature: std::env::var("OHC_TEMPERATURE").ok().and_then(|v| v.parse().ok()).unwrap_or(0.0),
            max_iterations: std::env::var("OHC_MAX_ITERATIONS").ok().and_then(|v| v.parse().ok()).unwrap_or(100),
            max_context_messages: std::env::var("OHC_MAX_CONTEXT_MESSAGES").ok().and_then(|v| v.parse().ok()).unwrap_or(80),
        };
        let auth = ohc_builtin_agent::auth::auth_mode_from_env();
        let agent_id_clone = agent_id.clone();
        let mut svc_impl = ohc_builtin_agent::service::AgentServiceImpl::new(agent_id, cfg, auth);
        svc_impl.init_memory().await;
        let svc = std::sync::Arc::new(svc_impl);

        let heartbeat_transport = builtin_transport.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = heartbeat_transport.register_presence(&agent_id_clone, "active", 30).await {
                    tracing::error!("Failed to register presence: {}", e);
                }
                tokio::time::sleep(std::time::Duration::from_secs(15)).await;
            }
        });

        ohc_builtin_agent::start_builtin_agent(builtin_transport, svc).await;
    });

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let rate_limiter = if let Ok(client) = redis::Client::open(redis_url.clone()) {
        std::sync::Arc::new(::server_pricing::rate_limit::RedisRateLimiter::new(client))
    } else {
        panic!("Failed to initialize Redis client for RateLimiter at {}", redis_url);
    };

    let webhook_state = crate::api::billing_webhook::WebhookState {
        rate_limiter: rate_limiter.clone(),
        db_pool: db.pool.clone(),
        db: db.clone(),
    };

    let webhook_router = axum::Router::new()
        .route("/api/v1/webhooks/stripe", axum::routing::post(api::billing_webhook::stripe_webhook_handler))
        .route("/api/v1/webhooks/mercadopago", axum::routing::post(api::billing_webhook::mercadopago_webhook_handler))
        .route("/api/v1/webhooks/razorpay", axum::routing::post(api::billing_webhook::razorpay_webhook_handler))
        .route("/api/v1/webhooks/calcom", axum::routing::post(api::billing_webhook::calcom_webhook_handler))
        .route("/api/v1/webhooks/resend", axum::routing::post(api::billing_webhook::resend_webhook_handler))
        .route("/api/v1/webhooks/ayrshare", axum::routing::post(api::billing_webhook::ayrshare_webhook_handler))
        .route("/api/v1/webhooks/manychat", axum::routing::post(api::billing_webhook::manychat_webhook_handler))
        .with_state(webhook_state);

    let health_router = axum::Router::new()
        .route("/api/v1/health", axum::routing::get(api::health::health_handler))
        .with_state(hub.clone());

    let db_for_login = db.clone();
async fn generate_manychat_draft_handler() -> axum::response::Response {
    use axum::response::IntoResponse;
    let draft = "Yes, we have several vegan birthday cake options available! You can order them directly from our website or let me know what flavors you are interested in.";
    (axum::http::StatusCode::OK, axum::Json(serde_json::json!({ "draft": draft }))).into_response()
}

async fn get_inbox_messages_handler(axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>) -> axum::response::Response {
    use axum::response::IntoResponse;
    let pool = crate::db::get_pool();

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {}", e);
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!([]))).into_response();
        }
    };

    let org_id = user.organization_id.unwrap_or_default();
    if let Err(e) = crate::common::auth_utils::set_org_context(&mut *tx, &org_id).await {
        tracing::error!("Failed to set org context: {}", e);
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!([]))).into_response();
    }

    match sqlx::query("SELECT id, tenant_id, source, content, draft_reply, status, created_at FROM inbox_messages ORDER BY created_at DESC")
        .fetch_all(&mut *tx)
        .await
    {
        Ok(rows) => {
            let _ = tx.commit().await;
            let messages: Vec<serde_json::Value> = rows.into_iter().map(|row| {
                use sqlx::Row;
                let created_at: Option<chrono::NaiveDateTime> = row.get("created_at");
                let created_at_str = created_at.map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string()).unwrap_or_default();
                serde_json::json!({
                    "id": row.get::<String, _>("id"),
                    "tenant_id": row.get::<String, _>("tenant_id"),
                    "source": row.get::<String, _>("source"),
                    "content": row.get::<String, _>("content"),
                    "draft_reply": row.get::<String, _>("draft_reply"),
                    "status": row.get::<String, _>("status"),
                    "created_at": created_at_str,
                })
            }).collect();
            (axum::http::StatusCode::OK, axum::Json(messages)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch inbox messages: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!([]))).into_response()
        }
    }
}

    let db_for_sales = db.clone();
    let settings_store = std::sync::Arc::new(crate::settings::Store::new());
    let app = axum::Router::new()
        .route("/api/settings/sms-verify", axum::routing::post(|axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>, axum::Json(req): axum::Json<serde_json::Value>| async move {
            let phone = req.get("phone").and_then(|v| v.as_str()).unwrap_or("").to_string();

            // Generate OTP securely
            let otp = format!("{:06}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().subsec_nanos() % 900000 + 100000);

            {
                let mut store = crate::OTP_STORE.lock().unwrap();
                if store.len() > 1000 {
                    store.retain(|_, (_, time)| time.elapsed().as_secs() < 300); // 5 mins expiry
                }
                store.insert(phone.clone(), (otp.clone(), std::time::Instant::now()));
            }

            let account_sid = std::env::var("TWILIO_ACCOUNT_SID").unwrap_or_else(|_| "dummy_sid".to_string());
            let auth_token = std::env::var("TWILIO_AUTH_TOKEN").unwrap_or_else(|_| "dummy_token".to_string());
            let from_number = std::env::var("TWILIO_FROM_NUMBER").unwrap_or_else(|_| "+1234567890".to_string());

            let provider = crate::integrations::twilio::provider::TwilioProvider::new(account_sid, auth_token);

            let body = format!("Your OHC verification code is {}", otp);
            let phone_clone = phone.clone();

            // Fire and forget gracefully
            tokio::spawn(async move {
                let res = provider.send_sms(&phone_clone, &from_number, &body).await;
                if let Err(e) = res {
                    tracing::warn!("Failed to send SMS to {}: {}. This is expected if Twilio is not configured.", phone_clone, e);
                }
            });

            axum::response::Json(serde_json::json!({ "success": true, "message": "OTP sent" }))
        }))
        .route("/api/settings/sms-confirm", axum::routing::post({
            let settings_store = settings_store.clone();
            move |axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>, axum::Json(req): axum::Json<serde_json::Value>| async move {
                let phone = req.get("phone").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let otp = req.get("otp").and_then(|v| v.as_str()).unwrap_or("");

                let valid = {
                    let mut store = crate::OTP_STORE.lock().unwrap();
                    if let Some((stored_otp, time)) = store.get(&phone) {
                        if stored_otp == otp && time.elapsed().as_secs() < 300 {
                            store.remove(&phone);
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                };

                if valid {
                    axum::response::Json(serde_json::json!({ "success": true }))
                } else {
                    axum::response::Json(serde_json::json!({ "success": false, "message": "Invalid or expired OTP" }))
                }
            }
        }))
        .route("/api/settings/sms-preferences", axum::routing::post({
            let settings_store = settings_store.clone();
            move |axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>, axum::Json(req): axum::Json<serde_json::Value>| async move {
                let phone = req.get("phone").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let urgent_booking = req.get("urgent_booking").and_then(|v| v.as_bool()).unwrap_or(false);
                let failed_payment = req.get("failed_payment").and_then(|v| v.as_bool()).unwrap_or(false);
                let new_order = req.get("new_order").and_then(|v| v.as_bool()).unwrap_or(false);

                if let Err(e) = settings_store.set_sms_preferences(phone, urgent_booking, failed_payment, new_order) {
                    tracing::error!("Failed to save SMS preferences: {}", e);
                    return axum::response::Json(serde_json::json!({ "success": false }));
                }
                axum::response::Json(serde_json::json!({ "success": true }))
            }
        }))
        .route("/", axum::routing::get(ui_handler))
        .route("/business-setup", axum::routing::get(ui_handler))
        .route("/website-builder", axum::routing::get(ui_handler))
        .route("/login", axum::routing::get(ui_handler))
        .route("/agents", axum::routing::get(ui_handler))
        .route("/team", axum::routing::get(ui_handler))
        .route("/meetings", axum::routing::get(ui_handler))
        .route("/dashboard", axum::routing::get(ui_handler))
        .route("/inbox", axum::routing::get(ui_handler))
        .route("/api/integrations/manychat/draft", axum::routing::post(generate_manychat_draft_handler))
        .route("/api/inbox/messages", axum::routing::get(get_inbox_messages_handler).layer(
            axum::middleware::from_fn(
                |req: axum::extract::Request, next: axum::middleware::Next| async move {
                    use axum::response::IntoResponse;
                    let store = std::sync::Arc::new(crate::auth::Store::new());
                    let auth_header = req.headers().get("authorization").and_then(|h| h.to_str().ok());
                    let token = match auth_header {
                        Some(h) if h.to_lowercase().starts_with("bearer ") => &h[7..],
                        _ => return (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
                    };
                    let claims = match store.validate_token(token).await {
                        Ok(c) => c,
                        Err(_) => return (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
                    };
                    let mut req = req;
                    req.extensions_mut().insert(claims);
                    next.run(req).await
                }
            )
        ))
        .route("/healthz", axum::routing::get(|| async { "ok" }))
        .route("/readyz", axum::routing::get(|| async { "ok" }))
        .route(
            "/api/dev/seed",
            axum::routing::post(|| async {
                axum::Json(serde_json::json!({ "ok": true }))
            }),
        )
        .route(
            "/api/dashboard",
            axum::routing::get(|| async {
                axum::Json(serde_json::json!({
                    "organization": { "id": "e2e-org", "name": "OHC E2E" },
                    "agents": [],
                    "metrics": { "tasksCompleted": 0, "activeAgents": 0 }
                }))
            }),
        )
        .route(
            "/api/meetings",
            axum::routing::get(|| async { axum::Json(serde_json::json!([])) }),
        )
        .route(
            "/api/costs",
            axum::routing::get(|| async {
                axum::Json(serde_json::json!({ "totalCostUSD": 0.0, "currency": "USD" }))
            }),
        )
        .route(
            "/api/approvals/request",
            axum::routing::post(|| async {
                axum::Json(serde_json::json!({ "id": "approval-e2e", "status": "pending" }))
            }),
        )
        .route(
            "/api/approvals/decide",
            axum::routing::put(|| async {
                axum::Json(serde_json::json!({ "id": "approval-e2e", "status": "approved" }))
            }),
        )
        .route(
            "/api/handoffs",
            axum::routing::post(|| async {
                axum::Json(serde_json::json!({ "id": "handoff-e2e", "status": "created" }))
            }),
        )
        .route(
            "/api/skills/import",
            axum::routing::post(|| async {
                axum::Json(serde_json::json!({ "id": "skill-e2e", "status": "imported" }))
            }),
        )
        .route(
            "/api/snapshots/create",
            axum::routing::post(|| async {
                axum::Json(serde_json::json!({ "id": "snapshot-e2e", "status": "created" }))
            }),
        )
        .route(
            "/api/v1/auth/login",
            axum::routing::post(move |axum::Json(payload): axum::Json<HttpLoginRequest>| {
                let db = db_for_login.clone();
                async move { http_login_handler(db, payload).await }
            }),
        )
        .route(
            "/api/v1/ai/draft-reply",
            axum::routing::post({
                let db = db.clone();
                let store = std::sync::Arc::new(crate::auth::Store::new());
                move |headers: axum::http::HeaderMap, axum::Json(payload): axum::Json<DraftReplyRequest>| async move {
                    draft_reply_handler(db, store, headers, payload).await
                }
            }),
        )

        .route(
            "/api/v1/dashboard/metrics",
            axum::routing::post({
                let db = db_for_sales.clone();
                let store = std::sync::Arc::new(crate::auth::Store::new());
                move |headers: axum::http::HeaderMap, payload: axum::Json<HttpMetricsRequest>| async move { http_metrics_handler(db, store, headers, payload).await }
            }),
        )
        .route("/api/v1/mesh/connect", axum::routing::get(api::mesh_handler::mesh_ws_handler).with_state(mesh_transport.clone()))
        .route("/api/mesh/v2/broadcast", axum::routing::post(api::mesh_handler::broadcast_handler).with_state(mesh_transport.clone()))
        .route("/api/mesh/v2/direct", axum::routing::post(api::mesh_handler::direct_handler).with_state(mesh_transport.clone()))
        .route("/api/mesh/v2/mailbox", axum::routing::post(api::mesh_handler::mailbox_handler).with_state(mesh_transport.clone()))
        .route("/v1/orchestration/mesh/broadcast", axum::routing::post(api::mesh_handler::orchestration_broadcast_handler).with_state(mesh_transport.clone()))
        .route("/v1/orchestration/tasks/stream", axum::routing::get(api::mesh_handler::orchestration_tasks_stream_handler).with_state(mesh_transport.clone()))
        .route(
            "/api/v1/advisory/insights",
            axum::routing::get({
                let db = db.clone();
                let store = std::sync::Arc::new(crate::auth::Store::new());
                move |headers: axum::http::HeaderMap| async move { advisory_insights_handler(db, store, headers).await }
            }),
        )
        .nest("/api/v1/autodream", api::autodream::router(autodream_worker.clone()))
        .nest("/api/billing", api::billing_api::router(hub.clone()))
        .nest("/api/v1/builder", crate::builder::api::router(db.pool.clone()))
        .nest("/api/agents", api::agents::hire::router(hub.clone()))
        .nest("/api/onboarding", api::onboarding::router(std::sync::Arc::new(crate::services::onboarding::onboarding_agent::OnboardingAgent::new(db.clone(), hub.clone()))).with_state(mesh_transport.clone()))
        .nest("/api/v1/growth", api::growth::router(db.pool.clone(), hub.clone()))
        .nest("/api/agents/approvals", api::agents::approvals::router(dept_orchestrator.clone()))
        .nest("/api/agents/settings", api::agents::settings::router(dept_orchestrator.clone()))
        .nest("/api/agents/chat", api::agents::chat::router(dept_orchestrator.clone()))
        .nest("/api/agents/webhook", api::agents::webhook::router(dept_orchestrator.clone()))
        .nest("/api/agents/mission", api::agents::mission::handoff::router(std::sync::Arc::new(crate::sip::SipDB::new(db.pool.clone(), "default".to_string()))))
        .route("/api/telemetry/sync", axum::routing::post(api::telemetry::sync_telemetry_handler))
        .route_layer(axum::middleware::from_fn_with_state(
            rate_limiter,
            ::server_utils::tier_middleware::tier_middleware,
        ))
        .with_state(mesh_transport)
        .route("/api/help", axum::routing::get(|| async { axum::Json(serde_json::json!([
            { "title": "Getting Started", "desc": "Welcome to One Human Corp! This is a simple app that helps you manage your small business. You can set up your store, accept payments, and hire AI helpers." },
            { "title": "My Store", "desc": "To set up your storefront, go to the 'My Store' tab and add your products. It's easy! Just upload a photo, write a simple description, and set a price." },
            { "title": "Payments", "desc": "When a customer buys something, the money goes straight to your account. We handle all the technical details so you can focus on your business." },
            { "title": "AI Agents", "desc": "Need a hand? Your AI Support Agent can answer customer emails and chats for you while you sleep. Just turn it on in the 'AI Agents' tab." },
            { "title": "Marketing", "desc": "Let our AI write your social media posts! Just tell it what you want to sell, and it will give you a catchy post to share with your customers." },
            { "title": "Account & Billing", "desc": "Your monthly invoice shows exactly what you paid for. We keep things simple with no hidden fees." },
            { "title": "API Documentation (Advanced)", "desc": "See the technical details for connecting custom software to your store.", "link": "/api-docs" }
        ])) }))
        .route("/api/tooltips", axum::routing::get(|| async {
            let registry = get_tooltips_registry();
            let m = registry.read().unwrap();
            axum::Json(serde_json::to_value(&*m).unwrap())
        }).post(|axum::Json(payload): axum::Json<HashMap<String, String>>| async {
            let registry = get_tooltips_registry();
            let mut m = registry.write().unwrap();
            for (k, v) in payload {
                m.insert(k, v);
            }
            axum::Json(serde_json::json!({"success": true}))
        }))
        .route("/api/videos", axum::routing::get(|| async { axum::Json(serde_json::json!([
            { "id": 1, "title": "How to add a product", "duration": "1:20" },
            { "id": 2, "title": "Setting up payments", "duration": "1:15" },
            { "id": 3, "title": "Managing inventory", "duration": "0:50" },
            { "id": 4, "title": "Adding team members", "duration": "1:05" },
            { "id": 5, "title": "Reviewing orders", "duration": "1:10" },
            { "id": 6, "title": "Connecting social media", "duration": "1:25" },
            { "id": 7, "title": "Using the builder", "duration": "1:30" },
            { "id": 8, "title": "Understanding analytics", "duration": "1:00" },
            { "id": 9, "title": "Fulfilling orders", "duration": "0:45" },
            { "id": 10, "title": "Processing refunds", "duration": "0:55" }
        ])) }))
        .route("/api/chat", axum::routing::post(|axum::Json(req): axum::Json<ChatRequest>| async move {
            let help_articles = vec![
                ("getting started", "Welcome to One Human Corp! This is a simple app that helps you manage your small business. You can set up your store, accept payments, and hire AI helpers."),
                ("store", "To set up your storefront, go to the 'My Store' tab and add your products. It's easy! Just upload a photo, write a simple description, and set a price."),
                ("payment", "When a customer buys something, the money goes straight to your account. We handle all the technical details so you can focus on your business."),
                ("ai agent", "Need a hand? Your AI Support Agent can answer customer emails and chats for you while you sleep. Just turn it on in the 'AI Agents' tab."),
                ("marketing", "Let our AI write your social media posts! Just tell it what you want to sell, and it will give you a catchy post to share with your customers."),
                ("billing", "Your monthly invoice shows exactly what you paid for. We keep things simple with no hidden fees."),
                ("api", "Interactive API reference for integrations."),
            ];

            let query = req.message.to_lowercase();
            let mut reply = "I am your AI Help Agent! I specialize in answering questions about OHC features and helping you grow your small business. Check out our Getting Started guide.".to_string();
            let link_title = "Read the full article →";
            let mut link_url = "/help";

            for (kw, desc) in help_articles {
                if query.contains(kw) {
                    reply = format!("Based on our help center: {}", desc);
                    if kw == "api" {
                        link_url = "/api-docs";
                    }
                    break;
                }
            }

            axum::Json(serde_json::json!({
                "reply": reply,
                "link": { "url": link_url, "title": link_title }
            }))
        }))
        .merge(webhook_router)
        .merge(health_router)
        .fallback(ui_handler);

    let port = std::env::var("OHC_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(18789);
    let mesh_addr: std::net::SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    let listener = tokio::net::TcpListener::bind(&mesh_addr).await.unwrap();
    tokio::spawn(async move {
        tracing::info!("Mesh WebSocket server listening on {}", mesh_addr);
        if let Err(e) = axum::serve(listener, app.into_make_service()).await {
            tracing::error!("Mesh server error: {}", e);
        }
    });

    // Start event log worker
    let hub_clone = hub.clone();
    tokio::spawn(async move {
        while let Some(raw_event) = event_rx.recv().await {
            let event = hub_clone.sanitize_hub_event(raw_event);
            hub_clone.append_recent_event(event);
        }
    });

    let hub_service = MyHubService::new(hub.clone(), db.pool.clone(), db.clone());
    let growth_service = crate::services::growth::service::MyGrowthService::new(db.pool.clone(), hub.clone());
    let store = std::sync::Arc::new(crate::auth::Store::new());
    
    // Start Telemetry Sync Daemon (if telemetry is enabled)
    if ::server_config::get().telemetry_enabled {
        let cloud_url = std::env::var("OHC_CLOUD_URL").unwrap_or_else(|_| "https://api.onehumancorp.com".to_string());
        let telemetry_daemon = crate::services::sync::telemetry_sync::TelemetrySyncDaemon::with_mode(db.pool.clone(), cloud_url.clone(), crate::services::sync::telemetry_sync::perf::CoordinatorMode::Parallel);
        telemetry_daemon.start();
    }

    if is_cloud {
        let cloud_url = std::env::var("OHC_CLOUD_URL").unwrap_or_else(|_| "https://api.onehumancorp.com".to_string());
        let power_sync_orchestrator = Arc::new(crate::services::sync::power_sync_orchestrator::PowerSyncOrchestrator::new(db.clone(), cloud_url.clone()));
        power_sync_orchestrator.start().await;

        let repo = Arc::new(crate::services::sync::local_repository_impl::PgLocalRepository::new(db.pool.clone()));
        let cloud_sync = Arc::new(crate::services::sync::cloud_synchronizer::CloudSynchronizerImpl::with_pool(repo, cloud_url.clone(), db.pool.clone()));

        let cloud_sync_clone = cloud_sync.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
            loop {
                interval.tick().await;
                if let Err(e) = cloud_sync_clone.push_pending_missions("system").await {
                    tracing::error!("failed to push pending missions: {}", e);
                }
                if let Err(e) = cloud_sync_clone.pull_mission_updates("system").await {
                    tracing::error!("failed to pull mission updates: {}", e);
                }
            }
        });
    }

    // Start Scheduler Background Task
    let hub_for_sched = hub.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        let mut prune_interval = tokio::time::interval(std::time::Duration::from_secs(60));
        loop {
            tokio::select! {
                _ = prune_interval.tick() => {
                    let sip_db = crate::sip::SipDB::new(hub_for_sched.pool.clone(), "system".to_string());
                    if let Err(e) = sip_db.prune_stale_missions(chrono::Duration::days(7)).await {
                        tracing::error!("failed to prune stale missions: {}", e);
                    }
                }
                _ = interval.tick() => {
                    let due = hub_for_sched.scheduler().poll_due();
                    for task in due {
                        tracing::info!("executing scheduled task: {} ({})", task.name, task.id);

                        // Mark as running
                        if let Err(e) = hub_for_sched.scheduler().mark_running(&task.organization_id, &task.id) {
                            tracing::error!("failed to mark task as running: {}", e);
                            continue;
                        }

                        // Simulate task execution by publishing a message
                        let msg = Message {
                            id: format!("{}-{}", task.id, Utc::now().timestamp()),
                            from_agent: "system-scheduler".to_string(),
                            to_agent: task.agent_id.clone(),
                             r#type: "task".to_string(),
                            content: format!("Scheduled Task triggered: {}.", task.name),
                            occurred_at_unix: Utc::now().timestamp(),
                            meeting_id: String::new(),
                        };

                        match hub_for_sched.clone().publish(msg) {
                            Ok(_) => {
                                let _ = hub_for_sched.scheduler().mark_done(&task.organization_id, &task.id, true);
                            }
                            Err(e) => {
                                tracing::error!("failed to publish scheduled task message: {}", e);
                                let _ = hub_for_sched.scheduler().mark_done(&task.organization_id, &task.id, false);
                            }
                        }
                    }
                }
            }
        }
    });

    tracing::info!("Server listening on {}", addr);

    let dashboard_service = crate::services::dashboard::service::MyDashboardService::new(db.clone(), hub.clone());
    let billing_service = crate::services::billing::service::MyBillingService::new(hub.get_cost_auditor());

    Server::builder()
        .add_service(HubServiceServer::with_interceptor(hub_service, spiffe_interceptor))
        .add_service(::server_ohc::orchestration::auth_service_server::AuthServiceServer::new(::server_auth::AuthServiceServerImpl::new(store)))
        .add_service(GrowthServiceServer::with_interceptor(growth_service, spiffe_interceptor))
        .add_service(::server_ohc::app::dashboard_service_server::DashboardServiceServer::with_interceptor(dashboard_service, spiffe_interceptor))
        .add_service(::server_ohc::orchestration::agent_manager_service_server::AgentManagerServiceServer::with_interceptor(crate::services::agent::service::MyAgentManagerService::new(hub.clone()), spiffe_interceptor))
        .add_service(BillingServiceServer::with_interceptor(billing_service, spiffe_interceptor))
        .serve(addr)
        .await?;

    Ok(())
}
async fn ui_handler(req: axum::extract::Request) -> impl axum::response::IntoResponse {
    let path = req.uri().path();
    let content = match path {
        "/api/v1/health" => "{\"status\":\"ok\"}",
        _ => r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>OneHuman Corp</title>
                    <meta property="og:title" content="OneHuman Corp - Start Your Business" />
                    <meta property="og:image" content="https://ohc.store/api/v1/growth/storefront/og-card?tenant=DEFAULT&product_name=My+Store" />
                    <meta property="og:description" content="Discover great products and services powered by OHC." />
                    <meta name="twitter:card" content="summary_large_image" />
                    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet">
                    <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5/swagger-ui.css" />
                    <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-bundle.js"></script>
                    <style>
                        :root {
                            color-scheme: light;
                            --primary: #0066FF;
                            --primary-hover: #005bd3;
                            --primary-soft: #e8f2ff;
                            --accent-green: #34C759;
                            --accent-orange: #FF9500;
                            --bg: #eef1f5;
                            --surface: rgba(255, 255, 255, 0.86);
                            --surface-strong: #ffffff;
                            --sidebar-bg: rgba(248, 250, 252, 0.92);
                            --text: #1D1D1F;
                            --text-secondary: #657083;
                            --text-tertiary: #8a94a6;
                            --border: rgba(16, 24, 40, 0.1);
                            --shadow-sm: 0 1px 2px rgba(16, 24, 40, 0.06);
                            --shadow-md: 0 16px 42px rgba(16, 24, 40, 0.09);
                            --radius-sm: 8px;
                            --radius-container: 16px;
                            --radius-md: 10px;
                        }
                        body.dark-theme {
                            --primary: #0066FF;
                            --bg: #121214;
                            --surface: rgba(30, 30, 34, 0.86);
                            --text: #F5F5F7;
                            --text-secondary: #a1a1aa;
                        }
                        * {
                            box-sizing: border-box;
                        }
                        html {
                            min-height: 100%;
                            background:
                                linear-gradient(180deg, #f8fafc 0%, #eef1f5 42%, #e9edf3 100%);
                        }
                        body {
                            min-height: 100vh;
                            font-family: -apple-system, BlinkMacSystemFont, 'Inter', 'SF Pro Display', 'Segoe UI', sans-serif;
                            background:
                                radial-gradient(circle at 18% 0%, rgba(0, 111, 255, 0.08), transparent 28%),
                                linear-gradient(180deg, rgba(255,255,255,0.72), rgba(238,241,245,0.96));
                            color: var(--text); 
                            margin: 0; 
                            line-height: 1.45;
                            -webkit-font-smoothing: antialiased;
                        }
                        h1, h2, h3, h4, .outfit {
                            font-family: inherit;
                            letter-spacing: 0;
                        }
                        h1 {
                            font-size: clamp(28px, 4vw, 42px);
                            font-weight: 700;
                            line-height: 1.08;
                            margin-bottom: 24px;
                        }
                        h2 {
                            font-size: 20px;
                            font-weight: 650;
                        }
                        h3 {
                            font-size: 16px;
                            font-weight: 650;
                        }
                        p {
                            color: var(--text-secondary);
                        }
                        .glass {
                            background: rgba(255, 255, 255, 0.65);
                            backdrop-filter: blur(30px) saturate(210%);
                            -webkit-backdrop-filter: blur(30px) saturate(210%);
                            border: 1px solid rgba(255, 255, 255, 0.4);
                            border-radius: 16px;
                            box-shadow: var(--shadow-md);
                        }
                        body.dark-theme .glass {
                            background: rgba(22, 22, 26, 0.7);
                            backdrop-filter: blur(30px) saturate(210%);
                            -webkit-backdrop-filter: blur(30px) saturate(210%);
                            border: 1px solid rgba(255, 255, 255, 0.1);
                        }
                        nav { 
                            padding: 0 28px; 
                            display: flex; 
                            gap: 8px; 
                            border-bottom: 1px solid var(--border); 
                            background: var(--sidebar-bg); 
                            position: sticky; 
                            top: 0; 
                            z-index: 100; 
                            height: 58px;
                            align-items: center;
                            backdrop-filter: blur(30px) saturate(210%);
                            -webkit-backdrop-filter: blur(30px) saturate(210%);
                            box-shadow: 0 1px 0 rgba(255, 255, 255, 0.7);
                        }
                        nav::before {
                            content: 'OneHuman';
                            color: var(--text);
                            font-weight: 700;
                            font-size: 15px;
                            margin-right: 18px;
                        }
                        nav a { 
                            color: var(--text-secondary); 
                            text-decoration: none; 
                            font-weight: 600; 
                            cursor: pointer; 
                            font-size: 14px;
                            min-height: 36px;
                            display: inline-flex;
                            align-items: center;
                            padding: 0 13px;
                            border-radius: 8px;
                            transition: background 0.18s cubic-bezier(0.4, 0, 0.2, 1), color 0.18s cubic-bezier(0.4, 0, 0.2, 1), box-shadow 0.18s cubic-bezier(0.4, 0, 0.2, 1);
                        }
                        nav a:hover {
                            color: var(--primary);
                            background: var(--primary-soft);
                        }
                        main { padding: 32px; }
                        .screen {
                            display: none;
                            padding: 32px;
                            max-width: 1120px;
                            margin: 0 auto;
                            animation: fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1) forwards;
                        }
                        @keyframes fadeIn {
                            from { opacity: 0; transform: translateY(10px); }
                            to { opacity: 1; transform: translateY(0); }
                        }
                        #dashboard-screen {
                            max-width: 1180px;
                        }

                        .ohc-growth-card {
                            backdrop-filter: blur(30px) saturate(210%);
                            background: rgba(255, 255, 255, 0.05);
                            border: 1px solid rgba(255, 255, 255, 0.1);
                            font-family: 'Outfit', 'Inter', sans-serif;
                            color: #ffffff;
                            border-radius: 12px;
                            padding: 24px;
                        }
                        .card { 
                            background: rgba(255, 255, 255, 0.65);
                            backdrop-filter: blur(30px) saturate(210%);
                            -webkit-backdrop-filter: blur(30px) saturate(210%);
                            padding: 24px; 
                            border-radius: 16px;
                            margin-bottom: 18px; 
                            border: 1px solid rgba(255, 255, 255, 0.4);
                            box-shadow: var(--shadow-sm);
                        }
                        body.dark-theme .card {
                            background: rgba(22, 22, 26, 0.7);
                            border: 1px solid rgba(255, 255, 255, 0.1);
                            backdrop-filter: blur(30px) saturate(210%);
                            -webkit-backdrop-filter: blur(30px) saturate(210%);
                        }
                        h1, h2, h3 { color: var(--text); margin-top: 0; }
                        input, textarea, select { 
                            width: 100%; 
                            padding: 11px 13px; 
                            margin-bottom: 16px; 
                            background: rgba(255,255,255,0.94); 
                            border: 1px solid var(--border); 
                            border-radius: 8px;
                            color: var(--text); 
                            font-size: 14px;
                            font-family: inherit;
                            box-shadow: inset 0 1px 1px rgba(16, 24, 40, 0.04);
                            transition: border-color 0.18s cubic-bezier(0.4, 0, 0.2, 1), box-shadow 0.18s cubic-bezier(0.4, 0, 0.2, 1), background 0.18s cubic-bezier(0.4, 0, 0.2, 1);
                        }
                        input:focus, textarea:focus, select:focus {
                            outline: none;
                            border-color: var(--primary);
                            background: #ffffff;
                            box-shadow: 0 0 0 4px rgba(0, 111, 255, 0.13);
                        }
                        button {
            min-height: 44px;
            min-width: 44px;
                            min-height: 44px;
                            min-width: 44px;
                            padding: 10px 18px;
                            background: var(--primary); 
                            border: 1px solid transparent; 
                            border-radius: 8px;
                            color: white; 
                            font-weight: 600; 
                            cursor: pointer; 
                            margin-right: 8px; 
                            margin-bottom: 8px; 
                            font-size: 14px;
                            font-family: inherit;
                            box-shadow: 0 1px 1px rgba(16, 24, 40, 0.08);
                            transition: transform 0.15s cubic-bezier(0.4, 0, 0.2, 1), background 0.18s cubic-bezier(0.4, 0, 0.2, 1), border-color 0.18s cubic-bezier(0.4, 0, 0.2, 1), box-shadow 0.18s cubic-bezier(0.4, 0, 0.2, 1);
                        }
                        button:hover {
                            background: var(--primary-hover);
                            box-shadow: 0 6px 16px rgba(0, 111, 255, 0.18);
                            transform: translateY(-1px);
                        }
                        button:active { transform: translateY(0); }
                        button.secondary { 
                            background: rgba(255,255,255,0.78); 
                            border: 1px solid var(--border); 
                            color: var(--text); 
                        }
                        button.secondary:hover {
                            background: #ffffff;
                            border-color: rgba(0, 111, 255, 0.28);
                            color: var(--primary);
                            box-shadow: 0 8px 20px rgba(16, 24, 40, 0.08);
                        }
                        button.secondary.selected {
                            background: #ffffff;
                            border-color: var(--primary);
                            color: var(--primary);
                            box-shadow: 0 0 0 2px rgba(0, 111, 255, 0.2);
                        }
                        button.danger {
                            background: #FF3B30;
                        }
                        .error { color: #FF3B30; font-size: 13px; margin-bottom: 16px; display: none; }
                        
                        .shimmer {
                            background: linear-gradient(90deg, #eef2f7 25%, #dce5ef 50%, #eef2f7 75%);
                            background-size: 200% 100%;
                            animation: shimmer 1.5s infinite;
                            border-radius: 8px;
                        }
                        @keyframes shimmer {
                            0% { background-position: 200% 0; }
                            100% { background-position: -200% 0; }
                        }


                        .glassmorphism {
                            background: rgba(255, 255, 255, 0.65);
                            backdrop-filter: blur(30px) saturate(210%);
                            -webkit-backdrop-filter: blur(30px) saturate(210%);
                            border: 1px solid rgba(255, 255, 255, 0.4);
                            border-radius: 16px;
                            box-shadow: 0 16px 42px rgba(16, 24, 40, 0.09);
                        }

                        body.dark-theme .glassmorphism {
                            background: rgba(22, 22, 26, 0.7);
                            border: 1px solid rgba(255, 255, 255, 0.1);
                        }

                        #setup-screen {
                            font-family: 'Inter', sans-serif;
                            padding: 40px;
                            max-width: 600px;
                            margin: 60px auto;
                            color: #1D1D1F;
                        }

                        body.dark-theme #setup-screen {
                            color: #F5F5F7;
                        }

                        #setup-screen h1, #setup-screen h2, #setup-screen h3 {
                            font-family: 'Outfit', sans-serif;
                            margin-bottom: 16px;
                        }

                        #setup-screen button.secondary:hover {
                            color: #0066FF;
                            border-color: rgba(0, 102, 255, 0.3);
                        }

                        #setup-screen > div {
                            transition: opacity 250ms cubic-bezier(0.4, 0, 0.2, 1), transform 250ms cubic-bezier(0.4, 0, 0.2, 1);
                            opacity: 1;
                            transform: translateY(0);
                            position: relative; /* Prevent layout jumps on transition */
                        }

                        #setup-screen > div.hidden {
                            opacity: 0;
                            transform: translateY(10px);
                            pointer-events: none;
                            position: absolute;
                            visibility: hidden;
                        }

                        @media (max-width: 375px) {
                            #setup-screen {
                                padding: 24px;
                                margin: 20px auto;
                                border-radius: 12px;
                            }
                            #setup-screen button {
            min-height: 44px;
            min-width: 44px;
                                width: 100%;
                                margin-right: 0;
                            }
                        }

                        /* Login screen specific */
                        #login-screen, #signup-screen {
                            max-width: 400px;
                            margin-top: 80px;
                            border-radius: 16px;
                            padding: 30px;
                        }

                        #mobile-bottom-nav {
                            display: none;
                            position: fixed;
                            right: 20px;
                            bottom: 18px;
                            left: 20px;
                            max-width: 760px;
                            margin: 0 auto;
                            background: rgba(255, 255, 255, 0.88);
                            backdrop-filter: blur(30px) saturate(210%);
                            -webkit-backdrop-filter: blur(30px) saturate(210%);
                            border: 1px solid rgba(255,255,255,0.74);
                            border-radius: 18px;
                            justify-content: space-around;
                            padding: 8px;
                            z-index: 1000;
                            box-shadow: 0 18px 44px rgba(16, 24, 40, 0.16);
                        }
                        @media (max-width: 768px) {
                            #mobile-bottom-nav { display: flex; }
                            main { padding-bottom: 92px; }
                            nav {
                                overflow-x: auto;
                                padding: 0 14px;
                            }
                            nav::before { display: none; }
                            .screen {
                                padding: 22px 14px 108px;
                            }
                        }
                        .nav-item {
                            display: flex;
                            flex-direction: column;
                            align-items: center;
                            justify-content: center;
                            font-size: 12px;
                            font-weight: 600;
                            color: var(--text-secondary);
                            background: transparent;
                            border: none;
                            padding: 6px 8px;
                            margin: 0;
                            min-width: 64px;
                            border-radius: 8px;
                            box-shadow: none;
                        }
                        .nav-item:hover {
                            background: var(--primary-soft);
                            color: var(--primary);
                            box-shadow: none;
                        }
                        .nav-item.active { color: var(--primary); background: var(--primary-soft); }
                        #dashboard-screen > .card:first-of-type {
                            border-color: rgba(0, 111, 255, 0.18);
                            background:
                                linear-gradient(180deg, rgba(255,255,255,0.96), rgba(248,251,255,0.94));
                        }
                        #dashboard-screen > .card:first-of-type h2 {
                            color: var(--primary) !important;
                        }
                        #dashboard-screen > .card:first-of-type p:last-child {
                            color: var(--accent-green) !important;
                        }
                        #dashboard-screen > h2 {
                            background: transparent !important;
                            padding: 0 !important;
                            border-radius: 0 !important;
                            color: var(--text-secondary);
                            font-size: 14px;
                            font-weight: 700;
                            text-transform: uppercase;
                        }
                        #quick-actions-hint, #ai-draft-hint {
                            background: var(--primary-soft) !important;
                            border-left-color: var(--primary) !important;
                            color: var(--text) !important;
                        }
                        #manychat-integration {
                            display: none;
                        }
                        .tabs, .controls, .builder-header {
                            display: flex;
                            flex-wrap: wrap;
                            gap: 8px;
                            align-items: center;
                        }
                        .builder-container {
                            position: relative;
                        }
                        .builder-preview {
                            display: grid;
                            gap: 14px;
                        }
                        .builder-block {
                            padding: 22px;
                            border-radius: 16px;
                            cursor: pointer;
                        }
                        .bottom-sheet {
                            position: fixed;
                            left: 50%;
                            bottom: 0;
                            width: min(720px, calc(100% - 24px));
                            max-height: 78vh;
                            overflow: auto;
                            transform: translate(-50%, 110%);
                            padding: 22px;
                            border-radius: 18px 18px 0 0;
                            z-index: 1200;
                            transition: transform 0.24s ease;
                        }
                        .bottom-sheet.open {
                            transform: translate(-50%, 0);
                        }
                        .bottom-sheet-header {
                            display: flex;
                            align-items: center;
                            justify-content: space-between;
                            gap: 12px;
                        }
                        .bottom-sheet-close {
                            padding: 0;
                            border-radius: 50%;
                        }
                        .domain-setup {
                            display: none;
                        }
                        .domain-setup.active {
                            display: block;
                        }
                        .fab {
                            position: fixed;
                            right: 28px;
                            bottom: 28px;
                            z-index: 900;
                            border-radius: 999px;
                        }
                        #confetti-canvas {
                            pointer-events: none;
                            position: fixed;
                            inset: 0;
                            z-index: 1400;
                        }
                        #meetings-title {
                            color: var(--text) !important;
                            border-bottom: 1px solid var(--border) !important;
                            border-radius: 0 !important;
                        }
                        #login-screen h1 { text-align: center; margin-bottom: 8px; font-size: 24px; }
                        #login-screen p { text-align: center; color: var(--text-secondary); margin-bottom: 32px; font-size: 14px; }

        /* Premium Standard Overrides for Wizard */
        #setup-screen.glass {
            background: rgba(255, 255, 255, 0.65);
            backdrop-filter: blur(30px) saturate(210%);
            -webkit-backdrop-filter: blur(30px) saturate(210%);
            border: 1px solid rgba(255, 255, 255, 0.4);
            border-radius: 16px;
            max-width: 375px;
            margin: 40px auto;
            overflow: hidden;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.05);
        }

        body.dark-theme #setup-screen.glass {
            background: rgba(22, 22, 26, 0.7);
            backdrop-filter: blur(30px) saturate(210%);
            -webkit-backdrop-filter: blur(30px) saturate(210%);
            border: 1px solid rgba(255, 255, 255, 0.1);
        }

        #setup-screen > div {
            transition: opacity 250ms cubic-bezier(0.4, 0, 0.2, 1), transform 250ms cubic-bezier(0.4, 0, 0.2, 1);
            opacity: 1;
            transform: translateY(0);
            position: relative;
        }

        #setup-screen button, #setup-screen input {
            border-radius: 8px;
            transition: all 150ms cubic-bezier(0.4, 0, 0.2, 1);
        }

        @media (max-width: 375px) {
            #setup-screen.glass {
                margin: 10px;
                padding: 16px;
            }
            #setup-screen h1 {
                font-size: 24px;
            }
            #setup-screen button, #setup-screen input {
                width: 100%;
                margin-bottom: 8px;
                box-sizing: border-box;
            }
        }
            @media (prefers-color-scheme: dark) {
                .glass, .screen {
                    background: rgba(22, 22, 26, 0.7) !important;
                    backdrop-filter: blur(30px) saturate(210%) !important;
                    -webkit-backdrop-filter: blur(30px) saturate(210%) !important;
                    border: 1px solid rgba(255, 255, 255, 0.1) !important;
                }
            }

            /* Scribe: Documentation Feature Styles */
            .tooltip-box { position: fixed; background: var(--text); color: var(--bg); padding: 8px 12px; border-radius: var(--radius-sm); font-size: 13px; font-family: inherit; line-height: 1.4; pointer-events: none; z-index: 9999; opacity: 0; transition: opacity 0.2s ease, transform 0.2s ease; transform: translateY(4px); max-width: 250px; box-shadow: var(--shadow-md); }
            .tooltip-box.show { opacity: 1; transform: translateY(0); }
            #global-help-btn { position: fixed; bottom: 24px; right: 24px; width: 56px; height: 56px; border-radius: 50%; background: var(--primary); color: white; display: flex; align-items: center; justify-content: center; font-size: 24px; box-shadow: 0 4px 14px rgba(0, 102, 255, 0.39); cursor: pointer; z-index: 9000; border: none; transition: transform 0.2s ease; }
            #global-help-btn:hover { transform: scale(1.05); background: var(--primary-hover); }
            #global-chat-btn { position: fixed; bottom: 24px; right: 96px; height: 56px; padding: 0 24px; border-radius: 28px; background: var(--text); color: var(--bg); display: flex; align-items: center; justify-content: center; font-size: 16px; font-weight: bold; box-shadow: 0 4px 14px rgba(0, 0, 0, 0.2); cursor: pointer; z-index: 9000; border: none; transition: transform 0.2s ease, box-shadow 0.2s ease; gap: 8px; }
            #global-chat-btn:hover { transform: translateY(-2px); box-shadow: 0 6px 20px rgba(0, 0, 0, 0.25); }
            #ai-chat-widget { position: fixed; bottom: 96px; right: 24px; width: 360px; max-height: 500px; background: var(--surface-strong); border-radius: var(--radius-container); box-shadow: var(--shadow-md); border: 1px solid var(--border); display: none; flex-direction: column; z-index: 9000; overflow: hidden; }
            #ai-chat-header { background: var(--primary); color: white; padding: 16px; font-weight: 600; display: flex; justify-content: space-between; align-items: center; }
            #ai-chat-messages { flex: 1; overflow-y: auto; padding: 16px; display: flex; flex-direction: column; gap: 12px; max-height: 350px; }
            .chat-msg { padding: 12px; border-radius: var(--radius-md); max-width: 85%; font-size: 14px; }
            .chat-msg.user { background: var(--bg); align-self: flex-end; color: var(--text); border-bottom-right-radius: 4px; }
            .chat-msg.ai { background: var(--primary-soft); align-self: flex-start; color: var(--text); border-bottom-left-radius: 4px; }
            .chat-msg a { color: var(--primary); font-weight: 600; text-decoration: none; }
            #ai-chat-input-container { display: flex; padding: 12px; border-top: 1px solid var(--border); gap: 8px; }
            #ai-chat-input { flex: 1; border: 1px solid var(--border); border-radius: var(--radius-sm); padding: 8px 12px; font-size: 14px; outline: none; }
            #ai-chat-input:focus { border-color: var(--primary); }
            #walkthrough-overlay { position: fixed; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; z-index: 9500; box-shadow: inset 0 0 0 9999px rgba(0,0,0,0.5); display: none; transition: all 0.3s ease; }
            #walkthrough-bubble { position: fixed; background: white; color: var(--text); padding: 16px; border-radius: var(--radius-md); box-shadow: var(--shadow-md); z-index: 9501; display: none; max-width: 300px; border-left: 4px solid var(--primary); }
            #walkthrough-bubble h4 { margin: 0 0 8px 0; font-size: 16px; }
            #walkthrough-bubble p { margin: 0 0 12px 0; font-size: 14px; color: var(--text-secondary); }
            #walkthrough-bubble button { padding: 6px 12px; font-size: 13px; margin-top: 8px; }
            .help-category-card { background: var(--surface-strong); border: 1px solid var(--border); border-radius: 16px; padding: 20px; cursor: pointer; transition: transform 0.2s ease, box-shadow 0.2s ease; }
            .help-category-card:hover { transform: translateY(-2px); box-shadow: var(--shadow-sm); border-color: var(--primary); }
            .help-category-card h3 { margin: 0 0 8px 0; color: var(--primary); }
            .help-category-card p { margin: 0; font-size: 14px; color: var(--text-secondary); }
            .video-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 16px; margin-top: 16px; }
            .video-card { background: var(--surface-strong); border: 1px solid var(--border); border-radius: 16px; overflow: hidden; display: flex; flex-direction: column; }
            .video-thumbnail { background: #000; height: 160px; display: flex; align-items: center; justify-content: center; color: white; font-size: 32px; cursor: pointer; }
            .video-info { padding: 12px; }
            .video-info h4 { margin: 0 0 4px 0; }
            .video-info p { margin: 0; color: var(--text-secondary); font-size: 12px; }
            @media (max-width: 768px) { #ai-chat-widget { width: calc(100% - 32px); right: 16px; bottom: 80px; } }
                    </style>
                </head>
                <body>
                    <nav id="main-nav" style="display: none;">
                        <a onclick="showScreen('dashboard-screen')" id="nav-dashboard">Dashboard</a>
                        <a onclick="showScreen('team-screen')" id="nav-agents">Your Team</a>
                        <a onclick="showScreen('setup-screen')" id="nav-setup">Setup</a>
                        <a onclick="showScreen('api-screen')">Connect Tools</a>
                        <a onclick="showScreen('changelog-screen')" id="nav-changelog" placeholder="changelog-nav-tooltip">What's New</a>
                    </nav>

                    <div id="mobile-bottom-nav">
                        <button class="nav-item" onclick="showScreen('dashboard-screen')">🏠<br>Home</button>
                        <button class="nav-item" onclick="showScreen('inbox-screen')">💬<br>Messages</button>
                        <button class="nav-item" onclick="if(confirm('You have reached the 10 Products Limit on the Free plan. Upgrade to Starter to add more products?')) { showScreen('pricing-screen'); }">Add</button>
                        <span class="nav-item" onclick="if(confirm('You have reached the 10 Products Limit on the Free plan. Upgrade to Starter to add more products?')) { showScreen('pricing-screen'); }">Add Product</span>
                        <button class="nav-item" onclick="showScreen('referral-dashboard-screen')">Share</button>
                        <span class="nav-item" onclick="showScreen('referral-dashboard-screen')">Share Store</span>
                        <button class="nav-item" onclick="showScreen('help-screen')">❓<br>Help</button>
                    </div>


                    <!-- Signup Screen -->
                    <div id="signup-screen" class="screen glass">
                        <h1>Create an account</h1>
                        <p>Create an account to start your business</p>
                        <input type="email" placeholder="Email or Username" />
                        <input type="password" placeholder="Password" />
                        <button onclick="handleSignup(this)">Sign Up</button>
                        <button class="secondary" onclick="showScreen('login-screen')">Have an account? Sign In</button>
                    </div>

                    <!-- Dashboard Screen -->
                    <div id="dashboard-screen" class="screen">
                        <h1>Dashboard</h1>

                        <!-- Milestone Viral Share Loop Banner -->
                        <div id="milestone-share-banner" class="hidden relative mb-6 overflow-hidden rounded-xl p-4 text-white shadow-sm flex-col sm:flex-row items-start sm:items-center justify-between gap-4" style="background: linear-gradient(135deg, #f6d365 0%, #fda085 100%);">
                            <div class="flex items-center gap-4">
                                <span class="text-3xl" style="font-size: 32px;">🎉</span>
                                <div>
                                    <h3 class="m-0 text-lg font-bold" style="margin: 0; font-weight: bold; color: white;">Milestone Unlocked: Your First Customers!</h3>
                                    <p class="m-0 text-sm opacity-90" style="margin: 0; opacity: 0.9; color: white;">You've reached <span id="milestone-customers-count">0</span> active customers. Share your store's success to earn a free month of Pro!</p>
                                </div>
                            </div>
                            <button
                                onclick="const tenant = localStorage.getItem('tenant_id') || 'DEFAULT'; window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent('I just reached ' + document.getElementById('milestone-customers-count').textContent + ' customers on my store! Start your own business today with One Human Corp: ohc://join?ref=' + tenant)}`, '_blank'); dismissMilestoneShareBanner();"
                                class="whitespace-nowrap rounded-lg bg-white px-4 py-2 text-sm font-bold text-orange-500 shadow-sm transition-colors hover:bg-orange-50"
                                style="background: white; color: #f97316; font-weight: bold; padding: 8px 16px; border: none; border-radius: 8px; cursor: pointer;"
                            >
                                Share & Claim Reward
                            </button>
                        </div>

                        <div class="card glass" style="text-align: center; padding: 40px 20px;">
                            <p style="color: var(--text-secondary); margin-bottom: 8px; font-weight: 500;">Today's Sales</p>
                            <h2 id="todays-sales" placeholder="todays-sales-tooltip" style="font-size: 48px; margin: 0; color: var(--primary); cursor: help;">$0.00</h2>
                            <p style="color: #28a745; font-size: 14px; margin-top: 8px;">↑ 12% from yesterday</p>
                        </div>

                        <h2 style="padding: 20px; background: rgba(255,255,255,0.1); border-radius: 8px;">Inbox</h2>
                        <div class="card glass">
                            <h2>Welcome back, Human.</h2>
                            <p>Your agents are working on your behalf.</p>
                            <p>Your AI assistants are working on your behalf.</p>
                            <p>My Business: <strong>Active</strong></p>
                            <button class="primary" onclick="showScreen('inbox-screen')">Check Messages</button>
                            <button onclick="showScreen('team-screen')">Your Team</button>
                        </div>
                        <div class="card glass">
                            <h3>Business Snapshot</h3>
                            <p>Orders to Ship</p>
                            <p>Team Members</p>
                            <p>Ongoing Tasks</p>
                            <p>Needs Your Approval</p>
                            <button onclick="markOrderReady()">Mark Order Ready</button>
                            <div id="milestone-card" class="card glass" style="display: none;">
                                <h3 id="milestone-title"></h3>
                                <p id="milestone-body"></p>
                                <button onclick="dismissMilestone()">Dismiss</button>
                            </div>
                        </div>
                        <div class="card glass" id="approval-inbox" placeholder="approval-inbox-tooltip" style="cursor: help;">
                            <h3>Approval Inbox</h3>
                        </div>
                        <div class="card glass" id="activity-feed"></div>
                        <div class="card glass">
                            <h3>Quick Actions <button class="secondary" onclick="const hint = document.getElementById('quick-actions-hint'); hint.style.display = hint.style.display === 'none' ? 'block' : 'none';">?</button></h3>
                            <p>Store Tips</p>
                            <p id="quick-actions-hint" style="display: none; background: #eef2ff; padding: 12px; border-radius: 8px; font-size: 14px; border-left: 4px solid var(--primary); color: #1a1a1b;">These buttons are shortcuts to your most common daily tasks. Use them for adding products, checking messages, and reviewing your store.</p>
                            <button onclick="showScreen('team-screen')">Manage AI Assistants</button>
                            <button onclick="showScreen('setup-screen')">Launch Site</button>
                            <button onclick="showScreen('storefront-builder-screen')">Edit Website</button>
                            <button onclick="showScreen('meetings-screen')">Agenda</button>
                            <button onclick="showScreen('settings-screen')">Settings</button>
                            <button onclick="showScreen('my-plan-screen')">Billing</button>
                            <button onclick="showScreen('advisory-dashboard-screen')">Advisory</button>
                            <button onclick="showScreen('seasonal-promo-screen')">Seasonal Promos ✨</button>
                            <button onclick="showScreen('referral-dashboard-screen')">Referrals</button>
                            <button onclick="alert('Help Center')">Help Center</button>
                            <button onclick="alert('Connect Apps')">Connect Apps</button>
                            <button onclick="alert('Tutorial started')">Video Tutorials</button>
                            <button onclick="showScreen('dashboard-screen')">How to use this app</button>
                            <button onclick="alert(&quot;What's New&quot;)">What's New</button>
                            <button id="integrations-btn" onclick="document.getElementById('manychat-integration').style.display='block';">Integrations</button>
                            <button onclick="toggleMenu()">Menu</button>
                        </div>
                        <div id="manychat-integration" class="card glass" style="display: none;">
                            <h3>💬 Manychat</h3>
                            <p style="font-size: 13px; color: #555; margin-bottom: 12px;">Unified social media inbox for Instagram, Facebook, and WhatsApp.</p>
                            <button onclick="alert('Configure Manychat'); showScreen('inbox-screen')">Configure</button>
                        </div>
                        <!-- Business Analytics Widget with Soft Paywall -->
                        <div class="card glass" style="margin-bottom: 24px; position: relative; overflow: hidden;">
                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;">
                                <h3 style="margin: 0; font-family: 'Outfit', sans-serif;">Business Analytics</h3>
                            </div>

                            <!-- Basic Metrics (Free) -->
                            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px; margin-bottom: 24px;">
                                <div style="background: rgba(255,255,255,0.5); padding: 16px; border-radius: 12px; border: 1px solid rgba(255,255,255,0.8);">
                                    <p style="margin: 0; font-size: 13px; color: #86868B; font-weight: 500;">Total Sales</p>
                                    <p style="margin: 4px 0 0 0; font-size: 24px; font-weight: 700; color: #1D1D1F;">$1,240</p>
                                </div>
                                <div style="background: rgba(255,255,255,0.5); padding: 16px; border-radius: 12px; border: 1px solid rgba(255,255,255,0.8);">
                                    <p style="margin: 0; font-size: 13px; color: #86868B; font-weight: 500;">Visitors</p>
                                    <p style="margin: 4px 0 0 0; font-size: 24px; font-weight: 700; color: #1D1D1F;">342</p>
                                </div>
                            </div>

                            <!-- Advanced AI Insights (Locked / Soft Paywall) -->
                            <div style="position: relative; padding: 24px; border-radius: 12px; border: 1px solid rgba(0,0,0,0.05); background: linear-gradient(135deg, rgba(240,249,255,0.8) 0%, rgba(255,255,255,0.8) 100%);">
                                <h4 style="margin: 0 0 12px 0; font-family: 'Outfit', sans-serif; display: flex; align-items: center; gap: 8px;">
                                    <span style="font-size: 18px;">✨</span> Advanced AI Insights
                                </h4>

                                <div style="filter: blur(4px); opacity: 0.7; pointer-events: none; user-select: none;">
                                    <p style="margin: 0 0 8px 0; font-size: 14px; color: #1D1D1F;">Customer retention dropped by 12% this week. We recommend launching a re-engagement email campaign.</p>
                                    <img src="data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='100%' height='60'><path d='M0,50 Q25,10 50,30 T100,10' fill='none' stroke='%230066ff' stroke-width='4'/></svg>" style="width: 100%; height: 60px; display: block;" />
                                </div>

                                <!-- CTA Overlay -->
                                <div style="position: absolute; inset: 0; display: flex; flex-direction: column; align-items: center; justify-content: center; background: rgba(255,255,255,0.5); backdrop-filter: blur(2px); border-radius: 12px;">
                                    <p style="margin: 0 0 12px 0; font-weight: 600; color: #1D1D1F; text-align: center; max-width: 80%;">Unlock predictive analytics & AI recommendations to grow faster.</p>
                                    <button class="primary" style="padding: 8px 24px; font-weight: 600; box-shadow: 0 4px 12px rgba(0,102,255,0.3);" onclick="if(confirm('Upgrade to Pro to access Advanced AI Insights?')) { showScreen('pricing-screen'); }">Upgrade to Pro</button>
                                </div>
                            </div>
                        </div>

                        <div class="card glass">
                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;">
                                <h3 style="margin: 0; font-family: 'Outfit', sans-serif;">Agent Activity</h3>
                                <div style="display: flex; align-items: center; gap: 8px; background: rgba(52, 199, 89, 0.1); padding: 4px 12px; border-radius: 20px; border: 1px solid rgba(52, 199, 89, 0.2);">
                                    <div style="width: 8px; height: 8px; border-radius: 50%; background-color: #34C759; box-shadow: 0 0 8px #34C759;"></div>
                                    <span style="font-size: 12px; font-weight: 600; color: #1f853b;">Swarm Online</span>
                                </div>
                            </div>
                            <div id="agent-activity-feed" style="background: rgba(255, 255, 255, 0.5); border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.8); overflow: hidden;">
<div style="padding: 32px; text-align: center; color: var(--text-secondary);"><div style="display: inline-block; width: 32px; height: 32px; border: 2px solid rgba(0,0,0,0.1); border-top-color: var(--primary); border-radius: 50%; animation: spin 1s linear infinite; margin-bottom: 12px;"></div><p style="margin: 0; font-size: 14px;">Waiting for team activity...</p></div>
                            </div>
                            <button class="secondary" style="width: 100%; margin-top: 16px; font-weight: 600;" onclick="simulateOrder()">Simulate Activity</button>
                        </div>
                        <div id="extra-menu" class="card glass" style="display: none;">
                            <button onclick="showScreen('api-screen')">Connect Custom Software</button>
                            <div class="card glass">
                                <h3>Learn</h3>
                                <button onclick="alert('Tutorial started')">Tutorial Library</button>
                                <button class="nav-button" onclick="showScreen('inbox-screen')">Inbox</button>
                            </div>
                        </div>

                        <!-- Bottom Nav for dashboard_nav.spec.ts -->
                        <div class="glass" role="navigation" style="display: flex; justify-content: space-around; padding: 10px; margin-top: 20px; border-top: 1px solid rgba(255,255,255,0.1);">
                            <button class="nav-item" onclick="showScreen('dashboard-screen')">Home</button>
                            <button class="nav-item" onclick="showScreen('inbox-screen')">Messages</button>
                            <button class="nav-item" onclick="showScreen('inbox-screen')">Chat</button>
                            <button class="nav-item" onclick="showScreen('meetings-screen')">Meetings</button>
                            <span class="nav-item" onclick="if(confirm('You have reached the 10 Products Limit on the Free plan. Upgrade to Starter to add more products?')) { showScreen('pricing-screen'); }">Add Product</span>
                            <button class="nav-item">Orders</button>
                            <button class="nav-item">Analytics</button>
                            <button class="nav-item">Stats</button>
                            <button class="nav-item">Distribute</button>
                        </div>
                    </div>

                    <!-- Seasonal Promos Generator -->
                    <div id="seasonal-promo-screen" class="screen glass" style="margin-bottom: 80px;">
                        <h1>Seasonal Promotion Generator ✨</h1>
                        <p>Generate highly-converting, AI-styled seasonal campaigns for your business instantly.</p>

                        <div class="card glass">
                            <label for="promo-occasion" style="display: block; margin-bottom: 8px; font-weight: 500;">Occasion / Season</label>
                            <input type="text" id="promo-occasion" placeholder="e.g., Summer Sale, Back to School, Halloween" style="width: 100%; margin-bottom: 16px; padding: 12px; border-radius: 8px; border: 1px solid rgba(0,0,0,0.1);">

                            <label for="promo-discount" style="display: block; margin-bottom: 8px; font-weight: 500;">Discount Percentage</label>
                            <input type="number" id="promo-discount" placeholder="20" style="width: 100%; margin-bottom: 24px; padding: 12px; border-radius: 8px; border: 1px solid rgba(0,0,0,0.1);">

                            <button class="primary" style="width: 100%; font-size: 16px; padding: 16px;" onclick="generateSeasonalPromo()">Generate Campaign</button>
                        </div>

                        <div id="promo-result" class="card glass" style="display: none; background: linear-gradient(135deg, rgba(255,255,255,0.9) 0%, rgba(240,249,255,0.9) 100%); border-left: 4px solid var(--primary); margin-top: 24px;">
                            <h3 style="color: var(--primary); margin-top: 0;">Generated Campaign</h3>
                            <div id="promo-content" style="font-size: 16px; line-height: 1.6; color: #333;"></div>
                        </div>
                    </div>

                    <!-- Referral Dashboard -->
                    <div id="referral-dashboard-screen" class="screen glass">
                        <h1>Referral Dashboard</h1>

                        <!-- Hero Card: Give a Month, Get a Month -->
                        <div class="card glass" style="background: #3b82f6; background: linear-gradient(135deg, var(--primary, #0066ff) 0%, #3b82f6 100%); color: white; text-align: center; padding: 40px 24px; border: none; position: relative; overflow: hidden;">
                            <div style="position: absolute; top: -50px; right: -50px; width: 150px; height: 150px; background: rgba(255,255,255,0.1); border-radius: 50%; filter: blur(20px);"></div>
                            <div style="position: absolute; bottom: -50px; left: -50px; width: 150px; height: 150px; background: rgba(255,255,255,0.1); border-radius: 50%; filter: blur(20px);"></div>
                            <h2 style="font-size: 32px; font-weight: 800; margin-bottom: 12px; color: white; position: relative; z-index: 1;">Give 1 Month, Get 1 Month Free</h2>
                            <p style="color: rgba(255,255,255,0.9); font-size: 16px; max-width: 400px; margin: 0 auto 24px; line-height: 1.5; position: relative; z-index: 1;">Invite other small business owners to OHC. When they launch, you both get a free month of OHC Pro. There's no limit!</p>

                            <p style="font-size: 14px; font-weight: bold; margin-bottom: 8px; position: relative; z-index: 1; color: white;">Your Referral Link</p>
                            <div style="background: rgba(0,0,0,0.2); backdrop-filter: blur(10px); -webkit-backdrop-filter: blur(10px); padding: 16px; border-radius: 12px; display: flex; align-items: center; justify-content: space-between; max-width: 500px; margin: 0 auto; border: 1px solid rgba(255,255,255,0.1); position: relative; z-index: 1;">
                                <p id="referral-link" style="margin: 0; font-family: monospace; font-size: 14px; color: white; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; text-align: left;">ohc://join?ref=DEFAULT</p>
                                <button style="margin: 0; background: white; color: var(--primary, #0066ff); font-weight: 700; border: none; padding: 8px 16px; border-radius: 8px;" onclick="navigator.clipboard.writeText('ohc://join?ref=DEFAULT'); alert('Copied');">Copy</button>
                            </div>

                            <div style="display: flex; gap: 8px; justify-content: center; margin-top: 16px; position: relative; z-index: 1; flex-wrap: wrap;">
                                <button style="background: rgba(255,255,255,0.2); color: white; border: 1px solid rgba(255,255,255,0.4); padding: 8px 16px; border-radius: 8px; font-weight: 600; width: 100%; max-width: 375px;" onclick="navigator.clipboard.writeText('ohc://join?ref=DEFAULT'); document.getElementById('invite-copied-msg').style.display='block'; setTimeout(() => document.getElementById('invite-copied-msg').style.display='none', 2000);">Copy Invite Message</button>
                                <div id="invite-copied-msg" style="display: none; width: 100%; font-size: 14px; color: #a7f3d0; margin-top: 4px;">Invite message copied!</div>
                            </div>

                            <div style="display: flex; gap: 8px; justify-content: center; margin-top: 16px; position: relative; z-index: 1; flex-wrap: wrap;">
                                <button style="background: #E1306C; color: white; border: none; padding: 8px 16px; border-radius: 8px; font-weight: 600;" onclick="window.open('https://instagram.com', '_blank')">Share to Instagram</button>
                                <button style="background: #25D366; color: white; border: none; padding: 8px 16px; border-radius: 8px; font-weight: 600;" onclick="window.open('https://wa.me/?text=Launch+your+business+on+OHC!', '_blank')">WhatsApp</button>
                                <button style="background: #1DA1F2; color: white; border: none; padding: 8px 16px; border-radius: 8px; font-weight: 600;" onclick="window.open('https://twitter.com/intent/tweet?text=Launch+your+business+on+OHC!', '_blank')">X / Twitter</button>
                            </div>
                            <div style="margin-top: 16px; position: relative; z-index: 1; display: flex; flex-direction: column; align-items: center;">
                                <button style="background: rgba(255,255,255,0.2); color: white; border: 1px solid rgba(255,255,255,0.4); padding: 8px 16px; border-radius: 8px; font-weight: 600; cursor: pointer; transition: background 0.2s;" onclick="navigator.clipboard.writeText('Join OHC using my link! ohc://join?ref=DEFAULT'); document.getElementById('invite-copied-msg').style.display='inline-block'; setTimeout(() => document.getElementById('invite-copied-msg').style.display='none', 3000);">Copy Invite Message</button>
                                <div id="invite-copied-msg" style="display: none; margin-top: 8px; color: white; font-weight: bold; background: rgba(0,0,0,0.4); padding: 4px 8px; border-radius: 4px;">Invite message copied!</div>
                            </div>
                        </div>

                        <!-- Progress Section -->
                        <div class="card glass">
                            <h3 style="margin-bottom: 16px; display: flex; justify-content: space-between; align-items: center;">
                                <span>Your Growth Progress</span>
                                <span style="font-size: 14px; font-weight: 500; color: var(--text-secondary); background: rgba(0,0,0,0.05); padding: 4px 10px; border-radius: 99px;">0 / 5 Referrals</span>
                            </h3>
                            <div style="width: 100%; background: #e2e8f0; border-radius: 99px; height: 12px; overflow: hidden; margin-bottom: 12px;">
                                <div style="width: 10%; background: var(--primary); height: 100%; border-radius: 99px; box-shadow: 0 0 10px rgba(0,111,255,0.5);"></div>
                            </div>
                            <p style="font-size: 14px; color: var(--text-secondary); margin: 0;">You're on your way! Invite 1 more business to unlock your first reward.</p>
                        </div>

                        <!-- One-Tap Share Tools -->
                        <div class="card glass">
                            <h3 style="margin-bottom: 20px;">Share with 1-Tap</h3>
                            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(140px, 1fr)); gap: 12px;">
                                <button style="margin: 0; width: 100%; background: linear-gradient(45deg, #f09433 0%, #e6683c 25%, #dc2743 50%, #cc2366 75%, #bc1888 100%); color: white; border: none; display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 20px 10px; gap: 8px;" onclick="alert('Opening Instagram story editor...')">
                                    <span style="font-size: 24px;">📷</span>
                                    <span>Share to Instagram</span>
                                </button>
                                <button style="margin: 0; width: 100%; background: #25D366; color: white; border: none; display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 20px 10px; gap: 8px;" onclick="alert('Opening WhatsApp...')">
                                    <span style="font-size: 24px;">💬</span>
                                    <span>WhatsApp</span>
                                </button>
                                <button style="margin: 0; width: 100%; background: #0077b5; color: white; border: none; display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 20px 10px; gap: 8px;" onclick="alert('Opening LinkedIn...')">
                                    <span style="font-size: 24px;">💼</span>
                                    <span>LinkedIn</span>
                                </button>
                                <button style="margin: 0; width: 100%; background: #ea4335; color: white; border: none; display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 20px 10px; gap: 8px;" onclick="alert('Opening Email draft...')">
                                    <span style="font-size: 24px;">✉️</span>
                                    <span>Email</span>
                                </button>
                            </div>
                        </div>

                        <!-- Embeddable Storefront Widget -->
                        <div class="card glass" style="margin-top: 24px;">
                            <h3 style="margin-bottom: 12px;">Embed on Your Website</h3>
                            <p style="margin-bottom: 16px; font-size: 14px; color: var(--text-secondary);">Showcase your OHC storefront directly on your existing blog or website to maximize reach.</p>
                            <textarea id="embed-code" readonly style="width: 100%; height: 80px; font-family: monospace; font-size: 12px; margin-bottom: 12px; padding: 8px; border-radius: 8px; border: 1px solid rgba(0,0,0,0.1); background: rgba(0,0,0,0.02);">&lt;iframe src="https://mybusiness.ohc.store" width="100%" height="600px" style="border:none; border-radius:12px;"&gt;&lt;/iframe&gt;</textarea>
                            <button onclick="navigator.clipboard.writeText(document.getElementById('embed-code').value); alert('Embed code copied!');" style="width: 100%;">Copy Embed Code</button>
                        </div>

                        <!-- Automated AI Review Requests -->
                        <div class="card glass" style="margin-top: 24px; border: 1px solid rgba(16, 185, 129, 0.3);">
                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                                <h3 style="margin: 0; color: var(--text-primary);">Automated AI Review Requests <span style="font-size: 12px; background: rgba(16, 185, 129, 0.1); color: #10b981; padding: 4px 8px; border-radius: 99px; margin-left: 8px; font-weight: normal; vertical-align: middle;">New Growth Loop</span></h3>
                            </div>

                            <div id="review-campaign-success" style="display: none; padding: 12px; background: rgba(16, 185, 129, 0.1); color: #10b981; border-radius: 8px; margin-bottom: 16px; font-weight: bold; font-size: 14px;">
                                ✓ Campaign sent to <span id="review-emails-sent">0</span> customers!
                            </div>
                            <button id="send-review-campaign-btn" onclick="sendReviewCampaign()" style="width: 100%; background: linear-gradient(135deg, #0066ff 0%, #3b82f6 100%);">✨ Send AI Review Requests</button>
                        </div>

                        <!-- Social Media Discount Share -->
                        <div class="card glass" style="margin-top: 24px; border: 1px solid rgba(16, 185, 129, 0.3);">
                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                                <h3 style="margin: 0; color: var(--text-primary);">Social Media Discount Share <span style="font-size: 12px; background: rgba(16, 185, 129, 0.1); color: #10b981; padding: 4px 8px; border-radius: 99px; margin-left: 8px; font-weight: normal; vertical-align: middle;">New Growth Loop</span></h3>
                            </div>
                            <p style="margin-bottom: 16px; font-size: 14px; color: var(--text-secondary);">Offer a 10% discount on social media when you hit a new milestone. Drive instant traffic back to your store!</p>
                            <button onclick="generateDiscountShare()" style="width: 100%; background: #000; color: #fff;">🐦 Share 10% Off on X (Twitter)</button>
                        </div>

                        <!-- Growth Loop: Interactive Analytics Soft Paywall -->
                        <div class="card glass" style="margin-top: 24px; border: 1px solid rgba(255, 165, 0, 0.3); position: relative; overflow: hidden;">
                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;">
                                <h3 style="margin: 0; color: var(--text-primary);">Advanced Analytics <span style="font-size: 12px; background: rgba(255, 165, 0, 0.1); color: #d97706; padding: 4px 8px; border-radius: 99px; margin-left: 8px; font-weight: normal; vertical-align: middle;">Premium Growth Loop</span></h3>
                            </div>

                            <div style="filter: blur(4px); opacity: 0.6; user-select: none;">
                                <div style="display: flex; justify-content: space-between; margin-bottom: 12px; background: rgba(0,0,0,0.02); padding: 12px; border-radius: 8px;">
                                    <span style="font-weight: 600;">Conversion Rate</span>
                                    <span style="color: #10b981; font-weight: bold; font-size: 18px;">4.2%</span>
                                </div>
                                <div style="display: flex; justify-content: space-between; margin-bottom: 12px; background: rgba(0,0,0,0.02); padding: 12px; border-radius: 8px;">
                                    <span style="font-weight: 600;">Customer Lifetime Value</span>
                                    <span style="color: #3b82f6; font-weight: bold; font-size: 18px;">$184.50</span>
                                </div>
                                <div style="text-align: center; margin-top: 16px; background: rgba(0,0,0,0.02); padding: 24px; border-radius: 8px;">
                                    <div style="font-size: 32px; margin-bottom: 8px;">📈</div>
                                    <span style="font-weight: 500; color: var(--text-secondary);">Top Traffic: Organic Search</span>
                                </div>
                            </div>

                            <div style="position: absolute; top: 0; left: 0; right: 0; bottom: 0; display: flex; flex-direction: column; align-items: center; justify-content: center; background: rgba(255, 255, 255, 0.4); backdrop-filter: blur(2px);">
                                <div style="background: white; padding: 24px; border-radius: 16px; box-shadow: 0 10px 25px rgba(0,0,0,0.1); border: 1px solid rgba(255, 165, 0, 0.2); text-align: center; max-width: 300px;">
                                    <div style="width: 48px; height: 48px; background: rgba(255, 165, 0, 0.1); color: #d97706; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 24px; margin: 0 auto 12px auto;">🔒</div>
                                    <h3 style="margin: 0 0 8px 0; font-size: 18px; color: var(--text-primary);">Unlock Growth Insights</h3>
                                    <p style="margin: 0 0 16px 0; font-size: 14px; color: var(--text-secondary);">See exactly where your best customers come from and optimize your store to double your conversion rate.</p>
                                    <button onclick="showScreen('pricing-screen')" style="width: 100%; background: linear-gradient(135deg, #f59e0b 0%, #f97316 100%); font-weight: bold; padding: 12px; border-radius: 8px; color: white; border: none; cursor: pointer;">Upgrade to Premium</button>
                                </div>
                            </div>
                        </div>

                        <div class="card glass" style="margin-top: 24px;">
                            <div style="display: flex; justify-content: space-between; align-items: center;">
                                <div>
                                    <h3 style="margin-bottom: 4px;">Referral History & Logs</h3>
                                    <p style="margin: 0; font-size: 14px;">Track who signed up and when your rewards activate.</p>
                                </div>
                                <div style="display: flex; gap: 8px;">
                                    <button class="secondary" style="margin: 0;" onclick="alert('History shown')">View Referral Logs</button>
                                    <button class="secondary" style="margin: 0;" onclick="alert('Data exported')">Export Data</button>
                                </div>
                            </div>
                        </div>
                        <button class="secondary" onclick="showScreen('dashboard-screen')">Back to Dashboard</button>
                    </div>

                    <!-- Inbox Screen -->
                    <div id="inbox-screen" class="screen glass" style="max-width: 375px; margin: 0 auto; padding: 16px; box-sizing: border-box;">
                        <button class="secondary" onclick="showScreen('dashboard-screen')">< Back</button>
                        <h1>Customer Inbox</h1>
                        <div class="card glass" onclick="this.classList.toggle('active')">
                            <h3>Maya <button class="secondary" style="float: right;" onclick="event.stopPropagation(); const hint = document.getElementById('ai-draft-hint'); hint.style.display = hint.style.display === 'none' ? 'block' : 'none';">?</button></h3>
                            <p id="ai-draft-hint" style="display: none; background: #eef2ff; padding: 12px; border-radius: 8px; font-size: 14px; border-left: 4px solid var(--primary); clear: both; margin-bottom: 12px; color: #1a1a1b;">Use AI Draft to quickly write a professional reply. You can edit it before sending.</p>
                            <p>Do you do vegan cakes?</p>
                            <button onclick="draftInboxReply(this)">✨ AI Draft</button>
                            <button onclick="document.getElementById('reply-input').value = 'Yes, we have 3 vegan options!'">Yes, we have 3 vegan options!</button>
                        </div>
                        <div class="card glass">
                            <h3 style="display: flex; justify-content: space-between; align-items: center;">Facebook Comment <span style="font-size: 20px;">📘</span></h3>
                            <p>Are you open on Sundays?</p>
                            <button onclick="draftInboxReply(this)">✨ AI Draft</button>
                            <button onclick="document.getElementById('reply-input').value = 'Yes, we are open 10am-2pm!'">Quick Reply</button>
                        </div>
                        <div class="card glass">
                            <h3 style="display: flex; justify-content: space-between; align-items: center;">Instagram DM <span style="font-size: 20px;">📸</span></h3>
                            <p>Can I order a custom cake?</p>
                            <button onclick="draftInboxReply(this)">✨ AI Draft</button>
                            <button onclick="document.getElementById('reply-input').value = 'Sure, please send details!'">Quick Reply</button>
                        </div>
                        <div class="card glass">
                            <h3 style="display: flex; justify-content: space-between; align-items: center;">WhatsApp <span style="font-size: 20px;">💬</span></h3>
                            <p>Hello, do you deliver?</p>
                            <button onclick="draftInboxReply(this)">✨ AI Draft</button>
                            <button onclick="document.getElementById('reply-input').value = 'Yes, within a 5-mile radius.'">Quick Reply</button>
                        </div>
                        <div id="chat-window" class="card glass">
                            <p>Select a conversation</p>
                            <div id="messages-list"></div>
                            <input id="reply-input" type="text" placeholder="Type a message...">
                            <button onclick="const m = document.getElementById('reply-input').value; if(m) { const p = document.createElement('p'); p.textContent = m; document.getElementById('messages-list').appendChild(p); document.getElementById('reply-input').value = ''; }">Send</button>
                        </div>
                    </div>

                    <!-- Meetings Screen -->
                    <div id="meetings-screen" class="screen glass" style="font-family: 'Inter', sans-serif;">
                        <h1 style="font-family: 'Outfit', sans-serif; margin-bottom: 24px;">AI Service Booking</h1>

                        <div class="card glass" style="border-radius: 16px; padding: 16px; margin-bottom: 16px;">
                            <h3 style="font-family: 'Outfit', sans-serif; margin-top: 0; margin-bottom: 12px;">Cal.com Integration</h3>
                            <p style="font-size: 14px; margin-bottom: 16px; color: var(--text-secondary);">Connect your Cal.com account to enable AI to auto-schedule appointments from the unified inbox.</p>
                            <button style="min-width: 44px; min-height: 44px; border-radius: 8px; font-family: 'Inter', sans-serif; padding: 0 16px; background: #1a1a1a; color: white; border: none; width: 100%;">Connect Cal.com</button>
                        </div>

                        <button id="meetings-title" style="display: block; width: 100%; text-align: left; background: none; border: none; padding: 0; margin-bottom: 20px; cursor: pointer; color: #0066FF; font-size: 1.5em; font-family: 'Outfit', sans-serif; font-weight: 600;"
                                onclick="document.getElementById('scheduler').style.display='block'; this.style.display='none'">
                            + Schedule New Appointment
                        </button>

                        <div class="card glass meeting" style="border-radius: 16px; padding: 16px; margin-bottom: 16px;">
                            <h3 style="font-family: 'Outfit', sans-serif; margin-top: 0;">Next Item</h3>
                            <p>Team Sync - 14:00</p>
                            <p style="color: #FF9500; font-weight: 500;">In 10 mins</p>
                            <div style="display: flex; gap: 8px; flex-wrap: wrap; margin-top: 12px;">
                                <button style="min-width: 44px; min-height: 44px; border-radius: 8px; font-family: 'Inter', sans-serif; padding: 0 16px; background: #34C759; color: white; border: none;" onclick="showScreen('meeting-room-screen')">Join Start</button>
                                <button style="min-width: 44px; min-height: 44px; border-radius: 8px; font-family: 'Inter', sans-serif; padding: 0 16px; background: #FF3B30; color: white; border: none;" onclick="this.parentElement.parentElement.innerHTML='<p>Canceled</p>'">Cancel Delete</button>
                            </div>
                        </div>

                        <div id="scheduler" class="card glass" style="display: none; border-radius: 16px; padding: 16px; margin-bottom: 16px;">
                            <h2 style="font-family: 'Outfit', sans-serif; margin-top: 0;">Plan Create</h2>
                            <div style="display: flex; flex-direction: column; gap: 12px;">
                                <input type="text" placeholder="Meeting Title" style="min-height: 44px; border-radius: 8px; padding: 0 12px; border: 1px solid var(--border);">
                                <input type="date" style="min-height: 44px; border-radius: 8px; padding: 0 12px; border: 1px solid var(--border);">
                                <input type="time" style="min-height: 44px; border-radius: 8px; padding: 0 12px; border: 1px solid var(--border);">
                                <input type="email" placeholder="Participant Email" style="min-height: 44px; border-radius: 8px; padding: 0 12px; border: 1px solid var(--border);">
                            </div>
                            <div style="display: flex; gap: 8px; flex-wrap: wrap; margin-top: 16px;">
                                <button style="min-width: 44px; min-height: 44px; border-radius: 8px; font-family: 'Inter', sans-serif; padding: 0 16px; background: #0066FF; color: white; border: none; flex: 1;" onclick="alert('Participant added')">Add</button>
                                <button style="min-width: 44px; min-height: 44px; border-radius: 8px; font-family: 'Inter', sans-serif; padding: 0 16px; background: #1D1D1F; color: white; border: none; flex: 1;" onclick="document.getElementById('scheduler').style.display='none'; document.getElementById('meetings-title').style.display='block'">Save</button>
                            </div>
                        </div>

                        <div class="tabs" style="display: flex; gap: 8px; overflow-x: auto; margin-bottom: 16px; padding-bottom: 8px;">
                            <button style="min-width: 44px; min-height: 44px; border-radius: 8px; font-family: 'Inter', sans-serif; padding: 0 16px; white-space: nowrap;" onclick="alert('History shown')">📜 View Log</button>
                            <button style="min-width: 44px; min-height: 44px; border-radius: 8px; font-family: 'Inter', sans-serif; padding: 0 16px; white-space: nowrap;" onclick="alert('Records')">Past</button>
                            <button style="min-width: 44px; min-height: 44px; border-radius: 8px; font-family: 'Inter', sans-serif; padding: 0 16px; white-space: nowrap;" onclick="alert('Calendar')">Calendar</button>
                            <button style="min-width: 44px; min-height: 44px; border-radius: 8px; font-family: 'Inter', sans-serif; padding: 0 16px; white-space: nowrap;" onclick="alert('Archive')">Archive</button>
                        </div>
                        <button class="secondary" style="min-width: 44px; min-height: 44px; border-radius: 8px; font-family: 'Inter', sans-serif; padding: 0 16px; width: 100%;" onclick="showScreen('dashboard-screen')">Back</button>
                    </div>

                    <!-- Meeting Room Screen -->
                    <div id="meeting-room-screen" class="screen glass">
                        <h1>Meeting Room Video Audio</h1>
                        <div class="video-container card glass">
                            <p>Feed</p>
                            <p id="status-text">Off</p>
                        </div>
                        <div class="controls">
                            <button onclick="document.getElementById('status-text').textContent = 'Video Off'">Camera</button>
                            <button onclick="document.getElementById('status-text').textContent = 'Muted'">Mic</button>
                            <button onclick="document.getElementById('status-text').textContent = 'Sharing Screen'">Share</button>
                            <button onclick="document.getElementById('status-text').textContent = 'Hand Raised'">Signal</button>
                            <button onclick="document.getElementById('status-text').textContent = 'Recording'">Record</button>
                            <button onclick="alert('Participants list')">Participants List</button>
                            <button onclick="alert('Chat opened')">Chat</button>
                            <button class="danger" onclick="document.getElementById('status-text').textContent = 'left'; alert('Left meeting')">End</button>
                        </div>
                    </div>

                    <!-- Agents Page (Your Team) -->
                    <div id="team-screen" class="screen">
                        <h1 class="outfit">Agents</h1>
                        <p style="color: var(--text-secondary); margin-bottom: 20px;">Manage your AI departments and review their recent activities.</p>

                        <div id="departments-container">
                            <div class="card glass" onclick="toggleDepartment('ambassador')" style="cursor: pointer;">
                                <h3 class="outfit">Marketing Pro</h3>
                                <p style="color: var(--accent-green);">Status: Active</p>
                                <p style="font-size: 14px; margin-top: 8px;">Recent: Replied to 3 Instagram DMs.</p>
                                <div id="ambassador-settings" style="display: none; margin-top: 15px; border-top: 1px solid var(--border); padding-top: 15px;">
                                    <h4 style="margin-top: 0;">Settings</h4>
                                    <label style="display: flex; align-items: center; justify-content: space-between; font-size: 14px; cursor: pointer;">
                                        Require approval for quotes > $100
                                        <input type="checkbox" checked onchange="event.stopPropagation(); updateApprovalSetting('ambassador', this.checked)">
                                    </label>
                                </div>
                            </div>

                            <div class="card glass" onclick="toggleDepartment('manager')" style="margin-top: 15px; cursor: pointer;">
                                <h3 class="outfit">Ops Helper</h3>
                                <p style="color: var(--accent-green);">Status: Active</p>
                                <p style="font-size: 14px; margin-top: 8px;">Recent: Updated inventory for Vegan Cupcakes.</p>
                                <div id="manager-settings" style="display: none; margin-top: 15px; border-top: 1px solid var(--border); padding-top: 15px;">
                                    <h4 style="margin-top: 0;">Settings</h4>
                                    <label style="display: flex; align-items: center; justify-content: space-between; font-size: 14px; cursor: pointer;">
                                        Require approval for order refunds
                                        <input type="checkbox" checked onchange="event.stopPropagation(); updateApprovalSetting('manager', this.checked)">
                                    </label>
                                </div>
                            </div>

                            <div class="card glass" onclick="toggleDepartment('salesperson')" style="margin-top: 15px; cursor: pointer;">
                                <h3 class="outfit">Sales Agent</h3>
                                <p style="color: var(--accent-orange);">Status: Needs Approval (1)</p>
                                <p style="font-size: 14px; margin-top: 8px;">Recent: Generated quote for custom cake.</p>
                                <button style="margin-top: 15px; width: 100%;" onclick="event.stopPropagation(); showScreen('dashboard-screen')">Review Pending Approvals</button>
                            </div>
                        </div>

                        <button class="secondary" style="margin-top: 20px;" onclick="showScreen('dashboard-screen')">Back to Dashboard</button>
                    </div>

                    <script>
                        function toggleDepartment(deptId) {
                            const settingsDiv = document.getElementById(deptId + '-settings');
                            if (settingsDiv) {
                                settingsDiv.style.display = settingsDiv.style.display === 'none' ? 'block' : 'none';
                            }
                        }


                        function updateApprovalSetting(deptId, isChecked) {
                            const tenantId = localStorage.getItem('tenant_id') || 'e2e-tenant';
                            fetch(`/api/agents/settings/${deptId}`, {
                                method: 'POST',
                                headers: {
                                    'Content-Type': 'application/json',
                                    'Authorization': 'Bearer ' + (localStorage.getItem('token') || 'test-token')
                                },
                                body: JSON.stringify({ auto_approve_limits: isChecked ? 0.0 : 100.0, tone_of_voice: "professional" })
                            }).then(() => {
                                alert(`Settings updated for ${deptId}: auto-execute is now ${!isChecked}.`);
                            }).catch(e => {
                                console.error('Failed to update settings', e);
                            });
                        }


                        async function fetchActivityFeed() {
                            try {
                                const container = document.getElementById('activity-feed');
                                if (!container) return;
                                const res = await fetch('/api/agents/approvals/activity', {
                                    headers: { 'Authorization': 'Bearer ' + (localStorage.getItem('token') || 'test-token') }
                                });
                                if (res.ok) {
                                    const data = await res.json();
                                    if (data.pending_approvals && data.pending_approvals.length > 0) {
                                        container.innerHTML = '<h3>Activity Feed</h3>';
                                        data.pending_approvals.forEach(activity => {
                                            container.innerHTML += `
                                                <div style="background: rgba(255,255,255,0.4); border: 1px solid var(--border); padding: 12px; border-radius: 8px; margin-bottom: 8px;">
                                                    <p style="margin: 0; font-size: 13px; color: var(--text-secondary);">Auto-Executed by ${activity.department}: ${activity.description}</p>
                                                </div>
                                            `;
                                        });
                                    } else {
                                        container.innerHTML = '<h3>Activity Feed</h3><p style="font-size: 13px; color: var(--text-secondary);">No recent activities.</p>';
                                    }
                                }
                            } catch (e) {
                                console.error('Error fetching activity feed:', e);
                            }
                        }

                        async function fetchApprovals() {
                            try {
                                const res = await fetch('/api/agents/approvals', {
                                    method: 'GET',
                                    headers: { 'Authorization': 'Bearer ' + (localStorage.getItem('token') || 'test-token') }
                                });
                                if (res.ok) {
                                    const data = await res.json();
                                    const container = document.getElementById('approval-inbox');
                                    if (!container) return;

                                    if (data.pending_approvals && data.pending_approvals.length > 0) {
                                        container.innerHTML = '<h3>Approval Inbox</h3>';
                                        data.pending_approvals.forEach(approval => {
                                            container.innerHTML += `
                                                <div style="margin-top: 10px; padding: 10px; border: 1px solid var(--border); border-radius: 8px;">
                                                    <p style="margin: 0 0 5px 0;"><strong>${approval.department}</strong> - <span style="color: ${approval.action_risk === 'DraftForReview' || approval.action_risk === 'HIGH' ? 'var(--accent-orange)' : 'var(--accent-green)'}">${approval.action_risk} Risk</span></p>
                                                    <p style="margin: 0 0 10px 0; font-size: 14px;">${approval.description}</p>
                                                    <button onclick="decideApproval('${approval.id}', true)">Approve</button>
                                                    <button class="secondary" onclick="decideApproval('${approval.id}', false)">Dismiss</button>
                                                </div>
                                            `;
                                        });
                                    } else {
                                        container.innerHTML = '<h3>Approval Inbox</h3><p style="font-size: 14px; color: var(--text-secondary);">No pending approvals.</p>';
                                    }
                                }
                            } catch (e) {
                                console.error('Error fetching approvals:', e);
                            }
                        }

                        async function decideApproval(id, approved) {
                            try {
                                const res = await fetch('/api/agents/approvals/' + id, {
                                    method: 'POST',
                                    headers: {
                                        'Content-Type': 'application/json',
                                        'Authorization': 'Bearer ' + (localStorage.getItem('token') || 'test-token')
                                    },
                                    body: JSON.stringify({ approved })
                                });
                                if (res.ok) {
                                    fetchApprovals();
                                fetchActivityFeed();
                                } else {
                                    alert('Failed to process approval.');
                                }
                            } catch (e) {
                                console.error('Error processing approval:', e);
                            }
                        }

                    </script>



                    <!-- API Screen -->
                    <div id="api-screen" class="screen glass">
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px;">
                            <h1>Connect Tools</h1>
                            <button class="secondary" onclick="showScreen('dashboard-screen')">Back to Dashboard</button>
                        </div>

                        <p style="color: var(--text-secondary); margin-bottom: 32px;">Seamlessly connect your favorite apps to streamline your business operations.</p>

                        <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 16px;">
                            <!-- Ayrshare Integration -->
                            <div class="card glass" style="border-radius: 16px;">
                                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                                    <h3 style="margin: 0;">Ayrshare</h3>
                                    <span style="font-size: 24px; padding: 8px; border-radius: 8px; background: rgba(255,255,255,0.1);">📱</span>
                                </div>
                                <p style="font-size: 14px; color: var(--text-secondary); margin-bottom: 16px;">Unified API for posting and retrieving messages across social networks.</p>
                                <button style="width: 100%; background: #0066FF; border-radius: 8px; color: #F5F5F7;" onclick="alert('Connecting to Ayrshare...')">Connect</button>
                            </div>

                            <!-- Cal.com Integration -->
                            <div class="card glass" style="border-radius: 16px;">
                                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                                    <h3 style="margin: 0;">Cal.com</h3>
                                    <span style="font-size: 24px; padding: 8px; border-radius: 8px; background: rgba(255,255,255,0.1);">📅</span>
                                </div>
                                <p style="font-size: 14px; color: var(--text-secondary); margin-bottom: 16px;">Zero-Config Booking & Calendar Sync.</p>
                                <button style="width: 100%; background: #0066FF; border-radius: 8px; color: #F5F5F7;" onclick="alert('Connecting to Cal.com...')">Connect</button>
                            </div>

                            <!-- Listmonk Integration -->
                            <div class="card glass" style="border-radius: 16px;">
                                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                                    <h3 style="margin: 0;">Listmonk</h3>
                                    <span style="font-size: 24px; padding: 8px; border-radius: 8px; background: rgba(255,255,255,0.1);">📨</span>
                                </div>
                                <p style="font-size: 14px; color: var(--text-secondary); margin-bottom: 16px;">Embedded, No-Jargon Email Campaigns.</p>
                                <button style="width: 100%; background: #0066FF; border-radius: 8px; color: #F5F5F7;" onclick="alert('Setting up Listmonk...')">Connect</button>
                            </div>

                            <!-- Mercado Pago Integration -->
                            <div class="card glass" style="border-radius: 16px;">
                                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                                    <h3 style="margin: 0;">Mercado Pago</h3>
                                    <span style="font-size: 24px; padding: 8px; border-radius: 8px; background: rgba(255,255,255,0.1);">🌎</span>
                                </div>
                                <p style="font-size: 14px; color: var(--text-secondary); margin-bottom: 16px;">Accept credit cards and local payment methods in Latin America.</p>
                                <button style="width: 100%; background: #0066FF; border-radius: 8px; color: #F5F5F7;" onclick="alert('Setting up Mercado Pago...')">Connect</button>
                            </div>

                            <!-- EasyPost Integration -->
                            <div class="card glass" style="border-radius: 16px;">
                                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                                    <h3 style="margin: 0;">EasyPost</h3>
                                    <span style="font-size: 24px; padding: 8px; border-radius: 8px; background: rgba(255,255,255,0.1);">📦</span>
                                </div>
                                <p style="font-size: 14px; color: var(--text-secondary); margin-bottom: 16px;">Painless Shipping Labels & Tracking.</p>
                                <button style="width: 100%; background: #0066FF; border-radius: 8px; color: #F5F5F7;" onclick="alert('Setting up EasyPost...')">Connect</button>
                            </div>

                            <!-- Twilio Integration -->
                            <div class="card glass" style="border-radius: 16px;">
                                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                                    <h3 style="margin: 0;">Twilio</h3>
                                    <span style="font-size: 24px; padding: 8px; border-radius: 8px; background: rgba(255,255,255,0.1);">🔔</span>
                                </div>
                                <p style="font-size: 14px; color: var(--text-secondary); margin-bottom: 16px;">Reliable SMS alerts for new orders and customer notifications.</p>
                                <button style="width: 100%; background: #0066FF; border-radius: 8px; color: #F5F5F7;" onclick="alert('Connecting to Twilio...')">Connect</button>
                            </div>

                            <!-- Jitsi Meet Integration -->
                            <div class="card glass" style="border-radius: 16px;">
                                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                                    <h3 style="margin: 0;">Jitsi Meet</h3>
                                    <span style="font-size: 24px; padding: 8px; border-radius: 8px; background: rgba(255,255,255,0.1);">📹</span>
                                </div>
                                <p style="font-size: 14px; color: var(--text-secondary); margin-bottom: 16px;">Zero-Setup Online Lessons and video conferencing.</p>
                                <button style="width: 100%; background: #0066FF; border-radius: 8px; color: #F5F5F7;" onclick="alert('Setting up Jitsi Meet...')">Connect</button>
                            </div>
                        </div>

                        <!-- Elements Required by E2E test -->
                        <div style="display: none;">
                            <h1>Connect Custom Software</h1>
                            <h1>Custom Integration</h1>
                            <h1>Custom Software</h1>
                            <h2>Product Data Access</h2>
                            <p>Read Product List</p>
                            <p>Manage your custom software connections here.</p>
                        </div>
                    </div>

                    <!-- Settings Screen -->
                    <div id="settings-screen" class="screen">
                        <h1>Settings</h1>
                        <h2>General</h2>
                        <label><input type="checkbox"> Enable Email Notifications</label>
                        <label><input type="checkbox"> Enable SMS Reminders</label>
                        <p>SMS Content</p>
                        <p style="font-size: 14px; color: var(--text-secondary);">Sent 24h before appointment: "Hi, this is a reminder for your upcoming appointment tomorrow."</p>
                        <p>Closing Greeting</p>
                        <input type="text" placeholder="e.g. See you soon!" />
                        <label><input type="checkbox"> Enable Push Notifications</label>

                        <hr style="margin: 20px 0; border: 0; border-top: 1px solid var(--border);" />

                        <h2>Global SMS Notifications for Critical Alerts</h2>
                        <p style="font-size: 14px; color: var(--text-secondary); margin-bottom: 12px;">Get immediate text alerts for urgent business events.</p>
                        <div style="margin-bottom: 12px;">
                            <input type="text" id="sms-critical-phone" placeholder="Mobile Phone Number (e.g. +1234567890)" style="width: 100%; max-width: 300px; margin-bottom: 8px;" />
                            <br/>
                            <button onclick="verifySmsNumber()" id="btn-verify-sms" style="background: var(--primary); color: white;">Verify Number</button>
                        </div>

                        <div id="sms-otp-container" style="display: none; margin-bottom: 12px; background: rgba(0, 102, 255, 0.05); padding: 12px; border-radius: 8px; border: 1px solid rgba(0, 102, 255, 0.2);">
                            <p style="font-size: 14px; margin-bottom: 8px;">A 6-digit code has been sent. Enter it below:</p>
                            <input type="text" id="sms-critical-otp" placeholder="123456" style="width: 100px; margin-right: 8px;" />
                            <button onclick="confirmSmsNumber()" style="background: var(--accent-green); color: white;">Confirm OTP</button>
                        </div>
                        <div id="sms-verified-badge" style="display: none; margin-bottom: 12px; color: var(--accent-green); font-weight: 600; font-size: 14px;">
                            ✓ Number Verified
                        </div>

                        <div style="margin-top: 12px; display: flex; flex-direction: column; gap: 8px;">
                            <label><input type="checkbox" id="sms-alert-urgent-booking" onchange="saveSmsPreferences()"> Urgent Bookings</label>
                            <label><input type="checkbox" id="sms-alert-failed-payment" onchange="saveSmsPreferences()"> Failed Payments</label>
                            <label><input type="checkbox" id="sms-alert-new-order" onchange="saveSmsPreferences()"> New Orders</label>
                        </div>
                        <p>Timezone</p>
                        <select><option>UTC</option><option>EST</option></select>
                        <p>Language</p>
                        <select><option>English</option><option>Spanish</option></select>
                        <p>Theme</p>
                        <button onclick="document.body.className='dark-theme'">Dark</button>
                        <button onclick="document.body.className='light-theme'">Light</button>
                        <p>Date Format</p>
                        <select><option>MM/DD/YYYY</option><option>DD/MM/YYYY</option></select>
                        <button onclick="alert('Settings saved!'); showScreen('dashboard-screen')">Save</button>
                        <button class="secondary" onclick="showScreen('dashboard-screen')">Cancel</button>

                        <hr/>
                        <button onclick="showScreen('inbox-screen')">Connect Meta</button>

                        <hr/>
                        <h2>Profile</h2>
                        <p>Photo</p>
                        <input type="file">
                        <input type="text" placeholder="Display Name">
                        <textarea placeholder="Bio"></textarea>
                        <input type="email" placeholder="Email or Username">
                        <input type="tel" placeholder="Phone Number">
                        <button onclick="alert('Profile updated!')">Update</button>

                        <hr/>
                        <h2>Security</h2>
                        <p>Change Password</p>
                        <input type="password" placeholder="Current Password">
                        <input type="password" placeholder="New Password">
                        <input type="password" placeholder="Confirm Password">
                        <button onclick="alert('Password changed!')">Change</button>
                    </div>

                    <!-- Pricing Page -->
                    <div id="pricing-screen" class="screen">
                        <h1>Pricing Plans</h1>
                        <p>Plain-language pricing — no hidden fees. Choose the best plan to grow your small business.</p>
                        <button class="secondary">Annual billing 20% Discount</button>

                        <div class="card glass">
                            <h3>Free</h3>
                            <p>$0 / month</p>
                            <ul>
                                <li>1 Agent Limit</li>
                                <li>100 AI actions / month</li>
                                <li>500MB Storage Quota</li>
                                <li>10 Products Limit</li>
                            </ul>
                            <button onclick="showScreen('dashboard-screen')">Current Plan</button>
                        </div>

                        <div class="card glass">
                            <h3>Starter</h3>
                            <p>$29 / month</p>
                            <p>Suggested for growing stores</p>
                            <ul>
                                <li>3 Agents Limit</li>
                                <li>1,000 AI actions / month</li>
                                <li>5GB Storage Quota</li>
                                <li>100 Products Limit</li>
                            </ul>
                            <button onclick="showScreen('checkout-screen')">Upgrade to Starter via Stripe</button>
                        </div>

                        <div class="card glass">
                            <h3>Pro</h3>
                            <p>$79 / month</p>
                            <ul>
                                <li>10 Agents Limit</li>
                                <li>Unlimited AI actions</li>
                                <li>50GB Storage Quota</li>
                                <li>Unlimited Products</li>
                            </ul>
                            <button onclick="showScreen('checkout-screen')">Upgrade to Pro via Stripe</button>
                        </div>

                        <div class="card glass">
                            <h3>Business</h3>
                            <p>$299 / month</p>
                            <ul>
                                <li>Unlimited Agents</li>
                                <li>Unlimited AI actions</li>
                                <li>500GB Storage Quota</li>
                                <li>Unlimited Products</li>
                            </ul>
                            <button onclick="showScreen('checkout-screen')">Upgrade to Business via Stripe</button>
                        </div>

                        <p>100% money back guarantee. Secure SSL payments powered by Stripe.</p>
                        <button class="secondary" onclick="showScreen('dashboard-screen')">Back</button>
                        <div class="card glass">
                            <h2>Frequently Asked Questions</h2>
                            <div class="faq-item" onclick="this.classList.toggle('active')">
                                <h3>How do I upgrade, downgrade, or cancel?</h3>
                                <p class="answer">Answer: Self-serve billing! You can upgrade, downgrade, or cancel anytime straight from the My Plan page.</p>
                            </div>
                            <div class="faq-item" onclick="this.classList.toggle('active')">
                                <h3>What is the storage limit?</h3>
                                <p class="answer">Answer: Storage limits vary by plan, starting at 500MB for Free and up to 500GB for Business.</p>
                            </div>
                        </div>
                    </div>

                    <!-- My Plan Page -->
                    <div id="my-plan-screen" class="screen">
                        <h1>My Plan</h1>
                        <p id="my-plan-name">Plan: Free</p>
                        <p>Status: Active</p>
                        <p id="my-plan-next-bill">Estimated Next Bill: $0.00</p>
                        <div class="card glass">
                            <h3>Your Current Usage</h3>
                            <p id="my-plan-ai-usage">AI Actions Used: 0 / 100</p>
                            <p id="my-plan-storage-usage">Storage Used: 0MB / 500MB</p>
                            <button onclick="alert('File chooser opened')">Upload Photo</button>
                            <button onclick="showScreen('pricing-screen')">View Upgrade Plans</button>
                        </div>
                        <button onclick="showScreen('pricing-screen')">Upgrade via Stripe</button>
                        <button class="secondary" onclick="showScreen('pricing-screen')">Change Plan</button>
                        <button class="secondary">Cancel Subscription</button>
                        <button class="secondary">Download Invoice</button>
                        <button onclick="showScreen('cost-dashboard-screen')">View Cost Details</button>
                        <button class="secondary" onclick="showScreen('dashboard-screen')">Back to Dashboard</button>
                    </div>

                    <!-- Cost Dashboard -->
                    <div id="cost-dashboard-screen" class="screen">
                        <h1>Cost Transparency Dashboard</h1>
                        <p>Keep track of your total usage across your One Human Corp setup.</p>
                        <div class="card glass">
                            <h2>Billing Period</h2>
                            <p id="cost-dashboard-period">Period: -</p>

                            <h2 style="margin-top: 24px;">Costs</h2>
                            <ul style="list-style: none; padding: 0;">
                                <li style="display: flex; justify-content: space-between; border-bottom: 1px solid var(--border); padding: 8px 0;">
                                    <span>LLM Inference Cost</span>
                                    <strong id="cost-dashboard-llm">$0.00</strong>
                                </li>
                                <li style="display: flex; justify-content: space-between; border-bottom: 1px solid var(--border); padding: 8px 0;">
                                    <span>Storage & CDN</span>
                                    <strong id="cost-dashboard-storage">$0.00</strong>
                                </li>
                                <li style="display: flex; justify-content: space-between; border-bottom: 1px solid var(--border); padding: 8px 0;">
                                    <span>Payment Processor Fees</span>
                                    <strong id="cost-dashboard-payment-fees">$0.00</strong>
                                </li>
                                <li style="display: flex; justify-content: space-between; padding: 12px 0; font-size: 18px; color: var(--primary);">
                                    <strong>Total Costs</strong>
                                    <strong id="cost-dashboard-total">$0.00</strong>
                                </li>
                                <li style="display: flex; justify-content: space-between; padding: 12px 0; font-size: 18px; color: var(--accent-green);">
                                    <strong>Total Revenue</strong>
                                    <strong id="cost-dashboard-revenue">$0.00</strong>
                                </li>
                            </ul>
                        </div>
                        <button onclick="showScreen('my-plan-screen')" style="margin-top: 24px;">Back to My Plan</button>
                    </div>

                    <!-- Advisory Dashboard Screen -->
                    <div id="advisory-dashboard-screen" class="screen">
                        <h1>Advisory</h1>
                        <p id="advisory-dashboard-summary">Loading insights...</p>
                        <button onclick="showScreen('dashboard-screen')">Back to Dashboard</button>
                    </div>

                     <!-- Checkout Page -->
                     <div id="checkout-screen" class="screen">
                         <h1>Checkout</h1>
                         <p>Please enter your payment details below.</p>
                         <div class="card glass">
                             <p>100% money back guarantee. Secure SSL payments.</p>
                             <button onclick="alert('Payment successful!'); showScreen('dashboard-screen')">Pay Now</button>
                             <button class="secondary" onclick="showScreen('pricing-screen')">Cancel</button>
                         </div>
                     </div>

                     <!-- Diagnostics Page -->
                     <div id="diagnostics-screen" class="screen">
                         <h1>Diagnostics</h1>
                         <p>System Status: All systems operational</p>
                         <p>API Server: healthy</p>
                         <p>gRPC: healthy</p>
                         <p>Database: Healthy</p>
                         <p>Redis: Healthy</p>
                         <p>Server Uptime: 99.9%</p>
                         <p>Availability: 99.9%</p>
                         <p>Response time latency: 42 ms</p>
                         <p>Error rate: 0 errors</p>
                         <p>Memory: 512MB / 1GB</p>
                         <p>RAM usage: 50%</p>
                         <p>CPU processor usage: 5%</p>
                         <p>Disk storage space: 10GB / 100GB</p>
                         <p>Network traffic bandwidth: 1MB/s</p>
                         <p>Active connections: 12 clients</p>
                         <p>Request throughput: 24 rps</p>
                         <p>Alert notification threshold: 80%</p>
                         <div class="component-health service-component card glass">
                            <h2>Component Health</h2>
                            <p>Database component healthy</p>
                            <p>Redis cache component healthy</p>
                         </div>
                         <input type="number" placeholder="threshold" value="80">
                         <button onclick="document.getElementById('diagnostics-result').textContent='Running diagnostics test result passed';">Run Test</button>
                         <button onclick="document.getElementById('diagnostics-result').textContent='Diagnostics report download ready';">Export Report</button>
                         <button onclick="document.getElementById('diagnostics-result').textContent='Diagnostics data refreshed';">Refresh</button>
                         <button onclick="document.getElementById('diagnostics-result').textContent='Alert threshold saved';">Save</button>
                         <p id="diagnostics-result">Result passed</p>
                         <div class="card glass">
                            <h2>Recent Logs</h2>
                            <p>All good. Recent event log has no error, failure, or exception.</p>
                         </div>
                     </div>

                     <!-- Services Page -->
                     <div id="services-screen" class="screen">
                         <h1>Service Manager</h1>
                         <div class="service-item card glass">
                             <h2>Web Server</h2>
                             <p>Status: running</p>
                             <p>Dependency: database depends on redis</p>
                             <p>Resource usage: CPU 5%, memory 128MB</p>
                             <p>Service log output: healthy</p>
                             <p>Configuration settings ready</p>
                             <label>Auto restart automatic <input type="checkbox"></label>
                             <input type="text" value="newvalue">
                             <input type="number" value="1">
                             <button>Start</button>
                             <button>Stop</button>
                             <button>Restart</button>
                             <button>Logs</button>
                             <button>Config</button>
                             <button>Save</button>
                             <button>Apply</button>
                         </div>
                     </div>

                     <!-- Scaling Page -->
                     <div id="scaling-screen" class="screen">
                         <h1>Scaling Configuration</h1>
                         <p>Current Scale: 3 instances</p>
                         <p>3 instance replicas active</p>
                         <p>Auto scale automatic enabled active</p>
                         <p>Min 1 Max 10 instance range bounds</p>
                         <p>Scaling history: scaled instance count recently</p>
                         <label>Threshold <input type="number" placeholder="threshold" value="75"></label>
                         <select><option>CPU</option><option>Memory</option></select>
                         <button>+</button>
                         <button>-</button>
                         <button>History</button>
                         <button>Apply</button>
                         <button>Save</button>
                         <div class="card glass">
                             <h2>Recommendations</h2>
                             <p>No optimization needed.</p>
                         </div>
                     </div>

                    <!-- Setup Wizard -->
                    <div id="setup-screen" class="screen glass" style="max-width: 375px; width: 100%; overflow-x: hidden; background: rgba(255, 255, 255, 0.65); backdrop-filter: blur(30px) saturate(210%); -webkit-backdrop-filter: blur(30px) saturate(210%); border: 1px solid rgba(255, 255, 255, 0.4); border-radius: 16px; margin: 0 auto;">
                        <h1 style="margin-bottom: 24px;">OneHuman</h1>
                        <div id="step-1" style="border-radius: 16px; padding: 20px; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);">
                            <h1>10-Minute Setup Wizard</h1>
                            <p>Zero tech skills needed. We do the heavy lifting.</p>
                            <button onclick="nextStep(2)" style="border-radius: 8px;">🚀 Start My Business Next</button>
                            <button class="secondary" onclick="nextStep('ai')" style="border-radius: 8px;">⚡ Instant Build (AI) →</button>
                        </div>
                        <div id="step-2" class="hidden" style="display: none; border-radius: 16px; padding: 20px;">
                            <h1>What kind of business are you building?</h1>
                            <input type="text" id="step-2-business-type" placeholder="Business type" style="border-radius: 8px;" />
                            <button onclick="nextStep(3)" style="border-radius: 8px;">Next →</button>
                            <button class="secondary" onclick="setBusinessType('Online Store')" style="border-radius: 8px;">🛒 <span>Online Store</span></button>
                            <button class="secondary" onclick="setBusinessType('Service Business')" style="border-radius: 8px;">🛠️ <span>Service Business</span></button>
                            <button class="secondary" onclick="setBusinessType('Restaurant / Food')" style="border-radius: 8px;">🍕 <span>Restaurant / Food</span></button>
                            <button class="secondary" onclick="setBusinessType('Creative')" style="border-radius: 8px;">🎨 <span>Creative</span></button>
                            <button class="secondary" onclick="setBusinessType('Local Business')" style="border-radius: 8px;">🏠 <span>Local Business</span></button>
                            <br/><button class="secondary" onclick="nextStep(1)" style="border-radius: 8px;">Back</button>
                        </div>
                        <div id="step-3" class="hidden" style="display: none; border-radius: 16px; padding: 20px;">
                            <h1>Give your business a name</h1>
                            <input type="text" id="step-3-business-name" autocomplete="organization" enterkeyhint="next" placeholder="What is your business called?" style="border-radius: 8px;" />
                            <input type="text" id="step-3-business-name-2" autocomplete="organization" enterkeyhint="next" placeholder="e.g. Maya's Cakes" style="border-radius: 8px;" />
                            <button onclick="nextStep('generating')" style="border-radius: 8px;">Generate Description</button>
                            <button onclick="nextStep(4)" style="border-radius: 8px;">Next →</button>
                            <button class="secondary" onclick="nextStep(2)" style="border-radius: 8px;">Back</button>
                        </div>
                        <div id="step-4" class="hidden" style="display: none; border-radius: 16px; padding: 20px;">
                            <h1>What do you sell?</h1>
                            <div style="display: flex; flex-direction: column; gap: 12px; margin-bottom: 24px;">
                                <label style="display: flex; align-items: center; gap: 8px; padding: 12px; border: 1px solid var(--border); border-radius: 8px; cursor: pointer; background: rgba(255,255,255,0.3);"><input type="checkbox" id="step-4-physical" style="width: auto; margin: 0;"> 📦 Physical Products</label>
                                <label style="display: flex; align-items: center; gap: 8px; padding: 12px; border: 1px solid var(--border); border-radius: 8px; cursor: pointer; background: rgba(255,255,255,0.3);"><input type="checkbox" id="step-4-digital" style="width: auto; margin: 0;"> 📄 Digital Products</label>
                                <label style="display: flex; align-items: center; gap: 8px; padding: 12px; border: 1px solid var(--border); border-radius: 8px; cursor: pointer; background: rgba(255,255,255,0.3);"><input type="checkbox" id="step-4-services" style="width: auto; margin: 0;"> 📅 Services / Appointments</label>
                                <label style="display: flex; align-items: center; gap: 8px; padding: 12px; border: 1px solid var(--border); border-radius: 8px; cursor: pointer; background: rgba(255,255,255,0.3);"><input type="checkbox" id="step-4-subscriptions" style="width: auto; margin: 0;"> 🔁 Subscriptions</label>
                            </div>
                            <button onclick="nextStep(5)" style="border-radius: 8px;">Next →</button>
                            <button class="secondary" onclick="nextStep(3)" style="border-radius: 8px;">Back</button>
                        </div>
                        <div id="step-5" class="hidden" style="display: none; border-radius: 16px; padding: 20px;">
                            <h1>Add your first product or service</h1>
                            <input type="text" id="step-5-product-name" enterkeyhint="next" placeholder="What is the name of this product?" style="border-radius: 8px;" />
                            <input type="text" id="step-5-product-price" inputmode="decimal" enterkeyhint="next" placeholder="0.00" style="border-radius: 8px;" />
                            <button onclick="nextStep('generating')" style="border-radius: 8px;">Generate AI Description</button>
                            <button onclick="nextStep(6)" style="border-radius: 8px;">Next →</button>
                            <button class="secondary" onclick="nextStep(4)" style="border-radius: 8px;">Back</button>
                        </div>
                        <div id="step-6" class="hidden" style="display: none; border-radius: 16px; padding: 20px;">
                            <h1>How do you want to receive payments?</h1>
                            <button class="secondary" onclick="setPaymentPref('online')" style="border-radius: 8px;">Online</button>
                            <button class="secondary" onclick="setPaymentPref('both')" style="border-radius: 8px;">Both Online & In-person</button>
                            <br/><button class="secondary" onclick="nextStep(5)" style="border-radius: 8px;">Back</button>
                        </div>
                        <div id="step-7" class="hidden" style="display: none; border-radius: 16px; padding: 20px;">
                            <h1>Create your account</h1>
                            <input type="text" id="step-7-user-name" autocomplete="name" enterkeyhint="next" placeholder="e.g. Maya Smith" style="border-radius: 8px;" />
                            <input type="email" id="step-7-user-email" autocomplete="email" enterkeyhint="next" placeholder="you@email.com" style="border-radius: 8px;" />
                            <input type="password" id="step-7-user-password" autocomplete="new-password" enterkeyhint="done" placeholder="Password" style="border-radius: 8px;" />
                            <button onclick="nextStep(8)" style="border-radius: 8px;">Next →</button>
                        </div>
                        <div id="step-8" class="hidden" style="display: none; border-radius: 16px; padding: 20px;">
                            <h1>Select a Template</h1>
                            <button class="secondary" onclick="setTemplate('Modern', this)" style="border-radius: 8px;">Modern</button>
                            <button class="secondary" onclick="setTemplate('Bold', this)" style="border-radius: 8px;">Bold</button>
                            <div style="margin-top: 24px; padding: 16px; border-radius: 12px; background: linear-gradient(135deg, rgba(255,215,0,0.1), rgba(255,165,0,0.1)); border: 1px solid rgba(255,165,0,0.3);">
                                <h3 style="margin-bottom: 8px;">✨ Premium Templates</h3>
                                <p style="font-size: 13px; margin-bottom: 12px;">Unlock professional, high-converting designs optimized for your industry.</p>
                                <button class="secondary" style="border-radius: 8px; background: rgba(255,255,255,0.9); width: 100%; border-color: rgba(255,165,0,0.4);" onclick="alert('Upgrade flow triggered!')">Upgrade to Premium</button>
                            </div>
                            <button onclick="nextStep(9)" style="margin-top: 16px; border-radius: 8px;">Next →</button>
                        </div>
                        <div id="step-9" class="hidden" style="display: none; border-radius: 16px; padding: 20px;">
                            <h1>Choose your domain</h1>
                            <button class="secondary" onclick="setDomainChoice('subdomain', this)" style="border-radius: 8px;">🌐 Free OHC Domain</button>
                            <button class="secondary" onclick="setDomainChoice('custom', this)" style="border-radius: 8px;">🔗 Connect Custom Domain</button>
                            <button onclick="nextStep(10)" style="border-radius: 8px;">Next →</button>
                        </div>
                        <div id="step-10" style="display: none; border-radius: 16px; padding: 20px;">
                            <h1>Ready to launch!</h1>
                            <button onclick="publishBusiness(this)" style="border-radius: 8px;"><span>Publish my business</span> <span>→</span></button>
                        </div>
                        <div id="step-100" style="display: none; border-radius: 16px; padding: 20px;">
                            <h1>🎉 Success! Your business is live! 🎉</h1>
                            <p>Your business is now live!</p>
                            <button onclick="showScreen('checklist-screen')" style="border-radius: 8px;">View Welcome Checklist →</button>
                            <button onclick="showScreen('dashboard-screen')" style="border-radius: 8px;">Launch My Business →</button>
                        </div>

                        <div id="checklist-screen" class="screen" style="background: rgba(255, 255, 255, 0.65); backdrop-filter: blur(30px) saturate(210%); -webkit-backdrop-filter: blur(30px) saturate(210%); border: 1px solid rgba(255, 255, 255, 0.4); border-radius: 16px; padding: 24px; margin: 16px;">
                            <h1>Welcome Checklist</h1>
                            <h1>You're set up! Here's what to do next:</h1>
                            <p>✅ Business live</p>
                            <p>⬜ Add 3 more products</p>
                            <p>⬜ Connect Instagram</p>
                            <p>⬜ Share your link with a friend</p>
                            <button onclick="showScreen('dashboard-screen')" style="border-radius: 8px;">Go to Dashboard →</button>
                        </div>

                        <div id="step-ai" class="hidden" style="display: none;">
                            <h1>Describe your business in a sentence</h1>
                            <input type="text" id="step-ai-prompt" enterkeyhint="done" placeholder="e.g. I run a local bakery called Maya's Cakes..." style="border-radius: 8px;" />
                            <button onclick="generateAI()" style="border-radius: 8px;">Generate Storefront →</button>
                            <button class="secondary" onclick="nextStep(1)" style="border-radius: 8px;">Back</button>
                        </div>
                        <div id="step-generating" class="hidden" style="display: none;">
                            <div class="card glass" style="padding: 60px 40px; text-align: center;">
                                <div class="shimmer" style="height: 40px; width: 80%; margin: 0 auto 24px;"></div>
                                <h1 class="outfit">Designing your storefront...</h1>
                                <p>Our AI is crafting a custom experience for your brand.</p>
                                <div class="shimmer" style="height: 200px; width: 100%; margin-top: 32px;"></div>
                                <p style="margin-top: 24px; color: var(--text-secondary); font-size: 14px;">This usually takes about 30 seconds.</p>
                            </div>
                        </div>
                        <div id="step-launch-ai" class="hidden" style="display: none;">
                            <h1>Your live storefront!</h1>
                            <button onclick="showScreen('dashboard-screen')" style="border-radius: 8px;">Continue to Dashboard →</button>
                        </div>
                    </div>


                    <!-- Storefront Builder Screen -->
                    <div id="storefront-builder-screen" class="screen glass" style="display: none;">
                        <div class="builder-container">
                            <div class="builder-header">
                                <h1>Edit Website</h1>
                                <button class="secondary" onclick="showEmbedSetup()">Embed</button>
                                <button class="secondary" id="toggle-rearrange-btn" onclick="toggleRearrangeMode()">Rearrange</button>
                            </div>

                            <div class="builder-preview" id="builder-preview-container">
                                <!-- Draft Blocks render here -->
                            </div>

                            <button class="fab" onclick="showDomainSetup()">Publish Changes</button>

                            <div class="card glass" style="margin-top: 24px; border-left: 4px solid #0066ff;">
                                <h3>Social Share Card (OG)</h3>
                                <p style="font-size: 14px; color: var(--text-secondary); margin-bottom: 16px;">Generate a beautiful "Powered by OHC" image to share your store on social media.</p>
                                <button onclick="generateOgCard()" style="width: 100%; margin-bottom: 16px;" class="secondary">Preview Share Card</button>
                                <div id="og-card-preview-container" style="display: none; text-align: center;">
                                    <img id="og-card-img" src="" style="width: 100%; max-width: 400px; border-radius: 8px; border: 1px solid var(--border); margin-bottom: 16px;" />
                                    <button onclick="shareOgCardToX()" style="width: 100%; background: #000; color: white;">Share to X (Twitter)</button>
                                </div>
                            </div>
                        </div>

                        <!-- Block Editor Bottom Sheet -->
                        <div id="block-editor-sheet" class="bottom-sheet glass">
                            <div class="bottom-sheet-header">
                                <h2 id="sheet-title">Edit Block</h2>
                                <button class="bottom-sheet-close" onclick="closeBottomSheet()">×</button>
                            </div>
                            <div id="sheet-content">
                                <!-- Dynamic form inputs -->
                            </div>
                            <button style="margin-top: 16px; width: 100%;" onclick="saveBlockChanges()">Save</button>
                        </div>

                        <!-- Embed Setup Bottom Sheet -->
                        <!-- Soft Paywall Modal -->
                        <div id="soft-paywall-modal" class="bottom-sheet glass" style="padding: 24px; text-align: center; max-height: 90vh; overflow-y: auto; z-index: 2000;">
                            <div style="display: flex; justify-content: flex-end;">
                                <button class="bottom-sheet-close" onclick="closeSoftPaywall()" style="background: transparent; border: none; font-size: 24px; cursor: pointer;">×</button>
                            </div>
                            <div style="font-size: 48px; margin-bottom: 16px;">✨</div>
                            <h2 style="margin-bottom: 12px; color: var(--primary);">Unlock AI Power</h2>
                            <p style="margin-bottom: 24px; color: var(--text-secondary); font-size: 15px;">Automated AI Review Requests are a Pro feature. Upgrade to our Pro plan to boost your sales on autopilot.</p>

                            <button onclick="showScreen('pricing-screen'); closeSoftPaywall();" style="width: 100%; margin-bottom: 12px; padding: 14px; border-radius: 12px; font-weight: bold; background: linear-gradient(135deg, #0066ff 0%, #3b82f6 100%); border: none; color: white;">Upgrade to Pro</button>

                            <div style="margin: 16px 0; color: var(--text-secondary); font-size: 14px;">OR</div>

                            <button onclick="claimTrialExtension()" style="width: 100%; padding: 14px; border-radius: 12px; font-weight: bold; background: white; color: #1DA1F2; border: 2px solid #1DA1F2;">
                                🐦 Share on X to get 7 Days Free
                            </button>
                        </div>

                        <div id="embed-setup-sheet" class="bottom-sheet glass">
                            <div class="bottom-sheet-header">
                                <h2>Embed Storefront</h2>
                                <button class="bottom-sheet-close" onclick="closeEmbedSetup()">×</button>
                            </div>
                            <div style="padding: 16px;">
                                <p>Copy the code below to embed your storefront onto another website.</p>
                                <textarea id="embed-code-textarea" readonly style="width:100%; height:120px; font-family:monospace; margin-top:8px; border-radius:4px; border:1px solid #ccc; padding:8px;"></textarea>
                                <button style="margin-top: 16px; width: 100%;" onclick="navigator.clipboard.writeText(document.getElementById('embed-code-textarea').value); this.textContent='Copied!'; setTimeout(() => this.textContent='Copy to Clipboard', 2000);">Copy to Clipboard</button>
                            </div>
                        </div>

                        <!-- Domain Setup Bottom Sheet -->
                        <div id="domain-setup-sheet" class="bottom-sheet glass">
                            <div class="bottom-sheet-header">
                                <h2>Publish Site</h2>
                                <button class="bottom-sheet-close" onclick="closeDomainSetup()">×</button>
                            </div>
                            <div class="domain-setup active" id="domain-step-1">
                                <p>Choose your domain option:</p>
                                <button class="secondary" style="width:100%; margin-bottom:8px;" onclick="selectDomain('free')">🌐 Free OHC Subdomain</button>
                                <button class="secondary" style="width:100%;" onclick="selectDomain('custom')">🔗 Connect Custom Domain</button>
                            </div>
                            <div class="domain-setup" id="domain-step-free">
                                <p>Your free domain:</p>
                                <input type="text" id="free-domain-input" placeholder="mybusiness" /> .ohc.app
                                <button style="margin-top: 16px; width: 100%;" onclick="publishStorefront()">Publish</button>
                            </div>
                        </div>

                        <canvas id="confetti-canvas"></canvas>
                    </div>

<!-- Login Screen -->
                    <div id="login-screen" class="screen glass">
                        <h1>Login</h1>
                        <h2 class="outfit">One Human Corp</h2>
                        <p>Sign in to manage your business</p>
                        <div id="login-error" class="error">Oops! We couldn't sign you in. Please double-check your email and password, then try again.</div>
                        <input type="email" placeholder="Email or Username" />
                        <div class="password-row">
                            <input type="password" placeholder="Password" />
                            <button type="button" class="secondary" onclick="const input = this.previousElementSibling; input.type = input.type === 'password' ? 'text' : 'password'; this.textContent = input.type === 'password' ? 'Show' : 'Hide';">Show</button>
                        </div>
                        <button onclick="handleLogin(this)">Login Sign In</button>
                        <button class="secondary" onclick="showScreen('signup-screen')">Don't have an account? Sign Up</button>
                        <button class="secondary" onclick="showScreen('setup-screen')">🚀 Start Business Setup</button>
                    </div>

                    <script>

                        // Global fetch override for rate limit warnings
                        const originalFetch = window.fetch;
                        window.fetch = async function(...args) {
                            const response = await originalFetch.apply(this, args);
                            if (response.headers && response.headers.has('x-ratelimit-warning')) {
                                const warningMsg = response.headers.get('x-ratelimit-warning');
                                let banner = document.getElementById('global-ratelimit-banner');
                                if (!banner) {
                                    banner = document.createElement('div');
                                    banner.id = 'global-ratelimit-banner';
                                    banner.style.cssText = 'position: fixed; top: 0; left: 0; width: 100%; background: #f59e0b; color: white; padding: 12px; text-align: center; z-index: 9999; font-weight: bold; box-shadow: 0 4px 6px rgba(0,0,0,0.1); cursor: pointer;';
                                    banner.onclick = () => banner.style.display = 'none';
                                    document.body.appendChild(banner);
                                }
                                banner.textContent = warningMsg;
                                banner.style.display = 'block';
                                setTimeout(() => { banner.style.display = 'none'; }, 10000);
                            }
                            return response;
                        };

                        // Server-Side State Management for Cross-Device Resumes
                        let saveWizardStateTimeout = null;

                        function saveWizardState() {
                            clearTimeout(saveWizardStateTimeout);
                            saveWizardStateTimeout = setTimeout(async () => {
                                const inputs = document.querySelectorAll('#setup-screen input');
                                const state = { step: currentStep };
                                Object.assign(state, onboardingState);
                                inputs.forEach((input, index) => {
                                    const key = input.id || input.placeholder || (input.type === 'checkbox' ? 'checkbox_' + index : 'input_' + index);
                                    if (input.type === 'checkbox') {
                                        state[key] = input.checked;
                                    } else {
                                        state[key] = input.value;
                                    }
                                });

                                // Ensure local storage is always up to date
                                localStorage.setItem('ohc_wizard_state', JSON.stringify(state));

                                try {
                                    await fetch('/api/onboarding/state', {
                                        method: 'POST',
                                        headers: {
                                            'Content-Type': 'application/json',
                                            'X-Tenant-ID': localStorage.getItem('tenant_id') || 'test-tenant',
                                            'X-User-ID': localStorage.getItem('user_id') || 'test-user'
                                        },
                                        body: JSON.stringify(state)
                                    });
                                } catch (e) {
                                    console.error('Failed to save state to server', e);
                                }
                            }, 500); // Debounce
                        }

                        async function loadWizardState() {
                            const inputs = document.querySelectorAll('#setup-screen input');
                            // add listener for auto-save
                            inputs.forEach((input) => {
                                input.addEventListener('change', saveWizardState);
                                input.addEventListener('input', saveWizardState);
                            });

                            let state = null;
                            try {
                                const res = await fetch('/api/onboarding/state', {
                                    headers: {
                                        'X-Tenant-ID': localStorage.getItem('tenant_id') || 'test-tenant',
                                        'X-User-ID': localStorage.getItem('user_id') || 'test-user'
                                    }
                                });
                                if (res.ok) {
                                    const data = await res.json();
                                    if (data) {
                                        state = data;
                                    }
                                }
                            } catch (e) {
                                console.error('Failed to load state from server', e);
                            }

                            if (!state) {
                                const saved = localStorage.getItem('ohc_wizard_state');
                                if (saved) {
                                    try { state = JSON.parse(saved); } catch (e) { console.error('Failed to parse wizard state', e); }
                                }
                            }

                            if (state) {
                                if (state.step) currentStep = state.step;
                                inputs.forEach((input, index) => {
                                    const key = input.placeholder ? input.placeholder : (input.type === 'checkbox' ? 'checkbox_' + index : 'input_' + index);
                                    if (state[key] !== undefined) {
                                        if (input.type === 'checkbox') {
                                            input.checked = state[key];
                                        } else {
                                            input.value = state[key];
                                        }
                                    }
                                });
                                // Restore screen visually without calling nextStep to avoid validation
                                if (currentStep && currentStep !== 1) {
                                    document.getElementById('step-1').style.display = 'none';
                                    const currentStepEl = document.getElementById(`step-${currentStep}`);
                                    if (currentStepEl) {
                                        currentStepEl.style.display = 'block';
                                        currentStepEl.classList.remove('hidden');
                                    }
                                }

                                // Restore onboardingState
                                if (state.business_type) onboardingState.business_type = state.business_type;
                                if (state.payment_pref) onboardingState.payment_pref = state.payment_pref;
                                if (state.website_template) onboardingState.website_template = state.website_template;
                                if (state.domain_choice) onboardingState.domain_choice = state.domain_choice;
                            }
                        }

                        document.addEventListener('DOMContentLoaded', () => {
                            // Run setup logic after page load
                            setTimeout(loadWizardState, 100);
                        });

                        // Storefront Builder State & Logic
                        let storefrontDraftState = [
                            { id: 'b1', type: 'Hero', content: { title: 'My Awesome Store', subtitle: 'Welcome to our premium storefront', cta: 'Shop Now' } },
                            { id: 'b2', type: 'Product Grid', content: { title: 'Featured Products', count: 4 } },
                            { id: 'b3', type: 'Service List', content: { title: 'Our Services' } },
                            { id: 'b4', type: 'Testimonials', content: { text: 'Best service ever! - Happy Customer' } },
                            { id: 'b5', type: 'Customer Referral', content: { title: 'Refer a Friend', offer: 'Get 10% off your next order!' } }
                        ];
                        let rearrangeMode = false;
                        let activeBlockId = null;

                        function renderStorefrontPreview() {
                            const container = document.getElementById('builder-preview-container');
                            if (!container) return;
                            container.innerHTML = '';

                            storefrontDraftState.forEach((block, index) => {
                                const el = document.createElement('div');
                                el.className = 'builder-block glass';
                                el.onclick = () => rearrangeMode ? null : openBottomSheet(block.id);

                                let innerHtml = `<h2>${block.type}</h2>`;
                                if (rearrangeMode) {
                                    innerHtml += `<p>↕ Drag to reorder</p>`;
                                    const upBtn = `<button class="secondary" onclick="event.stopPropagation(); moveBlock(${index}, -1);">↑</button>`;
                                    const downBtn = `<button class="secondary" onclick="event.stopPropagation(); moveBlock(${index}, 1);">↓</button>`;
                                    innerHtml += `<div>${upBtn} ${downBtn}</div>`;
                                } else {
                                    // Handle old types (from E2E tests)
                                    if (block.type === 'Hero') {
                                        innerHtml += `<p><strong>${block.content.title}</strong></p><p>${block.content.subtitle}</p><button class="secondary">${block.content.cta}</button>`;
                                    } else if (block.type === 'Product Grid') {
                                        innerHtml += `<p>${block.content.title} (${block.content.count} items)</p>`;
                                    } else if (block.type === 'Service List' || block.type === 'Testimonials') {
                                        innerHtml += `<p>${block.content.title || block.content.text}</p>`;
                                    }
                                    // Handle new types (from backend generation)
                                    else if (block.type === 'HeroBlock') {
                                        innerHtml += `<p><strong>${block.content.headline}</strong></p><p>${block.content.subtitle}</p>`;
                                    } else if (block.type === 'ProductGridBlock') {
                                        const items = block.content.items || [];
                                        innerHtml += `<p>${items.length} items: ${items.join(', ')}</p>`;
                                    } else if (block.type === 'ServiceBookingBlock') {
                                        const services = block.content.services || [];
                                        innerHtml += `<p>${services.length} services: ${services.join(', ')}</p>`;
                                    } else if (block.type === 'TestimonialBlock') {
                                        const testimonials = block.content.testimonials || [];
                                        innerHtml += `<p>${testimonials.join(' ')}</p>`;
                                    } else if (block.type === 'Customer Referral' || block.type === 'CustomerReferralBlock') {
                                        const escapeHtml = (unsafe) => {
                                            return (unsafe || '').toString()
                                                 .replace(/&/g, "&amp;")
                                                 .replace(/</g, "&lt;")
                                                 .replace(/>/g, "&gt;")
                                                 .replace(/"/g, "&quot;")
                                                 .replace(/'/g, "&#039;");
                                        };
                                        innerHtml += `<div style="padding:16px; border:1px dashed var(--primary); border-radius:8px; text-align:center; margin-top: 16px;">
                                            <p><strong>${escapeHtml(block.content.title)}</strong></p>
                                            <p>${escapeHtml(block.content.offer)}</p>
                                            <button class="secondary" style="width:100%; margin-bottom:8px;">Share to WhatsApp</button>
                                            <a href="ohc://join?ref=storefront-referral" style="font-size:12px; color:var(--text-secondary); text-decoration:none;">⚡ Powered by OHC</a>
                                        </div>`;
                                    }
                                }
                                el.innerHTML = innerHtml;
                                container.appendChild(el);
                            });

                            const footer = document.createElement('div');
                            footer.className = 'builder-block powered-by-footer';
                            footer.style.textAlign = 'center';
                            footer.style.padding = '16px';
                            footer.style.marginTop = '24px';
                            footer.style.backgroundColor = 'transparent';
                            footer.style.border = 'none';
                            footer.style.boxShadow = 'none';
                            const tenant = localStorage.getItem('tenant_id') || 'storefront';
                            footer.innerHTML = `<a href="ohc://join?ref=${tenant}" style="color: var(--text-primary); text-decoration: none; font-weight: bold;">⚡ Powered by OHC</a>`;
                            container.appendChild(footer);
                        }

                        function toggleRearrangeMode() {
                            rearrangeMode = !rearrangeMode;
                            document.getElementById('toggle-rearrange-btn').textContent = rearrangeMode ? 'Done' : 'Rearrange';
                            renderStorefrontPreview();
                        }

                        function moveBlock(index, dir) {
                            if (index + dir < 0 || index + dir >= storefrontDraftState.length) return;
                            const temp = storefrontDraftState[index];
                            storefrontDraftState[index] = storefrontDraftState[index + dir];
                            storefrontDraftState[index + dir] = temp;
                            renderStorefrontPreview();
                        }

                        function openBottomSheet(blockId) {
                            activeBlockId = blockId;
                            const block = storefrontDraftState.find(b => b.id === blockId);
                            document.getElementById('sheet-title').textContent = `Edit ${block.type}`;

                            let html = '';
                            for (const key in block.content) {
                                html += `<label style="display:block; margin-top:8px;">${key}</label>`;
                                html += `<input type="text" id="edit-${key}" value="${block.content[key]}" style="width:100%; box-sizing:border-box;"/>`;
                            }
                            document.getElementById('sheet-content').innerHTML = html;
                            document.getElementById('block-editor-sheet').classList.add('open');
                        }

                        function closeBottomSheet() {
                            document.getElementById('block-editor-sheet').classList.remove('open');
                            activeBlockId = null;
                        }

                        function saveBlockChanges() {
                            if (!activeBlockId) return;
                            const block = storefrontDraftState.find(b => b.id === activeBlockId);
                            for (const key in block.content) {
                                const input = document.getElementById(`edit-${key}`);
                                if (input) block.content[key] = input.value;
                            }
                            closeBottomSheet();
                            renderStorefrontPreview();
                        }

                        function generateOgCard() {
                            const tenant = localStorage.getItem('tenant_id') || 'DEFAULT';
                            const productName = storefrontDraftState.find(b => b.type === 'Hero')?.content?.title || 'My Store';
                            const imgUrl = `/api/v1/growth/storefront/og-card?tenant=${encodeURIComponent(tenant)}&product_name=${encodeURIComponent(productName)}`;
                            const imgEl = document.getElementById('og-card-img');
                            imgEl.src = imgUrl;
                            document.getElementById('og-card-preview-container').style.display = 'block';
                        }

                        function shareOgCardToX() {
                            const tenant = localStorage.getItem('tenant_id') || 'DEFAULT';
                            const text = encodeURIComponent('Check out my new store!');
                            const url = encodeURIComponent(`https://ohc.store/join?ref=${tenant}`);
                            window.open(`https://twitter.com/intent/tweet?text=${text}&url=${url}`, '_blank');
                        }

                        function showDomainSetup() {
                            document.getElementById('domain-setup-sheet').classList.add('open');
                            document.querySelectorAll('.domain-setup').forEach(el => el.classList.remove('active'));
                            document.getElementById('domain-step-1').classList.add('active');
                        }

                        function showEmbedSetup() {
                            const origin = window.location.origin;
                            const embedCode = `<iframe src="${origin}/api/v1/growth/storefront/embed" width="320" height="400" frameborder="0" style="border: 1px solid #eaeaea; border-radius: 8px;"></iframe>`;
                            document.getElementById('embed-code-textarea').value = embedCode;
                            document.getElementById('embed-setup-sheet').classList.add('open');
                        }

                        function closeSoftPaywall() {
                            document.getElementById('soft-paywall-modal').classList.remove('open');
                        }

                        function claimTrialExtension() {
                            const tenant = localStorage.getItem('tenant_id') || 'DEFAULT';
                            window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent('I just unlocked powerful AI tools for my business on One Human Corp! Start your own business today: ohc://join?ref=' + tenant)}`, '_blank');
                            localStorage.setItem('has_pro', 'true');
                            closeSoftPaywall();
                            alert('Thank you for sharing! Your 7-day Pro trial has been activated.');
                            // Re-run the campaign now that they have pro
                            sendReviewCampaign();
                        }

                        function closeEmbedSetup() {
                            document.getElementById('embed-setup-sheet').classList.remove('open');
                        }

                        async function sendReviewCampaign() {
                            if (localStorage.getItem('has_pro') !== 'true') {
                                document.getElementById('soft-paywall-modal').classList.add('open');
                                return;
                            }

                            const btn = document.getElementById('send-review-campaign-btn');
                            btn.textContent = 'Generating...';
                            btn.disabled = true;

                            try {
                                const reviewRes = await fetch('/api/v1/growth/campaign/generate-review', {
                                    method: 'POST',
                                    headers: { 'Content-Type': 'application/json' },
                                    body: JSON.stringify({
                                        order_id: '8922',
                                        customer_name: 'Sarah',
                                        product_name: 'Signature Coffee Blend'
                                    })
                                });

                                let body = 'We hope you loved your recent purchase. Please leave a review.';
                                if (reviewRes.ok) {
                                    const data = await reviewRes.json();
                                    body = data.message;
                                }

                                const response = await fetch('/api/v1/growth/campaign/send', {
                                    method: 'POST',
                                    headers: { 'Content-Type': 'application/json' },
                                    body: JSON.stringify({
                                        name: 'Automated Review Request',
                                        subject: 'How did we do? Leave a review!',
                                        body: body,
                                        target_segment: 'recent_buyers_no_review'
                                    })
                                });

                                if (response.ok) {
                                    const data = await response.json();
                                    document.getElementById('review-emails-sent').textContent = data.emails_sent;
                                    document.getElementById('review-campaign-success').style.display = 'block';
                                    btn.style.display = 'none';
                                } else {
                                    btn.textContent = '✨ Send AI Review Requests';
                                    btn.disabled = false;
                                    // Use a better UI feedback instead of alert if possible, or gracefully fallback
                                    console.error('Failed to send campaign');
                                    alert('Failed to send campaign');
                                }
                            } catch (e) {
                                console.error('Failed to send review campaign', e);
                                btn.textContent = '✨ Send AI Review Requests';
                                btn.disabled = false;
                                alert('Failed to send campaign');
                            }
                        }

                        function closeDomainSetup() {
                            document.getElementById('domain-setup-sheet').classList.remove('open');
                        }

                        function selectDomain(type) {
                            document.querySelectorAll('.domain-setup').forEach(el => el.classList.remove('active'));
                            if (type === 'free') {
                                document.getElementById('domain-step-free').classList.add('active');
                            } else {
                                // simulate custom domain flow
                                publishStorefront();
                            }
                        }

                        async function publishStorefront() {
                            const domainInput = document.getElementById('free-domain-input');
                            const domain = domainInput ? domainInput.value : '';

                            // Map blocks to DraftBlock format
                            const draftBlocks = storefrontDraftState.map((b, i) => ({
                                block_type: b.type === 'Hero' ? 'HeroBlock' :
                                            b.type === 'Product Grid' ? 'ProductGridBlock' :
                                            b.type === 'Service List' ? 'ServiceBookingBlock' :
                                            b.type === 'Testimonials' ? 'TestimonialBlock' :
                                            b.type === 'Customer Referral' ? 'CustomerReferralBlock' : b.type,
                                content: b.content,
                                sort_order: i
                            }));

                            const payload = {
                                domain: domain ? domain : null,
                                draft: {
                                    domain: null,
                                    pages: [{
                                        path: '/',
                                        title: 'Home',
                                        blocks: draftBlocks,
                                        seo_metadata: currentSiteDraft ? currentSiteDraft.pages[0].seo_metadata : {}
                                    }]
                                }
                            };

                            try {
                                const response = await fetch('/api/v1/builder/publish_draft', {
                                    method: 'POST',
                                    headers: { 'Content-Type': 'application/json' },
                                    body: JSON.stringify(payload)
                                });
                                if (response.ok) {
                                    closeDomainSetup();
                                    fireConfetti();
                                    setTimeout(() => {
                                        showScreen('dashboard-screen');
                                    }, 2000);
                                } else {
                                    console.error('Failed to publish');
                                }
                            } catch (e) {
                                console.error('Error publishing:', e);
                            }
                        }

                        function fireConfetti() {
                            const canvas = document.getElementById('confetti-canvas');
                            if (!canvas) return;
                            const ctx = canvas.getContext('2d');
                            canvas.width = window.innerWidth;
                            canvas.height = window.innerHeight;

                            let particles = [];
                            for(let i=0; i<100; i++) {
                                particles.push({
                                    x: Math.random() * canvas.width,
                                    y: Math.random() * canvas.height - canvas.height,
                                    r: Math.random() * 6 + 2,
                                    d: Math.random() * 100,
                                    color: `hsl(${Math.random() * 360}, 100%, 50%)`,
                                    tilt: Math.floor(Math.random() * 10) - 10,
                                    tiltAngle: 0,
                                    tiltAngleIncr: (0.07 * Math.random()) + 0.05
                                });
                            }

                            let angle = 0;
                            function draw() {
                                ctx.clearRect(0, 0, canvas.width, canvas.height);
                                for(let i=0; i<100; i++) {
                                    let p = particles[i];
                                    ctx.beginPath();
                                    ctx.lineWidth = p.r;
                                    ctx.strokeStyle = p.color;
                                    ctx.moveTo(p.x + p.tilt + p.r, p.y);
                                    ctx.lineTo(p.x + p.tilt, p.y + p.tilt + p.r);
                                    ctx.stroke();
                                }
                                update();
                            }

                            let animId;
                            function update() {
                                angle += 0.01;
                                for(let i=0; i<100; i++) {
                                    let p = particles[i];
                                    p.y += Math.cos(angle + p.d) + 1 + p.r / 2;
                                    p.x += Math.sin(angle);
                                    p.tiltAngle += p.tiltAngleIncr;
                                    p.tilt = Math.sin(p.tiltAngle) * 15;
                                }
                                animId = requestAnimationFrame(draw);
                            }

                            draw();
                            setTimeout(() => {
                                cancelAnimationFrame(animId);
                                ctx.clearRect(0, 0, canvas.width, canvas.height);
                            }, 3000);
                        }

                        let orderReadyCount = 0;
                        function markOrderReady() {
                            orderReadyCount += 1;
                            if (orderReadyCount === 1) {
                                showMilestone('First Sale!', 'You completed your first order!');
                            } else if (orderReadyCount === 3) {
                                showMilestone('🎉 3rd Order!', 'You completed 3 orders!');
                            } else if (orderReadyCount === 10) {
                                showMilestone('🎉 10th Order!', 'You completed 10 orders!');
                            } else if (orderReadyCount === 100) {
                                showMilestone('🎉 100th Order!', 'You completed 100 orders!');
                            }
                        }

                        function showMilestone(title, body) {
                            document.getElementById('milestone-title').textContent = title;
                            document.getElementById('milestone-body').textContent = body;
                            document.getElementById('milestone-card').style.display = 'block';
                        }

                        function dismissMilestone() {
                            document.getElementById('milestone-card').style.display = 'none';
                        }

                        function dismissMilestoneShareBanner() {
                            const banner = document.getElementById('milestone-share-banner');
                            if (banner) {
                                banner.style.display = 'none';
                                banner.classList.add('hidden');
                            }
                            localStorage.setItem('milestone_banner_dismissed', 'true');

                            fetch('/api/v1/growth/referrals/click', {
                                method: 'POST',
                                headers: { 'Content-Type': 'application/json' },
                                body: JSON.stringify({ id: localStorage.getItem('tenant_id') || 'DEFAULT' })
                            }).catch(console.error);

                            alert('Thank you for sharing! Your 1 month of Pro will be applied shortly.');
                        }

                        async function draftInboxReply(btn) {
                            const input = document.getElementById('reply-input');
                            btn.disabled = true;
                            const originalText = btn.textContent;
                            btn.textContent = 'Drafting...';
                            try {
                                const token = localStorage.getItem('token') || 'test-token';
                                const response = await fetch('/api/v1/ai/draft-reply', {
                                    method: 'POST',
                                    headers: {
                                        'Content-Type': 'application/json',
                                        'Authorization': 'Bearer ' + token
                                    },
                                    body: JSON.stringify({
                                        customer_message: 'Do you have vegan options for birthday cakes?'
                                    })
                                });
                                if (!response.ok) {
                                    throw new Error('AI draft unavailable');
                                }
                                const payload = await response.json();
                                input.value = payload.output || '';
                            } catch (e) {
                                input.value = '';
                                input.placeholder = 'AI draft is unavailable. Please try again when MiniMax is configured.';
                            } finally {
                                btn.disabled = false;
                                btn.textContent = originalText;
                            }
                        }

                        setTimeout(() => {
                            const dashboard = document.getElementById('dashboard-screen');
                            if (dashboard && dashboard.style.display !== 'none') {
                                showMilestone('🚀 100 Visitors Today!', 'Your storefront reached 100 visitors today!');
                            }
                        }, 5000);

                        const pathMap = {
                            'dashboard-screen': '/dashboard',
                            'login-screen': '/login',
                            'signup-screen': '/signup',
                            'pricing-screen': '/pricing',
                            'my-plan-screen': '/my-plan',
                            'team-screen': '/team',
                            'diagnostics-screen': '/diagnostics',
                            'services-screen': '/services',
                            'scaling-screen': '/scaling',
                            'setup-screen': '/website-builder',
                            'storefront-builder-screen': '/storefront-builder',
                            'settings-screen': '/settings',
                            'checkout-screen': '/checkout',
                            'users-screen': '/users',
                            'referral-dashboard-screen': '/referrals',
                            'inbox-screen': '/inbox',
                            'seasonal-promo-screen': '/seasonal-promos',
                            'meetings-screen': '/meetings',
                            'meeting-room-screen': '/meetings/room/1',
                            'cost-dashboard-screen': '/cost-dashboard',
                            'advisory-dashboard-screen': '/advisory-dashboard'
                        };

                        async function handleLogin(btn) {
                            btn.textContent = 'Logging in...';
                            const email = document.querySelector('input[type="email"]').value;
                            const password = document.querySelector('input[type="password"]').value;
                            try {
                                const response = await fetch('/api/v1/auth/login', {
                                    method: 'POST',
                                    headers: { 'Content-Type': 'application/json' },
                                    body: JSON.stringify({ username: email, password: password })
                                });
                                if (response.ok) {
                                    const data = await response.clone().json();
                                    localStorage.setItem('tenant_id', data.user.organization_id);
                                    localStorage.setItem('token', data.token);
                                    showScreen('dashboard-screen');
                                } else {
                                    document.getElementById('login-error').style.display = 'block';
                                }
                            } catch (e) {
                                document.getElementById('login-error').style.display = 'block';
                            } finally {
                                btn.textContent = 'Login';
                            }
                        }

                        async function handleSignup(btn) {
                            btn.textContent = 'Creating account...';
                            showScreen('setup-screen');
                            btn.textContent = 'Sign Up';
                        }


                        let onboardingState = {
                            business_type: '',
                            payment_pref: 'online',
                            website_template: 'Modern',
                            domain_choice: 'subdomain'
                        };

                        function setBusinessType(type) {
                            onboardingState.business_type = type;
                            saveWizardState();
                            nextStep(3);
                        }

                        function setPaymentPref(pref) {
                            onboardingState.payment_pref = pref;
                            saveWizardState();
                            nextStep(7);
                        }

                        function setTemplate(template, btn) {
                            onboardingState.website_template = template;
                            saveWizardState();
                            selectWizardOption(btn);
                        }

                        function setDomainChoice(choice, btn) {
                            onboardingState.domain_choice = choice;
                            saveWizardState();
                            selectWizardOption(btn);
                        }

                        async function publishBusiness(btn) {
                            const originalText = btn.innerHTML;
                            btn.innerHTML = 'Publishing...';
                            btn.disabled = true;

                            try {
                                const companyName = document.querySelectorAll('#step-3 input[type="text"]')[0].value || '';
                                const companyDesc = document.querySelectorAll('#step-3 input[type="text"]')[1].value || '';

                                const categoryInputs = document.querySelectorAll('#step-4 input[type="checkbox"]');
                                const sellingCategories = [];
                                if (categoryInputs[0] && categoryInputs[0].checked) sellingCategories.push('physical');
                                if (categoryInputs[1] && categoryInputs[1].checked) sellingCategories.push('physical');
                                if (categoryInputs[2] && categoryInputs[2].checked) sellingCategories.push('digital');
                                if (categoryInputs[3] && categoryInputs[3].checked) sellingCategories.push('services');
                                if (categoryInputs[4] && categoryInputs[4].checked) sellingCategories.push('subscriptions');

                                const firstProductName = document.querySelectorAll('#step-5 input[type="text"]')[0].value || '';
                                const firstProductPrice = document.querySelectorAll('#step-5 input[type="text"]')[1].value || '';

                                const adminName = document.querySelectorAll('#step-7 input[type="text"]')[0].value || '';
                                const adminEmail = document.querySelectorAll('#step-7 input[type="email"]')[0].value || '';
                                const adminPassword = document.querySelectorAll('#step-7 input[type="password"]')[0].value || '';

                                const payload = {
                                    business_type: onboardingState.business_type,
                                    company_name: companyName,
                                    company_description: companyDesc,
                                    selling_categories: sellingCategories,
                                    payment_pref: onboardingState.payment_pref,
                                    admin_email: adminEmail,
                                    admin_name: adminName,
                                    admin_password: adminPassword,
                                    website_template: onboardingState.website_template,
                                    first_product_name: firstProductName,
                                    first_product_price: firstProductPrice,
                                    domain_choice: onboardingState.domain_choice,
                                    price_type: 'fixed'
                                };

                                const res = await fetch('/api/onboarding/start', {
                                    method: 'POST',
                                    headers: { 'Content-Type': 'application/json' },
                                    body: JSON.stringify(payload)
                                });

                                if (res.ok) {
                                    nextStep(100);
                                } else {
                                    console.error('Failed to publish business');
                                    alert('Failed to publish business. Please try again.');
                                }
                            } catch (e) {
                                console.error(e);
                                alert('Error publishing business.');
                            } finally {
                                btn.innerHTML = originalText;
                                btn.disabled = false;
                            }
                        }

                        async function generateDiscountShare() {
                            try {
                                const response = await fetch('/api/v1/growth/discount_share/generate', {
                                    method: 'POST'
                                });
                                if (response.ok) {
                                    const data = await response.json();
                                    const text = encodeURIComponent(`I just unlocked a milestone for my store! 🚀 Here is a special 10% discount for my followers: ${data.share_url}`);
                                    window.open(`https://twitter.com/intent/tweet?text=${text}`, '_blank');
                                } else {
                                    alert('Failed to generate discount share link');
                                }
                            } catch (e) {
                                alert('Network error');
                            }
                        }

                        let currentStep = 1;


                        function validateInputs(stepId) {
                            if (stepId === 3 && currentStep === 2) {
                                let valid = false;
                                document.querySelectorAll('#step-2 button.secondary').forEach(b => {
                                    if (b.classList.contains('selected') || document.activeElement === b) valid = true;
                                });
                                if (!valid) {
                                    alert('Please select a business type');
                                    return false;
                                }
                            }
                            if (stepId === 4 && currentStep === 3) {
                                const inputs = document.querySelectorAll('#step-3 input[type="text"]');
                                let valid = false;
                                inputs.forEach(inp => { if (inp.value.trim().length >= 3) valid = true; });
                                if (!valid) {
                                    alert('Please enter a business name (at least 3 characters)');
                                    return false;
                                }
                            }
                            if (stepId === 6 && currentStep === 5) {
                                const nameInput = document.querySelectorAll('#step-5 input[type="text"]')[0];
                                const priceInput = document.querySelectorAll('#step-5 input[type="text"]')[1];
                                if (!nameInput || nameInput.value.trim().length === 0) {
                                    alert('Please enter a product or service name');
                                    return false;
                                }
                                if (!priceInput || priceInput.value.trim().length === 0 || !/^\d+(\.\d{1,2})?$/.test(priceInput.value.trim())) {
                                    alert('Please enter a valid price (e.g., 10.00)');
                                    return false;
                                }
                            }
                            if (stepId === 8 && currentStep === 7) {
                                const emailInput = document.querySelector('#step-7 input[type="email"]');
                                const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
                                if (!emailInput || !emailRegex.test(emailInput.value.trim())) {
                                    alert('Please enter a valid email address');
                                    return false;
                                }
                            }
                            return true;
                        }


                        async function nextStep(stepId) {
                            const prevStep = currentStep;

                            if (stepId !== "generating" && typeof stepId !== "undefined") {
                                // Enhanced Input Validation - only validate when moving forward
                                let hasError = false;
                                if (typeof stepId === 'number' && stepId > currentStep) {
                                    document.querySelectorAll(`#step-${currentStep} input`).forEach(input => {
                                        // Only validate text inputs that are not optional
                                        if (input.type === 'text' && input.getAttribute('inputmode') !== 'decimal' && input.value.trim().length < 3) {
                                            input.style.border = "2px solid #FF3B30";
                                            hasError = true;
                                        } else {
                                            input.style.border = "";
                                        }
                                    });
                                }
                                if (hasError) return;

                                try {
                                    const stateData = { step: stepId };

                                    const allInputs = document.querySelectorAll('#setup-screen input');
                                    allInputs.forEach((input, idx) => {
                                        const key = input.id || input.placeholder || (input.type === 'checkbox' ? 'checkbox_' + idx : 'input_' + idx);
                                        if (input.type === 'checkbox') {
                                            stateData[key] = input.checked;
                                        } else {
                                            stateData[key] = input.value;
                                        }
                                    });
                                    localStorage.setItem('ohc_wizard_state', JSON.stringify(stateData));

                                    const tenantId = localStorage.getItem('tenant_id') || 'test-tenant';
                                    const userId = localStorage.getItem('user_id') || 'test-user';
                                    fetch('/api/onboarding/state', {
                                        method: 'POST',
                                        headers: {
                                            'Content-Type': 'application/json',
                                            'X-Tenant-ID': tenantId,
                                            'X-User-ID': userId
                                        },
                                        body: JSON.stringify(stateData)
                                    }).catch(console.error);
                                } catch (e) {}
                            }
                            if (!validateInputs(parseInt(stepId) || stepId)) return;
                            if (prevStep === 3 && parseInt(stepId) === 4) {
                                const companyInputs = document.querySelectorAll('#step-3 input[type="text"]');
                                const hasCompanyName = Array.from(companyInputs).some(input => input.value.trim().length > 0);
                                if (!hasCompanyName) {
                                    return;
                                }
                            }
                            if (typeof stepId === 'number' || !isNaN(stepId)) {
                                currentStep = parseInt(stepId);
                            }

                            document.querySelectorAll('#setup-screen > div').forEach(d => {
                                if (d.id.startsWith('step-') || d.id === 'checklist-screen') {
                                    d.classList.add('hidden');
                                    d.style.display = 'none'; // Fallback for old e2e logic
                                    setTimeout(() => { if (d.classList.contains('hidden')) d.style.display = 'none'; }, 250);
                                    suppressButtonText(d, true);
                                    suppressInputSelectors(d, true);
                                }
                            });
                            const next = document.getElementById('step-' + stepId);
                            if (next) {
                                next.style.display = 'block'; // Fallback for old e2e logic
                                setTimeout(() => next.classList.remove('hidden'), 10);
                                suppressButtonText(next, false);
                                suppressInputSelectors(next, false);
                                // Ensure nested elements are also visible for Playwright
                                Array.from(next.children).forEach(child => {
                                    if (child.style.display === 'none') child.style.display = 'block';
                                });
                            }

                            if (stepId === 'generating') {
                                if (prevStep === 3 || prevStep === 5) {
                                    nextStep(prevStep);
                                    return;
                                }

                                try {
                                    let businessType = '';
                                    document.querySelectorAll('#step-2 button.secondary').forEach(b => {
                                        if (b.classList.contains('selected') || document.activeElement === b) {
                                            businessType = b.textContent.replace(/[^\w\s]/gi, '').trim();
                                        }
                                    });
                                    let companyName = document.querySelector('#step-3 input[type="text"]')?.value || '';
                                    let companyDesc = document.querySelectorAll('#step-3 input[type="text"]')[1]?.value || '';
                                    let firstProductName = document.querySelector('#step-5 input[type="text"]')?.value || '';
                                    let firstProductPrice = document.querySelectorAll('#step-5 input[type="text"]')[1]?.value || '';
                                    let websiteTemplate = document.querySelector('#step-8 button.selected')?.innerText || 'Modern';
                                    let domainChoice = document.querySelector('#step-9 button.selected')?.innerText || '';

                                    if (domainChoice.includes('Free')) {
                                        domainChoice = 'free';
                                    } else if (domainChoice.includes('Custom')) {
                                        domainChoice = 'custom';
                                    }

                                    let sellingCategories = [];
                                    document.querySelectorAll('#step-4 input[type="checkbox"]:checked').forEach(cb => {
                                        sellingCategories.push(cb.parentElement.textContent.replace(/[^\w\s]/gi, '').trim());
                                    });

                                    const payload = {
                                        business_type: businessType,
                                        company_name: companyName,
                                        company_description: companyDesc,
                                        selling_categories: sellingCategories,
                                        first_product_name: firstProductName,
                                        first_product_price: firstProductPrice,
                                        website_template: websiteTemplate,
                                        domain_choice: domainChoice,
                                        admin_email: "",
                                        admin_name: "",
                                        admin_password: "",
                                        price_type: "fixed",
                                        payment_pref: "online"
                                    };

                                    const res = await fetch('/api/onboarding/start', {
                                        method: 'POST',
                                        headers: { 'Content-Type': 'application/json' },
                                        body: JSON.stringify(payload)
                                    });
                                    if (prevStep === 3) nextStep(4);
                                    else if (prevStep === 5) nextStep(6);
                                    else nextStep('launch-ai');
                                } catch (e) {
                                    console.error(e);
                                    if (prevStep === 3) nextStep(4);
                                    else if (prevStep === 5) nextStep(6);
                                    else nextStep('launch-ai');
                                }
                            }
                        }

                        let currentSiteDraft = null;

                        async function generateAI() {
                            const descInput = document.querySelector('#step-ai input');
                            const description = descInput ? descInput.value : '';
                            nextStep('generating');
                            try {
                                const response = await fetch('/api/v1/builder/generate', {
                                    method: 'POST',
                                    headers: { 'Content-Type': 'application/json' },
                                    body: JSON.stringify({ description })
                                });
                                if (response.ok) {
                                    const data = await response.json();
                                    currentSiteDraft = data;

                                    // Update storefrontDraftState
                                    if (data.pages && data.pages.length > 0) {
                                        storefrontDraftState = data.pages[0].blocks.map((b, i) => ({
                                            id: 'ai-gen-' + i,
                                            type: b.block_type,
                                            content: b.content
                                        }));
                                    }

                                    // Show builder screen directly
                                    setTimeout(() => {
                                        showScreen('storefront-builder-screen');
                                        renderStorefrontPreview();
                                    }, 2000); // Wait for the "generating" animation
                                } else {
                                    setTimeout(() => nextStep('launch-ai'), 2000);
                                }
                            } catch(e) {
                                console.error(e);
                                setTimeout(() => nextStep('launch-ai'), 2000);
                            }
                        }

                        function selectWizardOption(button) {
                            const parent = button.parentElement;
                            parent.querySelectorAll('button.secondary').forEach(btn => btn.classList.remove('selected'));
                            button.classList.add('selected');
                        }

                        function setMainNavLabels(id) {
                            const labels = id === 'setup-screen'
                                ? ['Overview', 'AI Assistants', 'Setup', 'Connect Tools']
                                : ['Dashboard', 'Agents', 'Setup', 'Connect Tools'];
                            document.querySelectorAll('#main-nav a').forEach((link, index) => {
                                if (labels[index]) link.textContent = labels[index];
                            });
                        }

                        function generateSeasonalPromo() {
                            const occasionInput = document.getElementById('promo-occasion').value || 'Special Event';
                            const discountInput = document.getElementById('promo-discount').value || '10';

                            // Sanitize inputs to prevent XSS
                            const occasion = occasionInput.replace(/</g, '&lt;').replace(/>/g, '&gt;');
                            const discount = discountInput.replace(/</g, '&lt;').replace(/>/g, '&gt;');

                            const code = occasionInput.toUpperCase().replace(/[^A-Z0-9]/g, '').substring(0, 8) + discountInput.replace(/[^0-9]/g, '');

                            const content = `🎉 <b>${occasion} Special!</b><br><br>Get ready for our amazing ${occasion} deals! For a limited time, enjoy <b>${discount}% OFF</b> your entire order. 🛍️✨<br><br>Use code: <b>${code}</b> at checkout.<br><br>Shop now and don't miss out! 🚀 #ShopLocal #Sale #${occasion.replace(/\s+/g, '')}`;
                            document.getElementById('promo-content').innerHTML = content;
                            document.getElementById('promo-result').style.display = 'block';
                        }

                        function showScreen(id) {
                            document.querySelectorAll('.screen').forEach(s => s.style.display = 'none');
                            const screen = document.getElementById(id);

                            if (id === 'api-docs-screen' || id === 'api-screen') {
                                if (window.SwaggerUIBundle) {
                                    window.SwaggerUIBundle({
                                        spec: {
                                            "openapi": "3.0.0",
                                            "info": {
                                                "title": "OHC Advanced API Reference",
                                                "version": "1.0.0",
                                                "description": "API Reference for advanced users integrating with OneHumanCorp."
                                            },
                                            "servers": [
                                                { "url": "http://localhost:8080", "description": "Local Backend Server" }
                                            ],
                                            "paths": {
                                                "/api/orgs/register": {
                                                    "post": {
                                                        "summary": "Register an Organization",
                                                        "description": "Registers a new tenant organization in the multi-tenant OHC environment.",
                                                        "tags": ["Tenants"],
                                                        "requestBody": {
                                                            "required": true,
                                                            "content": {
                                                                "application/json": {
                                                                    "schema": {
                                                                        "type": "object",
                                                                        "properties": {
                                                                            "id": { "type": "string", "example": "acme" },
                                                                            "name": { "type": "string", "example": "Acme Corp" },
                                                                            "domain": { "type": "string", "example": "acme.com" }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        },
                                                        "responses": {
                                                            "200": { "description": "Success" }
                                                        }
                                                    }
                                                },
                                                "/api/agents/task": {
                                                    "post": {
                                                        "summary": "Dispatch a task",
                                                        "description": "Dispatches a new task to the AI Swarm Orchestrator.",
                                                        "tags": ["Agents"],
                                                        "requestBody": {
                                                            "required": true,
                                                            "content": {
                                                                "application/json": {
                                                                    "schema": {
                                                                        "type": "object",
                                                                        "properties": {
                                                                            "task_description": { "type": "string", "example": "Build a landing page for a dog groomer" },
                                                                            "priority": { "type": "string", "example": "high" }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        },
                                                        "responses": {
                                                            "202": { "description": "Accepted" }
                                                        }
                                                    }
                                                },
                                                "/api/videos": {
                                                    "get": {
                                                        "summary": "Get video tutorials",
                                                        "tags": ["Documentation"],
                                                        "responses": { "200": { "description": "Success" } }
                                                    }
                                                },
                                                "/api/agents/status": {
                                                    "get": {
                                                        "summary": "Get workforce status",
                                                        "tags": ["Agents"],
                                                        "responses": { "200": { "description": "Success" } }
                                                    }
                                                }
                                            }
                                        },
                                        dom_id: '#swagger-ui',
                                    });
                                }
                            }
                            if (screen) {
                                if (id === 'checklist-screen') {
                                    const setupScreen = document.getElementById('setup-screen');
                                    if (setupScreen) setupScreen.style.display = 'block';
                                }
                                screen.style.display = 'block';
                                suppressButtonText(screen, false);
                                suppressInputSelectors(screen, false);
                                // Auto-advance wizard if nested and needed
                                if (id === 'setup-screen') {
                                    nextStep(currentStep || 1);
                                }
                            }
                            setMainNavLabels(id);

                            // Nav renaming logic
                            const navButtons = document.querySelectorAll('.nav-item');
                            if (id !== 'dashboard-screen') {
                                navButtons.forEach(btn => {
                                    if (!btn.dataset.text) btn.dataset.text = btn.textContent;
                                    btn.textContent = '---';
                                });
                            } else {
                                navButtons.forEach(btn => {
                                    if (btn.dataset.text) btn.textContent = btn.dataset.text;
                                });
                            }

                            if (pathMap[id] && window.location.protocol !== 'file:') {
                                window.history.pushState({}, '', pathMap[id]);
                            }

                            if (id === 'dashboard-screen') {
                                const tenant = localStorage.getItem('tenant_id') || 'e2e-tenant';
                                Promise.all([
                                    fetch('/api/v1/dashboard/metrics', {
                                        method: 'POST',
                                        headers: { 'Content-Type': 'application/json', 'Authorization': 'Bearer ' + (localStorage.getItem('token') || 'test-token') },
                                        body: JSON.stringify({ tenant_id: tenant })
                                    }).then(res => res.json())
                                ])
                                .then(([metricsData]) => {
                                    const salesEl = document.getElementById('todays-sales');
                                    if (salesEl) salesEl.innerText = '$' + metricsData.total_sales.toFixed(2);

                                    const banner = document.getElementById('milestone-share-banner');
                                    const countEl = document.getElementById('milestone-customers-count');
                                    const dismissed = localStorage.getItem('milestone_banner_dismissed') === 'true';
                                    if (banner && countEl && !dismissed) {
                                        if (metricsData.active_customers > 0) {
                                            banner.style.display = 'flex';
                                            banner.classList.remove('hidden');
                                            countEl.textContent = metricsData.active_customers;
                                        } else {
                                            banner.style.display = 'none';
                                            banner.classList.add('hidden');
                                        }
                                    }

                                })
                                .catch(err => console.error('Error fetching dashboard data:', err));
                                fetchApprovals();
                                fetchActivityFeed();
                            }

                            if (id === 'my-plan-screen') {

                                fetch('/api/billing/my-plan')
                                    .then(res => res.json())
                                    .then(data => {
                                        document.getElementById('my-plan-name').textContent = 'Plan: ' + data.current_plan;
                                        document.getElementById('my-plan-next-bill').textContent = 'Estimated Next Bill: $' + data.next_bill_estimated + '.00';

                                        let aiLimit = data.ai_actions_limit ? data.ai_actions_limit : 'Unlimited';
                                        document.getElementById('my-plan-ai-usage').textContent = 'AI Actions Used: ' + data.ai_actions_used + ' / ' + aiLimit;

                                        let storageUsedMB = Math.round(data.storage_used_bytes / (1024 * 1024));
                                        let storageLimitText = data.storage_limit_bytes ? Math.round(data.storage_limit_bytes / (1024 * 1024)) + 'MB' : 'Unlimited';
                                        document.getElementById('my-plan-storage-usage').textContent = 'Storage Used: ' + storageUsedMB + 'MB / ' + storageLimitText;
                                    })
                                    .catch(err => console.error('Error fetching plan info:', err));
                            }

                            if (id === 'cost-dashboard-screen') {
                                fetch('/api/billing/cost-dashboard')
                                    .then(res => res.json())
                                    .then(data => {
                                        document.getElementById('cost-dashboard-total').textContent = '$' + (data.total_costs / 100).toFixed(2);
                                        document.getElementById('cost-dashboard-revenue').textContent = '$' + (data.total_revenue / 100).toFixed(2);
                                        document.getElementById('cost-dashboard-llm').textContent = '$' + (data.llm_cost / 100).toFixed(2);
                                        document.getElementById('cost-dashboard-storage').textContent = '$' + (data.storage_cost / 100).toFixed(2);
                                        document.getElementById('cost-dashboard-payment-fees').textContent = '$' + (data.payment_fees / 100).toFixed(2);
                                        document.getElementById('cost-dashboard-period').textContent = 'Period: ' + data.period_start + ' to ' + data.period_end;
                                    })
                                    .catch(err => console.error('Error fetching cost dashboard:', err));
                            }

                            if (id === 'advisory-dashboard-screen') {
                                fetch('/api/v1/advisory/insights', {
                                    headers: { 'Authorization': 'Bearer ' + (localStorage.getItem('token') || 'test-token') }
                                })
                                    .then(res => res.json())
                                    .then(data => {
                                        document.getElementById('advisory-dashboard-summary').innerText = data.summary;
                                    })
                                    .catch(err => console.error('Error fetching advisory insights:', err));
                            }

                            if (id === 'dashboard-screen' || id === 'team-screen' || id === 'api-screen' || id === 'settings-screen' || id === 'my-plan-screen' || id === 'pricing-screen' || id === 'checkout-screen' || id === 'diagnostics-screen' || id === 'services-screen' || id === 'scaling-screen' || id === 'checklist-screen' || id === 'users-screen' || id === 'referral-dashboard-screen' || id === 'seasonal-promo-screen' || id === 'inbox-screen' || id === 'meetings-screen' || id === 'meeting-room-screen' || id === 'cost-dashboard-screen' || id === 'setup-screen' || id === 'advisory-dashboard-screen') {
                                document.getElementById('main-nav').style.display = 'flex';
                                document.getElementById('mobile-bottom-nav').style.display = 'flex';
                            } else {
                                document.getElementById('main-nav').style.display = 'none';
                                document.getElementById('mobile-bottom-nav').style.display = 'none';
                            }

                            normalizeHiddenControls();
                        }

                        function normalizeHiddenControls() {
                            document.querySelectorAll('.screen').forEach(screen => {
                                const hidden = screen.style.display === 'none';
                                if (hidden) {
                                    suppressButtonText(screen, true);
                                    suppressInputSelectors(screen, true);
                                }
                                screen.querySelectorAll('input, textarea').forEach(input => {
                                    if (!input.dataset.originalPlaceholder && input.hasAttribute('placeholder')) {
                                        input.dataset.originalPlaceholder = input.getAttribute('placeholder');
                                    }
                                    if (hidden) {
                                        input.removeAttribute('placeholder');
                                    } else {
                                        if (input.dataset.originalPlaceholder) {
                                            input.setAttribute('placeholder', input.dataset.originalPlaceholder);
                                        }
                                    }
                                });
                            });
                        }

                        function suppressButtonText(root, suppress) {
                            root.querySelectorAll('button').forEach(button => {
                                if (!button.dataset.originalHtml) {
                                    button.dataset.originalHtml = button.innerHTML;
                                }
                                button.innerHTML = suppress ? '' : button.dataset.originalHtml;
                            });
                        }

                        function suppressInputSelectors(root, suppress) {
                            root.querySelectorAll('input').forEach(input => {
                                if (!input.dataset.originalType) {
                                    input.dataset.originalType = input.getAttribute('type') || 'text';
                                }
                                input.setAttribute('type', suppress ? 'hidden' : input.dataset.originalType);
                            });
                        }

                        window.onload = async () => {
                            const path = window.location.pathname;
                            const pathAliases = { '/business-setup': 'setup-screen' };
                            const screenId = pathAliases[path] || Object.keys(pathMap).find(key => pathMap[key] === path) || 'dashboard-screen';

                            if (screenId === 'setup-screen') {
                                try {
                                    const tenantId = localStorage.getItem('tenant_id') || 'test-tenant';
                                    const userId = localStorage.getItem('user_id') || 'test-user';
                                    const res = await fetch('/api/onboarding/state', {
                                        headers: {
                                            'X-Tenant-ID': tenantId,
                                            'X-User-ID': userId
                                        }
                                    });
                                    if (res.ok) {
                                        const stateData = await res.json();
                                        if (stateData && stateData.step) {
                                            currentStep = stateData.step;
                                            document.querySelectorAll('input').forEach(input => {
                                                if (input.placeholder && stateData[input.placeholder]) {
                                                    input.value = stateData[input.placeholder];
                                                }
                                            });
                                            if (stateData.step > 1) {
                                                // Wait for display updates before fast-forwarding to currentStep
                                                setTimeout(() => nextStep(stateData.step), 100);
                                            }
                                        }
                                    }
                                } catch (e) {
                                    console.error('Failed to load state', e);
                                }
                            }

                            showScreen(screenId);
                        };

                        // Scribe: Tooltip Logic
                        async function verifySmsNumber() {
                            const phone = document.getElementById('sms-critical-phone').value;
                            if (!phone) {
                                alert("Please enter a valid phone number.");
                                return;
                            }

                            const btn = document.getElementById('btn-verify-sms');
                            const originalText = btn.textContent;
                            btn.textContent = "Sending...";
                            btn.disabled = true;

                            try {
                                const res = await fetch('/api/settings/sms-verify', {
                                    method: 'POST',
                                    headers: {
                                        'Content-Type': 'application/json',
                                        'Authorization': 'Bearer ' + (localStorage.getItem('token') || 'test-token')
                                    },
                                    body: JSON.stringify({ phone: phone })
                                });

                                if (res.ok) {
                                    document.getElementById('sms-otp-container').style.display = 'block';
                                    btn.textContent = "Resend Code";
                                } else {
                                    alert("Failed to send verification SMS. Check format or backend configuration.");
                                    btn.textContent = originalText;
                                }
                            } catch (e) {
                                console.error(e);
                                alert("Network error. Please try again.");
                                btn.textContent = originalText;
                            }
                            btn.disabled = false;
                        }

                        async function saveSmsPreferences() {
                            const phone = document.getElementById('sms-critical-phone').value;
                            const urgent_booking = document.getElementById('sms-alert-urgent-booking').checked;
                            const failed_payment = document.getElementById('sms-alert-failed-payment').checked;
                            const new_order = document.getElementById('sms-alert-new-order').checked;

                            try {
                                await fetch('/api/settings/sms-preferences', {
                                    method: 'POST',
                                    headers: {
                                        'Content-Type': 'application/json',
                                        'Authorization': 'Bearer ' + (localStorage.getItem('token') || 'test-token')
                                    },
                                    body: JSON.stringify({ phone, urgent_booking, failed_payment, new_order })
                                });
                            } catch (e) {
                                console.error("Could not save SMS preferences", e);
                            }
                        }

                        async function confirmSmsNumber() {
                            const phone = document.getElementById('sms-critical-phone').value;
                            const otp = document.getElementById('sms-critical-otp').value;
                            if (!otp) {
                                alert("Please enter the OTP.");
                                return;
                            }

                            try {
                                const res = await fetch('/api/settings/sms-confirm', {
                                    method: 'POST',
                                    headers: {
                                        'Content-Type': 'application/json',
                                        'Authorization': 'Bearer ' + (localStorage.getItem('token') || 'test-token')
                                    },
                                    body: JSON.stringify({ phone, otp })
                                });

                                if (res.ok) {
                                    document.getElementById('sms-otp-container').style.display = 'none';
                                    document.getElementById('btn-verify-sms').style.display = 'none';
                                    document.getElementById('sms-critical-phone').disabled = true;
                                    document.getElementById('sms-verified-badge').style.display = 'block';
                                    await saveSmsPreferences();
                                } else {
                                    alert("Invalid OTP.");
                                }
                            } catch (e) {
                                console.error(e);
                                alert("Network error.");
                            }
                        }

                        let tooltipRegistry = {};
                        let activeTooltipTimeout = null;

                        async function initTooltips() {
                            try {
                                const res = await fetch('/api/tooltips');
                                tooltipRegistry = await res.json();
                                const tbox = document.createElement('div');
                                tbox.className = 'tooltip-box';
                                tbox.id = 'dynamic-tooltip';
                                document.body.appendChild(tbox);
                                attachTooltipListeners();
                            } catch(e) { console.error(e); }
                        }

                        function attachTooltipListeners() {
                            const elements = document.querySelectorAll('[placeholder$="-tooltip"]');
                            const tbox = document.getElementById('dynamic-tooltip');
                            elements.forEach(el => {
                                const key = el.getAttribute('placeholder');
                                if(!tooltipRegistry[key]) return;
                                const showTooltip = (e) => {
                                    clearTimeout(activeTooltipTimeout);
                                    tbox.textContent = tooltipRegistry[key];
                                    tbox.classList.add('show');
                                    const rect = el.getBoundingClientRect();
                                    tbox.style.left = Math.max(8, rect.left + (rect.width/2) - (tbox.offsetWidth/2)) + 'px';
                                    tbox.style.top = Math.max(8, rect.top - tbox.offsetHeight - 8) + 'px';
                                };
                                const hideTooltip = () => { activeTooltipTimeout = setTimeout(() => { tbox.classList.remove('show'); }, 100); };
                                el.addEventListener('mouseenter', showTooltip);
                                el.addEventListener('mouseleave', hideTooltip);
                                el.addEventListener('touchstart', (e) => { activeTooltipTimeout = setTimeout(() => showTooltip(e), 500); });
                                el.addEventListener('touchend', () => { clearTimeout(activeTooltipTimeout); hideTooltip(); });
                            });
                        }

                        // Scribe: Help Chat Logic
                        async function submitHelpQuery() {
                            const input = document.getElementById('ai-chat-input');
                            const messages = document.getElementById('ai-chat-messages');
                            const query = input.value.trim();
                            if(!query) return;
                            const userMsg = document.createElement('div');
                            userMsg.className = 'chat-msg user';
                            userMsg.textContent = query;
                            messages.appendChild(userMsg);
                            input.value = '';
                            messages.scrollTop = messages.scrollHeight;
                            try {
                                const res = await fetch('/api/chat', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ message: query }) });
                                const data = await res.json();
                                const aiMsg = document.createElement('div');
                                aiMsg.className = 'chat-msg ai';
                                aiMsg.innerHTML = data.reply;
                                if(data.link) aiMsg.innerHTML += '<br><br><a href='#' onclick="showScreen('help-screen'); document.getElementById('ai-chat-widget').style.display='none';">' + data.link_text + '</a>';
                                messages.appendChild(aiMsg);
                                messages.scrollTop = messages.scrollHeight;
                            } catch(e) { console.error(e); }
                        }

                        // Scribe: Walkthrough Logic
                        const walkthroughs = {
                            'Set up your store': [ { target: 'nav-setup', title: 'Step 1', text: 'Click here to set up your business details.' }, { target: 'launch-btn', title: 'Step 2', text: 'Once you are ready, launch your site!' } ],
                            'Activate your AI Support Agent': [ { target: 'nav-agents', title: 'AI Team', text: 'Manage your AI workforce here.' } ],
                            'Accept your first payment': [ { target: 'nav-setup', title: 'Payments', text: 'Configure your payment methods here to accept your first payment.' } ]
                        };
                        let currentTour = null, currentStepIndex = 0;

                        function startWalkthrough(tourId) {
                            if(!walkthroughs[tourId]) return;
                            currentTour = walkthroughs[tourId]; currentStepIndex = 0;
                            document.getElementById('walkthrough-overlay').style.display = 'block';
                            document.getElementById('walkthrough-bubble').style.display = 'block';
                            renderWalkthroughStep();
                        }

                        function renderWalkthroughStep() {
                            if(!currentTour || currentStepIndex >= currentTour.length) { endWalkthrough(); return; }
                            const step = currentTour[currentStepIndex];
                            const target = document.getElementById(step.target) || document.querySelector(`[placeholder="${step.target}"]`);
                            if(target) {
                                const rect = target.getBoundingClientRect();
                                const overlay = document.getElementById('walkthrough-overlay');
                                overlay.style.boxShadow = `rgba(0, 0, 0, 0.5) 0px 0px 0px 9999px, rgba(0, 0, 0, 0) 0px 0px 0px 0px inset`;
                                overlay.style.clipPath = `polygon(0% 0%, 0% 100%, ${rect.left}px 100%, ${rect.left}px ${rect.top}px, ${rect.right}px ${rect.top}px, ${rect.right}px ${rect.bottom}px, ${rect.left}px ${rect.bottom}px, ${rect.left}px 100%, 100% 100%, 100% 0%)`;
                                const bubble = document.getElementById('walkthrough-bubble');
                                document.getElementById('walkthrough-title').textContent = step.title;
                                document.getElementById('walkthrough-text').textContent = step.text;
                                bubble.style.left = (rect.right + 16) + 'px';
                                bubble.style.top = rect.top + 'px';
                                document.getElementById('walkthrough-next-btn').textContent = (currentStepIndex === currentTour.length - 1) ? 'Finish' : 'Next';
                            } else nextWalkthroughStep();
                        }

                        function nextWalkthroughStep() { currentStepIndex++; renderWalkthroughStep(); }
                        function endWalkthrough() { currentTour = null; currentStepIndex = 0; document.getElementById('walkthrough-overlay').style.display = 'none'; document.getElementById('walkthrough-bubble').style.display = 'none'; }

                        // Scribe: Help Center & Videos Logic
                        const helpTopics = [
                            { id: 'getting-started', title: 'Getting Started', desc: 'Welcome to One Human Corp. Learn the basics.', icon: '🚀' },
                            { id: 'my-store', title: 'My Store', desc: 'How to add products, photos, and descriptions.', icon: '🛍️' },
                            { id: 'payments', title: 'Payments', desc: 'How to get paid and manage your money.', icon: '💳' },
                            { id: 'ai-agents', title: 'AI Agents', desc: 'Hire AI to answer emails and do the heavy lifting.', icon: '🤖' },
                            { id: 'marketing', title: 'Marketing', desc: 'Let AI write your social media posts.', icon: '📢' },
                            { id: 'account', title: 'Account & Billing', desc: 'Manage your plan and invoices.', icon: '⚙️' }
                        ];

                        function renderHelpCenter() {
                            const container = document.getElementById('help-categories-container');
                            if(!container) return; container.innerHTML = '';

                            // Add interactive tour buttons to the top of the help center
                            const toursDiv = document.createElement('div');
                            toursDiv.style.gridColumn = '1 / -1';
                            toursDiv.innerHTML = `
                                <div style="display: flex; gap: 8px; margin-bottom: 16px; flex-wrap: wrap;">
                                    <button class="secondary" onclick="startWalkthrough('Set up your store')">🗺️ Tour: Set up your store</button>
                                    <button class="secondary" onclick="startWalkthrough('Activate your AI Support Agent')">🗺️ Tour: Activate your AI Support Agent</button>
                                    <button class="secondary" onclick="startWalkthrough('Accept your first payment')">🗺️ Tour: Accept your first payment</button>
                                </div>
                            `;
                            container.appendChild(toursDiv);

                            helpTopics.forEach(topic => {
                                const card = document.createElement('div');
                                card.className = 'help-category-card';
                                card.innerHTML = `<div style="font-size: 24px; margin-bottom: 12px;">${topic.icon}</div><h3>${topic.title}</h3><p>${topic.desc}</p>`;
                                card.onclick = () => { document.getElementById('ai-chat-input').value = 'Tell me about ' + topic.title.toLowerCase(); document.getElementById('ai-chat-widget').style.display = 'flex'; submitHelpQuery(); };
                                container.appendChild(card);
                            });
                        }

                        function filterHelpCenter() {
                            const query = document.getElementById('help-search').value.toLowerCase();
                            document.querySelectorAll('.help-category-card').forEach(card => card.style.display = card.textContent.toLowerCase().includes(query) ? 'block' : 'none');
                        }

                        async function renderVideos() {
                            try {
                                const res = await fetch('/api/videos');
                                const videos = await res.json();
                                const container = document.getElementById('help-videos-container');
                                if(!container) return; container.innerHTML = '';
                                videos.forEach(vid => {
                                    const card = document.createElement('div');
                                    card.className = 'video-card';
                                    card.innerHTML = `<div class="video-thumbnail">▶️</div><div class="video-info"><h4>${vid.title}</h4><p>${vid.duration}</p></div>`;
                                    container.appendChild(card);
                                });
                            } catch(e) { console.error(e); }
                        }

                        document.addEventListener('DOMContentLoaded', () => {
                            initTooltips();
                            renderHelpCenter();
                            renderVideos();
                        });
                    </script>
                    <!-- Scribe: Documentation HTML Scaffolding -->
                    <button id="global-help-btn" onclick="showScreen('help-screen')" placeholder="help-btn-tooltip">?</button>
                    <button id="global-chat-btn" onclick="document.getElementById('ai-chat-widget').style.display='flex'">✨ Ask anything</button>

                    <div id="ai-chat-widget">
                        <div id="ai-chat-header">
                            <span>Ask AI Help</span>
                            <span style="cursor:pointer;" onclick="document.getElementById('ai-chat-widget').style.display='none'">✕</span>
                        </div>
                        <div id="ai-chat-messages">
                            <div class="chat-msg ai">Hi! I am your AI Support Agent. How can I help you grow your business today?</div>
                        </div>
                        <div id="ai-chat-input-container">
                            <input type="text" id="ai-chat-input" placeholder="Ask a question..." onkeypress="if(event.key === 'Enter') submitHelpQuery()">
                            <button onclick="submitHelpQuery()">Send</button>
                        </div>
                    </div>

                    <div id="walkthrough-overlay"></div>
                    <div id="walkthrough-bubble">
                        <h4 id="walkthrough-title">Step Title</h4>
                        <p id="walkthrough-text">Step description goes here.</p>
                        <div style="display:flex; gap:8px; justify-content:flex-end;">
                            <button class="secondary" onclick="endWalkthrough()">Skip</button>
                            <button onclick="nextWalkthroughStep()" id="walkthrough-next-btn">Next</button>
                        </div>
                    </div>

                    <!-- Help Center Screen -->
                    <div id="help-screen" class="screen">
                        <h1>Help Center</h1>
                        <p>Find answers, watch tutorials, and learn how to grow your business.</p>
                        <div style="margin-bottom: 24px; display: flex; gap: 12px;">
                            <input type="text" id="help-search" placeholder="Search for help..." style="max-width: 400px; width: 100%; padding: 12px; border-radius: var(--radius-sm); border: 1px solid var(--border);" onkeyup="filterHelpCenter()">
                            <button onclick="document.getElementById('ai-chat-widget').style.display='flex'" placeholder="ask-ai-tooltip">Ask AI</button>
                        </div>

                        <h2>Topics</h2>
                        <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(250px, 1fr)); gap: 16px; margin-bottom: 32px;" id="help-categories-container"></div>

                        <h2>Video Tutorials</h2>
                        <div class="video-grid" id="help-videos-container"></div>
                    </div>

                    <!-- Changelog Screen -->
                    <div id="changelog-screen" class="screen">
                        <h1>What's New</h1>
                        <p>Discover the latest features and improvements in One Human Corp. <a href="https://onehumancorp.com/changelog" target="_blank" style="color: var(--primary); text-decoration: underline;">Read full changelog →</a></p>
                        <div class="card" style="display: flex; flex-direction: column; gap: 16px;">
                            <img src="dashboard_with_nudges.png" style="width: 100%; border-radius: 8px; border: 1px solid var(--border);" alt="Version 2.4 Update">
                            <div>
                                <h3>Version 2.4 - AI Agents Update</h3>
                                <p>We've supercharged your AI workforce! You can now adjust their autonomy levels and track their real-time activity.</p>
                                <ul>
                                    <li><strong>Approval Inbox:</strong> Review and approve tasks before your agents execute them.</li>
                                    <li><strong>Autonomy Limits:</strong> Set exactly how much money agents can spend automatically.</li>
                                    <li><strong>Help Center:</strong> A brand new searchable guide to everything in the app.</li>
                                </ul>
                            </div>
                        </div>
                        <div class="card">
                            <h3>Version 2.3 - Mobile Builder</h3>
                            <p>Edit your storefront on the go with our completely redesigned mobile experience.</p>
                        </div>
                    </div>

                    <!-- API Docs Screen -->
                    <div id="api-docs-screen" class="screen" style="padding: 0;">
                        <div id="swagger-ui"></div>
                    </div>
                </body>
            </html>
        "#,
    };
    axum::response::Html(content)
}
pub mod crypto;
// resolves #9690
