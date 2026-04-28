use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct LocalProxyClient {
    server_url: String,
    session_id: String,
}

impl LocalProxyClient {
    pub fn new(server_url: &str, session_id: &str) -> Self {
        LocalProxyClient {
            server_url: server_url.to_string(),
            session_id: session_id.to_string(),
        }
    }

    pub async fn connect(&self) -> Result<(), String> {
        let s_id = self.session_id.clone();
        crate::record_telemetry(move || {
            println!("Telemetry: MCP Local Proxy Client connecting for session={}", s_id);
        });
        Ok(())
    }

    pub async fn forward_request(&self, request: &str) -> Result<String, String> {
        // Mock forwarding request to local MCP server
        Ok(format!("Forwarded: {}", request))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_local_proxy_client() {
        let client = LocalProxyClient::new("http://localhost:8080", "test-session");

        assert!(client.connect().await.is_ok());

        let req = "test-request";
        let res = client.forward_request(req).await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), format!("Forwarded: {}", req));
    }
}
