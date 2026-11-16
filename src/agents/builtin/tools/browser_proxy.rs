use crate::network_policy;
use omnisolo_builtin_agent_core::types::ToolError;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::watch;
use url::Url;

/// Loopback HTTP + HTTPS CONNECT proxy enforcing `network_policy` (SSRF and DNS-rebinding
/// protection) across every browser navigation, redirect, XHR, and subresource request.
pub struct BrowserPolicyProxy {
    addr: SocketAddr,
    shutdown_tx: watch::Sender<bool>,
}

impl BrowserPolicyProxy {
    pub async fn start(allow_private: bool) -> Result<Self, ToolError> {
        let listener = TcpListener::bind("127.0.0.1:0").await.map_err(|e| {
            ToolError::Unexpected(format!("browser proxy: failed to bind loopback port: {e}"))
        })?;
        let addr = listener.local_addr().map_err(|e| {
            ToolError::Unexpected(format!("browser proxy: failed to read local addr: {e}"))
        })?;
        let (shutdown_tx, mut shutdown_rx) = watch::channel(false);

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    biased;
                    _ = shutdown_rx.changed() => {
                        if *shutdown_rx.borrow() {
                            break;
                        }
                    }
                    accepted = listener.accept() => {
                        let Ok((stream, _)) = accepted else { break };
                        tokio::spawn(async move {
                            let _ = handle_client(stream, allow_private).await;
                        });
                    }
                }
            }
        });

        Ok(Self { addr, shutdown_tx })
    }

    pub fn proxy_url(&self) -> String {
        format!("http://{}", self.addr)
    }

    pub fn shutdown(&self) {
        let _ = self.shutdown_tx.send(true);
    }
}

impl Drop for BrowserPolicyProxy {
    fn drop(&mut self) {
        self.shutdown();
    }
}

async fn connect_pinned(addresses: &[SocketAddr]) -> std::io::Result<TcpStream> {
    let mut last_err = None;
    for &addr in addresses {
        match tokio::time::timeout(std::time::Duration::from_secs(10), TcpStream::connect(addr))
            .await
        {
            Ok(Ok(stream)) => return Ok(stream),
            Ok(Err(e)) => last_err = Some(e),
            Err(_) => {
                last_err = Some(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "upstream connect timed out",
                ));
            }
        }
    }
    Err(last_err.unwrap_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::AddrNotAvailable, "no addresses")
    }))
}

