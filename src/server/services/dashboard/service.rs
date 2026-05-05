use tonic::{Request, Response, Status};
use crate::ohc::app::*;
use crate::ohc::app::dashboard_service_server::DashboardService;
use std::sync::Arc;

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
        let is_mobile = request.metadata().get("x-client-platform").map(|v| v.to_str().unwrap_or("")).unwrap_or("") == "mobile";
        let req = request.into_inner();

        let hub1 = self.hub.clone();
        let hub2 = self.hub.clone();
        let hub3 = self.hub.clone();
        let db_clone1 = self.db.clone();
        let db_clone2 = self.db.clone();
        let db_clone3 = self.db.clone();

        let org_id_1 = req.organization_id.clone();
        let org_id_2 = req.organization_id.clone();
        let org_id_3 = req.organization_id.clone();

        let (agents_res, meetings_res, cost_res, org_res, products_res, orders_res) = tokio::join!(
            tokio::task::spawn_blocking(move || hub1.get_agents()),
            tokio::task::spawn_blocking(move || hub2.get_meetings()),
            tokio::task::spawn_blocking(move || {
                let cost_auditor = hub3.get_cost_auditor();
                (cost_auditor.get_total_cost(), cost_auditor.get_total_tokens(), cost_auditor.get_agent_costs_snapshot())
            }),
            async move {
                sqlx::query("SELECT tenant_id as id, business_name as name, tier FROM tenants WHERE tenant_id = $1")
                    .bind(&org_id_1)
                    .fetch_optional(&db_clone1.pool)
                    .await
            },
            async move {
                sqlx::query("SELECT id, organization_id, name, description, price_cents, currency, fulfillment_strategy FROM products WHERE organization_id = $1 OR tenant_id = $1 LIMIT 50")
                    .bind(&org_id_2)
                    .fetch_all(&db_clone2.pool)
                    .await
            },
            async move {
                sqlx::query("SELECT id, customer_id, total_amount, status FROM orders WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 10")
                    .bind(&org_id_3)
                    .fetch_all(&db_clone3.pool)
                    .await
            }
        );

        let agents = agents_res.map_err(|e| Status::internal(e.to_string()))?;
        let meetings = meetings_res.map_err(|e| Status::internal(e.to_string()))?;
        let (total_cost, total_tokens, _agent_costs_data) = cost_res.map_err(|e| Status::internal(e.to_string()))?;

        // 1. Filter agents
        let filtered_agents: Vec<crate::ohc::orchestration::Agent> = agents.iter()
            .filter(|a| a.organization_id == req.organization_id || a.id.starts_with(&format!("{}-", req.organization_id)))
            .cloned()
            .collect();

        // 2. Filter and Map meetings
        let mut mapped_meetings: Vec<crate::ohc::app::MeetingRoom> = meetings.iter()
            .filter(|m| {
                m.participants.iter().any(|p| filtered_agents.iter().any(|a| &a.id == p))
            })
            .map(|m| {
                crate::ohc::app::MeetingRoom {
                    id: m.id.clone(),
                    participants: m.participants.clone(),
                    transcript: m.transcript.iter().map(|msg| {
                        crate::ohc::agent::AgentMessage {
                            id: msg.id.clone(),
                            from_agent_id: msg.from_agent.clone(),
                            to_agent_id: msg.to_agent.clone(),
                            message_type: msg.r#type.clone(),
                            content: msg.content.clone(),
                            meeting_id: msg.meeting_id.clone(),
                            occurred_at_unix: msg.occurred_at_unix,
                        }
                    }).collect(),
                }
            })
            .collect();

        // Mobile Payload Optimization
        if is_mobile {
            for m in &mut mapped_meetings {
                m.transcript.clear();
            }
        }

        let mapped_agents: Vec<crate::ohc::agent::Agent> = filtered_agents.iter().map(|a| {
            let status_val = match a.status.as_str() {
                "IDLE" => 1,
                "ACTIVE" => 2,
                "IN_MEETING" => 3,
                "BLOCKED" => 4,
                _ => 0,
            };
            crate::ohc::agent::Agent {
                id: a.id.clone(),
                role: 0,
                name: a.name.clone(),
                status: status_val,
                organization_id: a.organization_id.clone(),
            }
        }).collect();

        // 3. Fix statuses cross-tenant data leak
        let mut status_map = std::collections::HashMap::new();
        for a in filtered_agents.iter() {
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

        use sqlx::Row;

        let organization = match org_res {
            Ok(Some(row)) => {
                Some(crate::ohc::organization::Organization {
                    id: row.try_get("id").unwrap_or_default(),
                    name: row.try_get("name").unwrap_or_default(),
                    domain: "".to_string(),
                    ceo_id: "".to_string(),
                    created_at_unix: 0,
                    members: vec![],
                    role_profiles: vec![],
                    tier: row.try_get("tier").unwrap_or_default(),
                })
            },
            _ => None,
        };

        let mut products = vec![];
        if let Ok(rows) = products_res {
            for row in rows {
                products.push(crate::ohc::organization::Product {
                    id: row.try_get("id").unwrap_or_default(),
                    organization_id: row.try_get("organization_id").unwrap_or_default(),
                    name: row.try_get("name").unwrap_or_default(),
                    description: row.try_get("description").unwrap_or_default(),
                    price_cents: row.try_get::<i32, _>("price_cents").unwrap_or_default() as i64,
                    currency: row.try_get("currency").unwrap_or_default(),
                    fulfillment_strategy: row.try_get("fulfillment_strategy").unwrap_or_default(),
                    metadata_json: "".to_string(),
                });
            }
        }

        let mut recent_orders = vec![];
        if let Ok(rows) = orders_res {
            for row in rows {
                recent_orders.push(Order {
                    id: row.try_get("id").unwrap_or_default(),
                    customer_id: row.try_get("customer_id").unwrap_or_default(),
                    total_amount: row.try_get::<f32, _>("total_amount").unwrap_or_default() as f64,
                    status: row.try_get("status").unwrap_or_default(),
                });
            }
        }

        Ok(Response::new(DashboardSnapshot {
            organization,
            agents: mapped_agents,
            meetings: mapped_meetings,
            cost_summary: Some(cost_summary),
            statuses,
            updated_at: chrono::Utc::now().to_rfc3339(),
            products,
            recent_orders,
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
