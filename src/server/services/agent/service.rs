use tonic::{Request, Response, Status};
use ::server_ohc::orchestration::*;
use ::server_ohc::orchestration::agent_manager_service_server::AgentManagerService;
use std::sync::{Arc, RwLock};
use chrono::Utc;
use crate::hub::Hub;

pub struct MyAgentManagerService {
    hub: Arc<Hub>,
    skills: RwLock<std::collections::HashMap<String, Vec<SkillPack>>>,
    snapshots: RwLock<std::collections::HashMap<String, Vec<OrgSnapshot>>>,
    snapshot_cache: ::server_utils::cache::HybridCache<DashboardSnapshot>,
}

impl MyAgentManagerService {
    pub fn new(hub: Arc<Hub>) -> Self {
        let redis_client = hub.redis_client.clone();
        MyAgentManagerService {
            hub,
            skills: RwLock::new(std::collections::HashMap::new()),
            snapshots: RwLock::new(std::collections::HashMap::new()),
            snapshot_cache: ::server_utils::cache::HybridCache::new(redis_client),
        }
    }

    async fn get_snapshot(&self, org_id: &str, mobile_optimized: bool) -> Result<DashboardSnapshot, Status> {
        let cache_key = format!("agent_dashboard_snapshot_{}:mobile:{}", org_id, mobile_optimized);
        if let Some(snapshot) = self.snapshot_cache.get(&cache_key).await {
            return Ok(snapshot);
        }

        let hub_cost = self.hub.clone();
        let hub_agents = self.hub.clone();
        let hub_meetings = self.hub.clone();
        let hub_tasks = self.hub.clone();
        let org_id_clone = org_id.to_string();
        let org_id_clone_for_cost = org_id.to_string();
        let org_id_clone_for_agents = org_id.to_string();
        let org_id_clone_for_meetings = org_id.to_string();
        let (agents_res, meetings_res, cost_res_spawn, tasks_res) = if mobile_optimized {
            let (r1, r2) = tokio::join!(
                tokio::task::spawn_blocking(move || Arc::new(hub_agents.get_agents_by_org(&org_id_clone_for_agents))),
                tokio::spawn(async move { hub_meetings.get_meetings_by_org(&org_id_clone_for_meetings).await })
            );
            (r1, r2, Ok((0.0, 0, vec![])), Ok(vec![]))
        } else {
            tokio::join!(
                tokio::task::spawn_blocking(move || Arc::new(hub_agents.get_agents_by_org(&org_id_clone_for_agents))),
                tokio::spawn(async move { hub_meetings.get_meetings_by_org(&org_id_clone_for_meetings).await }),
                tokio::task::spawn_blocking(move || {
                    let cost_auditor = hub_cost.get_cost_auditor();
                    (
                        cost_auditor.get_tenant_cost(&org_id_clone_for_cost),
                        cost_auditor.get_tenant_tokens(&org_id_clone_for_cost),
                        cost_auditor.get_agent_costs_snapshot(),
                    )
                }),
                tokio::spawn(async move {
                    hub_tasks.task_manager().get_pending_approvals(&org_id_clone)
                })
            )
        };
        let agents = agents_res.unwrap();
        let meetings = meetings_res.unwrap();
        let (total_cost, total_tokens, agent_costs_data) = cost_res_spawn.unwrap();
        let task_queue = tasks_res.unwrap();
        let queue_length = task_queue.len() as i32;
        let proto_task_queue = task_queue.into_iter().map(|t| t.into_proto()).collect();
        let mut proto_task_queue_mut: Vec<::server_ohc::orchestration::SharedTask> = proto_task_queue;
        if mobile_optimized {
            for task in proto_task_queue_mut.iter_mut() {
                task.description.clear();
                task.payload.clear();
            }
        }

        let mut agent_costs = Vec::new();
        let org_agent_ids: std::collections::HashSet<&str> =
            agents.iter().map(|agent| agent.id.as_str()).collect();
        for (name, cost, _token_used, roi, efficiency, _storage) in agent_costs_data {
            if !org_agent_ids.contains(name.as_str()) {
                continue;
            }
            let pct = if total_cost > 0.0 { (cost / total_cost) as f32 } else { 0.0 };
            agent_costs.push(AgentCostSummary {
                name: if mobile_optimized { String::new() } else { name },
                cost_usd: cost,
                roi,
                efficiency,
                pct,
            });
        }

        let costs = Summary {
            total_cost_usd: total_cost,
            agent_costs,
            total_tokens,
        };

        let mut agents_list = Arc::unwrap_or_clone(agents);
        let mut meetings_list = Arc::unwrap_or_clone(meetings);
        if mobile_optimized {
            for agent in agents_list.iter_mut() {
                agent.name = String::new();
                agent.role = String::new();
                agent.organization_id = String::new();
            }
            for meeting in meetings_list.iter_mut() {
                meeting.transcript.clear();
            }
        }

        let mut status_map = std::collections::HashMap::new();
        for a in agents_list.iter() {
            *status_map.entry(a.status.clone()).or_insert(0) += 1;
        }
        let statuses = status_map.into_iter().map(|(status, count)| StatusCount { status, count }).collect();

        let snapshot = DashboardSnapshot {
            meetings: meetings_list,
            costs: Some(costs),
            agents: agents_list,
            statuses,
            task_queue: proto_task_queue_mut,
            queue_length,
            updated_at_unix: Utc::now().timestamp(),
        };
        self.snapshot_cache.set(&cache_key, snapshot.clone(), std::time::Duration::from_secs(5)).await;
        Ok(snapshot)
    }
}

