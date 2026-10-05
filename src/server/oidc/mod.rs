#![allow(clippy::upper_case_acronyms, clippy::collapsible_if)]
use ::server_common::Claims;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, Validation, decode, decode_header,
    jwk::{AlgorithmParameters, Jwk, JwkSet, KeyAlgorithm, KeyOperations, PublicKeyUse},
};
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
struct OIDCDiscovery {
    issuer: String,
    jwks_uri: String,
}

#[derive(Clone)]
enum CachedJwksResult {
    Available(std::sync::Arc<Vec<Jwk>>),
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
    if std::env::var("OMNISOLO_ALLOW_LOCAL_IPS")
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
    let allow_local_http =
        std::env::var("OMNISOLO_OIDC_ALLOW_HTTP").is_ok_and(|value| value == "true");
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
) -> Option<Result<std::sync::Arc<Vec<Jwk>>, String>> {
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
) -> Result<std::sync::Arc<Vec<Jwk>>, String>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<Vec<Jwk>, String>>,
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

fn validate_jwks(keys: Vec<Jwk>) -> Result<Vec<Jwk>, String> {
    if keys.is_empty() || keys.len() > MAX_JWKS_KEYS {
        return Err("invalid JWKS key count".to_string());
    }
    let mut kids = HashSet::with_capacity(keys.len());
    for key in &keys {
        let kid = key.common.key_id.as_deref().unwrap_or_default();
        if kid.is_empty() || kid.len() > MAX_KID_BYTES || !kids.insert(kid) {
            return Err("invalid JWKS key".to_string());
        }
        if let AlgorithmParameters::RSA(rsa) = &key.algorithm
            && (rsa.n.is_empty()
                || rsa.n.len() > MAX_MODULUS_BYTES
                || rsa.e.is_empty()
                || rsa.e.len() > MAX_EXPONENT_BYTES)
        {
            return Err("invalid JWKS key".to_string());
        }
    }
    // Other supported key types can coexist in a provider's set. Only RSA keys
    // whose optional metadata permits RS256 verification enter the token cache.
    Ok(keys
        .into_iter()
        .filter(|key| {
            matches!(key.algorithm, AlgorithmParameters::RSA(_))
                && key
                    .common
                    .public_key_use
                    .as_ref()
                    .is_none_or(|usage| *usage == PublicKeyUse::Signature)
                && key
                    .common
                    .key_operations
                    .as_ref()
                    .is_none_or(|operations| operations.contains(&KeyOperations::Verify))
                && key
                    .common
                    .key_algorithm
                    .is_none_or(|algorithm| algorithm == KeyAlgorithm::RS256)
        })
        .collect())
}

