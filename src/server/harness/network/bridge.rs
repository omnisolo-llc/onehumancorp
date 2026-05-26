use std::collections::HashSet;
use std::sync::Arc;
use tokio::net::{TcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Child;
use tokio::process::Command;
use uuid::Uuid;
use std::process::Stdio;

pub struct NetworkBridgeProxy {
    #[allow(dead_code)]

    #[allow(dead_code)]
    socket_path: String,
    #[allow(dead_code)]
    socat_child: Child,
    proxy_task: tokio::task::JoinHandle<()>,
}

impl NetworkBridgeProxy {
    pub async fn new(blocked_domains: Vec<String>) -> Result<Self, String> {
        let listener = TcpListener::bind("127.0.0.1:0").await.map_err(|e| e.to_string())?;
        let proxy_port = listener.local_addr().map_err(|e| e.to_string())?.port();

        let socket_path = format!("/tmp/ohc-agent-http-{}.sock", Uuid::new_v4());

        // Spawn host socat
        let socat_child = Command::new("socat")
            .arg(format!("UNIX-LISTEN:{},fork", socket_path))
            .arg(format!("TCP:127.0.0.1:{}", proxy_port))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .kill_on_drop(true)
            .spawn()
            .map_err(|e| format!("Failed to spawn socat: {}", e))?;

        let blocked_domains_set: HashSet<String> = blocked_domains.into_iter().collect();
        let blocked = Arc::new(blocked_domains_set);

        let proxy_task = tokio::spawn(async move {
            loop {
                if let Ok((mut socket, _)) = listener.accept().await {
                    let blocked = Arc::clone(&blocked);
                    tokio::spawn(async move {
                        let mut buf = [0u8; 4096];
                        if let Ok(n) = socket.read(&mut buf).await {
                            if n == 0 { return; }
                            let request = String::from_utf8_lossy(&buf[..n]);
                            let mut drop_req = false;

                            // Simple extraction of Host from CONNECT or GET
                            if let Some(host_line) = request.lines().find(|l| l.to_lowercase().starts_with("host:")) {
                                let host = host_line[5..].trim().split(':').next().unwrap_or("");
                                if blocked.contains(host) {
                                    drop_req = true;
                                }
                            } else if request.starts_with("CONNECT ") {
                                let parts: Vec<&str> = request.split_whitespace().collect();
                                if parts.len() > 1 {
                                    let host = parts[1].split(':').next().unwrap_or("");
                                    if blocked.contains(host) {
                                        drop_req = true;
                                    }
                                }
                            }

                            if drop_req {
                                let _ = socket.write_all(b"HTTP/1.1 403 Forbidden\r\n\r\nBlocked").await;
                            } else {
                                // Extract destination host and port
                                let mut dest_host = String::new();
                                let mut is_connect = false;

                                if request.starts_with("CONNECT ") {
                                    is_connect = true;
                                    let parts: Vec<&str> = request.split_whitespace().collect();
                                    if parts.len() > 1 {
                                        dest_host = parts[1].to_string();
                                    }
                                } else if let Some(host_line) = request.lines().find(|l| l.to_lowercase().starts_with("host:")) {
                                    let h = host_line[5..].trim();
                                    if !h.contains(':') {
                                        dest_host = format!("{}:80", h);
                                    } else {
                                        dest_host = h.to_string();
                                    }
                                }

                                if dest_host.is_empty() {
                                    let _ = socket.write_all(b"HTTP/1.1 400 Bad Request\r\n\r\n").await;
                                    return;
                                }

                                if let Ok(mut server_socket) = tokio::net::TcpStream::connect(&dest_host).await {
                                    if is_connect {
                                        let _ = socket.write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n").await;
                                    } else {
                                        let _ = server_socket.write_all(&buf[..n]).await;
                                    }

                                    let (mut client_read, mut client_write) = socket.split();
                                    let (mut server_read, mut server_write) = server_socket.split();

                                    let client_to_server = tokio::io::copy(&mut client_read, &mut server_write);
                                    let server_to_client = tokio::io::copy(&mut server_read, &mut client_write);

                                    let _ = tokio::try_join!(client_to_server, server_to_client);
                                } else {
                                    let _ = socket.write_all(b"HTTP/1.1 502 Bad Gateway\r\n\r\n").await;
                                }
                            }
                        }
                    });
                }
            }
        });

        Ok(Self {
            socket_path,
            socat_child,
            proxy_task,
        })
    }

    pub fn socket_path(&self) -> &str {
        &self.socket_path
    }
}

impl Drop for NetworkBridgeProxy {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.socket_path);
        self.proxy_task.abort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::UnixStream;
    use std::time::Duration;

    #[tokio::test]
    async fn test_proxy_allowed_and_blocked() {
        let blocked = vec!["evil.com".to_string(), "bad.org".to_string()];
        let proxy = NetworkBridgeProxy::new(blocked).await.expect("Failed to start proxy");

        // Give socat a moment to start
        tokio::time::sleep(Duration::from_millis(500)).await;

        let socket_path = proxy.socket_path();

        // Test allowed request
        // Because "good.com" might not be accessible from the sandbox or resolves slowly,
        // we connect to a local port or simply check if the connection gets established
        // For testing we will just check if we get 502 (meaning it tried to proxy, rather than 403 Forbidden)
        let mut stream = UnixStream::connect(socket_path).await.expect("Failed to connect to proxy socket");
        stream.write_all(b"CONNECT 127.0.0.1:443 HTTP/1.1\r\nHost: 127.0.0.1:443\r\n\r\n").await.unwrap();
        let mut response = vec![0; 1024];
        let n = stream.read(&mut response).await.unwrap();
        let resp_str = String::from_utf8_lossy(&response[..n]);
        assert!(resp_str.contains("502 Bad Gateway") || resp_str.contains("200 Connection Established"));

        // Test blocked request
        let mut stream2 = UnixStream::connect(socket_path).await.expect("Failed to connect to proxy socket");
        stream2.write_all(b"CONNECT evil.com:443 HTTP/1.1\r\nHost: evil.com:443\r\n\r\n").await.unwrap();
        let mut response2 = vec![0; 1024];
        let n2 = stream2.read(&mut response2).await.unwrap();
        let resp_str2 = String::from_utf8_lossy(&response2[..n2]);
        assert!(resp_str2.contains("403 Forbidden"));
    }
}
