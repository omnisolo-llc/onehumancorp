// ohc-builtin-agent: Rust reimplementation of the OHC builtin agent.
//
// Configuration via environment variables:
//   OHC_AGENT_ADDRESS          gRPC listen address (default: 127.0.0.1:50051)
//   OHC_AGENT_ID               agent identifier
//   ANTHROPIC_API_KEY          enables Anthropic Claude backend
//   OPENAI_API_KEY             enables OpenAI backend
//   OHC_LOCAL_LLM_ENDPOINT     Ollama endpoint
//   OHC_LLM_PROVIDER           "anthropic" | "openai" | "ollama"
//   OHC_LLM_MODEL              LLM model name
//   OHC_MAX_TOKENS             max tokens per LLM response (default 2048)
//   OHC_MAX_ITERATIONS         max ReAct iterations (default 100)
//   OHC_AGENT_AUTH_DISABLED    "true" to disable auth (dev/test only)
//   OHC_AGENT_TOKEN            pre-shared token for token-based auth
use crate::mesh::TeammateMesh;

pub use ohc_builtin_agent_core::*;

pub mod agent;
pub mod service;
pub mod departments;
pub mod guardrails;

pub use ohc_builtin_agent_llm as llm;
pub use ohc_builtin_agent_tools as tools;
pub mod proto;

pub mod mesh;

use std::{env, net::SocketAddr};
use std::sync::Arc;
use tonic::transport::Server;
use tokio_stream::StreamExt;
use prost::Message;

pub async fn start_builtin_agent(
    address: &str,
    agent_id: &str,
    mesh_client: Arc<dyn crate::mesh::TeammateMesh>,
) -> Result<(), Box<dyn std::error::Error>> {
    let cfg = service::AgentConfig {
        llm_provider: env::var("OHC_LLM_PROVIDER").unwrap_or_default(),
        model: env::var("OHC_LLM_MODEL").unwrap_or_default(),
        llm_endpoint: env::var("OHC_LOCAL_LLM_ENDPOINT").unwrap_or_default(),
        system_prompt: env::var("OHC_SYSTEM_PROMPT").unwrap_or_default(),
        max_tokens: env::var("OHC_MAX_TOKENS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(2048),
        temperature: env::var("OHC_TEMPERATURE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0),
        max_iterations: env::var("OHC_MAX_ITERATIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100),
        max_context_messages: env::var("OHC_MAX_CONTEXT_MESSAGES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(80),
    };

    let auth = auth::auth_mode_from_env();
    tracing::info!("Starting in-process OHC builtin agent (Rust) at {} (id: {})", address, agent_id);

    let addr: SocketAddr = address.parse()?;
    let mut svc_impl = service::AgentServiceImpl::new(agent_id.to_string(), cfg, auth);
    svc_impl.init_memory().await;
    let svc = Arc::new(svc_impl);

    let mesh_client_for_run = mesh_client.clone();
    let svc_for_mesh = svc.clone();

    use crate::proto::agent_service_server::AgentService;
    let handler = Box::new(move |msg: crate::mesh::transport::Message| {
        if let Ok(req) = proto::RunTaskRequest::decode(&msg.payload[..]) {
            tracing::info!("Received job from Mesh (in-process): {}", req.task_id);

            let svc = svc_for_mesh.clone();
            let mesh = mesh_client_for_run.clone();

            tokio::spawn(async move {
                match svc.run_task(tonic::Request::new(req)).await {
                    Ok(resp) => {
                        let mut stream = resp.into_inner();
                        while let Some(Ok(evt)) = stream.next().await {
                            let mut buf = Vec::new();
                            if prost::Message::encode(&evt, &mut buf).is_ok() {
                                if let Err(e) = mesh.publish_coordination(buf).await {
                                    tracing::error!("Failed to publish agent_events via Mesh (in-process): {}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Error running task from Mesh (in-process): {}", e);
                    }
                }
            });
        }
    });

    match mesh_client.subscribe_tasks(handler).await {
        Ok(cancel) => {
            tracing::info!("Subscribed to Mesh channel for tasks (in-process)");
            Box::leak(cancel);
        }
        Err(e) => {
            tracing::error!("Failed to subscribe to mesh tasks (in-process): {}", e);
            return Err(e.into());
        }
    };

    tokio::spawn(async move {
        if let Err(e) = Server::builder()
            .add_service(proto::agent_service_server::AgentServiceServer::new(service::SharedAgentService(svc)))
            .serve(addr)
            .await
        {
            tracing::error!("In-process Builtin Agent Server Error: {}", e);
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;


    #[tokio::test]
    async fn test_start_builtin_agent_spawns_successfully() {
        let transport = std::sync::Arc::new(crate::mesh::transport::MemoryTransport::new());
        let mesh_client = std::sync::Arc::new(crate::mesh::TeammateMeshClient::new(transport));

        let res = start_builtin_agent("127.0.0.1:0", "test-agent-1", mesh_client).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_start_builtin_agent_handles_messages() {
        use crate::mesh::TeammateMesh;
        let transport = std::sync::Arc::new(crate::mesh::transport::MemoryTransport::new());
        let mesh_client = std::sync::Arc::new(crate::mesh::TeammateMeshClient::new(transport));

        let res = start_builtin_agent("127.0.0.1:0", "test-agent-2", mesh_client.clone()).await;
        assert!(res.is_ok());

        // Create a dummy RunTaskRequest
        let req = proto::RunTaskRequest {
            task_id: "test-task-123".to_string(),
            task: "dummy task".to_string(),
            model: "".to_string(),
            llm_provider: "".to_string(),
            llm_endpoint: "".to_string(),
            system_prompt: "".to_string(),
            max_tokens: 0,
            temperature: 0.0,
            max_context_messages: 0,
            runtime_config: None,
            toolset_config: None,
            department: "".to_string(),
        };

        let mut payload = Vec::new();
        prost::Message::encode(&req, &mut payload).unwrap();

        // Publish to tasks topic to trigger the closure
        let pub_res = mesh_client.publish_task(payload).await;
        assert!(pub_res.is_ok());

        // Wait a bit to let the closure process the message
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}
