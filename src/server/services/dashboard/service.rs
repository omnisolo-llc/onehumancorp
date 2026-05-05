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
        request: Request<GetDashboardRequest>,
    ) -> Result<Response<DashboardSnapshot>, Status> {
        let req = request.into_inner();
        let org_id = req.organization_id;

        let hub1 = self.hub.clone();
        let hub2 = self.hub.clone();
        let hub3 = self.hub.clone();

        let (agents_res, meetings_res, cost_res) = tokio::join!(
            tokio::task::spawn_blocking(move || hub1.get_agents()),
            tokio::task::spawn_blocking(move || hub2.get_meetings()),
            tokio::task::spawn_blocking(move || {
                let cost_auditor = hub3.get_cost_auditor();
                (cost_auditor.get_total_cost(), cost_auditor.get_total_tokens(), cost_auditor.get_agent_costs_snapshot())
            })
        );

        let agents = agents_res.map_err(|e| Status::internal(e.to_string()))?;
        let meetings = meetings_res.map_err(|e| Status::internal(e.to_string()))?;
        let (total_cost, total_tokens, agent_costs_data) = cost_res.map_err(|e| Status::internal(e.to_string()))?;

        let org_row = sqlx::query("SELECT name, domain, tier FROM organizations WHERE id = $1 LIMIT 1")
            .bind(&org_id)
            .fetch_optional(&self.db.pool)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let org = if let Some(r) = org_row {
            use sqlx::Row;
            crate::ohc::organization::Organization {
                id: org_id.clone(),
                name: r.try_get("name").unwrap_or_else(|_| "Default Organization".to_string()),
                domain: r.try_get("domain").unwrap_or_else(|_| "default".to_string()),
                ceo_id: "".to_string(),
                created_at_unix: 0,
                members: vec![],
                role_profiles: vec![],
                tier: r.try_get("tier").unwrap_or_else(|_| "free".to_string()),
            }
        } else {
            crate::ohc::organization::Organization {
                id: org_id.clone(),
                name: "Default Organization".to_string(),
                domain: "default".to_string(),
                ceo_id: "".to_string(),
                created_at_unix: 0,
                members: vec![],
                role_profiles: vec![],
                tier: "free".to_string(),
            }
        };

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

        let cost_summary = crate::ohc::billing::CostSummary {
            organization_id: org.id.clone(),
            total_cost_usd: total_cost,
            total_tokens,
            projected_monthly_usd: total_cost * 30.0,
            agents: agent_costs,
        };

        let mut status_map = std::collections::HashMap::new();
        for a in agents.iter() {
            *status_map.entry(a.status.clone()).or_insert(0) += 1;
        }
        let statuses = status_map.into_iter().map(|(status, count)| crate::ohc::app::StatusCount { status, count: count as u32 }).collect();


        let mut app_agents = Vec::new();
        for a in agents.iter() {
            app_agents.push(crate::ohc::agent::Agent {
                id: a.id.clone(),
                role: 0,
                name: a.name.clone(),
                status: 0,
                organization_id: a.organization_id.clone(),
            });
        }

        let mut app_meetings = Vec::new();
        for m in meetings.iter() {
            let mut app_transcript = Vec::new();
            for t in m.transcript.iter() {
                app_transcript.push(crate::ohc::agent::AgentMessage {
                    id: t.id.clone(),
                    from_agent_id: t.from_agent.clone(),
                    to_agent_id: "".to_string(),
                    message_type: "text".to_string(),
                    content: t.content.clone(),
                    meeting_id: m.id.clone(),
                    occurred_at_unix: t.occurred_at_unix,
                });
            }
            app_meetings.push(crate::ohc::app::MeetingRoom {
                id: m.id.clone(),
                participants: m.participants.clone(),
                transcript: app_transcript,
            });
        }

        Ok(Response::new(DashboardSnapshot {
            organization: Some(org),
            agents: app_agents,
            meetings: app_meetings,
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
    async fn test_get_dashboard() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if database_url == "postgres://localhost/dummy" || !matches!(tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&database_url)).await, Ok(Ok(_))) {
            return;
        }

        let pool = sqlx::PgPool::connect(&database_url).await.unwrap();
        let store = crate::db::DbStore::Postgres;
        let db = Arc::new(crate::db::DB { pool: pool.clone(), store });

        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(crate::hub::Hub::new(tx, pool));

        let service = MyDashboardService::new(db, hub);

        let req = Request::new(GetDashboardRequest {
            organization_id: "system".to_string(),
        });

        let resp = service.get_dashboard(req).await;
        assert!(resp.is_ok());
    }
}