fn authenticated_org<T>(request: &Request<T>) -> Result<String, Status> {
    let identity = ::server_auth::extract_spiffe_id_from_metadata(request.metadata())
        .map_err(Status::unauthenticated)?;
    let (org_id, _) = ::server_auth::parse_spiffe_id(&identity)?;
    if org_id.is_empty() {
        return Err(Status::unauthenticated("SPIFFE organization is empty"));
    }
    Ok(org_id)
}

#[tonic::async_trait]
impl AgentManagerService for MyAgentManagerService {
    async fn hire_agent(
        &self,
        request: Request<HireAgentRequest>,
    ) -> Result<Response<DashboardSnapshot>, Status> {
        let org_id = authenticated_org(&request)?;
        let req = request.into_inner();
        if req.name.is_empty() || req.role.is_empty() {
            return Err(Status::invalid_argument("name and role are required"));
        }

        let id = format!("agent-{}", Utc::now().timestamp());
        let agent = Agent {
            id,
            name: req.name,
            role: req.role,
            organization_id: org_id.clone(),
            status: "IDLE".to_string(),
            provider_type: if req.provider_type.is_empty() { "builtin".to_string() } else { req.provider_type },
        };

        self.hub.register_agent(agent);
        self.snapshot_cache.invalidate(&format!("agent_dashboard_snapshot_{}:mobile:false", org_id)).await;
        self.snapshot_cache.invalidate(&format!("agent_dashboard_snapshot_{}:mobile:true", org_id)).await;
        Ok(Response::new(self.get_snapshot(&org_id, false).await?))
    }

    async fn fire_agent(
        &self,
        request: Request<FireAgentRequest>,
    ) -> Result<Response<DashboardSnapshot>, Status> {
        let org_id = authenticated_org(&request)?;
        let req = request.into_inner();
        if req.agent_id.is_empty() {
            return Err(Status::invalid_argument("agentId is required"));
        }

        let agent = self
            .hub
            .get_agent(&req.agent_id)
            .ok_or_else(|| Status::permission_denied("agent does not belong to organization"))?;
        if agent.organization_id != org_id {
            return Err(Status::permission_denied(
                "agent does not belong to organization",
            ));
        }
        self.hub.fire_agent(&req.agent_id);
        self.snapshot_cache.invalidate(&format!("agent_dashboard_snapshot_{}:mobile:false", org_id)).await;
        self.snapshot_cache.invalidate(&format!("agent_dashboard_snapshot_{}:mobile:true", org_id)).await;
        Ok(Response::new(self.get_snapshot(&org_id, false).await?))
    }

