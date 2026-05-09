use ohc_builtin_agent::{
    auth::{auth_mode_from_env, AuthMode},
    proto::agent_service_server::AgentServiceServer,
    service::{AgentConfig, AgentServiceImpl, DEFAULT_ADDRESS, SharedAgentService},
    agent::{Agent, AgentRunConfig, AgentEvent},
    llm::LlmClient,
};
use std::{env, net::SocketAddr, sync::Arc};
use tonic::transport::Server;
use clap::Parser;
use serde_json;
use tracing::{info, Level};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use opentelemetry::{global, KeyValue};
use opentelemetry_sdk::{
    propagation::TraceContextPropagator,
    runtime,
    Resource,
};
use opentelemetry_otlp::WithExportConfig;

fn get_env(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

fn get_env_int(key: &str, default: i32) -> i32 {
    env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn init_otel() {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let otlp_endpoint = env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:4317".to_string());

    let tracer = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(otlp_endpoint)
        .build()
        .expect("failed to build tracer");

    let tracer_provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_batch_exporter(tracer, runtime::Tokio)
        .with_resource(Resource::new(vec![KeyValue::new("service.name", "ohc-agent")]))
        .build();

    global::set_tracer_provider(tracer_provider.clone());
    let tracer = opentelemetry::trace::TracerProvider::tracer(&tracer_provider, "ohc-agent");

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Task for autonomous execution.
    #[arg(long)]
    task: Option<String>,

    /// Git worktree path for isolated execution.
    #[arg(long)]
    worktree: Option<String>,

    /// Parent context JSON (for forks).
    #[arg(long)]
    parent_context: Option<String>,

    /// Mailbox directory for teammate communication.
    #[arg(long)]
    mailbox: Option<String>,

    /// LLM provider override.
    #[arg(long)]
    provider: Option<String>,

    /// LLM model override.
    #[arg(long)]
    model: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging and OTEL.
    init_otel();

    let args = Args::parse();

    let address = get_env("OHC_AGENT_ADDRESS", DEFAULT_ADDRESS);
    let agent_id = get_env(
        "OHC_AGENT_ID",
        &uuid::Uuid::new_v4().hyphenated().to_string(),
    );

    let cfg = AgentConfig {
        llm_provider: args.provider.unwrap_or_else(|| get_env("OHC_LLM_PROVIDER", "")),
        model: args.model.unwrap_or_else(|| get_env("OHC_LLM_MODEL", "")),
        llm_endpoint: get_env("OHC_LOCAL_LLM_ENDPOINT", ""),
        system_prompt: get_env("OHC_SYSTEM_PROMPT", ""),
        max_tokens: get_env_int("OHC_MAX_TOKENS", 2048),
        temperature: env::var("OHC_TEMPERATURE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0),
        max_iterations: get_env_int("OHC_MAX_ITERATIONS", 100),
        max_context_messages: get_env_int("OHC_MAX_CONTEXT_MESSAGES", 80),
    };

    let auth = auth_mode_from_env();

    if let Some(task) = args.task {
        info!("Running autonomous task (id: {}): {}", agent_id, task);

        // Subagent Orchestration (Worktree/Fork) mechanic:
        // When running in autonomous mode, we might be in a worktree.
        if let Some(wt) = &args.worktree {
            info!("Entering worktree: {}", wt);
            std::env::set_current_dir(wt)?;
        }

        let mut svc_impl = AgentServiceImpl::new(agent_id.clone(), cfg, AuthMode::Disabled);
        svc_impl.init_memory().await;

        let llm = svc_impl.resolve_llm(&svc_impl.cfg.llm_provider, &svc_impl.cfg.model, &svc_impl.cfg.llm_endpoint);

        // State Management: Fork mechanic - load parent context if provided
        let injected_context = if let Some(ctx_json) = args.parent_context {
            serde_json::from_str(&ctx_json).ok()
        } else {
            None
        };

        let run_cfg = AgentRunConfig {
            agent_id: agent_id.clone(),
            model: svc_impl.cfg.model.clone(),
            server_system_message: svc_impl.cfg.system_prompt.clone(),
            injected_context,
            workspace_path: args.worktree.clone(),
            // Rule: Subagents return 1k-2k token condensed summaries.
            is_subagent: true,
            ..AgentRunConfig::default()
        };

        let observation_store = Arc::new(dashmap::DashMap::new());
        let tools = ohc_builtin_agent_tools::all_tools(
            Arc::new(tokio::sync::RwLock::new(vec![])),
            Arc::new(tokio::sync::RwLock::new(ohc_builtin_agent_tools::task::TaskStore::default())),
            Arc::new(tokio::sync::RwLock::new(ohc_builtin_agent_tools::sendmessage::Mailbox::default())),
            args.worktree.map(std::path::PathBuf::from),
            None,
            observation_store.clone(),
        );

        let mut agent = Agent::new(llm, tools);
        agent.observation_store = observation_store;

        let mut no_op = |evt: AgentEvent| {
             match evt {
                 AgentEvent::TextChunk { content } => print!("{}", content),
                 AgentEvent::ToolCall { name, result, .. } => info!("Tool {}: {}", name, result),
                 AgentEvent::TaskComplete { content } => println!("\n[TASK COMPLETE]\n{}", content),
                 AgentEvent::TaskError { error } => eprintln!("\n[TASK ERROR] {}", error),
                 _ => {}
             }
        };

        let result = agent.run(&run_cfg, &task, &mut no_op).await?;

        // Teammate communication: Write result to outbox if mailbox is provided
        if let Some(mailbox) = args.mailbox {
            let outbox = std::path::Path::new(&mailbox).join("outbox.txt");
            tokio::fs::write(outbox, result).await?;
        } else {
            println!("{}", result);
        }

        return Ok(());
    }

    info!("Starting OHC builtin agent (Rust) gRPC server at {} (id: {})", address, agent_id);

    let addr: SocketAddr = address.parse()?;
    let mut svc_impl = AgentServiceImpl::new(agent_id.clone(), cfg, auth);
    svc_impl.init_memory().await;
    let svc = std::sync::Arc::new(svc_impl);
    let svc_for_redis = svc.clone();

    let is_cloud = get_env("STANDALONE_MODE", "true") != "true";
    let redis_url = get_env("OHC_REDIS_URL", "redis://127.0.0.1:6379");
    
    match ohc_builtin_agent::mesh::transport::create_transport(Some(&redis_url), is_cloud).await {
        Ok(transport) => {
            let heartbeat_transport = transport.clone();
            let heartbeat_agent_id = agent_id.clone();
            tokio::spawn(async move {
                loop {
                    if let Err(e) = heartbeat_transport.register_presence(&heartbeat_agent_id, "active", 30).await {
                        tracing::error!("Failed to register presence: {}", e);
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(15)).await;
                }
            });
            ohc_builtin_agent::start_builtin_agent(transport, svc_for_redis).await;
        }
        Err(e) => {
            tracing::error!("Failed to create mesh transport: {}", e);
        }
    }

    Server::builder()
        .add_service(AgentServiceServer::new(SharedAgentService(svc)))
        .serve(addr)
        .await?;

    Ok(())
}