async fn fetch_jwks_uncached(issuer_url: &str) -> Result<Vec<Jwk>, String> {
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
    if disc.issuer != issuer_url {
        return Err("OIDC discovery issuer does not match configured authority".to_string());
    }
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
    let keys: JwkSet = bounded_json(
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

async fn fetch_jwks(issuer_url: &str) -> Result<std::sync::Arc<Vec<Jwk>>, String> {
    fetch_jwks_cached(issuer_url, || fetch_jwks_uncached(issuer_url)).await
}

async fn refresh_jwk_for_unknown_kid<F, Fut>(
    issuer_url: &str,
    kid: &str,
    fetch: F,
) -> Result<Option<Jwk>, String>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<Vec<Jwk>, String>>,
{
    let fetch_lock = issuer_fetch_lock(issuer_url)?;
    let _guard = tokio::time::timeout(FETCH_WAIT_TIMEOUT, fetch_lock.lock_owned())
        .await
        .map_err(|_| "timed out waiting for OIDC authority refresh".to_string())?;
    if let Some(result) = cached_jwks(issuer_url, Utc::now()) {
        let keys = result?;
        if let Some(key) = keys
            .iter()
            .find(|key| key.common.key_id.as_deref() == Some(kid))
        {
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
    Ok(keys
        .iter()
        .find(|key| key.common.key_id.as_deref() == Some(kid))
        .cloned())
}

async fn fetch_jwk(issuer_url: &str, kid: &str) -> Result<Option<Jwk>, String> {
    let had_positive_cache = matches!(cached_jwks(issuer_url, Utc::now()), Some(Ok(_)));
    let keys = fetch_jwks(issuer_url).await?;
    if let Some(key) = keys
        .iter()
        .find(|key| key.common.key_id.as_deref() == Some(kid))
    {
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

    let decoding_key = DecodingKey::from_jwk(&key).map_err(|_| OidcValidationError::Unavailable)?;

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

    if raw.get("email_verified").and_then(|value| value.as_bool()) != Some(true) {
        tracing::warn!(event = "auth.oidc.unverified_email");
        return Err(OidcValidationError::InvalidToken);
    }

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
        jti: raw
            .get("jti")
            .and_then(|v| v.as_str())
            .filter(|value| !value.trim().is_empty())
            .map(str::to_string)
            .unwrap_or_else(|| {
                format!(
                    "oidc:{}",
                    raw.get("sub").and_then(|v| v.as_str()).unwrap_or_default()
                )
            }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    fn rsa_jwk_json(kid: &str) -> serde_json::Value {
        serde_json::json!({"kty": "RSA", "kid": kid, "n": "n".repeat(256), "e": "AQAB"})
    }

    fn rsa_jwk(kid: &str) -> Jwk {
        serde_json::from_value(rsa_jwk_json(kid)).unwrap()
    }

    fn validated_test_jwks(keys: Vec<serde_json::Value>) -> Result<Vec<Jwk>, String> {
        let set: JwkSet = serde_json::from_value(serde_json::json!({"keys": keys}))
            .map_err(|error| error.to_string())?;
        validate_jwks(set.keys)
    }

    #[test]
    fn mixed_jwks_retains_only_rs256_verification_keys() {
        let keys = validated_test_jwks(vec![
            rsa_jwk_json("rsa"),
            serde_json::json!({"kty": "EC", "kid": "ec", "crv": "P-256", "x": "AA", "y": "AA"}),
            serde_json::json!({"kty": "oct", "kid": "oct", "k": "AA"}),
        ])
        .expect("other supported Jwk types must not break RSA verification");
        assert_eq!(keys.len(), 1);
    }

    #[test]
    fn jwks_excludes_keys_with_ineligible_signature_metadata() {
        for metadata in [
            serde_json::json!({"use": "enc"}),
            serde_json::json!({"use": "unknown"}),
            serde_json::json!({"key_ops": ["sign"]}),
            serde_json::json!({"key_ops": []}),
            serde_json::json!({"key_ops": ["decrypt"]}),
            serde_json::json!({"alg": "RS384"}),
            serde_json::json!({"alg": "HS256"}),
            serde_json::json!({"alg": "RSA-OAEP"}),
            serde_json::json!({"alg": "unrecognized"}),
        ] {
            let mut key = rsa_jwk_json("ineligible");
            key.as_object_mut()
                .unwrap()
                .extend(metadata.as_object().unwrap().clone());
            assert!(
                validated_test_jwks(vec![key]).unwrap().is_empty(),
                "ineligible metadata: {metadata}"
            );
        }
    }

    #[test]
    fn jwks_accepts_optional_or_explicit_rs256_verification_metadata() {
        for metadata in [
            serde_json::json!({}),
            serde_json::json!({"use": "sig", "key_ops": ["verify"], "alg": "RS256"}),
        ] {
            let mut key = rsa_jwk_json("eligible");
            key.as_object_mut()
                .unwrap()
                .extend(metadata.as_object().unwrap().clone());
            assert_eq!(validated_test_jwks(vec![key]).unwrap().len(), 1);
        }
    }

    #[test]
    fn jwks_rejects_missing_empty_and_oversized_identifiers_and_rsa_fields() {
        let mut missing_kid = rsa_jwk_json("key");
        missing_kid.as_object_mut().unwrap().remove("kid");
        assert!(validated_test_jwks(vec![missing_kid]).is_err());
        for (field, value) in [
            ("kid", String::new()),
            ("kid", "k".repeat(MAX_KID_BYTES + 1)),
            ("n", String::new()),
            ("n", "n".repeat(MAX_MODULUS_BYTES + 1)),
            ("e", String::new()),
            ("e", "e".repeat(MAX_EXPONENT_BYTES + 1)),
        ] {
            let mut key = rsa_jwk_json("key");
            key[field] = serde_json::Value::String(value);
            assert!(
                validated_test_jwks(vec![key]).is_err(),
                "invalid {field} accepted"
            );
        }
    }

    #[test]
    fn jwks_rejects_duplicate_ids_before_filtering_ineligible_keys() {
        let mut encryption_key = rsa_jwk_json("shared-id");
        encryption_key["use"] = serde_json::json!("enc");
        assert!(validated_test_jwks(vec![rsa_jwk_json("shared-id"), encryption_key]).is_err());
    }

    #[test]
    fn discovery_issuer_must_match_the_configured_authority() {
        temp_env::with_vars(
            [
                ("OMNISOLO_ALLOW_LOCAL_IPS", Some("true")),
                ("OMNISOLO_OIDC_ALLOW_HTTP", Some("true")),
            ],
            || {
                tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
                    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
                    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
                    let issuer = format!("http://{}", listener.local_addr().unwrap());
                    let jwks_uri = format!("{issuer}/keys");
                    let server = tokio::spawn(async move {
                        for body in [
                            serde_json::json!({"issuer": "https://wrong-authority.example", "jwks_uri": jwks_uri}),
                            serde_json::json!({"keys": [rsa_jwk_json("key")]}),
                        ] {
                            let (stream, _) = listener.accept().await.unwrap();
                            let mut reader = BufReader::new(stream);
                            let mut request = Vec::new();
                            loop {
                                assert!(reader.read_until(b'\n', &mut request).await.unwrap() > 0);
                                assert!(request.len() <= 4096);
                                if request.ends_with(b"\r\n\r\n") {
                                    break;
                                }
                            }
                            let mut stream = reader.into_inner();
                            let body = body.to_string();
                            stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body).as_bytes()).await.unwrap();
                        }
                    });
                    let result = fetch_jwks_uncached(&issuer).await;
                    server.abort();
                    assert!(result.is_err(), "discovery from a different issuer was accepted");
                });
            },
        );
    }

    fn signed_token_fixture(
        issuer: &str,
        overrides: serde_json::Value,
    ) -> (String, serde_json::Value) {
        let encoding_key = jsonwebtoken::EncodingKey::from_rsa_pem(include_bytes!(
            "fixtures/test-rsa-private.pem"
        ))
        .unwrap();
        let mut jwk =
            jsonwebtoken::jwk::Jwk::from_encoding_key(&encoding_key, Algorithm::RS256).unwrap();
        jwk.common.key_id = Some("signed-key".to_string());
        let mut claims = serde_json::json!({
            "iss": issuer, "aud": "test-client", "sub": "test-subject",
            "email": "user@example.test", "email_verified": true,
            "iat": Utc::now().timestamp(), "exp": Utc::now().timestamp() + 300,
        });
        claims
            .as_object_mut()
            .unwrap()
            .extend(overrides.as_object().unwrap().clone());
        let mut header = jsonwebtoken::Header::new(Algorithm::RS256);
        header.kid = Some("signed-key".to_string());
        (
            jsonwebtoken::encode(&header, &claims, &encoding_key).unwrap(),
            serde_json::to_value(jwk).unwrap(),
        )
    }

    #[test]
    fn rs256_tokens_preserve_issuer_audience_expiry_and_verified_email_checks() {
        // Token failures record telemetry, whose configuration must stay in test mode.
        temp_env::with_vars([("TEST_WORKSPACE", Some("oidc-signature-test"))], || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    for (case, overrides) in [
                        ("valid", serde_json::json!({})),
                        (
                            "wrong-issuer",
                            serde_json::json!({"iss": "https://other.example"}),
                        ),
                        (
                            "wrong-audience",
                            serde_json::json!({"aud": "another-client"}),
                        ),
                        (
                            "expired",
                            serde_json::json!({"exp": Utc::now().timestamp() - 120}),
                        ),
                        (
                            "unverified-email",
                            serde_json::json!({"email_verified": false}),
                        ),
                        ("empty-subject", serde_json::json!({"sub": ""})),
                    ] {
                        let issuer = format!("https://{case}.example.test");
                        let (token, jwk) = signed_token_fixture(&issuer, overrides);
                        cache_jwks(
                            &issuer,
                            CachedJwksResult::Available(Arc::new(
                                validated_test_jwks(vec![jwk]).unwrap(),
                            )),
                            Utc::now(),
                        );
                        let cfg = OIDCConfig {
                            issuer_url: issuer,
                            client_id: "test-client".into(),
                            enabled: true,
                        };
                        let result = validate_oidc_token(&token, &cfg).await;
                        if case == "valid" {
                            let claims = result.unwrap();
                            assert_eq!(claims.sub, "test-subject");
                            assert_eq!(claims.roles, ["VIEWER"]);
                        } else {
                            assert!(
                                matches!(result, Err(OidcValidationError::InvalidToken)),
                                "invalid {case}: {result:?}"
                            );
                        }
                    }
                });
        });
    }

    #[tokio::test]
    async fn encryption_only_jwk_cannot_authenticate_a_valid_rs256_signature() {
        let issuer = "https://encryption-only.example.test";
        let (token, mut jwk) = signed_token_fixture(issuer, serde_json::json!({}));
        jwk["use"] = serde_json::json!("enc");
        cache_jwks(
            issuer,
            CachedJwksResult::Available(Arc::new(validated_test_jwks(vec![jwk]).unwrap())),
            Utc::now(),
        );
        // Do not perform an external refresh for this deliberately ineligible key.
        unknown_kid_refresh_allowed(issuer, Utc::now());
        let cfg = OIDCConfig {
            issuer_url: issuer.into(),
            client_id: "test-client".into(),
            enabled: true,
        };
        assert!(matches!(
            validate_oidc_token(&token, &cfg).await,
            Err(OidcValidationError::InvalidToken)
        ));
    }

    #[test]
    fn test_is_blocked_ip() {
        temp_env::with_vars(vec![("OMNISOLO_ALLOW_LOCAL_IPS", None::<String>)], || {
            assert!(is_blocked_ip("127.0.0.1".parse().unwrap()));
            assert!(is_blocked_ip("0.0.0.0".parse().unwrap()));
            assert!(is_blocked_ip("169.254.169.254".parse().unwrap())); // Link local
            assert!(is_blocked_ip("224.0.0.1".parse().unwrap())); // Multicast

            // Private IPs (assuming OMNISOLO_ALLOW_LOCAL_IPS is not set to true)
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
                ("OMNISOLO_ALLOW_LOCAL_IPS", Some("true")),
                ("OMNISOLO_OIDC_ALLOW_HTTP", Some("true")),
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
        temp_env::with_vars(vec![("OMNISOLO_OIDC_ALLOW_HTTP", None::<String>)], || {
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
                    Ok(vec![rsa_jwk("shared-key")])
                })
                .await
            });
        }
        while let Some(result) = tasks.join_next().await {
            let keys = result.unwrap().unwrap();
            assert_eq!(keys.len(), 1);
            assert_eq!(keys[0].common.key_id.as_deref(), Some("shared-key"));
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
            CachedJwksResult::Available(Arc::new(vec![rsa_jwk("old-key")])),
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
                    Ok(vec![rsa_jwk("new-key")])
                })
                .await
            });
        }
        while let Some(result) = tasks.join_next().await {
            assert_eq!(
                result.unwrap().unwrap().unwrap().common.key_id.as_deref(),
                Some("new-key")
            );
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
        let key = || rsa_jwk("key-1");
        assert!(validate_jwks(vec![key()]).is_ok());
        assert!(validate_jwks(Vec::new()).is_err());
        assert!(validate_jwks(vec![key(); MAX_JWKS_KEYS + 1]).is_err());
        assert!(validate_jwks(vec![key(), key()]).is_err());
        let mut oversized = key();
        if let AlgorithmParameters::RSA(rsa) = &mut oversized.algorithm {
            rsa.n = "n".repeat(MAX_MODULUS_BYTES + 1);
        }
        assert!(validate_jwks(vec![oversized]).is_err());
    }

    #[test]
    fn test_validate_url_and_get_ip_blocked() {
        temp_env::with_vars(
            vec![
                ("OMNISOLO_ALLOW_LOCAL_IPS", Some("false")),
                ("OMNISOLO_OIDC_ALLOW_HTTP", Some("true")),
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
