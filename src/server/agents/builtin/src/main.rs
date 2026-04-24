use ohc_builtin_agent::{
    auth::auth_mode_from_env,
    proto::agent_service_server::{AgentServiceServer, AgentService},
    service::{AgentConfig, AgentServiceImpl, DEFAULT_ADDRESS, SharedAgentService},
};
use std::{env, net::SocketAddr};
use tonic::transport::Server;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use prost::Message;
use tokio_stream::StreamExt;

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
    let svc = std::sync::Arc::new(AgentServiceImpl::new(agent_id, cfg, auth));
    let svc_for_redis = svc.clone();

    let redis_url = get_env("OHC_REDIS_URL", "redis://127.0.0.1:6379");
    
    let redis_url_clone = redis_url.clone();
    tokio::spawn(async move {
        tracing::info!("Connecting to Redis at {}", redis_url_clone);
        if let Ok(client) = redis::Client::open(redis_url_clone.clone()) {
            if let Ok(mut con) = client.get_async_connection().await {
                let mut pubsub = con.into_pubsub();
                if pubsub.subscribe("agent_jobs").await.is_ok() {
                    tracing::info!("Subscribed to Redis channel 'agent_jobs'");
                    let mut stream = pubsub.on_message();
                    while let Some(msg) = stream.next().await {
                        let payload: Vec<u8> = msg.get_payload().unwrap_or_default();
                        if let Ok(req) = ohc_builtin_agent::proto::RunTaskRequest::decode(&payload[..]) {
                            tracing::info!("Received job from Redis: {}", req.task_id);
                            
                            let svc = svc_for_redis.clone();
                            let redis_url_inner = redis_url_clone.clone();
                            
                            tokio::spawn(async move {
                                if let Ok(client) = redis::Client::open(redis_url_inner) {
                                    if let Ok(mut con) = client.get_async_connection().await {
                                        match svc.run_task(tonic::Request::new(req)).await {
                                            Ok(resp) => {
                                                let mut stream = resp.into_inner();
                                                while let Some(Ok(evt)) = stream.next().await {
                                                    let mut buf = Vec::new();
                                                    if prost::Message::encode(&evt, &mut buf).is_ok() {
                                                        let _: Result<(), _> = redis::cmd("PUBLISH")
                                                            .arg("agent_events")
                                                            .arg(buf)
                                                            .query_async(&mut con)
                                                            .await;
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                tracing::error!("Error running task: {}", e);
                                            }
                                        }
                                    }
                                }
                            });
                        }
                    }
                }
            }
        }
    });

    Server::builder()
        .add_service(AgentServiceServer::new(SharedAgentService(svc)))
        .serve(addr)
        .await?;

    Ok(())
}