async fn write_forbidden(stream: &mut TcpStream, reason: &str) -> std::io::Result<()> {
    let body = format!("Blocked by OmniSolo outbound network policy: {reason}\n");
    let response = format!(
        "HTTP/1.1 403 Forbidden\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    stream.write_all(response.as_bytes()).await?;
    stream.flush().await
}

async fn handle_client(client: TcpStream, allow_private: bool) -> std::io::Result<()> {
    let mut reader = BufReader::new(client);
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).await? == 0 {
        return Ok(());
    }
    let trimmed = request_line.trim_end_matches(['\r', '\n']);
    let mut parts = trimmed.split_whitespace();
    let Some(method) = parts.next() else {
        return Ok(());
    };
    let Some(target) = parts.next() else {
        return Ok(());
    };
    let version = parts.next().unwrap_or("HTTP/1.1");

    let mut headers = Vec::new();
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).await? == 0 {
            break;
        }
        if line == "\r\n" || line == "\n" {
            break;
        }
        if headers.len() > 128 {
            return Ok(());
        }
        headers.push(line);
    }

    if method.eq_ignore_ascii_case("CONNECT") {
        if target.contains(['/', '?', '#', '@', '\\']) {
            let mut stream = reader.into_inner();
            return write_forbidden(&mut stream, "CONNECT target must be host:port").await;
        }
        let authority_url = format!("https://{target}/");
        let parsed = match Url::parse(&authority_url) {
            Ok(url) => url,
            Err(e) => {
                let mut stream = reader.into_inner();
                return write_forbidden(&mut stream, &e.to_string()).await;
            }
        };
        let addresses = match network_policy::validate_and_resolve(&parsed, allow_private).await {
            Ok(addrs) => addrs,
            Err(err) => {
                let mut stream = reader.into_inner();
                return write_forbidden(&mut stream, &err.to_string()).await;
            }
        };
        let mut upstream = match connect_pinned(&addresses).await {
            Ok(stream) => stream,
            Err(err) => {
                let mut stream = reader.into_inner();
                let resp = format!("HTTP/1.1 502 Bad Gateway\r\nConnection: close\r\n\r\n{err}\n");
                return stream.write_all(resp.as_bytes()).await;
            }
        };
        let buffered = reader.buffer().to_vec();
        let mut client_stream = reader.into_inner();
        client_stream
            .write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n")
            .await?;
        client_stream.flush().await?;
        if !buffered.is_empty() && upstream.write_all(&buffered).await.is_err() {
            return Ok(());
        }
        let _ = tokio::io::copy_bidirectional(&mut client_stream, &mut upstream).await;
        return Ok(());
    }

    let parsed = match Url::parse(target) {
        Ok(url) if matches!(url.scheme(), "http" | "https") => url,
        _ => {
            let mut stream = reader.into_inner();
            return write_forbidden(&mut stream, "proxy requires absolute http/https URL").await;
        }
    };
    let addresses = match network_policy::validate_and_resolve(&parsed, allow_private).await {
        Ok(addrs) => addrs,
        Err(err) => {
            let mut stream = reader.into_inner();
            return write_forbidden(&mut stream, &err.to_string()).await;
        }
    };

    let mut upstream = match connect_pinned(&addresses).await {
        Ok(stream) => stream,
        Err(err) => {
            let mut stream = reader.into_inner();
            let resp = format!("HTTP/1.1 502 Bad Gateway\r\nConnection: close\r\n\r\n{err}\n");
            return stream.write_all(resp.as_bytes()).await;
        }
    };

    let path_and_query = match parsed.query() {
        Some(q) => format!("{}?{}", parsed.path(), q),
        None => parsed.path().to_string(),
    };
    let outbound_first_line = format!("{method} {path_and_query} {version}\r\n");
    upstream.write_all(outbound_first_line.as_bytes()).await?;
    for header in &headers {
        let lower = header.to_ascii_lowercase();
        if lower.starts_with("proxy-connection:") || lower.starts_with("connection:") {
            continue;
        }
        upstream.write_all(header.as_bytes()).await?;
    }
    upstream.write_all(b"Connection: close\r\n\r\n").await?;

    let buffered = reader.buffer().to_vec();
    let mut client_stream = reader.into_inner();
    if !buffered.is_empty() {
        upstream.write_all(&buffered).await?;
    }
    upstream.flush().await?;

    // Forward any remaining request body or tunnel response back to client.
    let (mut client_read, mut client_write) = client_stream.split();
    let (mut up_read, mut up_write) = upstream.split();
    let client_to_up = async {
        let mut buf = [0u8; 8192];
        // Non-blocking drain if content-length body was sent; close write when done or idle.
        while let Ok(Ok(n)) = tokio::time::timeout(
            std::time::Duration::from_millis(50),
            client_read.read(&mut buf),
        )
        .await
        {
            if n == 0 {
                break;
            }
            if up_write.write_all(&buf[..n]).await.is_err() {
                break;
            }
        }
    };
    let up_to_client = tokio::io::copy(&mut up_read, &mut client_write);
    let _ = tokio::join!(client_to_up, up_to_client);
    let _ = client_write.flush().await;
    Ok(())
}

#[derive(Default)]
struct SharedProxySlots {
    public: Option<BrowserPolicyProxy>,
    private: Option<BrowserPolicyProxy>,
}

/// Shared lazy proxy handle so sessions in the same policy mode reuse one loopback listener.
#[derive(Clone, Default)]
pub struct SharedBrowserProxy {
    inner: Arc<tokio::sync::Mutex<SharedProxySlots>>,
}

