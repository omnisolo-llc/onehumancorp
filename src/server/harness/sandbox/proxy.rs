use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::RwLock;

use super::manager::SandboxPolicy;

pub struct NetworkProxy {
    policy: Arc<RwLock<SandboxPolicy>>,
    listener_addr: SocketAddr,
}

impl NetworkProxy {
    pub fn new(policy: Arc<RwLock<SandboxPolicy>>, addr: SocketAddr) -> Self {
        Self {
            policy,
            listener_addr: addr,
        }
    }

    pub fn get_policy(&self) -> Arc<RwLock<SandboxPolicy>> {
        self.policy.clone()
    }

    pub async fn handle_connection(mut client_stream: TcpStream, policy: Arc<RwLock<SandboxPolicy>>) -> Result<(), std::io::Error> {
        let mut buffer = [0; 4096];
        let bytes_read = client_stream.read(&mut buffer).await?;
        if bytes_read == 0 {
            return Ok(());
        }

        if buffer[0] == 0x05 {
            // SOCKS5 Support
            let p = policy.read().await;

            let _ = client_stream.write_all(b"\x05\x00").await;

            let mut req_buf = [0; 256];
            let req_len = client_stream.read(&mut req_buf).await?;
            if req_len > 4 && req_buf[1] == 0x01 { // CONNECT
                // Extract domain from SOCKS request (simplistic)
                let host = if req_buf[3] == 0x03 { // Domain name
                    let len = req_buf[4] as usize;
                    if len + 5 <= req_len {
                        String::from_utf8_lossy(&req_buf[5..5+len]).to_string()
                    } else {
                        String::new()
                    }
                } else if req_buf[3] == 0x01 { // IPv4
                    format!("{}.{}.{}.{}", req_buf[4], req_buf[5], req_buf[6], req_buf[7])
                } else {
                    String::new()
                };

                let port = if req_buf[3] == 0x03 {
                    let len = req_buf[4] as usize;
                    if len + 7 <= req_len {
                        ((req_buf[5+len] as u16) << 8) | (req_buf[6+len] as u16)
                    } else {
                        80
                    }
                } else if req_buf[3] == 0x01 {
                    if req_len >= 10 {
                        ((req_buf[8] as u16) << 8) | (req_buf[9] as u16)
                    } else {
                        80
                    }
                } else {
                    80
                };

                if p.blocked_domains.iter().any(|d| host.contains(d)) {
                    let _ = client_stream.write_all(b"\x05\x02\x00\x01\x00\x00\x00\x00\x00\x00").await; // Connection not allowed by ruleset
                    return Ok(());
                }

                let target = format!("{}:{}", host, port);
                match tokio::net::TcpStream::connect(&target).await {
                    Ok(mut server_stream) => {
                        let _ = client_stream.write_all(b"\x05\x00\x00\x01\x00\x00\x00\x00\x00\x00").await;
                        let (mut client_read, mut client_write) = client_stream.into_split();
                        let (mut server_read, mut server_write) = server_stream.into_split();

                        let client_to_server = tokio::io::copy(&mut client_read, &mut server_write);
                        let server_to_client = tokio::io::copy(&mut server_read, &mut client_write);

                        let _ = tokio::try_join!(client_to_server, server_to_client);
                    },
                    Err(_) => {
                        let _ = client_stream.write_all(b"\x05\x04\x00\x01\x00\x00\x00\x00\x00\x00").await;
                    }
                }
            }
            return Ok(());
        }

        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        if request.starts_with("CONNECT ") {
            let parts: Vec<&str> = request.split_whitespace().collect();
            if parts.len() >= 2 {
                let target = parts[1];
                let host = target.split(':').next().unwrap_or(target);

                let p = policy.read().await;
                if p.blocked_domains.iter().any(|d| host.contains(d)) {
                    let _ = client_stream.write_all(b"HTTP/1.1 403 Forbidden\r\n\r\n").await;
                    return Ok(());
                }

                match tokio::net::TcpStream::connect(target).await {
                    Ok(mut server_stream) => {
                        let _ = client_stream.write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n").await;
                        let (mut client_read, mut client_write) = client_stream.into_split();
                        let (mut server_read, mut server_write) = server_stream.into_split();

                        let client_to_server = tokio::io::copy(&mut client_read, &mut server_write);
                        let server_to_client = tokio::io::copy(&mut server_read, &mut client_write);

                        let _ = tokio::try_join!(client_to_server, server_to_client);
                    },
                    Err(_) => {
                        let _ = client_stream.write_all(b"HTTP/1.1 502 Bad Gateway\r\n\r\n").await;
                    }
                }
            }
        } else {
            // Standard HTTP
            let lines: Vec<&str> = request.lines().collect();
            if let Some(first_line) = lines.first() {
                let parts: Vec<&str> = first_line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let mut url = parts[1].to_string();
                    if url.starts_with("http://") {
                        url = url[7..].to_string();
                    }

                    let host_port: Vec<&str> = url.split('/').next().unwrap_or(&url).split(':').collect();
                    let host = host_port[0];
                    let port = if host_port.len() > 1 { host_port[1] } else { "80" };
                    let target = format!("{}:{}", host, port);

                    let p = policy.read().await;
                    if p.blocked_domains.iter().any(|d| host.contains(d)) {
                        let _ = client_stream.write_all(b"HTTP/1.1 403 Forbidden\r\n\r\n").await;
                        return Ok(());
                    }

                    match tokio::net::TcpStream::connect(&target).await {
                        Ok(mut server_stream) => {
                            let _ = server_stream.write_all(&buffer[..bytes_read]).await;

                            let (mut client_read, mut client_write) = client_stream.into_split();
                            let (mut server_read, mut server_write) = server_stream.into_split();

                            let client_to_server = tokio::io::copy(&mut client_read, &mut server_write);
                            let server_to_client = tokio::io::copy(&mut server_read, &mut client_write);

                            let _ = tokio::try_join!(client_to_server, server_to_client);
                        },
                        Err(_) => {
                            let _ = client_stream.write_all(b"HTTP/1.1 502 Bad Gateway\r\n\r\n").await;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_proxy_block_domain() {
        let mut policy = SandboxPolicy::default();
        policy.blocked_domains.push("evil.com".to_string());

        let policy_arc = Arc::new(RwLock::new(policy));
        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();

        let listener = TcpListener::bind(addr).await.unwrap();
        let local_addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            if let Ok((stream, _)) = listener.accept().await {
                let _ = NetworkProxy::handle_connection(stream, policy_arc).await;
            }
        });

        let mut client = TcpStream::connect(local_addr).await.unwrap();
        client.write_all(b"CONNECT evil.com:443 HTTP/1.1\r\n\r\n").await.unwrap();

        let mut response = String::new();
        let _ = tokio::time::timeout(Duration::from_secs(1), client.read_to_string(&mut response)).await;
        assert!(response.contains("403 Forbidden"));
    }

    #[tokio::test]
    async fn test_proxy_socks_block_domain() {
        let mut policy = SandboxPolicy::default();
        policy.blocked_domains.push("mocked.com".to_string());

        let policy_arc = Arc::new(RwLock::new(policy));
        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();

        let listener = TcpListener::bind(addr).await.unwrap();
        let local_addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            if let Ok((stream, _)) = listener.accept().await {
                let _ = NetworkProxy::handle_connection(stream, policy_arc).await;
            }
        });

        let mut client = TcpStream::connect(local_addr).await.unwrap();
        client.write_all(b"\x05\x01\x00").await.unwrap();

        let mut response = [0; 2];
        client.read_exact(&mut response).await.unwrap();
        assert_eq!(&response, b"\x05\x00");

        client.write_all(b"\x05\x01\x00\x03\x0amocked.com\x00\x50").await.unwrap();

        let mut resp2 = [0; 10];
        client.read_exact(&mut resp2).await.unwrap();
        assert_eq!(resp2[1], 0x02); // blocked
    }
}
