use tonic::{Request, Response, Status};
use crate::ohc::orchestration::*;
use crate::ohc::orchestration::agent_manager_service_server::AgentManagerService;
use std::sync::{Arc, RwLock};
use chrono::Utc;
use crate::hub::Hub;

pub struct MyAgentManagerService {
    hub: Arc<Hub>,
    skills: RwLock<Vec<SkillPack>>,
    snapshots: RwLock<Vec<OrgSnapshot>>,
}

impl MyAgentManagerService {
    pub fn new(hub: Arc<Hub>) -> Self {
        MyAgentManagerService {
            hub,
            skills: RwLock::new(Vec::new()),
            snapshots: RwLock::new(Vec::new()),
        }
    }

    async fn get_snapshot(&self) -> DashboardSnapshot {
        let hub1 = self.hub.clone();
        let hub2 = self.hub.clone();
        let (agents_res, meetings_res) = tokio::join!(
            tokio::task::spawn_blocking(move || hub1.get_agents()),
            tokio::task::spawn_blocking(move || hub2.get_meetings())
        );
        let agents = agents_res.unwrap();
        let meetings = meetings_res.unwrap();
        let cost_auditor = self.hub.get_cost_auditor();
        let agent_costs_summary: Vec<crate::ohc::orchestration::AgentCost> = agents.iter().map(|a| {
            let cost = cost_auditor.get_agent_cost(&a.id);
            let tokens = cost_auditor.get_agent_output_tokens(&a.id);
            let revenue = cost_auditor.get_agent_revenue(&a.id);
            let roi = cost_auditor.calculate_roi(cost, revenue);
            let efficiency = cost_auditor.calculate_efficiency(cost, tokens);

            crate::ohc::orchestration::AgentCost {
                agent_id: a.id.clone(),
                cost_usd: cost,
                roi_pct: roi,
                efficiency_tok_per_usd: efficiency,
            }
        }).collect();

        let costs = Summary {
            agent_costs: agent_costs_summary,
            total_cost_usd: cost_auditor.get_total_cost(),
            total_tokens: cost_auditor.get_total_tokens(),
        };

        let mut status_map = std::collections::HashMap::new();
        for a in agents.iter() {
            *status_map.entry(a.status.clone()).or_insert(0) += 1;
        }
        let statuses = status_map.into_iter().map(|(status, count)| StatusCount { status, count }).collect();

        DashboardSnapshot {
            meetings: meetings.to_vec(),
            costs: Some(costs),
            agents: agents.to_vec(),
            statuses,
            task_queue: vec![],
            queue_length: 0,
            updated_at_unix: Utc::now().timestamp(),
        }
    }
}

#[tonic::async_trait]
impl AgentManagerService for MyAgentManagerService {
    async fn hire_agent(
        &self,
        request: Request<HireAgentRequest>,
    ) -> Result<Response<DashboardSnapshot>, Status> {
        let req = request.into_inner();
        if req.name.is_empty() || req.role.is_empty() {
            return Err(Status::invalid_argument("name and role are required"));
        }

        let id = format!("agent-{}", Utc::now().timestamp());
        let agent = Agent {
            id,
            name: req.name,
            role: req.role,
            organization_id: "system".to_string(),
            status: "IDLE".to_string(),
            provider_type: if req.provider_type.is_empty() { "builtin".to_string() } else { req.provider_type },
        };

        self.hub.register_agent(agent);

        Ok(Response::new(self.get_snapshot().await))
    }

    async fn fire_agent(
        &self,
        request: Request<FireAgentRequest>,
    ) -> Result<Response<DashboardSnapshot>, Status> {
        let req = request.into_inner();
        if req.agent_id.is_empty() {
            return Err(Status::invalid_argument("agentId is required"));
        }

        self.hub.fire_agent(&req.agent_id);

        Ok(Response::new(self.get_snapshot().await))
    }

    async fn delegate_task(
        &self,
        request: Request<DelegateTaskRequest>,
    ) -> Result<Response<DashboardSnapshot>, Status> {
        let req = request.into_inner();
        let task = req.task.ok_or_else(|| Status::invalid_argument("task is required"))?;
        
        if req.from_agent_id.is_empty() || req.to_agent_id.is_empty() {
            return Err(Status::invalid_argument("from_agent_id and to_agent_id are required"));
        }

        self.hub.clone().delegate_task(req.from_agent_id.clone(), req.to_agent_id.clone(), task)
            .map_err(|e| Status::invalid_argument(e))?;

        Ok(Response::new(self.get_snapshot().await))
    }

