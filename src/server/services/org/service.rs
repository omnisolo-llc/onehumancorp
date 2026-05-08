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
        request: Request<EmptyRequest>,
    ) -> Result<Response<AnalyticsSummaryResponse>, Status> {
        let tenant_id_str = match request.metadata().get("x-tenant-id") {
            Some(v) => v.to_str().unwrap_or("system").to_string(),
            None => "system".to_string()
        };

        let hub1 = self.hub.clone();
        let hub2 = self.hub.clone();
        let hub3 = self.hub.clone();
        let tenant_id_for_task = tenant_id_str.clone();
        let (agents_res, meetings_res, summary_res) = tokio::join!(
            tokio::task::spawn_blocking(move || hub1.get_agents()),
            tokio::task::spawn_blocking(move || hub2.get_meetings()),
            tokio::task::spawn_blocking(move || hub3.tracker().summary(&tenant_id_for_task))
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
        
        let db = crate::db::DB {
            pool: self.hub.pool.clone(),
            store: if std::env::var("DATABASE_URL").unwrap_or_default().starts_with("sqlite") {
                crate::db::DbStore::Sqlite(sqlx::sqlite::SqlitePoolOptions::new().connect_lazy(&std::env::var("DATABASE_URL").unwrap()).unwrap())
            } else {
                crate::db::DbStore::Postgres
            },
        };
        let tenant_id = tenant_id_str.as_str();
        let orders_res = db.get_orders_for_analytics(tenant_id).await.unwrap_or_default();
        let products_res = db.get_products_for_analytics(tenant_id).await.unwrap_or_default();
        let traffic_res = db.get_traffic_for_analytics(tenant_id).await.unwrap_or_default();

        let mut order_points = Vec::new();
        let mut revenue_points = Vec::new();
        for (dt, total, count) in orders_res {
            order_points.push(UiDataPointProto {
                label: dt.clone(),
                value: count as f64,
                display_value: count.to_string(),
            });
            revenue_points.push(UiDataPointProto {
                label: dt,
                value: total,
                display_value: format!("${:.2}", total),
            });
        }

        let mut product_points = Vec::new();
        for (name, qty) in products_res {
            product_points.push(UiDataPointProto {
                label: name,
                value: qty as f64,
                display_value: qty.to_string(),
            });
        }

        let mut traffic_points = Vec::new();
        for (source, clicks) in traffic_res {
            traffic_points.push(UiDataPointProto {
                label: source,
                value: clicks,
                display_value: format!("{:.0}", clicks),
            });
        }

        let charts = vec![
            UiChartDataProto {
                title: "Revenue Over Time".to_string(),
                points: revenue_points,
            },
            UiChartDataProto {
                title: "Orders by Day".to_string(),
                points: order_points,
            },
            UiChartDataProto {
                title: "Top Products".to_string(),
                points: product_points,
            },
            UiChartDataProto {
                title: "Traffic Sources".to_string(),
                points: traffic_points,
            }
        ];

        Ok(Response::new(AnalyticsSummaryResponse {
            human_agent_ratio,
            total_agents,
            total_humans,
            audit_fidelity_pct,
            resumption_latency_ms: 4800,
            pending_approvals: 2,
            active_handoffs: 1,
            token_velocity: summary.total_tokens,
            charts,
        }))
    }
}
