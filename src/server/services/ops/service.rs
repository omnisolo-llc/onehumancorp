use tonic::{Request, Response, Status};
use ::server_ohc::orchestration::*;
use ::server_ohc::orchestration::ops_service_server::OpsService;
use std::sync::{Arc, RwLock};
use chrono::Utc;
use crate::hub::Hub;
use tokio_stream::Stream;
use std::pin::Pin;
use tokio_stream::StreamExt;

pub struct MyOpsService {
    hub: Arc<Hub>,
    incidents: RwLock<Vec<Incident>>,
    compute_profiles: RwLock<Vec<ComputeProfile>>,
    budget_alerts: RwLock<Vec<BudgetAlert>>,
    pipelines: RwLock<Vec<Pipeline>>,
}

impl MyOpsService {
    pub fn new(hub: Arc<Hub>) -> Self {
        MyOpsService {
            hub,
            incidents: RwLock::new(Vec::new()),
            compute_profiles: RwLock::new(Vec::new()),
            budget_alerts: RwLock::new(Vec::new()),
            pipelines: RwLock::new(Vec::new()),
        }
    }
}

#[tonic::async_trait]
impl OpsService for MyOpsService {
    async fn get_incidents(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<IncidentsResponse>, Status> {
        let incidents = self.incidents.read().unwrap();
        Ok(Response::new(IncidentsResponse {
            incidents: incidents.clone(),
        }))
    }

    async fn create_incident(
        &self,
        request: Request<CreateIncidentRequest>,
    ) -> Result<Response<Incident>, Status> {
        let req = request.into_inner();
        if req.severity.is_empty() || req.summary.is_empty() {
            return Err(Status::invalid_argument("severity and summary are required"));
        }
        
        let now = Utc::now();
        let incident = Incident {
            id: format!("inc-{}", now.timestamp()),
            severity: req.severity,
            summary: req.summary,
            rca: req.rca,
            status: "INVESTIGATING".to_string(),
            created_at_unix: now.timestamp(),
            updated_at_unix: now.timestamp(),
            resolution_plan_id: String::new(),
        };
        
        let mut incidents = self.incidents.write().unwrap();
        incidents.push(incident.clone());
        
        Ok(Response::new(incident))
    }

    async fn update_incident_status(
        &self,
        request: Request<IncidentStatusRequest>,
    ) -> Result<Response<Incident>, Status> {
        let req = request.into_inner();
        if req.incident_id.is_empty() || req.status.is_empty() {
            return Err(Status::invalid_argument("incidentId and status are required"));
        }
        
        let mut incidents = self.incidents.write().unwrap();
        let mut found = false;
        let mut updated = None;
        
        for inc in incidents.iter_mut() {
            if inc.id == req.incident_id {
                inc.status = req.status.clone();
                inc.updated_at_unix = Utc::now().timestamp();
                if !req.resolution_plan_id.is_empty() {
                    inc.resolution_plan_id = req.resolution_plan_id.clone();
                }
                if !req.rca.is_empty() {
                    inc.rca = req.rca.clone();
                }
                updated = Some(inc.clone());
                found = true;
                break;
            }
        }
        
        if !found {
            return Err(Status::not_found("incident not found"));
        }
        
        Ok(Response::new(updated.unwrap()))
    }

    async fn get_compute_profiles(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<ComputeProfilesResponse>, Status> {
        let profiles = self.compute_profiles.read().unwrap();
        Ok(Response::new(ComputeProfilesResponse {
            profiles: profiles.clone(),
        }))
    }

    async fn create_compute_profile(
        &self,
        request: Request<CreateComputeProfileRequest>,
    ) -> Result<Response<ComputeProfile>, Status> {
        let req = request.into_inner();
        if req.role_id.is_empty() {
            return Err(Status::invalid_argument("roleId is required"));
        }
        
        let profile = ComputeProfile {
            role_id: req.role_id,
            min_vram_gb: req.min_vram_gb,
            preferred_gpu_type: req.preferred_gpu_type,
            scheduling_priority: req.scheduling_priority,
            created_at_unix: Utc::now().timestamp(),
        };
        
        let mut profiles = self.compute_profiles.write().unwrap();
        profiles.push(profile.clone());
        
        Ok(Response::new(profile))
    }

    async fn get_cluster_status(
        &self,
        request: Request<GetClusterStatusRequest>,
    ) -> Result<Response<ClusterStatus>, Status> {
        let req = request.into_inner();
        if req.region.is_empty() {
            return Err(Status::invalid_argument("region is required"));
        }
        
        Ok(Response::new(ClusterStatus {
            region: req.region,
            status: "healthy".to_string(),
            latency_ms: 3,
            available_nodes: 5,
            checked_at_unix: Utc::now().timestamp(),
        }))
    }

    async fn get_budget_alerts(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<BudgetAlertsResponse>, Status> {
        let alerts = self.budget_alerts.read().unwrap();
        Ok(Response::new(BudgetAlertsResponse {
            alerts: alerts.clone(),
        }))
    }

    async fn create_budget_alert(
        &self,
        request: Request<CreateBudgetAlertRequest>,
    ) -> Result<Response<BudgetAlert>, Status> {
        let req = request.into_inner();
        if req.threshold_usd <= 0.0 {
            return Err(Status::invalid_argument("thresholdUsd must be greater than zero"));
        }
        
        let alert = BudgetAlert {
            id: format!("alert-{}", Utc::now().timestamp()),
            organization_id: req.organization_id,
            threshold_usd: req.threshold_usd,
            notify_at_pct: req.notify_at_pct,
            predictive: req.predictive,
            forecast_hours: req.forecast_hours,
            triggered: false,
            created_at_unix: Utc::now().timestamp(),
        };
        
        let mut alerts = self.budget_alerts.write().unwrap();
        alerts.push(alert.clone());
        
        Ok(Response::new(alert))
    }

    async fn get_pipelines(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<PipelinesResponse>, Status> {
        let pipelines = self.pipelines.read().unwrap();
        Ok(Response::new(PipelinesResponse {
            pipelines: pipelines.clone(),
        }))
    }

    async fn create_pipeline(
        &self,
        request: Request<CreatePipelineRequest>,
    ) -> Result<Response<Pipeline>, Status> {
        let req = request.into_inner();
        if req.name.is_empty() {
            return Err(Status::invalid_argument("name is required"));
        }
        
        let now = Utc::now();
        let pipeline = Pipeline {
            id: format!("pipeline-{}", now.timestamp()),
            name: req.name,
            status: "PENDING".to_string(),
            branch: req.branch,
            initiated_by: req.initiated_by,
            staging_url: String::new(),
            created_at_unix: now.timestamp(),
            updated_at_unix: now.timestamp(),
        };
        
        let mut pipelines = self.pipelines.write().unwrap();
        pipelines.push(pipeline.clone());
        
        Ok(Response::new(pipeline))
    }

    async fn promote_pipeline(
        &self,
        request: Request<PipelinePromoteRequest>,
    ) -> Result<Response<Pipeline>, Status> {
        let req = request.into_inner();
        let mut pipelines = self.pipelines.write().unwrap();
        let mut found = false;
        let mut updated = None;
        
        for p in pipelines.iter_mut() {
            if p.id == req.pipeline_id {
                if p.status != "STAGING" {
                    return Err(Status::failed_precondition("pipeline must be in STAGING status to promote"));
                }
                p.status = "PROMOTED".to_string();
                p.updated_at_unix = Utc::now().timestamp();
                updated = Some(p.clone());
                found = true;
                break;
            }
        }
        
        if !found {
            return Err(Status::not_found("pipeline not found"));
        }
        
        Ok(Response::new(updated.unwrap()))
    }

    async fn update_pipeline_status(
        &self,
        request: Request<UpdatePipelineStatusRequest>,
    ) -> Result<Response<Pipeline>, Status> {
        let req = request.into_inner();
        let mut pipelines = self.pipelines.write().unwrap();
        let mut found = false;
        let mut updated = None;
        
        for p in pipelines.iter_mut() {
            if p.id == req.pipeline_id {
                p.status = req.status.clone();
                p.updated_at_unix = Utc::now().timestamp();
                if !req.staging_url.is_empty() {
                    p.staging_url = req.staging_url.clone();
                }
                updated = Some(p.clone());
                found = true;
                break;
            }
        }
        
        if !found {
            return Err(Status::not_found("pipeline not found"));
        }
        
        Ok(Response::new(updated.unwrap()))
    }

    async fn scale(
        &self,
        request: Request<ScaleRequest>,
    ) -> Result<Response<ScaleResponse>, Status> {
        let spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata()).map_err(|e| Status::unauthenticated(e))?;
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(&spiffe_id_str)?;
        let org_id = if tenant_id.is_empty() { "system".to_string() } else { tenant_id };


        let req = request.into_inner();
        if req.role.is_empty() {
            return Err(Status::invalid_argument("role is required"));
        }

        let agents = self.hub.get_agents().await;
        let mut current_count = 0;
        let mut idle_agent_ids = Vec::new();
        let mut active_agent_ids = Vec::new();

        for agent in agents.iter() {
            if agent.role == req.role {
                current_count += 1;
                if agent.status == "IDLE" {
                    idle_agent_ids.push(agent.id.clone());
                } else {
                    active_agent_ids.push(agent.id.clone());
                }
            }
        }

        let diff = req.count - current_count;

        if diff > 0 {
            for i in 0..diff {
                let id = format!("agent-{}-{}", Utc::now().timestamp(), i);
                let new_agent = Agent {
                    id: id.clone(),
                    name: req.role.clone(),
                    role: req.role.clone(),
                    organization_id: org_id.clone(),
                    status: "IDLE".to_string(),
                    provider_type: "mock".to_string(),
                };
                self.hub.register_agent(new_agent);
            }
        } else if diff < 0 {
            let to_remove = -diff;
            for i in 0..to_remove {
                if i < idle_agent_ids.len() as i32 {
                    self.hub.fire_agent(&idle_agent_ids[i as usize]);
                } else if (i as usize - idle_agent_ids.len()) < active_agent_ids.len() {
                    self.hub.fire_agent(&active_agent_ids[i as usize - idle_agent_ids.len()]);
                }
            }
        }

        Ok(Response::new(ScaleResponse {
            status: "success".to_string(),
            role: req.role,
            count: req.count,
        }))
    }

    type StreamScaleEventsStream = Pin<Box<dyn Stream<Item = Result<ScaleEvent, Status>> + Send + 'static>>;

    async fn stream_scale_events(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<Self::StreamScaleEventsStream>, Status> {
        let events = vec![
            ScaleEvent { event: "AI Workforce Manager: Reconciling Team Member resource.".to_string(), status: "INFO".to_string() },
            ScaleEvent { event: "AI Workforce Manager: Allocating compute profiles...".to_string(), status: "INFO".to_string() },
            ScaleEvent { event: "AI Workforce Manager: Provisioning SPIFFE identities...".to_string(), status: "INFO".to_string() },
            ScaleEvent { event: "AI Workforce Manager: Integrating with orchestration Hub...".to_string(), status: "INFO".to_string() },
            ScaleEvent { event: "AgentHired".to_string(), status: "Ready".to_string() },
        ];

        let stream = tokio_stream::iter(events).map(|e| Ok(e));
        Ok(Response::new(Box::pin(stream) as Self::StreamScaleEventsStream))
    }

    async fn prune_missions(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<PruneMissionsResponse>, Status> {
        let spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata()).map_err(|e| Status::unauthenticated(e))?;
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(&spiffe_id_str)?;
        let org_id = if tenant_id.is_empty() { "system".to_string() } else { tenant_id };


        let sip_db = crate::sip::SipDB::new(self.hub.pool.clone(), org_id);

        match sip_db.prune_stale_missions(chrono::Duration::days(7)).await {
            Ok(_) => Ok(Response::new(PruneMissionsResponse {
                status: "success".to_string(),
                message: "agent missions pruned".to_string(),
            })),
            Err(e) => Err(Status::internal(format!("failed to prune missions: {}", e))),
        }
    }
}
