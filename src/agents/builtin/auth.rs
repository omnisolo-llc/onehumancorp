use std::env;

/// Authentication mode.
#[derive(Debug, Clone)]
pub enum AuthMode {
    /// SPIFFE/mTLS peer certificate.
    Spiffe { allowed_id: String },
}

/// Build an AuthMode from environment variables.
///
///   OHC_AGENT_SPIFFE_ID            – restricts SPIFFE ID (enables SPIFFE mode)
pub fn auth_mode_from_env() -> AuthMode {
    AuthMode::Spiffe {
        allowed_id: env::var("OHC_AGENT_SPIFFE_ID").unwrap_or_default(),
    }
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
