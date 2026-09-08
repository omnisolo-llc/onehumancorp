use ::server_telemetry::record_bubblewrap_violation;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::oneshot;

pub struct NetworkProxy {
    allowed_domains: Arc<Vec<String>>,
    agent_id: String,
    task_id: String,
}

impl NetworkProxy {
    pub fn new(allowed_domains: Vec<String>, agent_id: String, task_id: String) -> Self {
        Self {
            allowed_domains: Arc::new(allowed_domains),
            agent_id,
            task_id,
        }
    }

    pub async fn start(&self, port: u16) -> Result<(u16, oneshot::Sender<()>), std::io::Error> {
        let listener = TcpListener::bind(("127.0.0.1", port)).await?;
        let actual_port = listener.local_addr()?.port();
        let allowed_domains = self.allowed_domains.clone();
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel::<()>();

        let agent_id = self.agent_id.clone();
        let task_id = self.task_id.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    accept_res = listener.accept() => {
                        match accept_res {
                            Ok((stream, _)) => {
                                let domains = allowed_domains.clone();
                                let a_id = agent_id.clone();
                                let t_id = task_id.clone();
                                tokio::spawn(async move {
                                    if let Err(e) = handle_connection(stream, domains, a_id, t_id).await {
                                        tracing::debug!("Proxy connection error: {}", e);
                                    }
                                });
                            }
                            Err(e) => tracing::error!("Failed to accept connection: {}", e),
                        }
                    }
                    _ = &mut shutdown_rx => {
                        tracing::debug!("Shutting down network proxy on port {}", actual_port);
                        break;
                    }
                }
            }
        });

        Ok((actual_port, shutdown_tx))
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    allowed_domains: Arc<Vec<String>>,
    agent_id: String,
    task_id: String,
) -> Result<(), String> {
    let mut buffer = [0; 4096];
    let bytes_read = stream.read(&mut buffer).await.map_err(|e| e.to_string())?;

    if bytes_read == 0 {
        return Ok(());
    }

    let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);

    let mut host_with_port = "";
    let mut host_without_port = "";

    if request_str.starts_with("CONNECT") {
        if let Some(hp) = request_str.split_whitespace().nth(1) {
            host_with_port = hp;
            host_without_port = hp.split(':').next().unwrap_or(hp);
        }
    } else {
        for line in request_str.lines() {
            if line.to_lowercase().starts_with("host:") {
                host_with_port = line[5..].trim();
                host_without_port = host_with_port.split(':').next().unwrap_or(host_with_port);
                break;
            }
        }
    }

    if host_with_port.is_empty() {
        let _ = stream.write_all(b"HTTP/1.1 400 Bad Request\r\n\r\n").await;
        return Err("Missing Host header or CONNECT target".to_string());
    }

    let is_allowed = allowed_domains
        .iter()
        .any(|d| host_without_port == d || host_without_port.ends_with(&format!(".{}", d)));

    if !is_allowed {
        record_bubblewrap_violation(&agent_id, &task_id, "network_violation_denied");
        let response = "HTTP/1.1 403 Forbidden\r\n\r\nDenied by sandbox proxy policy";
        let _ = stream.write_all(response.as_bytes()).await;
        return Err(format!("Domain {} is blocked", host_without_port));
    }

    // Proxy forwarding logic using hyper or basic TCP bridging for CONNECT
    if request_str.starts_with("CONNECT") {
        let target_port = if host_with_port.contains(':') {
            host_with_port.to_string()
        } else {
            format!("{}:443", host_with_port)
        };
        if let Ok(mut target_stream) = TcpStream::connect(&target_port).await {
            let _ = stream
                .write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n")
                .await;

            let (mut ri, mut wi) = stream.split();
            let (mut ro, mut wo) = target_stream.split();

            let client_to_server = tokio::io::copy(&mut ri, &mut wo);
            let server_to_client = tokio::io::copy(&mut ro, &mut wi);

            let _ = tokio::try_join!(client_to_server, server_to_client);
        } else {
            let _ = stream.write_all(b"HTTP/1.1 502 Bad Gateway\r\n\r\n").await;
        }
    } else {
        let target_port = if host_with_port.contains(':') {
            host_with_port.to_string()
        } else {
            format!("{}:80", host_with_port)
        };
        if let Ok(mut target_stream) = TcpStream::connect(&target_port).await {
            let _ = target_stream.write_all(&buffer[..bytes_read]).await;

            let (mut ri, mut wi) = stream.split();
            let (mut ro, mut wo) = target_stream.split();

            let client_to_server = tokio::io::copy(&mut ri, &mut wo);
            let server_to_client = tokio::io::copy(&mut ro, &mut wi);

            let _ = tokio::try_join!(client_to_server, server_to_client);
        } else {
            let _ = stream.write_all(b"HTTP/1.1 502 Bad Gateway\r\n\r\n").await;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_proxy_denied() {
        let proxy = NetworkProxy::new(
            vec!["example.com".to_string()],
            "test_agent".to_string(),
            "test_task".to_string(),
        );
        let (port, _shutdown) = proxy.start(0).await.unwrap();

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port))
            .await
            .unwrap();
        stream
            .write_all(b"GET / HTTP/1.1\r\nHost: evil.com\r\n\r\n")
            .await
            .unwrap();

        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).await.unwrap();
        let response = String::from_utf8_lossy(&buffer[..bytes_read]);

        assert!(response.contains("403 Forbidden"));
    }

    #[tokio::test]
    async fn test_proxy_allowed() {
        let proxy = NetworkProxy::new(
            vec!["example.com".to_string()],
            "test_agent".to_string(),
            "test_task".to_string(),
        );
        let (port, _shutdown) = proxy.start(0).await.unwrap();

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port))
            .await
            .unwrap();
        stream
            .write_all(b"CONNECT api.example.com:443 HTTP/1.1\r\n\r\n")
            .await
            .unwrap();

        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).await.unwrap();
        let response = String::from_utf8_lossy(&buffer[..bytes_read]);

        // It will likely return 502 because api.example.com may not be reachable or exist
        // or 200 if it is reachable. Both mean it passed the sandbox check.
        assert!(
            response.contains("200 Connection Established") || response.contains("502 Bad Gateway")
        );
    }
}
