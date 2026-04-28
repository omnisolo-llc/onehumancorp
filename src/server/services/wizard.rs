use tonic::{Request, Response, Status};
use crate::ohc::orchestration::*;
use crate::ohc::orchestration::wizard_service_server::WizardService;
use std::sync::RwLock;

pub struct MyWizardService {
    settings: RwLock<WizardConfigureRequest>,
}

impl MyWizardService {
    pub fn new() -> Self {
        MyWizardService {
            settings: RwLock::new(WizardConfigureRequest {
                listen_addr: "".to_string(),
                db_path: "".to_string(),
                postgres_url: "".to_string(),
                redis_url: "".to_string(),
                centrifuge_url: "".to_string(),
                minimax_api_key: "".to_string(),
                extras: std::collections::HashMap::new(),
                ai_providers: vec![],
            }),
        }
    }
}

#[tonic::async_trait]
impl WizardService for MyWizardService {
    async fn get_wizard_status(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<WizardStatusProtoResponse>, Status> {
        let cfg = self.settings.read().unwrap();
        
        let has_enabled_provider = cfg.ai_providers.iter().any(|p| p.enabled);
        
        let steps = WizardStepsProto {
            server: !cfg.listen_addr.is_empty() && !cfg.db_path.is_empty(),
            ai_provider: has_enabled_provider,
            centrifuge: !cfg.centrifuge_url.is_empty(),
        };
        
        let configured = steps.server && steps.ai_provider && steps.centrifuge;
        
        Ok(Response::new(WizardStatusProtoResponse {
            configured,
            steps: Some(steps),
        }))
    }

    async fn configure_wizard(
        &self,
        request: Request<WizardConfigureRequest>,
    ) -> Result<Response<WizardStatusProtoResponse>, Status> {
        let req = request.into_inner();
        let mut cfg = self.settings.write().unwrap();
        
        if !req.listen_addr.is_empty() {
            cfg.listen_addr = req.listen_addr;
        }
        if !req.db_path.is_empty() {
            cfg.db_path = req.db_path;
        }
        if !req.postgres_url.is_empty() {
            cfg.postgres_url = req.postgres_url;
        }
        if !req.redis_url.is_empty() {
            cfg.redis_url = req.redis_url;
        }
        if !req.centrifuge_url.is_empty() {
            cfg.centrifuge_url = req.centrifuge_url;
        }
        if !req.minimax_api_key.is_empty() {
            cfg.minimax_api_key = req.minimax_api_key;
        }
        
        for (k, v) in req.extras {
            cfg.extras.insert(k, v);
        }
        
        if !req.ai_providers.is_empty() {
            cfg.ai_providers = req.ai_providers;
        }

        let has_enabled_provider = cfg.ai_providers.iter().any(|p| p.enabled);
        
        let steps = WizardStepsProto {
            server: !cfg.listen_addr.is_empty() && !cfg.db_path.is_empty(),
            ai_provider: has_enabled_provider,
            centrifuge: !cfg.centrifuge_url.is_empty(),
        };
        
        let configured = steps.server && steps.ai_provider && steps.centrifuge;
        
        Ok(Response::new(WizardStatusProtoResponse {
            configured,
            steps: Some(steps),
        }))
    }

    async fn verify_onboarding(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<OnboardingVerifyResponse>, Status> {
        let run_mode = std::env::var("OHC_STANDALONE").unwrap_or_default();
        let is_standalone = run_mode == "true";
        
        let mut health_checks = Vec::new();
        let mut is_all_healthy = true;

        if !is_standalone {
            let db_url = std::env::var("DATABASE_URL").unwrap_or_default();
            if db_url.is_empty() {
                is_all_healthy = false;
                health_checks.push(DiagnosticCheckProto {
                    check: "DATABASE_URL".to_string(),
                    status: "missing".to_string(),
                    message: "DATABASE_URL is required in cloud mode".to_string(),
                });
            } else {
                health_checks.push(DiagnosticCheckProto {
                    check: "DATABASE_URL".to_string(),
                    status: "ok".to_string(),
                    message: "DATABASE_URL is configured".to_string(),
                });
            }

            let redis_url = std::env::var("REDIS_URL").unwrap_or_default();
            if redis_url.is_empty() {
                is_all_healthy = false;
                health_checks.push(DiagnosticCheckProto {
                    check: "REDIS_URL".to_string(),
                    status: "missing".to_string(),
                    message: "REDIS_URL is required in cloud mode".to_string(),
                });
            } else {
                health_checks.push(DiagnosticCheckProto {
                    check: "REDIS_URL".to_string(),
                    status: "ok".to_string(),
                    message: "REDIS_URL is configured".to_string(),
                });
            }
        } else {
            health_checks.push(DiagnosticCheckProto {
                check: "OHC_STANDALONE".to_string(),
                status: "ok".to_string(),
                message: "Standalone mode active".to_string(),
            });
        }

        let resp_status = if is_all_healthy { "healthy" } else { "degraded" };
        let mode = if is_standalone { "standalone" } else { "cloud" };

        Ok(Response::new(OnboardingVerifyResponse {
            status: resp_status.to_string(),
            mode: mode.to_string(),
            diagnostics: health_checks,
        }))
    }

    async fn setup_business(
        &self,
        request: Request<BusinessSetupRequest>,
    ) -> Result<Response<BusinessSetupResponse>, Status> {
        let req = request.into_inner();
        let mut cfg = self.settings.write().unwrap();
        cfg.extras.insert("business_type".to_string(), req.business_type);
        cfg.extras.insert("company_name".to_string(), req.company_name);
        Ok(Response::new(BusinessSetupResponse { success: true }))
    }

    async fn build_website(
        &self,
        request: Request<WebsiteBuilderRequest>,
    ) -> Result<Response<WebsiteBuilderResponse>, Status> {
        let req = request.into_inner();
        let mut cfg = self.settings.write().unwrap();
        cfg.extras.insert("website_template".to_string(), req.selected_template);
        Ok(Response::new(WebsiteBuilderResponse { success: true }))
    }

    async fn configure_agent(
        &self,
        request: Request<AgentConfigRequest>,
    ) -> Result<Response<AgentConfigResponse>, Status> {
        let req = request.into_inner();
        let mut cfg = self.settings.write().unwrap();
        cfg.extras.insert("agent_role".to_string(), req.agent_role);
        Ok(Response::new(AgentConfigResponse { success: true }))
    }

    async fn tune_prompt(
        &self,
        request: Request<PromptTuningRequest>,
    ) -> Result<Response<PromptTuningResponse>, Status> {
        let req = request.into_inner();
        let mut cfg = self.settings.write().unwrap();
        cfg.extras.insert("prompt_tone".to_string(), req.tone);
        Ok(Response::new(PromptTuningResponse { success: true }))
    }

    async fn grow_business(
        &self,
        request: Request<GrowBusinessRequest>,
    ) -> Result<Response<GrowBusinessResponse>, Status> {
        let req = request.into_inner();
        let mut cfg = self.settings.write().unwrap();
        cfg.extras.insert("growth_strategy".to_string(), req.selected_strategy);
        Ok(Response::new(GrowBusinessResponse { success: true }))
    }

    async fn configure_billing(
        &self,
        request: Request<BillingWizardRequest>,
    ) -> Result<Response<BillingWizardResponse>, Status> {
        let req = request.into_inner();
        let mut cfg = self.settings.write().unwrap();
        cfg.extras.insert("billing_api_key".to_string(), req.api_key);
        Ok(Response::new(BillingWizardResponse { success: true }))
    }

    async fn fix_issue(
        &self,
        request: Request<FixWizardRequest>,
    ) -> Result<Response<FixWizardResponse>, Status> {
        let _req = request.into_inner();
        Ok(Response::new(FixWizardResponse { success: true }))
    }
}
