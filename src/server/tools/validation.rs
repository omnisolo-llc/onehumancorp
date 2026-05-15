
use serde::{Deserialize, Serialize};

/// Validates integration payloads against standard OWASP and data hygiene rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationValidator {
    pub max_payload_size: usize,
    pub allowed_regions: Vec<String>,
}

impl IntegrationValidator {
    pub fn new(max_payload_size: usize, regions: Vec<&str>) -> Self {
        Self {
            max_payload_size,
            allowed_regions: regions.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn is_valid_payload(&self, payload: &str) -> bool {
        if payload.len() > self.max_payload_size {
            return false;
        }

        if payload.contains("<script>") {
            return false; // Basic XSS prevention
        }

        true
    }
}

#[cfg(test)]
mod validation_test;
