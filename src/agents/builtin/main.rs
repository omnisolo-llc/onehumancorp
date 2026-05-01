use ohc_builtin_agent::{
    auth::auth_mode_from_env,
    proto::agent_service_server::{AgentServiceServer, AgentService},
    service::{AgentConfig, AgentServiceImpl, DEFAULT_ADDRESS, SharedAgentService},
};
use std::{env, net::SocketAddr};
use tonic::transport::Server;
use tracing::{info, Level};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use prost::Message;
use tokio_stream::StreamExt;
use opentelemetry::{global, KeyValue};
use opentelemetry_sdk::{
    propagation::TraceContextPropagator,
    runtime,
    trace::{self, Sampler},
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
    info!("Starting OHC builtin agent (Rust) at {} (id: {})", address, agent_id);

    let addr: SocketAddr = address.parse()?;
    let mut svc_impl = AgentServiceImpl::new(agent_id, cfg, auth);
    svc_impl.init_memory().await;
    let svc = std::sync::Arc::new(svc_impl);
    let svc_for_redis = svc.clone();

        let redis_url = get_env("OHC_REDIS_URL", "redis://127.0.0.1:6379");
    let standalone = env::var("STANDALONE_MODE").unwrap_or_else(|_| "true".to_string()) == "true";
    let transport = ohc_mesh::transport::create_transport(
        Some(redis_url.as_str()),
        standalone
    ).await.expect("Failed to create MeshTransport in builtin agent");

    // State Handoff: Publish a sync request to the mesh on startup.
    // The server listens to "agent_sync" and will re-publish any pending tasks for this agent
    // to "agent_jobs", guaranteeing no missed tasks during mode switches.
    let agent_id_for_sync = agent_id.clone();
    let transport_for_sync = transport.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        let sync_msg = ohc_mesh::transport::Message {
            topic: "agent_sync".to_string(),
            payload: agent_id_for_sync.into_bytes(),
        };
        if let Err(e) = transport_for_sync.publish("agent_sync", sync_msg).await {
            tracing::warn!("Failed to publish state synchronization request: {}", e);
        } else {
            tracing::info!("Published state synchronization request to MeshTransport");
        }
    });

    let transport_clone = transport.clone();
    let transport_for_sub = transport.clone();

    let _ = transport_for_sub.subscribe("agent_jobs", Box::new(move |msg| {
        let payload = msg.payload;
        if let Ok(req) = ohc_builtin_agent::proto::RunTaskRequest::decode(&payload[..]) {
            tracing::info!("Received job from MeshTransport: {}", req.task_id);

            let svc = svc_for_redis.clone();
            let transport_inner = transport_clone.clone();

            tokio::spawn(async move {
                match svc.run_task(tonic::Request::new(req)).await {
                    Ok(resp) => {
                        let mut stream = resp.into_inner();
                        while let Some(Ok(evt)) = stream.next().await {
                            let mut buf = Vec::new();
                            if prost::Message::encode(&evt, &mut buf).is_ok() {
                                let _ = transport_inner.publish("agent_events", ohc_mesh::transport::Message {
                                    topic: "agent_events".to_string(),
                                    payload: buf,
                                }).await;
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Error running task from MeshTransport: {}", e);
                    }
                }
            });
        }
    })).await.expect("Failed to subscribe to agent_jobs");

    Server::builder()
        .add_service(AgentServiceServer::new(SharedAgentService(svc)))
        .serve(addr)
        .await?;

    Ok(())
}
