use super::permission::{BridgeTransport, PermissionRequest, AuthorizationResponse};
use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
// Workaround since url crate isn't directly configured, reqwest::Url is often available
use reqwest::Url;

pub struct WsBridgeTransport {
    pub url: String,
}

impl WsBridgeTransport {
    pub fn new(url: String) -> Self {
        WsBridgeTransport { url }
    }
}

#[async_trait]
impl BridgeTransport for WsBridgeTransport {
    async fn request_permission(&self, req: PermissionRequest) -> Result<AuthorizationResponse, String> {
        let _parsed_url = Url::parse(&self.url)
            .map_err(|e| format!("Invalid URL: {}", e))?;

        // connect_async expects something that can be converted to an IntoClientRequest.
        // We'll pass the URL string as it works with tokio-tungstenite.
        let (ws_stream, _) = connect_async(self.url.as_str())
            .await
            .map_err(|e| format!("Failed to connect: {}", e))?;

        let (mut write, mut read) = ws_stream.split();

        let json_payload = serde_json::to_string(&req)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        write.send(Message::Text(json_payload.into()))
            .await
            .map_err(|e| format!("Failed to send message: {}", e))?;

        if let Some(msg_result) = read.next().await {
            let msg = msg_result.map_err(|e| format!("Error receiving message: {}", e))?;
            if let Message::Text(text) = msg {
                let text_str = text.to_string();
                let resp: AuthorizationResponse = serde_json::from_str(&text_str)
                    .map_err(|e| format!("Failed to parse response: {}", e))?;
                return Ok(resp);
            } else {
                return Err("Unexpected message type received from server".to_string());
            }
        }

        Err("No response received from remote orchestrator".to_string())
    }
}

pub struct PermissionInterceptor {
    transport: Arc<dyn BridgeTransport>,
    pub session_id: String,
}

impl PermissionInterceptor {
    pub fn new(transport: Arc<dyn BridgeTransport>, session_id: String) -> Self {
        PermissionInterceptor { transport, session_id }
    }
}

#[async_trait]
impl ohc_builtin_agent::tools::runner::CommandInterceptor for PermissionInterceptor {
    async fn check_permission(&self, tool_name: &str, command: &str) -> Result<(), String> {
        let req = PermissionRequest {
            tool_name: tool_name.to_string(),
            command: command.to_string(),
            session_id: self.session_id.clone(),
        };

        let resp = self.transport.request_permission(req).await?;

        if resp.authorized {
            Ok(())
        } else {
            Err(resp.reason.unwrap_or_else(|| "Authorization denied".to_string()))
        }
    }
}
