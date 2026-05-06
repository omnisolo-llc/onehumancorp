use serde::Deserialize;
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation, Algorithm};
use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::OnceLock;
use chrono::{Utc, Duration};
use crate::auth::Claims;

#[derive(Debug, Clone, Deserialize)]
pub struct OIDCConfig {
    pub issuer_url: String,
    pub client_id: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct JWK {
    kid: String,
    n: String,
    e: String,
}

#[derive(Debug, Clone, Deserialize)]
struct JWKSet {
    keys: Vec<JWK>,
}

#[derive(Debug, Clone, Deserialize)]
struct OIDCDiscovery {
    jwks_uri: String,
}

struct CachedJWKS {
    keys: Vec<JWK>,
    fetch_at: chrono::DateTime<Utc>,
}

static JWKS_CACHE: OnceLock<RwLock<HashMap<String, CachedJWKS>>> = OnceLock::new();

fn get_cache() -> &'static RwLock<HashMap<String, CachedJWKS>> {
    JWKS_CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

fn is_blocked_ip(ip: std::net::IpAddr) -> bool {
    if std::env::var("OHC_ALLOW_LOCAL_IPS").map(|v| v == "true").unwrap_or(false) {
        return false;
    }
    ip.is_loopback() || ip.is_unspecified() || ip.is_multicast() ||
    match ip {
        std::net::IpAddr::V4(ipv4) => ipv4.is_private() || ipv4.is_link_local(),
        std::net::IpAddr::V6(ipv6) => ipv6.is_loopback() || ipv6.is_unspecified(),
    }
}

async fn validate_url_and_get_ip(url_str: &str) -> Result<(String, std::net::IpAddr), String> {
    let url = reqwest::Url::parse(url_str).map_err(|e| e.to_string())?;
    if url.scheme() != "http" && url.scheme() != "https" {
        return Err("invalid scheme".to_string());
    }
    let host = url.host_str().ok_or_else(|| "missing host".to_string())?;
    let port = url.port().unwrap_or(if url.scheme() == "https" { 443 } else { 80 });
    
    let addr_str = format!("{}:{}", host, port);
    let addrs = tokio::net::lookup_host(addr_str).await.map_err(|e| e.to_string())?;
    
    let mut valid_ip = None;
    for addr in addrs {
        let ip = addr.ip();
        if !is_blocked_ip(ip) {
            valid_ip = Some(ip);
            break;
        }
    }
    
    let ip = valid_ip.ok_or_else(|| "URL resolves to blocked IP or no IPs found".to_string())?;
    Ok((host.to_string(), ip))
}

async fn fetch_jwks(issuer_url: &str) -> Result<Vec<JWK>, String> {
    {
        let cache = get_cache().read().unwrap();
        if let Some(cached) = cache.get(issuer_url) {
            if Utc::now() - cached.fetch_at < Duration::minutes(5) {
                return Ok(cached.keys.clone());
            }
        }
    }

    let disc_url = format!("{}/.well-known/openid-configuration", issuer_url.trim_end_matches('/'));
    
    let (host, ip) = validate_url_and_get_ip(&disc_url).await?;
    let client = reqwest::Client::builder()
        .resolve(&host, std::net::SocketAddr::new(ip, if disc_url.starts_with("https") { 443 } else { 80 }))
        .build()
        .map_err(|e| e.to_string())?;

    let disc: OIDCDiscovery = client.get(&disc_url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;
        
    let (jwks_host, jwks_ip) = validate_url_and_get_ip(&disc.jwks_uri).await?;
    let jwks_client = reqwest::Client::builder()
        .resolve(&jwks_host, std::net::SocketAddr::new(jwks_ip, if disc.jwks_uri.starts_with("https") { 443 } else { 80 }))
        .build()
        .map_err(|e| e.to_string())?;

    let keys: JWKSet = jwks_client.get(&disc.jwks_uri)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;
        
    let mut cache = get_cache().write().unwrap();
    cache.insert(issuer_url.to_string(), CachedJWKS {
        keys: keys.keys.clone(),
        fetch_at: Utc::now(),
    });
    
    Ok(keys.keys)
}

pub async fn validate_oidc_token(token_str: &str, cfg: &OIDCConfig) -> Result<Claims, String> {
    if !cfg.enabled {
        return Err("OIDC not configured".to_string());
    }
    
    let header = decode_header(token_str).map_err(|e| e.to_string())?;
    let kid = header.kid.ok_or_else(|| "missing kid in header".to_string())?;
    
    let keys = fetch_jwks(&cfg.issuer_url).await?;
    let key = keys.iter().find(|k| k.kid == kid).ok_or_else(|| "no matching JWK found".to_string())?;
    
    let decoding_key = DecodingKey::from_rsa_components(&key.n, &key.e).map_err(|e| e.to_string())?;
    
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[&cfg.client_id]);
    validation.set_issuer(&[&cfg.issuer_url]);
    
    let token_data = decode::<serde_json::Value>(token_str, &decoding_key, &validation).map_err(|e| e.to_string())?;
    
    let raw = token_data.claims;
    
    let mut roles = Vec::new();
    if let Some(r) = raw.get("roles") {
        if let Some(arr) = r.as_array() {
            for v in arr {
                if let Some(s) = v.as_str() {
                    roles.push(s.to_string());
                }
            }
        }
    }
    
    if let Some(ra) = raw.get("realm_access") {
        if let Some(r) = ra.get("roles") {
             if let Some(arr) = r.as_array() {
                 for v in arr {
                     if let Some(s) = v.as_str() {
                         roles.push(s.to_string());
                     }
                 }
             }
        }
    }
    
    if roles.is_empty() {
        roles.push("VIEWER".to_string());
    }
    
    Ok(Claims {
        sub: {
            let sub = raw.get("sub").and_then(|v| v.as_str()).unwrap_or_default().to_string();
            if sub.trim().is_empty() { return Err("missing or empty sub in OIDC token".to_string()); }
            sub
        },
        username: raw.get("preferred_username").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
        email: raw.get("email").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
        roles,
        organization_id: None,
        session_id: None,
        iat: raw.get("iat").and_then(|v| v.as_i64()).unwrap_or_default(),
        exp: raw.get("exp").and_then(|v| v.as_i64()).unwrap_or_default(),
        jti: {
            let jti = raw.get("jti").and_then(|v| v.as_str()).unwrap_or_default().to_string();
            if jti.trim().is_empty() { return Err("missing or empty jti in OIDC token".to_string()); }
            jti
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_blocked_ip() {
        assert!(is_blocked_ip("127.0.0.1".parse().unwrap()));
        assert!(is_blocked_ip("0.0.0.0".parse().unwrap()));
        assert!(is_blocked_ip("169.254.169.254".parse().unwrap())); // Link local
        assert!(is_blocked_ip("224.0.0.1".parse().unwrap())); // Multicast
        
        // Private IPs (assuming OHC_ALLOW_LOCAL_IPS is not set to true)
        assert!(is_blocked_ip("10.0.0.1".parse().unwrap()));
        assert!(is_blocked_ip("172.16.0.1".parse().unwrap()));
        assert!(is_blocked_ip("192.168.0.1".parse().unwrap()));
        
        // Public IP
        assert!(!is_blocked_ip("8.8.8.8".parse().unwrap()));
    }

    #[tokio::test]
    async fn test_validate_url_and_get_ip_valid() {
        let res = validate_url_and_get_ip("https://google.com").await;
        assert!(res.is_ok());
        let (host, ip) = res.unwrap();
        assert_eq!(host, "google.com");
        assert!(!is_blocked_ip(ip));
    }

    #[tokio::test]
    async fn test_validate_url_and_get_ip_invalid_scheme() {
        let res = validate_url_and_get_ip("ftp://google.com").await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "invalid scheme");
    }

    #[test]
    fn test_validate_url_and_get_ip_blocked() {
        temp_env::with_vars([("OHC_ALLOW_LOCAL_IPS", Some("false"))], || {
            let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
            rt.block_on(async {
                let res = validate_url_and_get_ip("http://localhost").await;
                assert!(res.is_err());
                assert!(res.unwrap_err().contains("resolves to blocked IP"));
            });
        });
    }
}
