use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use url::Url;
use std::sync::Arc;
use reqwest::{Client, ClientBuilder};

pub fn build_safe_client() -> Result<Client, String> {
    ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none()) // Prevent SSRF via redirects
        .dns_resolver(SafeResolver::new())
        .build()
        .map_err(|e| format!("Failed to build safe HTTP client: {}", e))
}

pub fn validate_url(url_str: &str) -> Result<(), String> {
    if url_str.is_empty() {
        return Ok(());
    }

    let url = Url::parse(url_str).map_err(|_| "invalid URL format".to_string())?;
    let host = url.host_str().ok_or("URL must contain a host".to_string())?;

    // Default to port 80 if none is specified for resolution purposes
    let port = url.port().unwrap_or(80);
    let addr_str = format!("{}:{}", host, port);

    // This is a naive check. A complete solution would use SafeResolver for all requests,
    // but the `validate_url` function acts as a quick fail-fast for obvious bad URLs.
    let addrs = addr_str.to_socket_addrs().map_err(|_| "DNS resolution failed".to_string())?;

    for addr in addrs {
        let ip = addr.ip();
        if ip.is_loopback() || is_private(&ip) || is_link_local(&ip) || is_unspecified(&ip) {
            return Err("URL resolves to a blocked IP address".to_string());
        }
    }

    Ok(())
}

struct SafeResolver;

impl SafeResolver {
    fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl reqwest::dns::Resolve for SafeResolver {
    fn resolve(&self, name: reqwest::dns::Name) -> reqwest::dns::Resolving {
        // We do not want to implement a fully custom DNS resolver here to keep it simple,
        // but we need to block at the connection level.
        // reqwest does not easily allow returning an error from resolve, so we will resolve using the default resolver
        // and filter the results.

        // Actually, implementing a custom resolver is complex because reqwest::dns::Resolve requires returning a Box<dyn Iterator<Item = SocketAddr> + Send>.

        let name_str = name.as_str().to_string();
        Box::pin(async move {
            let addrs = tokio::net::lookup_host((name_str.as_str(), 0)).await;
            match addrs {
                Ok(iter) => {
                    let safe_addrs: Vec<SocketAddr> = iter.filter(|addr| {
                        let ip = addr.ip();
                        !(ip.is_loopback() || is_private(&ip) || is_link_local(&ip) || is_unspecified(&ip))
                    }).collect();

                    if safe_addrs.is_empty() {
                         let err: Box<dyn std::error::Error + Send + Sync> = "DNS resolved to a blocked or private IP address".into();
                         Err(err)
                    } else {
                         Ok(Box::new(safe_addrs.into_iter()) as Box<dyn reqwest::dns::Addrs>)
                    }
                }
                Err(e) => {
                    let err: Box<dyn std::error::Error + Send + Sync> = format!("DNS resolution failed: {}", e).into();
                    Err(err)
                }
            }
        })
    }
}

fn is_private(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => ipv4.is_private(),
        IpAddr::V6(ipv6) => is_ipv6_private(ipv6),
    }
}

fn is_ipv6_private(ip: &std::net::Ipv6Addr) -> bool {
    // Unique Local Addresses (fc00::/7) and Site-Local (fec0::/10)
    (ip.segments()[0] & 0xfe00) == 0xfc00 || (ip.segments()[0] & 0xffc0) == 0xfec0
}

fn is_link_local(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => ipv4.is_link_local(),
        IpAddr::V6(ipv6) => is_ipv6_link_local(ipv6),
    }
}

fn is_ipv6_link_local(ip: &std::net::Ipv6Addr) -> bool {
    (ip.segments()[0] & 0xffc0) == 0xfe80
}

fn is_unspecified(ip: &IpAddr) -> bool {
    ip.is_unspecified()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_private() {
        let ip_private = "10.0.0.1".parse::<IpAddr>().unwrap();
        let ip_public = "8.8.8.8".parse::<IpAddr>().unwrap();
        assert!(is_private(&ip_private));
        assert!(!is_private(&ip_public));
    }
}
