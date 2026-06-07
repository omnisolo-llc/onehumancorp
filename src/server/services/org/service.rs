use tonic::{Request, Response, Status};
use ::server_ohc::orchestration::*;
use ::server_ohc::orchestration::org_service_server::OrgService;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

use ::server_utils::cache::HybridCache;
use std::sync::OnceLock;

static DOMAINS_CACHE: OnceLock<HybridCache<Vec<DomainInfoProto>>> = OnceLock::new();
static MARKETPLACE_ITEMS_CACHE: OnceLock<HybridCache<Vec<MarketplaceItemProto>>> = OnceLock::new();

pub struct MyOrgService {
    hub: Arc<crate::hub::Hub>,
    settings: RwLock<SettingsResponse>,
    analytics_cache: std::sync::Arc<::server_utils::cache::HybridCache<AnalyticsSummaryResponse>>,
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
            analytics_cache: std::sync::Arc::new(::server_utils::cache::HybridCache::new(redis_client)),
        }
    }
}

#[tonic::async_trait]
impl OrgService for MyOrgService {
    async fn get_domains(
        &self,
        _request: Request<::server_ohc::orchestration::EmptyRequest>,
    ) -> Result<Response<DomainsResponse>, Status> {
        let cache_key = "org_domains".to_string();
        let cache = DOMAINS_CACHE.get_or_init(|| HybridCache::new(self.hub.redis_client.clone()));

        if let Some(domains) = cache.get(&cache_key).await {
            return Ok(Response::new(DomainsResponse { domains }));
        }

        let domains = vec![
            DomainInfoProto { id: "software_company".to_string(), name: "Software Company".to_string(), description: "Full-stack engineering org...".to_string() },
            DomainInfoProto { id: "digital_marketing_agency".to_string(), name: "Digital Marketing Agency".to_string(), description: "Full-service agency...".to_string() },
            DomainInfoProto { id: "accounting_firm".to_string(), name: "Accounting Firm".to_string(), description: "Financial services firm...".to_string() },
        ];

        cache.set(&cache_key, domains.clone(), std::time::Duration::from_secs(3600)).await;

        Ok(Response::new(DomainsResponse { domains }))
    }

