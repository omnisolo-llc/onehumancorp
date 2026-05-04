use tonic::{Request, Response, Status};
use crate::ohc::app::*;
use crate::ohc::app::dashboard_service_server::DashboardService;
use std::sync::Arc;
use crate::hub::Hub;
use chrono::Utc;

pub struct MyDashboardService {
    db: Arc<crate::db::DB>,
    hub: Arc<Hub>,
}

impl MyDashboardService {
    pub fn new(db: Arc<crate::db::DB>, hub: Arc<Hub>) -> Self {
        Self { db, hub }
    }
}

#[tonic::async_trait]
impl DashboardService for MyDashboardService {
    async fn get_dashboard(
        &self,
        _request: Request<GetDashboardRequest>,
    ) -> Result<Response<DashboardSnapshot>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }

    async fn get_lightweight_dashboard(
        &self,
        request: Request<GetDashboardRequest>,
    ) -> Result<Response<LightweightDashboardSnapshot>, Status> {
        let req = request.into_inner();
        let org_id = req.organization_id;

        let hub1 = self.hub.clone();
        let hub2 = self.hub.clone();
        let (agents_res, meetings_res) = tokio::join!(
            tokio::task::spawn_blocking(move || hub1.get_agents()),
            tokio::task::spawn_blocking(move || hub2.get_meetings())
        );

        let agents = agents_res.map_err(|e| Status::internal(e.to_string()))?;
        let meetings = meetings_res.map_err(|e| Status::internal(e.to_string()))?;

        let cost_auditor = self.hub.get_cost_auditor();
        let auditor1 = cost_auditor.clone();
        let auditor2 = cost_auditor.clone();
        let auditor3 = cost_auditor.clone();

        let (total_cost_res, total_tokens_res, agent_costs_res) = tokio::join!(
            tokio::task::spawn_blocking(move || auditor1.get_total_cost()),
            tokio::task::spawn_blocking(move || auditor2.get_total_tokens()),
            tokio::task::spawn_blocking(move || auditor3.get_agent_costs_snapshot())
        );

        let total_cost = total_cost_res.map_err(|e| Status::internal(e.to_string()))?;
        let total_tokens = total_tokens_res.map_err(|e| Status::internal(e.to_string()))?;
        let agent_costs_data = agent_costs_res.map_err(|e| Status::internal(e.to_string()))?;

        let mut status_map = std::collections::HashMap::new();
        for a in agents.iter() {
            *status_map.entry(a.status.clone()).or_insert(0) += 1;
        }
        let statuses = status_map.into_iter().map(|(status, count)| StatusCount { status, count: count as u32 }).collect();

        let mut agent_costs = Vec::new();
        for (name, cost, _roi, _efficiency) in agent_costs_data {
            agent_costs.push(crate::ohc::billing::AgentCostSummary {
                agent_id: name,
                cost_usd: cost,
                token_used: 0, // Simplified for lightweight
            });
        }

        let cost_summary = crate::ohc::billing::CostSummary {
            organization_id: org_id.clone(),
            total_cost_usd: total_cost,
            total_tokens,
            projected_monthly_usd: total_cost * 30.0, // Rough estimate
            agents: agent_costs,
        };

        Ok(Response::new(LightweightDashboardSnapshot {
            organization: None, // Simplified
            agent_count: agents.len() as u32,
            meeting_count: meetings.len() as u32,
            cost_summary: Some(cost_summary),
            statuses,
            updated_at: Utc::now().to_rfc3339(),
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
    use tokio::sync::mpsc;
    use crate::hub::Hub;

    #[tokio::test]
    async fn test_get_lightweight_dashboard() {
        if std::env::var("DATABASE_URL").is_err() { return; }
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy(&db_url).unwrap();

        let (tx, _) = mpsc::channel(100);
        let hub = Arc::new(Hub::new(tx, pool.clone()));
        let db = Arc::new(crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });
        let service = MyDashboardService::new(db, hub);

        let req = tonic::Request::new(GetDashboardRequest {
            organization_id: "org-1".to_string(),
        });

        let resp = service.get_lightweight_dashboard(req).await.unwrap();
        let snapshot = resp.into_inner();

        assert!(snapshot.cost_summary.is_some());
        assert!(snapshot.updated_at.len() > 0);
    }
}
