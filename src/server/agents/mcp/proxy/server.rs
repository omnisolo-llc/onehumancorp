use ::server_ohc::mcp_proxy::mcp_reverse_tunnel_service_server::McpReverseTunnelService;
use ::server_ohc::mcp_proxy::{ServerToProxy, ProxyToServer};
use tonic::{Request, Response, Status, Streaming};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tracing::info;
use std::sync::Arc;
use sqlx::PgPool;

#[derive(Clone)]
pub struct ReverseTunnelServer {
    pub pool: Arc<PgPool>,
    pub connections: Arc<dashmap::DashMap<String, mpsc::Sender<Result<ServerToProxy, Status>>>>,
}

impl ReverseTunnelServer {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool,
            connections: Arc::new(dashmap::DashMap::new()),
        }
    }

    pub async fn forward_webhook(&self, agent_id: &str, payload: Vec<u8>) -> Result<(), Status> {
        let sender = match self.connections.get(agent_id) {
            Some(s) => s.clone(),
            None => return Err(Status::not_found("Agent not connected")),
        };

        let req = ::server_ohc::mcp_proxy::InvokeCommandRequest {
            tool_id: "webhook_forward".to_string(),
            params: String::from_utf8_lossy(&payload).into_owned(),
        };

        let msg = ServerToProxy {
            request_id: uuid::Uuid::new_v4().to_string(),
            payload: Some(::server_ohc::mcp_proxy::server_to_proxy::Payload::InvokeRequest(req)),
        };

        match sender.send(Ok(msg)).await {
            Ok(_) => Ok(()),
            Err(_) => Err(Status::internal("Failed to send webhook")),
        }
    }
}

#[tonic::async_trait]
impl McpReverseTunnelService for ReverseTunnelServer {
    type EstablishTunnelStream = ReceiverStream<Result<ServerToProxy, Status>>;

    async fn establish_tunnel(
        &self,
        request: Request<Streaming<ProxyToServer>>,
    ) -> Result<Response<<ReverseTunnelServer as McpReverseTunnelService>::EstablishTunnelStream>, Status> {
        let mut in_stream = request.into_inner();

        let (tx, rx) = mpsc::channel(128);
        let pool = self.pool.clone();
        let connections = self.connections.clone();

        tokio::spawn(async move {
            let mut active_spiffe_id: Option<String> = None;
            let mut active_agent_id: Option<String> = None;
            let tunnel_start = std::time::Instant::now();

            while let Ok(Some(msg)) = in_stream.message().await {
                if let Some(payload) = msg.payload {
                    match payload {
                        ::server_ohc::mcp_proxy::proxy_to_server::Payload::Register(reg) => {
                            info!("Registered local proxy with SPIFFE ID: {}", reg.spiffe_id);

                            // In a full implementation we would enforce SPIFFE identity here,
                            // but currently the proxy unauthenticated connection is enough
                            active_spiffe_id = Some(reg.spiffe_id.clone());

                            let agent_id = reg.spiffe_id.split('/').last().unwrap_or("").to_string();
                            active_agent_id = Some(agent_id.clone());
                            connections.insert(agent_id, tx.clone());

                            ::server_telemetry::record_harness_init_latency(tunnel_start.elapsed().as_secs_f64());
                            let _ = ::server_telemetry::record_mcp_proxy_connections_active(&pool, &reg.spiffe_id, 1.0).await;
                        }
                        ::server_ohc::mcp_proxy::proxy_to_server::Payload::InvokeResponse(res) => {
                            info!("Received response for {}: success={}", msg.request_id, res.success);
                        }
                    }
                }
            }

            if let Some(agent_id) = active_agent_id {
                connections.remove(&agent_id);
            }

            if let Some(spiffe_id) = active_spiffe_id {
                let _ = ::server_telemetry::record_mcp_proxy_connections_active(&pool, &spiffe_id, -1.0).await;
            }

            info!("Tunnel connection closed.");
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
