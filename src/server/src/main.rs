use tonic::{transport::Server, Request, Response, Status};
use tokio_stream::Stream;
use tokio_stream::StreamExt;
use chrono::Utc;
use std::pin::Pin;

mod db;
mod auth;
mod hub;
mod minimax;
mod billing;
mod ultraplan;
mod autodream;
mod tasks;
mod settings;
mod scheduler;
mod msgbus;
mod pipeline;
mod oidc;
mod sip;
mod orchestrator;
mod spawner;
mod queue;
mod agents;
mod domain;
pub mod pricing;
pub mod analytics;
pub mod services {
    pub mod wizard;
    pub mod billing {
        pub mod auditor;
    }
    pub mod growth;
    pub mod onboarding;
    pub mod sync;
}

use tokio::sync::mpsc;
use std::sync::OnceLock;

static TELEMETRY_CHAN: OnceLock<mpsc::Sender<Box<dyn FnOnce() + Send>>> = OnceLock::new();

fn get_telemetry_chan() -> &'static mpsc::Sender<Box<dyn FnOnce() + Send>> {
    TELEMETRY_CHAN.get_or_init(|| {
        let (tx, rx) = mpsc::channel::<Box<dyn FnOnce() + Send>>(10000);
        let rx = std::sync::Arc::new(tokio::sync::Mutex::new(rx));
        
        for _ in 0..16 {
            let rx = rx.clone();
            tokio::spawn(async move {
                loop {
                    let job = {
                        let mut rx = rx.lock().await;
                        rx.recv().await
                    };
                    
                    if let Some(job) = job {
                        job();
                    } else {
                        break;
                    }
                }
            });
        }
        tx
    })
}

pub fn record_telemetry<F>(f: F)
where
    F: FnOnce() + Send + 'static,
{
    let tx = get_telemetry_chan();
    let _ = tx.try_send(Box::new(f));
}

fn spiffe_interceptor(req: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
    if let Some(spiffe_id) = req.metadata().get("x-spiffe-id") {
        if let Ok(spiffe_id_str) = spiffe_id.to_str() {
             match crate::auth::parse_spiffe_id(spiffe_id_str) {
                 Ok((org_id, agent_id)) => {
                     println!("Authenticated SPIFFE ID: org={}, agent={}", org_id, agent_id);
                 }
                 Err(e) => return Err(tonic::Status::permission_denied(e)),
             }
        } else {
             return Err(tonic::Status::invalid_argument("invalid x-spiffe-id header"));
        }
    }
    Ok(req)
}

pub mod ohc {
    pub mod orchestration {
        tonic::include_proto!("ohc.orchestration");
    }
    pub mod agent {
        pub mod service {
            tonic::include_proto!("ohc.agent.service");
        }
    }
    pub mod organization {
        tonic::include_proto!("ohc.organization");
    }
    pub mod common {
        tonic::include_proto!("ohc.common");
    }
}

use ohc::orchestration::hub_service_server::{HubService, HubServiceServer};
use ohc::orchestration::*;

use std::sync::Arc;
use hub::Hub;

pub struct MyHubService {
    hub: Arc<Hub>,
}

impl MyHubService {
    pub fn new(hub: Arc<Hub>) -> Self {
        MyHubService { hub }
    }
}

#[tonic::async_trait]
impl HubService for MyHubService {
    async fn register_agent(
        &self,
        request: Request<RegisterAgentRequest>,
    ) -> Result<Response<RegisterAgentResponse>, Status> {
        let req = request.into_inner();
        if let Some(agent) = req.agent {
            self.hub.register_agent(agent);
            Ok(Response::new(RegisterAgentResponse { success: true }))
        } else {
            Err(Status::invalid_argument("agent is required"))
        }
    }

    async fn handle_config_wizard(
        &self,
        request: tonic::Request<crate::ohc::orchestration::AgentConfig>,
    ) -> Result<tonic::Response<crate::ohc::orchestration::WizardResponse>, tonic::Status> {
        let req = request.into_inner();
        let resp = services::wizard::handle_config_wizard(req);
        Ok(tonic::Response::new(resp))
    }

