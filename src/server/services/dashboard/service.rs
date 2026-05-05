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
        let req = request.into_inner();
        let org_id = req.organization_id.clone();

        let hub1 = self.hub.clone();
        let hub2 = self.hub.clone();
        let hub3 = self.hub.clone();
        let db_clone = self.db.clone();
        let org_id_clone = org_id.clone();

        // Phase 2: Parallel Fetching Optimization
        // Fetch agents, meetings, costs, and organization info concurrently.
        let (agents_res, meetings_res, cost_res, org_res) = tokio::join!(
            tokio::task::spawn_blocking(move || hub1.get_agents_by_org(&org_id_clone)),
            tokio::task::spawn_blocking(move || hub2.get_meetings()),
            tokio::task::spawn_blocking(move || {
                let cost_auditor = hub3.get_cost_auditor();
                (cost_auditor.get_total_cost(), cost_auditor.get_total_tokens(), cost_auditor.get_agent_costs_snapshot())
            }),
            async move {
                use sqlx::Row;
                sqlx::query("SELECT tenant_id as id, business_name as name FROM tenants WHERE tenant_id = $1")
                    .bind(&org_id)
                    .fetch_optional(&db_clone.pool)
                    .await
            }
        );

        let agents = agents_res.map_err(|e| Status::internal(e.to_string()))?;
        let meetings = meetings_res.map_err(|e| Status::internal(e.to_string()))?;
        let (total_cost, total_tokens, _agent_costs_data) = cost_res.map_err(|e| Status::internal(e.to_string()))?;
        let org_row = org_res.map_err(|e| Status::internal(e.to_string()))?;

        let organization = org_row.map(|row| {
            use sqlx::Row;
            crate::ohc::organization::Organization {
                id: row.get("id"),
                name: row.get("name"),
                ..Default::default()
            }
        });

        let mut status_map = std::collections::HashMap::new();
        for a in agents.iter() {
            *status_map.entry(a.status.clone()).or_insert(0) += 1;
        }
        let statuses = status_map.into_iter().map(|(status, count)| StatusCount { status, count: count as u32 }).collect();

        let cost_summary = crate::ohc::billing::CostSummary {
            organization_id: req.organization_id.clone(),
            total_cost_usd: total_cost,
            total_tokens,
            projected_monthly_usd: total_cost * 30.0,
            agents: vec![],
        };

        // Filter and map meetings to the response format
        let filtered_meetings: Vec<MeetingRoom> = meetings.iter()
            .filter(|m| m.participants.iter().any(|p| p.starts_with(&req.organization_id)))
            .cloned()
            .map(|m| MeetingRoom {
                id: m.id,
                participants: m.participants,
                transcript: m.transcript.into_iter().map(|msg| {
                    use crate::ohc::agent::AgentMessage;
                    AgentMessage {
                        id: msg.id,
                        from_agent_id: msg.from_agent,
                        to_agent_id: msg.to_agent,
                        message_type: msg.r#type,
                        content: msg.content,
                        meeting_id: msg.meeting_id,
                        occurred_at_unix: msg.occurred_at_unix,
                    }
                }).collect(),
            })
            .collect();

        Ok(Response::new(DashboardSnapshot {
            organization,
            agents: agents.into_iter().map(|a| crate::ohc::agent::Agent {
                id: a.id,
                name: a.name,
                organization_id: a.organization_id,
                ..Default::default()
            }).collect(),
            meetings: filtered_meetings,
            cost_summary: Some(cost_summary),
            statuses,
            updated_at: chrono::Utc::now().to_rfc3339(),
        }))
    }

    async fn get_lightweight_dashboard(
        &self,
        request: Request<GetDashboardRequest>,
    ) -> Result<Response<LightweightDashboardSnapshot>, Status> {
        let req = request.into_inner();
        let org_id = req.organization_id.clone();

        let hub1 = self.hub.clone();
        let hub2 = self.hub.clone();
        let hub3 = self.hub.clone();
        let db_clone = self.db.clone();
        let org_id_clone = org_id.clone();

        // Performance: Avoid fetching full agent and meeting details.
        let (agents_res, meetings_res, cost_res, org_res) = tokio::join!(
            tokio::task::spawn_blocking(move || hub1.get_agents_by_org(&org_id_clone)),
            tokio::task::spawn_blocking(move || hub2.get_meetings()),
            tokio::task::spawn_blocking(move || {
                let cost_auditor = hub3.get_cost_auditor();
                (cost_auditor.get_total_cost(), cost_auditor.get_total_tokens())
            }),
            async move {
                use sqlx::Row;
                sqlx::query("SELECT tenant_id as id, business_name as name FROM tenants WHERE tenant_id = $1")
                    .bind(&org_id)
                    .fetch_optional(&db_clone.pool)
                    .await
            }
        );

        let agents = agents_res.map_err(|e| Status::internal(e.to_string()))?;
        let meetings = meetings_res.map_err(|e| Status::internal(e.to_string()))?;
        let (total_cost, total_tokens) = cost_res.map_err(|e| Status::internal(e.to_string()))?;
        let org_row = org_res.map_err(|e| Status::internal(e.to_string()))?;

        let organization = org_row.map(|row| {
            use sqlx::Row;
            crate::ohc::organization::Organization {
                id: row.get("id"),
                name: row.get("name"),
                ..Default::default()
            }
        });

        let mut status_map = std::collections::HashMap::new();
        for a in agents.iter() {
            *status_map.entry(a.status.clone()).or_insert(0) += 1;
        }
        let statuses = status_map.into_iter().map(|(status, count)| StatusCount { status, count: count as u32 }).collect();

        let cost_summary = crate::ohc::billing::CostSummary {
            organization_id: req.organization_id.clone(),
            total_cost_usd: total_cost,
            total_tokens,
            projected_monthly_usd: total_cost * 30.0,
            agents: vec![],
        };

        let meeting_count = meetings.iter()
            .filter(|m| m.participants.iter().any(|p| p.starts_with(&req.organization_id)))
            .count() as u32;

        Ok(Response::new(LightweightDashboardSnapshot {
            organization,
            agent_count: agents.len() as u32,
            meeting_count,
            cost_summary: Some(cost_summary),
            statuses,
            updated_at: chrono::Utc::now().to_rfc3339(),
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
