use tonic::{Request, Response, Status};
use ::server_ohc::orchestration::*;
use ::server_ohc::orchestration::wizard_service_server::WizardService;
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
mod wizard_impl;


#[cfg(test)]
#[path = "wizard_tests.rs"]
mod tests;