    async fn handle_prompt_tuning(
        &self,
        request: tonic::Request<crate::ohc::orchestration::PromptTuningConfig>,
    ) -> Result<tonic::Response<crate::ohc::orchestration::WizardResponse>, tonic::Status> {
        let req = request.into_inner();
        let resp = services::wizard::handle_prompt_tuning(req);
        Ok(tonic::Response::new(resp))
    }

    async fn open_meeting(
        &self,
        request: Request<OpenMeetingRequest>,
    ) -> Result<Response<MeetingRoom>, Status> {
        let req = request.into_inner();
        let meeting = self.hub.open_meeting(req.meeting_id, req.participants, req.agenda);
        Ok(Response::new(meeting))
    }

    async fn publish(
        &self,
        request: Request<PublishMessageRequest>,
    ) -> Result<Response<PublishMessageResponse>, Status> {
        let req = request.into_inner();
        if let Some(msg) = req.message {
            match self.hub.clone().publish(msg) {
                Ok(_) => Ok(Response::new(PublishMessageResponse { success: true })),
                Err(e) => Err(Status::internal(e)),
            }
        } else {
            Err(Status::invalid_argument("message is required"))
        }
    }

    async fn delegate_task(
        &self,
        request: Request<DelegateTaskRequest>,
    ) -> Result<Response<DelegateTaskResponse>, Status> {
        let req = request.into_inner();
        if let Some(task) = req.task {
            match self.hub.clone().delegate_task(req.from_agent_id, req.to_agent_id, task) {
                Ok(_) => Ok(Response::new(DelegateTaskResponse { success: true })),
                Err(e) => Err(Status::internal(e)),
            }
        } else {
            Err(Status::invalid_argument("task is required"))
        }
    }

    async fn verify_environment(
        &self,
        request: tonic::Request<VerifyEnvironmentRequest>,
    ) -> Result<tonic::Response<VerifyEnvironmentResponse>, tonic::Status> {
        let req = request.into_inner();
        let env_vars = req.env_vars;
        
        match services::onboarding::env_verifier::verify_environment(&env_vars) {
            Ok(config) => {
                Ok(tonic::Response::new(VerifyEnvironmentResponse {
                    status: "success".to_string(),
                    config: Some(EnvConfig {
                        mode: config.mode,
                        multi_tenant: config.multi_tenant,
                        headless: config.headless,
                        telemetry_enabled: config.telemetry_enabled,
                        api_endpoint: config.api_endpoint,
                        database_url: config.database_url,
                    }),
                    error: String::new(),
                }))
            }
            Err(e) => {
                Ok(tonic::Response::new(VerifyEnvironmentResponse {
                    status: "error".to_string(),
                    config: None,
                    error: e,
                }))
            }
        }
    }

    async fn generate_config(
        &self,
        request: tonic::Request<GenerateConfigRequest>,
    ) -> Result<tonic::Response<GenerateConfigResponse>, tonic::Status> {
        let req = request.into_inner();
        let mode = req.mode;

        let mut config = std::collections::HashMap::new();
        if mode == "cloud" {
            config.insert("swarm_size".to_string(), "large".to_string());
            config.insert("database".to_string(), "postgresql".to_string());
            config.insert("cache".to_string(), "redis".to_string());
        } else if mode == "standalone" {
            config.insert("swarm_size".to_string(), "small".to_string());
            config.insert("database".to_string(), "sqlite".to_string());
            config.insert("cache".to_string(), "memory".to_string());
        } else if mode == "thin_client" {
            config.insert("swarm_size".to_string(), "none".to_string());
            config.insert("database".to_string(), "remote".to_string());
            config.insert("cache".to_string(), "none".to_string());
        } else {
            return Ok(tonic::Response::new(GenerateConfigResponse {
                status: "error".to_string(),
                config: std::collections::HashMap::new(),
            }));
        }

        Ok(tonic::Response::new(GenerateConfigResponse {
            status: "success".to_string(),
            config,
        }))
    }

