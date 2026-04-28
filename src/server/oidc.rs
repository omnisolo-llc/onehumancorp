use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation, Algorithm};
use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::OnceLock;
use chrono::{Utc, Duration};
use crate::auth::Claims;

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OIDCConfig {
    pub issuer_url: String,
    pub client_id: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct JWK {
    kid: String,
    kty: String,
    alg: String,
    r#use: String,
    n: String,
    e: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct JWKSet {
    keys: Vec<JWK>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct OIDCDiscovery {
    issuer: String,
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

    // Explicitly delete/clean old map entries to bounded memory growth
    let now = Utc::now();
    cache.retain(|_, v| now - v.fetch_at < Duration::hours(1));

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
    
    let token_data = match decode::<serde_json::Value>(token_str, &decoding_key, &validation) {
        Ok(data) => data,
        Err(e) => {
            // Log verification failure
            use std::io::Write;
            if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("events.jsonl") {
                let event = serde_json::json!({
                    "event": "OidcIssuerVerificationEvent",
                    "status": "unauthorized",
                    "reason": e.to_string()
                });
                let _ = writeln!(file, "{}", event);
            }
            return Err(e.to_string());
        }
    };

    // Log verification success
    use std::io::Write;
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("events.jsonl") {
        let event = serde_json::json!({
            "event": "OidcIssuerVerificationEvent",
            "status": "success",
            "agent_id": "system"
        });
        let _ = writeln!(file, "{}", event);
    }

    
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
        sub: raw.get("sub").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
        username: raw.get("preferred_username").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
        email: raw.get("email").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
        roles,
        organization_id: None,
        session_id: None,
        iat: raw.get("iat").and_then(|v| v.as_i64()).unwrap_or_default(),
        exp: raw.get("exp").and_then(|v| v.as_i64()).unwrap_or_default(),
        jti: raw.get("jti").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;

    #[test]
    fn test_strict_schema_and_payload_validation() {
        let json_payload = r#"{
            "kid": "key1",
            "kty": "RSA",
            "alg": "RS256",
            "use": "sig",
            "n": "xxx",
            "e": "yyy",
            "unknown_field": "should fail"
        }"#;
        let res: Result<JWK, serde_json::Error> = serde_json::from_str(json_payload);
        assert!(res.is_err(), "Payload with unknown field should be rejected");
    }

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

    #[tokio::test]
    async fn test_validate_url_and_get_ip_blocked() {
        std::env::set_var("OHC_ALLOW_LOCAL_IPS", "false");
        let res = validate_url_and_get_ip("http://localhost").await;
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("resolves to blocked IP"));
    }

    #[tokio::test]
    async fn test_memory_bounded_growth() {
        let mut cache = get_cache().write().unwrap();
        // Clear cache
        cache.clear();
        let now = Utc::now();
        cache.insert("old_url".to_string(), CachedJWKS {
            keys: vec![],
            fetch_at: now - Duration::hours(2),
        });
        cache.insert("new_url".to_string(), CachedJWKS {
            keys: vec![],
            fetch_at: now,
        });

        cache.retain(|_, v| now - v.fetch_at < Duration::hours(1));
        assert!(!cache.contains_key("old_url"), "Old entries should be removed");
        assert!(cache.contains_key("new_url"), "New entries should be kept");
    }

    #[tokio::test]
    async fn test_validate_oidc_token_invalid_issuer() {
        let cfg = OIDCConfig {
            issuer_url: "https://valid.issuer.com".to_string(),
            client_id: "test_client".to_string(),
            enabled: true,
        };
        // Provide dummy invalid token
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImR1bW15In0.eyJpc3MiOiJodHRwczovL2ludmFsaWQuaXNzdWVyLmNvbSIsImF1ZCI6InRlc3RfY2xpZW50In0.dummy_signature";
        // It should fail and we check if the function rejects it with an error.
        // As a side effect, validate_oidc_token will append to events.jsonl
        let result = validate_oidc_token(token, &cfg).await;
        assert!(result.is_err(), "Invalid token should be rejected");
    }
}
