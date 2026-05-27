use std::sync::Arc;
use serde_json::{json, Value};
use crate::telemetry::ViolationStore;
use dashmap::DashMap;

/// Represents the capability profile for a session.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct CapabilityProfile {
    /// Capabilities that are explicitly allowed.
    #[serde(default)]
    pub allowed_capabilities: Vec<String>,
    /// Capabilities that are explicitly denied.
    #[serde(default)]
    pub denied_capabilities: Vec<String>,
}

/// Authorizes capabilities and logs violations if access is denied.
pub struct CapabilityAuthorizer {
    violation_store: Arc<ViolationStore>,
    profiles: DashMap<String, CapabilityProfile>,
}

impl CapabilityAuthorizer {
    pub fn new(violation_store: Arc<ViolationStore>) -> Self {
        Self {
            violation_store,
            profiles: DashMap::new(),
        }
    }

    pub fn set_profile(&self, session_id: String, profile: CapabilityProfile) {
        self.profiles.insert(session_id, profile);
    }

    pub fn remove_profile(&self, session_id: &str) {
        self.profiles.remove(session_id);
    }

    /// Checks if a given capability is allowed.
    /// Denied capabilities take precedence.
    /// If a capability is neither in allowed nor denied list, it is considered denied by default
    /// (strict allow/deny capability system).
    pub async fn authorize(
        &self,
        tenant_id: &str,
        agent_id: &str,
        session_id: &str,
        capability: &str,
        action_details: Value,
    ) -> Result<(), String> {
        let (is_denied, is_allowed) = if let Some(profile) = self.profiles.get(session_id) {
            let is_denied = profile.denied_capabilities.iter().any(|c| c == capability);
            let is_allowed = profile.allowed_capabilities.iter().any(|c| c == capability);
            (is_denied, is_allowed)
        } else {
            // Default deny if no profile exists
            (false, false)
        };

        if is_denied || !is_allowed {
            let details = json!({
                "capability": capability,
                "action": action_details,
                "reason": if is_denied { "explicitly_denied" } else { "not_allowed" }
            });

            // Log the violation to the ViolationStore
            let _ = self.violation_store.record_violation(
                tenant_id,
                agent_id,
                session_id,
                "capability_denied",
                details,
            ).await;

            return Err(format!("Capability '{}' denied", capability));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_capability_allowed() {
        let store = Arc::new(ViolationStore::new(None));
        let authorizer = CapabilityAuthorizer::new(store);
        let profile = CapabilityProfile {
            allowed_capabilities: vec!["read".to_string(), "bash".to_string()],
            denied_capabilities: vec![],
        };
        authorizer.set_profile("session".to_string(), profile);

        let result = authorizer.authorize("tenant", "agent", "session", "read", json!({})).await;
        assert!(result.is_ok());

        let result2 = authorizer.authorize("tenant", "agent", "session", "bash", json!({})).await;
        assert!(result2.is_ok());
    }

    #[tokio::test]
    async fn test_capability_denied() {
        let store = Arc::new(ViolationStore::new(None));
        let authorizer = CapabilityAuthorizer::new(store);
        let profile = CapabilityProfile {
            allowed_capabilities: vec!["read".to_string(), "bash".to_string()],
            denied_capabilities: vec!["bash".to_string()],
        };
        authorizer.set_profile("session".to_string(), profile);

        // Explicitly denied (takes precedence)
        let result = authorizer.authorize("tenant", "agent", "session", "bash", json!({})).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Capability 'bash' denied");

        // Allowed
        let result2 = authorizer.authorize("tenant", "agent", "session", "read", json!({})).await;
        assert!(result2.is_ok());

        // Not explicitly allowed -> implicitly denied
        let result3 = authorizer.authorize("tenant", "agent", "session", "write", json!({})).await;
        assert!(result3.is_err());
        assert_eq!(result3.unwrap_err().to_string(), "Capability 'write' denied");
    }

    #[tokio::test]
    async fn test_no_profile_denied() {
        let store = Arc::new(ViolationStore::new(None));
        let authorizer = CapabilityAuthorizer::new(store);

        let result = authorizer.authorize("tenant", "agent", "session", "read", json!({})).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Capability 'read' denied");
    }
}