    async fn save_wizard_state(
        &self,
        request: tonic::Request<SaveWizardStateRequest>,
    ) -> Result<tonic::Response<SaveWizardStateResponse>, tonic::Status> {
        let req = request.into_inner();
        let state = req.state;

        let mut wizard_state = self.hub.wizard_state.write().map_err(|e| tonic::Status::internal(e.to_string()))?;
        
        for (k, v) in state {
            wizard_state.insert(k, serde_json::Value::String(v));
        }

        Ok(tonic::Response::new(SaveWizardStateResponse {
            status: "saved".to_string(),
        }))
    }

    async fn get_wizard_state(
        &self,
        _request: tonic::Request<GetWizardStateRequest>,
    ) -> Result<tonic::Response<GetWizardStateResponse>, tonic::Status> {
        let wizard_state = self.hub.wizard_state.read().map_err(|e| tonic::Status::internal(e.to_string()))?;
        
        let mut state = std::collections::HashMap::new();
        for (k, v) in wizard_state.iter() {
            if let Some(s) = v.as_str() {
                state.insert(k.clone(), s.to_string());
            } else {
                state.insert(k.clone(), v.to_string());
            }
        }

        Ok(tonic::Response::new(GetWizardStateResponse {
            state,
        }))
    }

    async fn reset_wizard_state(
        &self,
        _request: tonic::Request<ResetWizardStateRequest>,
    ) -> Result<tonic::Response<ResetWizardStateResponse>, tonic::Status> {
        let mut wizard_state = self.hub.wizard_state.write().map_err(|e| tonic::Status::internal(e.to_string()))?;
        wizard_state.clear();

        Ok(tonic::Response::new(ResetWizardStateResponse {
            status: "reset".to_string(),
        }))
    }

    async fn provision(
        &self,
        request: tonic::Request<ProvisionRequest>,
    ) -> Result<tonic::Response<ProvisionResponse>, tonic::Status> {
        let _req = request.into_inner();
        
        Ok(tonic::Response::new(ProvisionResponse {
            status: "provisioned".to_string(),
            message: "State persisted successfully".to_string(),
        }))
    }

    async fn audit_setup(
        &self,
        request: tonic::Request<AuditSetupRequest>,
    ) -> Result<tonic::Response<AuditSetupResponse>, tonic::Status> {
        let req = request.into_inner();
        let env = req.env;

        match services::onboarding::env_verifier::verify_environment(&env) {
            Ok(config) => {
                Ok(tonic::Response::new(AuditSetupResponse {
                    status: "success".to_string(),
                    config: Some(EnvConfig {
                        mode: config.mode,
                        multi_tenant: config.multi_tenant,
                        headless: config.headless,
                        telemetry_enabled: config.telemetry_enabled,
                        api_endpoint: config.api_endpoint,
                        database_url: config.database_url,
                    }),
                    error: String::new(),
                }))
            }
            Err(e) => {
                Ok(tonic::Response::new(AuditSetupResponse {
                    status: "error".to_string(),
                    config: None,
                    error: e,
                }))
            }
        }
    }

