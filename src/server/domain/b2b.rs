use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustAgreement {
    pub id: String,
    pub partner_org: String,
    pub partner_jwks_url: String,
    pub allowed_roles: Vec<String>,
    pub status: String, // PENDING, ACTIVE, REVOKED
}

pub struct TrustManager;

impl TrustManager {
    pub fn parse_jwks(&self, partner_org: &str, jwks_json: &str, allowed_roles: Vec<String>) -> Result<TrustAgreement, serde_json::Error> {
        let _jwks: HashMap<String, serde_json::Value> = serde_json::from_str(jwks_json)?;
        
        let id = format!("b2b-trust-{}", Utc::now().format("%Y%m%d%H%M%S%.3f"));
        
        Ok(TrustAgreement {
            id,
            partner_org: partner_org.to_string(),
            partner_jwks_url: jwks_json.to_string(),
            allowed_roles,
            status: "ACTIVE".to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2BMessage {
    pub content: String,
    pub cross_org: bool,
    pub blocked: bool,
}

pub struct EgressFilter;

impl EgressFilter {
    pub fn scan(&self, message: &str, keywords: &[String]) -> B2BMessage {
        let lower_msg = message.to_lowercase();
        let blocked = keywords.iter().any(|kw| lower_msg.contains(&kw.to_lowercase()));
        
        B2BMessage {
            content: message.to_string(),
            cross_org: true,
            blocked,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trust_manager_parse_jwks() {
        let tm = TrustManager;

        // Valid JSON
        let res = tm.parse_jwks("globex.com", r#"{"keys": [{"kty": "RSA"}]}"#, vec!["Sales Agent".to_string()]);
        assert!(res.is_ok());
        let got = res.unwrap();
        assert_eq!(got.status, "ACTIVE");
        assert_eq!(got.partner_org, "globex.com");
        assert!(got.id.starts_with("b2b-trust-"));

        // Invalid JSON
        let res = tm.parse_jwks("bad-org.com", "{bad json}", vec![]);
        assert!(res.is_err());
    }

    #[test]
    fn test_egress_filter_scan() {
        let ef = EgressFilter;
        let keywords = vec!["Internal Project X".to_string(), "Confidential".to_string()];

        // Not blocked
        let got = ef.scan("Hello, we would like to purchase 100 server racks.", &keywords);
        assert!(!got.blocked);
        assert!(got.cross_org);
        assert_eq!(got.content, "Hello, we would like to purchase 100 server racks.");

        // Blocked (exact)
        let got = ef.scan("We can offer a discount based on Internal Project X budget.", &keywords);
        assert!(got.blocked);

        // Blocked (case-insensitive)
        let got = ef.scan("This is very CONFIDENTIAL information.", &keywords);
        assert!(got.blocked);
    }
}
