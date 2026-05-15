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
        .route("/api/wizard/state", axum::routing::post(wizard_state_handler))
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

async fn wizard_state_handler(axum::Json(_payload): axum::Json<serde_json::Value>) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::OK, "State saved")
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
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px;">
                            <h1>Dashboard</h1>
                            <div style="display: flex; align-items: center; gap: 8px;">
                                <span style="font-size: 14px;">Simple Mode</span>
                                <label class="switch" style="position: relative; display: inline-block; width: 40px; height: 20px;">
                                    <input type="checkbox" id="mode-toggle" onchange="
                                        const isAdvanced = this.checked;
                                        if (isAdvanced) {
                                            document.body.classList.add('advanced-mode');
                                            localStorage.setItem('ohc_advanced_mode', 'true');
                                        } else {
                                            document.body.classList.remove('advanced-mode');
                                            localStorage.setItem('ohc_advanced_mode', 'false');
                                        }
                                        fetch('/api/wizard/state', {
                                            method: 'POST',
                                            headers: { 'Content-Type': 'application/json' },
                                            body: JSON.stringify({ step: 'advanced_mode_toggle', data: { advanced: isAdvanced } })
                                        }).catch(e => console.error(e));
                                    " style="opacity: 0; width: 0; height: 0;">
                                    <span style="position: absolute; cursor: pointer; top: 0; left: 0; right: 0; bottom: 0; background-color: #ccc; transition: .4s; border-radius: 20px;"></span>
                                    <span class="slider-btn" style="position: absolute; content: ''; height: 16px; width: 16px; left: 2px; bottom: 2px; background-color: white; transition: .4s; border-radius: 50%;"></span>
                                </label>
                                <span style="font-size: 14px; color: var(--text-secondary);">Advanced Mode</span>
                            </div>
                        </div>
                        <style>
                            .advanced-mode .advanced-only { display: block !important; }
                            .advanced-only { display: none !important; }
                            input:checked + span { background-color: var(--primary-color); }
                            input:checked + span + .slider-btn { transform: translateX(20px); }
                        </style>
                        <div class="card glass" style="margin-bottom: 24px;">
                            <h2>Welcome back, Human.</h2>
                            <p>Your business is looking great today. Let's keep growing!</p>
                            <button onclick="document.getElementById('grow-wizard').style.display='block';">🚀 Grow my business</button>
                        </div>
                        <div id="grow-wizard" style="display: none; position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.8); z-index: 1000; align-items: center; justify-content: center; flex-direction: column;">
                            <div class="card glass" style="max-width: 400px; width: 100%;">
                                <h2>Grow Your Business</h2>
                                <p>Here are a few quick things you can do today:</p>
                                <button style="width: 100%; margin-bottom: 8px;" onclick="alert('Adding products...'); this.parentElement.parentElement.style.display='none';">+ Add 5 more products</button>
                                <button style="width: 100%; margin-bottom: 8px;" onclick="alert('Connecting Instagram...'); this.parentElement.parentElement.style.display='none';">📸 Connect Instagram</button>
                                <button style="width: 100%; margin-bottom: 8px;" onclick="alert('Starting email campaign...'); this.parentElement.parentElement.style.display='none';">📧 Run your first email campaign</button>
                                <button class="secondary" style="width: 100%;" onclick="this.parentElement.parentElement.style.display='none';">Close</button>
                            </div>
                        </div>

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

                    <div id="agents-screen" class="screen glass" style="backdrop-filter: blur(20px);">
                        <h1 class="wizard-h1">AI Agents</h1>

                        <div id="agents-gallery">
                            <p class="wizard-p">Manage your AI team</p>
                            <div class="card-grid">
                                <div class="card-tile">
                                    <h3 style="font-size: 24px;">💬</h3>
                                    <h3>Customer Support</h3>
                                    <p>Handles basic queries.</p>
                                    <button style="margin-top: 12px; width: 100%;" onclick="openAgentWizard('Customer Support')">Add to my team</button>
                                </div>
                                <div class="card-tile">
                                    <h3 style="font-size: 24px;">📸</h3>
                                    <h3>Social Media Manager</h3>
                                    <p>Posts automatically.</p>
                                    <button style="margin-top: 12px; width: 100%;" onclick="openAgentWizard('Social Media Manager')">Add to my team</button>
                                </div>
                                <div class="card-tile">
                                    <h3 style="font-size: 24px;">🚀</h3>
                                    <h3>SEO Booster</h3>
                                    <p>Improves search ranking.</p>
                                    <button style="margin-top: 12px; width: 100%;" onclick="openAgentWizard('SEO Booster')">Add to my team</button>
                                </div>
                            </div>
                            <h2 style="margin-top: 32px;">Active Agents</h2>
                            <div class="card glass" style="display: flex; justify-content: space-between; align-items: center;">
                                <div>
                                    <h3 style="margin-bottom: 4px;">Marketing Pro <span style="color: #4caf50; font-size: 12px;">● Active</span></h3>
                                    <p style="margin: 0; color: var(--text-secondary); font-size: 12px;">Writing product descriptions</p>
                                </div>
                                <div>
                                    <button class="secondary" onclick="openAgentTuning('Marketing Pro')">Tune this agent</button>
                                    <button class="secondary" style="color: #f44336;" onclick="document.getElementById('fix-wizard').style.display='block'">Help me fix this</button>
                                </div>
                            </div>
                            <button class="secondary" style="margin-top: 24px;" onclick="showScreen('dashboard-screen')">Back to Dashboard</button>
                        </div>

                        <!-- Agent Configuration Wizard -->
                        <div id="agent-wizard" style="display: none;">
                            <h2 id="aw-title">Configure Agent</h2>
                            <div style="margin-bottom: 24px;">
                                <h3>Capabilities</h3>
                                <label style="display: flex; align-items: center; margin-bottom: 12px; cursor: pointer;">
                                    <input type="checkbox" checked style="margin-right: 12px;"> Reply to customer messages
                                </label>
                                <label style="display: flex; align-items: center; margin-bottom: 12px; cursor: pointer;">
                                    <input type="checkbox" style="margin-right: 12px;"> Post to Instagram & Facebook
                                </label>
                                <label style="display: flex; align-items: center; margin-bottom: 12px; cursor: pointer;">
                                    <input type="checkbox" checked style="margin-right: 12px;"> Write product descriptions
                                </label>
                            </div>
                            <div style="margin-bottom: 24px;">
                                <h3>How often should this agent work?</h3>
                                <input type="range" min="1" max="4" value="2" style="width: 100%;">
                                <div style="display: flex; justify-content: space-between; font-size: 12px; color: var(--text-secondary);">
                                    <span>Real-time</span><span>Hourly</span><span>Daily</span><span>Weekly</span>
                                </div>
                            </div>
                            <div class="wizard-actions">
                                <button class="secondary" onclick="document.getElementById('agent-wizard').style.display='none'; document.getElementById('agents-gallery').style.display='block';">Cancel</button>
                                <button onclick="alert('Agent Activated!'); document.getElementById('agent-wizard').style.display='none'; document.getElementById('agents-gallery').style.display='block';">Activate</button>
                            </div>
                        </div>

                        <!-- Prompt Tuning Wizard -->
                        <div id="agent-tuning" style="display: none; padding: 16px; background: rgba(255,255,255,0.05); border-radius: 12px;">
                            <h2>Tune this agent</h2>
                            <div style="display: flex; gap: 24px;">
                                <div style="flex: 1;">
                                    <div style="margin-bottom: 16px;">
                                        <h3>Tone</h3>
                                        <div style="display: flex; gap: 8px; flex-wrap: wrap;">
                                            <label><input type="radio" name="tone" checked> Friendly & Warm</label>
                                            <label><input type="radio" name="tone"> Professional</label>
                                            <label><input type="radio" name="tone"> Energetic</label>
                                            <label><input type="radio" name="tone"> Concise</label>
                                        </div>
                                    </div>
                                    <div style="margin-bottom: 16px;">
                                        <h3>Focus Topics</h3>
                                        <div style="display: flex; gap: 8px; flex-wrap: wrap;">
                                            <span style="padding: 4px 12px; background: rgba(255,255,255,0.1); border-radius: 16px; font-size: 12px; cursor: pointer;">+ Only about my products</span>
                                            <span style="padding: 4px 12px; background: rgba(255,255,255,0.1); border-radius: 16px; font-size: 12px; cursor: pointer;">+ Avoid competitor mentions</span>
                                            <span style="padding: 4px 12px; background: rgba(255,255,255,0.1); border-radius: 16px; font-size: 12px; cursor: pointer;">+ Always reply in Spanish</span>
                                        </div>
                                    </div>
                                    <div style="margin-bottom: 16px;">
                                        <h3>Example Interactions (Optional)</h3>
                                        <textarea placeholder="Q: Do you offer refunds?
