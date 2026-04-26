use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::agent::{Agent, AgentEvent, AgentRunConfig};
use crate::auth::AuthMode;
use ohc_builtin_agent_llm::{
    anthropic::AnthropicClient, ollama::OllamaClient, openai::OpenAIClient, LlmClient,
};
use crate::memory::{inject_memories_into_prompt, MemoryEntry, MemoryStore};
use crate::proto::{
    agent_service_server::AgentService, EventType, PingRequest, PingResponse, RunTaskEvent,
    RunTaskRequest, SubAgentRequest, SubAgentResponse,
};
use ohc_builtin_agent_tools::{
    sendmessage::Mailbox, task::TaskStore, todowrite::TodoItem, SharedMailbox, SharedTaskStore,
    SharedTodos,
};
use chrono::Utc;
use crate::departments::{Department, get_department_config};
use std::str::FromStr;
use tokio::sync::RwLock;

pub const DEFAULT_ADDRESS: &str = "127.0.0.1:50051";
const AGENT_VERSION: &str = "1.0.0";

/// Top-level config for the agent service.
#[derive(Debug, Clone, Default)]
pub struct AgentConfig {
    pub llm_provider: String,
    pub model: String,
    pub llm_endpoint: String,
    pub system_prompt: String,
    pub max_tokens: i32,
    pub temperature: f32,
    pub max_iterations: i32,
    pub max_context_messages: i32,
}

/// Implements the AgentService gRPC service.
pub struct AgentServiceImpl {
    agent_id: String,
    cfg: AgentConfig,
    auth: AuthMode,
    memory: Arc<MemoryStore>,
    /// Optional LLM client override for testing.
    llm_override: Option<Arc<dyn LlmClient>>,
}

impl AgentServiceImpl {
    pub fn new(agent_id: impl Into<String>, cfg: AgentConfig, auth: AuthMode) -> Self {
        Self {
            agent_id: agent_id.into(),
            cfg,
            auth,
            memory: Arc::new(MemoryStore::default()),
            llm_override: None,
        }
    }

    /// Install a mock LLM for testing.
    pub fn with_llm_override(mut self, llm: Arc<dyn LlmClient>) -> Self {
        self.llm_override = Some(llm);
        self
    }

    fn check_auth<T>(&self, req: &Request<T>) -> Result<(), Status> {
        match &self.auth {
            AuthMode::Disabled => Ok(()),
            AuthMode::Token { token_hash } => {
                let meta = req.metadata();
                let auth_val = meta
                    .get("authorization")
                    .ok_or_else(|| Status::unauthenticated("missing authorization header"))?
                    .to_str()
                    .map_err(|_| Status::unauthenticated("invalid authorization header"))?;
                let tok = auth_val
                    .strip_prefix("Bearer ")
                    .ok_or_else(|| Status::unauthenticated("authorization must be Bearer token"))?;
                if !crate::auth::check_token(tok, token_hash) {
                    return Err(Status::unauthenticated("invalid token"));
                }
                Ok(())
            }
            AuthMode::Spiffe { .. } => {
                // SPIFFE/mTLS check would be done at the transport layer.
                // For simplicity we allow if TLS is used.
                Ok(())
            }
        }
    }

