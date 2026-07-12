use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;

type HmacSha256 = Hmac<Sha256>;

/// Authentication mode.
#[derive(Clone)]
pub enum AuthMode {
    /// No authentication (dev/test only).
    Disabled,
    /// Pre-shared HMAC-SHA256 token.
    Token {
        token_hash: Vec<u8>,
        verification_key: Vec<u8>,
    },
    /// SPIFFE/mTLS peer certificate (available after verified peer extraction is wired).
    Spiffe { allowed_id: String },
}

impl std::fmt::Debug for AuthMode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disabled => formatter.write_str("AuthMode::Disabled"),
            Self::Token { .. } => formatter.write_str("AuthMode::Token { <redacted> }"),
            Self::Spiffe { allowed_id } => formatter
                .debug_struct("AuthMode::Spiffe")
                .field("allowed_id", allowed_id)
                .finish(),
        }
    }
}

/// Build an AuthMode from environment variables.
///
///   OHC_AGENT_AUTH_DISABLED=true   – skip auth (dev only)
///   OHC_AGENT_TOKEN                – enables token mode
///   OHC_AGENT_SPIFFE_ID            – validates the desired identity, then fails closed until
///                                    verified mTLS peer extraction is available
pub fn auth_mode_from_env() -> Result<AuthMode, String> {
    let auth_disabled = env::var("OHC_AGENT_AUTH_DISABLED")
        .is_ok_and(|value| value.trim().eq_ignore_ascii_case("true"));
    if auth_disabled {
        let environment = env::var("OHC_ENV").unwrap_or_default();
        if matches!(
            environment.trim().to_ascii_lowercase().as_str(),
            "development" | "test"
        ) {
            return Ok(AuthMode::Disabled);
        }
        return Err(
            "OHC_AGENT_AUTH_DISABLED=true is allowed only when OHC_ENV is development or test"
                .to_string(),
        );
    }

    if let Ok(token) = env::var("OHC_AGENT_TOKEN")
        && !token.trim().is_empty()
    {
        let key = env::var("OHC_AGENT_AUTH_KEY")
            .map_err(|_| "OHC_AGENT_AUTH_KEY is required in token mode".to_string())?;
        if key.trim().is_empty() || key.len() < 32 {
            return Err("OHC_AGENT_AUTH_KEY must contain at least 32 bytes".to_string());
        }
        let verification_key = key.into_bytes();
        let token_hash = hmac_token(&token, &verification_key);
        return Ok(AuthMode::Token {
            token_hash,
            verification_key,
        });
    }

    let allowed_id = env::var("OHC_AGENT_SPIFFE_ID")
        .map_err(|_| "configure OHC_AGENT_TOKEN or OHC_AGENT_SPIFFE_ID".to_string())?;
    if allowed_id.trim().is_empty() {
        return Err("OHC_AGENT_SPIFFE_ID must not be empty".to_string());
    }
    validate_spiffe_id(&allowed_id)?;
    Err(
        "SPIFFE authentication requires verified mTLS peer identity extraction, which is not yet configured for the builtin agent; use token authentication"
            .to_string(),
    )
}

/// Check a bearer token against an expected HMAC hash.
/// Returns true if the token matches.
pub fn check_token(provided: &str, expected_hash: &[u8], key: &[u8]) -> bool {
    let Ok(mut mac) = HmacSha256::new_from_slice(key) else {
        return false;
    };
    mac.update(provided.as_bytes());
    mac.verify_slice(expected_hash).is_ok()
}

/// Compute HMAC-SHA256 of the token using the application key.
/// Mirrors Go hmacToken.
pub fn hmac_token(tok: &str, key: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC accepts keys of any size");
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
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn test_token_match() {
        let key = b"0123456789abcdef0123456789abcdef";
        let hash = hmac_token("my-secret", key);
        assert!(check_token("my-secret", &hash, key));
        assert!(!check_token("wrong-secret", &hash, key));
    }

    #[test]
    fn auth_mode_requires_complete_configuration() {
        let _lock = ENV_LOCK.lock().unwrap();
        let variables = [
            "OHC_AGENT_TOKEN",
            "OHC_AGENT_AUTH_KEY",
            "OHC_AGENT_SPIFFE_ID",
            "OHC_AGENT_AUTH_DISABLED",
            "OHC_ENV",
        ];

        temp_env::with_vars(variables.map(|name| (name, None::<&str>)), || {
            assert!(auth_mode_from_env().is_err());
        });
        temp_env::with_vars(
            [
                ("OHC_AGENT_TOKEN", Some("secret-token")),
                ("OHC_AGENT_AUTH_KEY", None),
                ("OHC_AGENT_SPIFFE_ID", None),
                ("OHC_AGENT_AUTH_DISABLED", None),
                ("OHC_ENV", None),
            ],
            || assert!(auth_mode_from_env().is_err()),
        );
        temp_env::with_vars(
            [
                ("OHC_AGENT_TOKEN", Some("secret-token")),
                (
                    "OHC_AGENT_AUTH_KEY",
                    Some("0123456789abcdef0123456789abcdef"),
                ),
                ("OHC_AGENT_SPIFFE_ID", None),
                ("OHC_AGENT_AUTH_DISABLED", None),
                ("OHC_ENV", None),
            ],
            || {
                assert!(matches!(
                    auth_mode_from_env().unwrap(),
                    AuthMode::Token { .. }
                ))
            },
        );
        temp_env::with_vars(
            [
                ("OHC_AGENT_TOKEN", None),
                ("OHC_AGENT_AUTH_KEY", None),
                (
                    "OHC_AGENT_SPIFFE_ID",
                    Some("spiffe://onehumancorp.io/org/org-1/agent/agent-1"),
                ),
                ("OHC_AGENT_AUTH_DISABLED", None),
                ("OHC_ENV", None),
            ],
            || {
                let error = auth_mode_from_env().unwrap_err();
                assert!(error.contains("mTLS"), "unexpected error: {error}");
            },
        );

        for environment in ["development", "test"] {
            temp_env::with_vars(
                [
                    ("OHC_AGENT_TOKEN", None),
                    ("OHC_AGENT_AUTH_KEY", None),
                    ("OHC_AGENT_SPIFFE_ID", None),
                    ("OHC_AGENT_AUTH_DISABLED", Some("true")),
                    ("OHC_ENV", Some(environment)),
                ],
                || assert!(matches!(auth_mode_from_env().unwrap(), AuthMode::Disabled)),
            );
        }
        temp_env::with_vars(
            [
                ("OHC_AGENT_TOKEN", None),
                ("OHC_AGENT_AUTH_KEY", None),
                ("OHC_AGENT_SPIFFE_ID", None),
                ("OHC_AGENT_AUTH_DISABLED", Some("true")),
                ("OHC_ENV", Some("production")),
            ],
            || assert!(auth_mode_from_env().is_err()),
        );
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