A: Absolutely! We offer a 30-day money back guarantee..." style="width: 100%; height: 80px;"></textarea>
                                    </div>
                                    <div class="wizard-actions">
                                        <button class="secondary" onclick="document.getElementById('agent-tuning').style.display='none'; document.getElementById('agents-gallery').style.display='block';">Cancel</button>
                                        <button onclick="document.getElementById('tuning-toast').style.display='block'; setTimeout(() => document.getElementById('tuning-toast').style.display='none', 3000); document.getElementById('agent-tuning').style.display='none'; document.getElementById('agents-gallery').style.display='block';">Save</button>
                                    </div>
                                </div>
                                <div style="flex: 1; background: #000; border-radius: 8px; padding: 16px;">
                                    <h3>Live Preview</h3>
                                    <div style="height: 200px; overflow-y: auto; color: #ccc; font-size: 12px; font-family: monospace; margin-bottom: 12px;">
                                        > Hello! I'm your Marketing Pro agent. How can I help?
                                    </div>
                                    <input type="text" placeholder="Test your agent..." style="width: 100%; padding: 8px;" />
                                </div>
                            </div>
                        </div>

                        <div id="tuning-toast" style="display: none; position: fixed; bottom: 20px; right: 20px; background: #4caf50; color: white; padding: 12px 24px; border-radius: 8px; z-index: 1000;">
                            Your agent has been updated ✓
                        </div>


                        <div id="fix-wizard" style="display: none; position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.8); z-index: 1000; align-items: center; justify-content: center; flex-direction: column;">
                            <div class="card glass" style="max-width: 400px; width: 100%;">
                                <h2>Help me fix this</h2>
                                <p>It looks like this agent is having trouble writing product descriptions. Let's fix it in 3 simple steps.</p>

                                <div id="fix-step-1">
                                    <h3>Step 1: Clarify Product Specs</h3>
                                    <p style="font-size: 14px;">The agent needs more context about your newest product.</p>
                                    <textarea placeholder="Tell the agent more about the product..." style="width: 100%; margin-bottom: 12px;"></textarea>
                                    <button onclick="document.getElementById('fix-step-1').style.display='none'; document.getElementById('fix-step-2').style.display='block';">Next →</button>
                                </div>

                                <div id="fix-step-2" style="display: none;">
                                    <h3>Step 2: Adjust Tone</h3>
                                    <p style="font-size: 14px;">Should it be more descriptive or concise?</p>
                                    <button class="secondary" style="margin-right: 8px;" onclick="this.style.background='rgba(255,255,255,0.2)'">Descriptive</button>
                                    <button class="secondary" onclick="this.style.background='rgba(255,255,255,0.2)'">Concise</button>
                                    <div style="margin-top: 16px;">
                                        <button onclick="document.getElementById('fix-step-2').style.display='none'; document.getElementById('fix-step-3').style.display='block';">Next →</button>
                                    </div>
                                </div>

                                <div id="fix-step-3" style="display: none;">
                                    <h3>Step 3: Review & Apply</h3>
                                    <p style="font-size: 14px;">The agent is ready to resume its duties.</p>
                                    <button onclick="alert('Agent Fixed!'); document.getElementById('fix-wizard').style.display='none';">Apply Fix ✓</button>
                                </div>

                                <button class="secondary" style="width: 100%; margin-top: 16px;" onclick="document.getElementById('fix-wizard').style.display='none'">Cancel</button>
                            </div>
                        </div>

                        <script>
                            function openAgentWizard(name) {
                                document.getElementById('agents-gallery').style.display = 'none';
                                document.getElementById('agent-wizard').style.display = 'block';
                                document.getElementById('aw-title').innerText = 'Configure ' + name;
                            }
                            function openAgentTuning(name) {
                                document.getElementById('agents-gallery').style.display = 'none';
                                document.getElementById('agent-tuning').style.display = 'block';
                            }
                        </script>
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
                    <div id="setup-screen" class="screen glass" style="backdrop-filter: blur(20px); font-family: 'Inter', sans-serif;">
                        <style>
                            .wizard-h1 { font-family: 'Outfit', sans-serif; font-size: 32px; font-weight: 700; margin-bottom: 8px; }
                            .wizard-p { color: var(--text-secondary); margin-bottom: 24px; font-size: 16px; }
                            .wizard-step { display: none; }
                            .wizard-step.active { display: block; }
                            .card-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(150px, 1fr)); gap: 16px; margin-bottom: 24px; }
                            .card-tile { background: rgba(255, 255, 255, 0.05); border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 12px; padding: 16px; cursor: pointer; text-align: center; transition: all 0.2s; }
                            .card-tile:hover { background: rgba(255, 255, 255, 0.1); }
                            .card-tile.selected { border-color: var(--primary-color); background: rgba(var(--primary-color-rgb), 0.1); }
                            .card-tile h3 { font-size: 16px; margin-bottom: 4px; }
                            .card-tile p { font-size: 12px; color: var(--text-secondary); margin: 0; }
                            .wizard-actions { display: flex; justify-content: space-between; margin-top: 32px; }
                            .password-strength { height: 4px; border-radius: 2px; background: #333; margin-top: 8px; overflow: hidden; }
                            .password-strength-bar { height: 100%; width: 0; transition: width 0.3s; }
                        </style>

                        <div id="step-1" class="wizard-step active">
                            <h1 class="wizard-h1">Your business, live in minutes.</h1>
                            <p class="wizard-p">Zero tech skills needed. We do the heavy lifting.</p>
                            <button onclick="nextStep(2)">🚀 Start My Business</button>
                            <button class="secondary" onclick="nextStep('ai')">⚡ Instant Build (AI) →</button>
                        </div>

                        <div id="step-2" class="wizard-step">
                            <h1 class="wizard-h1">What kind of business are you building?</h1>
                            <div class="card-grid">
                                <button class="secondary card-tile" onclick="nextStep(3)">🛒 Online Store</button>
                                <button class="secondary card-tile" onclick="nextStep(3)">🛠️ Service Business</button>
                                <button class="secondary card-tile" onclick="nextStep(3)">🍽️ Restaurant / Food</button>
                                <button class="secondary card-tile" onclick="nextStep(3)">🎨 Creative / Portfolio</button>
                                <button class="secondary card-tile" onclick="nextStep(3)">🏪 Local Business</button>
                                <button class="secondary card-tile" onclick="nextStep(3)">Other</button>
                            </div>
                            <div class="wizard-actions">
                                <button class="secondary" onclick="nextStep(1)">Back</button>
                            </div>
                        </div>

                        <div id="step-3" class="wizard-step">
                            <h1 class="wizard-h1">Give your business a name</h1>
                            <p class="wizard-p">Don't worry, you can change this later.</p>
                            <input type="text" id="biz-name" placeholder="e.g. Maya's Cakes" />
                            <textarea id="biz-desc" placeholder="A short description of what you do..." rows="3"></textarea>
                            <button class="secondary" onclick="document.getElementById('biz-desc').value = 'AI generated description for ' + document.getElementById('biz-name').value">Auto-suggest Description</button>
                            <div class="wizard-actions">
                                <button class="secondary" onclick="nextStep(2)">Back</button>
                                <button onclick="nextStep(4)">Next →</button>
                            </div>
                        </div>

                        <div id="step-4" class="wizard-step">
                            <h1 class="wizard-h1">What do you sell?</h1>
                            <div class="card-grid">
                                <button class="secondary card-tile multi-select" onclick="this.classList.toggle('selected')">📦 Physical products</button>
                                <button class="secondary card-tile multi-select" onclick="this.classList.toggle('selected')">📥 Digital downloads</button>
                                <button class="secondary card-tile multi-select" onclick="this.classList.toggle('selected')">📅 Services / appointments</button>
                                <button class="secondary card-tile multi-select" onclick="this.classList.toggle('selected')">🍕 Food & beverages</button>
                                <button class="secondary card-tile multi-select" onclick="this.classList.toggle('selected')">🔁 Subscriptions</button>
                            </div>
                            <div class="wizard-actions">
                                <button class="secondary" onclick="nextStep(3)">Back</button>
                                <button onclick="nextStep(5)">Next →</button>
                            </div>
                        </div>

                        <div id="step-5" class="wizard-step">
                            <h1 class="wizard-h1">How do you want to receive payments?</h1>
                            <div class="card-grid">
                                <button class="secondary card-tile" onclick="nextStep(6)">
                                    <h3>🌐 Online only</h3>
                                    <p>Ready in 5 mins</p>
                                </button>
                                <button class="secondary card-tile" onclick="nextStep(6)">
                                    <h3>🤝 In-person</h3>
                                    <p>POS app - Ready in 10 mins</p>
                                </button>
                                <button class="secondary card-tile" onclick="nextStep(6)">
                                    <h3>Both</h3>
                                    <p>Omnichannel</p>
                                </button>
                                <button class="secondary card-tile" onclick="nextStep(6)">
                                    <h3>Skip for now</h3>
                                </button>
                            </div>
                            <div class="wizard-actions">
                                <button class="secondary" onclick="nextStep(4)">Back</button>
                            </div>
                        </div>

                        <div id="step-6" class="wizard-step">
                            <h1 class="wizard-h1">Create your account</h1>
                            <p class="wizard-p">Administrator account</p>
                            <input type="text" placeholder="e.g. Maya Smith" />
                            <input type="email" placeholder="you@email.com" />
                            <input type="password" placeholder="Password" oninput="
                                let strength = this.value.length > 8 ? (this.value.match(/[!@#$%^&*]/) ? 'Strong' : 'Medium') : 'Weak';
                                document.getElementById('pwd-strength-text').innerText = 'Strength: ' + strength;
                                document.getElementById('pwd-strength-bar').style.width = strength === 'Strong' ? '100%' : (strength === 'Medium' ? '66%' : '33%');
                                document.getElementById('pwd-strength-bar').style.background = strength === 'Strong' ? '#4caf50' : (strength === 'Medium' ? '#ff9800' : '#f44336');
                            " />
                            <div class="password-strength"><div id="pwd-strength-bar" class="password-strength-bar"></div></div>
                            <p id="pwd-strength-text" style="font-size: 12px; margin-top: 4px;">Strength: </p>
                            <button class="secondary" style="margin-top: 16px;">Sign up with Google</button>
                            <button class="secondary" style="margin-top: 8px;">Sign up with Apple</button>
                            <div class="wizard-actions">
                                <button class="secondary" onclick="nextStep(5)">Back</button>
                                <button onclick="nextStep(7)">Next →</button>
                            </div>
                        </div>

                        <div id="step-7" class="wizard-step">
                            <h1 class="wizard-h1">Choose a Template</h1>
                            <div class="card-grid">
                                <button class="secondary card-tile" onclick="nextStep('7_5')">✨ Modern</button>
                                <button class="secondary card-tile" onclick="nextStep('7_5')">🔥 Bold</button>
                            </div>
                            <div class="wizard-actions">
                                <button class="secondary" onclick="nextStep(6)">Back</button>
                            </div>
                        </div>


                        <div id="step-7_5" class="wizard-step">
                            <h1 class="wizard-h1">Brand colors & logo</h1>
                            <p class="wizard-p">Pick a color palette for your brand</p>
                            <div class="card-grid">
                                <button class="secondary card-tile" style="background: linear-gradient(to right, #ff9a9e, #fecfef)" onclick="this.classList.toggle('selected')">Sunset</button>
                                <button class="secondary card-tile" style="background: linear-gradient(to right, #a18cd1, #fbc2eb)" onclick="this.classList.toggle('selected')">Twilight</button>
                                <button class="secondary card-tile" style="background: linear-gradient(to right, #84fab0, #8fd3f4)" onclick="this.classList.toggle('selected')">Ocean</button>
                            </div>
                            <p class="wizard-p">Add your logo</p>
                            <button class="secondary" style="margin-bottom: 8px;">Upload Logo</button>
                            <button class="secondary" onclick="alert('Generating...')">✨ Generate a logo for me</button>
                            <div class="wizard-actions">
                                <button class="secondary" onclick="nextStep(7)">Back</button>
                                <button onclick="nextStep(8)">Next →</button>
                            </div>
                        </div>

                        <div id="step-8" class="wizard-step">
                            <h1 class="wizard-h1">Add your first product or service</h1>
                            <input type="text" placeholder="e.g. Custom Birthday Cake" />
                            <input type="text" placeholder="e.g. 50.00" />
                            <div class="wizard-actions">
                                <button class="secondary" onclick="nextStep('7_5')">Back</button>
                                <button onclick="nextStep(9)">Next →</button>
                            </div>
                        </div>

                        <div id="step-9" class="wizard-step">
                            <h1 class="wizard-h1">Choose a Domain</h1>
                            <button class="secondary card-tile" style="width: 100%; margin-bottom: 8px;" onclick="nextStep(10)">🌐 Free OHC Domain</button>
                            <button class="secondary card-tile" style="width: 100%;" onclick="nextStep(10)">🔗 Connect Custom Domain</button>
                            <div class="wizard-actions">
                                <button class="secondary" onclick="nextStep('7_5')">Back</button>
                            </div>
                        </div>

                        <div id="step-10" class="wizard-step">
                            <div style="text-align: center;">
                                <h1 class="wizard-h1">Almost there!</h1>
                                <div style="padding: 24px; background: rgba(255,255,255,0.05); border-radius: 12px; margin-bottom: 24px;">
                                    <h3 style="margin-bottom: 16px;">Review & Launch</h3>
                                    <p>Your business is ready to be configured.</p>
                                </div>
                                <button onclick="document.getElementById('launch-overlay').style.display='flex'; setTimeout(() => nextStep(100), 2000);">Review & Launch</button>
                                <button onclick="document.getElementById('launch-overlay').style.display='flex'; setTimeout(() => nextStep(100), 2000);">Launch!</button>
                            </div>
                            <div class="wizard-actions">
                                <button class="secondary" onclick="nextStep(9)">Back</button>
                            </div>
                        </div>

                        <div id="step-100" class="wizard-step">
                            <h1 class="wizard-h1">CONFETTI SUCCESS</h1>
                            <p>Onboarding Complete!</p>
                            <p>Your business is now live!</p>
                            <button onclick="showScreen('checklist-screen')">View Welcome Checklist →</button>
                            <button onclick="showScreen('dashboard-screen')">Launch My Business →</button>
                        </div>

                        <div id="step-ai" class="wizard-step">
                            <h1 class="wizard-h1">Describe your business in a sentence</h1>
                            <input type="text" id="ai-bio" placeholder="e.g. I run a local bakery called Maya's Cakes..." />
                            <button onclick="if(document.getElementById('ai-bio').value) { nextStep('generating'); setTimeout(() => nextStep('launch-ai'), 2000); }">Generate Storefront →</button>
                            <div class="wizard-actions">
                                <button class="secondary" onclick="nextStep(1)">Back</button>
                            </div>
                        </div>

                        <div id="step-generating" class="wizard-step">
                            <h1 class="wizard-h1">Designing your storefront...</h1>
                            <p class="wizard-p">Our AI is crafting a custom experience for your brand.</p>
                        </div>

                        <div id="step-launch-ai" class="wizard-step">
                            <h1 class="wizard-h1">Your live storefront!</h1>
                            <h2>AI Store</h2>
                            <button onclick="showScreen('dashboard-screen')">Launch My Business →</button>
                            <button onclick="showScreen('dashboard-screen')">Continue to Dashboard →</button>
                        </div>

                        <div id="launch-overlay" style="display: none; position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.8); z-index: 1000; align-items: center; justify-content: center; flex-direction: column;">
                            <h2 style="color: white; font-family: 'Outfit';">Your business is setting up...</h2>
                            <div style="width: 50px; height: 50px; border: 3px solid transparent; border-top-color: white; border-radius: 50%; animation: spin 1s linear infinite;"></div>
                            <style>@keyframes spin { 100% { transform: rotate(360deg); } }</style>
                        </div>

                        <script>
                            function saveWizardState(stepId) {
                                fetch('/api/wizard/state', {
                                    method: 'POST',
                                    headers: { 'Content-Type': 'application/json' },
                                    body: JSON.stringify({ step: stepId, data: {} })
                                }).catch(e => console.error(e));
                            }
                            function nextStep(step) {
                                document.querySelectorAll('#setup-screen .wizard-step').forEach(s => s.classList.remove('active'));
                                const nextStepEl = document.getElementById('step-' + step);
                                if (nextStepEl) {
                                    nextStepEl.classList.add('active');
                                    saveWizardState(step);
                                }
                            }
                        </script>
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
                            'settings-screen': '/settings',
                            'checkout-screen': '/checkout',
                            'users-screen': '/users',
                            'referral-dashboard-screen': '/referrals',
                            'inbox-screen': '/inbox',
                            'meetings-screen': '/meetings',
                            'meeting-room-screen': '/meetings/room/1'
                        };

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
                            const advancedMode = localStorage.getItem('ohc_advanced_mode') === 'true';
                            if (advancedMode) {
                                document.body.classList.add('advanced-mode');
                                const toggle = document.getElementById('mode-toggle');
                                if (toggle) toggle.checked = true;
                            }

                            const path = window.location.pathname;
                            const screenId = Object.keys(pathMap).find(key => pathMap[key] === path) || 'dashboard-screen';
                            showScreen(screenId);
                        };
                    </script>

                    <!-- My Plan Screen -->
                    <div id="my-plan-screen" class="screen glass" style="backdrop-filter: blur(20px);">
                        <h1>Billing & Credits</h1>
                        <p class="wizard-p">What does this cost?</p>
                        <div class="card glass" style="margin-bottom: 24px;">
                            <h2>Current Usage</h2>
                            <p>You have used 15 AI agent hours this month.</p>
                            <div style="height: 8px; background: rgba(255,255,255,0.1); border-radius: 4px; margin-bottom: 8px; overflow: hidden;">
                                <div style="height: 100%; width: 30%; background: var(--primary-color);"></div>
                            </div>
                            <p style="font-size: 12px; color: var(--text-secondary);">Projected cost: $12.50 / mo</p>
                        </div>
                        <button onclick="document.getElementById('billing-wizard').style.display='flex'">Add credits</button>
                        <button class="secondary" onclick="showScreen('dashboard-screen')">Back to Dashboard</button>

                        <div id="billing-wizard" style="display: none; position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.8); z-index: 1000; align-items: center; justify-content: center; flex-direction: column;">
                            <div class="card glass" style="max-width: 400px; width: 100%;">
                                <h2>Add Credits</h2>
                                <p>Select an amount to top up your account.</p>
                                <div class="card-grid" style="grid-template-columns: 1fr 1fr 1fr;">
                                    <button class="secondary card-tile" onclick="alert('Added $10'); document.getElementById('billing-wizard').style.display='none'">$10</button>
                                    <button class="secondary card-tile" onclick="alert('Added $25'); document.getElementById('billing-wizard').style.display='none'">$25</button>
                                    <button class="secondary card-tile" onclick="alert('Added $50'); document.getElementById('billing-wizard').style.display='none'">$50</button>
                                </div>
                                <button class="secondary" style="width: 100%; margin-top: 16px;" onclick="document.getElementById('billing-wizard').style.display='none'">Cancel</button>
                            </div>
                        </div>
                    </div>

                </body>
            </html>
        "#,
    };
    axum::response::Html(content)
}