    async fn diagnostics(
        &self,
        _request: tonic::Request<DiagnosticsRequest>,
    ) -> Result<tonic::Response<DiagnosticsResponse>, tonic::Status> {
        let env_vars = std::env::vars().collect::<std::collections::HashMap<String, String>>();
        
        let config_res = services::onboarding::env_verifier::verify_environment(&env_vars);
        
        let wizard_state = self.hub.wizard_state.read().map_err(|e| tonic::Status::internal(e.to_string()))?;
        
        let mut state = std::collections::HashMap::new();
        for (k, v) in wizard_state.iter() {
            if let Some(s) = v.as_str() {
                state.insert(k.clone(), s.to_string());
            } else {
                state.insert(k.clone(), v.to_string());
            }
        }

        match config_res {
            Ok(config) => {
                Ok(tonic::Response::new(DiagnosticsResponse {
                    status: "success".to_string(),
                    config: Some(EnvConfig {
                        mode: config.mode,
                        multi_tenant: config.multi_tenant,
                        headless: config.headless,
                        telemetry_enabled: config.telemetry_enabled,
                        api_endpoint: config.api_endpoint,
                        database_url: config.database_url,
                    }),
                    wizard_state: state,
                    error: String::new(),
                }))
            }
            Err(e) => {
                Ok(tonic::Response::new(DiagnosticsResponse {
                    status: "error".to_string(),
                    config: None,
                    wizard_state: state,
                    error: e,
                }))
            }
        }
    }

    async fn get_wizard_profile(
        &self,
        request: tonic::Request<GetWizardProfileRequest>,
    ) -> Result<tonic::Response<GetWizardProfileResponse>, tonic::Status> {
        let req = request.into_inner();
        let mode = req.mode;

        let profile = if mode == "cloud" {
            Some(EnvConfig {
                mode: "cloud".to_string(),
                multi_tenant: true,
                headless: false,
                telemetry_enabled: true,
                api_endpoint: String::new(),
                database_url: "postgresql://user:pass@localhost:5432/ohc".to_string(),
            })
        } else if mode == "standalone" {
            Some(EnvConfig {
                mode: "standalone".to_string(),
                multi_tenant: false,
                headless: false,
                telemetry_enabled: false,
                api_endpoint: String::new(),
                database_url: "sqlite://local.db".to_string(),
            })
        } else {
            return Ok(tonic::Response::new(GetWizardProfileResponse {
                status: "error".to_string(),
                profile: None,
                error: "Invalid mode requested".to_string(),
            }));
        };

        Ok(tonic::Response::new(GetWizardProfileResponse {
            status: "success".to_string(),
            profile,
            error: String::new(),
        }))
    }

    async fn create_task(
        &self,
        request: Request<CreateTaskRequest>,
    ) -> Result<Response<SharedTask>, Status> {
        let req = request.into_inner();
        let task = self.hub.task_manager().create_task(
            "default_org".to_string(),
            req.mission_id,
            req.title,
            req.description,
            req.priority,
        ).map_err(|e| Status::internal(e))?;
        
        Ok(Response::new(SharedTask {
            id: task.id,
            organization_id: task.organization_id,
            parent_plan_id: task.parent_plan_id,
            dependencies: task.dependencies,
            title: task.title,
            description: task.description.unwrap_or_default(),
            status: task.status,
            assigned_agent_id: task.assigned_agent_id.unwrap_or_default(),
            priority: task.priority,
            payload: task.payload,
            locked_until_unix: task.locked_until.map(|t| t.timestamp()).unwrap_or(0),
            created_at_unix: task.created_at.timestamp(),
            updated_at_unix: task.updated_at.timestamp(),
            action_risk: task.action_risk.unwrap_or_default(),
            approval_status: task.approval_status.unwrap_or_default(),
            proposed_content: task.proposed_content.unwrap_or_default(),
        }))
    }

    type PollTasksStream = Pin<Box<dyn Stream<Item = Result<SharedTask, Status>> + Send>>;
    
