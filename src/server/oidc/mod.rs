#![allow(clippy::upper_case_acronyms, clippy::collapsible_if)]
use ::server_common::Claims;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::{Deserialize, de::DeserializeOwned};
use std::{
    collections::{HashMap, HashSet},
    future::Future,
    sync::{OnceLock, RwLock},
    time::Duration as StdDuration,
};

const JWKS_CACHE_SECONDS: i64 = 300;
const JWKS_FAILURE_CACHE_SECONDS: i64 = 30;
const UNKNOWN_KID_REFRESH_SECONDS: i64 = 30;
const MAX_JWKS_CACHE_ENTRIES: usize = 32;
const MAX_DISCOVERY_BYTES: usize = 16 * 1024;
const MAX_JWKS_BYTES: usize = 256 * 1024;
const MAX_JWKS_KEYS: usize = 64;
const MAX_JWKS_URI_BYTES: usize = 2048;
const MAX_AUTHORITY_URL_BYTES: usize = 2048;
const MAX_KID_BYTES: usize = 256;
const MAX_MODULUS_BYTES: usize = 2048;
const MAX_EXPONENT_BYTES: usize = 16;
const DNS_TIMEOUT: StdDuration = StdDuration::from_secs(2);
const CONNECT_TIMEOUT: StdDuration = StdDuration::from_secs(2);
const REQUEST_TIMEOUT: StdDuration = StdDuration::from_secs(5);
const FETCH_TOTAL_TIMEOUT: StdDuration = StdDuration::from_secs(10);
const FETCH_WAIT_TIMEOUT: StdDuration = StdDuration::from_secs(11);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OidcValidationError {
    InvalidToken,
    Unavailable,
}

impl std::fmt::Display for OidcValidationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidToken => formatter.write_str("invalid OIDC token"),
            Self::Unavailable => formatter.write_str("OIDC validation unavailable"),
        }
    }
}

impl std::error::Error for OidcValidationError {}

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

#[derive(Clone)]
enum CachedJwksResult {
    Available(std::sync::Arc<Vec<JWK>>),
    Unavailable,
}

struct CachedJWKS {
    result: CachedJwksResult,
    fetch_at: chrono::DateTime<Utc>,
}

static JWKS_CACHE: OnceLock<RwLock<HashMap<String, CachedJWKS>>> = OnceLock::new();
static JWKS_FETCH_LOCKS: OnceLock<
    std::sync::Mutex<HashMap<String, std::sync::Arc<tokio::sync::Mutex<()>>>>,
> = OnceLock::new();
static UNKNOWN_KID_REFRESHES: OnceLock<std::sync::Mutex<HashMap<String, chrono::DateTime<Utc>>>> =
    OnceLock::new();

fn get_cache() -> &'static RwLock<HashMap<String, CachedJWKS>> {
    JWKS_CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

fn issuer_fetch_lock(issuer_url: &str) -> Result<std::sync::Arc<tokio::sync::Mutex<()>>, String> {
    let locks = JWKS_FETCH_LOCKS.get_or_init(|| std::sync::Mutex::new(HashMap::new()));
    let mut locks = locks.lock().expect("JWKS fetch-lock map poisoned");
    if let Some(lock) = locks.get(issuer_url) {
        return Ok(lock.clone());
    }
    if locks.len() >= MAX_JWKS_CACHE_ENTRIES {
        locks.retain(|_, lock| std::sync::Arc::strong_count(lock) > 1);
        if locks.len() >= MAX_JWKS_CACHE_ENTRIES {
            return Err("too many configured OIDC authorities".to_string());
        }
    }
    let lock = std::sync::Arc::new(tokio::sync::Mutex::new(()));
    locks.insert(issuer_url.to_string(), lock.clone());
    Ok(lock)
}

