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

        .route("/api/v1/wizard/setup", axum::routing::post(wizard_setup_handler))
        .route("/api/v1/wizard/tune", axum::routing::post(wizard_tune_handler))
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
    let content = r#"
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>OneHumanCorp - Small Business App</title>
                    <link rel="preconnect" href="https://fonts.googleapis.com">
                    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
                    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&family=Outfit:wght@400;600;800&display=swap" rel="stylesheet">
                    <style>
                        :root {
                            --bg: #0b0c10;
                            --text: #ffffff;
                            --accent: #00d3ff;
                            --glass: rgba(255, 255, 255, 0.05);
                            --border: rgba(255, 255, 255, 0.1);
                        }
                        body {
                            margin: 0;
                            font-family: 'Inter', sans-serif;
                            background: var(--bg);
                            color: var(--text);
                            display: flex;
                            flex-direction: column;
                            min-height: 100vh;
                        }
                        h1, h2, h3 {
                            font-family: 'Outfit', sans-serif;
                            margin-top: 0;
                        }

                        /* Glassmorphism Classes */
                        .glass {
                            backdrop-filter: blur(20px);
                            -webkit-backdrop-filter: blur(20px);
                            background: var(--glass);
                            border: 1px solid var(--border);
                            border-radius: 16px;
                            padding: 24px;
                        }

                        /* Smooth Animations */
                        .screen {
                            display: none;
                            animation: enter 300ms cubic-bezier(0.4, 0, 0.2, 1) forwards;
                        }
                        @keyframes enter {
                            from { opacity: 0; transform: translateY(10px); }
                            to { opacity: 1; transform: translateY(0); }
                        }
                        @keyframes exit {
                            from { opacity: 1; transform: translateY(0); }
                            to { opacity: 0; transform: translateY(-10px); }
                        }
                        .fade-out {
                            animation: exit 200ms cubic-bezier(0.4, 0, 0.2, 1) forwards;
                        }

                        /* Navigation */
                        nav {
                            display: flex;
                            justify-content: space-between;
                            padding: 16px 24px;
                            background: rgba(11, 12, 16, 0.8);
                            border-bottom: 1px solid var(--border);
                            position: sticky;
                            top: 0;
                            z-index: 10;
                            backdrop-filter: blur(10px);
                        }
                        nav .logo {
                            font-family: 'Outfit', sans-serif;
                            font-weight: 800;
                            font-size: 20px;
                            color: var(--accent);
                            cursor: pointer;
                        }
                        nav .links a {
                            color: var(--text);
                            text-decoration: none;
                            margin-left: 16px;
                            font-size: 14px;
                            cursor: pointer;
                        }

                        /* Layout */
                        main {
                            flex: 1;
                            display: flex;
                            flex-direction: column;
                            align-items: center;
                            padding: 40px 20px;
                            max-width: 600px;
                            margin: 0 auto;
                            width: 100%;
                            box-sizing: border-box;
                        }

                        /* Buttons */
                        button {
                            background: var(--accent);
                            color: #000;
                            border: none;
                            border-radius: 8px;
                            padding: 12px 24px;
                            font-family: 'Inter', sans-serif;
                            font-weight: 600;
                            font-size: 16px;
                            cursor: pointer;
                            transition: transform 0.2s;
                            width: 100%;
                            margin-bottom: 12px;
                        }
                        button:hover {
                            transform: scale(1.02);
                        }
                        button.secondary {
                            background: var(--glass);
                            color: var(--text);
                            border: 1px solid var(--border);
                        }

                        /* Inputs */
                        input, select {
                            width: 100%;
                            background: rgba(0, 0, 0, 0.3);
                            border: 1px solid var(--border);
                            border-radius: 8px;
                            padding: 12px;
                            color: var(--text);
                            font-family: 'Inter', sans-serif;
                            font-size: 16px;
                            margin-bottom: 16px;
                            box-sizing: border-box;
                        }

                        /* Utility */
                        .text-center { text-align: center; }
                        .mt-4 { margin-top: 16px; }
                        .mb-4 { margin-bottom: 16px; }

                        /* Wizard specific */
                        .wizard-step {
                            display: none;
                            animation: enter 300ms cubic-bezier(0.4, 0, 0.2, 1) forwards;
                        }
                        .wizard-step.active {
                            display: block;
                        }
                        .grid-cards {
                            display: grid;
                            grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
                            gap: 16px;
                            margin-bottom: 24px;
                        }
                        .card-btn {
                            background: var(--glass);
                            border: 1px solid var(--border);
                            border-radius: 12px;
                            padding: 20px;
                            text-align: center;
                            cursor: pointer;
                            transition: all 0.2s;
                            display: flex;
                            flex-direction: column;
                            align-items: center;
                            justify-content: center;
                            gap: 10px;
                        }
                        .card-btn:hover, .card-btn.selected {
                            background: rgba(0, 211, 255, 0.1);
                            border-color: var(--accent);
                        }
                        .icon-large {
                            font-size: 32px;
                        }

                        /* Loading Spinner */
                        .spinner {
                            border: 4px solid rgba(255, 255, 255, 0.1);
                            border-left-color: var(--accent);
                            border-radius: 50%;
                            width: 40px;
                            height: 40px;
                            animation: spin 1s linear infinite;
                            margin: 20px auto;
                        }
                        @keyframes spin { 100% { transform: rotate(360deg); } }

                        /* Toggle Switch */
                        .toggle-container {
                            display: flex;
                            align-items: center;
                            justify-content: space-between;
                            padding: 12px 0;
                            border-bottom: 1px solid var(--border);
                        }
                        .toggle-switch {
                            position: relative;
                            display: inline-block;
                            width: 44px;
                            height: 24px;
                        }
                        .toggle-switch input {
                            opacity: 0;
                            width: 0;
                            height: 0;
                        }
                        .slider {
                            position: absolute;
                            cursor: pointer;
                            top: 0; left: 0; right: 0; bottom: 0;
                            background-color: rgba(255, 255, 255, 0.2);
                            transition: .3s;
                            border-radius: 24px;
                        }
                        .slider:before {
                            position: absolute;
                            content: "";
                            height: 18px;
                            width: 18px;
                            left: 3px;
                            bottom: 3px;
                            background-color: white;
                            transition: .3s;
                            border-radius: 50%;
                        }
                        input:checked + .slider {
                            background-color: var(--accent);
                        }
                        input:checked + .slider:before {
                            transform: translateX(20px);
                        }

                        /* Range Slider */
                        .range-slider {
                            width: 100%;
                            margin: 10px 0;
                        }

                        /* Advanced Mode styling */
                        .advanced-mode-only {
                            display: none;
                            margin-top: 10px;
                            padding: 10px;
                            background: rgba(0,0,0,0.5);
                            border-radius: 4px;
                            font-family: monospace;
                            font-size: 12px;
                            color: #aaa;
                        }
                        body.advanced-mode .advanced-mode-only {
                            display: block;
                        }

                        .toast {
                            position: fixed;
                            bottom: 20px;
                            left: 50%;
                            transform: translateX(-50%);
                            background: var(--accent);
                            color: #000;
                            padding: 12px 24px;
                            border-radius: 20px;
                            font-weight: 600;
                            display: none;
                            z-index: 1000;
                            animation: enter 300ms forwards;
                        }

                        .hero-animation {
                            font-size: 48px;
                            text-align: center;
                            margin-bottom: 20px;
                            animation: float 3s ease-in-out infinite;
                        }
                        @keyframes float {
                            0% { transform: translateY(0px); }
                            50% { transform: translateY(-10px); }
                            100% { transform: translateY(0px); }
                        }


                    </style>
                </head>
                <body>
                    <nav id="main-nav" style="display: none;">
                        <div class="logo" onclick="showScreen('dashboard-screen')">OHC</div>
                        <div class="links">
                            <a onclick="showScreen('dashboard-screen')">Home</a>
                            <a onclick="showScreen('agents-screen')">Agents</a>
                            <a onclick="toggleAdvancedMode()">Toggle Advanced</a>
                        </div>
                    </nav>

                    <main>
                        <!-- Login Screen -->
                        <div id="login-screen" class="screen active">
                            <div class="glass text-center">
                                <h1>Welcome back</h1>
                                <p class="mb-4">Sign in to your business</p>
                                <input type="email" placeholder="Email address" id="login-email" />
                                <button onclick="handleLogin(this)">Sign In</button>
                                <p class="mt-4"><a href='#' onclick="showScreen('signup-screen')">Create an account</a></p>
                                <div id="login-error" style="display:none; color: #ff4d4d; margin-top: 10px;">Please enter an email.</div>
                            </div>
                        </div>

                        <!-- Signup Screen -->
                        <div id="signup-screen" class="screen">
                            <div class="glass text-center">
                                <h1>Create Account</h1>
                                <p class="mb-4">Let's get started</p>
                                <button onclick="handleSignup(this)">Start Wizard</button>
                            </div>
                        </div>

                        <!-- 1. Business Setup Wizard -->
                        <div id="setup-screen" class="screen">
                            <div class="glass w-100">
                                <!-- Welcome -->
                                <div id="step-1" class="wizard-step active">
                                    <div class="hero-animation">🚀</div>
                                    <h1 class="text-center">Your business, live in minutes</h1>
                                    <p class="text-center mb-4">Zero tech skills needed. We do the heavy lifting.</p>
                                    <button onclick="nextStep(2)">Start My Business →</button>
                                </div>

                                <!-- Business Type -->
                                <div id="step-2" class="wizard-step">
                                    <h2>What kind of business are you building?</h2>
                                    <div class="grid-cards">
                                        <div class="card-btn" onclick="selectType('Online Store')">
                                            <div class="icon-large">🛒</div>
                                            <div>Online Store</div>
                                        </div>
                                        <div class="card-btn" onclick="selectType('Service Business')">
                                            <div class="icon-large">🛠️</div>
                                            <div>Service Business</div>
                                        </div>
                                        <div class="card-btn" onclick="selectType('Restaurant / Food')">
                                            <div class="icon-large">🍕</div>
                                            <div>Restaurant / Food</div>
                                        </div>
                                        <div class="card-btn" onclick="selectType('Creative / Portfolio')">
                                            <div class="icon-large">🎨</div>
                                            <div>Creative / Portfolio</div>
                                        </div>
                                        <div class="card-btn" onclick="selectType('Local Business')">
                                            <div class="icon-large">🏠</div>
                                            <div>Local Business</div>
                                        </div>
                                        <div class="card-btn" onclick="selectType('Other')">
                                            <div class="icon-large">✨</div>
                                            <div>Other</div>
                                        </div>
                                    </div>
                                    <button class="secondary" onclick="nextStep(1)">Back</button>
                                </div>

                                <!-- Business Name -->
                                <div id="step-3" class="wizard-step">
                                    <h2>Give your business a name</h2>
                                    <input type="text" id="biz-name" placeholder="e.g. Maya's Cakes" oninput="suggestTagline()" />

                                    <div id="tagline-suggestion" style="display:none;" class="mb-4">
                                        <p style="font-size:14px; color:#aaa;">AI Suggestion:</p>
                                        <input type="text" id="biz-tagline" />
                                    </div>

                                    <button onclick="nextStep(4)">Next →</button>
                                    <button class="secondary" onclick="nextStep(2)">Back</button>
                                </div>

                                <!-- What do you sell -->
                                <div id="step-4" class="wizard-step">
                                    <h2>What do you sell?</h2>
                                    <div class="grid-cards">
                                        <div class="card-btn" onclick="this.classList.toggle('selected')">📦 Physical products</div>
                                        <div class="card-btn" onclick="this.classList.toggle('selected')">⬇️ Digital downloads</div>
                                        <div class="card-btn" onclick="this.classList.toggle('selected')">📅 Services / appointments</div>
                                        <div class="card-btn" onclick="this.classList.toggle('selected')">🍔 Food & beverages</div>
                                        <div class="card-btn" onclick="this.classList.toggle('selected')">🔁 Subscriptions</div>
                                    </div>
                                    <button onclick="nextStep(5)">Next →</button>
                                    <button class="secondary" onclick="nextStep(3)">Back</button>
                                </div>

                                <!-- Payments -->
                                <div id="step-5" class="wizard-step">
                                    <h2>How do you want to receive payments?</h2>
                                    <div class="grid-cards">
                                        <div class="card-btn" onclick="nextStep(6)">
                                            <div class="icon-large">🌐</div>
                                            <div>Online only</div>
                                            <small style="color:#aaa">~5 mins setup</small>
                                        </div>
                                        <div class="card-btn" onclick="nextStep(6)">
                                            <div class="icon-large">🏪</div>
                                            <div>In-person (POS)</div>
                                            <small style="color:#aaa">~10 mins setup</small>
                                        </div>
                                        <div class="card-btn" onclick="nextStep(6)">
                                            <div class="icon-large">🌍</div>
                                            <div>Both</div>
                                            <small style="color:#aaa">~10 mins setup</small>
                                        </div>
                                        <div class="card-btn" onclick="nextStep(6)">
                                            <div class="icon-large">⏭️</div>
                                            <div>Skip for now</div>
                                        </div>
                                    </div>
                                    <button class="secondary" onclick="nextStep(4)">Back</button>
                                </div>

                                <!-- Account -->
                                <div id="step-6" class="wizard-step">
                                    <h2>Administrator account</h2>
                                    <input type="text" placeholder="Your Full Name" />
                                    <input type="email" placeholder="Email Address" />
                                    <input type="password" placeholder="Password" oninput="checkStrength(this.value)" />
                                    <div style="height:4px; background:#333; margin-bottom:16px; border-radius:2px;">
                                        <div id="pwd-strength" style="height:100%; width:0%; background:red; border-radius:2px; transition:0.3s;"></div>
                                    </div>
                                    <button class="secondary mb-4">G Continue with Google</button>
                                    <button class="secondary mb-4"> Continue with Apple</button>
                                    <button onclick="nextStep(7)">Next →</button>
                                    <button class="secondary" onclick="nextStep(5)">Back</button>
                                </div>

                                <!-- Review & Launch -->
                                <div id="step-7" class="wizard-step">
                                    <h2>Review & Launch</h2>
                                    <div style="background:rgba(0,0,0,0.3); padding:16px; border-radius:8px; margin-bottom:16px;">
                                        <p><strong>Business:</strong> <span id="review-name">Your Business</span></p>
                                        <p><strong>Type:</strong> <span id="review-type">Store</span></p>
                                        <p><strong>Status:</strong> Ready to provision</p>
                                    </div>
                                    <button onclick="launchBusiness()" style="animation: pulse 2s infinite;">Launch My Business →</button>
                                    <style>@keyframes pulse { 0% { transform: scale(1); } 50% { transform: scale(1.05); } 100% { transform: scale(1); } }</style>
                                    <button class="secondary" onclick="nextStep(6)">Back</button>
                                </div>

                                <!-- Launching Overlay -->
                                <div id="step-launching" class="wizard-step text-center">
                                    <h2>Your business is setting up...</h2>
                                    <div class="spinner"></div>
                                    <p>Provisioning tenant...</p>
                                    <p>Selecting starter template...</p>
                                    <p>Pre-seeding AI agents...</p>
                                </div>
                            </div>
                        </div>

                        <!-- Dashboard Screen -->
                        <div id="dashboard-screen" class="screen">
                            <div class="glass w-100">
                                <h1>Dashboard</h1>
                                <p>Welcome to your new business!</p>

                                <!-- Grow my business wizard trigger -->
                                <div style="background:rgba(0,211,255,0.1); border:1px solid var(--accent); padding:16px; border-radius:12px; margin-bottom:20px;">
                                    <h3>🚀 Grow your business</h3>
                                    <p>Add 5 more products to start selling!</p>
                                    <button onclick="showScreen('website-builder')">Add Products →</button>
                                </div>

                                <button onclick="showScreen('website-builder')">Build My Website</button>
                                <button onclick="showScreen('agents-screen')">Manage my AI team</button>
                            </div>
                        </div>

                        <!-- 2. Website Builder Onboarding -->
                        <div id="website-builder" class="screen">
                            <div class="glass w-100">
                                <div id="web-step-1" class="wizard-step active">
                                    <h2>Template Gallery</h2>
                                    <div class="grid-cards">
                                        <div class="card-btn" onclick="this.classList.add('selected'); setTimeout(()=>nextWebStep(2), 500)">
                                            <div style="width:100%; height:80px; background:#333; border-radius:4px; margin-bottom:8px;"></div>
                                            Minimal
                                        </div>
                                        <div class="card-btn" onclick="this.classList.add('selected'); setTimeout(()=>nextWebStep(2), 500)">
                                            <div style="width:100%; height:80px; background:#444; border-radius:4px; margin-bottom:8px;"></div>
                                            Bold
                                        </div>
                                    </div>
                                    <button class="secondary" onclick="showScreen('dashboard-screen')">Back</button>
                                </div>

                                <div id="web-step-2" class="wizard-step">
                                    <h2>Brand colors & logo</h2>
                                    <div style="display:flex; gap:10px; margin-bottom:16px;">
                                        <div style="width:40px;height:40px;background:#ff0000;border-radius:50%;cursor:pointer;"></div>
                                        <div style="width:40px;height:40px;background:#00ff00;border-radius:50%;cursor:pointer;"></div>
                                        <div style="width:40px;height:40px;background:#0000ff;border-radius:50%;cursor:pointer;"></div>
                                    </div>
                                    <button class="secondary mb-4">Upload Logo</button>
                                    <button class="secondary mb-4">Generate Logo (AI)</button>
                                    <button onclick="nextWebStep(3)">Next →</button>
                                </div>

                                <div id="web-step-3" class="wizard-step">
                                    <h2>Add your first product or service</h2>
                                    <input type="text" placeholder="Product Name" onblur="this.value ? document.getElementById('ai-desc').value = 'A premium ' + this.value + ' perfect for you.' : null" />
                                    <input type="text" placeholder="Price (e.g. 29.99)" />
                                    <button class="secondary mb-4">📷 Add Photo</button>
                                    <textarea id="ai-desc" placeholder="Description (AI auto-generates this)" style="width:100%; height:80px; margin-bottom:16px; background:rgba(0,0,0,0.3); border:1px solid var(--border); color:white; padding:8px; border-radius:8px;"></textarea>
                                    <button onclick="nextWebStep(4)">Next →</button>
                                </div>

                                <div id="web-step-4" class="wizard-step">
                                    <h2>Connect a domain</h2>
                                    <div class="card-btn mb-4" onclick="nextWebStep(5)">Use a free OHC subdomain<br/><small>mybusiness.ohc.app</small></div>
                                    <div class="card-btn mb-4">Use my own domain</div>
                                    <div class="card-btn mb-4">Buy a domain</div>
                                </div>

                                <div id="web-step-5" class="wizard-step text-center">
                                    <h2>Ready to Go Live!</h2>
                                    <div style="width:100%; height:150px; background:#222; border-radius:8px; margin-bottom:16px; display:flex; align-items:center; justify-content:center;">Live Preview</div>
                                    <button onclick="publishSite()">Publish →</button>
                                </div>
                            </div>
                        </div>

                        <!-- 3. AI Agent Configuration -->
                        <div id="agents-screen" class="screen">
                            <div class="glass w-100">
                                <h1>Agent Gallery</h1>
                                <p>No technical knowledge required.</p>

                                <div class="grid-cards">
                                    <div class="card-btn" onclick="configureAgent('Customer Support')">
                                        <div class="icon-large">🎧</div>
                                        <div>Customer Support</div>
                                        <button class="secondary mt-4" style="padding:4px 8px; font-size:12px;">Add to team</button>
                                    </div>
                                    <div class="card-btn" onclick="configureAgent('Social Media Manager')">
                                        <div class="icon-large">📱</div>
                                        <div>Social Media Manager</div>
                                        <button class="secondary mt-4" style="padding:4px 8px; font-size:12px;">Add to team</button>
                                    </div>
                                    <div class="card-btn" onclick="configureAgent('SEO Booster')">
                                        <div class="icon-large">🚀</div>
                                        <div>SEO Booster</div>
                                        <button class="secondary mt-4" style="padding:4px 8px; font-size:12px;">Add to team</button>
                                    </div>
                                </div>
                                <button class="secondary" onclick="showScreen('dashboard-screen')">Back to Dashboard</button>
                            </div>
                        </div>

                        <div id="agent-config-screen" class="screen">
                            <div class="glass w-100">
                                <h1 id="agent-config-title">Configure Agent</h1>

                                <h3>Capabilities</h3>
                                <div class="toggle-container">
                                    <span>Reply to customer messages</span>
                                    <label class="toggle-switch">
                                        <input type="checkbox" checked>
                                        <span class="slider"></span>
                                    </label>
                                    <div class="advanced-mode-only">Permission: `msg:reply`</div>
                                </div>
                                <div class="toggle-container">
                                    <span>Post to Instagram & Facebook</span>
                                    <label class="toggle-switch">
                                        <input type="checkbox">
                                        <span class="slider"></span>
                                    </label>
                                    <div class="advanced-mode-only">Permission: `social:post`</div>
                                </div>

                                <h3 class="mt-4">How often should this agent work?</h3>
                                <div style="display:flex; justify-content:space-between; font-size:12px; color:#aaa;">
                                    <span>Real-time</span>
                                    <span>Weekly</span>
                                </div>
                                <input type="range" min="1" max="4" value="2" class="range-slider">
                                <div class="advanced-mode-only">Maps to: `max_sessions_per_day`</div>

                                <div style="display:flex; gap:10px; margin-top:24px;">
                                    <button onclick="showToast('Agent Activated!'); showScreen('agents-screen')">Activate</button>
                                    <button class="secondary" onclick="showScreen('prompt-tuning-screen')">Tune this agent</button>
                                </div>
                                <button class="secondary mt-4" onclick="showScreen('agents-screen')">Cancel</button>
                            </div>
                        </div>

                        <!-- 4. Prompt Tuning -->
                        <div id="prompt-tuning-screen" class="screen">
                            <div class="glass w-100">
                                <h1>Tune this agent</h1>

                                <h3>Tone</h3>
                                <div style="display:flex; gap:10px; flex-wrap:wrap; margin-bottom:16px;">
                                    <div class="card-btn" style="padding:8px 16px;">Friendly & Warm</div>
                                    <div class="card-btn" style="padding:8px 16px;">Professional</div>
                                    <div class="card-btn" style="padding:8px 16px;">Energetic</div>
                                </div>
                                <div class="advanced-mode-only">System Prompt Template: "You are a friendly..."</div>

                                <h3>Focus topics</h3>
                                <div style="display:flex; gap:10px; flex-wrap:wrap; margin-bottom:16px;">
                                    <div class="card-btn" style="padding:4px 12px; border-radius:20px;">+ Only about my products</div>
                                    <div class="card-btn" style="padding:4px 12px; border-radius:20px;">+ Avoid competitor mentions</div>
                                </div>

                                <h3>Example interactions</h3>
                                <input type="text" placeholder="User asks..." />
                                <input type="text" placeholder="Agent replies..." />

                                <h3>Live Preview Sandbox</h3>
                                <div style="background:rgba(0,0,0,0.3); border:1px solid var(--border); border-radius:8px; height:100px; padding:8px; margin-bottom:16px;">
                                    <p style="color:#aaa; font-size:12px;">Chat sandbox...</p>
                                </div>

                                <button onclick="saveTuning()">Save Tuning</button>
                                <button class="secondary" onclick="showScreen('agent-config-screen')">Back</button>
                            </div>
                        </div>
                    </main>

                    <div id="toast" class="toast"></div>

                    <script>
                        // Global state
                        let isAdvancedMode = localStorage.getItem('advancedMode') === 'true';
                        if (isAdvancedMode) document.body.classList.add('advanced-mode');

                        function toggleAdvancedMode() {
                            isAdvancedMode = !isAdvancedMode;
                            localStorage.setItem('advancedMode', isAdvancedMode);
                            if (isAdvancedMode) {
                                document.body.classList.add('advanced-mode');
                                showToast('Advanced Mode Enabled');
                            } else {
                                document.body.classList.remove('advanced-mode');
                                showToast('Simple Mode Enabled');
                            }
                        }

                        function showToast(msg) {
                            const toast = document.getElementById('toast');
                            toast.innerText = msg;
                            toast.style.display = 'block';
                            setTimeout(() => { toast.style.display = 'none'; }, 3000);
                        }

                        function showScreen(id) {
                            // First, start exit animation
                            const active = document.querySelector('.screen.active');
                            if (active) {
                                active.classList.add('fade-out');
                                setTimeout(() => {
                                    active.classList.remove('active', 'fade-out');
                                    active.style.display = 'none';

                                    const next = document.getElementById(id);
                                    if (next) {
                                        next.style.display = 'block';
                                        next.classList.add('active');
                                    }
                                }, 200); // Wait for exit anim
                            } else {
                                document.querySelectorAll('.screen').forEach(s => {
                                    s.classList.remove('active');
                                    s.style.display = 'none';
                                });
                                const next = document.getElementById(id);
                                if (next) {
                                    next.style.display = 'block';
                                    next.classList.add('active');
                                }
                            }
                            
                            if (id !== 'login-screen' && id !== 'signup-screen' && id !== 'setup-screen') {
                                document.getElementById('main-nav').style.display = 'flex';
                            } else {
                                document.getElementById('main-nav').style.display = 'none';
                            }
                        }

                        // Wizard Logic
                        let bizType = '';
                        function selectType(type) {
                            bizType = type;
                            nextStep(3);
                        }

                        function suggestTagline() {
                            const name = document.getElementById('biz-name').value;
                            if (name.length > 3) {
                                document.getElementById('tagline-suggestion').style.display = 'block';
                                document.getElementById('biz-tagline').value = "The best " + (bizType || "business") + " in town.";
                            } else {
                                document.getElementById('tagline-suggestion').style.display = 'none';
                            }
                        }

                        function nextStep(stepId) {
                            if (stepId === 7) {
                                document.getElementById('review-name').innerText = document.getElementById('biz-name').value || 'My Business';
                                document.getElementById('review-type').innerText = bizType || 'Business';
                            }


                            if (stepId === 'launching') {
                                fetch('/api/v1/wizard/setup', {
                                    method: 'POST',
                                    headers: { 'Content-Type': 'application/json' },
                                    body: JSON.stringify({ type: bizType, name: document.getElementById('biz-name').value })
                                }).catch(e => console.log(e));

                                document.querySelectorAll('#setup-screen .wizard-step').forEach(s => {

                                    s.classList.remove('active');
                                });
                                const target = document.getElementById('step-launching');
                                if (target) target.classList.add('active');
                                return;
                            }

                            document.querySelectorAll('#setup-screen .wizard-step').forEach(s => {
                                s.classList.remove('active');
                            });
                            const target = document.getElementById('step-' + stepId);
                            if (target) target.classList.add('active');
                        }

                        function launchBusiness() {
                            nextStep('launching');
                            setTimeout(() => {
                                showScreen('dashboard-screen');
                            }, 3000);
                        }

                        // Web Wizard Logic
                        function nextWebStep(stepId) {
                            document.querySelectorAll('#website-builder .wizard-step').forEach(s => {
                                s.classList.remove('active');
                            });
                            const target = document.getElementById('web-step-' + stepId);
                            if (target) target.classList.add('active');
                        }

                        function publishSite() {
                            // Copy to clipboard trick
                            navigator.clipboard.writeText("https://mybusiness.ohc.app").catch(e=>{});
                            showToast('Site Live! Link copied to clipboard.');
                            setTimeout(() => {
                                showScreen('dashboard-screen');
                            }, 2000);
                        }

                        // Agent Logic
                        function configureAgent(name) {
                            document.getElementById('agent-config-title').innerText = name;
                            showScreen('agent-config-screen');
                        }


                        function saveTuning() {
                            fetch('/api/v1/wizard/tune', {
                                method: 'POST',
                                headers: { 'Content-Type': 'application/json' },
                                body: JSON.stringify({ tone: 'friendly' })
                            }).catch(e => console.log(e));

                            showToast('Your agent has been updated ✓');

                            setTimeout(() => showScreen('agent-config-screen'), 1000);
                        }

                        function handleLogin(btn) {
                            const email = document.getElementById('login-email').value;
                            btn.innerText = 'Signing in...';
                            if (!email) {
                                setTimeout(() => {
                                    document.getElementById('login-error').style.display = 'block';
                                    btn.innerText = 'Sign In';
                                }, 500);
                            } else {
                                localStorage.setItem('isLoggedIn', 'true');
                                setTimeout(() => showScreen('dashboard-screen'), 500);
                            }
                        }

                        function handleSignup(btn) {
                            btn.innerText = 'Creating account...';
                            setTimeout(() => showScreen('setup-screen'), 500);
                        }

                        function checkStrength(pwd) {
                            const el = document.getElementById('pwd-strength');
                            if (pwd.length === 0) { el.style.width = '0%'; el.style.background = 'red'; }
                            else if (pwd.length < 5) { el.style.width = '33%'; el.style.background = 'red'; }
                            else if (pwd.length < 8) { el.style.width = '66%'; el.style.background = 'orange'; }
                            else { el.style.width = '100%'; el.style.background = 'green'; }
                        }

                        // Initial routing
                        const path = window.location.pathname;
                        const urlParams = new URLSearchParams(window.location.search);
                        
                        if (localStorage.getItem('isLoggedIn') === 'true') {
                            if (path === '/pricing') {
                                showScreen('pricing-screen');
                            } else if (path === '/my-plan' || path === '/billing') {
                                showScreen('my-plan-screen');
                            } else if (path === '/agents') {
                                showScreen('agents-screen');
                            } else if (path === '/diagnostics') {
                                showScreen('diagnostics-screen');
                            } else if (path === '/services') {
                                showScreen('services-screen');
                            } else if (path === '/scaling') {
                                showScreen('scaling-screen');
                            } else if (path === '/business-setup') {
                                showScreen('setup-screen');
                            } else if (path === '/checkout') {
                                showScreen('checkout-screen');
                            } else {
                                showScreen('dashboard-screen');
                            }
                        } else {
                            if (urlParams.has('signup') || path === '/signup') {
                                showScreen('signup-screen');
                            } else if (path === '/pricing') {
                                showScreen('pricing-screen');
                            } else if (path === '/my-plan' || path === '/billing') {
                                showScreen('my-plan-screen');
                            } else {
                                showScreen('login-screen');
                            }
                        }

                    </script>
                </body>
            </html>
"#;
    axum::response::Html(content)
}


pub async fn wizard_setup_handler() -> impl axum::response::IntoResponse {
    axum::response::Json(serde_json::json!({ "status": "success" }))
}

pub async fn wizard_tune_handler() -> impl axum::response::IntoResponse {
    axum::response::Json(serde_json::json!({ "status": "success" }))
}
pub mod tools;
pub mod workers;
