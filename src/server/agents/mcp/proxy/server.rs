use ::server_ohc::mcp_proxy::mcp_reverse_tunnel_service_server::McpReverseTunnelService;
use ::server_ohc::mcp_proxy::{ServerToProxy, ProxyToServer};
use tonic::{Request, Response, Status, Streaming};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tracing::info;
use std::sync::Arc;
use sqlx::PgPool;

pub struct ReverseTunnelServer {
    pub pool: Arc<PgPool>,
}

#[tonic::async_trait]
impl McpReverseTunnelService for ReverseTunnelServer {
    type EstablishTunnelStream = ReceiverStream<Result<ServerToProxy, Status>>;

    async fn establish_tunnel(
        &self,
        request: Request<Streaming<ProxyToServer>>,
    ) -> Result<Response<<ReverseTunnelServer as McpReverseTunnelService>::EstablishTunnelStream>, Status> {
        let mut in_stream = request.into_inner();

        let (_tx, rx) = mpsc::channel(128);
        let pool = self.pool.clone();

        tokio::spawn(async move {
            let mut active_spiffe_id: Option<String> = None;
            let tunnel_start = std::time::Instant::now();

            while let Ok(Some(msg)) = in_stream.message().await {
                if let Some(payload) = msg.payload {
                    match payload {
                        ::server_ohc::mcp_proxy::proxy_to_server::Payload::Register(reg) => {
                            info!("Registered local proxy with SPIFFE ID: {}", reg.spiffe_id);
                            active_spiffe_id = Some(reg.spiffe_id.clone());
                            ::server_telemetry::record_harness_init_latency(tunnel_start.elapsed().as_secs_f64());
                            let _ = ::server_telemetry::record_mcp_proxy_connections_active(&pool, &reg.spiffe_id, 1.0).await;
                        }
                        ::server_ohc::mcp_proxy::proxy_to_server::Payload::InvokeResponse(res) => {
                            info!("Received response for {}: success={}", msg.request_id, res.success);
                        }
                    }
                }
            }

            if let Some(spiffe_id) = active_spiffe_id {
                let _ = ::server_telemetry::record_mcp_proxy_connections_active(&pool, &spiffe_id, -1.0).await;
            }

            info!("Tunnel connection closed.");
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
