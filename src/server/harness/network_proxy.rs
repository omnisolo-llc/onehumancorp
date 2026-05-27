use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use ::server_telemetry::record_bubblewrap_violation;
use tokio::sync::watch;

pub struct NetworkProxy {
    blocked_domains: Vec<String>,
}

impl NetworkProxy {
    pub fn new(blocked_domains: Vec<String>) -> Self {
        Self {
            blocked_domains,
        }
    }

    pub async fn run(self, listener: TcpListener, mut shutdown_rx: watch::Receiver<bool>) {
        let blocked_domains = Arc::new(self.blocked_domains);
        loop {
            tokio::select! {
                res = listener.accept() => {
                    match res {
                        Ok((mut stream, _)) => {
                            let blocked = blocked_domains.clone();
                            tokio::spawn(async move {
                                if let Err(e) = Self::handle_connection(&mut stream, blocked).await {
                                    eprintln!("Proxy error: {}", e);
                                }
                            });
                        }
                        Err(e) => {
                            eprintln!("Failed to accept proxy connection: {}", e);
                        }
                    }
                }
                _ = shutdown_rx.changed() => {
                    if *shutdown_rx.borrow() {
                        break;
                    }
                }
            }
        }
    }

    async fn handle_connection(client_stream: &mut TcpStream, blocked_domains: Arc<Vec<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = [0; 4096];
        let bytes_read = client_stream.read(&mut buffer).await?;
        if bytes_read == 0 {
            return Ok(());
        }

        let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);
        let mut lines = request_str.lines();
        let first_line = lines.next().unwrap_or("");

        let mut target_host = String::new();
        let mut target_port = 80;

        if first_line.starts_with("CONNECT ") {
            let parts: Vec<&str> = first_line.split_whitespace().collect();
            if parts.len() >= 2 {
                let host_port: Vec<&str> = parts[1].split(':').collect();
                target_host = host_port[0].to_string();
                if host_port.len() > 1 {
                    target_port = host_port[1].parse().unwrap_or(443);
                } else {
                    target_port = 443;
                }
            }
        } else {
            for line in lines {
                if line.to_lowercase().starts_with("host: ") {
                    let host_val = line[6..].trim();
                    let host_port: Vec<&str> = host_val.split(':').collect();
                    target_host = host_port[0].to_string();
                    if host_port.len() > 1 {
                        target_port = host_port[1].parse().unwrap_or(80);
                    }
                    break;
                }
            }
        }

        if target_host.is_empty() {
            return Err("Could not determine target host".into());
        }

        // Logic fix: correctly check blocked domains precisely
        let is_blocked = blocked_domains.iter().any(|d| {
            target_host == *d || target_host.ends_with(&format!(".{}", d))
        });

        if is_blocked {
            record_bubblewrap_violation("unknown_agent", "unknown_task", "network_access_denied");
            let _ = client_stream.write_all(b"HTTP/1.1 403 Forbidden

").await;
            return Ok(());
        }

        let mut server_stream = TcpStream::connect(format!("{}:{}", target_host, target_port)).await?;

        if first_line.starts_with("CONNECT ") {
            client_stream.write_all(b"HTTP/1.1 200 Connection Established

").await?;
        } else {
            server_stream.write_all(&buffer[..bytes_read]).await?;
        }

        let (mut client_read, mut client_write) = client_stream.split();
        let (mut server_read, mut server_write) = server_stream.split();

        let client_to_server = tokio::io::copy(&mut client_read, &mut server_write);
        let server_to_client = tokio::io::copy(&mut server_read, &mut client_write);

        let _ = tokio::try_join!(client_to_server, server_to_client);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    async fn start_mock_server() -> SocketAddr {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            if let Ok((mut stream, _)) = listener.accept().await {
                let mut buf = [0; 1024];
                let _ = stream.read(&mut buf).await;
                let _ = stream.write_all(b"HTTP/1.1 200 OK

Hello").await;
            }
        });
        addr
    }

    #[tokio::test]
    async fn test_proxy_allowed_domain() {
        let server_addr = start_mock_server().await;
        let proxy_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let proxy_addr = proxy_listener.local_addr().unwrap();

        let proxy = NetworkProxy::new(vec!["example.com".to_string()]); // Example is blocked, local is allowed
        let (tx, rx) = watch::channel(false);
        tokio::spawn(proxy.run(proxy_listener, rx));

        let mut client = TcpStream::connect(proxy_addr).await.unwrap();
        let request = format!("GET / HTTP/1.1
Host: 127.0.0.1:{}

", server_addr.port());
        client.write_all(request.as_bytes()).await.unwrap();

        let mut response = String::new();
        client.read_to_string(&mut response).await.unwrap();
        assert!(response.contains("200 OK"));
        assert!(response.contains("Hello"));
        tx.send(true).unwrap();
    }

    #[tokio::test]
    async fn test_proxy_blocked_domain() {
        let server_addr = start_mock_server().await;
        let proxy_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let proxy_addr = proxy_listener.local_addr().unwrap();

        let proxy = NetworkProxy::new(vec!["127.0.0.1".to_string()]); // 127.0.0.1 is blocked
        let (tx, rx) = watch::channel(false);
        tokio::spawn(proxy.run(proxy_listener, rx));

        let mut client = TcpStream::connect(proxy_addr).await.unwrap();
        let request = format!("GET / HTTP/1.1
Host: 127.0.0.1:{}

", server_addr.port());
        client.write_all(request.as_bytes()).await.unwrap();

        let mut response = String::new();
        client.read_to_string(&mut response).await.unwrap();
        assert!(response.contains("403 Forbidden"));
        tx.send(true).unwrap();
    }
}
