pub mod claude_subagents;
pub mod compaction;
pub mod durable_execution;
pub mod omni_context;
pub mod plugins;
pub mod scalable_multi_agent;
// ohc-builtin-agent: Rust reimplementation of the OHC builtin agent.
//
// Configuration via environment variables:
//   OHC_AGENT_ADDRESS          gRPC listen address (default: 127.0.0.1:50051)
//   OHC_AGENT_ID               agent identifier
//   ANTHROPIC_API_KEY          enables Anthropic Claude backend
//   OPENAI_API_KEY             enables OpenAI backend
//   MINIMAX_API_KEY            enables MiniMax backend
//   OHC_LLM_API_KEY            generic key for OpenAI-compatible backends
//   OHC_LLM_BASE_URL           generic OpenAI-compatible /v1 API base URL
//   OHC_LOCAL_LLM_ENDPOINT     Ollama endpoint
//   OHC_LLM_PROVIDER           "anthropic" | "openai" | "openai-compatible" | "minimax" | "ollama"
//   OHC_LLM_MODEL              LLM model name
//   OHC_MAX_TOKENS             max tokens per LLM response (default 2048)
//   OHC_MAX_ITERATIONS         max ReAct iterations (default 100)
//   OHC_AGENT_WORKSPACE        workspace/sandbox root for file and shell tools
//   OHC_AGENT_EXECUTION_MODE   "standalone" | "cluster" | "cloud"; cluster/cloud use containers when available
//   OHC_AGENT_COMMAND_BACKEND  "container" to force Docker/Podman execution
//   OHC_AGENT_CONTAINER_IMAGE  container image for cluster command execution (default alpine:3.20)
//   OHC_AGENT_AUTH_DISABLED    "true" to disable auth (dev/test only)
//   OHC_AGENT_TOKEN            pre-shared token for token-based auth

pub use ohc_builtin_agent_core::*;

pub mod agent;
pub mod human_in_loop;
pub mod observability;
pub mod observation_masking;
pub mod perplexity;
pub mod tools_gating;
pub mod verification_loops;

pub mod autogen;
pub mod departments;
pub mod goose;
pub mod guardrails;
pub mod in_memory_store;
pub mod json_store;
pub mod memory;
pub mod memory_exhaustive_tests;
pub mod memory_store;
pub mod openhands;
pub mod openhands_runner;
pub mod prompt_construction;
pub mod ralph_loop;
pub mod ruflo;
pub mod service;

pub use ohc_builtin_agent_llm as llm;
pub use ohc_builtin_agent_tools as tools;
pub mod mesh;
pub mod proto;
pub use service::start_builtin_agent;

pub mod checkpointer;
pub mod codex_runner;
pub mod consolidation_worker;
pub mod harness;
pub mod hibernation;
pub mod json_rpc_server;
pub mod langgraph;
pub mod plane;
pub mod progressive_skills;
pub mod provider;
pub mod registry;
pub mod sqlite_memory;

pub mod actor_model;
pub mod agent_protocol;
pub mod gpt_researcher;
pub mod llm_condensation;
pub mod plan_and_execute;
pub mod sona_patterns;
pub mod swarm_topology;
pub mod visual_workflow;
pub mod visual_workflow_client;

pub mod agentic_seek;
pub mod pi;
pub mod ruflo_plugins;
pub mod tool_executor_engine;

