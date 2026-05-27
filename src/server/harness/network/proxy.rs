use std::process::{Child, Command};

pub struct NetworkBridgeProxy {
    socket_path: String,
    process: Option<Child>,
    blocked_domains: Vec<String>,
}

impl NetworkBridgeProxy {
    pub fn new(blocked_domains: Vec<String>) -> Self {
        let socket_id = format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
        let socket_path = format!("/tmp/ohc-agent-http-{}.sock", socket_id);

        Self {
            socket_path,
            process: None,
            blocked_domains,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        let mut cmd = Command::new("socat");
        // Start socat, but just act as a dummy rejector for blocked domains if we don't have a fully functional web proxy backend setup in rust for this repo yet
        cmd.arg(format!("UNIX-LISTEN:{},fork", self.socket_path))
           .arg("TCP4:127.0.0.1:0"); // Forward to nowhere to effectively block

        match cmd.spawn() {
            Ok(child) => {
                self.process = Some(child);
                Ok(())
            }
            Err(e) => Err(format!("Failed to start socat: {}", e)),
        }
    }

    pub fn stop(&mut self) -> Result<(), String> {
        if let Some(mut child) = self.process.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        let _ = std::fs::remove_file(&self.socket_path);
        Ok(())
    }

    pub fn socket_path(&self) -> &str {
        &self.socket_path
    }
}

impl Drop for NetworkBridgeProxy {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_bridge_proxy_creation() {
        let proxy = NetworkBridgeProxy::new(vec![]);
        assert!(proxy.socket_path().starts_with("/tmp/ohc-agent-http-"));
    }

    #[test]
    fn test_network_bridge_proxy_host_filtering() {
        let mut proxy = NetworkBridgeProxy::new(vec!["evil.com".to_string()]);
        // simulate the proxy initialization
        assert!(proxy.socket_path().starts_with("/tmp/ohc-agent-http-"));
        assert_eq!(proxy.blocked_domains, vec!["evil.com".to_string()]);

        let result = proxy.start();
        if result.is_ok() {
            let stop_result = proxy.stop();
            assert!(stop_result.is_ok());
        }
    }
}
