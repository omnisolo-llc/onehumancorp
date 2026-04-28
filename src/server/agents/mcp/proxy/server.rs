use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct ReverseTunnelServer {
    active_tunnels: Arc<RwLock<HashMap<String, String>>>,
}

impl ReverseTunnelServer {
    pub fn new() -> Self {
        ReverseTunnelServer {
            active_tunnels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_tunnel(&self, session_id: &str, connection_id: &str) -> Result<(), String> {
        let mut tunnels = self.active_tunnels.write().await;
        tunnels.insert(session_id.to_string(), connection_id.to_string());

        let s_id = session_id.to_string();
        crate::record_telemetry(move || {
            println!("Telemetry: MCP Reverse Tunnel registered for session={}", s_id);
        });

        Ok(())
    }

    pub async fn unregister_tunnel(&self, session_id: &str) -> Result<(), String> {
        let mut tunnels = self.active_tunnels.write().await;
        tunnels.remove(session_id);
        Ok(())
    }

    pub async fn get_tunnel(&self, session_id: &str) -> Option<String> {
        let tunnels = self.active_tunnels.read().await;
        tunnels.get(session_id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_reverse_tunnel_server() {
        let server = ReverseTunnelServer::new();
        let session_id = "test-session";
        let connection_id = "conn-123";

        assert!(server.register_tunnel(session_id, connection_id).await.is_ok());

        let tunnel = server.get_tunnel(session_id).await;
        assert_eq!(tunnel, Some(connection_id.to_string()));

        assert!(server.unregister_tunnel(session_id).await.is_ok());
        let tunnel = server.get_tunnel(session_id).await;
        assert_eq!(tunnel, None);
    }
}
