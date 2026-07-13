use ohc_builtin_agent_core::types::ToolError;
use std::net::{IpAddr, SocketAddr};
use std::sync::Once;
use url::{Host, Url};

static PRIVATE_NETWORK_WARNING: Once = Once::new();

fn policy_error(message: impl Into<String>) -> ToolError {
    ToolError::LlmRecoverable(format!("outbound network policy: {}", message.into()))
}

fn blocked_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            let [first, second, _, _] = v4.octets();
            let special_use = first == 0
                || (first == 100 && (64..=127).contains(&second))
                || (first == 192 && second == 0)
                || (first == 198 && (18..=19).contains(&second))
                || first >= 240;
            v4.is_private()
                || v4.is_loopback()
                || v4.is_link_local()
                || v4.is_unspecified()
                || v4.is_broadcast()
                || v4.is_multicast()
                || v4.is_documentation()
                || special_use
        }
        IpAddr::V6(v6) => {
            let site_local = v6.segments()[0] & 0xffc0 == 0xfec0;
            v6.is_loopback()
                || v6.is_unspecified()
                || v6.is_unique_local()
                || v6.is_unicast_link_local()
                || v6.is_multicast()
                || site_local
                || v6.to_ipv4_mapped().is_some_and(|v4| blocked_ip(v4.into()))
        }
    }
}

/// Returns whether the explicit private-network escape hatch is enabled.
pub fn private_network_allowed() -> bool {
    let allowed = std::env::var("OHC_AGENT_ALLOW_PRIVATE_NETWORK")
        .is_ok_and(|value| value.eq_ignore_ascii_case("true"));
    if allowed {
        PRIVATE_NETWORK_WARNING.call_once(|| {
            tracing::warn!(
                "OHC_AGENT_ALLOW_PRIVATE_NETWORK is enabled; outbound agent requests may access private networks"
            );
        });
    }
    allowed
}

/// Validates URL structure and rejects literal non-public targets by default.
pub fn validate_url(url: &Url, allow_private: bool) -> Result<(), ToolError> {
    if !matches!(url.scheme(), "http" | "https") {
        return Err(policy_error("only http and https URLs are allowed"));
    }
    if !url.username().is_empty() || url.password().is_some() {
        return Err(policy_error("URL credentials are not allowed"));
    }

    match url.host() {
        Some(Host::Domain(host)) => {
            if !allow_private && host.eq_ignore_ascii_case("localhost") {
                return Err(policy_error("localhost is blocked"));
            }
        }
        Some(Host::Ipv4(ip)) => {
            if !allow_private && blocked_ip(ip.into()) {
                return Err(policy_error(format!("address {ip} is blocked")));
            }
        }
        Some(Host::Ipv6(ip)) => {
            if !allow_private && blocked_ip(ip.into()) {
                return Err(policy_error(format!("address {ip} is blocked")));
            }
        }
        None => return Err(policy_error("URL must include a host")),
    }

    Ok(())
}

/// Resolves and validates every target address, returning the approved addresses
/// so callers can pin the subsequent connection and prevent DNS rebinding.
pub async fn validate_and_resolve(
    url: &Url,
    allow_private: bool,
) -> Result<Vec<SocketAddr>, ToolError> {
    validate_url(url, allow_private)?;

    let host = match url.host() {
        Some(Host::Domain(host)) => host.to_string(),
        Some(Host::Ipv4(ip)) => ip.to_string(),
        Some(Host::Ipv6(ip)) => ip.to_string(),
        None => return Err(policy_error("URL must include a host")),
    };
    let port = url
        .port_or_known_default()
        .ok_or_else(|| policy_error("URL has no resolvable port"))?;
    let addresses = tokio::net::lookup_host((host.as_str(), port))
        .await
        .map_err(|error| policy_error(format!("failed to resolve {host}: {error}")))?
        .collect::<Vec<_>>();

    if addresses.is_empty() {
        return Err(policy_error(format!("{host} resolved to no addresses")));
    }
    if !allow_private
        && let Some(address) = addresses.iter().find(|address| blocked_ip(address.ip()))
    {
        return Err(policy_error(format!(
            "resolved address {} is blocked",
            address.ip()
        )));
    }

    Ok(addresses)
}

/// Pins a domain client to the addresses returned by [`validate_and_resolve`].
pub fn pin_resolved_addresses(
    builder: reqwest::ClientBuilder,
    url: &Url,
    addresses: &[SocketAddr],
) -> reqwest::ClientBuilder {
    match url.host() {
        Some(Host::Domain(host)) => builder.resolve_to_addrs(host, addresses),
        _ => builder,
    }
}

#[cfg(test)]
mod tests {
    use super::{pin_resolved_addresses, validate_url};
    use std::net::SocketAddr;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use url::Url;

    fn rejected(url: &str) {
        let url = Url::parse(url).unwrap();
        assert!(validate_url(&url, false).is_err(), "accepted {url}");
    }

    #[test]
    fn network_policy_rejects_non_http_urls_and_credentials() {
        rejected("file:///etc/passwd");
        rejected("ftp://example.com/file");
        rejected("https://user:secret@example.com/");
    }

    #[test]
    fn network_policy_rejects_local_and_private_hosts() {
        for url in [
            "http://localhost/",
            "http://LOCALHOST/",
            "http://127.0.0.1/",
            "http://0.0.0.0/",
            "http://169.254.1.1/",
            "http://10.0.0.1/",
            "http://172.16.0.1/",
            "http://192.168.1.1/",
            "http://224.0.0.1/",
            "http://255.255.255.255/",
            "http://0.0.0.1/",
            "http://100.64.0.1/",
            "http://192.0.2.1/",
            "http://198.18.0.1/",
            "http://240.0.0.1/",
            "http://[::1]/",
            "http://[::]/",
            "http://[fc00::1]/",
            "http://[fe80::1]/",
            "http://[ff02::1]/",
            "http://[fec0::1]/",
            "http://[::ffff:192.168.1.1]/",
        ] {
            rejected(url);
        }
    }

    #[test]
    fn network_policy_accepts_public_https_urls() {
        let url = Url::parse("https://example.com/path").unwrap();
        validate_url(&url, false).unwrap();
    }

    #[test]
    fn network_policy_private_override_does_not_allow_unsafe_schemes_or_credentials() {
        for url in ["file:///etc/passwd", "https://user:secret@example.com/"] {
            let url = Url::parse(url).unwrap();
            assert!(validate_url(&url, true).is_err());
        }
    }

    #[tokio::test]
    async fn network_policy_pins_connections_to_validated_addresses() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = [0_u8; 4096];
            let _ = stream.read(&mut request).await;
            stream
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok")
                .await
                .unwrap();
        });
        let url = Url::parse(&format!("http://unresolvable.invalid:{}/", address.port())).unwrap();
        let addresses = [SocketAddr::new(address.ip(), address.port())];
        let client =
            pin_resolved_addresses(reqwest::Client::builder().no_proxy(), &url, &addresses)
                .build()
                .unwrap();

        let response = client.get(url).send().await.unwrap();
        server.await.unwrap();

        assert_eq!(response.text().await.unwrap(), "ok");
    }
}
