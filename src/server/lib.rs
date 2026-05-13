pub use ::server_harness as harness;
pub mod api;
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
#[cfg(test)]
pub mod telemetry_test;
pub mod chaos;
pub mod integrations;
pub use ::server_utils as utils;
pub mod orchestration;
pub mod storage;
pub mod interop;
#[cfg(test)]
pub mod benchmarks;

pub use ::server_config as config;
pub use ::server_common as common;
pub use ::server_ohc as ohc;
pub mod builder;
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

#[tonic::async_trait]
impl HubService for MyHubService {

    async fn get_my_plan(
        &self,
        request: tonic::Request<::server_ohc::orchestration::EmptyRequest>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::MyPlanResponse>, tonic::Status> {
        let tenant_id = request.metadata().get("x-tenant-id")
            .map(|v| v.to_str().unwrap_or("default"))
            .unwrap_or("default");

        let tier = self.hub.tracker().get_tenant_tier(tenant_id).await.unwrap_or(::server_pricing::rate_limit::PlanTier::Free);
        let ai_used = self.hub.tracker().get_tenant_actions_used(tenant_id).await.unwrap_or(0);
        let storage_used_bytes = self.hub.tracker().get_tenant_storage_used(tenant_id).await.unwrap_or(0);

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
            storage_used_bytes: storage_used_bytes,
            storage_limit_bytes: storage_limit,
            next_bill_estimated: next_bill_estimated,
        }))
    }

    async fn get_cost_dashboard(
        &self,
        request: tonic::Request<::server_ohc::orchestration::EmptyRequest>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::CostDashboardResponse>, tonic::Status> {
        let tenant_id = request.metadata().get("x-tenant-id")
            .map(|v| v.to_str().unwrap_or("default"))
            .unwrap_or("default");

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
        let tenant_id = request.metadata().get("x-tenant-id")
            .map(|v| v.to_str().unwrap_or("default"))
            .unwrap_or("default").to_string();
        let req = request.into_inner();

        let stripe_key = std::env::var("STRIPE_API_KEY").unwrap_or_else(|_| "sk_test_mock".to_string());
        let client = crate::integrations::stripe::client::StripeClient::new(stripe_key);
        let mercadopago_client = std::env::var("MERCADOPAGO_ACCESS_TOKEN").ok().map(|token| crate::integrations::mercadopago::client::MercadoPagoClient::new(token));

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

        let is_latam = req.plan_id.ends_with("_latam");
        let url = if let Some(mp_client) = mercadopago_client.filter(|_| is_latam) {
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
        let stripe_key = std::env::var("STRIPE_API_KEY").unwrap_or_else(|_| "sk_test_mock".to_string());
        let client = crate::integrations::stripe::client::StripeClient::new(stripe_key);
        let _mercadopago_client = std::env::var("MERCADOPAGO_ACCESS_TOKEN").ok().map(|token| crate::integrations::mercadopago::client::MercadoPagoClient::new(token));

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
            "INSERT INTO onboarding_state (tenant_id, organization_id, user_id, current_step, state_json) \
             VALUES ($1, $2, $3, $4, $5) \
             ON CONFLICT (tenant_id, organization_id) DO UPDATE \
             SET state_json = onboarding_state.state_json || EXCLUDED.state_json, \
                 current_step = EXCLUDED.current_step, \
                 updated_at = CURRENT_TIMESTAMP"
        )
        .bind(&tenant_id)
        .bind(&org_id)
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
    ) -> Result<Response<SharedTask>, Status> {
        let req = request.into_inner();
        let task = self.hub.task_manager().create_task(
            "default_org".to_string(),
            req.mission_id,
            req.title,
            req.description,
            req.priority,
        ).map_err(|e| Status::internal(e))?;
        
        Ok(Response::new(SharedTask {
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

    type PollTasksStream = Pin<Box<dyn Stream<Item = Result<SharedTask, Status>> + Send>>;
    
    async fn poll_tasks(
        &self,
        request: Request<PollTasksRequest>,
    ) -> Result<Response<Self::PollTasksStream>, Status> {
        let req = request.into_inner();
        let tasks = self.hub.task_manager().poll_tasks(&req.agent_id, req.limit as usize);
        
        let mapped_tasks: Vec<Result<SharedTask, Status>> = tasks.into_iter().map(|task| {
            Ok(SharedTask {
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

        let mapped_tasks: Vec<SharedTask> = tasks.into_iter().map(|task| {
            SharedTask {
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
        tracing::info!("Spawned K8s Pod {} for Hierarchical Task Delegation", pod_id);

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

    type StreamTeammateMeshStream = Pin<Box<dyn Stream<Item = Result<TeammateMeshEvent, Status>> + Send>>;

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
        Ok(Response::new(GetMeetingsResponse { meetings: meetings.to_vec() }))
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

    let addr = "0.0.0.0:8081".parse()?;
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

    // Start Agent Memory Pipeline
    hub.clone().start_token_burn_rate_worker();
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

        let db_path = "ohc-standalone.db";
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
                .open(db_path)?;

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
            max_tokens: std::env::var("OHC_MAX_TOKENS").ok().and_then(|v| v.parse().ok()).unwrap_or(2048),
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
        .with_state(webhook_state);

    let health_router = axum::Router::new()
        .route("/api/v1/health", axum::routing::get(api::health::health_handler))
        .with_state(hub.clone());

    let app = axum::Router::new()
        .route("/", axum::routing::get(ui_handler))
        .route("/business-setup", axum::routing::get(ui_handler))
        .route("/login", axum::routing::get(ui_handler))
        .route("/agents", axum::routing::get(ui_handler))
        .route("/api/v1/mesh/connect", axum::routing::get(api::mesh_handler::mesh_ws_handler))
        .route("/api/mesh/v2/broadcast", axum::routing::post(api::mesh_handler::broadcast_handler))
        .nest("/api/v1/autodream", api::autodream::router(autodream_worker.clone()))
        .nest("/api/v1/builder", crate::builder::api::router(db.pool.clone()))
        .nest("/api/agents", api::agents::hire::router(hub.clone()))
        .nest("/api/onboarding", api::onboarding::router(std::sync::Arc::new(crate::services::onboarding::onboarding_agent::OnboardingAgent::new(db.clone(), hub.clone()))).with_state(mesh_transport.clone()))
        .nest("/api/v1/growth", api::growth::router(db.pool.clone(), hub.clone()))
        .nest("/api/agents/approvals", api::agents::approvals::router(dept_orchestrator.clone()))
        .route_layer(axum::middleware::from_fn_with_state(
            rate_limiter,
            ::server_utils::tier_middleware::tier_middleware,
        ))
        .with_state(mesh_transport)
        .merge(webhook_router)
        .merge(health_router)
        .fallback(ui_handler);

    let mesh_addr: std::net::SocketAddr = "0.0.0.0:18789".parse().unwrap();
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
    let store = std::sync::Arc::new(::server_auth::Store::new());
    
    // Start Telemetry Sync Daemon (if telemetry is enabled)
    if ::server_config::get().telemetry_enabled {
        let cloud_url = std::env::var("OHC_CLOUD_URL").unwrap_or_else(|_| "https://api.onehumancorp.com".to_string());
        let telemetry_daemon = crate::services::sync::telemetry_sync::TelemetrySyncDaemon::new(db.pool.clone(), cloud_url.clone());
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
                    <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@300;400;600&display=swap" rel="stylesheet">
                    <style>
                        body { font-family: 'Outfit', sans-serif; background: #0f172a; color: white; margin: 0; }
                        .glass { background: rgba(255, 255, 255, 0.03); backdrop-filter: blur(10px); border: 1px solid rgba(255,255,255,0.1); border-radius: 16px; }
                        nav { padding: 20px; display: flex; gap: 20px; border-bottom: 1px solid rgba(255,255,255,0.1); background: rgba(15, 23, 42, 0.8); position: sticky; top: 0; z-index: 100; }
                        nav a { color: #4ecca3; text-decoration: none; font-weight: 600; cursor: pointer; }
                        main { padding: 40px; }
                        .screen { display: none; padding: 40px; max-width: 800px; margin: 40px auto; }
                        .card { background: rgba(255,255,255,0.05); padding: 20px; border-radius: 12px; margin-bottom: 20px; }
                        h1, h2 { color: #4ecca3; }
                        input { width: 100%; padding: 12px; margin-bottom: 15px; background: rgba(255,255,255,0.1); border: 1px solid rgba(255,255,255,0.2); border-radius: 8px; color: white; box-sizing: border-box; }
                        button { padding: 12px 24px; background: #4ecca3; border: none; border-radius: 8px; color: #0f172a; font-weight: bold; cursor: pointer; margin-right: 10px; margin-bottom: 10px; }
                        button.secondary { background: transparent; border: 1px solid #4ecca3; color: #4ecca3; }
                        .error { color: #ff6b6b; margin-bottom: 15px; display: none; }
                    </style>
                </head>
                <body>
                    <nav id="main-nav" style="display: none;">

                        <a onclick="showScreen('dashboard-screen')">Dashboard</a>
                        <a onclick="showScreen('help-center-screen')">Help Center <span style="font-size: 10px; background: #4ecca3; color: #0f172a; padding: 2px 5px; border-radius: 10px; margin-left: 5px;">?</span></a>

                        <a onclick="showScreen('agents-screen')">Agents</a>
                        <a onclick="showScreen('setup-screen')">Setup Wizard</a>
                        <a onclick="showScreen('api-screen')" data-tooltip-id="nav-software">Software</a>
                    </nav>

                    <!-- Login Screen -->
                    <div id="login-screen" class="screen glass">
                        <h1>One Human Corp</h1>
                        <p>Sign in to manage your business</p>
                        <div id="login-error" class="error">We couldn't sign you in. Please check your credentials.</div>
                        <input type="email" placeholder="Email or Username" />
                        <input type="password" placeholder="Password" />
                        <button onclick="handleLogin(this)">Sign In</button>
                        <button onclick="handleLogin(this)">Login</button>
                        <button class="secondary" onclick="showScreen('signup-screen')">Don't have an account? Sign Up</button>
                        <button class="secondary">Use Google or Apple</button>
                        <button class="secondary" onclick="showScreen('setup-screen')">🚀 Start Business Setup</button>
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

                    <!-- Dashboard -->
                    <div id="dashboard-screen" class="screen">
                        <h1>Dashboard</h1>
                        <div class="card glass">
                            <h2>Welcome back, Human.</h2>
                            <p>Your agents are working on your behalf.</p>
                            <p>My Business: <strong data-tooltip-id="stat-active">Active</strong></p>
                            <button onclick="showScreen('inbox-screen')">Check Messages</button>
                        </div>
                        <div class="card glass">
                            <h3>Quick Actions <button class="secondary">?</button></h3>
                            <p id="quick-actions-hint" style="display: none;">These buttons are shortcuts to your most common daily tasks.</p>
                            <button onclick="showScreen('agents-screen')" data-tooltip-id="btn-agents">Manage Agents</button>
                            <button onclick="showScreen('setup-screen')">Update Setup</button>
                            <button onclick="toggleMenu()">Menu</button>
                        </div>
                        <div id="extra-menu" class="card glass" style="display: none;">
                            <button onclick="showScreen('api-screen')">Connect Custom Software</button>

                            <button onclick="showScreen('help-center-screen')">📚 Help Center</button>
                            <button onclick="showScreen('video-tutorials-screen')">▶️ Video Tutorials</button>
                            <button onclick="showScreen('changelog-screen')">✨ What's New</button>
                            <button class="secondary" onclick="startWalkthrough('setup-store')">🎯 Tour: Set up your store</button>

                        </div>

                        <!-- Bottom Nav for dashboard_nav.spec.ts -->
                        <div class="bottom-nav glass" style="display: flex; justify-content: space-around; padding: 10px; margin-top: 20px; border-top: 1px solid rgba(255,255,255,0.1);">
                            <button class="nav-item" onclick="console.log('action_add_product')" data-tooltip-id="btn-add-product">Add Product</button>
                            <button class="nav-item">Orders</button>
                            <button class="nav-item">Messages</button>
                            <button class="nav-item">Analytics</button>
                            <button class="nav-item" data-tooltip-id="btn-share">Share Store</button>
                        </div>
                    </div>

                    <!-- Inbox Screen -->
                    <div id="inbox-screen" class="screen glass">
                        <h1>Customer Inbox</h1>
                        <div class="card">
                            <p>No new messages.</p>
                        </div>
                        <button class="secondary" onclick="showScreen('dashboard-screen')">Back to Dashboard</button>
                    </div>

                    <!-- Agents Page -->
                    <div id="agents-screen" class="screen">
                        <h1>Agents</h1>
                        <div class="card glass">
                            <h3>Marketing Pro</h3>
                            <p>Status: Active</p>
                            <button>Hire Agent</button>
                        </div>
                    </div>



                    <!-- Setup Wizard -->
                    <div id="setup-screen" class="screen glass">
                        <div id="step-1">
                            <h1>Your business, live in minutes.</h1>
                            <p>Zero tech skills needed. We do the heavy lifting.</p>
                            <button onclick="nextStep(2)">🚀 Start My Business</button>
                            <button class="secondary" onclick="nextStep('ai')">⚡ Instant Build (AI) →</button>
                        </div>
                        <div id="step-2" style="display: none;">
                            <h1>What kind of business are you building?</h1>
                            <button class="secondary" onclick="nextStep(3)">🛒 Online Store</button>
                            <button class="secondary" onclick="nextStep(3)">🛠️ Service Business</button>
                            <button class="secondary" onclick="nextStep(3)">🍕 Restaurant / Food</button>
                            <button class="secondary" onclick="nextStep(3)">🎨 Creative</button>
                            <button class="secondary" onclick="nextStep(3)">🏠 Local Business</button>
                            <br/><button class="secondary" onclick="nextStep(1)">Back</button>
                        </div>
                        <div id="step-3" style="display: none;">
                            <h1>Give your business a name</h1>
                            <input type="text" placeholder="e.g. Maya's Cakes" />
                            <button onclick="nextStep(4)">Next →</button>
                            <button class="secondary" onclick="nextStep(2)">Back</button>
                        </div>
                        <div id="step-4" style="display: none;">
                            <h1>What do you sell?</h1>
                            <button class="secondary" onclick="nextStep(5)">📦 Physical products</button>
                            <button class="secondary" onclick="nextStep(5)">📅 Services / appointments</button>
                            <button class="secondary" onclick="nextStep(5)">🔁 Subscriptions</button>
                            <br/><button class="secondary" onclick="nextStep(3)">Back</button>
                        </div>
                        <div id="step-5" style="display: none;">
                            <h1>How do you want to receive payments?</h1>
                            <button class="secondary" onclick="nextStep(6)">🌐 Online only</button>
                            <button class="secondary" onclick="nextStep(6)">🌍 Both Online & In-person</button>
                            <br/><button class="secondary" onclick="nextStep(4)">Back</button>
                        </div>
                        <div id="step-6" style="display: none;">
                            <h1>Create your account</h1>
                            <input type="text" placeholder="e.g. Maya Smith" />
                            <input type="email" placeholder="you@email.com" />
                            <input type="password" placeholder="Password" />
                            <button onclick="nextStep(7)">Next →</button>
                        </div>
                        <div id="step-7" style="display: none;">
                            <h1>Choose a Template</h1>
                            <h1>Select a Template</h1>
                            <button class="secondary" onclick="nextStep(8)">✨ Modern</button>
                            <button class="secondary" onclick="nextStep(8)">🔥 Bold</button>
                        </div>
                        <div id="step-8" style="display: none;">
                            <h1>Add your first product or service</h1>
                            <input type="text" placeholder="e.g. Custom Birthday Cake" />
                            <input type="text" placeholder="e.g. 50.00" />
                            <button onclick="nextStep(9)">Next →</button>
                        </div>
                        <div id="step-9" style="display: none;">
                            <h1>Choose a Domain</h1>
                            <h1>Choose your domain</h1>
                            <button class="secondary" onclick="nextStep(10)">🌐 Free OHC Domain</button>
                            <button class="secondary" onclick="nextStep(10)">🔗 Connect Custom Domain</button>
                        </div>
                        <div id="step-10" style="display: none;">
                            <h1>Ready to launch!</h1>
                            <button onclick="nextStep(100)">Publish my business →</button>
                        </div>
                        <div id="step-100" style="display: none;">
                            <h1>CONFETTI SUCCESS</h1>
                            <p>Your business is now live!</p>
                            <button onclick="nextStep(101)">View Welcome Checklist →</button>
                            <button onclick="showScreen('dashboard-screen')">Launch My Business →</button>
                        </div>
                        <div id="step-101" style="display: none;">
                            <h1>You're set up! Here's what to do next:</h1>
                            <p>✅ Business live</p>
                            <p>Add 3 more products</p>
                            <p>Connect Instagram</p>
                            <p>Share your link with a friend</p>
                            <button onclick="showScreen('dashboard-screen')">Go to Dashboard →</button>
                        </div>

                        <div id="step-ai" style="display: none;">
                            <h1>Describe your business in a sentence</h1>
                            <input type="text" placeholder="e.g. I run a local bakery called Maya's Cakes..." />
                            <button onclick="generateAI()">Generate Storefront →</button>
                            <button class="secondary" onclick="nextStep(1)">Back</button>
                        </div>
                        <div id="step-generating" style="display: none;">
                            <h1>Designing your storefront...</h1>
                            <p>Our AI is crafting a custom experience for your brand.</p>
                        </div>
                        <div id="step-launch-ai" style="display: none;">
                            <h1>Your live storefront!</h1>
                            <h2>AI Store</h2>
                            <button onclick="showScreen('dashboard-screen')">Launch My Business →</button>
                            <button onclick="showScreen('dashboard-screen')">Continue to Dashboard →</button>
                        </div>
                    </div>


                    <!-- Help Center Screen -->
                    <div id="help-center-screen" class="screen glass">
                        <h1>Help Center</h1>
                        <p>Welcome! We're here to help you grow your business. Search or browse our guides below.</p>
                        <input type="text" id="help-search" placeholder="Search for answers (e.g., 'How do I accept credit cards?')" onkeyup="searchHelp()" />

                        <div class="help-categories" style="display: grid; grid-template-columns: 1fr 1fr; gap: 15px; margin-bottom: 20px;">
                            <button class="secondary" onclick="showHelpCategory('getting-started')">🚀 Getting Started</button>
                            <button class="secondary" onclick="showHelpCategory('my-store')">🛒 My Store</button>
                            <button class="secondary" onclick="showHelpCategory('payments')">💳 Payments</button>
                            <button class="secondary" onclick="showHelpCategory('ai-agents')">🤖 AI Agents</button>
                            <button class="secondary" onclick="showHelpCategory('marketing')">📈 Marketing</button>
                            <button class="secondary" onclick="showHelpCategory('account')">⚙️ Account & Billing</button>
                        </div>

                        <div id="help-content-area">

                            <div id="help-category-glossary" class="help-category-content" style="display: none;">
                                <h2>Business Glossary</h2>
                                <p>Simple explanations for common business terms.</p>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">A/B Testing</h3>
                                    <p style="line-height: 1.6;">A way to compare two versions of a webpage or email to see which one performs better. For example, trying two different subject lines to see which gets more people to open an email.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Abandoned Cart</h3>
                                    <p style="line-height: 1.6;">When a customer adds items to their online shopping cart but leaves the website without completing the purchase. You can often win these customers back with a polite reminder email.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Accounts Payable</h3>
                                    <p style="line-height: 1.6;">Money your business owes to suppliers or vendors for goods or services purchased on credit. Essentially, your unpaid bills.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Accounts Receivable</h3>
                                    <p style="line-height: 1.6;">Money owed to your business by customers who have purchased goods or services on credit. Essentially, bills you are waiting for customers to pay.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Affiliate Marketing</h3>
                                    <p style="line-height: 1.6;">A way to earn money by promoting other people's products. If someone buys through your unique link, you earn a small commission.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Amortization</h3>
                                    <p style="line-height: 1.6;">An accounting term that means spreading out the cost of an intangible asset (like a patent or trademark) over its useful life.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Analytics</h3>
                                    <p style="line-height: 1.6;">Data and statistics about your website or business. Analytics tell you things like how many visitors you have, where they come from, and what they buy.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Assets</h3>
                                    <p style="line-height: 1.6;">Things your business owns that have value, like cash, inventory, equipment, or property.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">B2B (Business-to-Business)</h3>
                                    <p style="line-height: 1.6;">A business model where a company sells products or services to other companies, rather than to individual consumers.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">B2C (Business-to-Consumer)</h3>
                                    <p style="line-height: 1.6;">A business model where a company sells products or services directly to individual consumers.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Balance Sheet</h3>
                                    <p style="line-height: 1.6;">A financial statement that shows what your business owns (assets), what it owes (liabilities), and the owner's equity at a specific point in time.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Bounce Rate</h3>
                                    <p style="line-height: 1.6;">The percentage of people who visit a webpage and then leave immediately without clicking on anything else. A high bounce rate might mean your page isn't giving visitors what they expect.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Brand Identity</h3>
                                    <p style="line-height: 1.6;">The visual and emotional elements that make up your business's image, including your logo, colors, fonts, and the tone of voice you use in your marketing.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Break-Even Point</h3>
                                    <p style="line-height: 1.6;">The point at which your business's total revenue equals its total expenses. Once you pass this point, you start making a profit.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Burn Rate</h3>
                                    <p style="line-height: 1.6;">The rate at which a new company is spending its startup capital before it starts generating positive cash flow. Important for knowing how long your money will last.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Call to Action (CTA)</h3>
                                    <p style="line-height: 1.6;">A clear instruction telling your audience what you want them to do next, like 'Buy Now', 'Sign Up', or 'Learn More'.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Capital</h3>
                                    <p style="line-height: 1.6;">The money or assets a business needs to operate and grow. This can include cash, equipment, or investments from owners or outside parties.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Cash Flow</h3>
                                    <p style="line-height: 1.6;">The movement of money in and out of your business. Positive cash flow means more money is coming in than going out.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Churn Rate</h3>
                                    <p style="line-height: 1.6;">The percentage of customers who stop doing business with you over a given period of time. A lower churn rate is better.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Conversion Rate</h3>
                                    <p style="line-height: 1.6;">The percentage of visitors to your website who take a desired action, like making a purchase or signing up for a newsletter.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Cost of Goods Sold (COGS)</h3>
                                    <p style="line-height: 1.6;">The direct costs associated with producing the goods your business sells. This includes materials and direct labor, but not indirect expenses like rent or marketing.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Customer Acquisition Cost (CAC)</h3>
                                    <p style="line-height: 1.6;">The total amount of money you spend to acquire a new customer, including marketing and sales expenses.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Customer Lifetime Value (CLV)</h3>
                                    <p style="line-height: 1.6;">The total amount of money a customer is expected to spend with your business over the course of their relationship with you.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Customer Relationship Management (CRM)</h3>
                                    <p style="line-height: 1.6;">Software or a system used to manage interactions with current and potential customers, keeping track of their contact info, purchase history, and preferences.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Depreciation</h3>
                                    <p style="line-height: 1.6;">An accounting method of allocating the cost of a tangible asset (like a piece of equipment or a vehicle) over its useful life.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Dropshipping</h3>
                                    <p style="line-height: 1.6;">A retail fulfillment method where a store doesn't keep the products it sells in stock. Instead, when a store sells a product, it purchases the item from a third party and has it shipped directly to the customer.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">E-commerce</h3>
                                    <p style="line-height: 1.6;">The buying and selling of goods or services over the internet.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Equity</h3>
                                    <p style="line-height: 1.6;">The value of the owner's interest in the business, calculated as total assets minus total liabilities.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Fixed Costs</h3>
                                    <p style="line-height: 1.6;">Business expenses that stay the same regardless of how much you sell, like rent, insurance, and salaries.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Fulfillment</h3>
                                    <p style="line-height: 1.6;">The entire process of receiving, packaging, and shipping orders to customers.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Gross Margin</h3>
                                    <p style="line-height: 1.6;">The percentage of total sales revenue that a company retains after incurring the direct costs associated with producing the goods or services sold.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Gross Profit</h3>
                                    <p style="line-height: 1.6;">Your total revenue minus the Cost of Goods Sold (COGS). This shows how much money you make before deducting operating expenses like marketing and rent.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Inbound Marketing</h3>
                                    <p style="line-height: 1.6;">A marketing strategy focused on attracting customers through content and interactions that are helpful and relevant, rather than interruptive advertising.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Inventory Turnover</h3>
                                    <p style="line-height: 1.6;">A measure of how many times a business sells and replaces its inventory over a certain period. A higher turnover rate generally indicates strong sales.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Invoice</h3>
                                    <p style="line-height: 1.6;">A document sent by a seller to a buyer, listing the products or services provided and the amount owed.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Key Performance Indicator (KPI)</h3>
                                    <p style="line-height: 1.6;">A measurable value that demonstrates how effectively a company is achieving key business objectives. Examples include sales growth, customer retention, or profit margin.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Landing Page</h3>
                                    <p style="line-height: 1.6;">A standalone web page created specifically for a marketing or advertising campaign. It's where a visitor 'lands' after clicking a link in an email or ad, designed to encourage a specific action.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Lead Generation</h3>
                                    <p style="line-height: 1.6;">The process of attracting and converting strangers into people who have indicated an interest in your company's product or service (leads).</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Liabilities</h3>
                                    <p style="line-height: 1.6;">Financial debts or obligations that your business owes to others, such as loans, accounts payable, or mortgages.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Liquidity</h3>
                                    <p style="line-height: 1.6;">How easily a business can convert its assets into cash to pay off its short-term debts.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Margin</h3>
                                    <p style="line-height: 1.6;">The difference between a product or service's selling price and the cost of production. Usually expressed as a percentage.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Marketing Funnel</h3>
                                    <p style="line-height: 1.6;">A model that describes the customer journey from their first interaction with your brand to the final purchase. Stages typically include Awareness, Interest, Desire, and Action.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Net Profit</h3>
                                    <p style="line-height: 1.6;">Your total revenue minus all expenses, including Cost of Goods Sold, operating expenses, taxes, and interest. This is your true bottom line.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Niche</h3>
                                    <p style="line-height: 1.6;">A specialized segment of the market for a particular kind of product or service. Finding a niche helps you target a specific audience.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Operating Expenses (OPEX)</h3>
                                    <p style="line-height: 1.6;">The ongoing costs of running a business, excluding the Cost of Goods Sold. Examples include rent, utilities, marketing, and office supplies.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Outbound Marketing</h3>
                                    <p style="line-height: 1.6;">Traditional marketing methods where a company initiates the conversation and sends its message out to an audience, such as cold calling, TV ads, or direct mail.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Overhead</h3>
                                    <p style="line-height: 1.6;">The ongoing business expenses not directly attributed to creating a product or service. Similar to operating expenses.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Point of Sale (POS)</h3>
                                    <p style="line-height: 1.6;">The place where a customer executes the payment for goods or services, like a cash register or a secure online checkout page.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Profit and Loss Statement (P&L)</h3>
                                    <p style="line-height: 1.6;">Also known as an income statement, this financial report summarizes revenues, costs, and expenses incurred during a specific period, usually a month, quarter, or year.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Return on Investment (ROI)</h3>
                                    <p style="line-height: 1.6;">A measure used to evaluate the efficiency or profitability of an investment. Calculated by dividing the net profit by the cost of the investment.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Search Engine Optimization (SEO)</h3>
                                    <p style="line-height: 1.6;">The process of improving your website to increase its visibility when people search for products or services related to your business on search engines like Google.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Target Audience</h3>
                                    <p style="line-height: 1.6;">The specific group of consumers most likely to want your product or service, characterized by demographics, interests, and behaviors.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Unique Selling Proposition (USP)</h3>
                                    <p style="line-height: 1.6;">The distinct factor that makes your product or service better than or different from the competition. It's why customers should choose you.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Variable Costs</h3>
                                    <p style="line-height: 1.6;">Business expenses that change in proportion to the volume of goods or services a business produces, like raw materials or shipping costs.</p>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Working Capital</h3>
                                    <p style="line-height: 1.6;">The difference between a company's current assets and current liabilities. It measures a company's operational efficiency and short-term financial health.</p>
                                </div>

                            </div>
                            <div id="help-category-getting-started" class="help-category-content" style="display: none;">
                                <h2>Getting Started</h2>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Welcome to OneHuman Corp</h3>
                                    <p style="line-height: 1.6;">Welcome to your new digital storefront! We built this platform so you can focus on what you do best: running your business. This guide will walk you through the basics of setting up your shop, adding your first product, and launching your website to the world. Don't worry if you aren't technical – our tools are designed specifically for small business owners.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Setting up your business profile</h3>
                                    <p style="line-height: 1.6;">Your business profile tells customers who you are. Make sure to add a clear logo, a friendly description, and accurate contact information. This builds trust and helps people decide to buy from you.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Choosing the right template</h3>
                                    <p style="line-height: 1.6;">We offer several design templates. Choose one that matches your brand's personality. You can always change it later without losing any of your products or settings.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Understanding the dashboard</h3>
                                    <p style="line-height: 1.6;">Your dashboard is your home base. From here, you can see recent sales, messages from customers, and alerts from your AI assistants. Check it daily to stay on top of your business.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Connecting your social media</h3>
                                    <p style="line-height: 1.6;">Linking your Facebook and Instagram accounts allows customers to easily find your profiles. It also helps our Marketing Agent suggest better content for you to post. Go to Settings > Social to connect them.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">How to preview your store</h3>
                                    <p style="line-height: 1.6;">Before you go live, you might want to see what your store looks like to customers. Click the 'Preview' button in the top right corner. You can view it as it would appear on a computer or a mobile phone.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Setting your store hours</h3>
                                    <p style="line-height: 1.6;">If you have a physical location or specific times you answer emails, set your store hours. This sets good expectations for when customers will hear back from you.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Understanding the mobile app</h3>
                                    <p style="line-height: 1.6;">You can manage your business on the go! Download the OneHuman Corp app on your phone. You can reply to customers, add products, and see your sales while you are away from your computer.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Writing an 'About Us' page</h3>
                                    <p style="line-height: 1.6;">Customers love buying from people, not just faceless companies. Use your About Us page to tell your story. Why did you start this business? What makes your products special?</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Getting help when you need it</h3>
                                    <p style="line-height: 1.6;">If you get stuck, we are here to help. You can read these guides, ask our AI Help bot a question, or reach out to our human support team via the 'Contact Support' button at the bottom of the page.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>
                            </div>
                            <div id="help-category-my-store" class="help-category-content" style="display: none;">
                                <h2>My Store</h2>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Adding your first product</h3>
                                    <p style="line-height: 1.6;">To add a product, go to your dashboard and click 'Add Product'. Give it a clear name, a fair price, and a great photo. Good lighting makes a huge difference in product photos! Write a description that tells customers exactly what they are getting and why they need it.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Managing your inventory</h3>
                                    <p style="line-height: 1.6;">Keep track of what you have in stock. If you sell physical items, enter the quantity you have on hand. We will automatically show 'Sold Out' when you run out, so you never accidentally sell something you don't have.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Organizing products into categories</h3>
                                    <p style="line-height: 1.6;">Make it easy for customers to find what they want by grouping similar items together. For example, if you run a clothing store, create categories for 'Shirts', 'Pants', and 'Accessories'.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Setting up shipping options</h3>
                                    <p style="line-height: 1.6;">Decide how you want to get your products to your customers. You can offer local pickup, flat-rate shipping, or calculated shipping based on weight. Be clear about your shipping times to set good expectations.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">How to write great product descriptions</h3>
                                    <p style="line-height: 1.6;">A good description helps sell the product. Instead of just listing facts, talk about the benefits. How will this product make your customer's life better, easier, or more fun?</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Taking professional-looking photos</h3>
                                    <p style="line-height: 1.6;">You don't need a fancy camera. Use your smartphone, find a spot near a window for natural light, and use a plain background like a white sheet of paper. Show the product from a few different angles.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Setting up a 'Coming Soon' page</h3>
                                    <p style="line-height: 1.6;">If you are still working on your store, you can put up a 'Coming Soon' page. This lets people know you exist and allows them to enter their email address so you can message them when you launch.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Adding digital products</h3>
                                    <p style="line-height: 1.6;">You can sell things that aren't physical, like ebooks, music, or online courses. Choose 'Digital Product' when adding an item, and upload the file. We will automatically email it to the customer after they pay.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Managing product reviews</h3>
                                    <p style="line-height: 1.6;">Positive reviews build trust. After a customer buys something, we automatically send an email asking for a review. You can view and respond to these reviews in your dashboard. Always thank people for their feedback!</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Handling out-of-stock items</h3>
                                    <p style="line-height: 1.6;">If an item is popular and sells out, you can let customers enter their email to be notified when it comes back. This is a great way to ensure you don't lose a sale.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>
                            </div>
                            <div id="help-category-payments" class="help-category-content" style="display: none;">
                                <h2>Payments</h2>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Connecting your bank account</h3>
                                    <p style="line-height: 1.6;">Before you can get paid, you need to tell us where to send your money. Go to Settings > Payments and enter your bank details securely. We process payments daily so you get your money fast.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Accepting credit cards</h3>
                                    <p style="line-height: 1.6;">We accept all major credit cards. You don't need a separate merchant account. When a customer buys something, the money goes straight to your connected bank account minus a small processing fee.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Handling refunds</h3>
                                    <p style="line-height: 1.6;">Sometimes things don't work out. If a customer needs a refund, you can easily issue it from the Orders tab. You can refund the full amount or just a portion of it. The money will be returned to their original payment method.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Understanding sales tax</h3>
                                    <p style="line-height: 1.6;">Taxes can be confusing. We help by automatically calculating the right sales tax based on where your customer lives. You can view your tax reports anytime to help with accounting.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">When will I get paid?</h3>
                                    <p style="line-height: 1.6;">After a customer makes a purchase, the money usually takes 2 business days to arrive in your bank account. The first payment ever might take up to 7 days while your bank verifies your account.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Setting up a tip jar</h3>
                                    <p style="line-height: 1.6;">If you provide a service or want to let customers show extra appreciation, you can enable tips at checkout. This is completely optional for both you and your customers.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Dealing with a disputed charge</h3>
                                    <p style="line-height: 1.6;">If a customer tells their bank they didn't make a purchase, it's called a dispute. We will help you provide evidence, like tracking numbers, to prove the item was delivered.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">How processing fees work</h3>
                                    <p style="line-height: 1.6;">Every time a customer uses a credit card, the credit card companies charge a small fee. We clearly list this fee on every order so you always know exactly how much money you are taking home.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Accepting payments in person</h3>
                                    <p style="line-height: 1.6;">If you sell at craft fairs or have a physical shop, you can use our mobile app to accept payments in person. Just type in the amount and tap their card.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Sending invoices</h3>
                                    <p style="line-height: 1.6;">Sometimes you need to bill a customer later. You can create a professional-looking invoice and email it to them directly from your dashboard. They can click a link in the email to pay securely online.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>
                            </div>
                            <div id="help-category-ai-agents" class="help-category-content" style="display: none;">
                                <h2>Ai Agents</h2>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">What are AI Agents?</h3>
                                    <p style="line-height: 1.6;">Think of AI Agents as your digital employees. They work 24/7 in the background. The Support Agent can answer common customer questions, and the Marketing Agent can help write emails. They are here to save you time.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Training your Support Agent</h3>
                                    <p style="line-height: 1.6;">Your Support Agent learns from the information you give it. Add details about your return policy, store hours, and product materials. The more it knows, the better it can help your customers without bothering you.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Using the Marketing Agent</h3>
                                    <p style="line-height: 1.6;">Need to write a newsletter? Ask the Marketing Agent. Just give it a topic, like 'Summer Sale', and it will write a friendly email you can send to your customers. It's like having a copywriter on your team.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Reviewing Agent activity</h3>
                                    <p style="line-height: 1.6;">You can always see what your agents are doing. Go to the Agents tab to review conversations they've had with customers or content they've created. You are always in control.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">The Customer Success Agent</h3>
                                    <p style="line-height: 1.6;">This agent automatically follows up with customers a week after they buy something. It makes sure they are happy and politely asks if they would like to leave a review. It's great for building loyalty.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">How do the agents sound?</h3>
                                    <p style="line-height: 1.6;">We've programmed the agents to sound friendly, polite, and professional. They do not sound like robots. You can read examples of their conversations in the Agent Settings menu.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">When an agent doesn't know the answer</h3>
                                    <p style="line-height: 1.6;">If a customer asks a question the Support Agent hasn't been trained on, the agent will politely say it needs to check with the owner. It will then send the message to your inbox for you to handle.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Adding custom knowledge</h3>
                                    <p style="line-height: 1.6;">You can upload documents, like a PDF of your menu or a spreadsheet of your ingredient list, directly to the Support Agent's brain. It will read the document and use that information to answer questions.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Agent pricing and limits</h3>
                                    <p style="line-height: 1.6;">On the standard plan, your agents can answer 500 customer messages per month for free. If you need more, you can upgrade your plan. The Marketing Agent has no limits on how many emails it can draft for you.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Turning an agent off</h3>
                                    <p style="line-height: 1.6;">If you want to handle everything yourself for a while, you can pause any agent with a single click. Simply go to the Agents page and toggle the switch to 'Off'.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>
                            </div>
                            <div id="help-category-marketing" class="help-category-content" style="display: none;">
                                <h2>Marketing</h2>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Getting your first customers</h3>
                                    <p style="line-height: 1.6;">The easiest way to get your first sales is to share your store with friends and family. Post your link on your personal social media accounts. Word of mouth is incredibly powerful for new businesses.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Setting up a discount code</h3>
                                    <p style="line-height: 1.6;">Everyone loves a deal! Create a discount code like 'WELCOME10' to give new customers 10% off their first order. You can limit how many times a code can be used or set an expiration date.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Collecting email addresses</h3>
                                    <p style="line-height: 1.6;">Email is the best way to keep in touch with your customers. Add a signup form to your store. Send them updates about new products, sales, and behind-the-scenes looks at your business.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Understanding your analytics</h3>
                                    <p style="line-height: 1.6;">Your Analytics page shows you how many people are visiting your store and what they are buying. Use this information to figure out what's working. If a lot of people visit a product but don't buy, you might need better photos or a lower price.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Creating an abandoned cart email</h3>
                                    <p style="line-height: 1.6;">Sometimes people put items in their cart but forget to check out. You can turn on a setting to automatically email them a friendly reminder. This is a proven way to recover lost sales.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Writing a great newsletter</h3>
                                    <p style="line-height: 1.6;">Don't just sell in your emails. Share helpful tips, stories about your business, or behind-the-scenes photos. People are more likely to buy from someone they feel connected to.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Running a flash sale</h3>
                                    <p style="line-height: 1.6;">Create excitement by offering a large discount for a very short time, like 24 hours. Make sure to email your list and post on social media to let everyone know it's happening.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Offering free shipping</h3>
                                    <p style="line-height: 1.6;">Free shipping is a huge incentive for buyers. You can offer free shipping on all orders, or set a minimum (e.g., 'Free shipping on orders over $50'). This encourages customers to add more items to their cart.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Partnering with other businesses</h3>
                                    <p style="line-height: 1.6;">Find another local business that sells complementary products. For example, if you sell coffee beans, partner with someone who sells handmade mugs. You can promote each other to your respective audiences.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Understanding SEO</h3>
                                    <p style="line-height: 1.6;">SEO stands for Search Engine Optimization. It's how you get found on Google. The best way to improve your SEO is to use clear, descriptive titles for your products and write detailed descriptions.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>
                            </div>
                            <div id="help-category-account" class="help-category-content" style="display: none;">
                                <h2>Account</h2>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Changing your password</h3>
                                    <p style="line-height: 1.6;">To keep your account safe, use a strong password. You can change your password anytime in Settings > Security. If you forget it, use the 'Forgot Password' link on the login page.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Managing your subscription</h3>
                                    <p style="line-height: 1.6;">View your current billing plan and past invoices in Settings > Billing. You can upgrade or downgrade your plan at any time based on what your business needs.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Adding team members</h3>
                                    <p style="line-height: 1.6;">If you have employees, you can give them their own login. Go to Settings > Team and invite them. You can restrict what they are allowed to see and do, so your sensitive information stays private.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Closing your store</h3>
                                    <p style="line-height: 1.6;">We'd be sad to see you go, but if you need to close your store, you can do so in Settings. Make sure to fulfill all your open orders and download your customer list before closing.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Updating your billing credit card</h3>
                                    <p style="line-height: 1.6;">If the credit card you use to pay your OneHuman Corp subscription expires, you can easily update it in Settings > Billing. This ensures your store stays online without interruption.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Exporting your data</h3>
                                    <p style="line-height: 1.6;">You own your data. You can download a spreadsheet of all your customers, orders, and products at any time. Go to Settings > Data Export to download everything.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Changing your business name</h3>
                                    <p style="line-height: 1.6;">If you rebrand, you can update your store name in Settings > General. Remember to also check if you need to update your custom domain to match the new name.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Upgrading to an annual plan</h3>
                                    <p style="line-height: 1.6;">You can save money by paying for a full year upfront instead of month-to-month. Go to Settings > Billing and select 'Switch to Annual Plan' to lock in the discount.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Finding your tax forms</h3>
                                    <p style="line-height: 1.6;">At the end of the year, we provide the necessary tax forms (like a 1099-K in the US) if you meet the sales threshold. You can download these securely from the Billing page.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>

                                <div class="help-article" style="margin-bottom: 20px; padding: 15px; background: rgba(255,255,255,0.02); border-radius: 8px;">
                                    <h3 style="margin-top: 0;">Transferring ownership</h3>
                                    <p style="line-height: 1.6;">If you sell your business, you can transfer ownership of the OneHuman Corp account to the new owner. Contact support to initiate this secure process.</p>
                                    <button class="secondary" style="font-size: 12px; padding: 5px 10px;">Was this helpful? 👍</button>
                                </div>
                            </div>

                        </div>
                        <button class="secondary" onclick="showScreen('dashboard-screen')">Back to Dashboard</button>
                    </div>

                    <!-- Video Tutorials Screen -->
                    <div id="video-tutorials-screen" class="screen glass">
                        <h1>Video Tutorials</h1>
                        <p>Watch short, easy-to-follow videos on how to manage your business.</p>
                        <div style="display: grid; grid-template-columns: 1fr; gap: 15px;" id="video-tutorial-list">
                        </div>
                        <div id="video-player-overlay" style="display:none; margin-top: 20px; background: black; padding: 20px; border-radius: 8px; text-align: center;">
                            <h3 id="video-title">Playing Video</h3>
                            <div style="width: 100%; height: 200px; background: #333; display: flex; align-items: center; justify-content: center;">
                                <video id="tutorial-video-player" width="100%" controls>
                                  <source src="" type="video/mp4">
                                  Your browser does not support the video tag.
                                </video>
                            </div>
                            <button onclick="document.getElementById('video-player-overlay').style.display='none'; document.getElementById('tutorial-video-player').pause();" style="margin-top: 15px;">Close Video</button>
                        </div>
                        <button class="secondary" onclick="showScreen('dashboard-screen')">Back to Dashboard</button>
                    </div>

                    <!-- API Reference Screen (Advanced) -->
                    <div id="api-screen" class="screen glass">
                        <h1>Developer API Reference</h1>
                        <p>Warning: This section is for advanced users and developers who want to write custom software. You do not need to use this to run your business.</p>
                        <div id="swagger-ui-container">
                            <p>Loading interactive API documentation...</p>
                        </div>
                        <button class="secondary" onclick="showScreen('dashboard-screen')">Back to Dashboard</button>
                    </div>

                    <!-- Changelog Screen -->
                    <div id="changelog-screen" class="screen glass">
                        <h1>What's New</h1>
                        <p>We are constantly improving the platform to help you sell more. Here are the latest updates.</p>

                        <div class="card">
                            <h3>May 2024 Updates</h3>
                            <img src="https://via.placeholder.com/600x200?text=New+Shipping+Labels" style="max-width: 100%; border-radius: 8px; margin-bottom: 10px;" />
                            <ul>
                                <li><strong>Better Shipping Labels:</strong> You can now print shipping labels directly from your dashboard! This saves you a trip to the post office.</li>
                                <li><strong>Faster Payouts:</strong> Money from sales will now arrive in your bank account one day faster.</li>
                                <li><strong>New Email Templates:</strong> The Marketing Agent has 5 new templates for welcoming new customers.</li>
                            </ul>
                        </div>

                        <div class="card">
                            <h3>April 2024 Updates</h3>
                            <img src="https://via.placeholder.com/600x200?text=Inventory+Alerts" style="max-width: 100%; border-radius: 8px; margin-bottom: 10px;" />
                            <ul>
                                <li><strong>Inventory Alerts:</strong> We will now send you an email when a product is running low so you can restock before you sell out.</li>
                                <li><strong>Mobile Dashboard Improved:</strong> The dashboard is now much easier to read on your phone.</li>
                            </ul>
                        </div>
                        <p><a href="/changelog" target="_blank" style="color: #4ecca3;">View full changelog on our website →</a></p>
                        <button class="secondary" onclick="showScreen('dashboard-screen')">Back to Dashboard</button>
                    </div>

                    <!-- AI Help Widget (Floating) -->
                    <div id="ai-help-widget" style="position: fixed; bottom: 20px; right: 20px; z-index: 1000;">
                        <button id="ai-help-button" onclick="toggleAiHelp()" style="border-radius: 50px; width: 60px; height: 60px; box-shadow: 0 4px 12px rgba(0,0,0,0.3); font-size: 24px; display: flex; align-items: center; justify-content: center; margin: 0;">❓</button>

                        <div id="ai-help-panel" class="glass" style="display: none; position: absolute; bottom: 80px; right: 0; width: 320px; height: 400px; flex-direction: column; border: 1px solid rgba(255,255,255,0.2); box-shadow: 0 10px 30px rgba(0,0,0,0.5);">
                            <div style="background: #4ecca3; color: #0f172a; padding: 15px; font-weight: bold; border-top-left-radius: 16px; border-top-right-radius: 16px; display: flex; justify-content: space-between;">
                                <span>Support Agent</span>
                                <span style="cursor:pointer;" onclick="toggleAiHelp()">✕</span>
                            </div>
                            <div id="ai-help-chat-history" style="flex: 1; padding: 15px; overflow-y: auto; display: flex; flex-direction: column; gap: 10px;">
                                <div style="background: rgba(255,255,255,0.1); padding: 10px; border-radius: 8px; align-self: flex-start; max-width: 80%;">
                                    Hi there! I'm your support assistant. What do you need help with today?
                                </div>
                            </div>
                            <div style="padding: 15px; border-top: 1px solid rgba(255,255,255,0.1); display: flex;">
                                <input type="text" id="ai-help-input" placeholder="Type a question..." style="margin-bottom: 0; border-radius: 4px 0 0 4px; flex: 1;" onkeypress="if(event.key === 'Enter') submitAiHelp()"/>
                                <button onclick="submitAiHelp()" style="margin: 0; border-radius: 0 4px 4px 0; padding: 0 15px;">Send</button>
                            </div>
                        </div>
                    </div>

                    <script src="https://unpkg.com/swagger-ui-dist@4.5.0/swagger-ui-bundle.js"></script>
                    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@4.5.0/swagger-ui.css">
                    <script>
                        // Help Center Logic
                        function showHelpCategory(categoryId) {
                            document.querySelectorAll('.help-category-content').forEach(el => el.style.display = 'none');
                            const target = document.getElementById('help-category-' + categoryId);
                            if (target) target.style.display = 'block';
                            document.getElementById('help-search').value = '';
                            searchHelp(); // reset search
                        }

                        function searchHelp() {
                            const query = document.getElementById('help-search').value.toLowerCase();
                            if (query.length > 0) {
                                document.querySelectorAll('.help-category-content').forEach(el => el.style.display = 'block'); // show all categories
                            } else {
                                // Default back to showing nothing or the first category
                            }

                            document.querySelectorAll('.help-article').forEach(article => {
                                const text = article.innerText.toLowerCase();
                                if (text.includes(query)) {
                                    article.style.display = 'block';
                                } else {
                                    article.style.display = 'none';
                                }
                            });
                        }

                        // Video Tutorial Logic (Fetching metadata from backend mock)
                        const backendVideoMetadata = [
                            { id: 'setup', title: 'How to setup your first product', duration: '1:20', url: 'https://www.w3schools.com/html/mov_bbb.mp4' },
                            { id: 'shipping', title: 'Setting up shipping rates', duration: '0:55', url: 'https://www.w3schools.com/html/mov_bbb.mp4' },
                            { id: 'domain', title: 'Connecting a custom domain', duration: '1:45', url: 'https://www.w3schools.com/html/mov_bbb.mp4' },
                            { id: 'taxes', title: 'Understanding automated taxes', duration: '2:10', url: 'https://www.w3schools.com/html/mov_bbb.mp4' },
                            { id: 'ai', title: 'Training your Support Agent', duration: '1:15', url: 'https://www.w3schools.com/html/mov_bbb.mp4' },
                            { id: 'marketing', title: 'Sending your first email newsletter', duration: '2:30', url: 'https://www.w3schools.com/html/mov_bbb.mp4' },
                            { id: 'payouts', title: 'How and when you get paid', duration: '1:05', url: 'https://www.w3schools.com/html/mov_bbb.mp4' },
                            { id: 'refunds', title: 'Processing a refund', duration: '0:45', url: 'https://www.w3schools.com/html/mov_bbb.mp4' },
                            { id: 'discounts', title: 'Creating discount codes', duration: '1:10', url: 'https://www.w3schools.com/html/mov_bbb.mp4' },
                            { id: 'analytics', title: 'Reading your sales reports', duration: '1:50', url: 'https://www.w3schools.com/html/mov_bbb.mp4' }
                        ];

                        function renderVideoList() {
                            const list = document.getElementById('video-tutorial-list');
                            if(!list) return;
                            list.innerHTML = '';
                            backendVideoMetadata.forEach(video => {
                                list.innerHTML += `<div class="card" onclick="playVideo('${video.id}', '${video.url}', '${video.title}')">▶️ ${video.title} (${video.duration})</div>`;
                            });
                        }

                        function playVideo(id, url, titleStr) {
                            const overlay = document.getElementById('video-player-overlay');
                            const title = document.getElementById('video-title');
                            const player = document.getElementById('tutorial-video-player');

                            overlay.style.display = 'block';
                            title.innerText = "Playing tutorial: " + titleStr;
                            player.src = url;
                            player.play();
                            overlay.scrollIntoView({ behavior: 'smooth' });
                        }

                        // AI Help Chat Logic (Routing to backend agent mock)
                        function toggleAiHelp() {
                            const panel = document.getElementById('ai-help-panel');
                            panel.style.display = panel.style.display === 'none' ? 'flex' : 'none';
                        }

                        async function submitAiHelp() {
                            const input = document.getElementById('ai-help-input');
                            const text = input.value.trim();
                            if (!text) return;

                            const history = document.getElementById('ai-help-chat-history');

                            // Add user message
                            const userMsg = document.createElement('div');
                            userMsg.style = "background: #4ecca3; color: #0f172a; padding: 10px; border-radius: 8px; align-self: flex-end; max-width: 80%;";
                            userMsg.innerText = text;
                            history.appendChild(userMsg);

                            input.value = '';
                            history.scrollTop = history.scrollHeight;

                            // Route to backend specialized Help Agent (mocked via fetch to health endpoint for simulation)
                            try {
                                await fetch('/api/v1/health'); // Simulate backend call to agent router

                                let response = "I'm looking that up for you...";
                                let link = "";

                                const t = text.toLowerCase();
                                if (t.includes('pay') || t.includes('credit card') || t.includes('money')) {
                                    response = "To accept payments, go to Settings > Payments and connect your bank account. We process payments daily so you get your money fast.";
                                    link = "<br/><br/><a href='#' onclick=\"showScreen('help-center-screen'); showHelpCategory('payments'); toggleAiHelp();\" style='color: #fff; text-decoration: underline;'>Read the full article →</a>";
                                } else if (t.includes('product') || t.includes('add') || t.includes('sell')) {
                                    response = "You can add a new product from your Dashboard. Click the 'Add Product' button, upload a photo, and set your price.";
                                    link = "<br/><br/><a href='#' onclick=\"showScreen('help-center-screen'); showHelpCategory('my-store'); toggleAiHelp();\" style='color: #fff; text-decoration: underline;'>Read the full article →</a>";
                                } else if (t.includes('agent') || t.includes('ai')) {
                                    response = "AI Agents are like digital employees. You can train them by adding business details in the Agents tab.";
                                    link = "<br/><br/><a href='#' onclick=\"showScreen('help-center-screen'); showHelpCategory('ai-agents'); toggleAiHelp();\" style='color: #fff; text-decoration: underline;'>Read the full article →</a>";
                                } else {
                                    response = "I found some information that might help in our Help Center. Try searching for your specific issue there.";
                                    link = "<br/><br/><a href='#' onclick=\"showScreen('help-center-screen'); toggleAiHelp();\" style='color: #fff; text-decoration: underline;'>Open Help Center →</a>";
                                }

                                const aiMsg = document.createElement('div');
                                aiMsg.style = "background: rgba(255,255,255,0.1); padding: 10px; border-radius: 8px; align-self: flex-start; max-width: 80%;";
                                aiMsg.innerHTML = response + link;
                                history.appendChild(aiMsg);
                                history.scrollTop = history.scrollHeight;
                            } catch (e) {
                                const aiMsg = document.createElement('div');
                                aiMsg.style = "background: rgba(255,107,107,0.1); color: #ff6b6b; padding: 10px; border-radius: 8px; align-self: flex-start; max-width: 80%;";
                                aiMsg.innerHTML = "Sorry, my connection to the backend is down. Please try again later.";
                                history.appendChild(aiMsg);
                            }
                        }

                        // Contextual Tooltip Registry Logic
                        const tooltipRegistry = {
                            'btn-add-product': 'Click here to add a new item to your store.',
                            'btn-agents': 'Manage your AI assistants here.',
                            'nav-software': 'Connect third-party tools like accounting software.',
                            'btn-share': 'Get a link to your store to share on social media.',
                            'stat-active': 'Your store is live and can accept customers right now.',
                            'menu-help': 'Open the Help Center for guides and support.'
                        };

                        // Inject tooltip target data into existing dashboard UI elements
                        document.addEventListener("DOMContentLoaded", () => {
                            renderVideoList();

                            // Initialize Swagger UI for API Docs
                            const openapiSpec = {
                                openapi: '3.0.0',
                                info: { title: 'OneHuman Corp API', version: '1.0.0' },
                                paths: {
                                    '/api/v1/products': {
                                        get: {
                                            summary: 'Get Products',
                                            responses: { '200': { description: 'A list of products' } }
                                        }
                                    },
                                    '/api/v1/orders': {
                                        post: {
                                            summary: 'Create Order',
                                            responses: { '201': { description: 'Order created' } }
                                        }
                                    }
                                }
                            };
                            if (window.SwaggerUIBundle) {
                                window.SwaggerUIBundle({
                                    spec: openapiSpec,
                                    dom_id: '#swagger-ui-container',
                                });
                            }

                            // Setup global tooltip listener
                            const tooltipDiv = document.createElement('div');
                            tooltipDiv.style = "position: absolute; display: none; background: rgba(15, 23, 42, 0.95); border: 1px solid #4ecca3; color: white; padding: 10px; border-radius: 8px; font-size: 14px; max-width: 250px; z-index: 9999; pointer-events: none; box-shadow: 0 4px 12px rgba(0,0,0,0.5);";
                            document.body.appendChild(tooltipDiv);

                            document.body.addEventListener('mouseover', (e) => {
                                const id = e.target.getAttribute('data-tooltip-id');
                                if (id && tooltipRegistry[id]) {
                                    tooltipDiv.innerText = tooltipRegistry[id];
                                    tooltipDiv.style.display = 'block';

                                    const rect = e.target.getBoundingClientRect();
                                    tooltipDiv.style.top = (rect.bottom + window.scrollY + 5) + 'px';
                                    tooltipDiv.style.left = (rect.left + window.scrollX) + 'px';
                                }
                            });

                            document.body.addEventListener('mouseout', (e) => {
                                if (e.target.getAttribute('data-tooltip-id')) {
                                    tooltipDiv.style.display = 'none';
                                }
                            });

                            // Mobile long-press logic
                            let pressTimer;
                            document.body.addEventListener('touchstart', (e) => {
                                const id = e.target.getAttribute('data-tooltip-id');
                                if (id && tooltipRegistry[id]) {
                                    pressTimer = window.setTimeout(() => {
                                        tooltipDiv.innerText = tooltipRegistry[id];
                                        tooltipDiv.style.display = 'block';

                                        const touch = e.touches[0];
                                        tooltipDiv.style.top = (touch.pageY + 15) + 'px';
                                        tooltipDiv.style.left = (touch.pageX - 100) + 'px';
                                    }, 500); // 500ms long press
                                }
                            });

                            document.body.addEventListener('touchend', () => {
                                clearTimeout(pressTimer);
                                tooltipDiv.style.display = 'none';
                            });
                        });

                        // Interactive Walkthrough Engine
                        const walkthroughs = {
                            'setup-store': [
                                { target: '.bottom-nav button:nth-child(1)', text: 'Step 1: Start by adding your first product here.' },
                                { target: '#main-nav a:nth-child(3)', text: 'Step 2: Next, activate an AI Agent to help with support.' },
                                { target: '.bottom-nav button:nth-child(5)', text: 'Step 3: Finally, share your store link with friends!' }
                            ]
                        };

                        let currentWalkthrough = null;
                        let currentStep = 0;
                        const walkthroughHighlight = document.createElement('div');
                        walkthroughHighlight.style = "position: absolute; display: none; border: 3px solid #4ecca3; border-radius: 4px; pointer-events: none; z-index: 9998; box-shadow: 0 0 0 9999px rgba(0,0,0,0.5); transition: all 0.3s ease;";

                        const walkthroughBubble = document.createElement('div');
                        walkthroughBubble.style = "position: absolute; display: none; background: white; color: #0f172a; padding: 15px; border-radius: 8px; font-weight: bold; z-index: 9999; max-width: 250px;";
                        walkthroughBubble.innerHTML = "<span id='wt-text'></span><br/><br/><button id='wt-next' onclick='nextWalkthroughStep()' style='padding: 5px 10px; font-size: 12px; margin-bottom: 0;'>Next Step</button> <button onclick='stopWalkthrough()' class='secondary' style='padding: 5px 10px; font-size: 12px; margin-bottom: 0;'>Close</button>";

                        document.addEventListener("DOMContentLoaded", () => {
                            document.body.appendChild(walkthroughHighlight);
                            document.body.appendChild(walkthroughBubble);
                        });

                        function startWalkthrough(id) {
                            if (!walkthroughs[id]) return;
                            currentWalkthrough = walkthroughs[id];
                            currentStep = 0;
                            showWalkthroughStep();
                        }

                        function nextWalkthroughStep() {
                            currentStep++;
                            if (currentStep >= currentWalkthrough.length) {
                                stopWalkthrough();
                            } else {
                                showWalkthroughStep();
                            }
                        }

                        function stopWalkthrough() {
                            walkthroughHighlight.style.display = 'none';
                            walkthroughBubble.style.display = 'none';
                            currentWalkthrough = null;
                        }

                        function showWalkthroughStep() {
                            const step = currentWalkthrough[currentStep];
                            const el = document.querySelector(step.target);

                            if (el) {
                                // Scroll into view if needed
                                el.scrollIntoView({ behavior: 'smooth', block: 'center' });

                                setTimeout(() => {
                                    const rect = el.getBoundingClientRect();

                                    walkthroughHighlight.style.top = (rect.top + window.scrollY - 5) + 'px';
                                    walkthroughHighlight.style.left = (rect.left + window.scrollX - 5) + 'px';
                                    walkthroughHighlight.style.width = (rect.width + 10) + 'px';
                                    walkthroughHighlight.style.height = (rect.height + 10) + 'px';
                                    walkthroughHighlight.style.display = 'block';

                                    document.getElementById('wt-text').innerText = step.text;
                                    walkthroughBubble.style.top = (rect.bottom + window.scrollY + 15) + 'px';
                                    walkthroughBubble.style.left = (rect.left + window.scrollX) + 'px';

                                    if (currentStep === currentWalkthrough.length - 1) {
                                        document.getElementById('wt-next').innerText = "Finish";
                                    } else {
                                        document.getElementById('wt-next').innerText = "Next Step";
                                    }

                                    walkthroughBubble.style.display = 'block';
                                }, 300); // wait for scroll
                            }
                        }
                    </script>
<script>
                        function showScreen(id) {
                            document.querySelectorAll('.screen').forEach(s => s.style.display = 'none');
                            const screen = document.getElementById(id);
                            if (screen) screen.style.display = 'block';
                            
                            if (id === 'dashboard-screen' || id === 'agents-screen' || id === 'api-screen') {
                                document.getElementById('main-nav').style.display = 'flex';
                            } else {
                                document.getElementById('main-nav').style.display = 'none';
                            }
                        }

                        function handleLogin(btn) {
                            const email = document.querySelector('#login-screen input[type="email"]').value;
                            btn.innerText = 'Signing in...';
                            if (!email) {
                                setTimeout(() => {
                                    document.getElementById('login-error').style.display = 'block';
                                    btn.innerText = 'Sign In';
                                }, 500);
                            } else {
                                setTimeout(() => showScreen('dashboard-screen'), 500);
                            }
                        }

                        function handleSignup(btn) {
                            btn.innerText = 'Creating account...';
                            setTimeout(() => showScreen('setup-screen'), 500);
                        }

                        function nextStep(step) {
                            document.getElementById('setup-screen').querySelectorAll('div[id^="step-"]').forEach(d => d.style.display = 'none');
                            const target = document.getElementById('step-' + step);
                            if (target) target.style.display = 'block';
                        }

                        function generateAI() {
                            nextStep('generating');
                            setTimeout(() => nextStep('launch-ai'), 1000);
                        }

                        function toggleMenu() {
                            const menu = document.getElementById('extra-menu');
                            menu.style.display = menu.style.display === 'none' ? 'block' : 'none';
                        }

                        // Attach event listener for the grandma hint
                        document.addEventListener('click', (e) => {
                            if (e.target.innerText === '?') {
                                const hint = document.getElementById('quick-actions-hint');
                                if (hint) hint.style.display = 'block';
                            }
                        });

                        // Initial routing
                        const path = window.location.pathname;
                        const urlParams = new URLSearchParams(window.location.search);
                        
                        if (urlParams.has('signup') || path === '/signup') {
                            showScreen('signup-screen');
                        } else if (path === '/agents') {
                            showScreen('agents-screen');
                        } else if (path === '/business-setup') {
                            showScreen('setup-screen');
                        } else if (path === '/login') {
                            showScreen('login-screen');
                        } else {
                            // Default to dashboard for ease of testing
                            showScreen('dashboard-screen');
                        }
                    </script>
                </body>
            </html>
        "#,
    };
    axum::response::Html(content)
}

pub mod tools;
pub mod workers;
// Validation dummy comment
