use tonic::{Request, Response, Status};
use crate::ohc::app::*;
use crate::ohc::app::dashboard_service_server::DashboardService;
use std::sync::Arc;

pub struct MyDashboardService {
    db: Arc<crate::db::DB>,
    hub: Arc<crate::hub::Hub>,
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
        _request: Request<GetDashboardRequest>,
    ) -> Result<Response<DashboardSnapshot>, Status> {
        let req = _request.into_inner();
        let org_id = req.organization_id;

        let hub_clone_1 = self.hub.clone();
        let org_id_1 = org_id.clone();

        let hub_clone_2 = self.hub.clone();
        let org_id_2 = org_id.clone();

        let hub_clone_3 = self.hub.clone();

        // Execute operations concurrently using spawn_blocking to offload synchronous work from the async executor
        let (agents_res, meetings_res, cost_res) = tokio::join!(
            tokio::task::spawn_blocking(move || {
                Ok::<_, Status>(hub_clone_1.get_agents_by_org(&org_id_1))
            }),
            tokio::task::spawn_blocking(move || {
                let all_meetings = hub_clone_2.get_meetings();
                let org_prefix = format!("{}-", org_id_2);
                let filtered_meetings: Vec<_> = all_meetings
                    .iter()
                    .filter(|m| m.participants.iter().any(|p| p.starts_with(&org_prefix)))
                    .cloned()
                    .collect();
                Ok::<_, Status>(filtered_meetings)
            }),
            tokio::task::spawn_blocking(move || {
                let cost_auditor = hub_clone_3.get_cost_auditor();
                let total_cost = cost_auditor.get_total_cost();
                let _total_tokens = cost_auditor.get_total_tokens();
                let agent_costs_data = cost_auditor.get_agent_costs_snapshot();

                Ok::<_, Status>((total_cost, 0, agent_costs_data))
            })
        );

        let agents = agents_res.map_err(|e| Status::internal(e.to_string()))??;
        let meetings = meetings_res.map_err(|e| Status::internal(e.to_string()))??;
        let (_, _, global_agent_costs_data) = cost_res.map_err(|e| Status::internal(e.to_string()))??;

        let agent_ids: std::collections::HashSet<_> = agents.iter().map(|a| a.id.clone()).collect();
        let agent_costs_data: Vec<_> = global_agent_costs_data
            .into_iter()
            .filter(|(agent_id, _, _, _)| agent_ids.contains(agent_id))
            .collect();

        let total_cost: f64 = agent_costs_data.iter().map(|(_, cost, _, _)| cost).sum();






        let mut agent_costs = Vec::new();
        for (name, cost, roi, efficiency) in agent_costs_data {
            let pct = if total_cost > 0.0 { (cost / total_cost) as f32 } else { 0.0 };
            agent_costs.push(crate::ohc::billing::AgentCostSummary {
                agent_id: name,
                cost_usd: cost,
                roi,
                efficiency,
                pct,
                token_used: 0,
            });
        }

        let costs = crate::ohc::billing::CostSummary {
            organization_id: org_id.clone(),
            total_cost_usd: total_cost,
            total_tokens: 0, // Global total_tokens leak prevented,
            projected_monthly_usd: total_cost * 30.0,
            agents: agent_costs,
        };

        let mut status_map = std::collections::HashMap::new();
        for a in agents.iter() {
            *status_map.entry(a.status.clone()).or_insert(0) += 1;
        }
        let statuses = status_map.into_iter().map(|(status, count)| crate::ohc::app::StatusCount { status, count }).collect();

        let agents_proto: Vec<crate::ohc::agent::Agent> = agents.iter().map(|a| {
            crate::ohc::agent::Agent {
                id: a.id.clone(),
                role: match a.role.as_str() {
                    "CEO" => crate::ohc::common::Role::Ceo as i32,
                    "PRODUCT_MANAGER" => crate::ohc::common::Role::ProductManager as i32,
                    "SOFTWARE_ENGINEER" => crate::ohc::common::Role::SoftwareEngineer as i32,
                    _ => crate::ohc::common::Role::Unspecified as i32,
                },
                name: a.name.clone(),
                status: match a.status.as_str() {
                    "IDLE" => crate::ohc::common::AgentStatus::Idle as i32,
                    "ACTIVE" => crate::ohc::common::AgentStatus::Active as i32,
                    "IN_MEETING" => crate::ohc::common::AgentStatus::InMeeting as i32,
                    "BLOCKED" => crate::ohc::common::AgentStatus::Blocked as i32,
                    _ => crate::ohc::common::AgentStatus::StatusUnspecified as i32,
                },
                organization_id: a.organization_id.clone(),

            }
        }).collect();

        let meetings_proto: Vec<crate::ohc::api::v1::MeetingRoom> = meetings.iter().map(|m| {
            crate::ohc::api::v1::MeetingRoom {
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
        }).collect();

        Ok(Response::new(DashboardSnapshot {
            organization: None,
            meetings: meetings_proto,
            cost_summary: Some(costs),
            agents: agents_proto,
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
    use std::sync::Arc;
    use tokio::sync::mpsc;
    use crate::ohc::app::GetDashboardRequest;

    #[tokio::test]
    async fn test_get_dashboard_success() {
        let (tx, _) = mpsc::channel(100);

        let pool = sqlx::postgres::PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy("postgres://dummydatabase:5432/ohc")
            .unwrap();

        let db = Arc::new(crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });
        let hub = Arc::new(crate::hub::Hub::new(tx, pool));

        hub.register_agent(crate::ohc::orchestration::Agent {
            id: "org1-agent".to_string(),
            name: "Agent 1".to_string(),
            role: "SOFTWARE_ENGINEER".to_string(),
            organization_id: "org1".to_string(),
            status: "ACTIVE".to_string(),
            provider_type: "test".to_string(),
        });

        let service = MyDashboardService::new(db, hub);
        let req = Request::new(GetDashboardRequest {
            organization_id: "org1".to_string(),
        });

        // This relies on lazy connection so it doesn't fail setup, but actually executing get_dashboard
        // will trigger sqlx queries if get_agents etc query the DB.
        // Oh wait, hub.get_agents() reads from RwLock memory! It doesn't hit the DB in this test!
        // So the query will succeed without a real Postgres instance!

        let response = service.get_dashboard(req).await.unwrap();
        let snapshot = response.into_inner();
        assert!(snapshot.cost_summary.is_some());
        assert_eq!(snapshot.agents.len(), 1);
        assert_eq!(snapshot.agents[0].id, "org1-agent");
    }
}
