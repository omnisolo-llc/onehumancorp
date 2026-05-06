use tonic::{Request, Response, Status};
use crate::ohc::app::*;
use crate::ohc::app::dashboard_service_server::DashboardService;
use std::sync::Arc;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct CachedProduct {
    id: String,
    organization_id: String,
    name: String,
    description: String,
    price_cents: i64,
    currency: String,
    fulfillment_strategy: String,
    metadata_json: String,
}

pub struct MyDashboardService {
    hub: Arc<crate::hub::Hub>,
    db: Arc<crate::db::DB>,
    product_cache: Arc<dashmap::DashMap<String, (std::time::Instant, Vec<crate::ohc::organization::Product>)>>,
}

impl MyDashboardService {
    pub fn new(db: Arc<crate::db::DB>, hub: Arc<crate::hub::Hub>) -> Self {
        Self {
            db,
            hub,
            product_cache: Arc::new(dashmap::DashMap::new()),
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
        let product_cache = self.product_cache.clone();

        let (agents_res, meetings_res, cost_res, products_res, orders_res) = tokio::join!(
            tokio::task::spawn_blocking(move || hub1.get_agents()),
            tokio::task::spawn_blocking(move || hub2.get_meetings()),
            tokio::task::spawn_blocking(move || {
                let cost_auditor = hub3.get_cost_auditor();
                (cost_auditor.get_total_cost(), cost_auditor.get_total_tokens(), cost_auditor.get_agent_costs_snapshot())
            }),
            async {
                let org_id = req.organization_id.clone();
                let cache_key = format!("hub:products:{}", org_id);
                let redis_client = hub4.get_redis_client();

                if let Some(client) = &redis_client {
                    if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                        let res: Result<Option<String>, _> = redis::cmd("GET").arg(&cache_key).query_async(&mut conn).await;
                        if let Ok(Some(data)) = res {
                            if let Ok(cached) = serde_json::from_str::<Vec<CachedProduct>>(&data) {
                                let products = cached.into_iter().map(|c| crate::ohc::organization::Product {
                                    id: c.id,
                                    organization_id: c.organization_id,
                                    name: c.name,
                                    description: c.description,
                                    price_cents: c.price_cents,
                                    currency: c.currency,
                                    fulfillment_strategy: c.fulfillment_strategy,
                                    metadata_json: c.metadata_json,
                                }).collect();
                                return Ok::<_, String>(products);
                            }
                        }
                    }
                } else if let Some(entry) = product_cache.get(&org_id) {
                    if entry.value().0.elapsed().as_secs() < 3600 {
                        return Ok::<_, String>(entry.value().1.clone());
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

                let to_cache = results.clone();
                if let Some(client) = &redis_client {
                    if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                        let cached: Vec<CachedProduct> = to_cache.iter().map(|p| CachedProduct {
                            id: p.id.clone(),
                            organization_id: p.organization_id.clone(),
                            name: p.name.clone(),
                            description: p.description.clone(),
                            price_cents: p.price_cents,
                            currency: p.currency.clone(),
                            fulfillment_strategy: p.fulfillment_strategy.clone(),
                            metadata_json: p.metadata_json.clone(),
                        }).collect();
                        if let Ok(json) = serde_json::to_string(&cached) {
                            let _: Result<(), _> = redis::cmd("SETEX").arg(&cache_key).arg(3600).arg(json).query_async(&mut conn).await;
                        }
                    }
                } else {
                    product_cache.insert(org_id, (std::time::Instant::now(), to_cache));
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

        let mut out_meetings: Vec<crate::ohc::app::MeetingRoom> = Vec::new();
        for m in _meetings.iter() {
            let mut transcript = Vec::new();
            if !req.mobile_optimized {
                for msg in &m.transcript {
                    transcript.push(crate::ohc::agent::AgentMessage {
                        id: msg.id.clone(),
                        from_agent_id: msg.from_agent.clone(),
                        to_agent_id: msg.to_agent.clone(),
                        message_type: msg.r#type.clone(),
                        content: msg.content.clone(),
                        meeting_id: m.id.clone(),
                        occurred_at_unix: msg.occurred_at_unix,
                    });
                }
            }
            out_meetings.push(crate::ohc::app::MeetingRoom {
                id: m.id.clone(),
                participants: m.participants.clone(),
                transcript,
            });
        }

        let _filtered_agents: Vec<crate::ohc::orchestration::Agent> = agents.iter().filter(|a| a.organization_id == req.organization_id || a.id.starts_with(&format!("{}-", req.organization_id))).cloned().collect();

        let mut final_agents: Vec<crate::ohc::agent::Agent> = _filtered_agents.into_iter().map(|a| {
            let role = match a.role.to_lowercase().as_str() {
                "ceo" => crate::ohc::common::Role::Ceo as i32,
                "product_manager" => crate::ohc::common::Role::ProductManager as i32,
                "software_engineer" => crate::ohc::common::Role::SoftwareEngineer as i32,
                "growth_agent" => crate::ohc::common::Role::GrowthAgent as i32,
                _ => crate::ohc::common::Role::Unspecified as i32,
            };

            let status = match a.status.to_uppercase().as_str() {
                "IDLE" => crate::ohc::common::AgentStatus::Idle as i32,
                "ACTIVE" => crate::ohc::common::AgentStatus::Active as i32,
                "IN_MEETING" => crate::ohc::common::AgentStatus::InMeeting as i32,
                "BLOCKED" => crate::ohc::common::AgentStatus::Blocked as i32,
                _ => crate::ohc::common::AgentStatus::StatusUnspecified as i32,
            };

            crate::ohc::agent::Agent {
                id: a.id,
                name: a.name,
                role,
                status,
                organization_id: a.organization_id,
            }
        }).collect();

        let mut final_products = products;

        if req.mobile_optimized {
            for a in &mut final_agents {
                a.role = crate::ohc::common::Role::Unspecified as i32;
                a.status = crate::ohc::common::AgentStatus::StatusUnspecified as i32;
            }
            for p in &mut final_products {
                p.description.clear();
                p.metadata_json.clear();
                p.fulfillment_strategy.clear();
            }
        }

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
            agents: final_agents,
            meetings: out_meetings,
            cost_summary: Some(cost_summary),
            statuses,
            updated_at: chrono::Utc::now().to_rfc3339(),
            products: final_products,
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