    async fn poll_tasks(
        &self,
        request: Request<PollTasksRequest>,
    ) -> Result<Response<Self::PollTasksStream>, Status> {
        let req = request.into_inner();
        let tasks = self.hub.task_manager().poll_tasks(&req.agent_id, req.limit as usize);
        
        let mapped_tasks: Vec<Result<SharedTask, Status>> = tasks.into_iter().map(|task| {
            Ok(SharedTask {
                id: task.id,
                organization_id: task.organization_id,
                parent_plan_id: task.parent_plan_id,
                dependencies: task.dependencies,
                title: task.title,
                description: task.description.unwrap_or_default(),
                status: task.status,
                assigned_agent_id: task.assigned_agent_id.unwrap_or_default(),
                priority: task.priority,
                payload: task.payload,
                locked_until_unix: task.locked_until.map(|t| t.timestamp()).unwrap_or(0),
                created_at_unix: task.created_at.timestamp(),
                updated_at_unix: task.updated_at.timestamp(),
                action_risk: task.action_risk.unwrap_or_default(),
                approval_status: task.approval_status.unwrap_or_default(),
                proposed_content: task.proposed_content.unwrap_or_default(),
            })
        }).collect();
        
        let stream = tokio_stream::iter(mapped_tasks);
        Ok(Response::new(Box::pin(stream) as Self::PollTasksStream))
    }

    async fn update_task_status(
        &self,
        request: Request<UpdateTaskStatusRequest>,
    ) -> Result<Response<UpdateTaskStatusResponse>, Status> {
        let req = request.into_inner();
        
        match req.status.as_str() {
            "REVIEW" => {
                self.hub.task_manager().review_task(&req.task_id, &req.agent_id)
                    .map_err(|e| Status::internal(e))?;
            }
            "COMPLETED" => {
                self.hub.task_manager().complete_task(&req.task_id, &req.agent_id, req.result)
                    .map_err(|e| Status::internal(e))?;
            }
            _ => {
                self.hub.task_manager().update_task_status(&req.task_id, req.status)
                    .map_err(|e| Status::internal(e))?;
            }
        }
        
        Ok(Response::new(UpdateTaskStatusResponse { success: true }))
    }

    async fn decompose_task(
        &self,
        request: Request<DecomposeTaskRequest>,
    ) -> Result<Response<DecomposeTaskResponse>, Status> {
        let req = request.into_inner();
        
        for st in req.sub_tasks {
            let mut filtered_deps = Vec::new();
            for dep in st.dependencies {
                if dep != req.task_id {
                    filtered_deps.push(dep);
                }
            }
            
            self.hub.task_manager().create_task_with_plan(
                req.organization_id.clone(),
                String::new(),
                req.task_id.clone(),
                filtered_deps,
                st.title,
                st.description,
                st.priority,
            ).map_err(|e| Status::internal(e))?;
        }
        
        Ok(Response::new(DecomposeTaskResponse { success: true }))
    }

    type StreamMessagesStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;

    async fn stream_messages(
        &self,
        request: Request<StreamMessagesRequest>,
    ) -> Result<Response<Self::StreamMessagesStream>, Status> {
        let req = request.into_inner();
        let agent_id = req.agent_id.clone();
        
        let rx = self.hub.subscribe(agent_id.clone());
        let drained = self.hub.get_inbox(&agent_id);
        
        let drained_stream = tokio_stream::iter(drained.into_iter().map(Ok));
        
        let rx_stream = tokio_stream::wrappers::BroadcastStream::new(rx)
            .map(|res| match res {
                Ok(msg) => Ok(msg),
                Err(e) => Err(Status::internal(e.to_string())),
            });
            
        let full_stream = drained_stream.chain(rx_stream);
        
        Ok(Response::new(Box::pin(full_stream) as Self::StreamMessagesStream))
    }

    async fn reason(
        &self,
        request: Request<ReasonRequest>,
    ) -> Result<Response<ReasonResponse>, Status> {
        let req = request.into_inner();
        let api_key = self.hub.minimax_api_key().to_string();
        if api_key.is_empty() {
            return Err(Status::failed_precondition("Minimax API key is not configured"));
        }
        
        let client = minimax::MinimaxClient::new(api_key);
        match client.reason(&req.prompt).await {
            Ok(content) => Ok(Response::new(ReasonResponse { content })),
            Err(e) => Err(Status::internal(e)),
        }
    }

