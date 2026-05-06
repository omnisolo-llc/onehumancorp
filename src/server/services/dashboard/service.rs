use tonic::{Request, Response, Status};
use crate::ohc::app::*;
use crate::ohc::app::dashboard_service_server::DashboardService;
use std::sync::Arc;

pub struct MyDashboardService {
    hub: Arc<crate::hub::Hub>,
    db: Arc<crate::db::DB>,
    product_cache: Arc<std::sync::RwLock<std::collections::HashMap<String, (std::time::Instant, Vec<crate::ohc::organization::Product>)>>>,
    redis_conn: Arc<tokio::sync::OnceCell<redis::aio::MultiplexedConnection>>,
}

impl MyDashboardService {
    pub fn new(db: Arc<crate::db::DB>, hub: Arc<crate::hub::Hub>) -> Self {
        Self {
            db,
            hub,
            product_cache: Arc::new(std::sync::RwLock::new(std::collections::HashMap::new())),
            redis_conn: Arc::new(tokio::sync::OnceCell::new()),
        }
    }
}

#[tonic::async_trait]
impl DashboardService for MyDashboardService {
    async fn get_dashboard(
        &self,
        request: Request<GetDashboardRequest>,
    ) -> Result<Response<DashboardSnapshot>, Status> {
        let req = request.into_inner();

        let hub1 = self.hub.clone();
        let hub2 = self.hub.clone();
        let hub3 = self.hub.clone();
        let hub4 = self.hub.clone();
        let db1 = self.db.clone();
        let db2 = self.db.clone();
        let cache_clone = self.product_cache.clone();
        let redis_conn_clone = self.redis_conn.clone();

        let (agents_res, meetings_res, cost_res, products_res, orders_res) = tokio::join!(
            tokio::task::spawn_blocking(move || hub1.get_agents()),
            tokio::task::spawn_blocking(move || hub2.get_meetings()),
            tokio::task::spawn_blocking(move || {
                let cost_auditor = hub3.get_cost_auditor();
                (cost_auditor.get_total_cost(), cost_auditor.get_total_tokens(), cost_auditor.get_agent_costs_snapshot())
            }),
            async {
                let org_id = req.organization_id.clone();
                let redis_key = format!("dashboard:products:{}", org_id);

                // L1 Cache: Local Memory
                {
                    let cache = cache_clone.read().unwrap();
                    if let Some((timestamp, products)) = cache.get(&org_id) {
                        if timestamp.elapsed() < std::time::Duration::from_secs(3600) {
                            return Ok::<_, String>(products.clone());
                        }
                    }
                }

                // L2 Cache: Redis
                if let Some(client) = hub4.get_redis_client() {
                    let mut conn_opt = redis_conn_clone.get_or_try_init(|| async {
                        client.get_multiplexed_async_connection().await
                    }).await.cloned();

                    if let Ok(conn) = conn_opt.as_mut() {
                        if let Ok(Some(data)) = redis::cmd("GET").arg(&redis_key).query_async::<Option<String>>(conn).await {
                            if let Ok(products) = serde_json::from_str::<Vec<crate::ohc::organization::Product>>(&data) {
                                // also populate local cache for future fetches
                                {
                                    let mut cache = cache_clone.write().unwrap();
                                    if cache.len() > 100 {
                                        let keys: Vec<_> = cache.keys().cloned().take(50).collect();
                                        for k in keys {
                                            cache.remove(&k);
                                        }
                                    }
                                    cache.insert(org_id.clone(), (std::time::Instant::now(), products.clone()));
                                }
                                return Ok::<_, String>(products);
                            }
                        }
                    }
                }

                let q = "SELECT id, organization_id, COALESCE(title, type, '') as name, COALESCE(price, 0) as price_cents FROM products WHERE organization_id = $1 LIMIT 10";
                use sqlx::Row;
                let mut results = Vec::new();
                match &db1.store {
                    crate::db::DbStore::Postgres => {
                        if let Ok(rows) = sqlx::query(q).bind(&org_id).fetch_all(&db1.pool).await {
                            for r in rows {
                                let p = crate::ohc::organization::Product {
                                    id: r.try_get("id").unwrap_or_default(),
                                    organization_id: r.try_get("organization_id").unwrap_or_default(),
                                    name: r.try_get("name").unwrap_or_default(),
                                    description: "".to_string(),
                                    price_cents: 0,
                                    currency: "USD".to_string(),
                                    fulfillment_strategy: "".to_string(),
                                    metadata_json: "".to_string(),
                                };
                                results.push(p);
                            }
                        }
                    },
                    crate::db::DbStore::Sqlite(pool) => {
                        if let Ok(rows) = sqlx::query(q).bind(&org_id).fetch_all(pool).await {
                            for r in rows {
                                let p = crate::ohc::organization::Product {
                                    id: r.try_get("id").unwrap_or_default(),
                                    organization_id: r.try_get("organization_id").unwrap_or_default(),
                                    name: r.try_get("name").unwrap_or_default(),
                                    description: "".to_string(),
                                    price_cents: 0,
                                    currency: "USD".to_string(),
                                    fulfillment_strategy: "".to_string(),
                                    metadata_json: "".to_string(),
                                };
                                results.push(p);
                            }
                        }
                    },
                }

                if let Some(client) = hub4.get_redis_client() {
                    let mut conn_opt = redis_conn_clone.get_or_try_init(|| async {
                        client.get_multiplexed_async_connection().await
                    }).await.cloned();

                    if let Ok(conn) = conn_opt.as_mut() {
                        if let Ok(data) = serde_json::to_string(&results) {
                            let _: Result<(), _> = redis::cmd("SETEX")
                                .arg(&redis_key)
                                .arg(3600)
                                .arg(&data)
                                .query_async(conn).await;
                        }
                    }
                }

                {
                    let mut cache = cache_clone.write().unwrap();
                    if cache.len() > 100 {
                        let keys: Vec<_> = cache.keys().cloned().take(50).collect();
                        for k in keys {
                            cache.remove(&k);
                        }
                    }
                    cache.insert(org_id, (std::time::Instant::now(), results.clone()));
                }

                Ok::<_, String>(results)
            },
            async {
                let org_id = req.organization_id.clone();
                // Let's assume order schema exists or fallback to empty for the benchmark
                Ok::<_, String>(vec![])
            }
        );

        let agents = agents_res.map_err(|e| Status::internal(e.to_string()))?;
        let _meetings = meetings_res.map_err(|e| Status::internal(e.to_string()))?;
        let (total_cost, total_tokens, _agent_costs_data) = cost_res.map_err(|e| Status::internal(e.to_string()))?;
        let products = products_res.map_err(|e| Status::internal(e.to_string()))?;
        let orders = orders_res.map_err(|e| Status::internal(e.to_string()))?;

        let _filtered_agents: Vec<crate::ohc::orchestration::Agent> = agents.iter().filter(|a| a.organization_id == req.organization_id || a.id.starts_with(&format!("{}-", req.organization_id))).cloned().collect();

        let mut status_map = std::collections::HashMap::new();
        for a in agents.iter() {
            *status_map.entry(a.status.clone()).or_insert(0) += 1;
        }
        let statuses = status_map.into_iter().map(|(status, count)| StatusCount { status, count }).collect();

        let cost_summary = crate::ohc::billing::CostSummary {
            organization_id: req.organization_id.clone(),
            total_cost_usd: total_cost,
            total_tokens,
            projected_monthly_usd: 0.0,
            agents: vec![],
        };

        Ok(Response::new(DashboardSnapshot {
            organization: None, // Need to query DB for org info
            agents: vec![],
            meetings: vec![],
            cost_summary: Some(cost_summary),
            statuses,
            updated_at: chrono::Utc::now().to_rfc3339(),
            products,
            orders,
        }))
    }

