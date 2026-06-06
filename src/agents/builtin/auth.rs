use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;

type HmacSha256 = Hmac<Sha256>;

/// Authentication mode.
#[derive(Debug, Clone)]
pub enum AuthMode {
    /// No authentication (dev/test only).
    Disabled,
    /// Pre-shared HMAC-SHA256 token.
    Token { token_hash: Vec<u8> },
    /// SPIFFE/mTLS peer certificate.
    Spiffe { allowed_id: String },
}

/// Build an AuthMode from environment variables.
///
///   OHC_AGENT_AUTH_DISABLED=true   – skip auth (dev only)
///   OHC_AGENT_TOKEN                – enables token mode
///   OHC_AGENT_SPIFFE_ID            – restricts SPIFFE ID (enables SPIFFE mode)
pub fn auth_mode_from_env() -> AuthMode {
    if let Ok(tok) = env::var("OHC_AGENT_TOKEN")
        && !tok.is_empty() {
            let hash = hmac_token(&tok);
            return AuthMode::Token { token_hash: hash };
        }
    AuthMode::Spiffe {
        allowed_id: env::var("OHC_AGENT_SPIFFE_ID").unwrap_or_default(),
    }
}

/// Check a bearer token against an expected HMAC hash.
/// Returns true if the token matches.
pub fn check_token(provided: &str, expected_hash: &[u8]) -> bool {
    let provided_hash = hmac_token(provided);
    // Constant-time comparison
    if provided_hash.len() != expected_hash.len() {
        return false;
    }
    let mut diff = 0u8;
    for (a, b) in provided_hash.iter().zip(expected_hash.iter()) {
        diff |= a ^ b;
    }
    diff == 0
}

/// Compute HMAC-SHA256 of the token using the application key.
/// Mirrors Go hmacToken.
pub fn hmac_token(tok: &str) -> Vec<u8> {
    let key = std::env::var("OHC_AGENT_AUTH_KEY")
        .unwrap_or_else(|_| "default_auth_key_change_me".to_string());
    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC init failed");
    mac.update(tok.as_bytes());
    mac.finalize().into_bytes().to_vec()
}

/// Validate a SPIFFE ID — mirrors Go validateSPIFFEID.
pub fn validate_spiffe_id(id: &str) -> Result<(), String> {
    let lower = id.to_lowercase();
    if lower.contains("%2f") || lower.contains("%25") {
        return Err(format!("invalid SPIFFE ID: encoded slashes: {}", id));
    }
    if !id.starts_with("spiffe://") {
        return Err("invalid SPIFFE ID: missing spiffe:// prefix".to_string());
    }
    let trimmed = &id["spiffe://".len()..];
    if trimmed.contains("..") || trimmed.contains("//") {
        return Err(format!("invalid SPIFFE ID path: {}", id));
    }

    // Expected format: spiffe://<domain>/org/<org_id>/agent/<agent_id>
    // Parts: ["<domain>", "org", "<org_id>", "agent", "<agent_id>"]
    let parts: Vec<&str> = trimmed.split('/').collect();
    if parts.len() < 5 {
        return Err(format!(
            "SPIFFE ID too short, must match pattern spiffe://<domain>/org/<org_id>/agent/<agent_id>: {}",
            id
        ));
    }
    if parts[1] != "org" || parts[3] != "agent" {
        return Err(format!(
            "SPIFFE ID must contain /org/<org_id>/agent/<agent_id> structure: {}",
            id
        ));
    }
    if parts[2].is_empty() {
        return Err(format!("SPIFFE ID org_id cannot be empty: {}", id));
    }
    if parts[4].is_empty() {
        return Err(format!("SPIFFE ID agent_id cannot be empty: {}", id));
    }

    let domain = parts[0];
    match domain {
        "onehumancorp.io" | "ohc.local" | "ohc.os" | "ohc.global" => {}
        _ if domain.ends_with(".ohc.global") => {}
        _ => return Err(format!("untrusted SPIFFE domain {:?} in {}", domain, id)),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_match() {
        let hash = hmac_token("my-secret");
        assert!(check_token("my-secret", &hash));
        assert!(!check_token("wrong-secret", &hash));
    }

    #[test]
    fn test_validate_spiffe_valid() {
        assert!(validate_spiffe_id("spiffe://onehumancorp.io/org/org-1/agent/agent-1").is_ok());
        assert!(validate_spiffe_id("spiffe://ohc.local/org/org-2/agent/agent-2").is_ok());
        assert!(validate_spiffe_id("spiffe://us-east.ohc.global/org/org-4/agent/agent-4").is_ok());
    }

    #[test]
    fn test_validate_spiffe_invalid() {
        assert!(validate_spiffe_id("spiffe://evil.com/x").is_err());
        assert!(validate_spiffe_id("http://onehumancorp.io/x").is_err());
        assert!(validate_spiffe_id("spiffe://onehumancorp.io/%2F").is_err());
        assert!(validate_spiffe_id("spiffe://onehumancorp.io/org-1/agent-1").is_err()); // Missing /org/ and /agent/ structure
        assert!(validate_spiffe_id("spiffe://onehumancorp.io/org//agent/agent-1").is_err()); // Empty org_id
        assert!(validate_spiffe_id("spiffe://onehumancorp.io/org/org-1/agent/").is_err()); // Empty agent_id
    }
}
