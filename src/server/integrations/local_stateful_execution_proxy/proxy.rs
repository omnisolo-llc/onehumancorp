use std::sync::Arc;
use tokio::sync::Mutex;
use server_ohc::proto::mcp_proxy::{
    mcp_reverse_tunnel_service_client::McpReverseTunnelServiceClient,
    ProxyToServer, ServerToProxy, RegisterProxyRequest, InvokeCommandResponse,
    proxy_to_server::Payload as ProxyPayload,
    server_to_proxy::Payload as ServerPayload,
};
use tonic::transport::Channel;
use tokio::process::Command;

pub struct ProxyClient {
    pub server_address: String,
    pub spiffe_id: String,
    pub supported_tools: Vec<String>,
}

impl ProxyClient {
    pub fn new(server_address: String, spiffe_id: String, supported_tools: Vec<String>) -> Self {
        Self {
            server_address,
            spiffe_id,
            supported_tools,
        }
    }

    pub async fn connect_and_serve(&self, mut client: McpReverseTunnelServiceClient<Channel>) -> Result<(), String> {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let out_stream = tokio_stream::wrappers::ReceiverStream::new(rx);

        let mut in_stream = match client.establish_tunnel(out_stream).await {
            Ok(s) => s.into_inner(),
            Err(e) => return Err(format!("Failed to establish tunnel: {}", e)),
        };

        // Send registration
        let reg_msg = ProxyToServer {
            request_id: "init".to_string(),
            payload: Some(ProxyPayload::Register(RegisterProxyRequest {
                spiffe_id: self.spiffe_id.clone(),
                supported_tools: self.supported_tools.clone(),
            })),
        };

        if tx.send(reg_msg).await.is_err() {
            return Err("Failed to send registration".to_string());
        }

        while let match in_stream.message().await {
            Ok(Some(msg)) => Some(msg),
            Ok(None) => None,
            Err(e) => return Err(format!("Error receiving from stream: {}", e)),
        } {
            let msg = msg.unwrap();

            if let Some(ServerPayload::InvokeRequest(invoke_req)) = msg.payload {
                if invoke_req.tool_id == "shell" {
                    let (out, exec_err) = self.execute_shell_command(&invoke_req.params).await;

                    let resp = InvokeCommandResponse {
                        success: exec_err.is_none(),
                        result: out,
                        error_details: exec_err.unwrap_or_default(),
                    };

                    let resp_msg = ProxyToServer {
                        request_id: msg.request_id,
                        payload: Some(ProxyPayload::InvokeResponse(resp)),
                    };

                    if tx.send(resp_msg).await.is_err() {
                        return Err("Failed to send response".to_string());
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn execute_shell_command(&self, cmd: &str) -> (String, Option<String>) {
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .await;

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                if out.status.success() {
                    (stdout, None)
                } else {
                    (stdout, Some(stderr))
                }
            },
            Err(e) => {
                ("".to_string(), Some(e.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_shell_command_success() {
        let client = ProxyClient::new("".to_string(), "".to_string(), vec![]);
        let (out, err) = client.execute_shell_command("echo 'hello'").await;
        assert!(err.is_none());
        assert_eq!(out, "hello\n");
    }

    #[tokio::test]
    async fn test_execute_shell_command_error() {
        let client = ProxyClient::new("".to_string(), "".to_string(), vec![]);
        let (_, err) = client.execute_shell_command("non_existent_command_123").await;
        assert!(err.is_some());
    }
}