fn unknown_kid_refresh_allowed(issuer_url: &str, now: chrono::DateTime<Utc>) -> bool {
    let refreshes = UNKNOWN_KID_REFRESHES.get_or_init(|| std::sync::Mutex::new(HashMap::new()));
    let mut refreshes = refreshes.lock().expect("unknown-kid refresh map poisoned");
    if refreshes
        .get(issuer_url)
        .is_some_and(|last| now - *last < Duration::seconds(UNKNOWN_KID_REFRESH_SECONDS))
    {
        return false;
    }
    if !refreshes.contains_key(issuer_url) && refreshes.len() >= MAX_JWKS_CACHE_ENTRIES {
        refreshes.retain(|_, last| now - *last < Duration::seconds(UNKNOWN_KID_REFRESH_SECONDS));
        if refreshes.len() >= MAX_JWKS_CACHE_ENTRIES {
            return false;
        }
    }
    refreshes.insert(issuer_url.to_string(), now);
    true
}

fn is_blocked_ip(ip: std::net::IpAddr) -> bool {
    if std::env::var("OHC_ALLOW_LOCAL_IPS")
        .map(|v| v == "true")
        .unwrap_or(false)
    {
        return false;
    }
    ip.is_loopback()
        || ip.is_unspecified()
        || ip.is_multicast()
        || match ip {
            std::net::IpAddr::V4(ipv4) => ipv4.is_private() || ipv4.is_link_local(),
            std::net::IpAddr::V6(ipv6) => {
                let segs = ipv6.segments();
                let is_ula = (segs[0] & 0xfe00) == 0xfc00;
                let is_link_local = (segs[0] & 0xffc0) == 0xfe80;
                let is_v4_mapped = segs[0] == 0
                    && segs[1] == 0
                    && segs[2] == 0
                    && segs[3] == 0
                    && segs[4] == 0
                    && segs[5] == 0xffff;
                is_ula
                    || is_link_local
                    || is_v4_mapped
                    || ipv6.is_loopback()
                    || ipv6.is_unspecified()
            }
        }
}

async fn validate_url_and_get_ip(url_str: &str) -> Result<(String, std::net::IpAddr), String> {
    if url_str.is_empty() || url_str.len() > MAX_AUTHORITY_URL_BYTES {
        return Err("invalid authority URL".to_string());
    }
    let url = reqwest::Url::parse(url_str).map_err(|e| e.to_string())?;
    let allow_local_http = std::env::var("OHC_OIDC_ALLOW_HTTP").is_ok_and(|value| value == "true");
    if url.scheme() != "https" && !(url.scheme() == "http" && allow_local_http) {
        return Err("invalid scheme".to_string());
    }
    if !url.username().is_empty() || url.password().is_some() {
        return Err("authority URL credentials are not allowed".to_string());
    }
    let host = url.host_str().ok_or_else(|| "missing host".to_string())?;
    let port = url
        .port()
        .unwrap_or(if url.scheme() == "https" { 443 } else { 80 });

    let addr_str = format!("{}:{}", host, port);
    let addrs = tokio::time::timeout(DNS_TIMEOUT, tokio::net::lookup_host(addr_str))
        .await
        .map_err(|_| "DNS lookup timed out".to_string())?
        .map_err(|e| e.to_string())?;

    let mut valid_ip = None;
    for addr in addrs {
        let ip = addr.ip();
        if url.scheme() == "http" && !is_local_development_ip(ip) {
            continue;
        }
        if !is_blocked_ip(ip) {
            valid_ip = Some(ip);
            break;
        }
    }

    let ip = valid_ip.ok_or_else(|| "URL resolves to blocked IP or no IPs found".to_string())?;
    Ok((host.to_string(), ip))
}

fn is_local_development_ip(ip: std::net::IpAddr) -> bool {
    ip.is_loopback()
        || match ip {
            std::net::IpAddr::V4(ipv4) => ipv4.is_private(),
            std::net::IpAddr::V6(ipv6) => {
                let first = ipv6.segments()[0];
                (first & 0xfe00) == 0xfc00
            }
        }
}

