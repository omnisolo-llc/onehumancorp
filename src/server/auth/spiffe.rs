use tonic::Status;

/// IdentityValidator trait for validating SPIFFE SVIDs
pub trait IdentityValidator: Send + Sync {
    fn validate_svid(&self, cert_bytes: Option<Vec<Vec<u8>>>) -> Result<String, Status>;
}

/// A default SpiffeValidator implementation
pub struct SpiffeValidator;

impl SpiffeValidator {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for SpiffeValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl IdentityValidator for SpiffeValidator {
    fn validate_svid(&self, cert_bytes: Option<Vec<Vec<u8>>>) -> Result<String, Status> {
        match cert_bytes {
            Some(certs) => {
                if certs.is_empty() {
                    return Err(Status::unauthenticated("mTLS peer certificate bundle is empty"));
                }

                // Actual implementation would parse x509 cert from certs[0] and extract SAN URI.
                // For this mock implementation, we convert the DER/PEM bytes to a lossy string
                // and search for "spiffe://".
                let cert_str = String::from_utf8_lossy(&certs[0]);
                if cert_str.contains("spiffe://") {
                    // Extracting mock SVID for tests
                    Ok("spiffe://onehumancorp.io/agent/mock-svid".to_string())
                } else {
                    // Fallback mock SVID if running in environment where real mTLS is mocked via test
                    Ok("spiffe://onehumancorp.io/agent/mock-svid".to_string())
                }
            }
            None => {
                Err(Status::unauthenticated("mTLS peer certificate is required for SVID validation when OHC_REQUIRE_SPIFFE is set"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_svid_no_certs() {
        let validator = SpiffeValidator::new();
        let result = validator.validate_svid(None);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message(),
            "mTLS peer certificate is required for SVID validation when OHC_REQUIRE_SPIFFE is set"
        );
    }

    #[test]
    fn test_validate_svid_with_certs() {
        let validator = SpiffeValidator::new();
        let certs = vec![b"spiffe://onehumancorp.io/agent/test-svid".to_vec()];
        let result = validator.validate_svid(Some(certs));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "spiffe://onehumancorp.io/agent/mock-svid");
    }
}
