use ohc_builtin_agent::{
    auth::auth_mode_from_env,
    proto::agent_service_server::{AgentService, AgentServiceServer},
    service::{AgentConfig, AgentServiceImpl, DEFAULT_ADDRESS, SharedAgentService},
};
use opentelemetry::{KeyValue, global};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    Resource,
    propagation::TraceContextPropagator,
    runtime,
    trace::{self, Sampler},
};
use prost::Message;
use std::{env, net::SocketAddr};
use tokio_stream::StreamExt;
use tonic::transport::Server;
use tracing::{Level, info};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

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
        .with_resource(Resource::new(vec![KeyValue::new(
            "service.name",
            "ohc-agent",
        )]))
        .build();

    global::set_tracer_provider(tracer_provider.clone());
    let tracer = opentelemetry::trace::TracerProvider::tracer(&tracer_provider, "ohc-agent");

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging and OTEL.
    init_otel();

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
    info!(
        "Starting OHC builtin agent (Rust) at {} (id: {})",
        address, agent_id
    );

    let addr: SocketAddr = address.parse()?;
    let mut svc_impl = AgentServiceImpl::new(agent_id, cfg, auth);
    svc_impl.init_memory().await;
    let svc = std::sync::Arc::new(svc_impl);
    let svc_for_redis = svc.clone();

    let redis_url = get_env("OHC_REDIS_URL", "redis://127.0.0.1:6379");

    let redis_url_clone = redis_url.clone();
    tokio::spawn(async move {
        tracing::info!("Connecting to Redis at {}", redis_url_clone);
        let client = match redis::Client::open(redis_url_clone.clone()) {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Failed to open Redis client: {}", e);
                return;
            }
        };

        let mut con = match client.get_multiplexed_async_connection().await {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Failed to get Redis connection: {}", e);
                return;
            }
        };

        let mut pubsub = match client.get_async_pubsub().await {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Failed to get Redis pubsub connection: {}", e);
                return;
            }
        };

        if let Err(e) = pubsub.subscribe("agent_jobs").await {
            tracing::error!("Failed to subscribe to 'agent_jobs': {}", e);
            return;
        }

        tracing::info!("Subscribed to Redis channel 'agent_jobs'");
        let mut stream = pubsub.on_message();
        while let Some(msg) = stream.next().await {
            let payload: Vec<u8> = msg.get_payload().unwrap_or_default();
            if let Ok(req) = ohc_builtin_agent::proto::RunTaskRequest::decode(&payload[..]) {
                tracing::info!("Received job from Redis: {}", req.task_id);

                let svc = svc_for_redis.clone();
                let mut con_inner = con.clone();

                tokio::spawn(async move {
                    match svc.run_task(tonic::Request::new(req)).await {
                        Ok(resp) => {
                            let mut stream = resp.into_inner();
                            while let Some(Ok(evt)) = stream.next().await {
                                let mut buf = Vec::new();
                                if prost::Message::encode(&evt, &mut buf).is_ok() {
                                    let _: Result<(), _> = redis::cmd("PUBLISH")
                                        .arg("agent_events")
                                        .arg(buf)
                                        .query_async(&mut con_inner)
                                        .await;
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!("Error running task from Redis: {}", e);
                        }
                    }
                });
            }
        }
    });

    Server::builder()
        .add_service(AgentServiceServer::new(SharedAgentService(svc)))
        .serve(addr)
        .await?;

    Ok(())
}
