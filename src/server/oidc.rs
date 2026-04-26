use serde::{Deserialize, Serialize};
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
    kty: String,
    alg: String,
    r#use: String,
    n: String,
    e: String,
}

#[derive(Debug, Clone, Deserialize)]
struct JWKSet {
    keys: Vec<JWK>,
}

#[derive(Debug, Clone, Deserialize)]
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
    
    let disc: OIDCDiscovery = reqwest::get(&disc_url)
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;
        
    let keys: JWKSet = reqwest::get(&disc.jwks_uri)
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
