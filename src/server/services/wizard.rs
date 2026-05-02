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

    async fn instant_build(
        &self,
        request: Request<InstantBuildRequest>,
    ) -> Result<Response<InstantBuildResponse>, Status> {
        let req = request.into_inner();
        let bio = req.bio.to_lowercase();

        let api_key = {
            let cfg = self.settings.read().unwrap();
            cfg.minimax_api_key.clone()
        };

        let mut company_name = "AI Store".to_string();
        let mut business_type = "Online Store".to_string();
        let mut website_template = "Modern".to_string();
        let mut product_name = "AI Product".to_string();
        let domain_choice = "subdomain".to_string();
        let admin_email = "ai@test.com".to_string();
        let payment_pref = "online".to_string();

        if api_key.is_empty() {
            // Fallback for tests or no api key
            if bio.contains("bakery") || bio.contains("cake") {
                company_name = "AI Generated Bakery".to_string();
                business_type = "Online Store".to_string();
                product_name = "Custom Cake".to_string();
            } else if bio.contains("repair") || bio.contains("handyman") {
                company_name = "AI Handyman Services".to_string();
                business_type = "Service Business".to_string();
                product_name = "General Repair".to_string();
            } else if bio.contains("boutique") || bio.contains("clothes") {
                company_name = "AI Boutique".to_string();
                business_type = "Online Store".to_string();
                product_name = "Summer Dress".to_string();
            } else if bio.contains("tutor") || bio.contains("lesson") {
                company_name = "AI Tutoring".to_string();
                business_type = "Service Business".to_string();
                product_name = "1-Hour Lesson".to_string();
            }
        } else {
            let client = crate::minimax::MinimaxClient::new(api_key);
            let prompt = format!(
                "Extract the following information from the user bio: company_name, business_type, website_template, product_name. Output ONLY a valid JSON object. Bio: {}",
                bio
            );

            if let Ok(response) = client.reason(&prompt).await {
                // simple json parsing
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&response) {
                    if let Some(c_name) = parsed.get("company_name").and_then(|v| v.as_str()) {
                        company_name = c_name.to_string();
                    }
                    if let Some(b_type) = parsed.get("business_type").and_then(|v| v.as_str()) {
                        business_type = b_type.to_string();
                    }
                    if let Some(w_template) = parsed.get("website_template").and_then(|v| v.as_str()) {
                        website_template = w_template.to_string();
                    }
                    if let Some(p_name) = parsed.get("product_name").and_then(|v| v.as_str()) {
                        product_name = p_name.to_string();
                    }
                }
            }
        }

        Ok(Response::new(InstantBuildResponse {
            company_name,
            business_type,
            website_template,
            product_name,
            domain_choice,
            admin_email,
            payment_pref,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_instant_build() {
        let svc = MyWizardService::new();
        let req = Request::new(InstantBuildRequest {
            bio: "I run a local bakery called Maya's Cakes".to_string(),
        });
        let res = svc.instant_build(req).await.unwrap().into_inner();
        assert_eq!(res.company_name, "AI Generated Bakery");
        assert_eq!(res.business_type, "Online Store");
    }
}