fn cached_jwks(
    issuer_url: &str,
    now: chrono::DateTime<Utc>,
) -> Option<Result<std::sync::Arc<Vec<JWK>>, String>> {
    let cache = get_cache().read().expect("JWKS cache lock poisoned");
    let cached = cache.get(issuer_url)?;
    let age = now - cached.fetch_at;
    match &cached.result {
        CachedJwksResult::Available(keys) if age < Duration::seconds(JWKS_CACHE_SECONDS) => {
            Some(Ok(std::sync::Arc::clone(keys)))
        }
        CachedJwksResult::Unavailable if age < Duration::seconds(JWKS_FAILURE_CACHE_SECONDS) => {
            Some(Err("OIDC authority temporarily unavailable".to_string()))
        }
        _ => None,
    }
}

fn cache_jwks(issuer_url: &str, result: CachedJwksResult, now: chrono::DateTime<Utc>) {
    let mut cache = get_cache().write().expect("JWKS cache lock poisoned");
    if !cache.contains_key(issuer_url) && cache.len() >= MAX_JWKS_CACHE_ENTRIES {
        let oldest = cache
            .iter()
            .min_by_key(|(issuer, cached)| (cached.fetch_at, issuer.as_str()))
            .map(|(issuer, _)| issuer.clone());
        if let Some(oldest) = oldest {
            cache.remove(&oldest);
        }
    }
    cache.insert(
        issuer_url.to_string(),
        CachedJWKS {
            result,
            fetch_at: now,
        },
    );
}

async fn fetch_jwks_cached<F, Fut>(
    issuer_url: &str,
    fetch: F,
) -> Result<std::sync::Arc<Vec<JWK>>, String>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<Vec<JWK>, String>>,
{
    if let Some(result) = cached_jwks(issuer_url, Utc::now()) {
        return result;
    }

    let fetch_lock = issuer_fetch_lock(issuer_url)?;
    let _guard = tokio::time::timeout(FETCH_WAIT_TIMEOUT, fetch_lock.lock_owned())
        .await
        .map_err(|_| "timed out waiting for OIDC authority fetch".to_string())?;
    if let Some(result) = cached_jwks(issuer_url, Utc::now()) {
        return result;
    }

    let result = tokio::time::timeout(FETCH_TOTAL_TIMEOUT, fetch())
        .await
        .map_err(|_| "OIDC authority fetch timed out".to_string())
        .and_then(|result| result)
        .map(std::sync::Arc::new);
    let cached = match &result {
        Ok(keys) => CachedJwksResult::Available(std::sync::Arc::clone(keys)),
        Err(_) => CachedJwksResult::Unavailable,
    };
    cache_jwks(issuer_url, cached, Utc::now());
    result
}

fn pinned_client(host: &str, ip: std::net::IpAddr, port: u16) -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(REQUEST_TIMEOUT)
        .pool_max_idle_per_host(2)
        .resolve(host, std::net::SocketAddr::new(ip, port))
        .build()
        .map_err(|e| e.to_string())
}

async fn bounded_json<T: DeserializeOwned>(
    mut response: reqwest::Response,
    max_bytes: usize,
) -> Result<T, String> {
    if !response.status().is_success() {
        return Err("OIDC authority returned a non-success status".to_string());
    }
    if response
        .content_length()
        .is_some_and(|length| length > max_bytes as u64)
    {
        return Err("OIDC authority response is too large".to_string());
    }
    let mut body = Vec::with_capacity(
        response
            .content_length()
            .unwrap_or_default()
            .min(max_bytes as u64) as usize,
    );
    while let Some(chunk) = response.chunk().await.map_err(|e| e.to_string())? {
        if body.len().saturating_add(chunk.len()) > max_bytes {
            return Err("OIDC authority response is too large".to_string());
        }
        body.extend_from_slice(&chunk);
    }
    serde_json::from_slice(&body).map_err(|_| "invalid OIDC authority response".to_string())
}

