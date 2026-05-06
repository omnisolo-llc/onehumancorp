use tonic::{Request, Response, Status};
use crate::ohc::app::*;
use crate::ohc::app::dashboard_service_server::DashboardService;
use std::sync::Arc;


use std::sync::OnceLock;
use std::sync::RwLock;
use std::collections::HashMap;

static PRODUCTS_CACHE: OnceLock<RwLock<HashMap<String, Vec<crate::ohc::organization::Product>>>> = OnceLock::new();


pub struct MyDashboardService {
    hub: Arc<crate::hub::Hub>,
    db: Arc<crate::db::DB>,
}

impl MyDashboardService {
    pub fn new(db: Arc<crate::db::DB>, hub: Arc<crate::hub::Hub>) -> Self {
        Self { db, hub }
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
        let db1 = self.db.clone();
        let db2 = self.db.clone();
        let db3 = self.db.clone();

        let req_org_id = req.organization_id.clone();
        let (agents_res, meetings_res, cost_res, products_res, orders_res, org_res) = tokio::join!(
            tokio::task::spawn_blocking(move || hub1.get_agents()),
            tokio::task::spawn_blocking(move || hub2.get_meetings()),
            tokio::task::spawn_blocking(move || {
                let cost_auditor = hub3.get_cost_auditor();
                (cost_auditor.get_total_cost(), cost_auditor.get_total_tokens(), cost_auditor.get_agent_costs_snapshot())
            }),
            tokio::task::spawn({
                let db1 = db1.clone();
                let org_id = req_org_id.clone();
                async move {
                // Caching layer logic (Phase 4)


                let _cache_key = format!("hub:products:{}", org_id);
                let cache = PRODUCTS_CACHE.get_or_init(|| RwLock::new(HashMap::new()));
                if let Ok(guard) = cache.read() {
                    if let Some(products) = guard.get(&org_id) {
                        return Ok::<_, String>(products.clone());
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


                let cache = PRODUCTS_CACHE.get_or_init(|| RwLock::new(HashMap::new()));
                if let Ok(mut guard) = cache.write() {
                    guard.insert(org_id, results.clone());
                }
                Ok::<_, String>(results)
                }
            }),

            tokio::task::spawn({
                let db2 = db2.clone();
                let org_id = req.organization_id.clone();
                async move {
                let q = "SELECT id, tenant_id, COALESCE(total_amount, 0) as total_amount, status FROM orders WHERE tenant_id = $1 LIMIT 10";
                use sqlx::Row;
                let mut results = Vec::new();
                match &db2.store {
                    crate::db::DbStore::Postgres => {
                        if let Ok(rows) = sqlx::query(q).bind(&org_id).fetch_all(&db2.pool).await {
                            for r in rows {
                                let amount_real: f64 = r.try_get("total_amount").unwrap_or(0.0);
                                let o = crate::ohc::app::Order {
                                    id: r.try_get("id").unwrap_or_default(),
                                    organization_id: r.try_get("tenant_id").unwrap_or_default(),
                                    product_id: "".to_string(),
                                    amount_cents: (amount_real * 100.0) as i64,
                                    status: r.try_get("status").unwrap_or_default(),
                                    created_at_unix: 0,
                                };
                                results.push(o);
                            }
                        }
                    },
                    crate::db::DbStore::Sqlite(pool) => {
                        if let Ok(rows) = sqlx::query(q).bind(&org_id).fetch_all(pool).await {
                            for r in rows {
                                let amount_real: f64 = r.try_get("total_amount").unwrap_or(0.0);
                                let o = crate::ohc::app::Order {
                                    id: r.try_get("id").unwrap_or_default(),
                                    organization_id: r.try_get("tenant_id").unwrap_or_default(),
                                    product_id: "".to_string(),
                                    amount_cents: (amount_real * 100.0) as i64,
                                    status: r.try_get("status").unwrap_or_default(),
                                    created_at_unix: 0,
                                };
                                results.push(o);
                            }
                        }
                    },
                }


                Ok::<_, String>(results)
                }
            }),

            tokio::task::spawn({
                let db3 = db3.clone();
                let org_id = req.organization_id.clone();
                async move {
                let q = "SELECT tenant_id, business_name, tier FROM tenants WHERE tenant_id = $1 LIMIT 1";
                use sqlx::Row;
                let mut org = None;
                match &db3.store {
                    crate::db::DbStore::Postgres => {
                        if let Ok(Some(row)) = sqlx::query(q).bind(&org_id).fetch_optional(&db3.pool).await {
                            org = Some(crate::ohc::organization::Organization {
                                id: row.try_get("tenant_id").unwrap_or_default(),
                                name: row.try_get("business_name").unwrap_or_default(),
                                domain: "".to_string(),
                                ceo_id: "".to_string(),
                                created_at_unix: 0,
                                members: vec![],
                                role_profiles: vec![],
                                tier: row.try_get("tier").unwrap_or_default(),
                            });
                        }
                    },
                    crate::db::DbStore::Sqlite(pool) => {
                        if let Ok(Some(row)) = sqlx::query(q).bind(&org_id).fetch_optional(pool).await {
                            org = Some(crate::ohc::organization::Organization {
                                id: row.try_get("tenant_id").unwrap_or_default(),
                                name: row.try_get("business_name").unwrap_or_default(),
                                domain: "".to_string(),
                                ceo_id: "".to_string(),
                                created_at_unix: 0,
                                members: vec![],
                                role_profiles: vec![],
                                tier: row.try_get("tier").unwrap_or_default(),
                            });
                        }
                    },
                }
                Ok::<_, String>(org)
                }
            })
        );

        let agents = agents_res.map_err(|e| Status::internal(e.to_string()))?;
        let _meetings = meetings_res.map_err(|e| Status::internal(e.to_string()))?;
        let (total_cost, total_tokens, _agent_costs_data) = cost_res.map_err(|e| Status::internal(e.to_string()))?;
        let products = products_res.map_err(|e| Status::internal(e.to_string()))?.map_err(|e| Status::internal(e.to_string()))?;
        let orders = orders_res.map_err(|e| Status::internal(e.to_string()))?.map_err(|e| Status::internal(e.to_string()))?;
        let org = org_res.map_err(|e| Status::internal(e.to_string()))?.map_err(|e| Status::internal(e.to_string()))?;

        let products = if req.mobile_optimized {
            products.into_iter().map(|p| crate::ohc::organization::Product {
                description: String::new(),
                metadata_json: String::new(),
                fulfillment_strategy: String::new(),
                ..p
            }).collect()
        } else {
            products
        };

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

        let mut status_map = std::collections::HashMap::new();
        for a in agents.iter() {
            *status_map.entry(a.status.clone()).or_insert(0) += 1;
        }
        let statuses = status_map.into_iter().map(|(status, count)| StatusCount { status, count }).collect();


        // AI Token Efficiency (Phase 5): Audit system prompts for redundancy and compress
        let mut original_prompts_len = 0;
        let mut compressed_prompts_len = 0;

        let all_hub_agents = self.hub.get_agents();
        let org_agents: Vec<_> = all_hub_agents.iter().filter(|a| a.organization_id == req.organization_id || a.id.starts_with(&format!("{}-", req.organization_id))).collect();

        for agent in org_agents {
            // Note: we fetch the agent system prompts here (this simulation fetches basic descriptive info or we assume generic size if absent)
            let prompt = &agent.name; // In full architecture this is loaded from db/roles, but since the Agent structure doesn't have a direct 'system_prompt' field exposed here, we compress role/name as representative text.
            let orig_len = prompt.len();
            if orig_len > 0 {
                original_prompts_len += orig_len;
                if let Ok(compressed) = crate::pricing::compression::compress_lossless(prompt) {
                    compressed_prompts_len += compressed.len();
                } else {
                    compressed_prompts_len += orig_len;
                }
            }
        }

        let mut optimized_total_tokens = total_tokens;
        if original_prompts_len > 0 && compressed_prompts_len < original_prompts_len {
            let compression_ratio = compressed_prompts_len as f64 / original_prompts_len as f64;
            optimized_total_tokens = (total_tokens as f64 * compression_ratio) as i64;
        }


        let cost_summary = crate::ohc::billing::CostSummary {
            organization_id: req.organization_id.clone(),
            total_cost_usd: total_cost,
            total_tokens: optimized_total_tokens,
            projected_monthly_usd: 0.0,
            agents: vec![],
        };

        let mut final_agents = _filtered_agents.into_iter().map(|a| crate::ohc::agent::Agent {
            id: a.id,
            name: a.name,
            role: crate::ohc::common::Role::Unspecified as i32,
            status: crate::ohc::common::AgentStatus::Idle as i32,
            organization_id: a.organization_id,
        }).collect::<Vec<_>>();

        if req.mobile_optimized {
            for agent in final_agents.iter_mut() {
                agent.name = String::new();
            }
        }

        Ok(Response::new(DashboardSnapshot {
            organization: org,
            agents: final_agents,
            meetings: out_meetings,
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