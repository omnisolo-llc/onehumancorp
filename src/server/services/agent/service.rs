use tonic::{Request, Response, Status};
use crate::ohc::orchestration::*;
use crate::ohc::orchestration::agent_manager_service_server::AgentManagerService;
use std::sync::{Arc, RwLock};
use chrono::Utc;
use crate::hub::Hub;

struct CachedData {
    costs: Summary,
    statuses: Vec<StatusCount>,
    expires_at: std::time::Instant,
}

pub struct MyAgentManagerService {
    hub: Arc<Hub>,
    skills: RwLock<Vec<SkillPack>>,
    snapshots: RwLock<Vec<OrgSnapshot>>,
    cache: RwLock<Option<CachedData>>,
}

impl MyAgentManagerService {
    pub fn new(hub: Arc<Hub>) -> Self {
        MyAgentManagerService {
            hub,
            skills: RwLock::new(Vec::new()),
            snapshots: RwLock::new(Vec::new()),
            cache: RwLock::new(None),
        }
    }

    pub async fn get_snapshot(&self) -> DashboardSnapshot {
        // Optimized: Remove spawn_blocking for in-memory operations
        let agents = self.hub.get_agents();
        let mut meetings = self.hub.get_meetings();

        // Mobile Payload Optimization: Truncate transcripts to last 5 messages for dashboard preview
        for meeting in &mut meetings {
            if meeting.transcript.len() > 5 {
                let start = meeting.transcript.len() - 5;
                meeting.transcript = meeting.transcript.drain(start..).collect();
            }
        }

        let now = std::time::Instant::now();
        let (costs, statuses) = {
            let cache_read = self.cache.read().unwrap();
            if let Some(ref c) = *cache_read {
                if c.expires_at > now {
                    (c.costs.clone(), c.statuses.clone())
                } else {
                    drop(cache_read);
                    self.refresh_cache(now, &agents)
                }
            } else {
                drop(cache_read);
                self.refresh_cache(now, &agents)
            }
        };

        DashboardSnapshot {
            meetings,
            costs: Some(costs),
            agents,
            statuses,
            task_queue: vec![],
            queue_length: 0,
            updated_at_unix: Utc::now().timestamp(),
        }
    }

    fn refresh_cache(&self, now: std::time::Instant, agents: &[Agent]) -> (Summary, Vec<StatusCount>) {
        let cost_auditor = self.hub.get_cost_auditor();
        let costs = Summary {
            total_cost_usd: cost_auditor.get_total_cost(),
            total_tokens: cost_auditor.get_total_tokens(),
        };

        let mut status_map = std::collections::HashMap::new();
        for a in agents {
            *status_map.entry(a.status.clone()).or_insert(0) += 1;
        }
        let statuses: Vec<StatusCount> = status_map.into_iter().map(|(status, count)| StatusCount { status, count }).collect();

        let mut cache_write = self.cache.write().unwrap();
        *cache_write = Some(CachedData {
            costs: costs.clone(),
            statuses: statuses.clone(),
            expires_at: now + std::time::Duration::from_secs(5),
        });

        (costs, statuses)
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
        let agents = tokio::task::spawn_blocking({ let hub_clone = self.hub.clone(); move || hub_clone.get_agents() }).await.map_err(|e| tonic::Status::internal(e.to_string()))?;
        let now = Utc::now();
        let identities = agents.into_iter().map(|a| AgentIdentity {
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
        let hub_clone1 = self.hub.clone();
        let hub_clone2 = self.hub.clone();

        let (agents_res, meetings_res) = tokio::join!(
            tokio::task::spawn_blocking(move || hub_clone1.get_agents()),
            tokio::task::spawn_blocking(move || hub_clone2.get_meetings())
        );
        let agents = agents_res.map_err(|e| tonic::Status::internal(e.to_string()))?;
        let meetings = meetings_res.map_err(|e| tonic::Status::internal(e.to_string()))?;

        let mut msg_count = 0;
        for m in &meetings {
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
