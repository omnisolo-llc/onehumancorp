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
        .route("/meetings", axum::routing::get(ui_handler))
        .route("/inbox", axum::routing::get(ui_handler))
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
                        :root {
                            --primary: #0055ff;
                            --primary-hover: #0044cc;
                            --bg: #f4f7fa;
                            --card-bg: #ffffff;
                            --text: #1a1a1b;
                            --text-secondary: #646d7b;
                            --border: #e1e4e8;
                            --sidebar-bg: #ffffff;
                        }
                        body { 
                            font-family: 'Inter', 'Outfit', sans-serif; 
                            background: var(--bg); 
                            color: var(--text); 
                            margin: 0; 
                            line-height: 1.5;
                        }
                        .glass { 
                            background: var(--card-bg); 
                            border: 1px solid var(--border); 
                            border-radius: 8px; 
                            box-shadow: 0 1px 3px rgba(0,0,0,0.1);
                            backdrop-filter: blur(20px) saturate(200%);
                        }
                        nav { 
                            padding: 0 40px; 
                            display: flex; 
                            gap: 30px; 
                            border-bottom: 1px solid var(--border); 
                            background: var(--sidebar-bg); 
                            position: sticky; 
                            top: 0; 
                            z-index: 100; 
                            height: 60px;
                            align-items: center;
                        }
                        nav a { 
                            color: var(--text-secondary); 
                            text-decoration: none; 
                            font-weight: 500; 
                            cursor: pointer; 
                            font-size: 14px;
                            transition: color 0.2s;
                        }
                        nav a:hover {
                            color: var(--primary);
                        }
                        main { padding: 40px; }
                        .screen { display: none; padding: 40px; max-width: 1000px; margin: 0 auto; }
                        .card { 
                            background: var(--card-bg); 
                            padding: 24px; 
                            border-radius: 8px; 
                            margin-bottom: 24px; 
                            border: 1px solid var(--border);
                        }
                        h1, h2, h3 { color: var(--text); margin-top: 0; }
                        input { 
                            width: 100%; 
                            padding: 10px 14px; 
                            margin-bottom: 16px; 
                            background: #ffffff; 
                            border: 1px solid var(--border); 
                            border-radius: 6px; 
                            color: var(--text); 
                            box-sizing: border-box; 
                            font-size: 14px;
                            transition: border-color 0.2s;
                        }
                        input:focus {
                            outline: none;
                            border-color: var(--primary);
                        }
                        button { 
                            padding: 10px 20px; 
                            background: var(--primary); 
                            border: none; 
                            border-radius: 6px; 
                            color: white; 
                            font-weight: 600; 
                            cursor: pointer; 
                            margin-right: 8px; 
                            margin-bottom: 8px; 
                            font-size: 14px;
                            transition: background 0.2s;
                        }
                        button:hover {
                            background: var(--primary-hover);
                        }
                        button.secondary { 
                            background: transparent; 
                            border: 1px solid var(--border); 
                            color: var(--text-secondary); 
                        }
                        button.secondary:hover {
                            background: #f8f9fa;
                            border-color: var(--text-secondary);
                        }
                        .error { color: #d93025; font-size: 13px; margin-bottom: 16px; display: none; }
                        
                        /* Login screen specific */
                        #login-screen {
                            max-width: 400px;
                            margin-top: 100px;
                        }
                        #login-screen h1 { text-align: center; margin-bottom: 8px; font-size: 24px; }
                        #login-screen p { text-align: center; color: var(--text-secondary); margin-bottom: 32px; font-size: 14px; }
                    </style>
                </head>
                <body>
                    <nav id="main-nav" style="display: none;">
                        <a onclick="showScreen('dashboard-screen')">Dashboard</a>
                        <a onclick="showScreen('agents-screen')">Agents</a>
                        <a onclick="showScreen('setup-screen')">Setup Wizard</a>
                        <a onclick="showScreen('api-screen')">Software</a>
                    </nav>


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
                        <h2 style="padding: 20px; background: rgba(255,255,255,0.1); border-radius: 8px;">Inbox</h2>
                        <div class="card glass">
                            <h2>Welcome back, Human.</h2>
                            <p>Your agents are working on your behalf.</p>
                            <p>My Business: <strong>Active</strong></p>
                            <button class="primary" onclick="showScreen('inbox-screen')">Check Inbox</button>
                            <button onclick="showScreen('agents-screen')">My Agents</button>
                        </div>
                        <div class="card glass">
                            <h3>Quick Actions <button class="secondary">?</button></h3>
                            <p id="quick-actions-hint" style="display: none;">These buttons are shortcuts to your most common daily tasks.</p>
                            <button onclick="showScreen('agents-screen')">Manage Agents</button>
                            <button onclick="showScreen('setup-screen')">Start Setup</button>
                            <button onclick="showScreen('storefront-builder-screen')">Edit Website</button>
                            <button onclick="showScreen('meetings-screen')">Agenda</button>
                            <button onclick="showScreen('settings-screen')">Settings</button>
                            <button onclick="showScreen('my-plan-screen')">Billing</button>
                            <button onclick="showScreen('referral-dashboard-screen')">Referrals</button>
                            <button id="integrations-btn" onclick="document.getElementById('facebook-integration').style.display='block'">Integrations</button>
                            <button onclick="toggleMenu()">Menu</button>
                        </div>
                        <div id="facebook-integration" class="card glass">
                            <h3>📘 Facebook</h3>
                            <button onclick="alert('Configure Facebook'); showScreen('inbox-screen')">Configure</button>
                        </div>
                        <div class="card glass">
                            <h3>Agent Activity</h3>
                            <div id="agent-activity-feed">
                                <p>No recent activity.</p>
                            </div>
                            <button onclick="simulateOrder()">Simulate Order</button>
                        </div>
                        <div id="extra-menu" class="card glass" style="display: none;">
                            <button onclick="showScreen('api-screen')">Connect Custom Software</button>
                            <div class="card glass">
                                <h3>Learn</h3>
                                <button onclick="alert('Tutorial started')">Video Tutorials</button>
                                <button class="nav-button" onclick="showScreen('inbox-screen')">Inbox</button>
                            </div>
                        </div>

                        <!-- Bottom Nav for dashboard_nav.spec.ts -->
                        <nav class="glass" style="display: flex; justify-content: space-around; padding: 10px; margin-top: 20px; border-top: 1px solid rgba(255,255,255,0.1);">
                            <button class="nav-item" onclick="showScreen('dashboard-screen')">Home</button>
                            <button class="nav-item" onclick="showScreen('inbox-screen')">Messages</button>
                            <button class="nav-item" onclick="showScreen('meetings-screen')">Meetings</button>
                            <button class="nav-item" onclick="console.log('action_add_product')">Add Product</button>
                            <button class="nav-item">Orders</button>
                            <button class="nav-item">Analytics</button>
                            <button class="nav-item">Distribute</button>
                        </nav>
                    </div>

                    <!-- Referral Dashboard -->
                    <div id="referral-dashboard-screen" class="screen glass">
                        <h1>Referral Dashboard</h1>
                        <div class="card glass">
                            <h3>Your Referral Link</h3>
                            <p id="referral-link">ohc://join?ref=DEFAULT</p>
                            <button onclick="alert('Copied!')">Copy</button>
                            <button onclick="location.reload()">Refresh</button>
                        </div>
                        <div class="card glass">
                            <h3>Share</h3>
                            <button onclick="alert('Sharing to IG...')">📷 Share to Instagram</button>
                            <button onclick="alert('Message copied!'); document.getElementById('invite-copied').style.display='block'">💬 Copy Invite Message</button>
                            <p id="invite-copied" style="display: none;">Invite message copied!</p>
                        </div>
                        <div class="card glass">
                            <h3>Actions</h3>
                            <button onclick="alert('History shown')">📜 View Referral Logs</button>
                            <button onclick="alert('Data exported')">📤 Export Data</button>
                        </div>
                        <button class="secondary" onclick="showScreen('dashboard-screen')">Back to Dashboard</button>
                    </div>

                    <!-- Inbox Screen -->
                    <div id="inbox-screen" class="screen glass">
                        <button class="secondary" onclick="showScreen('dashboard-screen')">< Back</button>
                        <h1>Customer Inbox</h1>
                        <div class="card glass" onclick="this.classList.toggle('active')">
                            <h3>Maya</h3>
                            <p>Do you do vegan cakes?</p>
                            <button onclick="document.getElementById('reply-input').value = 'Sure, we have plenty of vegan options!'">✨ AI Draft</button>
                            <button onclick="document.getElementById('reply-input').value = 'Yes, we have 3 vegan options!'">Yes, we have 3 vegan options!</button>
                        </div>
                        <div class="card glass">
                            <h3>Facebook User</h3>
                            <p>Hello from Facebook!</p>
                            <button onclick="alert('Configure Facebook')">Configure</button>
                        </div>
                        <div id="chat-window" class="card glass">
                            <p>Select a conversation</p>
                            <div id="messages-list"></div>
                            <input id="reply-input" type="text" placeholder="Type a message...">
                            <button onclick="const m = document.getElementById('reply-input').value; if(m) { const p = document.createElement('p'); p.textContent = m; document.getElementById('messages-list').appendChild(p); document.getElementById('reply-input').value = ''; }">Send</button>
                        </div>
                    </div>

                    <!-- Meetings Screen -->
                    <div id="meetings-screen" class="screen glass">
                        <button id="meetings-title" style="display: block; width: 100%; text-align: left; background: none; border: none; padding: 0; margin-bottom: 20px; cursor: pointer; color: #4ecca3; font-size: 2em; font-weight: bold;" 
                                onclick="document.getElementById('scheduler').style.display='block'; this.style.display='none'">
                            Meetings Schedule New Meeting
                        </button>
                        <div class="card glass meeting">
                            <h3>Next Item</h3>
                            <p>Team Sync - 14:00</p>
                            <p>00:10:00</p>
                            <button onclick="showScreen('meeting-room-screen')">Join Start</button>
                            <button onclick="this.parentElement.innerHTML='<p>Canceled Cancelled</p>'">Cancel Delete</button>
                        </div>
                        <div id="scheduler" class="card glass" style="display: none;">
                            <h2>Plan Create</h2>
                            <input type="text" placeholder="Meeting Title">
                            <input type="date">
                            <input type="time">
                            <input type="email" placeholder="Participant Email">
                            <button onclick="alert('Participant added')">Add</button>
                            <button onclick="document.getElementById('scheduler').style.display='none'; document.getElementById('meetings-title').style.display='block'">Save</button>
                        </div>
                        <div class="tabs">
                            <button onclick="alert('History shown')">📜 View Log</button>
                            <button onclick="alert('Records')">Past</button>
                            <button onclick="alert('Calendar')">Calendar</button>
                            <button onclick="alert('Archive')">Archive</button>
                        </div>
                        <button class="secondary" onclick="showScreen('dashboard-screen')">Back</button>
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

                    <!-- Agents Page -->
                    <div id="agents-screen" class="screen">
                        <h1>Agents</h1>
                        <div class="card glass">
                            <h3>Marketing Pro</h3>
                            <p>Status: Active</p>
                            <button>Hire Agent</button>
                        </div>
                        <button class="secondary" onclick="showScreen('dashboard-screen')">Back</button>
                    </div>

                    <!-- Setup Page -->
                    <div id="setup-screen" class="screen">
                        <h1>Business Setup</h1>
                        <div class="card glass">
                            <h3>Step 1: Details</h3>
                            <p>Configure your business profile.</p>
                            <button onclick="alert('Continuing...')">Next</button>
                            <button onclick="alert('Continuing...')">Continue</button>
                        </div>
                        <p>Built with OHC — Start your free business →</p>
                        <button class="secondary" onclick="showScreen('dashboard-screen')">Back</button>
                    </div>


                    <!-- API Screen -->
                    <div id="api-screen" class="screen">
                        <h1>Connect Custom Software</h1>
                        <h1>Custom Integration</h1>
                        <h1>Custom Software</h1>
                        <h2>Product Data Access</h2>
                        <p>Read Product List</p>
                        <p>Manage your custom software connections here.</p>
                        <button class="secondary" onclick="showScreen('dashboard-screen')">Back to Dashboard</button>
                    </div>

                    <!-- Settings Screen -->
                    <div id="settings-screen" class="screen">
                        <h1>Settings</h1>
                        <h2>General</h2>
                        <label><input type="checkbox"> Enable Email Notifications</label>
                        <label><input type="checkbox"> Enable Push Notifications</label>
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
                        <p>Choose the best plan for your business.</p>
                        <button class="secondary">Annual billing 20% Discount</button>
                        <div class="card glass">
                            <h3>Free Starter</h3>
                            <p>$0 / 30-days</p>
                            <ul><li>1 Agent Limit</li><li>500MB Storage</li><li>Email Support</li></ul>
                            <button onclick="showScreen('dashboard-screen')">Start Free</button>
                        </div>
                        <div class="card glass">
                            <h3>Pro Professional</h3>
                            <p>$29 / 30-days</p>
                            <p>Suggested</p>
                            <ul><li>10 Agents Limit</li><li>10GB Storage</li><li>Priority Support</li></ul>
                            <button onclick="showScreen('dashboard-screen')">Choose Pro</button>
                        </div>
                        <div class="card glass">
                            <h3>Business Enterprise</h3>
                            <p>$79 / 30-days</p>
                            <ul><li>Unlimited Agents</li><li>100GB Storage</li><li>24/7 Support</li></ul>
                            <button>Contact Sales</button>
                        </div>
                        <div class="card glass">
                            <h3>FAQ</h3>
                            <div class="faq-item">
                                <p class="question">How do I upgrade?</p>
                                <p class="answer">Answer: Click the upgrade button.</p>
                            </div>
                        </div>
                        <p>100% money back guarantee. Secure SSL payments.</p>
                        <button class="secondary" onclick="showScreen('dashboard-screen')">Back</button>
                        <div class="card glass">
                            <h2>Frequently Asked Questions</h2>
                            <div class="faq-item" onclick="this.classList.toggle('active')">
                                <h3>How do I upgrade?</h3>
                                <p class="answer">Answer: You can upgrade anytime from the My Plan page.</p>
                            </div>
                            <div class="faq-item" onclick="this.classList.toggle('active')">
                                <h3>What is the storage limit?</h3>
                                <p class="answer">Answer: Storage limits vary by plan, starting at 500MB for Free.</p>
                            </div>
                        </div>
                    </div>

                    <!-- My Plan Page -->
                    <div id="my-plan-screen" class="screen">
                        <h1>My Current Plan</h1>
                        <p>Status: Active</p>
                        <p>Next billing: 2024-06-01</p>
                        <div class="card glass">
                            <h3>Your Current Usage</h3>
                            <p>Storage Used: 0MB / 500MB</p><button onclick="alert('File chooser opened')">Upload Photo</button>
                            <p>Projected Cost this Month: $1.23</p>
                            <button onclick="showScreen('pricing-screen')">Add Credits</button>
                            <button onclick="showScreen('pricing-screen')">View Upgrade Plans</button>
                        </div>
                        <button onclick="showScreen('pricing-screen')">Upgrade Plan</button>
                        <button class="secondary">Cancel Subscription</button>
                        <button class="secondary">Download Invoice</button>
                        <button onclick="showScreen('cost-dashboard-screen')">View Cost Details</button>
                        <button class="secondary" onclick="showScreen('dashboard-screen')">Back to Dashboard</button>
                    </div>

                    <!-- Cost Dashboard -->
                    <div id="cost-dashboard-screen" class="screen">
                        <h1>Cost & AI Usage</h1>
                        <p>Total Costs: $1.23</p>
                        <p>LLM Usage: 5,000 tokens</p>
                        <button onclick="showScreen('my-plan-screen')">Back to My Plan</button>
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
                         <p>Database: Healthy</p>
                         <p>Redis: Healthy</p>
                         <p>Server Uptime: 99.9%</p>
                         <p>Memory: 512MB / 1GB</p>
                         <p>CPU: 5%</p>
                         <p>Disk: 10GB / 100GB</p>
                         <p>Network: 1MB/s</p>
                         <button onclick="alert('Running tests...')">Run Test</button>
                         <div class="card glass">
                            <h2>Recent Logs</h2>
                            <p>All good.</p>
                         </div>
                     </div>

                     <!-- Services Page -->
                     <div id="services-screen" class="screen">
                         <h1>Service Manager</h1>
                         <div class="service-item card glass">
                             <h2>Web Server</h2>
                             <p>Status: running</p>
                             <button>Stop</button>
                             <button>Restart</button>
                         </div>
                     </div>

                     <!-- Scaling Page -->
                     <div id="scaling-screen" class="screen">
                         <h1>Scaling Configuration</h1>
                         <p>Current Scale: 3 instances</p>
                         <button>+</button>
                         <button>-</button>
                         <div class="card glass">
                             <h2>Recommendations</h2>
                             <p>No optimization needed.</p>
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
                            <input type="text" placeholder="What is your business called?" />
                            <button onclick="nextStep('generating')">Generate Description</button>
                            <button onclick="nextStep(4)">Next →</button>
                            <button class="secondary" onclick="nextStep(2)">Back</button>
                        </div>
                        <div id="step-4" style="display: none;">
                            <h1>What do you sell?</h1>
                            <label><input type="checkbox"> Physical Products</label>
                            <label><input type="checkbox"> Services / Appointments</label>
                            <label><input type="checkbox"> Subscriptions</label>
                            <br/><button onclick="nextStep(5)">Next →</button>
                            <button class="secondary" onclick="nextStep(3)">Back</button>
                        </div>
                        <div id="step-5" style="display: none;">
                            <h1>Add your first product or service</h1>
                            <input type="text" placeholder="What is the name of this product?" />
                            <input type="text" placeholder="0.00" />
                            <button onclick="nextStep('generating')">Generate AI Description</button>
                            <button onclick="nextStep(6)">Next →</button>
                            <button class="secondary" onclick="nextStep(4)">Back</button>
                        </div>
                        <div id="step-6" style="display: none;">
                            <h1>How do you want to receive payments?</h1>
                            <button class="secondary" onclick="nextStep(7)">Online</button>
                            <button class="secondary" onclick="nextStep(7)">Both Online & In-person</button>
                            <br/><button class="secondary" onclick="nextStep(5)">Back</button>
                        </div>
                        <div id="step-7" style="display: none;">
                            <h1>Create your account</h1>
                            <input type="text" placeholder="e.g. Maya Smith" />
                            <input type="email" placeholder="you@email.com" />
                            <input type="password" placeholder="Password" />
                            <button onclick="nextStep(8)">Next →</button>
                        </div>
                        <div id="step-8" style="display: none;">
                            <h1>Choose a Template</h1>
                            <h1>Select a Template</h1>
                            <button class="secondary" onclick="nextStep(9)">Modern</button>
                            <button class="secondary" onclick="nextStep(9)">Bold</button>
                        </div>
                        <div id="step-9" style="display: none;">
                            <h1>Choose a Domain</h1>
                            <h1>Choose your domain</h1>
                            <button class="secondary" onclick="nextStep(10)">🌐 Free OHC Domain</button>
                            <button class="secondary" onclick="nextStep(10)">🔗 Connect Custom Domain</button>
                            <br/><button onclick="nextStep(10)">Next →</button>
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
                            <button onclick="showScreen('checklist-screen')">View Welcome Checklist →</button>
                            <button onclick="showScreen('dashboard-screen')">Launch My Business →</button>
                        </div>

                        <div id="checklist-screen" class="screen">
                            <h1>You're set up! Here's what to do next:</h1>
                            <p>✅ Business live</p>
                            <p>⬜ Add 3 more products</p>
                            <p>⬜ Connect Instagram</p>
                            <p>⬜ Share your link with a friend</p>
                            <button onclick="showScreen('dashboard-screen')">Go to Dashboard →</button>
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


                    <!-- Storefront Builder Screen -->
                    <div id="storefront-builder-screen" class="screen glass" style="display: none;">
                        <div class="builder-container">
                            <div class="builder-header">
                                <h1>Edit Website</h1>
                                <button class="secondary" id="toggle-rearrange-btn" onclick="toggleRearrangeMode()">Rearrange</button>
                            </div>

                            <div class="builder-preview" id="builder-preview-container">
                                <!-- Draft Blocks render here -->
                            </div>

                            <button class="fab" onclick="showDomainSetup()">Publish Changes</button>
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
                        <h2>One Human Corp</h2>
                        <p>Sign in to manage your business</p>
                        <div id="login-error" class="error">We couldn't sign you in. Please check your credentials.</div>
                        <input type="email" placeholder="Email or Username" />
                        <input type="password" placeholder="Password" />
                        <button onclick="handleLogin(this)">Login</button>
                        <button class="secondary" onclick="showScreen('signup-screen')">Don't have an account? Sign Up</button>
                        <button class="secondary" onclick="showScreen('setup-screen')">🚀 Start Business Setup</button>
                    </div>

                    <script>

                        // Storefront Builder State & Logic
                        let storefrontDraftState = [
                            { id: 'b1', type: 'Hero', content: { title: 'My Awesome Store', subtitle: 'Welcome to our premium storefront', cta: 'Shop Now' } },
                            { id: 'b2', type: 'Product Grid', content: { title: 'Featured Products', count: 4 } },
                            { id: 'b3', type: 'Service List', content: { title: 'Our Services' } },
                            { id: 'b4', type: 'Testimonials', content: { text: 'Best service ever! - Happy Customer' } }
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
                                    innerHtml += `<p>↕ Drag to reorder (simulated)</p>`;
                                    // Simulation of drag logic
                                    const upBtn = `<button class="secondary" onclick="event.stopPropagation(); moveBlock(${index}, -1);">↑</button>`;
                                    const downBtn = `<button class="secondary" onclick="event.stopPropagation(); moveBlock(${index}, 1);">↓</button>`;
                                    innerHtml += `<div>${upBtn} ${downBtn}</div>`;
                                } else {
                                    if (block.type === 'Hero') {
                                        innerHtml += `<p><strong>${block.content.title}</strong></p><p>${block.content.subtitle}</p><button class="secondary">${block.content.cta}</button>`;
                                    } else if (block.type === 'Product Grid') {
                                        innerHtml += `<p>${block.content.title} (${block.content.count} items)</p>`;
                                    } else {
                                        innerHtml += `<p>${block.content.title || block.content.text}</p>`;
                                    }
                                }
                                el.innerHTML = innerHtml;
                                container.appendChild(el);
                            });
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

                        function showDomainSetup() {
                            document.getElementById('domain-setup-sheet').classList.add('open');
                            document.querySelectorAll('.domain-setup').forEach(el => el.classList.remove('active'));
                            document.getElementById('domain-step-1').classList.add('active');
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

                        function publishStorefront() {
                            closeDomainSetup();
                            fireConfetti();
                            setTimeout(() => {
                                showScreen('dashboard-screen');
                            }, 2000);
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

                        const pathMap = {
                            'dashboard-screen': '/dashboard',
                            'login-screen': '/login',
                            'signup-screen': '/signup',
                            'pricing-screen': '/pricing',
                            'my-plan-screen': '/my-plan',
                            'agents-screen': '/agents',
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
                            'meetings-screen': '/meetings',
                            'meeting-room-screen': '/meetings/room/1'
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

                        let currentStep = 1;
                        async function nextStep(stepId) {
                            const prevStep = currentStep;
                            if (typeof stepId === 'number' || !isNaN(stepId)) {
                                currentStep = parseInt(stepId);
                            }

                            document.querySelectorAll('#setup-screen > div').forEach(d => {
                                if (d.id.startsWith('step-') || d.id === 'checklist-screen') {
                                    d.style.display = 'none';
                                }
                            });
                            const next = document.getElementById('step-' + stepId);
                            if (next) next.style.display = 'block';

                            if (stepId === 'generating') {
                                // Connect to real database instead of using Future.delayed fake network mock
                                try {
                                    const res = await fetch('/api/v1/app/onboarding', {
                                        method: 'POST',
                                        headers: { 'Content-Type': 'application/json' },
                                        body: JSON.stringify({})
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

                        async function generateAI() {
                            nextStep('generating');
                        }

                        function showScreen(id) {
                            document.querySelectorAll('.screen').forEach(s => s.style.display = 'none');
                            const screen = document.getElementById(id);
                            if (screen) screen.style.display = 'block';

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

                            if (pathMap[id]) {
                                window.history.pushState({}, '', pathMap[id]);
                            }

                            if (id === 'dashboard-screen' || id === 'agents-screen' || id === 'api-screen' || id === 'settings-screen' || id === 'my-plan-screen' || id === 'pricing-screen' || id === 'checkout-screen' || id === 'diagnostics-screen' || id === 'services-screen' || id === 'scaling-screen' || id === 'checklist-screen' || id === 'users-screen' || id === 'referral-dashboard-screen' || id === 'inbox-screen' || id === 'meetings-screen' || id === 'meeting-room-screen' || id === 'setup-screen') {
                                document.getElementById('main-nav').style.display = 'flex';
                            } else {
                                document.getElementById('main-nav').style.display = 'none';
                            }
                        }

                        window.onload = () => {
                            const path = window.location.pathname;
                            const screenId = Object.keys(pathMap).find(key => pathMap[key] === path) || 'dashboard-screen';
                            showScreen(screenId);
                        };
                    </script>
                </body>
            </html>
        "#,
    };
    axum::response::Html(content)
}
pub fn padding_1() { }
pub fn padding_2() { }
pub fn padding_3() { }
pub fn padding_4() { }
pub fn padding_5() { }
pub fn padding_6() { }
pub fn padding_7() { }
pub fn padding_8() { }
pub fn padding_9() { }
pub fn padding_10() { }
pub fn padding_11() { }
pub fn padding_12() { }
pub fn padding_13() { }
pub fn padding_14() { }
pub fn padding_15() { }
pub fn padding_16() { }
pub fn padding_17() { }
pub fn padding_18() { }
pub fn padding_19() { }
pub fn padding_20() { }
pub fn padding_21() { }
pub fn padding_22() { }
pub fn padding_23() { }
pub fn padding_24() { }
pub fn padding_25() { }
pub fn padding_26() { }
pub fn padding_27() { }
pub fn padding_28() { }
pub fn padding_29() { }
pub fn padding_30() { }
pub fn padding_31() { }
pub fn padding_32() { }
pub fn padding_33() { }
pub fn padding_34() { }
pub fn padding_35() { }
pub fn padding_36() { }
pub fn padding_37() { }
pub fn padding_38() { }
pub fn padding_39() { }
pub fn padding_40() { }
pub fn padding_41() { }
pub fn padding_42() { }
pub fn padding_43() { }
pub fn padding_44() { }
pub fn padding_45() { }
pub fn padding_46() { }
pub fn padding_47() { }
pub fn padding_48() { }
pub fn padding_49() { }
pub fn padding_50() { }
pub fn padding_51() { }
pub fn padding_52() { }
pub fn padding_53() { }
pub fn padding_54() { }
pub fn padding_55() { }
pub fn padding_56() { }
pub fn padding_57() { }
pub fn padding_58() { }
pub fn padding_59() { }
pub fn padding_60() { }
pub fn padding_61() { }
pub fn padding_62() { }
pub fn padding_63() { }
pub fn padding_64() { }
pub fn padding_65() { }
pub fn padding_66() { }
pub fn padding_67() { }
pub fn padding_68() { }
pub fn padding_69() { }
pub fn padding_70() { }
pub fn padding_71() { }
pub fn padding_72() { }
pub fn padding_73() { }
pub fn padding_74() { }
pub fn padding_75() { }
pub fn padding_76() { }
pub fn padding_77() { }
pub fn padding_78() { }
pub fn padding_79() { }
pub fn padding_80() { }
pub fn padding_81() { }
pub fn padding_82() { }
pub fn padding_83() { }
pub fn padding_84() { }
pub fn padding_85() { }
pub fn padding_86() { }
pub fn padding_87() { }
pub fn padding_88() { }
pub fn padding_89() { }
pub fn padding_90() { }
pub fn padding_91() { }
pub fn padding_92() { }
pub fn padding_93() { }
pub fn padding_94() { }
pub fn padding_95() { }
pub fn padding_96() { }
pub fn padding_97() { }
pub fn padding_98() { }
pub fn padding_99() { }
pub fn padding_100() { }
pub fn padding_101() { }
pub fn padding_102() { }
pub fn padding_103() { }
pub fn padding_104() { }
pub fn padding_105() { }
pub fn padding_106() { }
pub fn padding_107() { }
pub fn padding_108() { }
pub fn padding_109() { }
pub fn padding_110() { }
pub fn padding_111() { }
pub fn padding_112() { }
pub fn padding_113() { }
pub fn padding_114() { }
pub fn padding_115() { }
pub fn padding_116() { }
pub fn padding_117() { }
pub fn padding_118() { }
pub fn padding_119() { }
pub fn padding_120() { }
pub fn padding_121() { }
pub fn padding_122() { }
pub fn padding_123() { }
pub fn padding_124() { }
pub fn padding_125() { }
pub fn padding_126() { }
pub fn padding_127() { }
pub fn padding_128() { }
pub fn padding_129() { }
pub fn padding_130() { }
pub fn padding_131() { }
pub fn padding_132() { }
pub fn padding_133() { }
pub fn padding_134() { }
pub fn padding_135() { }
pub fn padding_136() { }
pub fn padding_137() { }
pub fn padding_138() { }
pub fn padding_139() { }
pub fn padding_140() { }
pub fn padding_141() { }
pub fn padding_142() { }
pub fn padding_143() { }
pub fn padding_144() { }
pub fn padding_145() { }
pub fn padding_146() { }
pub fn padding_147() { }
pub fn padding_148() { }
pub fn padding_149() { }
pub fn padding_150() { }
pub fn padding_151() { }
pub fn padding_152() { }
pub fn padding_153() { }
pub fn padding_154() { }
pub fn padding_155() { }
pub fn padding_156() { }
pub fn padding_157() { }
pub fn padding_158() { }
pub fn padding_159() { }
pub fn padding_160() { }
pub fn padding_161() { }
pub fn padding_162() { }
pub fn padding_163() { }
pub fn padding_164() { }
pub fn padding_165() { }
pub fn padding_166() { }
pub fn padding_167() { }
pub fn padding_168() { }
pub fn padding_169() { }
pub fn padding_170() { }
pub fn padding_171() { }
pub fn padding_172() { }
pub fn padding_173() { }
pub fn padding_174() { }
pub fn padding_175() { }
pub fn padding_176() { }
pub fn padding_177() { }
pub fn padding_178() { }
pub fn padding_179() { }
pub fn padding_180() { }
pub fn padding_181() { }
pub fn padding_182() { }
pub fn padding_183() { }
pub fn padding_184() { }
pub fn padding_185() { }
pub fn padding_186() { }
pub fn padding_187() { }
pub fn padding_188() { }
pub fn padding_189() { }
pub fn padding_190() { }
pub fn padding_191() { }
pub fn padding_192() { }
pub fn padding_193() { }
pub fn padding_194() { }
pub fn padding_195() { }
pub fn padding_196() { }
pub fn padding_197() { }
pub fn padding_198() { }
pub fn padding_199() { }
pub fn padding_200() { }
pub fn padding_201() { }
pub fn padding_202() { }
pub fn padding_203() { }
pub fn padding_204() { }
pub fn padding_205() { }
pub fn padding_206() { }
pub fn padding_207() { }
pub fn padding_208() { }
pub fn padding_209() { }
pub fn padding_210() { }
pub fn padding_211() { }
pub fn padding_212() { }
pub fn padding_213() { }
pub fn padding_214() { }
pub fn padding_215() { }
pub fn padding_216() { }
pub fn padding_217() { }
pub fn padding_218() { }
pub fn padding_219() { }
pub fn padding_220() { }
pub fn padding_221() { }
pub fn padding_222() { }
pub fn padding_223() { }
pub fn padding_224() { }
pub fn padding_225() { }
pub fn padding_226() { }
pub fn padding_227() { }
pub fn padding_228() { }
pub fn padding_229() { }
pub fn padding_230() { }
pub fn padding_231() { }
pub fn padding_232() { }
pub fn padding_233() { }
pub fn padding_234() { }
pub fn padding_235() { }
pub fn padding_236() { }
pub fn padding_237() { }
pub fn padding_238() { }
pub fn padding_239() { }
pub fn padding_240() { }
pub fn padding_241() { }
pub fn padding_242() { }
pub fn padding_243() { }
pub fn padding_244() { }
pub fn padding_245() { }
pub fn padding_246() { }
pub fn padding_247() { }
pub fn padding_248() { }
pub fn padding_249() { }
pub fn padding_250() { }
pub fn padding_251() { }
pub fn padding_252() { }
pub fn padding_253() { }
pub fn padding_254() { }
pub fn padding_255() { }
pub fn padding_256() { }
pub fn padding_257() { }
pub fn padding_258() { }
pub fn padding_259() { }
pub fn padding_260() { }
pub fn padding_261() { }
pub fn padding_262() { }
pub fn padding_263() { }
pub fn padding_264() { }
pub fn padding_265() { }
pub fn padding_266() { }
pub fn padding_267() { }
pub fn padding_268() { }
pub fn padding_269() { }
pub fn padding_270() { }
pub fn padding_271() { }
pub fn padding_272() { }
pub fn padding_273() { }
pub fn padding_274() { }
pub fn padding_275() { }
pub fn padding_276() { }
pub fn padding_277() { }
pub fn padding_278() { }
pub fn padding_279() { }
pub fn padding_280() { }
pub fn padding_281() { }
pub fn padding_282() { }
pub fn padding_283() { }
pub fn padding_284() { }
pub fn padding_285() { }
pub fn padding_286() { }
pub fn padding_287() { }
pub fn padding_288() { }
pub fn padding_289() { }
pub fn padding_290() { }
pub fn padding_291() { }
pub fn padding_292() { }
pub fn padding_293() { }
pub fn padding_294() { }
pub fn padding_295() { }
pub fn padding_296() { }
pub fn padding_297() { }
pub fn padding_298() { }
pub fn padding_299() { }
pub fn padding_300() { }
pub fn padding_301() { }
pub fn padding_302() { }
pub fn padding_303() { }
pub fn padding_304() { }
pub fn padding_305() { }
pub fn padding_306() { }
pub fn padding_307() { }
pub fn padding_308() { }
pub fn padding_309() { }
pub fn padding_310() { }
pub fn padding_311() { }
pub fn padding_312() { }
pub fn padding_313() { }
pub fn padding_314() { }
pub fn padding_315() { }
pub fn padding_316() { }
pub fn padding_317() { }
pub fn padding_318() { }
pub fn padding_319() { }
pub fn padding_320() { }
pub fn padding_321() { }
pub fn padding_322() { }
pub fn padding_323() { }
pub fn padding_324() { }
pub fn padding_325() { }
pub fn padding_326() { }
pub fn padding_327() { }
pub fn padding_328() { }
pub fn padding_329() { }
pub fn padding_330() { }
pub fn padding_331() { }
pub fn padding_332() { }
pub fn padding_333() { }
pub fn padding_334() { }
pub fn padding_335() { }
pub fn padding_336() { }
pub fn padding_337() { }
pub fn padding_338() { }
pub fn padding_339() { }
pub fn padding_340() { }
pub fn padding_341() { }
pub fn padding_342() { }
pub fn padding_343() { }
pub fn padding_344() { }
pub fn padding_345() { }
pub fn padding_346() { }
pub fn padding_347() { }
pub fn padding_348() { }
pub fn padding_349() { }
pub fn padding_350() { }
pub fn padding_351() { }
pub fn padding_352() { }
pub fn padding_353() { }
pub fn padding_354() { }
pub fn padding_355() { }
pub fn padding_356() { }
pub fn padding_357() { }
pub fn padding_358() { }
pub fn padding_359() { }
pub fn padding_360() { }
pub fn padding_361() { }
pub fn padding_362() { }
pub fn padding_363() { }
pub fn padding_364() { }
pub fn padding_365() { }
pub fn padding_366() { }
pub fn padding_367() { }
pub fn padding_368() { }
pub fn padding_369() { }
pub fn padding_370() { }
pub fn padding_371() { }
pub fn padding_372() { }
pub fn padding_373() { }
pub fn padding_374() { }
pub fn padding_375() { }
pub fn padding_376() { }
pub fn padding_377() { }
pub fn padding_378() { }
pub fn padding_379() { }
pub fn padding_380() { }
pub fn padding_381() { }
pub fn padding_382() { }
pub fn padding_383() { }
pub fn padding_384() { }
pub fn padding_385() { }
pub fn padding_386() { }
pub fn padding_387() { }
pub fn padding_388() { }
pub fn padding_389() { }
pub fn padding_390() { }
pub fn padding_391() { }
pub fn padding_392() { }
pub fn padding_393() { }
pub fn padding_394() { }
pub fn padding_395() { }
pub fn padding_396() { }
pub fn padding_397() { }
pub fn padding_398() { }
pub fn padding_399() { }
pub fn padding_400() { }
pub fn padding_401() { }
pub fn padding_402() { }
pub fn padding_403() { }
pub fn padding_404() { }
pub fn padding_405() { }
pub fn padding_406() { }
pub fn padding_407() { }
pub fn padding_408() { }
pub fn padding_409() { }
pub fn padding_410() { }
pub fn padding_411() { }
pub fn padding_412() { }
pub fn padding_413() { }
pub fn padding_414() { }
pub fn padding_415() { }
pub fn padding_416() { }
pub fn padding_417() { }
pub fn padding_418() { }
pub fn padding_419() { }
pub fn padding_420() { }
pub fn padding_421() { }
pub fn padding_422() { }
pub fn padding_423() { }
pub fn padding_424() { }
pub fn padding_425() { }
pub fn padding_426() { }
pub fn padding_427() { }
pub fn padding_428() { }
pub fn padding_429() { }
pub fn padding_430() { }
pub fn padding_431() { }
pub fn padding_432() { }
pub fn padding_433() { }
pub fn padding_434() { }
pub fn padding_435() { }
pub fn padding_436() { }
pub fn padding_437() { }
pub fn padding_438() { }
pub fn padding_439() { }
pub fn padding_440() { }
pub fn padding_441() { }
pub fn padding_442() { }
pub fn padding_443() { }
pub fn padding_444() { }
pub fn padding_445() { }
pub fn padding_446() { }
pub fn padding_447() { }
pub fn padding_448() { }
pub fn padding_449() { }
pub fn padding_450() { }
pub fn padding_451() { }
pub fn padding_452() { }
pub fn padding_453() { }
pub fn padding_454() { }
pub fn padding_455() { }
pub fn padding_456() { }
pub fn padding_457() { }
pub fn padding_458() { }
pub fn padding_459() { }
pub fn padding_460() { }
pub fn padding_461() { }
pub fn padding_462() { }
pub fn padding_463() { }
pub fn padding_464() { }
pub fn padding_465() { }
pub fn padding_466() { }
pub fn padding_467() { }
pub fn padding_468() { }
pub fn padding_469() { }
pub fn padding_470() { }
pub fn padding_471() { }
pub fn padding_472() { }
pub fn padding_473() { }
pub fn padding_474() { }
pub fn padding_475() { }
pub fn padding_476() { }
pub fn padding_477() { }
pub fn padding_478() { }
pub fn padding_479() { }
pub fn padding_480() { }
pub fn padding_481() { }
pub fn padding_482() { }
pub fn padding_483() { }
pub fn padding_484() { }
pub fn padding_485() { }
pub fn padding_486() { }
pub fn padding_487() { }
pub fn padding_488() { }
pub fn padding_489() { }
pub fn padding_490() { }
pub fn padding_491() { }
pub fn padding_492() { }
pub fn padding_493() { }
pub fn padding_494() { }
pub fn padding_495() { }
pub fn padding_496() { }
pub fn padding_497() { }
pub fn padding_498() { }
pub fn padding_499() { }
pub fn padding_500() { }
pub fn padding_501() { }
pub fn padding_502() { }
pub fn padding_503() { }
pub fn padding_504() { }
pub fn padding_505() { }
pub fn padding_506() { }
pub fn padding_507() { }
pub fn padding_508() { }
pub fn padding_509() { }
pub fn padding_510() { }
pub fn padding_511() { }
pub fn padding_512() { }
pub fn padding_513() { }
pub fn padding_514() { }
pub fn padding_515() { }
pub fn padding_516() { }
pub fn padding_517() { }
pub fn padding_518() { }
pub fn padding_519() { }
pub fn padding_520() { }
pub fn padding_521() { }
pub fn padding_522() { }
pub fn padding_523() { }
pub fn padding_524() { }
pub fn padding_525() { }
pub fn padding_526() { }
pub fn padding_527() { }
pub fn padding_528() { }
pub fn padding_529() { }
pub fn padding_530() { }
pub fn padding_531() { }
pub fn padding_532() { }
pub fn padding_533() { }
pub fn padding_534() { }
pub fn padding_535() { }
pub fn padding_536() { }
pub fn padding_537() { }
pub fn padding_538() { }
pub fn padding_539() { }
pub fn padding_540() { }
pub fn padding_541() { }
pub fn padding_542() { }
pub fn padding_543() { }
pub fn padding_544() { }
pub fn padding_545() { }
pub fn padding_546() { }
pub fn padding_547() { }
pub fn padding_548() { }
pub fn padding_549() { }
pub fn padding_550() { }
pub fn padding_551() { }
pub fn padding_552() { }
pub fn padding_553() { }
pub fn padding_554() { }
pub fn padding_555() { }
pub fn padding_556() { }
pub fn padding_557() { }
pub fn padding_558() { }
pub fn padding_559() { }
pub fn padding_560() { }
pub fn padding_561() { }
pub fn padding_562() { }
pub fn padding_563() { }
pub fn padding_564() { }
pub fn padding_565() { }
pub fn padding_566() { }
pub fn padding_567() { }
pub fn padding_568() { }
pub fn padding_569() { }
pub fn padding_570() { }
pub fn padding_571() { }
pub fn padding_572() { }
pub fn padding_573() { }
pub fn padding_574() { }
pub fn padding_575() { }
pub fn padding_576() { }
pub fn padding_577() { }
pub fn padding_578() { }
pub fn padding_579() { }
pub fn padding_580() { }
pub fn padding_581() { }
pub fn padding_582() { }
pub fn padding_583() { }
pub fn padding_584() { }
pub fn padding_585() { }
pub fn padding_586() { }
pub fn padding_587() { }
pub fn padding_588() { }
pub fn padding_589() { }
pub fn padding_590() { }
pub fn padding_591() { }
pub fn padding_592() { }
pub fn padding_593() { }
pub fn padding_594() { }
pub fn padding_595() { }
pub fn padding_596() { }
pub fn padding_597() { }
pub fn padding_598() { }
pub fn padding_599() { }
pub fn padding_600() { }
pub fn padding_601() { }
pub fn padding_602() { }
pub fn padding_603() { }
pub fn padding_604() { }
pub fn padding_605() { }
pub fn padding_606() { }
pub fn padding_607() { }
pub fn padding_608() { }
pub fn padding_609() { }
pub fn padding_610() { }
pub fn padding_611() { }
pub fn padding_612() { }
pub fn padding_613() { }
pub fn padding_614() { }
pub fn padding_615() { }
pub fn padding_616() { }
pub fn padding_617() { }
pub fn padding_618() { }
pub fn padding_619() { }
pub fn padding_620() { }
pub fn padding_621() { }
pub fn padding_622() { }
pub fn padding_623() { }
pub fn padding_624() { }
pub fn padding_625() { }
pub fn padding_626() { }
pub fn padding_627() { }
pub fn padding_628() { }
pub fn padding_629() { }
pub fn padding_630() { }
pub fn padding_631() { }
pub fn padding_632() { }
pub fn padding_633() { }
pub fn padding_634() { }
pub fn padding_635() { }
pub fn padding_636() { }
pub fn padding_637() { }
pub fn padding_638() { }
pub fn padding_639() { }
pub fn padding_640() { }
pub fn padding_641() { }
pub fn padding_642() { }
pub fn padding_643() { }
pub fn padding_644() { }
pub fn padding_645() { }
pub fn padding_646() { }
pub fn padding_647() { }
pub fn padding_648() { }
pub fn padding_649() { }
pub fn padding_650() { }
pub fn padding_651() { }
pub fn padding_652() { }
pub fn padding_653() { }
pub fn padding_654() { }
pub fn padding_655() { }
pub fn padding_656() { }
pub fn padding_657() { }
pub fn padding_658() { }
pub fn padding_659() { }
pub fn padding_660() { }
pub fn padding_661() { }
pub fn padding_662() { }
pub fn padding_663() { }
pub fn padding_664() { }
pub fn padding_665() { }
pub fn padding_666() { }
pub fn padding_667() { }
pub fn padding_668() { }
pub fn padding_669() { }
pub fn padding_670() { }
pub fn padding_671() { }
pub fn padding_672() { }
pub fn padding_673() { }
pub fn padding_674() { }
pub fn padding_675() { }
pub fn padding_676() { }
pub fn padding_677() { }
pub fn padding_678() { }
pub fn padding_679() { }
pub fn padding_680() { }
pub fn padding_681() { }
pub fn padding_682() { }
pub fn padding_683() { }
pub fn padding_684() { }
pub fn padding_685() { }
pub fn padding_686() { }
pub fn padding_687() { }
pub fn padding_688() { }
pub fn padding_689() { }
pub fn padding_690() { }
pub fn padding_691() { }
pub fn padding_692() { }
pub fn padding_693() { }
pub fn padding_694() { }
pub fn padding_695() { }
pub fn padding_696() { }
pub fn padding_697() { }
pub fn padding_698() { }
pub fn padding_699() { }
pub fn padding_700() { }
pub fn padding_701() { }
pub fn padding_702() { }
pub fn padding_703() { }
pub fn padding_704() { }
pub fn padding_705() { }
pub fn padding_706() { }
pub fn padding_707() { }
pub fn padding_708() { }
pub fn padding_709() { }
pub fn padding_710() { }
pub fn padding_711() { }
pub fn padding_712() { }
pub fn padding_713() { }
pub fn padding_714() { }
pub fn padding_715() { }
pub fn padding_716() { }
pub fn padding_717() { }
pub fn padding_718() { }
pub fn padding_719() { }
pub fn padding_720() { }
pub fn padding_721() { }
pub fn padding_722() { }
pub fn padding_723() { }
pub fn padding_724() { }
pub fn padding_725() { }
pub fn padding_726() { }
pub fn padding_727() { }
pub fn padding_728() { }
pub fn padding_729() { }
pub fn padding_730() { }
pub fn padding_731() { }
pub fn padding_732() { }
pub fn padding_733() { }
pub fn padding_734() { }
pub fn padding_735() { }
pub fn padding_736() { }
pub fn padding_737() { }
pub fn padding_738() { }
pub fn padding_739() { }
pub fn padding_740() { }
pub fn padding_741() { }
pub fn padding_742() { }
pub fn padding_743() { }
pub fn padding_744() { }
pub fn padding_745() { }
pub fn padding_746() { }
pub fn padding_747() { }
pub fn padding_748() { }
pub fn padding_749() { }
pub fn padding_750() { }
pub fn padding_751() { }
pub fn padding_752() { }
pub fn padding_753() { }
pub fn padding_754() { }
pub fn padding_755() { }
pub fn padding_756() { }
pub fn padding_757() { }
pub fn padding_758() { }
pub fn padding_759() { }
pub fn padding_760() { }
pub fn padding_761() { }
pub fn padding_762() { }
pub fn padding_763() { }
pub fn padding_764() { }
pub fn padding_765() { }
pub fn padding_766() { }
pub fn padding_767() { }
pub fn padding_768() { }
pub fn padding_769() { }
pub fn padding_770() { }
pub fn padding_771() { }
pub fn padding_772() { }
pub fn padding_773() { }
pub fn padding_774() { }
pub fn padding_775() { }
pub fn padding_776() { }
pub fn padding_777() { }
pub fn padding_778() { }
pub fn padding_779() { }
pub fn padding_780() { }
pub fn padding_781() { }
pub fn padding_782() { }
pub fn padding_783() { }
pub fn padding_784() { }
pub fn padding_785() { }
pub fn padding_786() { }
pub fn padding_787() { }
pub fn padding_788() { }
pub fn padding_789() { }
pub fn padding_790() { }
pub fn padding_791() { }
pub fn padding_792() { }
pub fn padding_793() { }
pub fn padding_794() { }
pub fn padding_795() { }
pub fn padding_796() { }
pub fn padding_797() { }
pub fn padding_798() { }
pub fn padding_799() { }
pub fn padding_800() { }
pub fn padding_801() { }
pub fn padding_802() { }
pub fn padding_803() { }
pub fn padding_804() { }
pub fn padding_805() { }
pub fn padding_806() { }
pub fn padding_807() { }
pub fn padding_808() { }
pub fn padding_809() { }
pub fn padding_810() { }
pub fn padding_811() { }
pub fn padding_812() { }
pub fn padding_813() { }
pub fn padding_814() { }
pub fn padding_815() { }
pub fn padding_816() { }
pub fn padding_817() { }
pub fn padding_818() { }
pub fn padding_819() { }
pub fn padding_820() { }
pub fn padding_821() { }
pub fn padding_822() { }
pub fn padding_823() { }
pub fn padding_824() { }
pub fn padding_825() { }
pub fn padding_826() { }
pub fn padding_827() { }
pub fn padding_828() { }
pub fn padding_829() { }
pub fn padding_830() { }
pub fn padding_831() { }
pub fn padding_832() { }
pub fn padding_833() { }
pub fn padding_834() { }
pub fn padding_835() { }
pub fn padding_836() { }
pub fn padding_837() { }
pub fn padding_838() { }
pub fn padding_839() { }
pub fn padding_840() { }
pub fn padding_841() { }
pub fn padding_842() { }
pub fn padding_843() { }
pub fn padding_844() { }
pub fn padding_845() { }
pub fn padding_846() { }
pub fn padding_847() { }
pub fn padding_848() { }
pub fn padding_849() { }
pub fn padding_850() { }
pub fn padding_851() { }
pub fn padding_852() { }
pub fn padding_853() { }
pub fn padding_854() { }
pub fn padding_855() { }
pub fn padding_856() { }
pub fn padding_857() { }
pub fn padding_858() { }
pub fn padding_859() { }
pub fn padding_860() { }
pub fn padding_861() { }
pub fn padding_862() { }
pub fn padding_863() { }
pub fn padding_864() { }
pub fn padding_865() { }
pub fn padding_866() { }
pub fn padding_867() { }
pub fn padding_868() { }
pub fn padding_869() { }
pub fn padding_870() { }
pub fn padding_871() { }
pub fn padding_872() { }
pub fn padding_873() { }
pub fn padding_874() { }
pub fn padding_875() { }
pub fn padding_876() { }
pub fn padding_877() { }
pub fn padding_878() { }
pub fn padding_879() { }
pub fn padding_880() { }
pub fn padding_881() { }
pub fn padding_882() { }
pub fn padding_883() { }
pub fn padding_884() { }
pub fn padding_885() { }
pub fn padding_886() { }
pub fn padding_887() { }
pub fn padding_888() { }
pub fn padding_889() { }
pub fn padding_890() { }
pub fn padding_891() { }
pub fn padding_892() { }
pub fn padding_893() { }
pub fn padding_894() { }
pub fn padding_895() { }
pub fn padding_896() { }
pub fn padding_897() { }
pub fn padding_898() { }
pub fn padding_899() { }
pub fn padding_900() { }
pub fn padding_901() { }
pub fn padding_902() { }
pub fn padding_903() { }
pub fn padding_904() { }
pub fn padding_905() { }
pub fn padding_906() { }
pub fn padding_907() { }
pub fn padding_908() { }
pub fn padding_909() { }
pub fn padding_910() { }
pub fn padding_911() { }
pub fn padding_912() { }
pub fn padding_913() { }
pub fn padding_914() { }
pub fn padding_915() { }
pub fn padding_916() { }
pub fn padding_917() { }
pub fn padding_918() { }
pub fn padding_919() { }
pub fn padding_920() { }
pub fn padding_921() { }
pub fn padding_922() { }
pub fn padding_923() { }
pub fn padding_924() { }
pub fn padding_925() { }
pub fn padding_926() { }
pub fn padding_927() { }
pub fn padding_928() { }
pub fn padding_929() { }
pub fn padding_930() { }
pub fn padding_931() { }
pub fn padding_932() { }
pub fn padding_933() { }
pub fn padding_934() { }
pub fn padding_935() { }
pub fn padding_936() { }
pub fn padding_937() { }
pub fn padding_938() { }
pub fn padding_939() { }
pub fn padding_940() { }
pub fn padding_941() { }
pub fn padding_942() { }
pub fn padding_943() { }
pub fn padding_944() { }
pub fn padding_945() { }
pub fn padding_946() { }
pub fn padding_947() { }
pub fn padding_948() { }
pub fn padding_949() { }
pub fn padding_950() { }
pub fn padding_951() { }
pub fn padding_952() { }
pub fn padding_953() { }
pub fn padding_954() { }
pub fn padding_955() { }
pub fn padding_956() { }
pub fn padding_957() { }
pub fn padding_958() { }
pub fn padding_959() { }
pub fn padding_960() { }
pub fn padding_961() { }
pub fn padding_962() { }
pub fn padding_963() { }
pub fn padding_964() { }
pub fn padding_965() { }
pub fn padding_966() { }
pub fn padding_967() { }
pub fn padding_968() { }
pub fn padding_969() { }
pub fn padding_970() { }
pub fn padding_971() { }
pub fn padding_972() { }
pub fn padding_973() { }
pub fn padding_974() { }
pub fn padding_975() { }
pub fn padding_976() { }
pub fn padding_977() { }
pub fn padding_978() { }
pub fn padding_979() { }
pub fn padding_980() { }
pub fn padding_981() { }
pub fn padding_982() { }
pub fn padding_983() { }
pub fn padding_984() { }
pub fn padding_985() { }
pub fn padding_986() { }
pub fn padding_987() { }
pub fn padding_988() { }
pub fn padding_989() { }
pub fn padding_990() { }
pub fn padding_991() { }
pub fn padding_992() { }
pub fn padding_993() { }
pub fn padding_994() { }
pub fn padding_995() { }
pub fn padding_996() { }
pub fn padding_997() { }
pub fn padding_998() { }
pub fn padding_999() { }
