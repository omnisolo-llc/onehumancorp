use tonic::{Request, Status};



#[derive(Debug, Clone)]
enum AuthMode {
    SPIFFE { allowed_id: Option<String> },
}

#[derive(Debug, Clone)]
pub struct AuthConfig {
    mode: AuthMode,
}

impl AuthConfig {
    pub fn from_env() -> Self {
        // Enforce SPIFFE mode (Zero Secrets)
        AuthConfig {
            mode: AuthMode::SPIFFE {
                allowed_id: std::env::var("OHC_AGENT_SPIFFE_ID").ok(),
            },
        }
    }

    pub fn authenticate(&self, req: &Request<()>) -> Result<(), Status> {
        match &self.mode {
            AuthMode::SPIFFE { allowed_id } => self.check_spiffe(req, allowed_id.as_deref()),
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

pub fn validate_spiffe_id(id: &str) -> Result<(), Status> {
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

    // Expected format: spiffe://<domain>/org/<org_id>/agent/<agent_id>
    // Parts: ["<domain>", "org", "<org_id>", "agent", "<agent_id>"]
    let parts: Vec<&str> = trimmed.split('/').collect();
    if parts.len() != 5 {
        return Err(Status::permission_denied(format!("SPIFFE ID must exactly match spiffe://<domain>/org/<org_id>/agent/<agent_id>: {}", id)));
    }
    if parts[1] != "org" || parts[3] != "agent" {
        return Err(Status::permission_denied(format!("SPIFFE ID must contain /org/<org_id>/agent/<agent_id> structure: {}", id)));
    }
    if parts[2].is_empty() {
        return Err(Status::permission_denied(format!("SPIFFE ID org_id cannot be empty: {}", id)));
    }
    if parts[4].is_empty() {
        return Err(Status::permission_denied(format!("SPIFFE ID agent_id cannot be empty: {}", id)));
    }

    let domain = parts[0];
    
    match domain {
        "onehumancorp.io" | "ohc.local" | "ohc.os" | "ohc.global" => {}
        _ if domain.ends_with(".ohc.global") => {}
        _ => return Err(Status::permission_denied(format!("untrusted SPIFFE domain {:?} in {}", domain, id))),
    }
    
    Ok(())
}

pub fn interceptor(cfg: AuthConfig) -> impl Fn(Request<()>) -> Result<Request<()>, Status> + Clone {
    move |req: Request<()>| {
        cfg.authenticate(&req)?;
        Ok(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn test_validate_spiffe_id() {
        assert!(validate_spiffe_id("spiffe://onehumancorp.io/org/org-1/agent/agent-1").is_ok());
        assert!(validate_spiffe_id("spiffe://onehumancorp.io/org/org-2/agent/agent-2").is_ok());
        assert!(validate_spiffe_id("spiffe://onehumancorp.io/org/org-3/agent/agent-3").is_ok());
        assert!(validate_spiffe_id("spiffe://us-east.ohc.global/org/org-4/agent/agent-4").is_ok());

        assert!(validate_spiffe_id("invalid").is_err());
        assert!(validate_spiffe_id("spiffe://invalid.com/x").is_err());
        assert!(validate_spiffe_id("spiffe://onehumancorp.io/../bad").is_err());
        assert!(validate_spiffe_id("spiffe://onehumancorp.io/org-1/agent-1").is_err()); // Missing /org/ and /agent/ structure
        assert!(validate_spiffe_id("spiffe://onehumancorp.io/org//agent/agent-1").is_err()); // Empty org_id
        assert!(validate_spiffe_id("spiffe://onehumancorp.io/org/org-1/agent/").is_err()); // Empty agent_id
    }

    #[test]
    fn parse_spiffe_id_rejects_empty_and_untrusted_identities() {
        for id in [
            "spiffe://evil.example/org/acme/agent/a1",
            "spiffe://onehumancorp.io/org//agent/a1",
            "spiffe://onehumancorp.io/org/acme/agent/",
            "spiffe://onehumancorp.io/org/acme/agent/a1/extra",
            "spiffe://onehumancorp.io/org/acme%2Fother/agent/a1",
        ] {
            assert!(crate::parse_spiffe_id(id).is_err(), "accepted {id}");
        }
    }

}