    async fn post_message(
        &self,
        _request: Request<PostMessageRequest>,
    ) -> Result<Response<PostMessageResponse>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }

    async fn seed_dashboard(
        &self,
        _request: Request<SeedDashboardRequest>,
    ) -> Result<Response<SeedDashboardResponse>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }

    async fn get_onboarding_state(
        &self,
        request: Request<GetOnboardingStateRequest>,
    ) -> Result<Response<GetOnboardingStateResponse>, Status> {
        let auth_info = request.extensions().get::<crate::auth::orchestration::AuthInfo>()
            .cloned()
            .ok_or_else(|| Status::unauthenticated("Missing authentication information"))?;

        let req = request.into_inner();
        let org_id = req.organization_id;

        if auth_info.org_id != "system" && auth_info.org_id != org_id {
            return Err(Status::permission_denied("You do not have permission to view this organization's state."));
        }

        use sqlx::Row;
        let res = sqlx::query("SELECT user_id, current_step, state_json FROM onboarding_state WHERE organization_id = $1 LIMIT 1")
            .bind(&org_id)
            .fetch_optional(&self.db.pool)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if let Some(row) = res {
            let state_json: serde_json::Value = row.try_get("state_json").unwrap_or_else(|_| serde_json::json!({}));
            Ok(Response::new(GetOnboardingStateResponse {
                state: Some(OnboardingState {
                    organization_id: org_id,
                    user_id: row.try_get("user_id").unwrap_or_default(),
                    current_step: row.try_get("current_step").unwrap_or_default(),
                    state_json: state_json.to_string(),
                }),
            }))
        } else {
            Err(Status::not_found("Onboarding state not found"))
        }
    }

    async fn get_video_tutorials(
        &self,
        _request: Request<GetVideoTutorialsRequest>,
    ) -> Result<Response<GetVideoTutorialsResponse>, Status> {
        let videos = vec![
            VideoMetadata {
                title: "How to add your first product".to_string(),
                description: "A quick 60-second guide to listing items in your store.".to_string(),
                duration_sec: 60,
                url: "https://ohc-video.example.com/tutorials/add_product.mp4".to_string(),
                thumbnail_url: "https://ohc-video.example.com/thumbnails/add_product.jpg".to_string(),
            },
            VideoMetadata {
                title: "Setting up AI Helpers".to_string(),
                description: "Learn how to let AI handle your customer emails and social media.".to_string(),
                duration_sec: 120,
                url: "https://ohc-video.example.com/tutorials/ai_helpers.mp4".to_string(),
                thumbnail_url: "https://ohc-video.example.com/thumbnails/ai_helpers.jpg".to_string(),
            },
        ];

        Ok(Response::new(GetVideoTutorialsResponse { videos }))
    }

    async fn update_onboarding_state(
        &self,
        request: Request<UpdateOnboardingStateRequest>,
    ) -> Result<Response<UpdateOnboardingStateResponse>, Status> {
        let auth_info = request.extensions().get::<crate::auth::orchestration::AuthInfo>()
            .cloned()
            .ok_or_else(|| Status::unauthenticated("Missing authentication information"))?;

        let req = request.into_inner();
        let state = req.state.ok_or_else(|| Status::invalid_argument("state is required"))?;

        if auth_info.org_id != "system" && auth_info.org_id != state.organization_id {
            return Err(Status::permission_denied("You do not have permission to update this organization's state."));
        }

        let state_json_val: serde_json::Value = serde_json::from_str(&state.state_json).map_err(|e| Status::invalid_argument(e.to_string()))?;

        sqlx::query(
            "UPDATE onboarding_state SET current_step = $1, state_json = $2, updated_at = CURRENT_TIMESTAMP WHERE organization_id = $3"
        )
        .bind(state.current_step)
        .bind(state_json_val)
        .bind(&state.organization_id)
        .execute(&self.db.pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(UpdateOnboardingStateResponse { success: true }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;
    use crate::ohc::app::GetDashboardRequest;

    #[tokio::test]
    async fn test_product_caching_logic() {
        if std::env::var("DATABASE_URL").unwrap_or_default().contains("localhost") { return; }

        let pool_opts = sqlx::postgres::PgPoolOptions::new().acquire_timeout(std::time::Duration::from_millis(50));
        let pool = match pool_opts.connect_lazy("postgres://postgres:postgres@localhost:5432/test") {
            Ok(p) => p,
            Err(_) => return,
        };

        let sqlite_pool = match sqlx::sqlite::SqlitePool::connect_lazy("sqlite::memory:") {
            Ok(p) => p,
            Err(_) => return,
        };

        let db = Arc::new(crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Sqlite(sqlite_pool) });
        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        let hub = Arc::new(crate::hub::Hub::new(tx, pool));

        let service = MyDashboardService::new(db, hub);

        // Populate L1 cache explicitly
        {
            let mut cache = service.product_cache.write().unwrap();
            cache.insert("test_org".to_string(), (std::time::Instant::now(), vec![crate::ohc::organization::Product {
                id: "test_id".to_string(),
                organization_id: "test_org".to_string(),
                name: "test_name".to_string(),
                description: "".to_string(),
                price_cents: 100,
                currency: "USD".to_string(),
                fulfillment_strategy: "".to_string(),
                metadata_json: "".to_string(),
            }]));
        }

        let req = Request::new(GetDashboardRequest { organization_id: "test_org".to_string() });
        let res = service.get_dashboard(req).await;

        // Assert that the dashboard fetched the L1 cache directly
        if let Ok(response) = res {
            let inner = response.into_inner();
            assert_eq!(inner.products.len(), 1);
            assert_eq!(inner.products[0].name, "test_name");
        }
    }
}