impl SharedBrowserProxy {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn proxy_url(&self, allow_private: bool) -> Result<String, ToolError> {
        let mut guard = self.inner.lock().await;
        let slot = if allow_private {
            &mut guard.private
        } else {
            &mut guard.public
        };
        if let Some(existing) = slot.as_ref() {
            return Ok(existing.proxy_url());
        }
        let started = BrowserPolicyProxy::start(allow_private).await?;
        let url = started.proxy_url();
        *slot = Some(started);
        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn blocks_private_and_loopback_targets_when_private_network_disallowed() {
        let proxy = BrowserPolicyProxy::start(false).await.unwrap();
        let client = reqwest::Client::builder()
            .proxy(reqwest::Proxy::all(proxy.proxy_url()).unwrap())
            .build()
            .unwrap();

        let http_resp = client
            .get("http://127.0.0.1:8080/secret")
            .send()
            .await
            .unwrap();
        assert_eq!(http_resp.status(), reqwest::StatusCode::FORBIDDEN);

        let connect_err = client.get("https://127.0.0.1:8443/secret").send().await;
        assert!(connect_err.is_err());
    }

    #[tokio::test]
    async fn forwards_allowed_http_traffic_when_private_network_explicitly_enabled() {
        let upstream = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let upstream_addr = upstream.local_addr().unwrap();
        tokio::spawn(async move {
            let (mut sock, _) = upstream.accept().await.unwrap();
            let mut buf = [0u8; 1024];
            let _ = sock.read(&mut buf).await;
            sock.write_all(
                b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\nConnection: close\r\n\r\nhello",
            )
            .await
            .unwrap();
        });

        let proxy = BrowserPolicyProxy::start(true).await.unwrap();
        let client = reqwest::Client::builder()
            .proxy(reqwest::Proxy::all(proxy.proxy_url()).unwrap())
            .build()
            .unwrap();

        let resp = client
            .get(format!("http://{upstream_addr}/test"))
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), reqwest::StatusCode::OK);
        assert_eq!(resp.text().await.unwrap(), "hello");
    }

    #[tokio::test]
    async fn shared_proxy_isolates_public_and_private_slots_and_tunnels_connect() {
        let shared = SharedBrowserProxy::new();
        let public_url = shared.proxy_url(false).await.unwrap();
        let private_url = shared.proxy_url(true).await.unwrap();
        assert_ne!(public_url, private_url);
        assert_eq!(shared.proxy_url(false).await.unwrap(), public_url);
        assert_eq!(shared.proxy_url(true).await.unwrap(), private_url);

        // Public proxy rejects non-http(s) schemes and private CONNECT
        let public_client = reqwest::Client::builder()
            .proxy(reqwest::Proxy::all(&public_url).unwrap())
            .build()
            .unwrap();
        assert!(
            public_client
                .get("ftp://example.com/file")
                .send()
                .await
                .is_err()
        );

        // Private proxy supports bidirectional CONNECT tunneling to an allowed loopback target
        let echo_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let echo_addr = echo_listener.local_addr().unwrap();
        tokio::spawn(async move {
            let (mut sock, _) = echo_listener.accept().await.unwrap();
            let mut buf = [0u8; 16];
            let n = sock.read(&mut buf).await.unwrap();
            sock.write_all(&buf[..n]).await.unwrap();
        });

        let proxy_addr = private_url.trim_start_matches("http://");
        let mut client = TcpStream::connect(proxy_addr).await.unwrap();
        let connect_req = format!("CONNECT {echo_addr} HTTP/1.1\r\nHost: {echo_addr}\r\n\r\n");
        client.write_all(connect_req.as_bytes()).await.unwrap();
        let mut reader = BufReader::new(client);
        let mut status_line = String::new();
        reader.read_line(&mut status_line).await.unwrap();
        assert!(status_line.starts_with("HTTP/1.1 200"));
        loop {
            let mut hdr = String::new();
            reader.read_line(&mut hdr).await.unwrap();
            if hdr == "\r\n" || hdr == "\n" || hdr.is_empty() {
                break;
            }
        }
        let mut raw_stream = reader.into_inner();
        raw_stream.write_all(b"ping").await.unwrap();
        raw_stream.flush().await.unwrap();
        let mut reply = [0u8; 4];
        raw_stream.read_exact(&mut reply).await.unwrap();
        assert_eq!(&reply, b"ping");
    }
}