fn validate_jwks(keys: Vec<JWK>) -> Result<Vec<JWK>, String> {
    if keys.is_empty() || keys.len() > MAX_JWKS_KEYS {
        return Err("invalid JWKS key count".to_string());
    }
    let mut kids = HashSet::with_capacity(keys.len());
    for key in &keys {
        if key.kid.is_empty()
            || key.kid.len() > MAX_KID_BYTES
            || key.n.is_empty()
            || key.n.len() > MAX_MODULUS_BYTES
            || key.e.is_empty()
            || key.e.len() > MAX_EXPONENT_BYTES
            || !kids.insert(key.kid.as_str())
        {
            return Err("invalid JWKS key".to_string());
        }
    }
    Ok(keys)
}

async fn fetch_jwks_uncached(issuer_url: &str) -> Result<Vec<JWK>, String> {
    let disc_url = format!(
        "{}/.well-known/openid-configuration",
        issuer_url.trim_end_matches('/')
    );

    let (host, ip) = validate_url_and_get_ip(&disc_url).await?;
    let disc_port = reqwest::Url::parse(&disc_url)
        .map_err(|e| e.to_string())?
        .port_or_known_default()
        .ok_or_else(|| "missing discovery port".to_string())?;
    let client = pinned_client(&host, ip, disc_port)?;
    let disc: OIDCDiscovery = bounded_json(
        client
            .get(&disc_url)
            .send()
            .await
            .map_err(|e| e.to_string())?,
        MAX_DISCOVERY_BYTES,
    )
    .await?;
    if disc.jwks_uri.is_empty() || disc.jwks_uri.len() > MAX_JWKS_URI_BYTES {
        return Err("invalid JWKS URI".to_string());
    }
    ensure_no_transport_downgrade(&disc_url, &disc.jwks_uri)?;

    let (jwks_host, jwks_ip) = validate_url_and_get_ip(&disc.jwks_uri).await?;
    let jwks_port = reqwest::Url::parse(&disc.jwks_uri)
        .map_err(|e| e.to_string())?
        .port_or_known_default()
        .ok_or_else(|| "missing JWKS port".to_string())?;
    let jwks_client = if jwks_host == host && jwks_ip == ip && jwks_port == disc_port {
        client
    } else {
        pinned_client(&jwks_host, jwks_ip, jwks_port)?
    };
    let keys: JWKSet = bounded_json(
        jwks_client
            .get(&disc.jwks_uri)
            .send()
            .await
            .map_err(|e| e.to_string())?,
        MAX_JWKS_BYTES,
    )
    .await?;
    validate_jwks(keys.keys)
}

fn ensure_no_transport_downgrade(discovery_url: &str, jwks_uri: &str) -> Result<(), String> {
    let discovery = reqwest::Url::parse(discovery_url).map_err(|e| e.to_string())?;
    let jwks = reqwest::Url::parse(jwks_uri).map_err(|e| e.to_string())?;
    if discovery.scheme() == "https" && jwks.scheme() != "https" {
        return Err("OIDC discovery cannot downgrade JWKS transport".to_string());
    }
    Ok(())
}

async fn fetch_jwks(issuer_url: &str) -> Result<std::sync::Arc<Vec<JWK>>, String> {
    fetch_jwks_cached(issuer_url, || fetch_jwks_uncached(issuer_url)).await
}

async fn refresh_jwk_for_unknown_kid<F, Fut>(
    issuer_url: &str,
    kid: &str,
    fetch: F,
) -> Result<Option<JWK>, String>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<Vec<JWK>, String>>,
{
    let fetch_lock = issuer_fetch_lock(issuer_url)?;
    let _guard = tokio::time::timeout(FETCH_WAIT_TIMEOUT, fetch_lock.lock_owned())
        .await
        .map_err(|_| "timed out waiting for OIDC authority refresh".to_string())?;
    if let Some(result) = cached_jwks(issuer_url, Utc::now()) {
        let keys = result?;
        if let Some(key) = keys.iter().find(|key| key.kid == kid) {
            return Ok(Some(key.clone()));
        }
    }
    if !unknown_kid_refresh_allowed(issuer_url, Utc::now()) {
        return Ok(None);
    }

    let result = tokio::time::timeout(FETCH_TOTAL_TIMEOUT, fetch())
        .await
        .map_err(|_| "OIDC authority refresh timed out".to_string())
        .and_then(|result| result)
        .map(std::sync::Arc::new);
    let cached = match &result {
        Ok(keys) => CachedJwksResult::Available(std::sync::Arc::clone(keys)),
        Err(_) => CachedJwksResult::Unavailable,
    };
    cache_jwks(issuer_url, cached, Utc::now());
    let keys = result?;
    Ok(keys.iter().find(|key| key.kid == kid).cloned())
}

