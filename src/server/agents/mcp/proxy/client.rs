use super::blob::{BlobProvider, create_blob_provider};
use ::server_ohc::mcp_proxy::mcp_reverse_tunnel_service_client::McpReverseTunnelServiceClient;
use ::server_ohc::mcp_proxy::{
    ProxyToServer, RegisterProxyRequest, ServerToProxy, proxy_to_server,
};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Stdio;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::Request;
use tonic::transport::Channel;
use tracing::{error, info, warn};

use crate::orchestration::local_sandbox::LocalSandbox;
use crate::orchestration::sandbox::{OHCSandboxManager, SandboxConfig};

pub struct LocalProxyClient {
    client: McpReverseTunnelServiceClient<Channel>,
    spiffe_id: String,
    blob_provider: Arc<dyn BlobProvider>,
}

impl LocalProxyClient {
    pub async fn new(endpoint_url: String, spiffe_id: String) -> Self {
        let channel = Channel::from_shared(endpoint_url)
            .unwrap()
            .connect()
            .await
            .unwrap();

        Self {
            client: McpReverseTunnelServiceClient::new(channel),
            spiffe_id,
            blob_provider: create_blob_provider(),
        }
    }

    pub fn new_with_channel(
        client: McpReverseTunnelServiceClient<Channel>,
        spiffe_id: String,
    ) -> Self {
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
            supported_tools: vec![
                "shell".to_string(),
                "fs_read".to_string(),
                "fs_write".to_string(),
                "local_fs_sync".to_string(),
            ],
        };
        let _ = tx
            .send(ProxyToServer {
                request_id: "init".to_string(),
                payload: Some(proxy_to_server::Payload::Register(reg)),
            })
            .await;

        let request_stream = ReceiverStream::new(rx);
        let response = self
            .client
            .establish_tunnel(Request::new(request_stream))
            .await?;
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
                                        deny_list_dirs: vec![
                                            "/root".to_string(),
                                            "/etc/shadow".to_string(),
                                        ],
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
                                            let error_msg = format!(
                                                "Sandbox Violation: {} ({})",
                                                e.reason, e.command
                                            );
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
                                                details,
                                            )
                                            .await;

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
                                    ::server_telemetry::record_harness_db_io_latency(
                                        "fs_read",
                                        start.elapsed().as_secs_f64(),
                                    );
                                    res
                                }
                                "fs_write" => {
                                    let parts: Vec<&str> = req.params.splitn(2, "||").collect();
                                    if parts.len() == 2 {
                                        let start = std::time::Instant::now();
                                        let res = match blob_provider
                                            .write_blob(parts[0], parts[1])
                                            .await
                                        {
                                            Ok(_) => (
                                                true,
                                                "Successfully wrote file".to_string(),
                                                "".to_string(),
                                            ),
                                            Err(e) => (false, "".to_string(), e.to_string()),
                                        };
                                        ::server_telemetry::record_harness_db_io_latency(
                                            "fs_write",
                                            start.elapsed().as_secs_f64(),
                                        );
                                        res
                                    } else {
                                        (
                                            false,
                                            "".to_string(),
                                            "Invalid params for fs_write".to_string(),
                                        )
                                    }
                                }

                                "local_fs_sync" => {
                                    let tool = LocalFSSyncTool::new(
                                        std::env::var("HOME")
                                            .unwrap_or_else(|_| "/tmp".to_string()),
                                    );
                                    match tool.execute(&req.params).await {
                                        Ok((s, r, e)) => (s, r, e),
                                        Err(err) => (false, "".to_string(), err),
                                    }
                                }
                                _ => (
                                    false,
                                    "".to_string(),
                                    format!("Unknown tool: {}", req.tool_id),
                                ),
                            };

                            let _ = tx_clone
                                .send(ProxyToServer {
                                    request_id: msg.request_id,
                                    payload: Some(proxy_to_server::Payload::InvokeResponse(
                                        ::server_ohc::mcp_proxy::InvokeCommandResponse {
                                            success,
                                            result,
                                            error_details,
                                        },
                                    )),
                                })
                                .await;
                        }
                    }
                }
            }
            info!("Tunnel stream closed by server.");
        });

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct LocalFSSyncToolParams {
    pub action: String,
    pub path: String,
    pub content: Option<String>,
}

pub struct LocalFSSyncTool {
    base_dir: String,
}

impl LocalFSSyncTool {
    pub fn new(base_dir: String) -> Self {
        Self { base_dir }
    }

    pub async fn execute(&self, params_json: &str) -> Result<(bool, String, String), String> {
        let params: LocalFSSyncToolParams = match serde_json::from_str(params_json) {
            Ok(p) => p,
            Err(e) => return Ok((false, "".to_string(), format!("Invalid params: {}", e))),
        };

        if !params.path.starts_with(".agent-task/") {
            return Ok((
                false,
                "".to_string(),
                "Path must start with .agent-task/".to_string(),
            ));
        }
        if params.path.contains("..") {
            return Ok((false, "".to_string(), "Path traversal attempt".to_string()));
        }

        let full_path = Path::new(&self.base_dir).join(&params.path);

        match params.action.as_str() {
            "read" => match tokio::fs::read_to_string(&full_path).await {
                Ok(content) => {
                    let _ = ::server_telemetry::buffer_metric(
                        &crate::db::get_pool(),
                        "local_fs_sync_read",
                        "counter",
                        1.0,
                        serde_json::json!({"path": params.path}),
                    )
                    .await;
                    Ok((true, content, "".to_string()))
                }
                Err(e) => Ok((false, "".to_string(), e.to_string())),
            },
            "write" => {
                let content = params.content.unwrap_or_default();
                if let Some(parent) = full_path.parent() {
                    let _ = tokio::fs::create_dir_all(parent).await;
                }
                match tokio::fs::write(&full_path, content).await {
                    Ok(_) => {
                        let _ = ::server_telemetry::buffer_metric(
                            &crate::db::get_pool(),
                            "local_fs_sync_write",
                            "counter",
                            1.0,
                            serde_json::json!({"path": params.path}),
                        )
                        .await;
                        Ok((true, "Successfully wrote file".to_string(), "".to_string()))
                    }
                    Err(e) => Ok((false, "".to_string(), e.to_string())),
                }
            }
            "sync" => {
                let exists = full_path.exists();
                let _ = ::server_telemetry::buffer_metric(
                    &crate::db::get_pool(),
                    "local_fs_sync_sync",
                    "counter",
                    1.0,
                    serde_json::json!({"path": params.path}),
                )
                .await;
                if exists {
                    Ok((true, "Synced".to_string(), "".to_string()))
                } else {
                    Ok((false, "".to_string(), "File does not exist".to_string()))
                }
            }
            _ => Ok((false, "".to_string(), "Invalid action".to_string())),
        }
    }
}
