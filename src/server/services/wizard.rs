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
}

use crate::ohc::orchestration::hub_service_server::HubService;

// ... existing code ...