async fn fetch_jwk(issuer_url: &str, kid: &str) -> Result<Option<JWK>, String> {
    let had_positive_cache = matches!(cached_jwks(issuer_url, Utc::now()), Some(Ok(_)));
    let keys = fetch_jwks(issuer_url).await?;
    if let Some(key) = keys.iter().find(|key| key.kid == kid) {
        return Ok(Some(key.clone()));
    }
    if !had_positive_cache {
        return Ok(None);
    }
    refresh_jwk_for_unknown_kid(issuer_url, kid, || fetch_jwks_uncached(issuer_url)).await
}

pub async fn validate_oidc_token(
    token_str: &str,
    cfg: &OIDCConfig,
) -> Result<Claims, OidcValidationError> {
    if !cfg.enabled {
        return Err(OidcValidationError::InvalidToken);
    }

    let header = decode_header(token_str).map_err(|_| OidcValidationError::InvalidToken)?;
    if header.alg != Algorithm::RS256 {
        return Err(OidcValidationError::InvalidToken);
    }
    let kid = header.kid.ok_or(OidcValidationError::InvalidToken)?;

    let key = fetch_jwk(&cfg.issuer_url, &kid)
        .await
        .map_err(|_| OidcValidationError::Unavailable)?;
    let key = key.ok_or(OidcValidationError::InvalidToken)?;

    let decoding_key = DecodingKey::from_rsa_components(&key.n, &key.e)
        .map_err(|_| OidcValidationError::Unavailable)?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[&cfg.client_id]);
    validation.set_issuer(&[&cfg.issuer_url]);

    let token_data =
        decode::<serde_json::Value>(token_str, &decoding_key, &validation).map_err(|_| {
            ::server_telemetry::record_error_signal("[security] OIDC token validation failed");
            tracing::warn!(event = "auth.oidc.invalid_token");
            OidcValidationError::InvalidToken
        })?;

    let raw = token_data.claims;

    // Securely check for token expiration before processing
    let current_ts = Utc::now().timestamp();
    if let Some(exp) = raw.get("exp").and_then(|v| v.as_i64()) {
        if exp < current_ts {
            return Err(OidcValidationError::InvalidToken);
        }
    } else {
        return Err(OidcValidationError::InvalidToken);
    }

    if let Some(nbf) = raw.get("nbf").and_then(|v| v.as_i64()) {
        if nbf > current_ts {
            return Err(OidcValidationError::InvalidToken);
        }
    }

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
            let sub = raw
                .get("sub")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            if sub.trim().is_empty() {
                return Err(OidcValidationError::InvalidToken);
            }
            sub
        },
        username: raw
            .get("preferred_username")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string(),
        email: raw
            .get("email")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string(),
        roles,
        organization_id: raw
            .get("organization_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        session_id: None,
        iat: raw.get("iat").and_then(|v| v.as_i64()).unwrap_or_default(),
        exp: raw.get("exp").and_then(|v| v.as_i64()).unwrap_or_default(),
        jti: {
            let jti = raw
                .get("jti")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            if jti.trim().is_empty() {
                return Err(OidcValidationError::InvalidToken);
            }
            jti
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    #[test]
    fn test_is_blocked_ip() {
        temp_env::with_vars(vec![("OHC_ALLOW_LOCAL_IPS", None::<String>)], || {
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

            // IPv6 Link-local
            assert!(is_blocked_ip("fe80::1".parse().unwrap()));
            // IPv6 ULA
            assert!(is_blocked_ip("fc00::1".parse().unwrap()));
            // IPv4-mapped IPv6
            assert!(is_blocked_ip("::ffff:127.0.0.1".parse().unwrap()));
        });
    }

    #[test]
    fn test_validate_url_and_get_ip_valid() {
        temp_env::with_vars(
            vec![
                ("OHC_ALLOW_LOCAL_IPS", Some("true")),
                ("OHC_OIDC_ALLOW_HTTP", Some("true")),
            ],
            || {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        let res = validate_url_and_get_ip("http://127.0.0.1").await;
                        assert!(res.is_ok());
                        let (host, _ip) = res.unwrap();
                        assert_eq!(host, "127.0.0.1");
                    });
            },
        );
    }

    #[tokio::test]
    async fn test_validate_url_and_get_ip_invalid_scheme() {
        let res = validate_url_and_get_ip("ftp://google.com").await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "invalid scheme");
    }

    #[test]
    fn public_http_authorities_are_rejected_without_network_access() {
        temp_env::with_vars(vec![("OHC_OIDC_ALLOW_HTTP", None::<String>)], || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    assert_eq!(
                        validate_url_and_get_ip("http://example.com").await,
                        Err("invalid scheme".to_string())
                    );
                });
        });
    }

    #[test]
    fn https_discovery_cannot_downgrade_jwks_transport() {
        assert!(
            ensure_no_transport_downgrade(
                "https://issuer.example/.well-known/openid-configuration",
                "https://keys.example/jwks",
            )
            .is_ok()
        );
        assert!(
            ensure_no_transport_downgrade(
                "https://issuer.example/.well-known/openid-configuration",
                "http://127.0.0.1/jwks",
            )
            .is_err()
        );
    }

    #[tokio::test]
    async fn oidc_validation_distinguishes_invalid_tokens_from_authority_outages() {
        let config = OIDCConfig {
            issuer_url: "ftp://invalid.example".to_string(),
            client_id: "client".to_string(),
            enabled: true,
        };

        assert!(matches!(
            validate_oidc_token("not-a-token", &config).await,
            Err(OidcValidationError::InvalidToken)
        ));
        let wrong_algorithm_with_kid = "eyJhbGciOiJIUzI1NiIsImtpZCI6ImsifQ.e30.AA";
        assert!(matches!(
            validate_oidc_token(wrong_algorithm_with_kid, &config).await,
            Err(OidcValidationError::InvalidToken)
        ));
        let syntactically_valid_rs256 = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImsifQ.e30.AA";
        assert!(matches!(
            validate_oidc_token(syntactically_valid_rs256, &config).await,
            Err(OidcValidationError::Unavailable)
        ));
    }

    #[tokio::test]
    async fn jwks_outages_are_single_flight_and_negatively_cached() {
        const ISSUER: &str = "test://single-flight-outage";
        get_cache()
            .write()
            .expect("JWKS cache lock poisoned")
            .remove(ISSUER);
        let calls = Arc::new(AtomicUsize::new(0));
        let mut tasks = tokio::task::JoinSet::new();
        for _ in 0..16 {
            let calls = calls.clone();
            tasks.spawn(async move {
                fetch_jwks_cached(ISSUER, || async move {
                    calls.fetch_add(1, Ordering::SeqCst);
                    tokio::time::sleep(StdDuration::from_millis(20)).await;
                    Err("authority unavailable".to_string())
                })
                .await
            });
        }
        while let Some(result) = tasks.join_next().await {
            assert!(result.unwrap().is_err());
        }
        assert_eq!(calls.load(Ordering::SeqCst), 1);

        let calls_after = calls.clone();
        assert!(
            fetch_jwks_cached(ISSUER, || async move {
                calls_after.fetch_add(1, Ordering::SeqCst);
                Err("must remain cached".to_string())
            })
            .await
            .is_err()
        );
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn concurrent_cold_start_shares_one_successful_jwks_fetch() {
        const ISSUER: &str = "test://single-flight-success";
        get_cache()
            .write()
            .expect("JWKS cache lock poisoned")
            .remove(ISSUER);
        let calls = Arc::new(AtomicUsize::new(0));
        let mut tasks = tokio::task::JoinSet::new();
        for _ in 0..16 {
            let calls = calls.clone();
            tasks.spawn(async move {
                fetch_jwks_cached(ISSUER, || async move {
                    calls.fetch_add(1, Ordering::SeqCst);
                    tokio::time::sleep(StdDuration::from_millis(20)).await;
                    Ok(vec![JWK {
                        kid: "shared-key".to_string(),
                        n: "n".repeat(256),
                        e: "AQAB".to_string(),
                    }])
                })
                .await
            });
        }
        while let Some(result) = tasks.join_next().await {
            let keys = result.unwrap().unwrap();
            assert_eq!(keys.len(), 1);
            assert_eq!(keys[0].kid, "shared-key");
        }
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        let first = cached_jwks(ISSUER, Utc::now()).unwrap().unwrap();
        let second = cached_jwks(ISSUER, Utc::now()).unwrap().unwrap();
        assert!(Arc::ptr_eq(&first, &second));
    }

    #[tokio::test]
    async fn unknown_kid_refresh_is_single_flight_and_suppresses_repeated_misses() {
        const ISSUER: &str = "test://unknown-kid-refresh";
        cache_jwks(
            ISSUER,
            CachedJwksResult::Available(Arc::new(vec![JWK {
                kid: "old-key".to_string(),
                n: "n".repeat(256),
                e: "AQAB".to_string(),
            }])),
            Utc::now(),
        );
        UNKNOWN_KID_REFRESHES
            .get_or_init(|| std::sync::Mutex::new(HashMap::new()))
            .lock()
            .unwrap()
            .remove(ISSUER);
        let calls = Arc::new(AtomicUsize::new(0));
        let mut tasks = tokio::task::JoinSet::new();
        for _ in 0..16 {
            let calls = calls.clone();
            tasks.spawn(async move {
                refresh_jwk_for_unknown_kid(ISSUER, "new-key", || async move {
                    calls.fetch_add(1, Ordering::SeqCst);
                    tokio::time::sleep(StdDuration::from_millis(20)).await;
                    Ok(vec![JWK {
                        kid: "new-key".to_string(),
                        n: "n".repeat(256),
                        e: "AQAB".to_string(),
                    }])
                })
                .await
            });
        }
        while let Some(result) = tasks.join_next().await {
            assert_eq!(result.unwrap().unwrap().unwrap().kid, "new-key");
        }
        assert_eq!(calls.load(Ordering::SeqCst), 1);

        let suppressed_calls = calls.clone();
        assert!(
            refresh_jwk_for_unknown_kid(ISSUER, "still-missing", || async move {
                suppressed_calls.fetch_add(1, Ordering::SeqCst);
                Err("must be refresh-throttled".to_string())
            })
            .await
            .unwrap()
            .is_none()
        );
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn jwks_key_count_fields_and_duplicates_are_bounded() {
        let key = || JWK {
            kid: "key-1".to_string(),
            n: "n".repeat(256),
            e: "AQAB".to_string(),
        };
        assert!(validate_jwks(vec![key()]).is_ok());
        assert!(validate_jwks(Vec::new()).is_err());
        assert!(validate_jwks(vec![key(); MAX_JWKS_KEYS + 1]).is_err());
        assert!(validate_jwks(vec![key(), key()]).is_err());
        let mut oversized = key();
        oversized.n = "n".repeat(MAX_MODULUS_BYTES + 1);
        assert!(validate_jwks(vec![oversized]).is_err());
    }

    #[test]
    fn test_validate_url_and_get_ip_blocked() {
        temp_env::with_vars(
            vec![
                ("OHC_ALLOW_LOCAL_IPS", Some("false")),
                ("OHC_OIDC_ALLOW_HTTP", Some("true")),
            ],
            || {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        let res = validate_url_and_get_ip("http://localhost").await;
                        assert!(res.is_err());
                        assert!(res.unwrap_err().contains("resolves to blocked IP"));
                    });
            },
        );
    }
}