    async fn delegate_task(
        &self,
        request: Request<DelegateTaskRequest>,
    ) -> Result<Response<DashboardSnapshot>, Status> {
        let org_id = authenticated_org(&request)?;
        let req = request.into_inner();
        let task = req.task.ok_or_else(|| Status::invalid_argument("task is required"))?;
        
        if req.from_agent_id.is_empty() || req.to_agent_id.is_empty() {
            return Err(Status::invalid_argument("from_agent_id and to_agent_id are required"));
        }

        for agent_id in [&req.from_agent_id, &req.to_agent_id] {
            let agent = self
                .hub
                .get_agent(agent_id)
                .ok_or_else(|| Status::permission_denied("agent does not belong to organization"))?;
            if agent.organization_id != org_id {
                return Err(Status::permission_denied(
                    "agent does not belong to organization",
                ));
            }
        }

        self.hub.clone().delegate_task(req.from_agent_id.clone(), req.to_agent_id.clone(), task)
            .map_err(|e| Status::invalid_argument(e))?;
        self.snapshot_cache.invalidate(&format!("agent_dashboard_snapshot_{}:mobile:false", org_id)).await;
        self.snapshot_cache.invalidate(&format!("agent_dashboard_snapshot_{}:mobile:true", org_id)).await;
        Ok(Response::new(self.get_snapshot(&org_id, false).await?))
    }

