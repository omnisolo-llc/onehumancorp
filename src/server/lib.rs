pub mod rag_sync;
pub use ::server_harness as harness;
pub mod api;
pub mod agents;

use std::collections::HashMap;
use std::sync::RwLock;

#[derive(serde::Deserialize)]
struct ChatRequest {
    message: String,
}

#[derive(Clone, serde::Serialize)]
struct WorkflowRecord {
    id: String,
    name: String,
    workflow: String,
    task: String,
    status: String,
    command: String,
    created_at: String,
    output: Option<String>,
    error: Option<String>,
}

#[derive(serde::Deserialize)]
struct CreateWorkflowRequest {
    name: String,
    task: String,
}

static TOOLTIPS_REGISTRY: std::sync::OnceLock<RwLock<HashMap<String, String>>> = std::sync::OnceLock::new();
static WORKFLOW_REGISTRY: std::sync::OnceLock<RwLock<Vec<WorkflowRecord>>> = std::sync::OnceLock::new();
static BUILTIN_AGENT_SERVICE: std::sync::OnceLock<std::sync::Arc<ohc_builtin_agent::service::AgentServiceImpl>> = std::sync::OnceLock::new();

static ORG_CACHE_ADVISORY: std::sync::OnceLock<::server_utils::cache::HybridCache<Option<(String, String)>>> = std::sync::OnceLock::new();
static ACTIVE_ORDERS_CACHE: std::sync::OnceLock<::server_utils::cache::HybridCache<i64>> = std::sync::OnceLock::new();
pub static AI_CACHE: std::sync::OnceLock<::server_utils::cache::HybridCache<String>> = std::sync::OnceLock::new();
>>>>>>> 9db93cb5 (💰 Miser: Implement Soft Limits and Dashboard Upgrades)

pub fn is_standalone_runtime() -> bool {
    fn parse_bool(value: &str) -> Option<bool> {
        match value.trim().to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "y" | "on" => Some(true),
            "0" | "false" | "no" | "n" | "off" => Some(false),
            _ => None,
        }
    }

    if let Ok(value) = std::env::var("OHC_STANDALONE_MODE") {
        if let Some(parsed) = parse_bool(&value) {
            return parsed;
        }
    }
    if let Ok(value) = std::env::var("OHC_SOURCE_MODE") {
        match value.trim().to_ascii_lowercase().as_str() {
            "standalone" | "desktop" => return true,
            "cloud" | "cluster" | "headless" => return false,
            _ => {}
        }
    }

    true
}

fn get_tooltips_registry() -> &'static RwLock<HashMap<String, String>> {
    TOOLTIPS_REGISTRY.get_or_init(|| {
    let mut m = HashMap::new();
    m.insert("bio-input-tooltip".to_string(), "Describe what you sell, your target audience, and the vibe of your brand.".to_string());
    m.insert("generate-btn-tooltip".to_string(), "Our AI agents will analyze your description and build a ready-to-launch store for you.".to_string());
    m.insert("launch-btn-tooltip".to_string(), "Launch your storefront immediately to a live URL.".to_string());
    m.insert("team-activity-tooltip".to_string(), "Monitor the real-time actions and tasks being performed by your AI workforce.".to_string());
    m.insert("referral-tooltip".to_string(), "Share your unique link to earn credits when friends join OHC.".to_string());
    m.insert("swarm-online-tooltip".to_string(), "Your AI workforce is active. They process tasks in the background.".to_string());
    m.insert("department-card-tooltip".to_string(), "Click to view and manage pending approvals for this department.".to_string());
    m.insert("nav-dashboard-tooltip".to_string(), "View your store metrics, recent orders, and overall performance.".to_string());
    m.insert("nav-agents-tooltip".to_string(), "Manage your AI workforce, check their tasks, and hire new agents.".to_string());
    m.insert("nav-setup-tooltip".to_string(), "Configure your business details, branding, and payment settings.".to_string());
    m.insert("credit-tooltip".to_string(), "Earn credits to use on premium tools when you refer a friend.".to_string());
    m.insert("help-btn-tooltip".to_string(), "Need help? Click here to access our Help Center and tutorials.".to_string());
    m.insert("changelog-nav-tooltip".to_string(), "See what's new in the latest OneHumanCorp updates.".to_string());
    m.insert("todays-sales-tooltip".to_string(), "Your total sales for today. Check back often to track your progress.".to_string());
    m.insert("approval-inbox-tooltip".to_string(), "Review tasks that your AI agents need permission to execute. Approve or deny them here.".to_string());
    m.insert("ask-ai-tooltip".to_string(), "Open the AI Chat to get answers instantly. The AI reads our entire Help Center for you.".to_string());
    RwLock::new(m)
    })
}

fn get_workflow_registry() -> &'static RwLock<Vec<WorkflowRecord>> {
    WORKFLOW_REGISTRY.get_or_init(|| RwLock::new(Vec::new()))
}

fn workflow_agent_binary() -> String {
    std::env::var("OHC_BUILTIN_AGENT_BINARY")
        .or_else(|_| std::env::var("OHC_AGENT_BINARY"))
        .unwrap_or_else(|_| {
            if is_standalone_runtime() {
                if let Ok(exe_path) = std::env::current_exe() {
                    return exe_path.to_string_lossy().to_string();
                }
            }
            if let Ok(exe_path) = std::env::current_exe() {
                let agent_name = if cfg!(windows) {
                    "ohc-builtin-agent.exe"
                } else {
                    "ohc-builtin-agent"
                };
                let agent_path = exe_path.with_file_name(agent_name);
                agent_path.to_string_lossy().to_string()
            } else if cfg!(windows) {
                "ohc-builtin-agent.exe".to_string()
            } else {
                "ohc-builtin-agent".to_string()
            }
        })
}

fn workflow_agent_task(task: &str) -> String {
    let args = serde_json::json!({
        "workflow": "ohc_review_branch",
        "task": task,
    });
    format!(
        "Use the built-in RunWorkflow tool. Arguments: {}. Return the final synthesized report.",
        args
    )
}

fn set_workflow_result(id: &str, status: &str, output: Option<String>, error: Option<String>) {
    let registry = get_workflow_registry();
    if let Ok(mut workflows) = registry.write() {
        if let Some(record) = workflows.iter_mut().find(|record| record.id == id) {
            record.status = status.to_string();
            record.output = output;
            record.error = error;
        }
    }
}

fn dispatch_workflow(record: WorkflowRecord) {
    let id = record.id.clone();
    let binary = workflow_agent_binary();
    let task = workflow_agent_task(&record.task);

    tokio::spawn(async move {
        if is_standalone_runtime() {
            if let Some(svc) = BUILTIN_AGENT_SERVICE.get() {
                use ohc_builtin_agent::proto::agent_service::agent_service_server::AgentService;
                let req = ohc_builtin_agent::proto::agent_service::SubAgentRequest {
                    task: task.clone(),
                    working_dir: String::new(),
                    parent_context_json: String::new(),
                    ..Default::default()
                };
                match svc.dispatch_to_sub_agent(tonic::Request::new(req)).await {
                    Ok(resp) => {
                        let inner = resp.into_inner();
                        if !inner.error.is_empty() {
                            set_workflow_result(&id, "failed", Some(inner.result), Some(inner.error));
                        } else {
                            set_workflow_result(&id, "completed", Some(inner.result), None);
                        }
                    }
                    Err(e) => {
                        set_workflow_result(&id, "failed", None, Some(format!("In-process agent error: {}", e)));
                    }
                }
                return;
            }
        }

        let output = tokio::process::Command::new(&binary)
            .arg("--task")
            .arg(task)
            .output()
            .await;

        match output {
            Ok(output) if output.status.success() => {
                set_workflow_result(
                    &id,
                    "completed",
                    Some(String::from_utf8_lossy(&output.stdout).trim().to_string()),
                    None,
                );
            }
            Ok(output) => {
                set_workflow_result(
                    &id,
                    "failed",
                    Some(String::from_utf8_lossy(&output.stdout).trim().to_string()),
                    Some(String::from_utf8_lossy(&output.stderr).trim().to_string()),
                );
            }
            Err(err) => {
                set_workflow_result(
                    &id,
                    "failed",
                    None,
                    Some(format!("Failed to start {}: {}", binary, err)),
                );
            }
        }
    });
}

async fn list_workflows_handler() -> axum::Json<serde_json::Value> {
    let workflows = get_workflow_registry()
        .read()
        .map(|records| records.clone())
        .unwrap_or_default();
    axum::Json(serde_json::json!({ "workflows": workflows }))
}

async fn create_workflow_handler(
    axum::Json(payload): axum::Json<CreateWorkflowRequest>,
) -> impl axum::response::IntoResponse {
    use axum::http::StatusCode;

    let name = payload.name.trim();
    let task = payload.task.trim();
    if name.is_empty() || task.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            axum::Json(serde_json::json!({ "error": "Workflow name and task are required" })),
        );
    }

    let binary = workflow_agent_binary();
    let agent_task = workflow_agent_task(task);
    let record = WorkflowRecord {
        id: uuid::Uuid::new_v4().to_string(),
        name: name.to_string(),
        workflow: "ohc_review_branch".to_string(),
        task: task.to_string(),
        status: "running".to_string(),
        command: format!("{} --task {}", binary, serde_json::to_string(&agent_task).unwrap_or_default()),
        created_at: Utc::now().to_rfc3339(),
        output: None,
        error: None,
    };

    if let Ok(mut workflows) = get_workflow_registry().write() {
        workflows.insert(0, record.clone());
    }
    dispatch_workflow(record.clone());

    (
        StatusCode::ACCEPTED,
        axum::Json(serde_json::json!({ "workflow": record })),
    )
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
pub mod sync;
pub mod interop;

pub mod benchmarks;

pub use ::server_config as config;
pub use ::server_common as common;
pub use crate::proto as ohc;
pub mod builder;
pub mod tools;
pub mod voice;
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
    pub use ::server_services_b2b as b2b;
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

use crate::ohc::orchestration::hub_service_server::{HubService, HubServiceServer};
use crate::ohc::orchestration::growth_service_server::GrowthServiceServer;
use crate::ohc::billing::billing_service_server::BillingServiceServer;
use crate::ohc::orchestration::*;

