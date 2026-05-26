use tonic::{Request, Status};
use hmac::{Hmac, Mac};
use sha2::Sha256;

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum AuthMode {
    Disabled,
    Token(Vec<u8>), // HMAC-SHA256 of expected token
    SPIFFE { allowed_id: Option<String> },
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AuthConfig {
    mode: AuthMode,
}

#[allow(dead_code)]
impl AuthConfig {
    pub fn from_env() -> Self {
        if let Ok(tok) = std::env::var("OHC_AGENT_TOKEN") {
            let h = hmac_token(&tok);
            return AuthConfig { mode: AuthMode::Token(h) };
        }
        // Default: SPIFFE mode
        AuthConfig {
            mode: AuthMode::SPIFFE {
                allowed_id: std::env::var("OHC_AGENT_SPIFFE_ID").ok(),
            },
        }
    }

    pub fn authenticate(&self, req: &Request<()>) -> Result<(), Status> {
        match &self.mode {
            AuthMode::Disabled => Ok(()),
            AuthMode::Token(expected_hash) => self.check_token(req, expected_hash),
            AuthMode::SPIFFE { allowed_id } => self.check_spiffe(req, allowed_id.as_deref()),
        }
    }

    fn check_token(&self, req: &Request<()>, expected_hash: &[u8]) -> Result<(), Status> {
        let md = req.metadata();
        let auth_header = md.get("authorization")
            .ok_or_else(|| Status::unauthenticated("missing authorization header"))?;
        
        let auth_str = auth_header.to_str()
            .map_err(|_| Status::unauthenticated("invalid authorization header"))?;

        if !auth_str.starts_with("Bearer ") {
            return Err(Status::unauthenticated("authorization must be Bearer token"));
        }

        let token = &auth_str["Bearer ".len()..];
        if token.is_empty() { return Err(Status::unauthenticated("empty token")); }
        
        let app_key = std::env::var("JWT_SECRET")
            .map(|s| s.into_bytes())
            .unwrap_or_else(|_| {
                let secret_path = std::path::Path::new(".ohc_jwt_secret");
                if secret_path.exists() {
                    if let Ok(bytes) = std::fs::read(secret_path) {
                        if bytes.len() >= 32 {
                            return bytes;
                        }
                    }
                }
                panic!("JWT_SECRET or valid .ohc_jwt_secret must be present for token verification");
            });
        let mut mac = Hmac::<Sha256>::new_from_slice(&app_key).expect("HMAC can take key of any size");
        mac.update(token.as_bytes());
        
        if mac.verify(expected_hash.into()).is_ok() {
             Ok(())
        } else {
             Err(Status::unauthenticated("invalid token"))
        }
    }

    fn check_spiffe(&self, req: &Request<()>, allowed_id: Option<&str>) -> Result<(), Status> {
        let md = req.metadata();
        let spiffe_id = md.get("x-spiffe-id")
            .ok_or_else(|| Status::unauthenticated("missing x-spiffe-id header"))?
            .to_str()
            .map_err(|_| Status::unauthenticated("invalid x-spiffe-id header"))?;

        validate_spiffe_id(spiffe_id)?;

        if let Some(allowed) = allowed_id {
            if spiffe_id != allowed {
                return Err(Status::permission_denied(format!(
                    "SPIFFE ID {:?} not allowed (expected {:?})",
                    spiffe_id, allowed
                )));
            }
        }

        Ok(())
    }
}

#[allow(dead_code)]
fn hmac_token(tok: &str) -> Vec<u8> {
    let key = std::env::var("OHC_AGENT_AUTH_KEY")
        .unwrap_or_else(|_| {
            if ::server_config::get().multitenant {
                panic!("OHC_AGENT_AUTH_KEY must be set in Cloud/Multitenant Mode to ensure secure token hashing.");
            }
            let secret_path = std::path::Path::new(".ohc_agent_auth_key");
            if secret_path.exists() {
                if let Ok(bytes) = std::fs::read_to_string(secret_path) {
                    if bytes.len() >= 32 {
                        return bytes.trim().to_string();
                    }
                }
            }
            panic!("OHC_AGENT_AUTH_KEY or valid .ohc_agent_auth_key must be present for token verification");
        });
    let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(tok.as_bytes());
    mac.finalize().into_bytes().to_vec()
}

#[allow(dead_code)]
fn validate_spiffe_id(id: &str) -> Result<(), Status> {
    let lower = id.to_lowercase();
    if lower.contains("%2f") || lower.contains("%25") {
        return Err(Status::permission_denied(format!("invalid SPIFFE ID: encoded slashes: {}", id)));
    }
    if !id.starts_with("spiffe://") {
        return Err(Status::permission_denied("invalid SPIFFE ID: missing spiffe:// prefix"));
    }
    let trimmed = &id["spiffe://".len()..];
    if trimmed.contains("..") || trimmed.contains("//") {
        return Err(Status::permission_denied(format!("invalid SPIFFE ID path: {}", id)));
    }
    let parts: Vec<&str> = trimmed.splitn(6, '/').collect();
    if parts.len() < 2 {
        return Err(Status::permission_denied(format!("SPIFFE ID too short: {}", id)));
    }
    let domain = parts[0];
    
    match domain {
        "onehumancorp.io" | "ohc.local" | "ohc.os" => {}
        _ if domain == "ohc.global" || domain.ends_with(".ohc.global") => {}
        _ => return Err(Status::permission_denied(format!("untrusted SPIFFE domain {:?} in {}", domain, id))),
    }
    
    Ok(())
}

#[allow(dead_code)]
pub fn interceptor(cfg: AuthConfig) -> impl Fn(Request<()>) -> Result<Request<()>, Status> + Clone {
    move |req: Request<()>| {
        cfg.authenticate(&req)?;
        Ok(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::metadata::MetadataValue;
    use std::str::FromStr;

    #[test]
    fn test_validate_spiffe_id() {
        assert!(validate_spiffe_id("spiffe://onehumancorp.io/org-1/agent-1").is_ok());
        assert!(validate_spiffe_id("spiffe://ohc.local/org/org-2/agent/agent-2").is_ok());
        assert!(validate_spiffe_id("spiffe://ohc.os/agent/agent-3").is_ok());
        assert!(validate_spiffe_id("spiffe://us-east.ohc.global/org/org-4/agent/agent-4").is_ok());

        assert!(validate_spiffe_id("invalid").is_err());
        assert!(validate_spiffe_id("spiffe://invalid.com/x").is_err());
        assert!(validate_spiffe_id("spiffe://onehumancorp.io/../bad").is_err());
    }

    #[test]
    fn test_check_token() {
        let token = "secret_token";
        let hash = hmac_token(token);
        let cfg = AuthConfig { mode: AuthMode::Token(hash) };

        let mut req = Request::new(());
        req.metadata_mut().insert("authorization", MetadataValue::from_str(&format!("Bearer {}", token)).unwrap());

        assert!(cfg.authenticate(&req).is_ok());

        let mut req2 = Request::new(());
        req2.metadata_mut().insert("authorization", MetadataValue::from_str("Bearer wrong_token").unwrap());
        assert!(cfg.authenticate(&req2).is_err());
    }
}
