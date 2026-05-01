use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct QueueState {
    pub is_standalone: bool,
    pub redis_url: String,
    pub mesh_transport: Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>,
}

#[derive(Deserialize, Debug)]
pub struct SpawnRequest {
    pub job_name: String,
    pub agent_id: String,
    pub payload: serde_json::Value,
    #[serde(default)]
    pub standalone_fallback: bool,
}

#[derive(Serialize)]
pub struct SpawnResponse {
    pub status: String,
    pub job_id: Option<String>,
    pub message: String,
}

#[derive(Clone)]
pub struct AppState {
    pub mesh_transport: std::sync::Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>,
    pub queue_state: QueueState,
}

pub async fn spawn_agent(
    State(app_state): State<AppState>,
    Json(payload): Json<SpawnRequest>,
) -> impl IntoResponse {
    let fallback = app_state.queue_state.is_standalone || payload.standalone_fallback;

    if fallback {
        // Standalone Mode Fallback
        println!("Spawning local agent process");

        let mesh_transport = app_state.mesh_transport.clone();
        let agent_id = payload.agent_id.clone();

        tokio::spawn(async move {
            let cfg = ohc_builtin_agent::service::AgentConfig {
                llm_provider: std::env::var("OHC_LLM_PROVIDER").unwrap_or_default(),
                model: std::env::var("OHC_LLM_MODEL").unwrap_or_default(),
                llm_endpoint: std::env::var("OHC_LOCAL_LLM_ENDPOINT").unwrap_or_default(),
                system_prompt: std::env::var("OHC_SYSTEM_PROMPT").unwrap_or_default(),
                max_tokens: std::env::var("OHC_MAX_TOKENS").ok().and_then(|v| v.parse().ok()).unwrap_or(2048),
                temperature: std::env::var("OHC_TEMPERATURE").ok().and_then(|v| v.parse().ok()).unwrap_or(0.0),
                max_iterations: std::env::var("OHC_MAX_ITERATIONS").ok().and_then(|v| v.parse().ok()).unwrap_or(100),
                max_context_messages: std::env::var("OHC_MAX_CONTEXT_MESSAGES").ok().and_then(|v| v.parse().ok()).unwrap_or(80),
            };
            let auth = ohc_builtin_agent::auth::auth_mode_from_env();
            let mut svc_impl = ohc_builtin_agent::service::AgentServiceImpl::new(agent_id, cfg, auth);
            svc_impl.init_memory().await;
            let svc = Arc::new(svc_impl);
            ohc_builtin_agent::start_builtin_agent(mesh_transport, svc).await;
        });

        return (
            StatusCode::OK,
            Json(SpawnResponse {
                status: "success".to_string(),
                job_id: None,
                message: "Spawned local background agent successfully.".to_string(),
            }),
        );
    }

    // Production Mode: Enqueue to Redis via BullMQ (hornetmq)
    println!("Enqueuing job to BullMQ/Redis");

    let redis_url = if app_state.queue_state.redis_url.is_empty() {
        std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string())
    } else {
        app_state.queue_state.redis_url.clone()
    };

    // Use hornetmq
    let mut queue = hornetmq::Queue::new(payload.job_name.clone(), redis_url);

    let opts = hornetmq::AddJobOptions {
        delay: None,
        priority: None,
        attempts: Some(3),
        backoff: Some(hornetmq::BackoffStrategy::Exponential { base: 1000, max: 10000 }),
        job_id: None,
        lifo: None,
        remove_on_complete: None,
        remove_on_fail: None,
    };

    match queue.add(&payload.job_name, payload.payload.clone(), opts) {
        Ok(job) => {
            (
                StatusCode::OK,
                Json(SpawnResponse {
                    status: "success".to_string(),
                    job_id: Some(job),
                    message: "Job enqueued successfully.".to_string(),
                }),
            )
        }
        Err(e) => {
            eprintln!("Failed to add job to queue: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SpawnResponse {
                    status: "error".to_string(),
                    job_id: None,
                    message: format!("Failed to enqueue job: {:?}", e),
                }),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::{State, Json};
    use axum::response::IntoResponse;
    use ohc_builtin_agent::mesh::transport::MemoryTransport;

    #[tokio::test]
    async fn test_spawn_agent_standalone_fallback() {
        let transport = std::sync::Arc::new(MemoryTransport::new());
        let app_state = AppState {
            mesh_transport: transport.clone(),
            queue_state: QueueState {
                is_standalone: true,
                redis_url: "".to_string(),
                mesh_transport: transport,
            },
        };

        let payload = SpawnRequest {
            job_name: "test_job".to_string(),
            agent_id: "agent_123".to_string(),
            payload: serde_json::json!({}),
            standalone_fallback: true,
        };

        let response = spawn_agent(State(app_state), Json(payload)).await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }
}