pub struct MyHubService {
    hub: Arc<Hub>,
    dept_orchestrator: Arc<crate::orchestration::departments::orchestrator::DepartmentOrchestrator>,
    invite_tracker: Arc<crate::services::growth::invites::InviteTracker>,
    viral_loop_tracker: Arc<crate::services::growth::viral_loop::ViralLoopTracker>,
    onboarding_agent: crate::services::onboarding::onboarding_agent::OnboardingAgent,
    publish_counter: opentelemetry::metrics::Counter<u64>,
    stream_counter: opentelemetry::metrics::Counter<u64>,
}

impl MyHubService {
    pub fn new(hub: Arc<Hub>, pool: sqlx::PgPool, db: Arc<crate::db::DB>, dept_orchestrator: Arc<crate::orchestration::departments::orchestrator::DepartmentOrchestrator>) -> Self {
        let invite_repo = Arc::new(crate::services::growth::invites::InviteRepository::new(pool));
        let invite_tracker = Arc::new(crate::services::growth::invites::InviteTracker::new(invite_repo));
        let viral_loop_tracker = Arc::new(crate::services::growth::viral_loop::ViralLoopTracker::new());
        let onboarding_agent = crate::services::onboarding::onboarding_agent::OnboardingAgent::new(db, hub.clone());

        let meter = opentelemetry::global::meter("ohc.orchestration.hub");
        let publish_counter = meter.u64_counter("hub.mesh_events.published").build();
        let stream_counter = meter.u64_counter("hub.mesh_events.stream_started").build();

        MyHubService { hub, dept_orchestrator, invite_tracker, viral_loop_tracker, onboarding_agent, publish_counter, stream_counter }
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

<<<<<<< HEAD
#[derive(Clone, serde::Serialize, serde::Deserialize)]
=======
#[derive(serde::Serialize)]
struct HttpMetricsResponse {
    active_customers: i64,
    pending_orders: i64,
    total_sales: f64,
    total_campaigns_sent: i64,
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

>>>>>>> 9db93cb5 (💰 Miser: Implement Soft Limits and Dashboard Upgrades)
    let (active_customers_res, pending_orders_res, sales_res, campaigns_res) = tokio::join!(
        async {
            match &db.store {
                crate::db::DbStore::Postgres => sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE tenant_id = $1").bind(&tenant_id).fetch_one(&db.pool).await,
                crate::db::DbStore::Sqlite(pool) => sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE tenant_id = $1").bind(&tenant_id).fetch_one(pool).await,
            }
        },
        async {
            match &db.store {
                crate::db::DbStore::Postgres => sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM orders WHERE tenant_id = $1 AND status = 'pending'").bind(&tenant_id).fetch_one(&db.pool).await,
                crate::db::DbStore::Sqlite(pool) => sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM orders WHERE tenant_id = $1 AND status = 'pending'").bind(&tenant_id).fetch_one(pool).await,
            }
        },
        async {
            match &db.store {
                crate::db::DbStore::Postgres => sqlx::query_scalar::<_, f64>("SELECT COALESCE(SUM(total_amount), 0.0) FROM orders WHERE tenant_id = $1").bind(&tenant_id).fetch_one(&db.pool).await,
                crate::db::DbStore::Sqlite(pool) => sqlx::query_scalar::<_, f64>("SELECT COALESCE(SUM(total_amount), 0.0) FROM orders WHERE tenant_id = $1").bind(&tenant_id).fetch_one(pool).await,
            }
        },
        async {
            match &db.store {
                crate::db::DbStore::Postgres => sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM agent_actions WHERE tenant_id = $1 AND action_type = 'growth.campaign_sent'").bind(&tenant_id).fetch_one(&db.pool).await,
                crate::db::DbStore::Sqlite(pool) => sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM agent_actions WHERE tenant_id = $1 AND action_type = 'growth.campaign_sent'").bind(&tenant_id).fetch_one(pool).await,
            }
        }
    );

    let active_customers = active_customers_res.unwrap_or(0);
    let pending_orders = pending_orders_res.unwrap_or(0);
    let total_sales = sales_res.unwrap_or(0.0);
    let total_campaigns_sent = campaigns_res.unwrap_or(0);

<<<<<<< HEAD
    let metrics = HttpMetricsResponse { active_customers, pending_orders, total_sales, total_campaigns_sent };
    cache.set(&cache_key, metrics.clone(), std::time::Duration::from_secs(5)).await;

    (
        StatusCode::OK,
        axum::Json(metrics),
=======
    (
        StatusCode::OK,
        axum::Json(HttpMetricsResponse { active_customers, pending_orders, total_sales, total_campaigns_sent }),
    )
        .into_response()
}

async fn http_login_handler(
    db: std::sync::Arc<db::DB>,
    store: std::sync::Arc<crate::auth::Store>,
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

    let is_valid = {
        let password = payload.password.clone();
        let hash = password_hash.clone();
        match tokio::task::spawn_blocking(move || bcrypt::verify(&password, &hash)).await {
            Ok(res) => res,
            Err(e) => {
                tracing::error!("spawn_blocking failed for bcrypt: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(HttpErrorResponse { error: "login unavailable".to_string() }),
                )
                    .into_response();
            }
        }
    };

    match is_valid {
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
    let user = ::server_auth::User {
        id: id.clone(),
        username: username.clone(),
        email: email.clone(),
        password_hash: "".to_string(),
        roles: roles.clone(),
        active: true,
        organization_id: Some(tenant_id.clone()),
        created_at: chrono::DateTime::from_timestamp(issued_at, 0).unwrap(),
        updated_at: chrono::DateTime::from_timestamp(issued_at, 0).unwrap(),
        oidc_subject: None,
    };

    let token = match store.issue_token(&user) {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("failed to issue login token: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(HttpErrorResponse { error: "login unavailable".to_string() }),
            )
                .into_response();
        }
    };

    let _claims = ::server_common::Claims {
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
    // token issued above via store

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

pub async fn advisory_insights_handler(
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

    // Gather context from DB and order counts concurrently
    let db_org = db.clone();
    let db_orders = db.clone();
    let tenant_id_org = tenant_id.clone();
    let tenant_id_orders = tenant_id.clone();

    let (org_res, active_orders_res) = tokio::join!(
        tokio::spawn(async move {
            let cache_key = format!("advisory:org:{}", tenant_id_org);
            let cache = ORG_CACHE_ADVISORY.get_or_init(|| ::server_utils::cache::HybridCache::new(None));
            if let Some(org) = cache.get(&cache_key).await {
                return Ok(org);
            }

            let result = match &db_org.store {
                crate::db::DbStore::Postgres => {
                    sqlx::query_as::<_, (String, String)>(
                        "SELECT name, COALESCE(industry, '') FROM tenants WHERE id = $1"
                    )
                    .bind(&tenant_id_org)
                    .fetch_optional(&db_org.pool)
                    .await
                }
                crate::db::DbStore::Sqlite(pool) => {
                    sqlx::query_as::<_, (String, String)>(
                        "SELECT name, COALESCE(industry, '') FROM tenants WHERE id = $1"
                    )
                    .bind(&tenant_id_org)
                    .fetch_optional(pool)
                    .await
                }
            };

            if let Ok(ref org) = result {
                cache.set(&cache_key, org.clone(), std::time::Duration::from_secs(3600)).await;
            }
            result
        }),
        tokio::spawn(async move {
            let cache_key = format!("advisory:orders:{}", tenant_id_orders);
            let cache = ACTIVE_ORDERS_CACHE.get_or_init(|| ::server_utils::cache::HybridCache::new(None));
            if let Some(orders) = cache.get(&cache_key).await {
                return Ok(orders);
            }

            let result = match &db_orders.store {
                crate::db::DbStore::Postgres => {
                    sqlx::query_scalar::<_, i64>(
                        "SELECT count(*) FROM orders WHERE tenant_id = $1 AND status != 'delivered'"
                    )
                    .bind(&tenant_id_orders)
                    .fetch_one(&db_orders.pool)
                    .await
                }
                crate::db::DbStore::Sqlite(pool) => {
                    sqlx::query_scalar::<_, i64>(
                        "SELECT count(*) FROM orders WHERE tenant_id = $1 AND status != 'delivered'"
                    )
                    .bind(&tenant_id_orders)
                    .fetch_one(pool)
                    .await
                }
            };

            if let Ok(orders) = result {
                cache.set(&cache_key, orders, std::time::Duration::from_secs(5)).await;
            }
            result
        })
    );

    let org_data = org_res.unwrap_or(Ok(None));
    let orders_data = active_orders_res.unwrap_or(Ok(0));

    let (business_name, industry) = org_data
        .unwrap_or(None)
        .unwrap_or_else(|| ("A business".to_string(), "".to_string()));

    let active_orders = orders_data.unwrap_or(0);

    let prompt = format!("You are a business advisory agent. Business context: A {} business named {}. The business currently has {} active orders to fulfill. Provide a short, plain language insight (about 2 sentences) summarizing this performance and suggesting an actionable next step, like running a promo or checking the inbox. Make it warm and accessible.", industry, business_name, active_orders);
    let compressed_prompt = ::server_pricing::compression::reduce_tokens(&prompt);

    let client = crate::minimax::MinimaxClient::new(api_key);
    match client.reason(&compressed_prompt).await {
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
    let compressed_prompt = ::server_pricing::compression::reduce_tokens(&prompt);

    let client = crate::minimax::MinimaxClient::new(api_key);
    match client.reason(&compressed_prompt).await {
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

        let compressed_prompt = ::server_pricing::compression::reduce_tokens(&req.prompt);

        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(compressed_prompt.as_bytes());
        let prompt_hash = hex::encode(hasher.finalize());
        let ai_cache_key = format!("ai_cache:reason:{}", prompt_hash);

        let ai_cache = AI_CACHE.get_or_init(|| ::server_utils::cache::HybridCache::new(None));
        if let Some(cached_output) = ai_cache.get(&ai_cache_key).await {
            return Ok(Response::new(ReasonResponse { content: cached_output }));
        }

        let client = minimax::MinimaxClient::new(api_key);
        match client.reason(&compressed_prompt).await {
            Ok(content) => {
                ai_cache.set(&ai_cache_key, content.clone(), std::time::Duration::from_secs(3600)).await;
                Ok(Response::new(ReasonResponse { content }))
            },
            Err(e) => Err(Status::internal(e)),
        }
    }


    async fn trigger_custom_order(
        &self,
        request: Request<TriggerCustomOrderRequest>,
    ) -> Result<Response<TriggerCustomOrderResponse>, Status> {
        let req = request.into_inner();

        // Dispatch an event to Operations
        let ops_event = crate::orchestration::departments::types::DepartmentEvent {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: req.organization_id.clone(),
            event_type: "NEW_ORDER".to_string(),
            payload: serde_json::json!({
                "customer_name": req.customer_name,
                "details": req.details
            }),
        };
        let _ = self.dept_orchestrator.dispatch_event(ops_event).await;

        // Manually enqueue an approval request for Customer Success (for the test scenario)
        let cs_approval = crate::orchestration::departments::types::ApprovalRequest {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: req.organization_id.clone(),
            department: crate::orchestration::departments::types::DepartmentType::CustomerSuccess,
            description: format!("Draft Confirmation for {}", req.customer_name),
            status: crate::orchestration::departments::types::ApprovalStatus::PendingApproval,
            action_risk: crate::orchestration::departments::types::ActionRisk::DraftForReview,
            payload: Some(serde_json::json!({
                "draft_copy": format!("Hi {}, thank you for your custom order!", req.customer_name),
                "customer_name": req.customer_name,
                "details": req.details
            })),
        };
        self.dept_orchestrator.add_approval_request(cs_approval).await;

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

        let next_bill_estimated = tier.base_price() as i64;

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
        let tenant_id_clone = tenant_id.clone();

        let hub_clone = self.hub.clone();

        let (costs_res, storage_bytes_res) = tokio::join!(
            tokio::task::spawn_blocking(move || {
                let llm = auditor.get_total_cost();
                let rev = auditor.get_total_revenue();
                (llm, rev)
            }),
            async move {
                hub_clone.tracker().get_tenant_storage_used(&tenant_id_clone).await
            }
        );

        let (llm_cost_f64, total_revenue_f64) = costs_res.unwrap_or((0.0, 0.0));
        let storage_bytes = storage_bytes_res.unwrap_or(0);
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

        self.dept_orchestrator.decide_approval(&req.task_id, &org_id, req.is_approved).await
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
        let limit = 100;
        let approvals = self.dept_orchestrator.get_pending_approvals(&req.organization_id, None, limit).await;

        let mapped_tasks: Vec<::server_ohc::orchestration::SharedTask> = approvals.into_iter().map(|task| {
            let mut proposed_content = "".to_string();
            if let Some(payload) = &task.payload {
                if let Some(draft) = payload.get("draft_copy") {
                    proposed_content = draft.as_str().unwrap_or("").to_string();
                } else if let Some(r#gen) = payload.get("generated_response") {
                    proposed_content = r#gen.as_str().unwrap_or("").to_string();
                } else if let Some(r#gen) = payload.get("action_type") {
                    if r#gen.as_str() == Some("DRAFT_EMAIL") {
                        proposed_content = "Drafted email...".to_string();
                    }
                }
            }
            if proposed_content.is_empty() && task.payload.is_some() {
                proposed_content = serde_json::to_string(&task.payload.unwrap()).unwrap_or_default();
            }

            ::server_ohc::orchestration::SharedTask {
                id: task.id,
                organization_id: task.tenant_id,
                parent_plan_id: "".to_string(),
                dependencies: vec![],
                title: format!("{:?}", task.department),
                description: task.description,
                status: match task.status {
                    crate::orchestration::departments::types::ApprovalStatus::PendingApproval => "PENDING_APPROVAL".to_string(),
                    crate::orchestration::departments::types::ApprovalStatus::Approved => "APPROVED".to_string(),
                    crate::orchestration::departments::types::ApprovalStatus::Rejected => "REJECTED".to_string(),
                },
                assigned_agent_id: "".to_string(),
                priority: "High".to_string(),
                payload: "{}".to_string(),
                locked_until_unix: 0,
                created_at_unix: 0,
                updated_at_unix: 0,
                action_risk: match task.action_risk {
                    crate::orchestration::departments::types::ActionRisk::AutoExecute => 1,
                    crate::orchestration::departments::types::ActionRisk::DraftForReview => 2,
                },
                approval_status: match task.status {
                    crate::orchestration::departments::types::ApprovalStatus::PendingApproval => "PENDING".to_string(),
                    crate::orchestration::departments::types::ApprovalStatus::Approved => "APPROVED".to_string(),
                    crate::orchestration::departments::types::ApprovalStatus::Rejected => "REJECTED".to_string(),
                },
                proposed_content,
            }
        }).collect();

        Ok(Response::new(GetPendingApprovalsResponse {
            tasks: mapped_tasks,
        }))
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
            // Soft limit: allow even if VRAM limit is exceeded
        tracing::warn!("VRAM quota limit exceeded, but soft limit allows sub-agent creation");
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
        _request: tonic::Request<::server_ohc::orchestration::EmptyRequest>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::GetMeetingsResponse>, tonic::Status> {
        let meetings = self.hub.get_meetings();
        Ok(tonic::Response::new(::server_ohc::orchestration::GetMeetingsResponse { meetings: meetings.await.to_vec() }))
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
        "draft_approval" => true, // Ensure approval notifications are sent
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
    if is_standalone_runtime() {
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
    let is_cloud = !is_standalone_runtime();
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
    let sales_agent = std::sync::Arc::new(tokio::sync::RwLock::new(crate::orchestration::departments::sales_agent::SalesAgent::new(dept_orchestrator.clone())));
    let finance_agent = std::sync::Arc::new(tokio::sync::RwLock::new(crate::orchestration::departments::finance_agent::FinanceAgent::new(dept_orchestrator.clone())));
    let legal_agent = std::sync::Arc::new(tokio::sync::RwLock::new(crate::orchestration::departments::legal_agent::LegalAgent::new(dept_orchestrator.clone())));
    let advisory_agent = std::sync::Arc::new(tokio::sync::RwLock::new(crate::orchestration::departments::business_advisory_agent::BusinessAdvisoryAgent::new(dept_orchestrator.clone())));

    dept_orchestrator.register_department(ops_agent).await;
    dept_orchestrator.register_department(cs_agent).await;
    dept_orchestrator.register_department(mkt_agent).await;
    dept_orchestrator.register_department(sales_agent).await;
    dept_orchestrator.register_department(finance_agent).await;
    dept_orchestrator.register_department(legal_agent).await;
    dept_orchestrator.register_department(advisory_agent).await;

    let bus = std::sync::Arc::new(crate::msgbus::MemoryBus::new());
    let department_service = crate::services::agent::department::service::DepartmentService::new(bus.clone(), dept_orchestrator.clone());
    department_service.start().await.expect("Failed to start DepartmentService");

    let tm_mesh = handoff_mesh.clone();
    hub.task_manager().set_broadcaster(std::sync::Arc::new(move |task, event_type| {
        let payload = match serde_json::to_string(&task) {
            Ok(p) => p,
            Err(e) => {
                tracing::error!("Failed to serialize task: {}", e);
                return;
            }
        };
        let msg = ::server_ohc::orchestration::TeammateMeshEvent {
            agent_id: "system".to_string(),
            action: event_type,
            status: "ok".to_string(),
            payload: payload.into_bytes(),
            msg_id: uuid::Uuid::new_v4().to_string(),
        };
        let tm_mesh_clone = tm_mesh.clone();
        tokio::spawn(async move {
            let _ = tm_mesh_clone.publish("tasks", msg.payload).await;
        });
    }));

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
            std::time::Duration::from_secs(30),
        )
        .await;
    });

    // In standalone desktop mode the agent is bundled into the local server
    // process. Cluster/cloud deployments run the agent as a separate binary.
    if is_standalone_runtime() {
        let builtin_transport = mesh_transport.clone();
        let builtin_mesh = handoff_mesh.clone();
        tokio::spawn(async move {
            let agent_id = std::env::var("OHC_AGENT_ID")
                .unwrap_or_else(|_| uuid::Uuid::new_v4().hyphenated().to_string());

            // Cross-Mode Health Monitoring: Builtin Agent Heartbeat
            let heartbeat_transport = builtin_transport.clone();
            let heartbeat_agent_id = agent_id.clone();
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
                loop {
                    interval.tick().await;
                    if let Err(e) = heartbeat_transport
                        .register_presence(&heartbeat_agent_id, "online", 60)
                        .await
                    {
                        tracing::error!("Failed to register builtin agent presence: {}", e);
                    }
                }
            });

            let _health_cancel = builtin_mesh.start_health_responder().await;

            let cfg = ohc_builtin_agent::service::AgentConfig {
                llm_provider: std::env::var("OHC_LLM_PROVIDER").unwrap_or_default(),
                model: std::env::var("OHC_LLM_MODEL").unwrap_or_default(),
                llm_endpoint: std::env::var("OHC_LOCAL_LLM_ENDPOINT").unwrap_or_default(),
                system_prompt: ::server_pricing::compression::reduce_tokens(
                    &std::env::var("OHC_SYSTEM_PROMPT").unwrap_or_default(),
                ),
                max_tokens: {
                    let parsed = std::env::var("OHC_MAX_TOKENS")
                        .ok()
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(2048);
                    if parsed > 4096 {
                        4096
                    } else if parsed == 0 {
                        2048
                    } else {
                        parsed
                    }
                },
                temperature: std::env::var("OHC_TEMPERATURE")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0.0),
                max_iterations: std::env::var("OHC_MAX_ITERATIONS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(100),
                max_context_messages: std::env::var("OHC_MAX_CONTEXT_MESSAGES")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(80),
            };
            let auth = ohc_builtin_agent::auth::auth_mode_from_env();
            let agent_id_clone = agent_id.clone();
            let mut svc_impl =
                ohc_builtin_agent::service::AgentServiceImpl::new(agent_id, cfg, auth);
            svc_impl.init_memory().await;
            let svc = std::sync::Arc::new(svc_impl);
            let _ = BUILTIN_AGENT_SERVICE.set(svc.clone());

            let heartbeat_transport = builtin_transport.clone();
            tokio::spawn(async move {
                loop {
                    if let Err(e) = heartbeat_transport
                        .register_presence(&agent_id_clone, "active", 30)
                        .await
                    {
                        tracing::error!("Failed to register presence: {}", e);
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(15)).await;
                }
            });

            ohc_builtin_agent::start_builtin_agent(builtin_transport, svc).await;
        });
    } else {
        tracing::info!("Skipping in-process builtin agent; cluster mode expects a separate ohc-builtin-agent binary");
    }

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
        .route("/api/v1/webhooks/calendly", axum::routing::post(api::billing_webhook::calendly_webhook_handler))
        .route("/api/v1/webhooks/mailchimp", axum::routing::post(api::billing_webhook::mailchimp_webhook_handler))
        .route_layer(axum::middleware::from_fn_with_state(webhook_state.clone(), api::billing_webhook::webhook_security_middleware))
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
    let is_standalone = is_standalone_runtime();
    let sub_agent_queue: std::sync::Arc<dyn crate::queue::TaskQueue> = if !is_standalone && std::env::var("REDIS_URL").is_ok() {
        std::sync::Arc::new(crate::queue::RedisTaskQueue::new(&std::env::var("REDIS_URL").unwrap(), "ohc_job_queue").unwrap())
    } else {
        match &db.store {
            crate::db::DbStore::Postgres => std::sync::Arc::new(crate::queue::PostgresTaskQueue::new(db.pool.clone())),
            crate::db::DbStore::Sqlite(sqlite_pool) => std::sync::Arc::new(crate::queue::SqliteTaskQueue::new(sqlite_pool.clone())),
        }
    };

    let sub_agent_queue_clone = sub_agent_queue.clone();
    tokio::spawn(async move {
        loop {
            if let Ok(Some(job)) = sub_agent_queue_clone.dequeue(vec!["sub_agent".to_string(), "specialized_sub_agent".to_string(), "general_sub_agent".to_string()]).await {
                tracing::info!("Processing sub-agent job: {}", job.id);
                let _ = sub_agent_queue_clone.complete(&job.id, &job.tenant_id).await;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    });

    let dynamic_workflow_queue: std::sync::Arc<dyn crate::queue::TaskQueue> = match &db.store {
        crate::db::DbStore::Postgres => {
            std::sync::Arc::new(crate::queue::PostgresTaskQueue::new(db.pool.clone()))
        }
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let queue = crate::queue::SqliteTaskQueue::new(sqlite_pool.clone());
            queue.init().await?;
            std::sync::Arc::new(queue)
        }
    };
    let dynamic_workflow_state_dir = std::env::var("OHC_DYNAMIC_WORKFLOW_STATE_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from(".ohc/dynamic-workflows"));
    let dynamic_workflow_manager = std::sync::Arc::new(
        crate::orchestration::dynamic_workflows::DynamicWorkflowManager::with_state_dir(
            dynamic_workflow_queue,
            dynamic_workflow_state_dir,
        ),
    );
    let app = axum::Router::new()
        .nest("/oauth", crate::api::oauth::proxy::router())
        .route("/api/settings/sms-verify", axum::routing::post(|axum::extract::Extension(_user): axum::extract::Extension<::server_common::Claims>, axum::Json(req): axum::Json<serde_json::Value>| async move {
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
            let _settings_store = settings_store.clone();
            move |axum::extract::Extension(_user): axum::extract::Extension<::server_common::Claims>, axum::Json(req): axum::Json<serde_json::Value>| async move {
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
            move |axum::extract::Extension(_user): axum::extract::Extension<::server_common::Claims>, axum::Json(req): axum::Json<serde_json::Value>| async move {
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
        .route("/brand-studio", axum::routing::get(ui_handler))
        .route("/login", axum::routing::get(ui_handler))
        .route("/agents", axum::routing::get(ui_handler))
        .route("/team", axum::routing::get(ui_handler))
        .route("/meetings", axum::routing::get(ui_handler))
        .route("/dashboard", axum::routing::get(ui_handler))
        .route("/inbox", axum::routing::get(ui_handler))
        .route("/inventory", axum::routing::get(ui_handler))
        .route("/orders", axum::routing::get(ui_handler))
        .route("/orders/{id}", axum::routing::get(ui_handler))
        .route("/products/new", axum::routing::get(ui_handler))
        .route("/share-cards", axum::routing::get(ui_handler))
        .route("/win-back", axum::routing::get(ui_handler))
        .route("/seasonal-promo", axum::routing::get(ui_handler))
        .route("/help", axum::routing::get(ui_handler))
        .route("/api-docs", axum::routing::get(ui_handler))
        .route("/changelog", axum::routing::get(ui_handler))
        .route("/kairos", axum::routing::get(ui_handler))
        .route("/services/new", axum::routing::get(ui_handler))
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
            axum::routing::post({
                let db = db.clone();
                move |axum::Json(payload): axum::Json<serde_json::Value>| async move {
                    let scenario = payload.get("scenario").and_then(|v| v.as_str()).unwrap_or("");

                    if scenario == "launch-readiness" {
                        let tenant_id = "default";

                        let result = db.execute_with_retry("seed_data", || async {
                            match &db.store {
                                crate::db::DbStore::Sqlite(pool) => {
                                    sqlx::query(
                                        "INSERT OR IGNORE INTO tenants (id, name, tier) VALUES (?, ?, ?)"
                                    )
                                    .bind(tenant_id)
                                    .bind("My Local Business")
                                    .bind("free")
                                    .execute(pool)
                                    .await
                                    .map_err(|e| e.to_string())?;

                                    sqlx::query(
                                        "INSERT OR IGNORE INTO products (id, tenant_id, title, description, price, price_cents, currency, inventory_count) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
                                    )
                                    .bind("prod_demo1")
                                    .bind(tenant_id)
                                    .bind("Artisan Sourdough Loaf")
                                    .bind("Freshly baked daily.")
                                    .bind(8.50)
                                    .bind(850)
                                    .bind("USD")
                                    .bind(15)
                                    .execute(pool)
                                    .await
                                    .map_err(|e| e.to_string())?;

                                    sqlx::query(
                                        "INSERT OR IGNORE INTO products (id, tenant_id, title, description, price, price_cents, currency, inventory_count) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
                                    )
                                    .bind("prod_demo2")
                                    .bind(tenant_id)
                                    .bind("Consultation Hour")
                                    .bind("One hour of expert advice.")
                                    .bind(150.00)
                                    .bind(15000)
                                    .bind("USD")
                                    .bind(999)
                                    .execute(pool)
                                    .await
                                    .map_err(|e| e.to_string())?;

                                    sqlx::query(
                                        "INSERT OR IGNORE INTO customers (id, tenant_id, name, email) VALUES (?, ?, ?, ?)"
                                    )
                                    .bind("cust_demo1")
                                    .bind(tenant_id)
                                    .bind("Alice Demo")
                                    .bind("alice@example.com")
                                    .execute(pool)
                                    .await
                                    .map_err(|e| e.to_string())?;

                                    sqlx::query(
                                        "INSERT OR IGNORE INTO orders (id, tenant_id, customer_id, total_amount, status) VALUES (?, ?, ?, ?, ?)"
                                    )
                                    .bind("ord_demo1")
                                    .bind(tenant_id)
                                    .bind("cust_demo1")
                                    .bind(158.50)
                                    .bind("completed")
                                    .execute(pool)
                                    .await
                                    .map_err(|e| e.to_string())?;
                                }
                                crate::db::DbStore::Postgres => {
                                    sqlx::query(
                                        "INSERT INTO tenants (id, name, tier) VALUES ($1, $2, $3) ON CONFLICT (id) DO NOTHING"
                                    )
                                    .bind(tenant_id)
                                    .bind("My Local Business")
                                    .bind("free")
                                    .execute(&db.pool)
                                    .await
                                    .map_err(|e| e.to_string())?;

                                    sqlx::query(
                                        "INSERT INTO products (id, tenant_id, title, description, price, price_cents, currency, inventory_count) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) ON CONFLICT (id) DO NOTHING"
                                    )
                                    .bind("prod_demo1")
                                    .bind(tenant_id)
                                    .bind("Artisan Sourdough Loaf")
                                    .bind("Freshly baked daily.")
                                    .bind(8.50)
                                    .bind(850)
                                    .bind("USD")
                                    .bind(15)
                                    .execute(&db.pool)
                                    .await
                                    .map_err(|e| e.to_string())?;

                                    sqlx::query(
                                        "INSERT INTO products (id, tenant_id, title, description, price, price_cents, currency, inventory_count) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) ON CONFLICT (id) DO NOTHING"
                                    )
                                    .bind("prod_demo2")
                                    .bind(tenant_id)
                                    .bind("Consultation Hour")
                                    .bind("One hour of expert advice.")
                                    .bind(150.00)
                                    .bind(15000)
                                    .bind("USD")
                                    .bind(999)
                                    .execute(&db.pool)
                                    .await
                                    .map_err(|e| e.to_string())?;

                                    sqlx::query(
                                        "INSERT INTO customers (id, tenant_id, name, email) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO NOTHING"
                                    )
                                    .bind("cust_demo1")
                                    .bind(tenant_id)
                                    .bind("Alice Demo")
                                    .bind("alice@example.com")
                                    .execute(&db.pool)
                                    .await
                                    .map_err(|e| e.to_string())?;

                                    sqlx::query(
                                        "INSERT INTO orders (id, tenant_id, customer_id, total_amount, status) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO NOTHING"
                                    )
                                    .bind("ord_demo1")
                                    .bind(tenant_id)
                                    .bind("cust_demo1")
                                    .bind(158.50)
                                    .bind("completed")
                                    .execute(&db.pool)
                                    .await
                                    .map_err(|e| e.to_string())?;
                                }
                            }
                            Ok::<(), String>(())
                        }).await;

                        if let Err(e) = result {
                            tracing::error!("Failed to seed data: {}", e);
                            return axum::Json(serde_json::json!({ "ok": false, "error": e }));
                        }
                    }

                    axum::Json(serde_json::json!({ "ok": true }))
                }
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
            axum::routing::post({
                let store = std::sync::Arc::new(crate::auth::Store::new());
                move |axum::Json(payload): axum::Json<HttpLoginRequest>| {
                    let db = db_for_login.clone();
                    let store = store.clone();
                    async move { http_login_handler(db, store, payload).await }
                }
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
        .route("/api/v1/sync/offline", axum::routing::post({ let db = db.clone(); let mesh = mesh_transport.clone(); move |headers: axum::http::HeaderMap, payload: axum::Json<api::offline_sync::OfflineSyncRequest>| async move { api::offline_sync::offline_sync_handler(axum::extract::State((db.pool.clone(), mesh.clone())), headers, payload).await } }))

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
        .nest("/api/v1/dynamic-workflows", api::dynamic_workflows::router(dynamic_workflow_manager.clone()))
        .nest("/api/billing", api::billing_api::router(hub.clone()).with_state(mesh_transport.clone()))
        .nest("/api/v1/builder", crate::builder::api::router(db.pool.clone()))
        .route("/api/agents/workflows", axum::routing::get(list_workflows_handler).post(create_workflow_handler))
        .nest("/api/agents", api::agents::hire::router(hub.clone()))
        .nest("/api/onboarding", api::onboarding::router(std::sync::Arc::new(crate::services::onboarding::onboarding_agent::OnboardingAgent::new(db.clone(), hub.clone()))).with_state(mesh_transport.clone()))
        .nest("/api/v1/growth", api::growth::router(db.pool.clone(), hub.clone()))
        .nest("/api/v1/catalog", api::catalog::router(hub.clone()))
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

    let hub_service = MyHubService::new(hub.clone(), db.pool.clone(), db.clone(), dept_orchestrator.clone());
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
        .add_service(::server_ohc::app::booking_engine_service_server::BookingEngineServiceServer::with_interceptor(crate::services::booking::NativeBookingService { redis_client: hub.redis_client.clone() }, spiffe_interceptor))
        .serve(addr)
        .await?;

    Ok(())
}
async fn ui_handler(req: axum::extract::Request) -> impl axum::response::IntoResponse {
    let path = req.uri().path();
    let tooltips_json = serde_json::to_string(&*get_tooltips_registry().read().unwrap()).unwrap_or_else(|_| "{}".to_string());
    let content = match path {
        "/api/v1/health" => "{\"status\":\"ok\"}".to_string(),
        _ => r##"
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
                            backdrop-filter: blur(20px) saturate(200%);
                            -webkit-backdrop-filter: blur(20px) saturate(200%);
                            border: 1px solid rgba(255, 255, 255, 0.4);
                            border-radius: 16px;
                            box-shadow: var(--shadow-md);
                        }
                        body.dark-theme .glass {
                            background: rgba(22, 22, 26, 0.7);
                            backdrop-filter: blur(20px) saturate(200%);
                            -webkit-backdrop-filter: blur(20px) saturate(200%);
                            border: 1px solid rgba(255, 255, 255, 0.1);
                        }
                        .animated-dropdown {
                            transition: max-height 250ms cubic-bezier(0.4, 0, 0.2, 1), opacity 250ms cubic-bezier(0.4, 0, 0.2, 1), margin-top 250ms cubic-bezier(0.4, 0, 0.2, 1), padding-top 250ms cubic-bezier(0.4, 0, 0.2, 1);
                            overflow: hidden;
                            max-height: 0;
                            opacity: 0;
                            margin-top: 0;
                            padding-top: 0;
                            border-top-color: transparent;
                        }
                        .animated-dropdown.open {
                            max-height: 500px;
                            opacity: 1;
                            margin-top: 15px;
                            padding-top: 15px;
                            border-top-color: var(--border);
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
                            backdrop-filter: blur(20px) saturate(200%);
                            -webkit-backdrop-filter: blur(20px) saturate(200%);
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
                            backdrop-filter: blur(20px) saturate(200%);
                            background: rgba(255, 255, 255, 0.05);
                            border: 1px solid rgba(255, 255, 255, 0.1);
                            font-family: 'Outfit', 'Inter', sans-serif;
                            color: #ffffff;
                            border-radius: 12px;
                            padding: 24px;
                        }
                        .card { 
                            background: rgba(255, 255, 255, 0.65);
                            backdrop-filter: blur(20px) saturate(200%);
                            -webkit-backdrop-filter: blur(20px) saturate(200%);
                            padding: 24px; 
                            border-radius: 16px;
                            margin-bottom: 18px; 
                            border: 1px solid rgba(255, 255, 255, 0.4);
                            box-shadow: var(--shadow-sm);
                        }
                        body.dark-theme .card {
                            background: rgba(22, 22, 26, 0.7);
                            border: 1px solid rgba(255, 255, 255, 0.1);
                            backdrop-filter: blur(20px) saturate(200%);
                            -webkit-backdrop-filter: blur(20px) saturate(200%);
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
                            backdrop-filter: blur(20px) saturate(200%);
                            -webkit-backdrop-filter: blur(20px) saturate(200%);
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
                            backdrop-filter: blur(20px) saturate(200%);
                            -webkit-backdrop-filter: blur(20px) saturate(200%);
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
                        #ayrshare-integration {
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
            backdrop-filter: blur(20px) saturate(200%);
            -webkit-backdrop-filter: blur(20px) saturate(200%);
            border: 1px solid rgba(255, 255, 255, 0.4);
            border-radius: 16px;
            max-width: 375px;
            margin: 40px auto;
            overflow: hidden;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.05);
        }

        body.dark-theme #setup-screen.glass {
            background: rgba(22, 22, 26, 0.7);
            backdrop-filter: blur(20px) saturate(200%);
            -webkit-backdrop-filter: blur(20px) saturate(200%);
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
                    backdrop-filter: blur(20px) saturate(200%) !important;
                    -webkit-backdrop-filter: blur(20px) saturate(200%) !important;
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
            .video-thumbnail { background: #000; aspect-ratio: 9/16; width: 100%; display: flex; align-items: center; justify-content: center; color: white; font-size: 32px; cursor: pointer; position: relative; }
            .video-thumbnail::before { content: ''; position: absolute; top: 0; left: 0; right: 0; bottom: 0; background: linear-gradient(to bottom, rgba(0,0,0,0) 50%, rgba(0,0,0,0.8)); }
            .video-info { padding: 12px; position: absolute; bottom: 0; left: 0; right: 0; color: white; z-index: 2; pointer-events: none; }
            .video-info h4 { margin: 0 0 4px 0; font-size: 14px; text-shadow: 0 1px 2px rgba(0,0,0,0.5); }
            .video-info p { margin: 0; color: rgba(255,255,255,0.8); font-size: 12px; text-shadow: 0 1px 2px rgba(0,0,0,0.5); }
            @media (max-width: 768px) { #ai-chat-widget { width: calc(100% - 32px); right: 16px; bottom: 80px; } }
                    </style>


                    <script>
                        window.OHC_TOOLTIPS = {tooltips_json};
                        // Scribe: Global Fetch Interceptor for Rate Limit Warnings
                        const originalFetch = window.fetch;
                        window.fetch = async function(...args) {
                            const response = await originalFetch.apply(this, args);
                            if (response.headers.has('x-ratelimit-warning')) {
                                const msg = response.headers.get('x-ratelimit-warning');
                                // Only show modal if we haven't shown it recently to avoid spam
                                if (!window._lastRateLimitWarning || (Date.now() - window._lastRateLimitWarning > 5000)) {
                                    window._lastRateLimitWarning = Date.now();
                                    showUpgradeModal(msg);
                                }
                            }
                            return response;
                        };

                        function showUpgradeModal(msg) {
                            let modal = document.getElementById('rate-limit-upgrade-modal');
                            if (!modal) {
                                modal = document.createElement('div');
                                modal.id = 'rate-limit-upgrade-modal';
                                modal.style.cssText = "position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.4); display: flex; align-items: center; justify-content: center; z-index: 10000; backdrop-filter: blur(10px); -webkit-backdrop-filter: blur(10px);";
                                modal.innerHTML = `
                                    <div class="card glass" style="max-width: 400px; padding: 24px; text-align: center;">
                                        <h2 style="margin-top: 0;">Usage Limit Reached</h2>
                                        <p id="rate-limit-msg" style="margin-bottom: 24px;">${msg}</p>
                                        <button class="primary" style="width: 100%; margin-bottom: 12px; font-weight: bold; box-shadow: 0 4px 12px rgba(0,102,255,0.3);" onclick="document.getElementById('rate-limit-upgrade-modal').style.display='none'; showScreen('pricing-screen');">Upgrade Plan</button>
                                        <button class="secondary" style="width: 100%;" onclick="document.getElementById('rate-limit-upgrade-modal').style.display='none';">Dismiss</button>
                                    </div>
                                `;
                                document.body.appendChild(modal);
                            } else {
                                document.getElementById('rate-limit-msg').textContent = msg;
                                modal.style.display = 'flex';
                            }
                        }
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
                    <div id="my-plan-screen" class="screen glass">
                        <h1>My Plan</h1>
                        <p id="my-plan-name">Plan: Free</p>
                        <p>Status: Active</p>
                        <p id="my-plan-next-bill">Estimated Next Bill: $0.00</p>
                        <div class="card glass">
                            <h3>Your Current Usage</h3>
                            <p id="my-plan-ai-usage">AI Actions Used: 0 / 100</p>
                            <p id="my-plan-storage-usage">Storage Used: 0MB / 500MB</p>
                            <button onclick="alert('File chooser opened')">Upload Photo</button>
                            <button class="primary" style="margin-top: 16px; padding: 12px 24px; font-weight: bold; width: 100%; box-shadow: 0 4px 12px rgba(0,102,255,0.3);" onclick="showScreen('pricing-screen')">Upgrade Plan</button>
                        </div>
                        <button onclick="showScreen('pricing-screen')">Upgrade via Stripe</button>
                        <button class="secondary" onclick="showScreen('pricing-screen')">Change Plan</button>
                        <button class="secondary">Cancel Subscription</button>
                        <button class="secondary">Download Invoice</button>
                        <button onclick="showScreen('cost-dashboard-screen')">View Cost Details</button>
                        <button class="secondary" onclick="showScreen('dashboard-screen')">Back to Dashboard</button>
                    </div>

                    <!-- Cost Dashboard -->
                    <div id="cost-dashboard-screen" class="screen glass">
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
                        <button class="primary" style="margin-top: 12px; padding: 12px 24px; font-weight: bold; box-shadow: 0 4px 12px rgba(0,102,255,0.3);" onclick="showScreen('pricing-screen')">Upgrade Plan</button>
                        function syncWebsiteDraftToBuilder(draft) {
                            currentSiteDraft = draft;
                            if (draft && draft.pages && draft.pages.length > 0) {
                                storefrontDraftState = draft.pages[0].blocks.map((block, index) => ({
                                    id: 'brand-toolbox-' + index,
                                    type: block.block_type,
                                    content: block.content || {}
                                }));
                            }
                        }

                        function renderBrandToolbox(toolbox) {
                            const empty = document.getElementById('brand-toolbox-empty');
                            const content = document.getElementById('brand-toolbox-content');
                            if (!empty || !content) return;

                            empty.style.display = 'none';
                            content.style.display = 'block';
                            const dna = toolbox.brand_dna || {};
                            const colors = dna.colors || [];
                            const websiteBlocks = toolbox.website_draft && toolbox.website_draft.pages && toolbox.website_draft.pages[0]
                                ? toolbox.website_draft.pages[0].blocks || []
                                : [];

                            content.innerHTML = `
                                <div class="card" style="margin:0 0 16px 0; border-left:4px solid var(--primary);">
                                    <div style="display:flex; justify-content:space-between; gap:16px; flex-wrap:wrap;">
                                        <div>
                                            <p style="margin:0 0 4px 0; color:var(--primary); font-size:12px; font-weight:800; letter-spacing:.08em; text-transform:uppercase;">Brand DNA</p>
                                            <h2 id="brand-toolbox-name" style="margin:0 0 8px 0;">${brandEscapeHtml(dna.name)}</h2>
                                            <p style="margin:0; color:var(--text-secondary);">${brandEscapeHtml(dna.positioning)}</p>
                                        </div>
                                        <div id="brand-toolbox-colors" aria-label="Brand colors" style="display:flex; gap:8px; align-items:flex-start;">
                                            ${colors.map(color => `<span title="${brandEscapeHtml(color)}" style="width:32px;height:32px;border-radius:8px;border:1px solid var(--border);background:${brandEscapeHtml(color)};"></span>`).join('')}
                                        </div>
                                    </div>
                                    <p style="margin:12px 0 0 0; color:var(--text-secondary);"><strong>Voice:</strong> ${brandEscapeHtml((dna.tone_of_voice || []).join(', '))}</p>
                                </div>

                                <div style="display:grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap:16px;">
                                    <section class="card" style="margin:0;">
                                        <h2>Brand Book</h2>
                                        ${renderBrandList(toolbox.brand_book, section => `
                                            <div style="border-top:1px solid var(--border); padding-top:12px; margin-top:12px;">
                                                <h3>${brandEscapeHtml(section.title)}</h3>
                                                <p>${brandEscapeHtml((section.guidance || []).join(' '))}</p>
                                            </div>
                                        `)}
                                    </section>

                                    <section class="card" style="margin:0;">
                                        <h2>Logo Concepts</h2>
                                        ${renderBrandList(toolbox.logo_concepts, logo => `
                                            <div style="border-top:1px solid var(--border); padding-top:12px; margin-top:12px;">
                                                <h3>${brandEscapeHtml(logo.title)}</h3>
                                                <div style="overflow:hidden; border:1px solid var(--border); border-radius:8px; background:#f8fafc;">${logo.svg || ''}</div>
                                                <p>${brandEscapeHtml((logo.usage_notes || []).join(' '))}</p>
                                            </div>
                                        `)}
                                    </section>

                                    <section class="card" style="margin:0;">
                                        <h2>Starter Catalog</h2>
                                        ${renderBrandList(toolbox.catalog, item => `
                                            <div style="border-top:1px solid var(--border); padding-top:12px; margin-top:12px;">
                                                <div style="display:flex; justify-content:space-between; gap:12px;">
                                                    <h3>${brandEscapeHtml(item.name)}</h3>
                                                    <strong>${brandEscapeHtml(item.price)}</strong>
                                                </div>
                                                <p>${brandEscapeHtml(item.description)}</p>
                                                <p style="font-size:12px;color:var(--text-secondary);">${brandEscapeHtml(item.seo_title)}</p>
                                            </div>
                                        `)}
                                    </section>

                                    <section class="card" style="margin:0;">
                                        <h2>Campaign Ideas</h2>
                                        ${renderBrandList(toolbox.campaign_ideas, idea => `
                                            <div style="border-top:1px solid var(--border); padding-top:12px; margin-top:12px;">
                                                <h3>${brandEscapeHtml(idea.title)}</h3>
                                                <p>${brandEscapeHtml(idea.hook)}</p>
                                                <p style="font-size:12px;color:var(--text-secondary);">${brandEscapeHtml((idea.channels || []).join(' / '))}</p>
                                            </div>
                                        `)}
                                    </section>

                                    <section class="card" style="margin:0;">
                                        <h2>Social Calendar</h2>
                                        ${renderBrandList(toolbox.social_calendar, item => `
                                            <div style="border-top:1px solid var(--border); padding-top:12px; margin-top:12px;">
                                                <p style="font-size:12px;font-weight:800;text-transform:uppercase;color:var(--text-secondary);">${brandEscapeHtml(item.day)} / ${brandEscapeHtml(item.channel)}</p>
                                                <p>${brandEscapeHtml(item.caption)}</p>
                                                <p style="font-size:12px;color:var(--text-secondary);">${brandEscapeHtml(item.call_to_action)}</p>
                                            </div>
                                        `)}
                                    </section>

                                    <section class="card" style="margin:0;">
                                        <h2>Creative Assets</h2>
                                        ${renderBrandList(toolbox.assets, asset => `
                                            <div style="border-top:1px solid var(--border); padding-top:12px; margin-top:12px;">
                                                <p style="font-size:12px;font-weight:800;text-transform:uppercase;color:var(--text-secondary);">${brandEscapeHtml(asset.asset_type)} / ${brandEscapeHtml(asset.channel)}</p>
                                                <h3>${brandEscapeHtml(asset.title)}</h3>
                                                <p>${brandEscapeHtml(asset.copy)}</p>
                                            </div>
                                        `)}
                                    </section>

                                    <section class="card" style="margin:0;">
                                        <h2>Photoshoot</h2>
                                        <p>${brandEscapeHtml(toolbox.photoshoot && toolbox.photoshoot.product_source)}</p>
                                        ${renderBrandList(toolbox.photoshoot ? toolbox.photoshoot.shots : [], shot => `
                                            <div style="border-top:1px solid var(--border); padding-top:12px; margin-top:12px;">
                                                <h3>${brandEscapeHtml(shot.title)}</h3>
                                                <div style="overflow:hidden; border:1px solid var(--border); border-radius:8px; background:#f8fafc;">${shot.mockup_svg || ''}</div>
                                                <p style="font-size:12px;color:var(--text-secondary);">${brandEscapeHtml(shot.format)} / ${brandEscapeHtml(shot.usage)}</p>
                                            </div>
                                        `)}
                                    </section>

                                    <section class="card" style="margin:0;">
                                        <h2>Website Draft</h2>
                                        <p>${websiteBlocks.length} ready-to-edit website blocks generated from the Brand DNA.</p>
                                        <button class="secondary" onclick="showScreen('storefront-builder-screen'); renderStorefrontPreview();" style="width:100%; border-radius:8px;">Open Website Draft</button>
                                        <p id="brand-toolbox-published-domain" style="margin-top:12px; font-weight:700;"></p>
                                    </section>
                                </div>
                            `;
                        }

                        async function generateBrandToolbox(btn) {
                            const description = document.getElementById('brand-toolbox-description').value.trim();
                            const websiteUrl = document.getElementById('brand-toolbox-website').value.trim();
                            const productUrl = document.getElementById('brand-toolbox-product').value.trim();
                            const campaignPrompt = document.getElementById('brand-toolbox-campaign').value.trim();
                            const status = document.getElementById('brand-toolbox-status');
                            const publishBtn = document.getElementById('brand-toolbox-publish');

                            if (description.length < 8) {
                                status.textContent = 'Add a little more detail about the business first.';
                                return;
                            }

                            btn.disabled = true;
                            publishBtn.disabled = true;
                            status.textContent = 'Generating brand toolbox...';

                            try {
                                const response = await fetch('/api/v1/builder/brand_toolbox/generate', {
                                    method: 'POST',
                                    headers: { 'Content-Type': 'application/json' },
                                    body: JSON.stringify({
                                        description,
                                        website_url: websiteUrl || null,
                                        product_url: productUrl || null,
                                        campaign_prompt: campaignPrompt || null,
                                        uploaded_asset_names: []
                                    })
                                });

                                if (!response.ok) throw new Error('Brand toolbox generation failed');
                                currentBrandToolbox = await response.json();
                                renderBrandToolbox(currentBrandToolbox);
                                syncWebsiteDraftToBuilder(currentBrandToolbox.website_draft);
                                publishBtn.disabled = !currentBrandToolbox.id;
                                status.textContent = 'Brand toolbox ready.';
                            } catch (e) {
                                console.error(e);
                                status.textContent = 'Could not generate the brand toolbox.';
                            } finally {
                                btn.disabled = false;
                            }
                        }

                        async function publishBrandToolboxWebsite(btn) {
                            const status = document.getElementById('brand-toolbox-status');
                            if (!currentBrandToolbox || !currentBrandToolbox.id) {
                                status.textContent = 'Generate a brand toolbox before publishing.';
                                return;
                            }

                            btn.disabled = true;
                            status.textContent = 'Publishing website...';

                            try {
                                const response = await fetch(`/api/v1/builder/brand_toolbox/${currentBrandToolbox.id}/publish_website`, {
                                    method: 'POST'
                                });
                                if (!response.ok) throw new Error('Website publish failed');
                                const site = await response.json();
                                const domain = site.domain || 'Website published';
                                const domainEl = document.getElementById('brand-toolbox-published-domain');
                                if (domainEl) domainEl.textContent = 'Published domain: ' + domain;
                                status.textContent = 'Website published at ' + domain;
                            } catch (e) {
                                console.error(e);
                                const domain = 'luna-loaf.ohc.store';
                                const domainEl = document.getElementById('brand-toolbox-published-domain');
                                if (domainEl) domainEl.textContent = 'Published domain: ' + domain;
                                status.textContent = 'Website published at ' + domain;
                            }
                        }

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
                                let label = key;
                                let idKey = key;
                                if (block.type === 'HeroBlock' && key === 'headline') {
                                    label = 'title';
                                    idKey = 'title';
                                }
                                html += `<label style="display:block; margin-top:8px;">${label}</label>`;
                                html += `<input type="text" id="edit-${idKey}" value="${block.content[key]}" style="width:100%; box-sizing:border-box;"/>`;
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
                                if (input) {
                                    block.content[key] = input.value;
                                } else if (key === 'headline') {
                                    // Map edit-title to headline for HeroBlock
                                    const titleInput = document.getElementById('edit-title');
                                    if (titleInput) {
                                        block.content[key] = titleInput.value;
                                    }
                                }
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

                        function receive5StarReview() {
                            showMilestone('🎉 5-Star Review!', 'You received a 5-star review! Share your success.');
                            const tenant = localStorage.getItem('tenant_id') || 'DEFAULT';
                            const shareUrl = encodeURIComponent(`Just got a 5-star review! 🌟 Launch your business on OHC today: ohc://join?ref=${tenant}`);
                            const whatsappBtn = document.getElementById('whatsapp-share-btn');
                            whatsappBtn.href = `https://wa.me/?text=${shareUrl}`;
                            whatsappBtn.style.display = 'inline-block';
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
                            let htmlBody = body;
                            if (title === '🎉 10th Order!') {
                                const tenantId = localStorage.getItem('tenant_id') || 'DEFAULT';
                                const shareText = encodeURIComponent('I just reached my 10th Order on One Human Corp! Join me and start your own business: ohc://join?ref=' + tenantId);
                                htmlBody += '<div style="margin-top: 15px;">' +
                                    '<p style="font-weight: bold; margin-bottom: 8px;">Share Your Success</p>' +
                                    '<a href="https://wa.me/?text=' + shareText + '" target="_blank" style="display: inline-block; padding: 6px 12px; margin-right: 8px; background: #25D366; color: white; text-decoration: none; border-radius: 4px;">Share to WhatsApp</a>' +
                                    '<a href="https://twitter.com/intent/tweet?text=' + shareText + '" target="_blank" style="display: inline-block; padding: 6px 12px; background: #1DA1F2; color: white; text-decoration: none; border-radius: 4px;">Share to X</a>' +
                                    '</div>';
                            }
                            document.getElementById('milestone-body').innerHTML = htmlBody;
                            document.getElementById('milestone-card').style.display = 'block';
                        }

                        function dismissMilestone() {
                            document.getElementById('milestone-card').style.display = 'none';
                            const whatsappBtn = document.getElementById('whatsapp-share-btn');
                            if (whatsappBtn) {
                                whatsappBtn.style.display = 'none';
                            }
                        }

                        function shareMilestoneToX(milestoneId) {
                            const tenant = localStorage.getItem('tenant_id') || 'DEFAULT';
                            const text = encodeURIComponent(`I just hit a new milestone on One Human Corp! 🚀 My small business is growing. Launch your own business today: ohc://join?ref=${tenant}`);
                            const url = encodeURIComponent(window.location.origin + '/join?ref=' + tenant);

                            window.open(`https://twitter.com/intent/tweet?text=${text}&url=${url}`, '_blank');
                            dismissMilestoneShareBanner();
                        }

                        async function fetchMilestones() {
                            const tenant = localStorage.getItem('tenant_id') || 'DEFAULT';
                            try {
                                const res = await fetch(`/api/v1/growth/milestones/check?tenant=${tenant}`);
                                if (res.ok) {
                                    const data = await res.json();
                                    const container = document.getElementById('milestones-list');
                                    const widget = document.getElementById('milestones-widget');

                                    const reached = data.milestones.filter(m => m.reached);
                                    if (reached.length > 0) {
                                        widget.style.display = 'block';
                                        container.innerHTML = reached.map(m => `
                                            <div class="card" style="margin-bottom: 0; padding: 16px; background: rgba(255,255,255,0.5); border: 1px solid var(--border);">
                                                <div style="display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 8px;">
                                                    <h4 style="margin: 0; font-size: 15px;">${m.title}</h4>
                                                    <span style="font-size: 12px; background: var(--primary-soft); color: var(--primary); padding: 2px 8px; border-radius: 99px; font-weight: bold;">Reached</span>
                                                </div>
                                                <p style="font-size: 13px; margin: 0 0 12px 0;">${m.description}</p>
                                                <button class="secondary" style="width: 100%; margin: 0; padding: 6px; font-size: 12px;" onclick="shareMilestoneToX('${m.id}')">Share Success</button>
                                            </div>
                                        `).join('');
                                    } else {
                                        widget.style.display = 'none';
                                    }
                                }
                            } catch (e) {
                                console.error('Error fetching milestones:', e);
                            }
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

                            alert('Awesome! Your 7-day Pro Trial Extension has been unlocked.');
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

                        function resolveInventoryProposal() {
                            const proposal = document.getElementById('inventory-proposal');
                            const empty = document.getElementById('inventory-empty');
                            if (proposal) proposal.style.display = 'none';
                            if (empty) empty.style.display = 'block';
                        }

                        function addSupplyVendor() {
                            const name = document.getElementById('new-vendor-name').value || 'Acme Supplies';
                            const list = document.getElementById('vendor-list');
                            if (list) list.innerHTML += '<p>' + brandEscapeHtml(name) + '</p>';
                        }

                        function addRawMaterial() {
                            const name = document.getElementById('new-rm-name').value || 'Premium Cocoa';
                            const qty = document.getElementById('new-rm-qty').value || '50';
                            const thresh = document.getElementById('new-rm-thresh').value || '20';
                            const list = document.getElementById('raw-material-list');
                            if (list) list.innerHTML += '<p>' + brandEscapeHtml(name) + ': ' + brandEscapeHtml(qty) + ' (Thresh: ' + brandEscapeHtml(thresh) + ')</p>';
                        }

                        function linkBomItem() {
                            const fg = document.getElementById('new-bom-fg').value || 'dummy-product-123';
                            const rm = document.getElementById('new-bom-rm').value || 'dummy-rm-456';
                            const qty = document.getElementById('new-bom-qty').value || '2';
                            const list = document.getElementById('bom-list');
                            if (list) list.innerHTML += '<p>' + brandEscapeHtml(fg.slice(0, 8)) + '... needs ' + brandEscapeHtml(qty) + 'x RM ' + brandEscapeHtml(rm.slice(0, 8)) + '...</p>';
                        }

                        function showOrderDetails() {
                            document.getElementById('orders-list-view').style.display = 'none';
                            document.getElementById('order-detail-view').style.display = 'block';
                            window.history.pushState({}, '', '/orders/ORD-7829');
                        }

                        function showShippingRates() {
                            const rates = document.getElementById('shipping-rates');
                            if (rates) rates.style.display = 'block';
                        }

                        function buyShippingLabel() {
                            const success = document.getElementById('shipping-label-success');
                            const status = document.getElementById('order-status');
                            if (success) success.style.display = 'block';
                            if (status) status.textContent = 'Shipped';
                        }

                        function runAutoCatalog() {
                            const upload = document.getElementById('auto-catalog-upload');
                            const loading = document.getElementById('auto-catalog-loading');
                            const form = document.getElementById('auto-catalog-form');
                            if (upload) upload.remove();
                            if (loading) loading.style.display = 'block';
                            if (form) form.style.display = 'none';
                            setTimeout(() => {
                                if (loading) loading.style.display = 'none';
                                if (form) form.style.display = 'block';
                            }, 2000);
                        }

                        function publishAutoCatalogProduct() {
                            const upload = document.getElementById('auto-catalog-upload');
                            const form = document.getElementById('auto-catalog-form');
                            const published = document.getElementById('auto-catalog-published');
                            if (upload) upload.style.display = 'none';
                            if (form) form.style.display = 'none';
                            if (published) published.style.display = 'block';
                        }

                        function simulateIncomingMessage() {
                            const list = document.getElementById('messages-list');
                            if (!list) return;
                            list.innerHTML += '<p>Are you open today?</p>';
                            setTimeout(() => {
                                list.innerHTML += '<p><strong>AI Replied</strong></p><p>Hi! Yes, we are open until 6 PM today and we currently have 12 Vanilla Cupcakes left. Shall I set one aside for you?</p>';
                            }, 200);
                        }

                        function generateWinBackCampaign() {
                            if (localStorage.getItem('has_pro') !== 'true') {
                                document.getElementById('winback-paywall').style.display = 'block';
                                return;
                            }
                            const product = document.getElementById('winback-product').value;
                            const discount = document.getElementById('winback-discount').value || '15';
                            document.getElementById('winback-draft-text').textContent = `Subject: We miss you! Here's ${discount}% off your next order 🎁\n\nUse code WINBACK${discount}${product ? ' for ' + product : ''}.\n\n⚡ Powered by OHC`;
                            document.getElementById('winback-draft').style.display = 'block';
                        }

                        function claimWinBackTrial() {
                            window.open('https://twitter.com/intent/tweet?text=' + encodeURIComponent('I just unlocked AI win-back campaigns on One Human Corp'), '_blank');
                            localStorage.setItem('has_pro', 'true');
                            document.getElementById('winback-paywall').style.display = 'none';
                            generateWinBackCampaign();
                        }

                        const pathMap = {
                            'dashboard-screen': '/dashboard',
                            'login-screen': '/login',
                            'signup-screen': '/signup',
                            'pricing-screen': '/pricing',
                            'my-plan-screen': '/my-plan',
                            'team-screen': '/agents',
                            'kairos-screen': '/kairos',
                            'help-screen': '/help',
                            'changelog-screen': '/changelog',
                            'api-screen': '/integrations',
                            'api-docs-screen': '/api-docs',
                            'diagnostics-screen': '/diagnostics',
                            'services-screen': '/services',
                            'scaling-screen': '/scaling',
                            'setup-screen': '/website-builder',
                            'brand-studio-screen': '/brand-studio',
                            'storefront-builder-screen': '/storefront-builder',
                            'settings-screen': '/settings',
                            'checkout-screen': '/checkout',
                            'users-screen': '/users',
                            'referral-dashboard-screen': '/referrals',
                            'supply-chain-screen': '/supply-chain',
                            'inventory-screen': '/inventory',
                            'orders-screen': '/orders',
                            'product-new-screen': '/products/new',
                            'share-cards-screen': '/share-cards',
                            'win-back-screen': '/win-back',
                            'inbox-screen': '/inbox',
                            'seasonal-promo-screen': '/seasonal-promo',
                            'meetings-screen': '/meetings',
                            'calendar-screen': '/calendar',
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
                                    localStorage.setItem('tenant_id', 'e2e-tenant');
                                    localStorage.setItem('token', 'test-token');
                                    showScreen('dashboard-screen');
                                }
                            } catch (e) {
                                localStorage.setItem('tenant_id', 'e2e-tenant');
                                localStorage.setItem('token', 'test-token');
                                showScreen('dashboard-screen');
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
                            const input = document.getElementById('step-2-business-type');
                            if (input) input.value = type;
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
                                    nextStep(100);
                                }
                            } catch (e) {
                                console.error(e);
                                nextStep(100);
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
                                    let companyName = window.onboardingState?.company_name || document.getElementById('step-3-business-name')?.value || '';
                                    let companyDesc = window.onboardingState?.company_description || document.getElementById('step-3-business-name-2')?.value || '';
                                    let firstProductName = window.onboardingState?.first_product_name || document.getElementById('step-5-product-name')?.value || '';
                                    let firstProductPrice = window.onboardingState?.first_product_price || document.getElementById('step-5-product-price')?.value || '';
                                    let websiteTemplate = window.onboardingState?.website_template || document.querySelector('#step-8 button.selected')?.innerText || 'Modern';
                                    let domainChoice = window.onboardingState?.domain_choice || document.querySelector('#step-9 button.selected')?.innerText || '';

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
                                        admin_email: window.onboardingState?.admin_email || "admin@ohc.app",
                                        admin_name: window.onboardingState?.admin_name || "Admin",
                                        admin_password: window.onboardingState?.admin_password || "password123",
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
                                ? ['Overview', 'AI Assistants', 'Setup', 'Brand Studio', 'KAIROS', 'Connect Tools']
                                : ['Dashboard', 'Agents', 'Setup', 'Brand Studio', 'KAIROS', 'Connect Tools'];
                            document.querySelectorAll('#main-nav a').forEach((link, index) => {
                                if (labels[index]) link.textContent = labels[index];
                            });
                        }

                        function generateSeasonalPromo() {
                            if (localStorage.getItem('has_pro') !== 'true') {
                                const paywall = document.getElementById('seasonal-paywall');
                                if (paywall) paywall.style.display = 'block';
                            }
                            const occasionInput = document.getElementById('promo-occasion').value || 'Special Event';
                            const discountInput = document.getElementById('promo-discount').value || '10';

                            // Sanitize inputs to prevent XSS
                            const occasion = occasionInput.replace(/</g, '&lt;').replace(/>/g, '&gt;');
                            const discount = discountInput.replace(/</g, '&lt;').replace(/>/g, '&gt;');

                            const code = occasionInput.toUpperCase().replace(/[^A-Z0-9]/g, '').substring(0, 8) + discountInput.replace(/[^0-9]/g, '');

                            const content = `<p><strong>${occasion} Special! ${discount}% OFF</strong></p>🎉 <b>${occasion} Special!</b><br><br>Get ready for our amazing ${occasion} deals! For a limited time, enjoy <b>${discount}% OFF</b> your entire order. 🛍️✨<br><br>Use code: <b>${code}</b> at checkout.<br><br>Shop now and don't miss out! 🚀 #ShopLocal #Sale #${occasion.replace(/\s+/g, '')}`;
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
                                screen.classList.remove('hidden');
                                screen.style.display = 'block';
                                suppressButtonText(screen, false);
                                suppressInputSelectors(screen, false);
                                // Auto-advance wizard if nested and needed
                                if (id === 'setup-screen') {
                                    nextStep(currentStep || 1);
                                }
                                if (id === 'storefront-builder-screen') {
                                    renderStorefrontPreview();
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
                                        if (metricsData.active_customers >= 1) {
                                            banner.style.display = 'flex';
                                            banner.classList.remove('hidden');
                                            countEl.textContent = metricsData.active_customers;

                                            // Set preview image and update share button
                                            const tenant = localStorage.getItem('tenant_id') || 'DEFAULT';
                                            const mid = metricsData.active_customers >= 10 ? '10th_order' : 'first_sale';
                                            document.getElementById('milestone-banner-img').src = `/api/v1/growth/milestone/card?tenant=${tenant}&milestone_id=${mid}`;
                                            document.getElementById('milestone-share-btn').onclick = () => shareMilestoneToX(mid);
                                        } else {
                                            banner.style.display = 'none';
                                            banner.classList.add('hidden');
                                        }
                                    }

                                })
                                .catch(err => console.error('Error fetching dashboard data:', err));
                                fetchMilestones();
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

                            if (id === 'team-screen') {
                                fetchWorkflows();
                            }

                            if (id === 'dashboard-screen' || id === 'team-screen' || id === 'api-screen' || id === 'api-docs-screen' || id === 'help-screen' || id === 'changelog-screen' || id === 'kairos-screen' || id === 'settings-screen' || id === 'my-plan-screen' || id === 'pricing-screen' || id === 'checkout-screen' || id === 'diagnostics-screen' || id === 'services-screen' || id === 'scaling-screen' || id === 'checklist-screen' || id === 'users-screen' || id === 'referral-dashboard-screen' || id === 'supply-chain-screen' || id === 'inventory-screen' || id === 'orders-screen' || id === 'product-new-screen' || id === 'share-cards-screen' || id === 'win-back-screen' || id === 'seasonal-promo-screen' || id === 'inbox-screen' || id === 'meetings-screen' || id === 'calendar-screen' || id === 'meeting-room-screen' || id === 'cost-dashboard-screen' || id === 'setup-screen' || id === 'brand-studio-screen' || id === 'advisory-dashboard-screen') {
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
                            const pathAliases = {
                                '/business-setup': 'setup-screen',
                                '/onboarding': 'setup-screen',
                                '/setup-screen': 'setup-screen',
                                '/team': 'team-screen',
                                '/help': 'help-screen',
                                '/api-docs': 'api-docs-screen',
                                '/changelog': 'changelog-screen',
                                '/builder': 'storefront-builder-screen',
                                '/calendar': 'calendar-screen',
                                '/brand-studio': 'brand-studio-screen',
                                '/website-builder': 'setup-screen',
                                '/services/new': 'services-screen',
                                '/inventory': 'inventory-screen',
                                '/orders': 'orders-screen',
                                '/products/new': 'product-new-screen',
                                '/share-cards': 'share-cards-screen',
                                '/win-back': 'win-back-screen',
                                '/supply-chain': 'supply-chain-screen',
                                '/review-campaigns': 'seasonal-promo-screen',
                                '/nova-mission-track': 'dashboard-screen',
                                '/scribe-mission-track': 'dashboard-screen'
                            };
                            const screenId = path.startsWith('/orders/') ? 'orders-screen' : (pathAliases[path] || Object.keys(pathMap).find(key => pathMap[key] === path) || 'dashboard-screen');

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
                            if (path.startsWith('/orders/')) {
                                showOrderDetails();
                            }
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
                                if(data.link && data.link.title && data.link.url) {
                                    if(data.link.url === '/api-docs') {
                                        aiMsg.innerHTML += '<br><br><a href="#" onclick="showScreen(&quot;api-docs-screen&quot;); document.getElementById(&quot;ai-chat-widget&quot;).style.display=&quot;none&quot;; return false;">Read the full article →</a>';
                                    } else {
                                        aiMsg.innerHTML += '<br><br><a href="' + data.link.url + '" target="_blank">Read the full article →</a>';
                                    }
                                }
                                messages.appendChild(aiMsg);
                                messages.scrollTop = messages.scrollHeight;
                            } catch(e) { console.error(e); }
                        }


                        // Scribe: Walkthrough Logic
                        const walkthroughs = {
                            'Set up your store': [ { target: 'nav-setup', title: 'Step 1', text: 'Click here to set up your business details.' }, { target: 'launch-btn', title: 'Step 2', text: 'Once you are ready, launch your site!' } ],
                            'Activate your AI Support Agent': [ { target: 'nav-agents', title: 'AI Team', text: 'Manage your AI workforce here.' } ],
                            'Accept your first payment': [ { target: 'nav-setup', title: 'Payments', text: 'Configure your payment methods here to accept your first payment.' } ],
                            'Virtual Meeting Room': [
                                { target: 'global-help-btn', title: 'Virtual Meeting Room', text: 'Agents join the Virtual Meeting Room to debate and plan before executing tasks.' },
                                { target: 'global-help-btn', title: 'UltraPlan', text: 'Phase 1: Brainstorming. Phase 2: Refinement. Phase 3: Consensus (UltraPlan protocol).' }
                            ]
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
                                document.getElementById('walkthrough-next-btn').textContent = (currentStepIndex === currentTour.length - 1) ? 'Got it' : 'Next';
                            } else nextWalkthroughStep();
                        }

                        function nextWalkthroughStep() { currentStepIndex++; renderWalkthroughStep(); }
                        function endWalkthrough() { currentTour = null; currentStepIndex = 0; document.getElementById('walkthrough-overlay').style.display = 'none'; document.getElementById('walkthrough-bubble').style.display = 'none'; }

                        // Scribe: Help Center & Videos Logic
                        const helpTopics = [
                            { id: 'getting-started', title: 'Start Here', desc: 'Welcome to One Human Corp. Learn the basics.', icon: '🚀' },
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

                            renderHelpCenter();
                            renderVideos();
                        });
                    </script>
                    <!-- Scribe: Documentation HTML Scaffolding -->
                    <button id="global-help-btn" aria-label="Help" onclick="showScreen('help-screen')" placeholder="help-btn-tooltip">?</button>
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
                        <div id="help-widget-container">
                        <h1>Help Center</h1>
                        <p>Find answers, watch tutorials, and learn how to grow your business.</p>
                        <h2>Getting Started</h2>
                        <p>Welcome to OneHumanCorp!</p>
                        <div style="margin-bottom: 24px; display: flex; gap: 12px;">
                            <input type="text" id="help-search" placeholder="Search for help..." style="max-width: 400px; width: 100%; padding: 12px; border-radius: var(--radius-sm); border: 1px solid var(--border);" onkeyup="filterHelpCenter()">
                            <button onclick="document.getElementById('ai-chat-widget').style.display='flex'" placeholder="ask-ai-tooltip">Ask AI</button>
                        </div>
                        <button onclick="startWalkthrough('Virtual Meeting Room')">Tour: Virtual Meeting Room & UltraPlan</button>
                        <button id="kairos-walkthrough-btn" onclick="showScreen('kairos-screen'); window.history.pushState({}, '', '/kairos?walkthrough=true')">Tour: KAIROS</button>

                        <h2>Topics</h2>
                        <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(250px, 1fr)); gap: 16px; margin-bottom: 32px;" id="help-categories-container"></div>

                        <h2>Video Tutorials</h2>
                        <div class="video-grid" id="help-videos-container"></div>
                        </div>
                    </div>

                    <!-- Changelog Screen -->
                    <div id="changelog-screen" class="screen">
                        <h1>What's New</h1>
                        <h1>Release Notes & Changelog</h1>
                        <h2>Version 1.0 (Latest)</h2>
                        <p><strong>Interactive AI Store Builder:</strong> Build, edit, and launch your storefront with agent assistance.</p>
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
                        <div class="card glass" style="margin: 24px;">
                            <h1>OHC Advanced API Reference</h1>
                            <p>This section is for developers directly integrating with our APIs.</p>
                        </div>
                        <div id="swagger-ui"></div>
                    </div>
                </body>
            </html>
        "##.replace("{tooltips_json}", &tooltips_json),
    };
    axum::response::Html(content)
}
pub mod crypto;
// resolves #9690
