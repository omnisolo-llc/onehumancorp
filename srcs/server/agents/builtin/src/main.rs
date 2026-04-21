use ohc_builtin_agent::{
    auth::auth_mode_from_env,
    proto::agent_service::agent_service_server::AgentServiceServer,
    service::{AgentConfig, AgentServiceImpl, DEFAULT_ADDRESS},
};
use std::{env, net::SocketAddr};
use tonic::transport::Server;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn get_env(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

fn get_env_int(key: &str, default: i32) -> i32 {
    env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let address = get_env("OHC_AGENT_ADDRESS", DEFAULT_ADDRESS);
    let agent_id = get_env(
        "OHC_AGENT_ID",
        &uuid::Uuid::new_v4().hyphenated().to_string(),
    );

    let cfg = AgentConfig {
        llm_provider: get_env("OHC_LLM_PROVIDER", ""),
        model: get_env("OHC_LLM_MODEL", ""),
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
    info!("Starting OHC builtin agent (Rust) at {} (id: {})", address, agent_id);

    let addr: SocketAddr = address.parse()?;
    let svc = AgentServiceImpl::new(agent_id, cfg, auth);

    Server::builder()
        .add_service(AgentServiceServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