fn get_env(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

fn get_env_int(key: &str, default: i32) -> i32 {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

async fn run_direct_workflow_if_requested(task: &str) -> Option<Result<String, String>> {
    if !task.contains("Use the built-in RunWorkflow tool.") {
        return None;
    }
    let json_start = match task.find('{') {
        Some(index) => index,
        None => {
            return Some(Err(
                "RunWorkflow instruction did not include JSON arguments".to_string(),
            ));
        }
    };
    let mut deserializer = serde_json::Deserializer::from_str(&task[json_start..]);
    let args = match <serde_json::Value as serde::Deserialize>::deserialize(&mut deserializer) {
        Ok(args) => args,
        Err(err) => {
            return Some(Err(format!(
                "Failed to parse RunWorkflow arguments: {}",
                err
            )));
        }
    };

    use ohc_builtin_agent_tools::ToolExecutor;
    use ohc_builtin_agent_tools::pydantic::PydanticAdapter;
    let runner =
        std::sync::Arc::new(ohc_builtin_agent_tools::runner::SandboxedCommandRunner::new(None));
    let executor = ohc_builtin_agent_tools::workflow::WorkflowExecutor { runner };
    let adapter = PydanticAdapter::new(executor);
    Some(
        adapter
            .execute(args)
            .await
            .map_err(|err| format!("{:?}", err)),
    )
}

async fn hold_specialist_exit_if_requested(task: &str) {
    if !task.contains("Specialist:") {
        return;
    }
    let secs = std::env::var("OHC_AGENT_SPECIALIST_EXIT_HOLD_SECS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(0);
    if secs > 0 {
        tokio::time::sleep(std::time::Duration::from_secs(secs)).await;
    }
}

fn init_otel() {
    use opentelemetry_otlp::WithExportConfig;
    opentelemetry::global::set_text_map_propagator(
        opentelemetry_sdk::propagation::TraceContextPropagator::new(),
    );

    let otlp_endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:4317".to_string());

    let tracer = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(otlp_endpoint)
        .build()
        .expect("failed to build tracer");

    let tracer_provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_batch_exporter(tracer, opentelemetry_sdk::runtime::Tokio)
        .with_resource(opentelemetry_sdk::Resource::new(vec![
            opentelemetry::KeyValue::new("service.name", "ohc-agent"),
        ]))
        .build();

    opentelemetry::global::set_tracer_provider(tracer_provider.clone());
    let tracer = opentelemetry::trace::TracerProvider::tracer(&tracer_provider, "ohc-agent");

    let use_json = std::env::var("LOG_FORMAT").unwrap_or_default() == "json";

    if use_json {
        use tracing_subscriber::prelude::*;
        let _ = tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().json())
            .with(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing::Level::INFO.into()),
            )
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .try_init();
    } else {
        use tracing_subscriber::prelude::*;
        let _ = tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .with(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing::Level::INFO.into()),
            )
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .try_init();
    }
}

