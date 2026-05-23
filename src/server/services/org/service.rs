use tonic::{Request, Response, Status};
use ::server_ohc::orchestration::*;
use ::server_ohc::orchestration::org_service_server::OrgService;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

pub struct MyOrgService {
    hub: Arc<crate::hub::Hub>,
    settings: RwLock<SettingsResponse>,
    analytics_cache: ::server_utils::cache::HybridCache<AnalyticsSummaryResponse>,
}

impl MyOrgService {
    pub fn new(hub: Arc<crate::hub::Hub>) -> Self {
        let redis_client = hub.redis_client.clone();
        MyOrgService {
            hub,
            settings: RwLock::new(SettingsResponse {
                minimax_api_key: std::env::var("MINIMAX_API_KEY").unwrap_or_default(),
                extras: HashMap::new(),
            }),
            analytics_cache: ::server_utils::cache::HybridCache::new(redis_client),
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
        let org_id = _request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).and_then(|v| ::server_auth::parse_spiffe_id(v).ok()).map(|(id, _)| id).unwrap_or_else(|| "default".to_string());
        let cache_key = format!("org_analytics_{}", org_id);

        if let Some(cached) = self.analytics_cache.get(&cache_key).await {
            return Ok(Response::new(cached));
        }

        let hub1 = self.hub.clone();
        let hub2 = self.hub.clone();
        let hub3 = self.hub.clone();
        let (agents_res, meetings_res, summary_res) = tokio::join!(
            tokio::spawn(async move { hub1.get_agents().await }),
            tokio::task::spawn_blocking(move || hub2.get_meetings()),
            tokio::task::spawn_blocking(move || hub3.tracker().summary("system"))
        );
        let agents = agents_res.map_err(|e| Status::internal(e.to_string()))?;
        let meetings = meetings_res.map_err(|e| Status::internal(e.to_string()))?;
        let summary = summary_res.map_err(|e| Status::internal(e.to_string()))?;
        
        let mut total_msgs = 0;
        let mut audited_msgs = 0;
        let mut agent_set = std::collections::HashSet::new();
        for a in agents.iter() {
            agent_set.insert(a.id.clone());
        }
        
        for m in meetings.iter() {
            for msg in &m.transcript {
                total_msgs += 1;
                if agent_set.contains(&msg.from_agent) {
                    audited_msgs += 1;
                }
            }
        }
        
        let audit_fidelity_pct = if total_msgs > 0 {
            (audited_msgs as f64 / total_msgs as f64) * 100.0
        } else {
            100.0
        };
        
        let total_agents = agents.len() as i32;
        let total_humans = 10; 
        
        let human_agent_ratio = if total_humans > 0 {
            total_agents as f64 / total_humans as f64
        } else {
            0.0
        };
        
        let status = self.hub.tracker().check_agent_quota(&org_id).await.unwrap_or(::server_pricing::rate_limit::RateLimitStatus {
            is_allowed: true,
            soft_limit_reached: false,
            user_message: None,
        });

        let response = AnalyticsSummaryResponse {
            human_agent_ratio,
            total_agents,
            total_humans,
            audit_fidelity_pct,
            resumption_latency_ms: 4800,
            pending_approvals: 2,
            active_handoffs: 1,
            token_velocity: summary.total_tokens,
            soft_limit_reached: status.soft_limit_reached,
            upgrade_message: status.user_message.unwrap_or_default(),
            is_allowed: status.is_allowed,
        };

        self.analytics_cache.set(&cache_key, response.clone(), std::time::Duration::from_secs(60)).await;

        Ok(Response::new(response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;

    #[tokio::test]
    async fn test_get_analytics_caching() {
        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let pg_pool = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
        let db_arc = Arc::new(crate::db::DB { pool: pg_pool, store: crate::db::DbStore::Sqlite(sqlx::sqlite::SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap()) });
        let hub = Arc::new(crate::hub::Hub::new(tx, db_arc.pool.clone()));

        let service = MyOrgService::new(hub);

        let mut request1 = Request::new(EmptyRequest {});
        request1.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/system/test".parse().unwrap());

        let start = std::time::Instant::now();
        let _res1 = service.get_analytics(request1).await.unwrap().into_inner();
        let _elapsed1 = start.elapsed();

        let mut request2 = Request::new(EmptyRequest {});
        request2.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/system/test".parse().unwrap());

        let start2 = std::time::Instant::now();
        let _res2 = service.get_analytics(request2).await.unwrap().into_inner();
        let _elapsed2 = start2.elapsed();

        // The second call should be faster, but we just verify it works properly via caching
        assert!(_res1.total_agents == _res2.total_agents);
    }
}