    async fn get_agent_providers(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<AgentProvidersResponse>, Status> {
        let providers = vec![
            AgentProviderInfo { r#type: "builtin".to_string(), name: "Builtin".to_string(), authenticated: true },
            AgentProviderInfo { r#type: "claude".to_string(), name: "Claude".to_string(), authenticated: false },
        ];
        Ok(Response::new(AgentProvidersResponse { providers }))
    }

    async fn auth_agent_provider(
        &self,
        request: Request<AuthAgentProviderRequest>,
    ) -> Result<Response<AgentProvidersResponse>, Status> {
        let req = request.into_inner();
        if req.provider_type.is_empty() {
            return Err(Status::invalid_argument("providerType is required"));
        }

        Ok(Response::new(AgentProvidersResponse { providers: vec![] }))
    }

    async fn get_identities(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<IdentitiesResponse>, Status> {
        let agents = self.hub.get_agents();
        let now = Utc::now();
        let identities = agents.iter().map(|a| a.clone()).map(|a| AgentIdentity {
            agent_id: a.id.clone(),
            svid: format!("spiffe://onehumancorp.io/system/{}", a.id),
            trust_domain: "onehumancorp.io".to_string(),
            issued_at_unix: now.timestamp(),
            expires_at_unix: (now + chrono::Duration::hours(24)).timestamp(),
        }).collect();

        Ok(Response::new(IdentitiesResponse { identities }))
    }

    async fn get_skills(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<SkillsResponse>, Status> {
        let skills = self.skills.read().unwrap();
        Ok(Response::new(SkillsResponse { skills: skills.clone() }))
    }

    async fn import_skill(
        &self,
        request: Request<ImportSkillRequest>,
    ) -> Result<Response<SkillPack>, Status> {
        let req = request.into_inner();
        if req.name.is_empty() || req.domain.is_empty() {
            return Err(Status::invalid_argument("name and domain are required"));
        }

        let now = Utc::now();
        let pack = SkillPack {
            id: format!("skill-{}", now.timestamp()),
            name: req.name,
            domain: req.domain,
            description: req.description,
            source: if req.source.is_empty() { "custom".to_string() } else { req.source },
            author: req.author,
            roles: req.roles,
            imported_at_unix: now.timestamp(),
        };

        let mut skills = self.skills.write().unwrap();
        skills.push(pack.clone());

        Ok(Response::new(pack))
    }

    async fn get_snapshots(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<SnapshotsResponse>, Status> {
        let snapshots = self.snapshots.read().unwrap();
        Ok(Response::new(SnapshotsResponse { snapshots: snapshots.clone() }))
    }

    async fn create_snapshot(
        &self,
        request: Request<CreateSnapshotRequest>,
    ) -> Result<Response<OrgSnapshot>, Status> {
        let req = request.into_inner();
        let hub1 = self.hub.clone();
        let hub2 = self.hub.clone();
        let (agents_res, meetings_res) = tokio::join!(
            tokio::task::spawn_blocking(move || hub1.get_agents()),
            tokio::task::spawn_blocking(move || hub2.get_meetings())
        );
        let agents = agents_res.unwrap();
        let meetings = meetings_res.unwrap();

        let mut msg_count = 0;
        for m in meetings.iter() {
            msg_count += m.transcript.len() as i32;
        }

        let now = Utc::now();
        let label = if req.label.is_empty() {
            format!("Snapshot {}", now.format("%Y-%m-%d %H:%M"))
        } else {
            req.label
        };

        let snap = OrgSnapshot {
            id: format!("snap-{}", now.timestamp()),
            label,
            org_id: "system".to_string(),
            org_name: "System".to_string(),
            domain: "default".to_string(),
            agent_count: agents.len() as i32,
            meeting_count: meetings.len() as i32,
            message_count: msg_count,
            created_at_unix: now.timestamp(),
        };

        let mut snapshots = self.snapshots.write().unwrap();
        snapshots.push(snap.clone());

        Ok(Response::new(snap))
    }

    async fn restore_snapshot(
        &self,
        request: Request<RestoreSnapshotRequest>,
    ) -> Result<Response<DashboardSnapshot>, Status> {
        let req = request.into_inner();
        if req.snapshot_id.is_empty() {
            return Err(Status::invalid_argument("snapshotId is required"));
        }

        Ok(Response::new(self.get_snapshot().await))
    }
}
