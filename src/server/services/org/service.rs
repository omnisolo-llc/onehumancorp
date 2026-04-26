use tonic::{Request, Response, Status};
use crate::ohc::orchestration::*;
use crate::ohc::orchestration::org_service_server::OrgService;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

pub struct MyOrgService {
    hub: Arc<crate::hub::Hub>,
    settings: RwLock<SettingsResponse>,
}

impl MyOrgService {
    pub fn new(hub: Arc<crate::hub::Hub>) -> Self {
        MyOrgService {
            hub,
            settings: RwLock::new(SettingsResponse {
                minimax_api_key: std::env::var("MINIMAX_API_KEY").unwrap_or_default(),
                extras: HashMap::new(),
            }),
        }
    }
}

#[tonic::async_trait]
impl OrgService for MyOrgService {
    async fn get_domains(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<DomainsResponse>, Status> {
        let domains = vec![
            DomainInfoProto { id: "software_company".to_string(), name: "Software Company".to_string(), description: "Full-stack engineering org...".to_string() },
            DomainInfoProto { id: "digital_marketing_agency".to_string(), name: "Digital Marketing Agency".to_string(), description: "Full-service agency...".to_string() },
            DomainInfoProto { id: "accounting_firm".to_string(), name: "Accounting Firm".to_string(), description: "Financial services firm...".to_string() },
        ];
        Ok(Response::new(DomainsResponse { domains }))
    }

    async fn get_settings(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<SettingsResponse>, Status> {
        let settings = self.settings.read().unwrap();
        Ok(Response::new(settings.clone()))
    }

    async fn update_settings(
        &self,
        request: Request<UpdateSettingsRequest>,
    ) -> Result<Response<SettingsResponse>, Status> {
        let req = request.into_inner();
        let mut settings = self.settings.write().unwrap();
        settings.minimax_api_key = req.minimax_api_key;
        settings.extras = req.extras;
        Ok(Response::new(settings.clone()))
    }

    async fn get_marketplace_items(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<MarketplaceItemsResponse>, Status> {
        let items = vec![
            MarketplaceItemProto { id: "git-mcp".to_string(), name: "Git".to_string(), r#type: "tool".to_string(), author: "system".to_string(), description: "Git operations".to_string(), downloads: 100, rating: 4.5, tags: vec!["code".to_string()] },
        ];
        Ok(Response::new(MarketplaceItemsResponse { items }))
    }

    async fn get_analytics(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<AnalyticsSummaryResponse>, Status> {
        Ok(Response::new(AnalyticsSummaryResponse {
            human_agent_ratio: 1.5,
            total_agents: 15,
            total_humans: 10,
            audit_fidelity_pct: 95.0,
            resumption_latency_ms: 4800,
            pending_approvals: 2,
            active_handoffs: 1,
            token_velocity: 10000,
        }))
    }
}