    fn resolve_llm(&self, req_provider: &str, req_model: &str, req_endpoint: &str) -> Arc<dyn LlmClient> {
        if let Some(llm) = &self.llm_override {
            return llm.clone();
        }

        let provider = if !req_provider.is_empty() {
            req_provider
        } else {
            &self.cfg.llm_provider
        };

        match provider {
            "anthropic" => {
                let key = std::env::var("ANTHROPIC_API_KEY").unwrap_or_default();
                Arc::new(AnthropicClient::new(key))
            }
            "openai" => {
                let key = std::env::var("OPENAI_API_KEY").unwrap_or_default();
                let _model_name = if req_model.is_empty() { &self.cfg.model } else { req_model };
                if !req_endpoint.is_empty() {
                    Arc::new(OpenAIClient::with_base_url(key, req_endpoint))
                } else if !self.cfg.llm_endpoint.is_empty() {
                    Arc::new(OpenAIClient::with_base_url(key, &self.cfg.llm_endpoint))
                } else {
                    Arc::new(OpenAIClient::new(key))
                }
            }
            "ollama" => {
                let endpoint = if !req_endpoint.is_empty() {
                    req_endpoint.to_string()
                } else if !self.cfg.llm_endpoint.is_empty() {
                    self.cfg.llm_endpoint.clone()
                } else {
                    String::new()
                };
                Arc::new(OllamaClient::new(endpoint))
            }
            _ => {
                // Auto-detect from env vars
                if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
                    if !key.is_empty() {
                        return Arc::new(AnthropicClient::new(key));
                    }
                }
                if let Ok(key) = std::env::var("OPENAI_API_KEY") {
                    if !key.is_empty() {
                        return Arc::new(OpenAIClient::new(key));
                    }
                }
                // Fallback: Ollama
                Arc::new(OllamaClient::new(
                    std::env::var("OHC_LOCAL_LLM_ENDPOINT").unwrap_or_default(),
                ))
            }
        }
    }

    fn build_run_config(&self, req: &RunTaskRequest, _department: &str) -> AgentRunConfig {
        let mut run_cfg = AgentRunConfig {
            model: self.cfg.model.clone(),
            system: self.cfg.system_prompt.clone(),
            max_tokens: self.cfg.max_tokens,
            temperature: self.cfg.temperature,
            max_iterations: self.cfg.max_iterations,
            max_task_tokens: 0,
        };

        if let Some(runtime_config) = &req.runtime_config {
            if !runtime_config.model.is_empty() {
                run_cfg.model = runtime_config.model.clone();
            }
            if !runtime_config.system_prompt.is_empty() {
                run_cfg.system = runtime_config.system_prompt.clone();
            }
            if runtime_config.max_tokens > 0 {
                run_cfg.max_tokens = runtime_config.max_tokens;
            }
            if runtime_config.temperature > 0.0 {
                run_cfg.temperature = runtime_config.temperature;
            }
            if runtime_config.max_iterations > 0 {
                run_cfg.max_iterations = runtime_config.max_iterations;
            }
        } else {
            if !req.model.is_empty() {
                run_cfg.model = req.model.clone();
            }
            if !req.system_prompt.is_empty() {
                run_cfg.system = req.system_prompt.clone();
            }
            if req.max_tokens > 0 {
                run_cfg.max_tokens = req.max_tokens;
            }
            if req.temperature > 0.0 {
                run_cfg.temperature = req.temperature;
            }
        }

        if !_department.is_empty() {
            if let Ok(dep) = Department::from_str(_department) {
                run_cfg.system = get_department_config(dep).system_prompt.to_string();
            }
        }

        run_cfg
    }
}

#[tonic::async_trait]
impl AgentService for AgentServiceImpl {
    async fn ping(&self, req: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        self.check_auth(&req)?;
        Ok(Response::new(PingResponse {
            agent_id: self.agent_id.clone(),
            version: AGENT_VERSION.to_string(),
        }))
    }

    type RunTaskStream = ReceiverStream<Result<RunTaskEvent, Status>>;

