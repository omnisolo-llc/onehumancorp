pub mod mock_config;
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

                        /* Skeleton Shimmer Loading State */
                        @keyframes shimmer {
                            0% { background-position: -1000px 0; }
                            100% { background-position: 1000px 0; }
                        }
                        .skeleton {
                            background: linear-gradient(90deg, rgba(255,255,255,0.03) 25%, rgba(255,255,255,0.08) 50%, rgba(255,255,255,0.03) 75%);
                            background-size: 1000px 100%;
                            animation: shimmer 2s infinite cubic-bezier(0.4, 0, 0.2, 1);
                            border-radius: 8px;
                        }
                        .skeleton-title { height: 28px; width: 60%; margin-bottom: 16px; }
                        .skeleton-text { height: 16px; width: 100%; margin-bottom: 12px; }
                        .skeleton-text-short { height: 16px; width: 80%; margin-bottom: 24px; }
                        .skeleton-card { height: 150px; width: 100%; margin-bottom: 16px; }

                        /* Glassmorphism UI */
                        .glass {
                            background: rgba(255, 255, 255, 0.05);
                            backdrop-filter: blur(20px) saturate(200%);
                            border: 1px solid rgba(255, 255, 255, 0.1);
                            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
                        }

                        /* Async Loading Overlay */
                        .async-loading-overlay {
                            position: absolute;
                            top: 0; left: 0; right: 0; bottom: 0;
                            background: rgba(244, 247, 250, 0.8);
                            backdrop-filter: blur(10px) saturate(150%);
                            z-index: 50;
                            display: flex;
                            flex-direction: column;
                            padding: 24px;
                            opacity: 0;
                            pointer-events: none;
                            transition: opacity 200ms cubic-bezier(0.4, 0, 0.2, 1);
                        }
                        .async-loading-overlay.active {
                            opacity: 1;
                            pointer-events: all;
                            transition: opacity 300ms cubic-bezier(0.4, 0, 0.2, 1);
                        }
                        .async-loading-container {
                            position: relative;
                            min-height: 200px;
                        }

                        /* Dashboard Layout Fixes for 375px */
                        @media (max-width: 768px) {
                            #dashboard-screen .metric-card {
                                width: 100% !important;
                                min-width: 100% !important;
                                margin-bottom: 16px;
                            }
                            #dashboard-screen .grid {
                                display: block;
                            }
                            /* Touch Targets */
                            button, .nav-item, input[type="text"], select {
                                min-height: 44px;
                                padding: 10px 16px;
                            }
                        }


                        /* Skeleton Shimmer Loading State */
                        @keyframes shimmer {
                            0% { background-position: -1000px 0; }
                            100% { background-position: 1000px 0; }
                        }
                        .skeleton {
                            background: linear-gradient(90deg, rgba(255,255,255,0.03) 25%, rgba(255,255,255,0.08) 50%, rgba(255,255,255,0.03) 75%);
                            background-size: 1000px 100%;
                            animation: shimmer 2s infinite cubic-bezier(0.4, 0, 0.2, 1);
                            border-radius: 8px;
                        }
                        .skeleton-title { height: 28px; width: 60%; margin-bottom: 16px; }
                        .skeleton-text { height: 16px; width: 100%; margin-bottom: 12px; }
                        .skeleton-text-short { height: 16px; width: 80%; margin-bottom: 24px; }
                        .skeleton-card { height: 150px; width: 100%; margin-bottom: 16px; }

                        /* Glassmorphism UI */
                        .glass {
                            background: rgba(255, 255, 255, 0.05);
                            backdrop-filter: blur(20px) saturate(200%);
                            border: 1px solid rgba(255, 255, 255, 0.1);
                            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
                        }

                        /* Async Loading Overlay */
                        .async-loading-overlay {
                            position: absolute;
                            top: 0; left: 0; right: 0; bottom: 0;
                            background: rgba(244, 247, 250, 0.8);
                            backdrop-filter: blur(10px) saturate(150%);
                            z-index: 50;
                            display: flex;
                            flex-direction: column;
                            padding: 24px;
                            opacity: 0;
                            pointer-events: none;
                            transition: opacity 200ms cubic-bezier(0.4, 0, 0.2, 1);
                        }
                        .async-loading-overlay.active {
                            opacity: 1;
                            pointer-events: all;
                            transition: opacity 300ms cubic-bezier(0.4, 0, 0.2, 1);
                        }
                        .async-loading-container {
                            position: relative;
                            min-height: 200px;
                        }

                        /* Dashboard Layout Fixes for 375px */
                        @media (max-width: 768px) {
                            #dashboard-screen .metric-card {
                                width: 100% !important;
                                min-width: 100% !important;
                                margin-bottom: 16px;
                            }
                            #dashboard-screen .grid {
                                display: block;
                            }
                            /* Touch Targets */
                            button, .nav-item, input[type="text"], select {
                                min-height: 44px;
                                padding: 10px 16px;
                            }
                        }

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
                    <div id="dashboard-screen" class="screen async-loading-container">
                        <div id="dashboard-content">
                            <h2 style="font-family: 'Outfit', sans-serif;">Welcome to your Dashboard</h2>
                            <p>Here is your business at a glance.</p>

                            <div class="grid" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 24px; margin-top: 24px;">
                                <div class="card metric-card glass" style="padding: 24px; border-radius: 12px;">
                                    <h3 style="font-family: 'Outfit'; color: var(--text-secondary); margin-top:0;">Today's Sales</h3>
                                    <p style="font-size: 2.5rem; font-weight: 600; margin: 8px 0; color: var(--primary);">$1,240.00</p>
                                    <p style="color: #10B981; margin: 0; font-size: 0.875rem;">↑ 12% from yesterday</p>
                                </div>
                                <div class="card metric-card glass" style="padding: 24px; border-radius: 12px;">
                                    <h3 style="font-family: 'Outfit'; color: var(--text-secondary); margin-top:0;">Active Orders</h3>
                                    <p style="font-size: 2.5rem; font-weight: 600; margin: 8px 0;">42</p>
                                    <button class="secondary" onclick="simulateAsyncLoad('dashboard-screen')" style="margin-top: 12px; width: 100%;">View All Orders</button>
                                </div>
                                <div class="card metric-card glass" style="padding: 24px; border-radius: 12px;">
                                    <h3 style="font-family: 'Outfit'; color: var(--text-secondary); margin-top:0;">New Messages</h3>
                                    <p style="font-size: 2.5rem; font-weight: 600; margin: 8px 0;">5</p>
                                    <button class="secondary" onclick="showScreen('inbox-screen')" style="margin-top: 12px; width: 100%;">Check Messages</button>
                                </div>
                            </div>
                        </div>
                        <div id="dashboard-loading" class="async-loading-overlay glass">
                            <div class="skeleton skeleton-title"></div>
                            <div class="skeleton skeleton-text"></div>
                            <div class="grid" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 24px; margin-top: 24px;">
                                <div class="skeleton skeleton-card"></div>
                                <div class="skeleton skeleton-card"></div>
                                <div class="skeleton skeleton-card"></div>
                            </div>
                        </div>
                    </div>
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
                            <p>Projected Cost this cycle: $1.23</p>
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


                        function simulateAsyncLoad(containerId) {
                            const overlay = document.querySelector('#' + containerId + ' .async-loading-overlay');
                            if(overlay) {
                                overlay.classList.add('active');
                                setTimeout(() => {
                                    overlay.classList.remove('active');
                                }, 1500);
                            }
                        }


                        function simulateAsyncLoad(containerId) {
                            const overlay = document.querySelector('#' + containerId + ' .async-loading-overlay');
                            if(overlay) {
                                overlay.classList.add('active');
                                setTimeout(() => {
                                    overlay.classList.remove('active');
                                }, 1500);
                            }
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

// --- ORGANIC MOCK DATA FOR UX TESTING ---
pub mod ux_mock_data {
    pub struct Order {
        pub id: String,
        pub amount: u32,
        pub status: String,
    }
    pub fn get_mock_orders() -> Vec<Order> {
        vec![
            Order {
                id: "6dd5763f-c8c3-4d2b-9a64-f0f126d86148".to_string(),
                amount: 0,
                status: "processing".to_string(),
            },
            Order {
                id: "291c69ee-c335-4415-a3a2-1eb646d82917".to_string(),
                amount: 10,
                status: "processing".to_string(),
            },
            Order {
                id: "51910723-88b8-4733-b48e-add7c1a4ed8b".to_string(),
                amount: 20,
                status: "processing".to_string(),
            },
            Order {
                id: "2a0213c7-012d-4e63-8e9f-7cbd61891647".to_string(),
                amount: 30,
                status: "processing".to_string(),
            },
            Order {
                id: "37a829d3-494c-4402-b5ef-0ee77590ad36".to_string(),
                amount: 40,
                status: "processing".to_string(),
            },
            Order {
                id: "2868bb77-46fe-4795-a352-64680256e939".to_string(),
                amount: 50,
                status: "processing".to_string(),
            },
            Order {
                id: "f2519075-e29f-459d-9261-cba45abf8774".to_string(),
                amount: 60,
                status: "processing".to_string(),
            },
            Order {
                id: "655b3dab-aaa4-444a-ac30-606daf9fbaef".to_string(),
                amount: 70,
                status: "processing".to_string(),
            },
            Order {
                id: "647fe3c7-c0c9-4a23-887b-62ab221f9cbe".to_string(),
                amount: 80,
                status: "processing".to_string(),
            },
            Order {
                id: "a61aad4c-03d2-49f8-87a8-8ef65fb79325".to_string(),
                amount: 90,
                status: "processing".to_string(),
            },
            Order {
                id: "e76ceb23-e0a2-434c-8ca7-d6a8f53c54b6".to_string(),
                amount: 100,
                status: "processing".to_string(),
            },
            Order {
                id: "74ca763d-c5db-401e-9e24-85119ac797c2".to_string(),
                amount: 110,
                status: "processing".to_string(),
            },
            Order {
                id: "0246b034-28b8-4aa1-a19d-9f2691c8acad".to_string(),
                amount: 120,
                status: "processing".to_string(),
            },
            Order {
                id: "f5c537d2-8675-49ba-a7d8-22f10df85cf7".to_string(),
                amount: 130,
                status: "processing".to_string(),
            },
            Order {
                id: "aacfc295-c08d-4333-b7c6-3b8b36a56f65".to_string(),
                amount: 140,
                status: "processing".to_string(),
            },
            Order {
                id: "bc5a808f-0c7d-401b-9a42-787277dcdaf9".to_string(),
                amount: 150,
                status: "processing".to_string(),
            },
            Order {
                id: "73cb33c9-841f-4b0f-bc14-1147fddaaf01".to_string(),
                amount: 160,
                status: "processing".to_string(),
            },
            Order {
                id: "a8fe031f-8ea5-4142-bcd3-67cfefc69654".to_string(),
                amount: 170,
                status: "processing".to_string(),
            },
            Order {
                id: "16793f2a-4a39-4d65-b6fb-66158ec57e08".to_string(),
                amount: 180,
                status: "processing".to_string(),
            },
            Order {
                id: "ae99c80a-1ad8-485b-8632-1b0c5e50bfac".to_string(),
                amount: 190,
                status: "processing".to_string(),
            },
            Order {
                id: "a3bdd744-44c8-495a-ba29-2dc396667e5d".to_string(),
                amount: 200,
                status: "processing".to_string(),
            },
            Order {
                id: "970b05c0-ec89-4385-a8fb-2a3ff013525e".to_string(),
                amount: 210,
                status: "processing".to_string(),
            },
            Order {
                id: "fb1c4d2b-18a2-4ab0-a6f4-26ac22938412".to_string(),
                amount: 220,
                status: "processing".to_string(),
            },
            Order {
                id: "7cf76bbc-4513-4e58-8650-0f9594501f01".to_string(),
                amount: 230,
                status: "processing".to_string(),
            },
            Order {
                id: "0916b447-8d05-41da-a2cb-13caa6153925".to_string(),
                amount: 240,
                status: "processing".to_string(),
            },
            Order {
                id: "2a0f8930-d4d1-473f-843f-4dd4825c00ac".to_string(),
                amount: 250,
                status: "processing".to_string(),
            },
            Order {
                id: "a878e59d-1293-45bf-9e87-ff931fcb046a".to_string(),
                amount: 260,
                status: "processing".to_string(),
            },
            Order {
                id: "d5fbce69-a7e1-408b-b1d6-478a8cbb09be".to_string(),
                amount: 270,
                status: "processing".to_string(),
            },
            Order {
                id: "ac895d88-f296-4d9a-a6c0-75ca778f7e4f".to_string(),
                amount: 280,
                status: "processing".to_string(),
            },
            Order {
                id: "891c01c1-33cd-42c9-a27a-150cb7996654".to_string(),
                amount: 290,
                status: "processing".to_string(),
            },
            Order {
                id: "86654ee7-2e44-4ea6-89ce-afe101b4df82".to_string(),
                amount: 300,
                status: "processing".to_string(),
            },
            Order {
                id: "adbfd6f6-da61-4b55-8b03-28c3a8faa650".to_string(),
                amount: 310,
                status: "processing".to_string(),
            },
            Order {
                id: "4222a337-b8fc-4514-b7a5-ca3e26802554".to_string(),
                amount: 320,
                status: "processing".to_string(),
            },
            Order {
                id: "ba0486f9-52db-411a-88d0-b85ba6fdb2cb".to_string(),
                amount: 330,
                status: "processing".to_string(),
            },
            Order {
                id: "ae45feaa-ac54-4421-9d1e-e943b20e1ae0".to_string(),
                amount: 340,
                status: "processing".to_string(),
            },
            Order {
                id: "b464829c-f237-49c2-80b7-21d26c884c2b".to_string(),
                amount: 350,
                status: "processing".to_string(),
            },
            Order {
                id: "98fc7121-7277-495c-8b67-d7b7babb5259".to_string(),
                amount: 360,
                status: "processing".to_string(),
            },
            Order {
                id: "3dea63f8-1722-47f7-90af-daa5ab19dc54".to_string(),
                amount: 370,
                status: "processing".to_string(),
            },
            Order {
                id: "35e8f9da-6666-495c-a1c0-98bf77e1d938".to_string(),
                amount: 380,
                status: "processing".to_string(),
            },
            Order {
                id: "db6205bd-3c90-461b-aea3-44fe488450ff".to_string(),
                amount: 390,
                status: "processing".to_string(),
            },
            Order {
                id: "4b2f932c-283d-41fc-986d-8ca18f722405".to_string(),
                amount: 400,
                status: "processing".to_string(),
            },
            Order {
                id: "2c2eaacc-8116-46bc-9f38-84e939f395e1".to_string(),
                amount: 410,
                status: "processing".to_string(),
            },
            Order {
                id: "e1766354-17c6-40dc-9da7-d93de6366c3d".to_string(),
                amount: 420,
                status: "processing".to_string(),
            },
            Order {
                id: "ba616365-cb58-4f9a-b124-a60ab9ae8a77".to_string(),
                amount: 430,
                status: "processing".to_string(),
            },
            Order {
                id: "b76e5d2c-5099-4779-8841-1e24bc9bd5e3".to_string(),
                amount: 440,
                status: "processing".to_string(),
            },
            Order {
                id: "f2d92e63-4ec1-4da7-a6d6-9b9097ec226e".to_string(),
                amount: 450,
                status: "processing".to_string(),
            },
            Order {
                id: "4c4a9b87-6888-4286-ae6a-926031192a7e".to_string(),
                amount: 460,
                status: "processing".to_string(),
            },
            Order {
                id: "2e37cbc2-d741-40b4-86d1-472f048241dd".to_string(),
                amount: 470,
                status: "processing".to_string(),
            },
            Order {
                id: "9d2f25f8-4c3a-4442-9433-3cf3a2bd12bd".to_string(),
                amount: 480,
                status: "processing".to_string(),
            },
            Order {
                id: "cb757de6-3e03-41d2-a19d-452d30fa4aa4".to_string(),
                amount: 490,
                status: "processing".to_string(),
            },
            Order {
                id: "3283dfd1-df55-43f9-900b-772bfc1fe2e1".to_string(),
                amount: 500,
                status: "processing".to_string(),
            },
            Order {
                id: "5b57f413-2957-4173-b178-1108aa81732e".to_string(),
                amount: 510,
                status: "processing".to_string(),
            },
            Order {
                id: "f60e7597-2f32-46cd-95f1-829d6adb5231".to_string(),
                amount: 520,
                status: "processing".to_string(),
            },
            Order {
                id: "ca1e74da-9737-47fa-b9e8-f8e01b80b5a7".to_string(),
                amount: 530,
                status: "processing".to_string(),
            },
            Order {
                id: "27b05a81-b98b-4170-90a1-0d06e8367ce2".to_string(),
                amount: 540,
                status: "processing".to_string(),
            },
            Order {
                id: "b45bf93c-2c48-4f22-bbbc-4a5dfe0a4ca3".to_string(),
                amount: 550,
                status: "processing".to_string(),
            },
            Order {
                id: "5b69340a-c05e-40bd-878d-26987cc553d9".to_string(),
                amount: 560,
                status: "processing".to_string(),
            },
            Order {
                id: "46cf7025-3329-4639-a6ac-cf66dd3bf1bb".to_string(),
                amount: 570,
                status: "processing".to_string(),
            },
            Order {
                id: "46b1458e-ba33-426e-9844-fc141c3f3e45".to_string(),
                amount: 580,
                status: "processing".to_string(),
            },
            Order {
                id: "b101d6a2-9e77-4dfb-839a-0fbb21418488".to_string(),
                amount: 590,
                status: "processing".to_string(),
            },
            Order {
                id: "9cb2342e-c3d9-48b9-af94-c3f752320489".to_string(),
                amount: 600,
                status: "processing".to_string(),
            },
            Order {
                id: "50d6099e-3fac-434c-b356-7004b09a52e1".to_string(),
                amount: 610,
                status: "processing".to_string(),
            },
            Order {
                id: "bb304092-a792-4443-9e0c-b291446fa421".to_string(),
                amount: 620,
                status: "processing".to_string(),
            },
            Order {
                id: "bf81f0d7-fe8d-4e7c-aec5-38b4abf1dd08".to_string(),
                amount: 630,
                status: "processing".to_string(),
            },
            Order {
                id: "f83faa09-fe5f-4c74-bf22-14d49bfd9528".to_string(),
                amount: 640,
                status: "processing".to_string(),
            },
            Order {
                id: "e60ea4dd-3176-401b-92c3-965354df3dc2".to_string(),
                amount: 650,
                status: "processing".to_string(),
            },
            Order {
                id: "ba373d2c-a5a3-4b0c-a273-5940101da19a".to_string(),
                amount: 660,
                status: "processing".to_string(),
            },
            Order {
                id: "62c449ab-0b37-4f14-8dbe-f57fb79d29c8".to_string(),
                amount: 670,
                status: "processing".to_string(),
            },
            Order {
                id: "e7e52b40-c889-48fb-8942-11f2fe152b0f".to_string(),
                amount: 680,
                status: "processing".to_string(),
            },
            Order {
                id: "83c0b359-1b3e-4271-ae53-587cdcbf8d66".to_string(),
                amount: 690,
                status: "processing".to_string(),
            },
            Order {
                id: "ee6638fd-b83c-4ac1-907a-64c74b39b675".to_string(),
                amount: 700,
                status: "processing".to_string(),
            },
            Order {
                id: "c653a1c0-3606-4038-ac2b-d7f92faaaadb".to_string(),
                amount: 710,
                status: "processing".to_string(),
            },
            Order {
                id: "1a788bcd-9cda-42d5-b9df-f94d951147b2".to_string(),
                amount: 720,
                status: "processing".to_string(),
            },
            Order {
                id: "7c118a22-eeb5-4eb5-b8b2-50434ff04e5f".to_string(),
                amount: 730,
                status: "processing".to_string(),
            },
            Order {
                id: "36dd8014-f215-4723-a8dc-76e73b4b17b5".to_string(),
                amount: 740,
                status: "processing".to_string(),
            },
            Order {
                id: "bfe3a72c-debe-42d6-9f80-b4672221a67e".to_string(),
                amount: 750,
                status: "processing".to_string(),
            },
            Order {
                id: "58d59cae-20fd-4fee-a72a-8e031450c9af".to_string(),
                amount: 760,
                status: "processing".to_string(),
            },
            Order {
                id: "b4d6bb48-51b2-45a0-8085-91e1e2e90136".to_string(),
                amount: 770,
                status: "processing".to_string(),
            },
            Order {
                id: "40b7dffc-c0ba-426e-8e9b-8c3c6ea41e88".to_string(),
                amount: 780,
                status: "processing".to_string(),
            },
            Order {
                id: "932a0146-d10f-4b6c-8a17-5d719b38f5f6".to_string(),
                amount: 790,
                status: "processing".to_string(),
            },
            Order {
                id: "05e4fd3f-6ba6-4511-a530-18437ac4bb6e".to_string(),
                amount: 800,
                status: "processing".to_string(),
            },
            Order {
                id: "81a06dec-2269-4278-a4ee-46d133a193fa".to_string(),
                amount: 810,
                status: "processing".to_string(),
            },
            Order {
                id: "c15ce41f-b28d-4cee-a707-5c9518a52e34".to_string(),
                amount: 820,
                status: "processing".to_string(),
            },
            Order {
                id: "2befce56-e681-46a0-9827-6f75ae729ef1".to_string(),
                amount: 830,
                status: "processing".to_string(),
            },
            Order {
                id: "836edbea-352a-4fd5-aa16-0f576cb7e73d".to_string(),
                amount: 840,
                status: "processing".to_string(),
            },
            Order {
                id: "786f79d0-f8c5-40a8-a057-7766a01d4b46".to_string(),
                amount: 850,
                status: "processing".to_string(),
            },
            Order {
                id: "373e6e62-a358-4301-93b2-d2227820aa09".to_string(),
                amount: 860,
                status: "processing".to_string(),
            },
            Order {
                id: "46ab5768-b79f-42f1-b557-b46a20689541".to_string(),
                amount: 870,
                status: "processing".to_string(),
            },
            Order {
                id: "f93028b0-6119-4300-85e5-6b639f31387b".to_string(),
                amount: 880,
                status: "processing".to_string(),
            },
            Order {
                id: "36d09f40-c386-4868-bc35-e9e7314c2ed1".to_string(),
                amount: 890,
                status: "processing".to_string(),
            },
            Order {
                id: "eea0ed58-8c81-4452-a770-76a2e6dadbe5".to_string(),
                amount: 900,
                status: "processing".to_string(),
            },
            Order {
                id: "5fbf2d97-dbd0-4660-943c-5298d46c3fd3".to_string(),
                amount: 910,
                status: "processing".to_string(),
            },
            Order {
                id: "e1d79f4a-de0b-43e8-8a05-f3e38635cc1c".to_string(),
                amount: 920,
                status: "processing".to_string(),
            },
            Order {
                id: "79e5a8b3-7e0a-4510-be44-560f2a9240aa".to_string(),
                amount: 930,
                status: "processing".to_string(),
            },
            Order {
                id: "cbfcaf8d-1e56-4e30-8461-f9acdb7cce51".to_string(),
                amount: 940,
                status: "processing".to_string(),
            },
            Order {
                id: "54ed0c87-e2a7-4430-b345-dc1589feb05c".to_string(),
                amount: 950,
                status: "processing".to_string(),
            },
            Order {
                id: "4a991c68-70fb-427c-8a29-7022a8389f7b".to_string(),
                amount: 960,
                status: "processing".to_string(),
            },
            Order {
                id: "4169907c-5aab-481a-ba31-8e9fe4dc4d63".to_string(),
                amount: 970,
                status: "processing".to_string(),
            },
            Order {
                id: "07369ed3-19bc-4f6e-8eee-57b42c570a1c".to_string(),
                amount: 980,
                status: "processing".to_string(),
            },
            Order {
                id: "58b22814-342e-463a-96e3-0025e9db8f54".to_string(),
                amount: 990,
                status: "processing".to_string(),
            },
            Order {
                id: "7852e250-31a2-41fa-983e-03d8361ce27a".to_string(),
                amount: 1000,
                status: "processing".to_string(),
            },
            Order {
                id: "1d4d2c43-16cb-4843-a96d-7bbc2a868686".to_string(),
                amount: 1010,
                status: "processing".to_string(),
            },
            Order {
                id: "45d8a751-887c-4914-9095-857944ee9db7".to_string(),
                amount: 1020,
                status: "processing".to_string(),
            },
            Order {
                id: "b3950595-2ff7-4e1b-82fb-b10d1e0c1d69".to_string(),
                amount: 1030,
                status: "processing".to_string(),
            },
            Order {
                id: "a655890b-d385-4e6a-bc8a-c68e35a7c721".to_string(),
                amount: 1040,
                status: "processing".to_string(),
            },
            Order {
                id: "c782921c-b139-443c-8cc1-e28b7533f4b3".to_string(),
                amount: 1050,
                status: "processing".to_string(),
            },
            Order {
                id: "a7bf8dee-45e7-48bd-9d5b-e8699425eb8c".to_string(),
                amount: 1060,
                status: "processing".to_string(),
            },
            Order {
                id: "505f3edf-e99c-4742-b49f-5449e1817c66".to_string(),
                amount: 1070,
                status: "processing".to_string(),
            },
            Order {
                id: "7c961771-97b3-4076-bf94-e2d4387ecaf6".to_string(),
                amount: 1080,
                status: "processing".to_string(),
            },
            Order {
                id: "10b700f9-a511-4950-83c4-09f5807c6ef0".to_string(),
                amount: 1090,
                status: "processing".to_string(),
            },
            Order {
                id: "39b9414d-cce2-4da1-a504-5669030f44f2".to_string(),
                amount: 1100,
                status: "processing".to_string(),
            },
            Order {
                id: "6d266019-cdb2-4581-b8a7-c1cd05cdcfde".to_string(),
                amount: 1110,
                status: "processing".to_string(),
            },
            Order {
                id: "bb2ea47b-dd15-4fcd-bb6d-0eb0a99e423e".to_string(),
                amount: 1120,
                status: "processing".to_string(),
            },
            Order {
                id: "de7f6957-44ab-418d-9c84-c3f1e4f9d206".to_string(),
                amount: 1130,
                status: "processing".to_string(),
            },
            Order {
                id: "f0ee2013-9d1d-4778-864d-5b0dcb1cb9e9".to_string(),
                amount: 1140,
                status: "processing".to_string(),
            },
            Order {
                id: "d09bd924-8c28-4929-a6a8-bf780110a44e".to_string(),
                amount: 1150,
                status: "processing".to_string(),
            },
            Order {
                id: "c0afce88-2c4a-4382-8b45-fa4f475153c4".to_string(),
                amount: 1160,
                status: "processing".to_string(),
            },
            Order {
                id: "eff57971-c4f1-446f-acb0-85bc3544ca0b".to_string(),
                amount: 1170,
                status: "processing".to_string(),
            },
            Order {
                id: "7870e85a-86cb-4b3f-a3c1-80a69745a2aa".to_string(),
                amount: 1180,
                status: "processing".to_string(),
            },
            Order {
                id: "ff0898e4-efe3-46e4-9192-41bd3c928865".to_string(),
                amount: 1190,
                status: "processing".to_string(),
            },
            Order {
                id: "9cee5cef-815b-4e84-9712-60f5239ef0de".to_string(),
                amount: 1200,
                status: "processing".to_string(),
            },
            Order {
                id: "997c0c78-da77-48b1-a74b-6acd49fbadb9".to_string(),
                amount: 1210,
                status: "processing".to_string(),
            },
            Order {
                id: "d7527b8f-6b0c-4066-9882-7fdc362eeb82".to_string(),
                amount: 1220,
                status: "processing".to_string(),
            },
            Order {
                id: "0676a7eb-7248-4d74-a781-fd57fef6bb9a".to_string(),
                amount: 1230,
                status: "processing".to_string(),
            },
            Order {
                id: "a6f4e088-3866-43ca-9cd0-01416d9a34ea".to_string(),
                amount: 1240,
                status: "processing".to_string(),
            },
            Order {
                id: "60abbfd8-eb74-4c0f-8ab9-bd4fd54db839".to_string(),
                amount: 1250,
                status: "processing".to_string(),
            },
            Order {
                id: "02159f8e-83b7-406c-8941-b7ac3d1ea712".to_string(),
                amount: 1260,
                status: "processing".to_string(),
            },
            Order {
                id: "2e6e35f0-c75b-47b7-8246-15f085a4d6a3".to_string(),
                amount: 1270,
                status: "processing".to_string(),
            },
            Order {
                id: "f1a511ec-0318-4dd5-840e-7b2b7902153e".to_string(),
                amount: 1280,
                status: "processing".to_string(),
            },
            Order {
                id: "e9916831-808f-479a-9637-db74a4fff6da".to_string(),
                amount: 1290,
                status: "processing".to_string(),
            },
            Order {
                id: "92371c80-b0aa-4593-98ba-1b80924bd9fe".to_string(),
                amount: 1300,
                status: "processing".to_string(),
            },
            Order {
                id: "e46f372c-61a4-4d22-b405-14860a21931f".to_string(),
                amount: 1310,
                status: "processing".to_string(),
            },
            Order {
                id: "f34e258e-9342-4631-a1cb-be6ca738e7ce".to_string(),
                amount: 1320,
                status: "processing".to_string(),
            },
            Order {
                id: "0d6de1a4-3f3b-42e1-be5f-f2d3ec4f0ee5".to_string(),
                amount: 1330,
                status: "processing".to_string(),
            },
            Order {
                id: "0eafe9f1-1145-4ab3-8820-1054ccbbbddb".to_string(),
                amount: 1340,
                status: "processing".to_string(),
            },
            Order {
                id: "3dd048e6-e38c-4392-9c8a-81a35f2218be".to_string(),
                amount: 1350,
                status: "processing".to_string(),
            },
            Order {
                id: "c0648f9c-2c8c-4561-a2fa-bdc4480ef1a2".to_string(),
                amount: 1360,
                status: "processing".to_string(),
            },
            Order {
                id: "0eeef9bc-9229-4646-a735-709322f0b221".to_string(),
                amount: 1370,
                status: "processing".to_string(),
            },
            Order {
                id: "15ba89af-661b-4456-92f4-fac5f76084d6".to_string(),
                amount: 1380,
                status: "processing".to_string(),
            },
            Order {
                id: "514293b1-3a63-4115-863b-2b60321aea3c".to_string(),
                amount: 1390,
                status: "processing".to_string(),
            },
            Order {
                id: "cad9eeba-43d0-4494-a9c3-5b9abee2c8da".to_string(),
                amount: 1400,
                status: "processing".to_string(),
            },
            Order {
                id: "86ff0b28-c941-431b-a775-3f124a155e9e".to_string(),
                amount: 1410,
                status: "processing".to_string(),
            },
            Order {
                id: "ec40b0ec-d162-4981-9284-b6321c24c3ca".to_string(),
                amount: 1420,
                status: "processing".to_string(),
            },
            Order {
                id: "8dce9529-5eaa-491c-b794-cbe1cef774ea".to_string(),
                amount: 1430,
                status: "processing".to_string(),
            },
            Order {
                id: "bb751b59-de59-4551-ad2d-bb95893293d0".to_string(),
                amount: 1440,
                status: "processing".to_string(),
            },
            Order {
                id: "584ca856-a814-48b0-9409-35336bb40a82".to_string(),
                amount: 1450,
                status: "processing".to_string(),
            },
            Order {
                id: "4df854d3-365f-4065-bc1f-b5f32abf059e".to_string(),
                amount: 1460,
                status: "processing".to_string(),
            },
            Order {
                id: "25fa73a0-e09d-487c-a809-9948b56b781e".to_string(),
                amount: 1470,
                status: "processing".to_string(),
            },
            Order {
                id: "1971916a-d31d-407e-b64e-b490eb1f8c4f".to_string(),
                amount: 1480,
                status: "processing".to_string(),
            },
            Order {
                id: "90d914f9-8dd7-414f-b2bf-994b68f6b972".to_string(),
                amount: 1490,
                status: "processing".to_string(),
            },
            Order {
                id: "0ff979a3-93c7-48f1-a036-bbedcf9bb767".to_string(),
                amount: 1500,
                status: "processing".to_string(),
            },
            Order {
                id: "5480cbf5-24fc-4cc0-b509-6aacc8f0dde5".to_string(),
                amount: 1510,
                status: "processing".to_string(),
            },
            Order {
                id: "f83ade69-4eed-4a9f-bbe3-386a6a22bedb".to_string(),
                amount: 1520,
                status: "processing".to_string(),
            },
            Order {
                id: "9fa597f7-a453-4adf-8232-a983a96b662e".to_string(),
                amount: 1530,
                status: "processing".to_string(),
            },
            Order {
                id: "b782863c-7967-4c6c-8614-b4a942a482d0".to_string(),
                amount: 1540,
                status: "processing".to_string(),
            },
            Order {
                id: "7553a0ad-93ea-4ba7-a1cb-1d13d7d1912a".to_string(),
                amount: 1550,
                status: "processing".to_string(),
            },
            Order {
                id: "f8547a66-4a0c-4e00-ac09-426288f06db3".to_string(),
                amount: 1560,
                status: "processing".to_string(),
            },
            Order {
                id: "b1897bf5-1906-40f2-9cbb-3296e98321ad".to_string(),
                amount: 1570,
                status: "processing".to_string(),
            },
            Order {
                id: "ba45a953-2fc8-461f-9037-ddda43c4594a".to_string(),
                amount: 1580,
                status: "processing".to_string(),
            },
            Order {
                id: "349f3420-e696-4ce6-bb9e-bf7033176444".to_string(),
                amount: 1590,
                status: "processing".to_string(),
            },
            Order {
                id: "cb5b73b0-70c3-4fee-8490-f1d48fb55b2e".to_string(),
                amount: 1600,
                status: "processing".to_string(),
            },
            Order {
                id: "3b5f8ca4-4463-4edd-8c06-d3c605d1ce92".to_string(),
                amount: 1610,
                status: "processing".to_string(),
            },
            Order {
                id: "1906094b-f271-49f0-9da1-1a70273797a0".to_string(),
                amount: 1620,
                status: "processing".to_string(),
            },
            Order {
                id: "62fc635c-d697-43d0-bf80-7585f4736d64".to_string(),
                amount: 1630,
                status: "processing".to_string(),
            },
            Order {
                id: "dabb97b8-2e6f-466b-a5d1-7e4c80577287".to_string(),
                amount: 1640,
                status: "processing".to_string(),
            },
            Order {
                id: "441d119f-fe3d-4249-8b0b-752fb15e4d06".to_string(),
                amount: 1650,
                status: "processing".to_string(),
            },
            Order {
                id: "3efcc947-d6c9-46e9-bf9e-8d38e6cb3354".to_string(),
                amount: 1660,
                status: "processing".to_string(),
            },
            Order {
                id: "c486fa78-9875-4553-b624-40768d80cc0b".to_string(),
                amount: 1670,
                status: "processing".to_string(),
            },
            Order {
                id: "f9ad7146-6f24-4e3e-b33a-76c3f3ea3084".to_string(),
                amount: 1680,
                status: "processing".to_string(),
            },
            Order {
                id: "34789a0f-9695-48a1-bf15-b80082491d0c".to_string(),
                amount: 1690,
                status: "processing".to_string(),
            },
            Order {
                id: "de51eb3d-aa3c-4930-b42c-7ff8cf6a6be9".to_string(),
                amount: 1700,
                status: "processing".to_string(),
            },
            Order {
                id: "508ce4a8-7e8e-44f2-b0c2-05cf1a75be20".to_string(),
                amount: 1710,
                status: "processing".to_string(),
            },
            Order {
                id: "8eabc114-1425-41b6-abf3-f93166fd4b56".to_string(),
                amount: 1720,
                status: "processing".to_string(),
            },
            Order {
                id: "6035209d-8ec6-4705-ae15-98627c14b38b".to_string(),
                amount: 1730,
                status: "processing".to_string(),
            },
            Order {
                id: "de757193-a6ec-4e1b-8453-d3228e3d9b71".to_string(),
                amount: 1740,
                status: "processing".to_string(),
            },
            Order {
                id: "cb311440-a884-46d6-af1d-527ae42ef921".to_string(),
                amount: 1750,
                status: "processing".to_string(),
            },
            Order {
                id: "ab4b00bd-5e99-46ba-980b-a834697a2231".to_string(),
                amount: 1760,
                status: "processing".to_string(),
            },
            Order {
                id: "066f1a07-d2f9-4d46-a3c4-67e675a7a608".to_string(),
                amount: 1770,
                status: "processing".to_string(),
            },
            Order {
                id: "ce8eae2b-2401-436e-b709-f3ca8f5b7a09".to_string(),
                amount: 1780,
                status: "processing".to_string(),
            },
            Order {
                id: "81c58a5e-3a68-4f1e-9443-4e97025458cb".to_string(),
                amount: 1790,
                status: "processing".to_string(),
            },
            Order {
                id: "4620065d-9740-43a2-b406-ae0f3f3c158d".to_string(),
                amount: 1800,
                status: "processing".to_string(),
            },
            Order {
                id: "1755c326-59da-4e2e-b363-10852ceb8c56".to_string(),
                amount: 1810,
                status: "processing".to_string(),
            },
            Order {
                id: "3dcebad8-5936-42cd-809a-acd2c3c4c5c4".to_string(),
                amount: 1820,
                status: "processing".to_string(),
            },
            Order {
                id: "a2e3cbf6-a4fd-46b3-be9b-ed26e675ebc1".to_string(),
                amount: 1830,
                status: "processing".to_string(),
            },
            Order {
                id: "67eb035b-70a7-4b30-b6c6-d12b6b1fe856".to_string(),
                amount: 1840,
                status: "processing".to_string(),
            },
            Order {
                id: "4507fd30-0116-4db4-a9d4-66d0f938afbc".to_string(),
                amount: 1850,
                status: "processing".to_string(),
            },
            Order {
                id: "e80a52f3-0d0a-4959-850f-92155be0fe7f".to_string(),
                amount: 1860,
                status: "processing".to_string(),
            },
            Order {
                id: "b5d9f9d0-f2b5-44ee-9ef3-d6afa7907dc3".to_string(),
                amount: 1870,
                status: "processing".to_string(),
            },
            Order {
                id: "144ecf89-2f66-40ab-9a0b-1578b8a587c5".to_string(),
                amount: 1880,
                status: "processing".to_string(),
            },
            Order {
                id: "a07213bc-c507-4e81-898a-3f54f54b6948".to_string(),
                amount: 1890,
                status: "processing".to_string(),
            },
            Order {
                id: "21660227-e058-4a81-a094-b28c22d28f89".to_string(),
                amount: 1900,
                status: "processing".to_string(),
            },
            Order {
                id: "03a7a1d7-89f8-4bea-b13a-4e6d78c25172".to_string(),
                amount: 1910,
                status: "processing".to_string(),
            },
            Order {
                id: "ca61f349-c445-44d2-879b-a095643b8e5d".to_string(),
                amount: 1920,
                status: "processing".to_string(),
            },
            Order {
                id: "036d23ca-0891-417b-8b11-2c7c1964e139".to_string(),
                amount: 1930,
                status: "processing".to_string(),
            },
            Order {
                id: "90643bbb-9c03-456a-912f-47fd3160418d".to_string(),
                amount: 1940,
                status: "processing".to_string(),
            },
            Order {
                id: "3e3302b3-e111-489c-9ce7-82113b8be61c".to_string(),
                amount: 1950,
                status: "processing".to_string(),
            },
            Order {
                id: "48dfe105-e8d9-4f27-a859-92c1e599c28e".to_string(),
                amount: 1960,
                status: "processing".to_string(),
            },
            Order {
                id: "67d430ee-54e8-4656-aa0a-a11c04d3e1c5".to_string(),
                amount: 1970,
                status: "processing".to_string(),
            },
            Order {
                id: "8162a8e6-5e3b-42c8-ba0a-826ee4e598f4".to_string(),
                amount: 1980,
                status: "processing".to_string(),
            },
            Order {
                id: "670eb638-d6e9-42e7-9f37-a70c9b8ddcb6".to_string(),
                amount: 1990,
                status: "processing".to_string(),
            },
            Order {
                id: "91f70bc3-6dfe-4f07-944a-ddfb1b21ca0d".to_string(),
                amount: 2000,
                status: "processing".to_string(),
            },
            Order {
                id: "ecd4e388-3bb0-49a6-8bbe-21d92990ffe7".to_string(),
                amount: 2010,
                status: "processing".to_string(),
            },
            Order {
                id: "a5f8e21b-1246-4fad-8e6f-f477b78e4e5b".to_string(),
                amount: 2020,
                status: "processing".to_string(),
            },
            Order {
                id: "a6592a04-8790-49c6-8011-b144e44984ad".to_string(),
                amount: 2030,
                status: "processing".to_string(),
            },
            Order {
                id: "36c530be-0159-49b5-b672-0d5161dd7f73".to_string(),
                amount: 2040,
                status: "processing".to_string(),
            },
            Order {
                id: "b8109c11-c33b-4a6b-920c-e20d5d5c18e5".to_string(),
                amount: 2050,
                status: "processing".to_string(),
            },
            Order {
                id: "ff364696-75af-4833-b92d-c39c2240198a".to_string(),
                amount: 2060,
                status: "processing".to_string(),
            },
            Order {
                id: "78067a5f-9eb0-4da6-9ba7-9d5d69f150c6".to_string(),
                amount: 2070,
                status: "processing".to_string(),
            },
            Order {
                id: "9c5bec2d-2fd2-4a4a-8cfc-1126606809cd".to_string(),
                amount: 2080,
                status: "processing".to_string(),
            },
            Order {
                id: "fd34a2b8-7eef-4b82-b4e3-8c286783e471".to_string(),
                amount: 2090,
                status: "processing".to_string(),
            },
            Order {
                id: "2505ccf2-afdc-4c30-ad06-02e33c14e784".to_string(),
                amount: 2100,
                status: "processing".to_string(),
            },
            Order {
                id: "677b711c-d81a-43a7-b836-7cb943b3833e".to_string(),
                amount: 2110,
                status: "processing".to_string(),
            },
            Order {
                id: "4fc52cc8-e6ee-4c3a-9ddd-7ffc7dcb81d8".to_string(),
                amount: 2120,
                status: "processing".to_string(),
            },
            Order {
                id: "0fa422ab-b437-4244-9703-88c1629e26bf".to_string(),
                amount: 2130,
                status: "processing".to_string(),
            },
            Order {
                id: "45b32773-9904-44f1-87d6-33ab0b018184".to_string(),
                amount: 2140,
                status: "processing".to_string(),
            },
            Order {
                id: "a6848fd8-8c93-4ac8-9e81-c348c8c93a1a".to_string(),
                amount: 2150,
                status: "processing".to_string(),
            },
            Order {
                id: "93f3a6f5-4855-4ba2-82eb-2ab734f776a3".to_string(),
                amount: 2160,
                status: "processing".to_string(),
            },
            Order {
                id: "84ef0164-b5d7-410e-acc6-bbcd2fd02326".to_string(),
                amount: 2170,
                status: "processing".to_string(),
            },
            Order {
                id: "5ecdbaed-55fb-4e2f-8286-460610bb9744".to_string(),
                amount: 2180,
                status: "processing".to_string(),
            },
            Order {
                id: "2a580f18-50da-4d76-b3ad-31b4bfc31058".to_string(),
                amount: 2190,
                status: "processing".to_string(),
            },
            Order {
                id: "b6669060-42e6-4019-a393-cfd40efc59ff".to_string(),
                amount: 2200,
                status: "processing".to_string(),
            },
            Order {
                id: "97d87bc2-481b-426c-bacc-dae02c3a79ed".to_string(),
                amount: 2210,
                status: "processing".to_string(),
            },
            Order {
                id: "305ae53a-33c6-4ac2-8023-8b00186aa24a".to_string(),
                amount: 2220,
                status: "processing".to_string(),
            },
            Order {
                id: "726aefb6-9b14-425a-9114-5838799cae45".to_string(),
                amount: 2230,
                status: "processing".to_string(),
            },
            Order {
                id: "5eebe88e-05a7-4cad-bc28-f94f59cd7f53".to_string(),
                amount: 2240,
                status: "processing".to_string(),
            },
            Order {
                id: "5bc12539-6b7a-41bd-9402-6052e56164d9".to_string(),
                amount: 2250,
                status: "processing".to_string(),
            },
            Order {
                id: "30dc3a5c-2077-4582-818c-72857a26c65a".to_string(),
                amount: 2260,
                status: "processing".to_string(),
            },
            Order {
                id: "73e93f63-204b-47e0-8654-06d2fb0fc6ce".to_string(),
                amount: 2270,
                status: "processing".to_string(),
            },
            Order {
                id: "ca6b514c-ca00-476f-adf9-0b8bf23984b2".to_string(),
                amount: 2280,
                status: "processing".to_string(),
            },
            Order {
                id: "006c4c43-6e7d-465c-8386-20e240c63f44".to_string(),
                amount: 2290,
                status: "processing".to_string(),
            },
            Order {
                id: "555bad5f-872c-455e-8fd8-54519b5dc913".to_string(),
                amount: 2300,
                status: "processing".to_string(),
            },
            Order {
                id: "8bfac4a9-409c-4ff7-b527-bb2ffb38c679".to_string(),
                amount: 2310,
                status: "processing".to_string(),
            },
            Order {
                id: "90f84920-35cd-401a-aace-554ccc8e42a2".to_string(),
                amount: 2320,
                status: "processing".to_string(),
            },
            Order {
                id: "a78b798b-01df-42a1-a4d5-521ed7f7d078".to_string(),
                amount: 2330,
                status: "processing".to_string(),
            },
            Order {
                id: "500c4ff8-4824-48e7-bf9f-0a08182c3a8e".to_string(),
                amount: 2340,
                status: "processing".to_string(),
            },
            Order {
                id: "6e277e25-ae74-48ee-a311-e10768cd9375".to_string(),
                amount: 2350,
                status: "processing".to_string(),
            },
            Order {
                id: "8b5447b3-53eb-4420-884d-d61e6cc4feb8".to_string(),
                amount: 2360,
                status: "processing".to_string(),
            },
            Order {
                id: "37e8ddba-dfb8-468e-aa67-42605c2adf50".to_string(),
                amount: 2370,
                status: "processing".to_string(),
            },
            Order {
                id: "d8ae257d-44ff-4698-bd1d-367764d89704".to_string(),
                amount: 2380,
                status: "processing".to_string(),
            },
            Order {
                id: "5323766a-bae5-43c5-9a91-17fbc0cdc461".to_string(),
                amount: 2390,
                status: "processing".to_string(),
            },
            Order {
                id: "00e82f64-c8a5-4f5b-8c21-c4303243ee6b".to_string(),
                amount: 2400,
                status: "processing".to_string(),
            },
            Order {
                id: "73fbb57b-c307-42c9-95cf-05c4dbb97caf".to_string(),
                amount: 2410,
                status: "processing".to_string(),
            },
            Order {
                id: "99499ccc-a766-4992-a568-cb4a575716f9".to_string(),
                amount: 2420,
                status: "processing".to_string(),
            },
            Order {
                id: "90d97356-6fbb-43a7-b341-283440b7d54e".to_string(),
                amount: 2430,
                status: "processing".to_string(),
            },
            Order {
                id: "16ba0e9b-ba18-4ff2-a32c-7188b13723bb".to_string(),
                amount: 2440,
                status: "processing".to_string(),
            },
            Order {
                id: "3c4a80e5-e1f1-414e-a301-7d2f8ea44ed7".to_string(),
                amount: 2450,
                status: "processing".to_string(),
            },
            Order {
                id: "e2024c68-0fd8-4fd7-93bc-739f5cbb04bb".to_string(),
                amount: 2460,
                status: "processing".to_string(),
            },
            Order {
                id: "8e4488d3-4d7c-4983-99de-66856f1333da".to_string(),
                amount: 2470,
                status: "processing".to_string(),
            },
            Order {
                id: "c003ca6b-2605-4a7c-9be3-719a8bbed842".to_string(),
                amount: 2480,
                status: "processing".to_string(),
            },
            Order {
                id: "5b879812-a658-4d83-847a-9dd4369b3f83".to_string(),
                amount: 2490,
                status: "processing".to_string(),
            },
        ]
    }
}
