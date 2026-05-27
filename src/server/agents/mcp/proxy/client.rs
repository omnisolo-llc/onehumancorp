use ::server_ohc::mcp_proxy::mcp_reverse_tunnel_service_client::McpReverseTunnelServiceClient;
use ::server_ohc::mcp_proxy::{ServerToProxy, ProxyToServer, RegisterProxyRequest, proxy_to_server};
use tonic::transport::Channel;
use tonic::Request;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use std::process::Stdio;
use tracing::{info, warn, error};
use super::blob::{create_blob_provider, BlobProvider};
use std::sync::Arc;

use crate::orchestration::sandbox::{OHCSandboxManager, SandboxConfig};
use crate::orchestration::local_sandbox::LocalSandbox;


pub struct HybridContextTool;

impl HybridContextTool {
    pub async fn execute(pool: &sqlx::PgPool, params: &str) -> (bool, String, String) {
        let payload = serde_json::from_str::<serde_json::Value>(params).unwrap_or(serde_json::json!({}));
        let res = ::server_telemetry::buffer_metric(
            pool,
            "hybrid_ui_context",
            "event",
            1.0,
            payload
        ).await;

        match res {
            Ok(_) => (true, "{\"status\":\"success\"}".to_string(), "".to_string()),
            Err(e) => (false, "".to_string(), e.to_string()),
        }
    }
}

pub struct LocalProxyClient {
    client: McpReverseTunnelServiceClient<Channel>,
    spiffe_id: String,
    blob_provider: Arc<dyn BlobProvider>,
}

impl LocalProxyClient {
    pub async fn new(endpoint_url: String, spiffe_id: String) -> Self {
        let channel = Channel::from_shared(endpoint_url).unwrap()
            .connect()
            .await
            .unwrap();

        Self {
            client: McpReverseTunnelServiceClient::new(channel),
            spiffe_id,
            blob_provider: create_blob_provider(),
        }
    }

    pub fn new_with_channel(client: McpReverseTunnelServiceClient<Channel>, spiffe_id: String) -> Self {
        Self {
            client,
            spiffe_id,
            blob_provider: create_blob_provider(),
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (tx, rx) = mpsc::channel(128);

        // Initial registration
        let reg = RegisterProxyRequest {
            spiffe_id: self.spiffe_id.clone(),
            supported_tools: vec!["shell".to_string(), "fs_read".to_string(), "fs_write".to_string(), "hybrid_context".to_string()],
        };
        let _ = tx.send(ProxyToServer {
            request_id: "init".to_string(),
            payload: Some(proxy_to_server::Payload::Register(reg)),
        }).await;

        let request_stream = ReceiverStream::new(rx);
        let mut response = self.client.establish_tunnel(Request::new(request_stream)).await?;
        let mut in_stream = response.into_inner();

        let tx_clone = tx.clone();
        let blob_provider = self.blob_provider.clone();
        tokio::spawn(async move {
            while let Ok(Some(msg)) = in_stream.message().await {
                if let Some(payload) = msg.payload {
                    match payload {
                        ::server_ohc::mcp_proxy::server_to_proxy::Payload::InvokeRequest(req) => {
                            info!("Received invoke request for tool: {}", req.tool_id);

                            let (success, result, error_details) = match req.tool_id.as_str() {
                                "shell" => {
                                    let config = SandboxConfig {
                                        deny_list_dirs: vec!["/root".to_string(), "/etc/shadow".to_string()],
                                        ..Default::default()
                                    };
                                    let sandbox = LocalSandbox::new(config, None);

                                    match sandbox.execute(&req.params).await {
                                        Ok((success, stdout, stderr)) => {
                                            if success {
                                                (true, stdout, "".to_string())
                                            } else {
                                                (false, "".to_string(), stderr)
                                            }
                                        }
                                        Err(e) => {
                                            let error_msg = format!("Sandbox Violation: {} ({})", e.reason, e.command);
                                            error!("{}", error_msg);

                                            let details = serde_json::json!({
                                                "reason": e.reason,
                                                "command": e.command,
                                            });

                                            let _ = ::server_telemetry::buffer_metric(
                                                &crate::db::get_pool(),
                                                "sandbox_violation_event",
                                                "counter",
                                                1.0,
                                                details
                                            ).await;

                                            (false, "".to_string(), error_msg)
                                        }
                                    }
                                }
                                "fs_read" => {
                                    let start = std::time::Instant::now();
                                    let res = match blob_provider.read_blob(&req.params).await {
                                        Ok(content) => (true, content, "".to_string()),
                                        Err(e) => (false, "".to_string(), e.to_string()),
                                    };
                                    ::server_telemetry::record_harness_db_io_latency("fs_read", start.elapsed().as_secs_f64());
                                    res
                                }
                                "fs_write" => {
                                    let parts: Vec<&str> = req.params.splitn(2, "||").collect();
                                    if parts.len() == 2 {
                                        let start = std::time::Instant::now();
                                        let res = match blob_provider.write_blob(parts[0], parts[1]).await {
                                            Ok(_) => (true, "Successfully wrote file".to_string(), "".to_string()),
                                            Err(e) => (false, "".to_string(), e.to_string()),
                                        };
                                        ::server_telemetry::record_harness_db_io_latency("fs_write", start.elapsed().as_secs_f64());
                                        res
                                    } else {
                                        (false, "".to_string(), "Invalid params for fs_write".to_string())
                                    }
                                }
                                "hybrid_context" => {
                                    HybridContextTool::execute(&crate::db::get_pool(), &req.params).await
                                }
                                _ => (false, "".to_string(), format!("Unknown tool: {}", req.tool_id)),
                            };

                            let _ = tx_clone.send(ProxyToServer {
                                request_id: msg.request_id,
                                payload: Some(proxy_to_server::Payload::InvokeResponse(::server_ohc::mcp_proxy::InvokeCommandResponse {
                                    success,
                                    result,
                                    error_details,
                                })),
                            }).await;
                        }
                    }
                }
            }
            info!("Tunnel stream closed by server.");
        });

        Ok(())
    }
}