pub async fn run_agent() -> Result<(), Box<dyn std::error::Error>> {
    use crate::proto::agent_service::agent_service_server::AgentService;
    // Set up logging and OTEL.
    init_otel();

    let args: Vec<String> = std::env::args().collect();
    let mut task = None;
    let mut parent_context_file = None;
    let mut worktree = None;
    let mut mailbox = None;
    let mut ralph_loop = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--ralph-loop" => {
                ralph_loop = true;
            }
            "--task" => {
                if i + 1 < args.len() {
                    task = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--parent-context-file" => {
                if i + 1 < args.len() {
                    parent_context_file = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--worktree" => {
                if i + 1 < args.len() {
                    worktree = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--mailbox" => {
                if i + 1 < args.len() {
                    mailbox = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    let address = get_env("OHC_AGENT_ADDRESS", service::DEFAULT_ADDRESS);
    let agent_id = get_env(
        "OHC_AGENT_ID",
        &uuid::Uuid::new_v4().hyphenated().to_string(),
    );

    let cfg = service::AgentConfig {
        llm_provider: get_env("OHC_LLM_PROVIDER", ""),
        model: get_env("OHC_LLM_MODEL", ""),
        llm_endpoint: get_env(
            "OHC_LLM_BASE_URL",
            &get_env("OHC_LLM_ENDPOINT", &get_env("OHC_LOCAL_LLM_ENDPOINT", "")),
        ),
        system_prompt: get_env("OHC_SYSTEM_PROMPT", ""),
        max_tokens: get_env_int("OHC_MAX_TOKENS", 2048),
        temperature: std::env::var("OHC_TEMPERATURE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0),
        max_iterations: get_env_int("OHC_MAX_ITERATIONS", 100),
        max_context_messages: get_env_int("OHC_MAX_CONTEXT_MESSAGES", 80),
    };

    let auth = auth::auth_mode_from_env().map_err(|error| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("invalid agent authentication configuration: {error}"),
        )
    })?;

    let mut svc_impl = service::AgentServiceImpl::new(agent_id.clone(), cfg.clone(), auth);
    svc_impl.init_memory().await;

    if let Some(t) = task {
        // Run as a subagent (Fork, Worktree, Teammate)
        let working_dir = worktree.or(mailbox).unwrap_or_default();

        let parent_context_json = if let Some(path) = parent_context_file {
            std::fs::read_to_string(&path).unwrap_or_default()
        } else {
            String::new()
        };

        if let Some(result) = run_direct_workflow_if_requested(&t).await {
            match result {
                Ok(report) => {
                    tracing::info!("{}", report);
                    return Ok(());
                }
                Err(err) => {
                    tracing::error!("{}", err);
                    std::process::exit(1);
                }
            }
        }

        if ralph_loop {
            let req = proto::agent_service::RunTaskRequest {
                task_id: uuid::Uuid::new_v4().hyphenated().to_string(),
                task: t,
                llm_provider: cfg.llm_provider.clone(),
                model: cfg.model.clone(),
                llm_endpoint: cfg.llm_endpoint.clone(),
                department: "ralph".to_string(),
                ..Default::default()
            };
            svc_impl.run_ralph_loop(req).await;
            return Ok(());
        } else {
            let task_for_hold = t.clone();
            let req = proto::agent_service::SubAgentRequest {
                task: t,
                working_dir,
                parent_context_json,
                ..Default::default()
            };

            match svc_impl
                .dispatch_to_sub_agent(svc_impl.trusted_request(req))
                .await
            {
                Ok(resp) => {
                    let inner = resp.into_inner();
                    if !inner.error.is_empty() {
                        tracing::error!("{}", inner.error);
                        hold_specialist_exit_if_requested(&task_for_hold).await;
                        std::process::exit(1);
                    } else {
                        tracing::info!("{}", inner.result);
                        hold_specialist_exit_if_requested(&task_for_hold).await;
                        return Ok(());
                    }
                }
                Err(e) => {
                    tracing::error!("Subagent dispatch error: {}", e);
                    hold_specialist_exit_if_requested(&task_for_hold).await;
                    std::process::exit(1);
                }
            }
        }
    }

    tracing::info!(
        "Starting OHC builtin agent (Rust) at {} (id: {})",
        address,
        agent_id
    );

    let addr: std::net::SocketAddr = address.parse()?;
    let svc = std::sync::Arc::new(svc_impl);
    let svc_for_redis = svc.clone();

    let standalone_mode =
        std::env::var("OHC_STANDALONE_MODE").unwrap_or_else(|_| "true".to_string());
    let is_cloud = standalone_mode != "true";
    let redis_url = get_env("OHC_REDIS_URL", "redis://127.0.0.1:6379");

    match mesh::transport::create_transport(Some(&redis_url), is_cloud).await {
        Ok(transport) => {
            let heartbeat_transport = transport.clone();
            let heartbeat_agent_id = agent_id.clone();
            tokio::spawn(async move {
                loop {
                    if let Err(e) = heartbeat_transport
                        .register_presence(&heartbeat_agent_id, "active", 30)
                        .await
                    {
                        tracing::error!("Failed to register presence: {}", e);
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(15)).await;
                }
            });
            start_builtin_agent(transport, svc_for_redis).await;
        }
        Err(e) => {
            tracing::error!("Failed to create mesh transport: {}", e);
        }
    }

    tonic::transport::Server::builder()
        .add_service(
            proto::agent_service::agent_service_server::AgentServiceServer::new(
                service::SharedAgentService(svc),
            ),
        )
        .serve(addr)
        .await?;

    Ok(())
}
pub mod aider_repomap;
pub mod deerflow;
pub mod deerflow_subagents;
pub mod jit_retrieval;
pub mod microagent;
