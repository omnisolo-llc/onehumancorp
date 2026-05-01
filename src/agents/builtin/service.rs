use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::agent::{Agent, AgentEvent, AgentRunConfig};
use crate::auth::AuthMode;
use ohc_builtin_agent_llm::{
    anthropic::AnthropicClient, ollama::OllamaClient, openai::OpenAIClient, LlmClient,
};
use crate::memory::{inject_memories_into_prompt, PgVectorMemoryStore};
use crate::proto::agent_service::{
    agent_service_server::AgentService, EventType, PingRequest, PingResponse, RunTaskEvent,
    RunTaskRequest, SubAgentRequest, SubAgentResponse,
};
use ohc_builtin_agent_tools::{
    sendmessage::Mailbox, task::TaskStore, todowrite::TodoItem, SharedMailbox, SharedTaskStore,
    SharedTodos,
};
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
    memory: Option<Arc<PgVectorMemoryStore>>,
    /// Optional LLM client override for testing.
    llm_override: Option<Arc<dyn LlmClient>>,
}


async fn load_cascading_agents_md(current_dir: &std::path::Path, working_dir: Option<&str>) -> String {
    let current = current_dir.to_path_buf();
    let target = if let Some(wd) = working_dir {
        current.join(wd)
    } else {
        current.clone()
    };

    // Use tokio for canonicalize. Fallback to raw paths if it fails (e.g., dir doesn't exist yet).
    let target_path = tokio::fs::canonicalize(&target).await.unwrap_or(target);
    let current_path = tokio::fs::canonicalize(&current).await.unwrap_or(current);

    let mut paths_to_check = Vec::new();
    let mut ptr = target_path.as_path();

    // Traverse upwards from target to current
    loop {
        paths_to_check.push(ptr.join("AGENTS.md"));
        if ptr == current_path.as_path() {
            break; // Stop at the workspace root
        }
        if let Some(p) = ptr.parent() {
            ptr = p;
            // Prevent going out of the workspace if canonicalization mismatched
            if !ptr.starts_with(&current_path) && ptr != current_path.as_path() {
                break;
            }
        } else {
            break;
        }
    }

    // paths_to_check currently has [leaf, parent, grandparent, ..., root]
    // We want root to leaf (so leaf overrides/appends to root)
    paths_to_check.reverse();

    // Collect all contents, keeping track of them to handle truncation properly
    // so we don't truncate the leaf.
    let mut all_contents = Vec::new();
    for p in paths_to_check {
        if let Ok(content) = tokio::fs::read_to_string(&p).await {
            all_contents.push(content);
        }
    }

    let mut combined = String::new();
    // Reconstruct string backwards to ensure leaf is prioritized if truncation happens.


    for (i, content) in all_contents.iter().enumerate() {
        if i > 0 {
            combined.push_str("\n\n");
        }
        combined.push_str(content);
    }

    if combined.len() > 32768 {
        // We must truncate from the start of the string, keeping the end.
        let overflow = combined.len() - 32768;
        let mut start_idx = overflow;
        while start_idx < combined.len() && !combined.is_char_boundary(start_idx) {
            start_idx += 1;
        }
        // Return the rightmost 32KB
        combined = combined[start_idx..].to_string();
    }

    combined
}

impl AgentServiceImpl {
    pub fn new(agent_id: impl Into<String>, cfg: AgentConfig, auth: AuthMode) -> Self {
        Self {
            agent_id: agent_id.into(),
            cfg,
            auth,
            memory: None,
            llm_override: None,
        }
    }