    async fn delegate_sub_task(
        &self,
        request: Request<SubTask>,
    ) -> Result<Response<DelegateTaskResponse>, Status> {
        let req = request.into_inner();
        
        if req.task_id.is_empty() || req.target_role.is_empty() {
            return Err(Status::invalid_argument("task_id and target_role are required"));
        }
        
        // Quota Enforcement
        if self.hub.get_agents_count() >= 10 {
            return Err(Status::resource_exhausted("VRAM quota limit exceeded, cannot spawn sub-agent"));
        }
        
        let now_nano = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let sub_agent_id = format!("sub-agent-{}-{}", req.target_role, now_nano);
        
        let sub_agent = Agent {
            id: sub_agent_id.clone(),
            name: format!("Specialized {} Agent", req.target_role),
            role: req.target_role.clone(),
            organization_id: "dynamic-delegation".to_string(),
            status: "IDLE".to_string(),
            provider_type: "builtin".to_string(),
        };
        
        self.hub.register_agent(sub_agent);
        
        // Prompt injection checks
        if req.instruction.contains("SYSTEM:") || req.instruction.contains("\n\n") {
            return Err(Status::invalid_argument("instruction contains forbidden prompt injection sequences"));
        }
        if req.parent_thread_id.contains("SYSTEM:") || req.parent_thread_id.contains("\n\n") {
            return Err(Status::invalid_argument("parent_thread_id contains forbidden prompt injection sequences"));
        }
        
        let msg_id = format!("msg-{}-{}", req.task_id, now_nano);
        let msg = Message {
            id: msg_id,
            from_agent: req.from_agent_id,
            to_agent: sub_agent_id,
            r#type: "TaskDelegation".to_string(),
            content: format!("Execute Task: {}\nContext: {}", req.instruction, req.parent_thread_id),
            occurred_at_unix: Utc::now().timestamp(),
            meeting_id: String::new(),
        };
        
        match self.hub.clone().publish(msg) {
            Ok(_) => Ok(Response::new(DelegateTaskResponse { success: true })),
            Err(e) => Err(Status::internal(e)),
        }
    }

    async fn advertise_capabilities(
        &self,
        request: Request<AgentCapabilities>,
    ) -> Result<Response<PublishMessageResponse>, Status> {
        let req = request.into_inner();
        if req.agent_id.is_empty() {
            return Err(Status::invalid_argument("agent_id is required"));
        }
        
        match self.hub.advertise_capabilities(req) {
            Ok(_) => Ok(Response::new(PublishMessageResponse { success: true })),
            Err(e) => Err(Status::internal(e)),
        }
    }

    type DiscoverAgentsStream = Pin<Box<dyn Stream<Item = Result<AgentCapabilities, Status>> + Send>>;

    async fn discover_agents(
        &self,
        _request: Request<Query>,
    ) -> Result<Response<Self::DiscoverAgentsStream>, Status> {
        let rx = self.hub.subscribe_capabilities();
        
        let rx_stream = tokio_stream::wrappers::BroadcastStream::new(rx)
            .map(|res| match res {
                Ok(caps) => Ok(caps),
                Err(e) => Err(Status::internal(e.to_string())),
            });
            
        Ok(Response::new(Box::pin(rx_stream) as Self::DiscoverAgentsStream))
    }

    type StreamMeshEventsStream = Pin<Box<dyn Stream<Item = Result<MeshEvent, Status>> + Send>>;

    async fn stream_mesh_events(
        &self,
        request: Request<EventStreamRequest>,
    ) -> Result<Response<Self::StreamMeshEventsStream>, Status> {
        let req = request.into_inner();
        if req.topic.is_empty() {
            return Err(Status::invalid_argument("topic is required"));
        }
        
        let rx = self.hub.subscribe_mesh_events(req.topic);
        
        let rx_stream = tokio_stream::wrappers::BroadcastStream::new(rx)
            .map(|res| match res {
                Ok(event) => Ok(event),
                Err(e) => Err(Status::internal(e.to_string())),
            });
            
        Ok(Response::new(Box::pin(rx_stream) as Self::StreamMeshEventsStream))
    }