    async fn get_agent_providers(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<AgentProvidersResponse>, Status> {
        authenticated_org(&request)?;
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
        authenticated_org(&request)?;
        let req = request.into_inner();
        if req.provider_type.is_empty() {
            return Err(Status::invalid_argument("providerType is required"));
        }

        Ok(Response::new(AgentProvidersResponse { providers: vec![] }))
    }

    async fn get_identities(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<IdentitiesResponse>, Status> {
        let org_id = authenticated_org(&request)?;
        let agents = self.hub.get_agents_by_org(&org_id);
        let now = Utc::now();
        let identities = agents.into_iter().map(|a| AgentIdentity {
            agent_id: a.id.clone(),
            svid: format!("spiffe://onehumancorp.io/org/{org_id}/agent/{}", a.id),
            trust_domain: "onehumancorp.io".to_string(),
            issued_at_unix: now.timestamp(),
            expires_at_unix: (now + chrono::Duration::hours(24)).timestamp(),
        }).collect();

        Ok(Response::new(IdentitiesResponse { identities }))
    }

    async fn get_skills(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<SkillsResponse>, Status> {
        let org_id = authenticated_org(&request)?;
        let skills = self.skills.read().unwrap();
        Ok(Response::new(SkillsResponse {
            skills: skills.get(&org_id).cloned().unwrap_or_default(),
        }))
    }

    async fn import_skill(
        &self,
        request: Request<ImportSkillRequest>,
    ) -> Result<Response<SkillPack>, Status> {
        let org_id = authenticated_org(&request)?;
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
        skills.entry(org_id).or_default().push(pack.clone());

        Ok(Response::new(pack))
    }

    async fn get_snapshots(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<SnapshotsResponse>, Status> {
        let org_id = authenticated_org(&request)?;
        let snapshots = self.snapshots.read().unwrap();
        Ok(Response::new(SnapshotsResponse {
            snapshots: snapshots.get(&org_id).cloned().unwrap_or_default(),
        }))
    }

    async fn create_snapshot(
        &self,
        request: Request<CreateSnapshotRequest>,
    ) -> Result<Response<OrgSnapshot>, Status> {
        let org_id = authenticated_org(&request)?;
        let req = request.into_inner();
        let hub1 = self.hub.clone();
        let hub2 = self.hub.clone();
        let org_id_clone_for_agents = org_id.clone();
        let org_id_clone_for_meetings = org_id.clone();
        let (agents_res, meetings_res) = tokio::join!(
            tokio::task::spawn_blocking(move || Arc::new(hub1.get_agents_by_org(&org_id_clone_for_agents))),
            tokio::spawn(async move { hub2.get_meetings_by_org(&org_id_clone_for_meetings).await })
        );
        let agents = agents_res.map_err(|e| Status::internal(e.to_string()))?;
        let meetings = meetings_res.map_err(|e| Status::internal(e.to_string()))?;

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
            org_id: org_id.clone(),
            org_name: "System".to_string(),
            domain: "default".to_string(),
            agent_count: agents.len() as i32,
            meeting_count: meetings.len() as i32,
            message_count: msg_count,
            created_at_unix: now.timestamp(),
        };

        let mut snapshots = self.snapshots.write().unwrap();
        snapshots.entry(org_id).or_default().push(snap.clone());

        Ok(Response::new(snap))
    }


    async fn get_dashboard_snapshot(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<DashboardSnapshot>, Status> {
        let org_id_req = authenticated_org(&request)?;
        let mobile_optimized = request.metadata().get("x-mobile-optimized").map(|v| v.to_str().unwrap_or("false") == "true").unwrap_or(false);
        Ok(Response::new(self.get_snapshot(&org_id_req, mobile_optimized).await?))
    }

    async fn restore_snapshot(
        &self,
        request: Request<RestoreSnapshotRequest>,
    ) -> Result<Response<DashboardSnapshot>, Status> {
        let org_id = authenticated_org(&request)?;
        let req = request.into_inner();
        if req.snapshot_id.is_empty() {
            return Err(Status::invalid_argument("snapshotId is required"));
        }
        let found = {
            let snapshots = self.snapshots.read().unwrap();
            snapshots
                .get(&org_id)
                .is_some_and(|items| items.iter().any(|snapshot| snapshot.id == req.snapshot_id))
        };
        if !found {
            return Err(Status::permission_denied(
                "snapshot does not belong to organization",
            ));
        }

        Ok(Response::new(self.get_snapshot(&org_id, false).await?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::server_auth::orchestration::AuthInfo;
    use tonic::Request;

    fn request_for_org<T>(message: T, org_id: &str) -> Request<T> {
        let mut request = Request::new(message);
        let identity = format!(
            "spiffe://onehumancorp.io/org/{org_id}/agent/test-agent"
        );
        request
            .metadata_mut()
            .insert("x-spiffe-id", identity.parse().unwrap());
        request
    }


    async fn setup_test_agent_manager_service() -> MyAgentManagerService {
        let database_url = "sqlite::memory:";
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(1))
            .connect(database_url).await.unwrap();

        let pg_pool = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
        let db = Arc::new(crate::db::DB { pool: pg_pool, store: crate::db::DbStore::Sqlite(pool.clone()) });

        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(crate::hub::Hub::new(tx, db.pool.clone()));

        MyAgentManagerService::new(hub)
    }

    #[tokio::test]
    async fn cross_org_resources_and_mutations_are_rejected() {
        let service = setup_test_agent_manager_service().await;
        service.hub.register_agent(Agent {
            id: "org-a-agent".to_string(),
            name: "Agent A".to_string(),
            role: "assistant".to_string(),
            organization_id: "org-a".to_string(),
            status: "IDLE".to_string(),
            provider_type: "builtin".to_string(),
        });
        service
            .hub
            .get_cost_auditor()
            .record_manual_cost("org-b-agent", "org-b", 500);
        service.hub.register_agent(Agent {
            id: "org-b-agent".to_string(),
            name: "Agent B".to_string(),
            role: "assistant".to_string(),
            organization_id: "org-b".to_string(),
            status: "IDLE".to_string(),
            provider_type: "builtin".to_string(),
        });
        service.hub.register_agent(Agent {
            id: "org-a-spoofed-prefix".to_string(),
            name: "Spoofed prefix".to_string(),
            role: "assistant".to_string(),
            organization_id: "org-b".to_string(),
            status: "IDLE".to_string(),
            provider_type: "builtin".to_string(),
        });

        let identities = service
            .get_identities(request_for_org(EmptyRequest {}, "org-a"))
            .await
            .unwrap()
            .into_inner();
        assert_eq!(identities.identities.len(), 1);
        assert_eq!(identities.identities[0].agent_id, "org-a-agent");

        let dashboard = service
            .get_dashboard_snapshot(request_for_org(EmptyRequest {}, "org-a"))
            .await
            .unwrap()
            .into_inner();
        let costs = dashboard.costs.unwrap();
        assert_eq!(costs.total_cost_usd, 0.0);
        assert!(costs.agent_costs.is_empty());

        service
            .import_skill(request_for_org(
                ImportSkillRequest {
                    name: "Org B skill".to_string(),
                    domain: "operations".to_string(),
                    ..Default::default()
                },
                "org-b",
            ))
            .await
            .unwrap();
        let skills = service
            .get_skills(request_for_org(EmptyRequest {}, "org-a"))
            .await
            .unwrap()
            .into_inner();
        assert!(skills.skills.is_empty());

        service
            .create_snapshot(request_for_org(
                CreateSnapshotRequest {
                    label: "Org B snapshot".to_string(),
                },
                "org-b",
            ))
            .await
            .unwrap();
        let snapshots = service
            .get_snapshots(request_for_org(EmptyRequest {}, "org-a"))
            .await
            .unwrap()
            .into_inner();
        assert!(snapshots.snapshots.is_empty());

        let delegate_error = service
            .delegate_task(request_for_org(
                DelegateTaskRequest {
                    from_agent_id: "org-a-agent".to_string(),
                    to_agent_id: "org-b-agent".to_string(),
                    task: Some(Message {
                        content: "cross-tenant task".to_string(),
                        ..Default::default()
                    }),
                },
                "org-a",
            ))
            .await
            .unwrap_err();
        assert_eq!(delegate_error.code(), tonic::Code::PermissionDenied);

        let error = service
            .fire_agent(request_for_org(
                FireAgentRequest {
                    agent_id: "org-b-agent".to_string(),
                },
                "org-a",
            ))
            .await
            .unwrap_err();
        assert_eq!(error.code(), tonic::Code::PermissionDenied);
        assert!(service.hub.get_agent("org-b-agent").is_some());
    }

    #[tokio::test]
    async fn test_agent_hire_and_fire() {
        let service = setup_test_agent_manager_service().await;

        // Hire Agent
        let req = HireAgentRequest {
            name: "Test Agent".to_string(),
            role: "test_role".to_string(),
            provider_type: "builtin".to_string(),
            model: "".to_string(),
            region: "".to_string(),
        };
        let mut request = Request::new(req);
        let mut metadata = tonic::metadata::MetadataMap::new();
        metadata.insert("x-spiffe-id", "spiffe://onehumancorp.io/org/system/agent/test".parse().unwrap());
        *request.metadata_mut() = metadata;
        request.extensions_mut().insert(AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: "system".to_string(),
            agent_id: "test".to_string(),
        });

        let res = service.hire_agent(request).await.unwrap().into_inner();
        assert_eq!(res.agents.len(), 1);
        assert_eq!(res.agents[0].name, "Test Agent");

        let agent_id = res.agents[0].id.clone();

        // Fire Agent
        let fire_req = FireAgentRequest {
            agent_id,
        };
        let mut fire_request = Request::new(fire_req);
        let mut metadata2 = tonic::metadata::MetadataMap::new();
        metadata2.insert("x-spiffe-id", "spiffe://onehumancorp.io/org/system/agent/test".parse().unwrap());
        *fire_request.metadata_mut() = metadata2;
        fire_request.extensions_mut().insert(AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: "system".to_string(),
            agent_id: "test".to_string(),
        });

        let fire_res = service.fire_agent(fire_request).await.unwrap().into_inner();
        assert_eq!(fire_res.agents.len(), 0);
    }

    #[tokio::test]
    async fn test_agent_get_dashboard_snapshot() {
        let service = setup_test_agent_manager_service().await;

        let req = EmptyRequest {};
        let mut request = Request::new(req);
        let mut metadata = tonic::metadata::MetadataMap::new();
        metadata.insert("x-spiffe-id", "spiffe://onehumancorp.io/org/system/agent/test".parse().unwrap());
        *request.metadata_mut() = metadata;
        request.extensions_mut().insert(AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: "system".to_string(),
            agent_id: "test".to_string(),
        });

        let res = service.get_dashboard_snapshot(request).await.unwrap().into_inner();
        assert!(res.costs.is_some());
    }

    #[tokio::test]
    async fn test_agent_create_restore_snapshot() {
        let service = setup_test_agent_manager_service().await;

        let req = CreateSnapshotRequest {
            label: "Test Snapshot".to_string(),
        };
        let mut request = Request::new(req);
        let mut metadata = tonic::metadata::MetadataMap::new();
        metadata.insert("x-spiffe-id", "spiffe://onehumancorp.io/org/system/agent/test".parse().unwrap());
        *request.metadata_mut() = metadata;
        request.extensions_mut().insert(AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: "system".to_string(),
            agent_id: "test".to_string(),
        });

        let res = service.create_snapshot(request).await.unwrap().into_inner();
        assert_eq!(res.label, "Test Snapshot");
        assert_eq!(res.agent_count, 0);

        let restore_req = RestoreSnapshotRequest {
            snapshot_id: res.id,
        };
        let mut restore_request = Request::new(restore_req);
        let mut metadata2 = tonic::metadata::MetadataMap::new();
        metadata2.insert("x-spiffe-id", "spiffe://onehumancorp.io/org/system/agent/test".parse().unwrap());
        *restore_request.metadata_mut() = metadata2;
        restore_request.extensions_mut().insert(AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: "system".to_string(),
            agent_id: "test".to_string(),
        });

        let restore_res = service.restore_snapshot(restore_request).await.unwrap().into_inner();
        assert!(restore_res.costs.is_some());
    }
}


#[cfg(test)]
mod benchmark_tests {
    use super::*;
    use std::sync::Arc;

    async fn setup_test_service() -> MyAgentManagerService {
        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(crate::hub::Hub::new(tx, crate::db::get_pool().clone()));

        hub.register_agent(::server_ohc::orchestration::Agent {
            id: "agent_1".to_string(),
            name: "Test Agent".to_string(),
            role: "assistant".to_string(),
            organization_id: "test_org".to_string(),
            status: "IDLE".to_string(),
            provider_type: "builtin".to_string(),
        });

        hub.open_meeting("meeting_1".to_string(), vec!["agent_1".to_string()], "Test Agenda".to_string());

        MyAgentManagerService::new(hub)
    }

    #[tokio::test]
    async fn test_get_snapshot_latency_benchmark() {
        let service = setup_test_service().await;

        let start = std::time::Instant::now();
        let _snapshot = service.get_snapshot("test_org", false).await.unwrap();
        let elapsed = start.elapsed();

        tracing::info!("AgentManagerService::get_snapshot benchmark completed in {} ms", elapsed.as_millis());
        assert!(elapsed.as_millis() < 500, "AgentManagerService::get_snapshot took too long: {} ms", elapsed.as_millis());
    }
}