    pub async fn init_memory(&mut self) {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_default();
        let org_id = std::env::var("OHC_ORGANIZATION_ID").unwrap_or_else(|_| "system".to_string());

        if !db_url.is_empty() {
            match PgVectorMemoryStore::new(&db_url, org_id).await {
                Ok(store) => {
                    self.memory = Some(Arc::new(store));
                }
                Err(e) => {
                    tracing::error!("Failed to connect to database for memory store: {}", e);
                }
            }
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

    async fn build_run_config(&self, req: &RunTaskRequest, department: &str) -> AgentRunConfig {
        let model = if req.model.is_empty() {
            self.cfg.model.clone()
        } else {
            req.model.clone()
        };

        let memories = if let Some(store) = &self.memory {
            store.search(vec![], 5).await.unwrap_or_default()
        } else {
            vec![]
        };

        let server_system_message = if req.system_prompt.is_empty() {
            let base_prompt = if !department.is_empty() {
                if let Ok(dep) = Department::from_str(department) {
                    get_department_config(dep).system_prompt
                } else {
                    &self.cfg.system_prompt
                }
            } else {
                &self.cfg.system_prompt
            };
            inject_memories_into_prompt(&memories, base_prompt)
        } else {
            inject_memories_into_prompt(&memories, &req.system_prompt)
        };

        // Attempt to load AGENTS.md for user instructions
        let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let user_instructions = load_cascading_agents_md(&current_dir, None).await;
        let developer_instructions = "You are a highly capable AI assistant operating within the OneHumanCorp environment. Obey all security rules and always verify your actions.".to_string();

        let max_tokens = if req.max_tokens == 0 {
            if self.cfg.max_tokens == 0 { 2048 } else { self.cfg.max_tokens }
        } else {
            req.max_tokens
        };

        let max_iterations = if req.max_context_messages == 0 {
            self.cfg.max_iterations
        } else {
            req.max_context_messages
        };

        let confidence_threshold = if !department.is_empty() {
            if let Ok(dep) = Department::from_str(department) {
                get_department_config(dep).confidence_threshold
            } else {
                0.0
            }
        } else {
            0.0
        };

        AgentRunConfig {
            model,
            server_system_message,
            developer_instructions,
            user_instructions,
            max_tokens,
            temperature: if req.temperature == 0.0 { self.cfg.temperature } else { req.temperature },
            max_iterations: if max_iterations == 0 { 100 } else { max_iterations },
            max_task_tokens: 0,
            confidence_threshold,
            enable_observation_masking: true,
            guardrails: None,
            enable_llm_judge: false,
        }
    }
}

#[tonic::async_trait]
impl AgentService for AgentServiceImpl {
    #[tracing::instrument(skip(self, req))]
    async fn ping(&self, req: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        self.check_auth(&req)?;
        Ok(Response::new(PingResponse {
            agent_id: self.agent_id.clone(),
            version: AGENT_VERSION.to_string(),
        }))
    }

    type RunTaskStream = ReceiverStream<Result<RunTaskEvent, Status>>;

    #[tracing::instrument(skip(self, req))]
    async fn run_task(
        &self,
        req: Request<RunTaskRequest>,
    ) -> Result<Response<Self::RunTaskStream>, Status> {
        self.check_auth(&req)?;

        let task_req = req.into_inner();
        let llm = self.resolve_llm(&task_req.llm_provider, &task_req.model, &task_req.llm_endpoint);
        let run_cfg = self.build_run_config(&task_req, &task_req.department).await;
        let task = task_req.task.clone();
        let memory = self.memory.clone();

        let todos: SharedTodos = Arc::new(RwLock::new(Vec::<TodoItem>::new()));
        let task_store: SharedTaskStore = Arc::new(RwLock::new(TaskStore::default()));
        let mailbox: SharedMailbox = Arc::new(RwLock::new(Mailbox::default()));
        
        let mut all_tools = ohc_builtin_agent_tools::all_tools(todos, task_store, mailbox, None);
        all_tools.push(agent_tool());
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
        let _start = std::time::Instant::now();

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
            if let (Ok(content), Some(store)) = (&result, &memory) {
                let _ = store.write(content, vec![]).await;
            }
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
            let llm = self.resolve_llm(&sub_req.llm_provider, &sub_req.model, "");
            let run_cfg = AgentRunConfig {
                model: if sub_req.model.is_empty() { self.cfg.model.clone() } else { sub_req.model.clone() },
                server_system_message: self.cfg.system_prompt.clone(),
                developer_instructions: "You are a highly capable AI assistant operating within the OneHumanCorp environment. Obey all security rules and always verify your actions.".to_string(),
                user_instructions: {
                    let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                    load_cascading_agents_md(&current_dir, if sub_req.working_dir.is_empty() { None } else { Some(&sub_req.working_dir) }).await
                },
                max_tokens: if self.cfg.max_tokens == 0 { 2048 } else { self.cfg.max_tokens },
                temperature: self.cfg.temperature,
                max_iterations: 100,
                max_task_tokens: 0,
                confidence_threshold: 0.0,
                enable_observation_masking: true,
                guardrails: None,
            enable_llm_judge: false,
            };

            let todos: SharedTodos = Arc::new(RwLock::new(Vec::<TodoItem>::new()));
            let task_store: SharedTaskStore = Arc::new(RwLock::new(TaskStore::default()));
            let mailbox: SharedMailbox = Arc::new(RwLock::new(Mailbox::default()));

            let working_dir = if sub_req.working_dir.is_empty() { None } else { Some(std::path::PathBuf::from(&sub_req.working_dir)) };
            let mut tools = ohc_builtin_agent_tools::all_tools(todos, task_store, mailbox, working_dir);
            tools.push(agent_tool());
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
        use crate::proto::agent_service::{
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

use ohc_builtin_agent_core::types::ToolError;
use ohc_builtin_agent_tools::{Tool, ToolExecutor};
use serde_json::{json, Value};
use crate::proto::agent_service::agent_service_client::AgentServiceClient;

struct SubagentExecutor;

#[async_trait::async_trait]
impl ToolExecutor for SubagentExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let prompt = args["prompt"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("agent: prompt is required".to_string()))?;

        let task_id = uuid::Uuid::new_v4().to_string();
        let branch_name = format!("subagent-{}", task_id);
        let worktree_path = format!(".agent-worktrees/{}", task_id);

        // Create worktree
        let _ = tokio::process::Command::new("git")
            .args(["branch", &branch_name])
            .output()
            .await;

        let wt_output = tokio::process::Command::new("git")
            .args(["worktree", "add", &worktree_path, &branch_name])
            .output()
            .await;

        if let Err(e) = wt_output {
            return Err(ToolError::LlmRecoverable(format!("Failed to spawn worktree: {}", e)));
        }

        let mut req = SubAgentRequest::default();
        req.task = prompt.to_string();
        req.working_dir = worktree_path.clone();

        let addr = std::env::var("OHC_AGENT_ADDRESS").unwrap_or_else(|_| "127.0.0.1:50051".to_string());
        let res = async {
            let channel = tonic::transport::Channel::from_shared(format!("http://{}", addr))
                .map_err(|e| format!("invalid sub-agent address: {}", e))?
                .connect()
                .await
                .map_err(|e| format!("connect to sub-agent: {}", e))?;
            let mut client = AgentServiceClient::new(channel);
            client.dispatch_to_sub_agent(req).await.map_err(|e| e.to_string())
        }.await;

        // Cleanup
        let _ = tokio::process::Command::new("git")
            .args(["worktree", "remove", "--force", &worktree_path])
            .output()
            .await;
        let _ = tokio::process::Command::new("git")
            .args(["branch", "-D", &branch_name])
            .output()
            .await;

        match res {
            Ok(r) => {
                let inner = r.into_inner();
                if !inner.error.is_empty() {
                    Err(ToolError::LlmRecoverable(inner.error))
                } else {
                    Ok(inner.result)
                }
            }
            Err(e) => Err(ToolError::LlmRecoverable(format!("Subagent failed: {}", e))),
        }
    }
}

pub fn agent_tool() -> Tool {
    Tool {
        name: "Agent".to_string(),
        description: "Spawn a sub-agent to execute a task autonomously. \
            The sub-agent runs its own ReAct loop in an isolated git worktree. \
            Use for parallelizable or delegatable work."
            .to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "prompt": {
                    "type": "string",
                    "description": "The task prompt for the sub-agent."
                },
                "description": {
                    "type": "string",
                    "description": "Short description of the sub-task."
                }
            },
            "required": ["prompt"]
        }),
        execute: Arc::new(SubagentExecutor),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[tokio::test]
    async fn test_load_cascading_agents_md() {
        let base_dir = std::path::PathBuf::from(format!("/tmp/ohc_test_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&base_dir).unwrap();

        let mut root_file = std::fs::File::create(base_dir.join("AGENTS.md")).unwrap();
        root_file.write_all(b"ROOT INSTRUCTION").unwrap();
        root_file.flush().unwrap();

        std::fs::create_dir_all(base_dir.join("nested")).unwrap();
        let mut nested_file = std::fs::File::create(base_dir.join("nested").join("AGENTS.md")).unwrap();
        nested_file.write_all(b"NESTED INSTRUCTION").unwrap();
        nested_file.flush().unwrap();

        let result = load_cascading_agents_md(&base_dir, Some("nested")).await;

        let _ = std::fs::remove_dir_all(&base_dir);

        assert_eq!(result, "ROOT INSTRUCTION\n\nNESTED INSTRUCTION");
    }

    #[tokio::test]
    async fn test_load_cascading_agents_md_truncation() {
        let base_dir = std::path::PathBuf::from(format!("/tmp/ohc_test_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&base_dir).unwrap();

        let mut root_file = std::fs::File::create(base_dir.join("AGENTS.md")).unwrap();
        let massive_str = "A".repeat(40000);
        root_file.write_all(massive_str.as_bytes()).unwrap();
        root_file.flush().unwrap();

        std::fs::create_dir_all(base_dir.join("nested")).unwrap();
        let mut nested_file = std::fs::File::create(base_dir.join("nested").join("AGENTS.md")).unwrap();
        nested_file.write_all(b"CRITICAL_LEAF").unwrap();
        nested_file.flush().unwrap();

        let result = load_cascading_agents_md(&base_dir, Some("nested")).await;

        let _ = std::fs::remove_dir_all(&base_dir);

        assert!(result.len() <= 32768);
        assert!(result.ends_with("CRITICAL_LEAF"));
    }
}
