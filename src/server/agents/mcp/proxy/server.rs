use ::server_omnisolo::mcp_proxy::mcp_reverse_tunnel_service_server::McpReverseTunnelService;
use ::server_omnisolo::mcp_proxy::{ProxyToServer, ServerToProxy, proxy_to_server};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};
use tracing::info;

pub(super) struct TunnelConnection {
    generation: uuid::Uuid,
    sender: mpsc::Sender<Result<ServerToProxy, Status>>,
}

#[derive(Clone)]
pub struct ReverseTunnelServer {
    pub pool: Arc<PgPool>,
    pub(super) connections: Arc<dashmap::DashMap<(String, String), TunnelConnection>>,
}

impl ReverseTunnelServer {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool,
            connections: Arc::new(dashmap::DashMap::new()),
        }
    }

    pub async fn forward_webhook(
        &self,
        org_id: &str,
        agent_id: &str,
        payload: Vec<u8>,
    ) -> Result<(), Status> {
        let key = (org_id.to_owned(), agent_id.to_owned());
        let sender = match self.connections.get(&key) {
            Some(connection) => connection.sender.clone(),
            None => return Err(Status::not_found("Agent not connected")),
        };

        let req = ::server_omnisolo::mcp_proxy::InvokeCommandRequest {
            tool_id: "webhook_forward".to_string(),
            params: String::from_utf8_lossy(&payload).into_owned(),
        };

        let msg = ServerToProxy {
            request_id: uuid::Uuid::new_v4().to_string(),
            payload: Some(
                ::server_omnisolo::mcp_proxy::server_to_proxy::Payload::InvokeRequest(req),
            ),
        };

        match sender.send(Ok(msg)).await {
            Ok(_) => Ok(()),
            Err(_) => Err(Status::internal("Failed to queue webhook")),
        }
    }
}

#[tonic::async_trait]
impl McpReverseTunnelService for ReverseTunnelServer {
    type EstablishTunnelStream = ReceiverStream<Result<ServerToProxy, Status>>;

    async fn establish_tunnel(
        &self,
        request: Request<Streaming<ProxyToServer>>,
    ) -> Result<
        Response<<ReverseTunnelServer as McpReverseTunnelService>::EstablishTunnelStream>,
        Status,
    > {
        let auth = request
            .extensions()
            .get::<::server_auth::AuthInfo>()
            .cloned()
            .ok_or_else(|| Status::unauthenticated("Verified proxy identity is required"))?;
        let tunnel_start = std::time::Instant::now();
        let mut in_stream = request.into_inner();
        let first = in_stream
            .message()
            .await?
            .ok_or_else(|| Status::failed_precondition("Initial proxy registration is required"))?;
        let Some(proxy_to_server::Payload::Register(registration)) = first.payload else {
            return Err(Status::failed_precondition(
                "First proxy message must be a registration",
            ));
        };
        if registration.spiffe_id != auth.spiffe_id {
            return Err(Status::permission_denied(
                "Registration does not match the verified proxy identity",
            ));
        }

        // Validate and register before returning successful RPC headers: the
        // existing protocol has no separate registration acknowledgment.
        let key = (auth.org_id, auth.agent_id);
        let generation = uuid::Uuid::new_v4();
        let (tx, rx) = mpsc::channel(128);
        self.connections.insert(
            key.clone(),
            TunnelConnection {
                generation,
                sender: tx.clone(),
            },
        );
        let pool = self.pool.clone();
        let connections = self.connections.clone();
        info!("Registered local proxy with SPIFFE ID: {}", auth.spiffe_id);
        ::server_telemetry::record_harness_init_latency(tunnel_start.elapsed().as_secs_f64());

        tokio::spawn(async move {
            let _ = ::server_telemetry::record_mcp_proxy_connections_active(
                &pool,
                &auth.spiffe_id,
                1.0,
            )
            .await;
            while let Ok(Some(msg)) = in_stream.message().await {
                if let Some(payload) = msg.payload {
                    match payload {
                        proxy_to_server::Payload::Register(_) => {
                            let _ = tx
                                .send(Err(Status::failed_precondition(
                                    "Proxy connection is already registered",
                                )))
                                .await;
                            break;
                        }
                        proxy_to_server::Payload::InvokeResponse(res) => {
                            info!(
                                "Received response for {}: success={}",
                                msg.request_id, res.success
                            );
                        }
                    }
                }
            }

            // A replaced connection may disconnect later. Compare and remove
            // under the same map lock so it cannot evict its replacement.
            connections.remove_if(&key, |_, current| current.generation == generation);
            let _ = ::server_telemetry::record_mcp_proxy_connections_active(
                &pool,
                &auth.spiffe_id,
                -1.0,
            )
            .await;
            info!("Tunnel connection closed.");
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