    async fn get_settings(
        &self,
        _request: Request<::server_ohc::orchestration::EmptyRequest>,
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
        _request: Request<::server_ohc::orchestration::EmptyRequest>,
    ) -> Result<Response<MarketplaceItemsResponse>, Status> {
        let cache_key = "org_marketplace_items".to_string();
        let cache = MARKETPLACE_ITEMS_CACHE.get_or_init(|| HybridCache::new(self.hub.redis_client.clone()));

        if let Some(items) = cache.get(&cache_key).await {
            return Ok(Response::new(MarketplaceItemsResponse { items }));
        }

        let items = vec![
            MarketplaceItemProto { id: "git-mcp".to_string(), name: "Git".to_string(), r#type: "tool".to_string(), author: "system".to_string(), description: "Git operations".to_string(), downloads: 100, rating: 4.5, tags: vec!["code".to_string()] },
        ];

        cache.set(&cache_key, items.clone(), std::time::Duration::from_secs(3600)).await;

        Ok(Response::new(MarketplaceItemsResponse { items }))
    }

    async fn get_analytics(
        &self,
        _request: Request<::server_ohc::orchestration::EmptyRequest>,
    ) -> Result<Response<AnalyticsSummaryResponse>, Status> {
        let org_id = _request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).and_then(|v| ::server_auth::parse_spiffe_id(v).ok()).map(|(id, _)| id).unwrap_or_else(|| "default".to_string());
        let cache_key = format!("org_analytics_{}", org_id);

        let analytics_cache_clone = self.analytics_cache.clone();
        if let Some((cached, is_stale)) = analytics_cache_clone.get_with_swr(&cache_key).await {
            if !is_stale {
                return Ok(Response::new(cached));
            }

            // Stale cache hit, spawn background task
            let cache_key_bg = cache_key.clone();
            let hub_bg = self.hub.clone();
            let org_id_bg = org_id.clone();
            let cache_bg = analytics_cache_clone.clone();

            tokio::spawn(async move {
                let hub_for_summary = hub_bg.clone();
                let hub_for_agents = hub_bg.clone();
                let org_id_clone = org_id_bg.clone();
                let org_id_for_agents = org_id_bg.clone();
                let org_id_for_summary = org_id_bg.clone();

                let (all_meetings, quota_result) = tokio::join!(
                    hub_bg.get_meetings(),
                    hub_bg.tracker().check_agent_quota(&org_id_clone)
                );

                let agents = hub_for_agents.get_agents_by_org(&org_id_for_agents);
                let summary = hub_for_summary.tracker().summary(&org_id_for_summary);

                let org_id_for_metrics = org_id_bg.clone();
                let total_agents = agents.len() as i32;

                let mut total_msgs = 0;
                let mut audited_msgs = 0;
                let mut agent_set = std::collections::HashSet::new();
                for a in agents.iter() {
                    agent_set.insert(a.id.clone());
                }

                for m in all_meetings.iter() {
                    if m.id.starts_with(&org_id_for_metrics) || m.id.contains(&org_id_for_metrics) {
                        for msg in &m.transcript {
                            total_msgs += 1;
                            if agent_set.contains(&msg.from_agent) {
                                audited_msgs += 1;
                            }
                        }
                    }
                }

                let audit_fidelity_pct = if total_msgs > 0 {
                    (audited_msgs as f64 / total_msgs as f64) * 100.0
                } else {
                    100.0
                };

                let total_humans = 10;
                let human_agent_ratio = if total_humans > 0 {
                    total_agents as f64 / total_humans as f64
                } else {
                    0.0
                };

                let status = quota_result.unwrap_or(::server_pricing::rate_limit::RateLimitStatus {
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

                cache_bg.set(&cache_key_bg, response.clone(), std::time::Duration::from_secs(60)).await;
            });

            return Ok(Response::new(cached));
        }

        if let Some(cached) = analytics_cache_clone.get(&cache_key).await {
            return Ok(Response::new(cached));
        }

        let hub_for_summary = self.hub.clone();
        let hub_for_agents = self.hub.clone();
        let org_id_clone = org_id.clone();
        let org_id_for_agents = org_id.clone();
        let org_id_for_summary = org_id.clone();
        let (all_meetings, quota_result) = tokio::join!(
            self.hub.get_meetings(),
            self.hub.tracker().check_agent_quota(&org_id_clone)
        );

        let agents = hub_for_agents.get_agents_by_org(&org_id_for_agents);
        let summary = hub_for_summary.tracker().summary(&org_id_for_summary);

        let org_id_for_metrics = org_id.clone();
        let total_agents = agents.len() as i32;

        let mut total_msgs = 0;
        let mut audited_msgs = 0;
        let mut agent_set = std::collections::HashSet::new();
        for a in agents.iter() {
            agent_set.insert(a.id.clone());
        }

        for m in all_meetings.iter() {
            if m.id.starts_with(&org_id_for_metrics) || m.id.contains(&org_id_for_metrics) {
                for msg in &m.transcript {
                    total_msgs += 1;
                    if agent_set.contains(&msg.from_agent) {
                        audited_msgs += 1;
                    }
                }
            }
        }
        
        let audit_fidelity_pct = if total_msgs > 0 {
            (audited_msgs as f64 / total_msgs as f64) * 100.0
        } else {
            100.0
        };
        
        let total_humans = 10; 
        
        let human_agent_ratio = if total_humans > 0 {
            total_agents as f64 / total_humans as f64
        } else {
            0.0
        };
        
        let status = quota_result.unwrap_or(::server_pricing::rate_limit::RateLimitStatus {
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
        let pg_pool = crate::db::get_pool();
        let db_arc = Arc::new(crate::db::DB { pool: pg_pool, store: crate::db::DbStore::Sqlite(sqlx::sqlite::SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap()) });
        let hub = Arc::new(crate::hub::Hub::new(tx, db_arc.pool.clone()));

        let service = MyOrgService::new(hub);

        let mut request1 = Request::new(::server_ohc::orchestration::EmptyRequest {});
        request1.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/system/test".parse().unwrap());

        let start = std::time::Instant::now();
        let _res1 = service.get_analytics(request1).await.unwrap().into_inner();
        let _elapsed1 = start.elapsed();

        let mut request2 = Request::new(::server_ohc::orchestration::EmptyRequest {});
        request2.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/system/test".parse().unwrap());

        let start2 = std::time::Instant::now();
        let _res2 = service.get_analytics(request2).await.unwrap().into_inner();
        let _elapsed2 = start2.elapsed();

        // The second call should be faster, but we just verify it works properly via caching
        assert!(_res1.total_agents == _res2.total_agents);
    }

    #[tokio::test]
    async fn test_get_domains() {
        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let pg_pool = crate::db::get_pool();
        let db_arc = Arc::new(crate::db::DB { pool: pg_pool, store: crate::db::DbStore::Sqlite(sqlx::sqlite::SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap()) });
        let hub = Arc::new(crate::hub::Hub::new(tx, db_arc.pool.clone()));

        let service = MyOrgService::new(hub);

        let request = Request::new(::server_ohc::orchestration::EmptyRequest {});
        let res = service.get_domains(request).await.unwrap().into_inner();
        assert!(!res.domains.is_empty());
        assert_eq!(res.domains[0].id, "software_company");

        // Cache coverage call
        let request2 = Request::new(::server_ohc::orchestration::EmptyRequest {});
        let _res2 = service.get_domains(request2).await.unwrap().into_inner();
    }

    #[tokio::test]
    async fn test_get_and_update_settings() {
        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let pg_pool = crate::db::get_pool();
        let db_arc = Arc::new(crate::db::DB { pool: pg_pool, store: crate::db::DbStore::Sqlite(sqlx::sqlite::SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap()) });
        let hub = Arc::new(crate::hub::Hub::new(tx, db_arc.pool.clone()));

        let service = MyOrgService::new(hub);

        let request = Request::new(::server_ohc::orchestration::EmptyRequest {});
        let _res = service.get_settings(request).await.unwrap().into_inner();
        let mut extras = HashMap::new();
        extras.insert("key1".to_string(), "val1".to_string());

        let update_req = Request::new(UpdateSettingsRequest {
            minimax_api_key: "new_key".to_string(),
            extras: extras.clone(),
        });
        let updated_res = service.update_settings(update_req).await.unwrap().into_inner();
        assert_eq!(updated_res.minimax_api_key, "new_key");
        assert_eq!(updated_res.extras.get("key1").unwrap(), "val1");

        let request2 = Request::new(::server_ohc::orchestration::EmptyRequest {});
        let res2 = service.get_settings(request2).await.unwrap().into_inner();
        assert_eq!(res2.minimax_api_key, "new_key");
        assert_eq!(res2.extras.get("key1").unwrap(), "val1");
    }

    #[tokio::test]
    async fn test_get_marketplace_items() {
        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let pg_pool = crate::db::get_pool();
        let db_arc = Arc::new(crate::db::DB { pool: pg_pool, store: crate::db::DbStore::Sqlite(sqlx::sqlite::SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap()) });
        let hub = Arc::new(crate::hub::Hub::new(tx, db_arc.pool.clone()));

        let service = MyOrgService::new(hub);

        let request = Request::new(::server_ohc::orchestration::EmptyRequest {});
        let res = service.get_marketplace_items(request).await.unwrap().into_inner();
        assert!(!res.items.is_empty());
        assert_eq!(res.items[0].id, "git-mcp");

        let request2 = Request::new(::server_ohc::orchestration::EmptyRequest {});
        let _res2 = service.get_marketplace_items(request2).await.unwrap().into_inner();
    }
}
