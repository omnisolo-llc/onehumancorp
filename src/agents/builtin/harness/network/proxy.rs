use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::task::JoinHandle;

pub struct NetworkBridgeProxy {
    pub socket_path: String,
    socat_process: Option<tokio::process::Child>,
    proxy_task: Option<JoinHandle<()>>,
}

impl NetworkBridgeProxy {
    pub async fn start(allowed_hosts: Vec<String>) -> Result<Self, String> {
        let uuid = uuid::Uuid::new_v4().to_string();
        let socket_path = format!("/tmp/ohc-agent-http-{}.sock", uuid);

        let listener = TcpListener::bind("127.0.0.1:0").await
            .map_err(|e| format!("Failed to bind TCP listener: {}", e))?;
        let proxy_port = listener.local_addr().unwrap().port();

        let allowed_hosts = Arc::new(allowed_hosts);

        let proxy_task = tokio::spawn(async move {
            loop {
                if let Ok((mut stream, _)) = listener.accept().await {
                    let allowed = allowed_hosts.clone();
                    tokio::spawn(async move {
                        let mut buf = [0u8; 1024];
                        // Read initial request
                        if let Ok(n) = stream.read(&mut buf).await {
                            if n == 0 { return; }

                            let req_str = String::from_utf8_lossy(&buf[..n]);

                            let is_connect = req_str.starts_with("CONNECT ");
                            let target_host;
                            let target_port;

                            if is_connect {
                                let parts: Vec<&str> = req_str.split_whitespace().collect();
                                if parts.len() < 2 { return; }
                                let target = parts[1];
                                let mut split = target.split(':');
                                target_host = split.next().unwrap_or("").to_string();
                                target_port = split.next().unwrap_or("443").parse::<u16>().unwrap_or(443);
                            } else {
                                // Regular HTTP
                                // Parse Host and Port
                                let mut split = req_str.lines().find(|l| l.to_lowercase().starts_with("host:")).unwrap_or("").split(':');
                                let _ = split.next(); // Skip "Host"
                                target_host = split.next().unwrap_or("").trim().to_string();
                                target_port = split.next().unwrap_or("80").trim().parse::<u16>().unwrap_or(80);
                            }

                            let mut is_allowed = false;
                            if !target_host.is_empty() {
                                if allowed.contains(&target_host) {
                                    is_allowed = true;
                                }
                            }

                            if is_allowed {
                                if is_connect {
                                    if let Ok(target_stream) = TcpStream::connect((target_host.as_str(), target_port)).await {
                                        let _ = stream.write_all(b"HTTP/1.1 200 Connection established\r\n\r\n").await;
                                        let (mut client_read, mut client_write) = stream.into_split();
                                        let (mut target_read, mut target_write) = target_stream.into_split();

                                        let client_to_target = tokio::io::copy(&mut client_read, &mut target_write);
                                        let target_to_client = tokio::io::copy(&mut target_read, &mut client_write);

                                        let _ = tokio::join!(client_to_target, target_to_client);
                                    } else {
                                        let _ = stream.write_all(b"HTTP/1.1 502 Bad Gateway\r\n\r\n").await;
                                    }
                                } else {
                                    if let Ok(mut target_stream) = TcpStream::connect((target_host.as_str(), target_port)).await {
                                        let _ = target_stream.write_all(&buf[..n]).await;

                                        let (mut client_read, mut client_write) = stream.into_split();
                                        let (mut target_read, mut target_write) = target_stream.into_split();

                                        let client_to_target = tokio::io::copy(&mut client_read, &mut target_write);
                                        let target_to_client = tokio::io::copy(&mut target_read, &mut client_write);

                                        let _ = tokio::join!(client_to_target, target_to_client);
                                    } else {
                                        let _ = stream.write_all(b"HTTP/1.1 502 Bad Gateway\r\n\r\n").await;
                                    }
                                }
                            } else {
                                // Blocked
                                let _ = stream.write_all(b"HTTP/1.1 403 Forbidden\r\n\r\n").await;
                            }
                        }
                    });
                }
            }
        });

        // Start socat
        let socat_args = [
            format!("UNIX-LISTEN:{},fork", socket_path),
            format!("TCP:127.0.0.1:{}", proxy_port)
        ];

        let socat_process = tokio::process::Command::new("socat")
            .args(&socat_args)
            .spawn()
            .map_err(|e| format!("Failed to spawn socat: {}", e))?;

        // Wait a bit for socat to bind
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        Ok(NetworkBridgeProxy {
            socket_path,
            socat_process: Some(socat_process),
            proxy_task: Some(proxy_task),
        })
    }
}

impl Drop for NetworkBridgeProxy {
    fn drop(&mut self) {
        if let Some(mut child) = self.socat_process.take() {
            let _ = child.start_kill();
        }
        if let Some(task) = self.proxy_task.take() {
            task.abort();
        }
        let _ = std::fs::remove_file(&self.socket_path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use std::time::Duration;

    #[tokio::test]
    async fn test_proxy_drops_invalid_host() {
        let proxy = NetworkBridgeProxy::start(vec!["allowed.com".to_string()]).await.unwrap();

        // Connect to socat socket
        let mut stream = tokio::net::UnixStream::connect(&proxy.socket_path).await.expect("socat socket");

        let req = "GET / HTTP/1.1\r\nHost: blocked.com\r\n\r\n";
        stream.write_all(req.as_bytes()).await.unwrap();

        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf).await.unwrap();
        let resp = String::from_utf8_lossy(&buf[..n]);

        assert!(resp.contains("403 Forbidden"));
    }

    #[tokio::test]
    async fn test_proxy_allows_valid_host() {
        // Start a mock target server
        let mock_target = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let target_port = mock_target.local_addr().unwrap().port();

        tokio::spawn(async move {
            if let Ok((mut stream, _)) = mock_target.accept().await {
                let mut buf = [0u8; 1024];
                let _ = stream.read(&mut buf).await;
                let _ = stream.write_all(b"HTTP/1.1 200 OK\r\n\r\nSuccess").await;
            }
        });

        let proxy = NetworkBridgeProxy::start(vec!["127.0.0.1".to_string(), "localhost".to_string()]).await.unwrap();

        let mut stream = tokio::net::UnixStream::connect(&proxy.socket_path).await.expect("socat socket");

        let req = format!("GET / HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n", target_port);
        stream.write_all(req.as_bytes()).await.unwrap();

        let mut buf = [0u8; 1024];
        // Give it a moment to connect and respond
        tokio::time::sleep(Duration::from_millis(50)).await;

        let n = stream.read(&mut buf).await.unwrap_or(0);
        let resp = String::from_utf8_lossy(&buf[..n]);

        assert!(resp.contains("200 OK"));
        assert!(resp.contains("Success"));
    }
}