    async fn publish_teammate_mesh_event(
        &self,
        request: Request<PublishTeammateMeshEventRequest>,
    ) -> Result<Response<PublishMessageResponse>, Status> {
        let req = request.into_inner();
        if req.channel.is_empty() {
            return Err(Status::invalid_argument("channel is required"));
        }
        if let Some(event) = req.event {
            match self.hub.publish_teammate_event(req.channel, event) {
                Ok(_) => Ok(Response::new(PublishMessageResponse { success: true })),
                Err(e) => Err(Status::internal(e)),
            }
        } else {
            Err(Status::invalid_argument("event is required"))
        }
    }

    type StreamTeammateMeshStream = Pin<Box<dyn Stream<Item = Result<TeammateMeshEvent, Status>> + Send>>;

    async fn stream_teammate_mesh(
        &self,
        request: Request<EventStreamRequest>,
    ) -> Result<Response<Self::StreamTeammateMeshStream>, Status> {
        let req = request.into_inner();
        if req.topic.is_empty() {
            return Err(Status::invalid_argument("topic is required"));
        }
        
        let rx = self.hub.subscribe_teammate_mesh(req.topic);
        
        let rx_stream = tokio_stream::wrappers::BroadcastStream::new(rx)
            .map(|res| match res {
                Ok(event) => Ok(event),
                Err(e) => Err(Status::internal(e.to_string())),
            });
            
        Ok(Response::new(Box::pin(rx_stream) as Self::StreamTeammateMeshStream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database
    let db = Arc::new(db::DB::new().await?);
    db.run_migrations().await?;

    let addr = "[::1]:50051".parse()?;
    let (event_tx, mut event_rx) = tokio::sync::mpsc::channel(100);
    let hub = Arc::new(Hub::new(event_tx, db.pool.clone()));
    
    // Start event log worker
    let hub_clone = hub.clone();
    tokio::spawn(async move {
        while let Some(raw_event) = event_rx.recv().await {
            let event = hub_clone.sanitize_hub_event(raw_event);
            hub_clone.append_recent_event(event);
        }
    });

    let hub_service = MyHubService::new(hub.clone());
    let store = std::sync::Arc::new(auth::Store::new());
    
    // Start AutoDream worker
    let autodream_worker = autodream::AutoDreamWorker::new(db.clone());
    autodream_worker.start();

    // Start Scheduler Background Task
    let hub_for_sched = hub.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            let due = hub_for_sched.scheduler().poll_due();
            for task in due {
                println!("executing scheduled task: {} ({})", task.name, task.id);
                
                // Mark as running
                if let Err(e) = hub_for_sched.scheduler().mark_running(&task.organization_id, &task.id) {
                    println!("failed to mark task as running: {}", e);
                    continue;
                }
                
                // Simulate task execution by publishing a message
                let msg = Message {
                    id: format!("{}-{}", task.id, Utc::now().timestamp()),
                    from_agent: "system-scheduler".to_string(),
                    to_agent: task.agent_id.clone(),
                    r#type: "task".to_string(),
                    content: format!("Scheduled Task triggered: {}. Payload: {}", task.name, task.payload),
                    occurred_at_unix: Utc::now().timestamp(),
                    meeting_id: String::new(),
                };
                
                match hub_for_sched.clone().publish(msg) {
                    Ok(_) => {
                        let _ = hub_for_sched.scheduler().mark_done(&task.organization_id, &task.id, true);
                    }
                    Err(e) => {
                        println!("failed to publish scheduled task message: {}", e);
                        let _ = hub_for_sched.scheduler().mark_done(&task.organization_id, &task.id, false);
                    }
                }
            }
        }
    });

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(HubServiceServer::with_interceptor(hub_service, spiffe_interceptor))
        .add_service(crate::ohc::orchestration::auth_service_server::AuthServiceServer::new(store))
        .serve(addr)
        .await?;

    Ok(())
}