    async fn run_task(
        &self,
        req: Request<RunTaskRequest>,
    ) -> Result<Response<Self::RunTaskStream>, Status> {
        self.check_auth(&req)?;

        let task_req = req.into_inner();

        let mut req_provider = task_req.llm_provider.clone();
        let mut req_model = task_req.model.clone();
        let mut req_endpoint = task_req.llm_endpoint.clone();
        if let Some(runtime_config) = &task_req.runtime_config {
            if !runtime_config.llm_provider.is_empty() { req_provider = runtime_config.llm_provider.clone(); }
            if !runtime_config.model.is_empty() { req_model = runtime_config.model.clone(); }
            if !runtime_config.llm_endpoint.is_empty() { req_endpoint = runtime_config.llm_endpoint.clone(); }
        }
        let llm = self.resolve_llm(&req_provider, &req_model, &req_endpoint);
        let mut run_cfg = self.build_run_config(&task_req, &task_req.department);
        let task = task_req.task.clone();
        let memory = self.memory.clone();

        let todos: SharedTodos = Arc::new(RwLock::new(Vec::<TodoItem>::new()));
        let task_store: SharedTaskStore = Arc::new(RwLock::new(TaskStore::default()));
        let mailbox: SharedMailbox = Arc::new(RwLock::new(Mailbox::default()));

        let all_tools = ohc_builtin_agent_tools::all_tools(todos, task_store, mailbox);
        let tools = if !task_req.department.is_empty() {
            if let Ok(dep) = Department::from_str(&task_req.department) {
                let dep_cfg = get_department_config(dep);
                all_tools.into_iter()
                    .filter(|t| dep_cfg.allowed_tools.contains(&t.name.as_str()))
                    .collect()
            } else {
                all_tools
            }
        } else {
            all_tools
        };

        // Inject memories into system prompt
        run_cfg.system = inject_memories_into_prompt(&memory, &run_cfg.system);

        let agent = Arc::new(Agent::new(llm, tools));

        let (tx, rx) = mpsc::channel::<Result<RunTaskEvent, Status>>(64);

        // Send RUN_STARTED immediately.
        let _ = tx
            .send(Ok(RunTaskEvent {
                r#type: EventType::RunStarted as i32,
                iteration: 0,
                ..Default::default()
            }))
            .await;

        let agent_clone = agent.clone();
        let start = std::time::Instant::now();

        tokio::spawn(async move {
            let tx_clone = tx.clone();

            let mut on_event = |evt: AgentEvent| {
                let pb = match evt {
                    AgentEvent::RunStarted { iteration } => RunTaskEvent {
                        r#type: EventType::RunStarted as i32,
                        iteration,
                        ..Default::default()
                    },
                    AgentEvent::IterationStarted {
                        iteration,
                        message_count,
                    } => RunTaskEvent {
                        r#type: EventType::IterationStarted as i32,
                        iteration,
                        message_count: message_count as i32,
                        ..Default::default()
                    },
                    AgentEvent::TextChunk { content } => RunTaskEvent {
                        r#type: EventType::TextChunk as i32,
                        content,
                        ..Default::default()
                    },
                    AgentEvent::ToolCall {
                        name,
                        args_json,
                        result,
                        iteration,
                    } => RunTaskEvent {
                        r#type: EventType::ToolCall as i32,
                        tool_name: name,
                        tool_args_json: args_json,
                        tool_result: result,
                        iteration,
                        ..Default::default()
                    },
                    AgentEvent::TaskComplete { content } => RunTaskEvent {
                        r#type: EventType::TaskComplete as i32,
                        content,
                        ..Default::default()
                    },
                    AgentEvent::TaskError { error } => RunTaskEvent {
                        r#type: EventType::TaskError as i32,
                        error,
                        ..Default::default()
                    },
                };
                let _ = tx_clone.try_send(Ok(pb));
            };

            let result = agent_clone
                .run(&run_cfg, &task, &mut on_event)
                .await;

            // Record memory entry.
            let elapsed = start.elapsed();
            let _progress = agent_clone.progress.as_ref();
            let outcome = if result.is_ok() { "success" } else { "failure" };
            memory.write(MemoryEntry {
                task_id: format!("rust-{}", uuid::Uuid::new_v4()),
                summary: result.as_ref().cloned().unwrap_or_default().chars().take(200).collect(),
                tools_used: vec![],
                outcome: outcome.to_string(),
                duration_s: elapsed.as_secs_f64(),
                lessons: String::new(),
                completed_at: Utc::now(),
            });
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn dispatch_to_sub_agent(
        &self,
        req: Request<SubAgentRequest>,
    ) -> Result<Response<SubAgentResponse>, Status> {
        self.check_auth(&req)?;

        let sub_req = req.into_inner();

        // In-process dispatch when no remote address.
        if sub_req.sub_agent_address.is_empty() {
            let mut req_provider = sub_req.llm_provider.clone();
            let mut req_model = sub_req.model.clone();
            let mut req_endpoint = sub_req.llm_endpoint.clone();

            let mut run_cfg = AgentRunConfig {
                model: self.cfg.model.clone(),
                system: self.cfg.system_prompt.clone(),
                max_tokens: if self.cfg.max_tokens == 0 { 2048 } else { self.cfg.max_tokens },
                temperature: self.cfg.temperature,
                max_iterations: 100,
                max_task_tokens: 0,
            };

            if let Some(runtime_config) = &sub_req.runtime_config {
                if !runtime_config.llm_provider.is_empty() { req_provider = runtime_config.llm_provider.clone(); }
                if !runtime_config.model.is_empty() { req_model = runtime_config.model.clone(); run_cfg.model = runtime_config.model.clone(); }
                if !runtime_config.llm_endpoint.is_empty() { req_endpoint = runtime_config.llm_endpoint.clone(); }
                if !runtime_config.system_prompt.is_empty() { run_cfg.system = runtime_config.system_prompt.clone(); }
                if runtime_config.max_tokens > 0 { run_cfg.max_tokens = runtime_config.max_tokens; }
                if runtime_config.temperature > 0.0 { run_cfg.temperature = runtime_config.temperature; }
                if runtime_config.max_iterations > 0 { run_cfg.max_iterations = runtime_config.max_iterations; }
            } else {
                if !sub_req.model.is_empty() { run_cfg.model = sub_req.model.clone(); }
                if !sub_req.system_prompt.is_empty() { run_cfg.system = sub_req.system_prompt.clone(); }
                if sub_req.max_tokens > 0 { run_cfg.max_tokens = sub_req.max_tokens; }
                if sub_req.temperature > 0.0 { run_cfg.temperature = sub_req.temperature; }
            }

            let llm = self.resolve_llm(&req_provider, &req_model, &req_endpoint);

            let todos: SharedTodos = Arc::new(RwLock::new(Vec::<TodoItem>::new()));
            let task_store: SharedTaskStore = Arc::new(RwLock::new(TaskStore::default()));
            let mailbox: SharedMailbox = Arc::new(RwLock::new(Mailbox::default()));
            let tools = ohc_builtin_agent_tools::all_tools(todos, task_store, mailbox);
            let agent = Agent::new(llm, tools);

            let mut no_op = |_: AgentEvent| {};
            let result = agent
                .run(&run_cfg, &sub_req.task, &mut no_op)
                .await
                .unwrap_or_else(|e| e.to_string());

            return Ok(Response::new(SubAgentResponse {
                result,
                error: String::new(),
            }));
        }

        // Remote dispatch: forward to sub-agent gRPC server.
        use crate::proto::{
            agent_service_client::AgentServiceClient, RunTaskRequest,
        };

        let channel = tonic::transport::Channel::from_shared(
            format!("http://{}", sub_req.sub_agent_address),
        )
        .map_err(|e| Status::internal(format!("invalid sub-agent address: {}", e)))?
        .connect()
        .await
        .map_err(|e| Status::internal(format!("connect to sub-agent: {}", e)))?;

        let mut client = AgentServiceClient::new(channel);
        let run_req = RunTaskRequest {
            task: sub_req.task,
            model: sub_req.model,
            llm_provider: sub_req.llm_provider,
            max_tokens: self.cfg.max_tokens,
            ..Default::default()
        };

        let mut stream = client
            .run_task(run_req)
            .await
            .map_err(|e| Status::internal(format!("sub-agent run_task: {}", e)))?
            .into_inner();

        let mut final_result = String::new();
        let mut final_error = String::new();

        loop {
            match stream.message().await {
                Ok(Some(evt)) => {
                    if evt.r#type == EventType::TaskComplete as i32 {
                        final_result = evt.content;
                    } else if evt.r#type == EventType::TaskError as i32 {
                        final_error = evt.error;
                    }
                }
                Ok(None) => break,
                Err(e) => {
                    final_error = e.to_string();
                    break;
                }
            }
        }

        Ok(Response::new(SubAgentResponse {
            result: final_result,
            error: final_error,
        }))
    }
}

pub struct SharedAgentService(pub std::sync::Arc<AgentServiceImpl>);

#[tonic::async_trait]
impl AgentService for SharedAgentService {
    type RunTaskStream = <AgentServiceImpl as AgentService>::RunTaskStream;

    async fn run_task(
        &self,
        req: tonic::Request<RunTaskRequest>,
    ) -> Result<tonic::Response<Self::RunTaskStream>, tonic::Status> {
        self.0.run_task(req).await
    }

    async fn ping(&self, req: tonic::Request<PingRequest>) -> Result<tonic::Response<PingResponse>, tonic::Status> {
        self.0.ping(req).await
    }

    async fn dispatch_to_sub_agent(
        &self,
        req: tonic::Request<SubAgentRequest>,
    ) -> Result<tonic::Response<SubAgentResponse>, tonic::Status> {
        self.0.dispatch_to_sub_agent(req).await
    }
}
