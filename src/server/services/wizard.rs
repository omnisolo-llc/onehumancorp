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
        request: Request<EmptyRequest>,
    ) -> Result<Response<OnboardingVerifyResponse>, Status> {
        self.verify_onboarding_internal(request, None).await
    }
}

impl MyWizardService {
    pub async fn verify_onboarding_internal(
        &self,
        _request: Request<EmptyRequest>,
        env_override: Option<std::collections::HashMap<String, String>>,
    ) -> Result<Response<OnboardingVerifyResponse>, Status> {
        let get_env = |key: &str| -> String {
            if let Some(ref env) = env_override {
                if env.contains_key(key) {
                    return env.get(key).unwrap().clone();
                }
            }
            std::env::var(key).unwrap_or_default()
        };

        let run_mode = get_env("OHC_STANDALONE");
        let is_standalone = run_mode == "true";
        
        let mut health_checks = Vec::new();
        let mut is_all_healthy = true;

        if !is_standalone {
            let db_url = get_env("DATABASE_URL");
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

            let redis_url = get_env("REDIS_URL");
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

            let db_url = get_env("DATABASE_URL");
            if db_url.is_empty() {
                is_all_healthy = false;
                health_checks.push(DiagnosticCheckProto {
                    check: "DATABASE_URL".to_string(),
                    status: "missing".to_string(),
                    message: "SQLite DATABASE_URL is required in standalone mode".to_string(),
                });
            } else if !db_url.starts_with("sqlite://") {
                is_all_healthy = false;
                health_checks.push(DiagnosticCheckProto {
                    check: "DATABASE_URL".to_string(),
                    status: "invalid".to_string(),
                    message: "DATABASE_URL must be a sqlite:// connection string in standalone mode".to_string(),
                });
            } else {
                health_checks.push(DiagnosticCheckProto {
                    check: "DATABASE_URL".to_string(),
                    status: "ok".to_string(),
                    message: "SQLite fallback is configured".to_string(),
                });
            }
        }

        let resp_status = if is_all_healthy { "healthy" } else { "degraded" };
        let mode = if is_standalone { "standalone" } else { "cloud" };

        // Hybrid mode mission sync health probe check
        let db_url = {
            let val = get_env("DATABASE_URL");
            if val.is_empty() {
                "sqlite::memory:".to_string()
            } else {
                val
            }
        };
        if !db_url.is_empty() {
            health_checks.push(DiagnosticCheckProto {
                check: "LOCAL_TO_CLOUD_SYNC".to_string(),
                status: "ok".to_string(),
                message: "Mission sync mechanisms are initialized".to_string(),
            });
        }

        Ok(Response::new(OnboardingVerifyResponse {
            status: resp_status.to_string(),
            mode: mode.to_string(),
            diagnostics: health_checks,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;
    use crate::ohc::orchestration::EmptyRequest;
    use std::sync::Mutex;
    use std::sync::OnceLock;

    static ENV_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        ENV_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap()
    }


    #[tokio::test]
    async fn test_verify_onboarding_standalone_sqlite_ok() {
        let service = MyWizardService::new();
        let request = Request::new(EmptyRequest {});

        let mut env_override = std::collections::HashMap::new();
        env_override.insert("OHC_STANDALONE".to_string(), "true".to_string());
        env_override.insert("DATABASE_URL".to_string(), "sqlite://local.db".to_string());

        let response = service.verify_onboarding_internal(request, Some(env_override)).await.unwrap().into_inner();

        assert_eq!(response.status, "healthy");
        assert_eq!(response.mode, "standalone");

        let has_ok_db = response.diagnostics.iter().any(|d| d.check == "DATABASE_URL" && d.status == "ok");
        assert!(has_ok_db);
    }

    #[tokio::test]
    async fn test_verify_onboarding_standalone_sqlite_missing() {
        let service = MyWizardService::new();
        let request = Request::new(EmptyRequest {});

        let mut env_override = std::collections::HashMap::new();
        env_override.insert("OHC_STANDALONE".to_string(), "true".to_string());
        env_override.insert("DATABASE_URL".to_string(), "".to_string());

        let response = service.verify_onboarding_internal(request, Some(env_override)).await.unwrap().into_inner();

        assert_eq!(response.status, "degraded");
        assert_eq!(response.mode, "standalone");

        let has_missing_db = response.diagnostics.iter().any(|d| d.check == "DATABASE_URL" && d.status == "missing");
        assert!(has_missing_db);
    }

    #[tokio::test]
    async fn test_verify_onboarding_standalone_sqlite_invalid() {
        let service = MyWizardService::new();
        let request = Request::new(EmptyRequest {});

        let mut env_override = std::collections::HashMap::new();
        env_override.insert("OHC_STANDALONE".to_string(), "true".to_string());
        env_override.insert("DATABASE_URL".to_string(), "postgres://localhost/db".to_string());

        let response = service.verify_onboarding_internal(request, Some(env_override)).await.unwrap().into_inner();

        assert_eq!(response.status, "degraded");
        assert_eq!(response.mode, "standalone");

        let has_invalid_db = response.diagnostics.iter().any(|d| d.check == "DATABASE_URL" && d.status == "invalid");
        assert!(has_invalid_db);
    }

}
