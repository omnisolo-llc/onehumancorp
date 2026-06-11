use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::RwLock;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CapabilityProfile {
    pub allowed_capabilities: Vec<String>,
    pub denied_capabilities: Vec<String>,
}

pub trait SandboxViolationStore: Send + Sync {
    fn log_violation(&self, session_id: &str, capability: &str, tool_name: &str) -> Result<(), String>;
    #[allow(dead_code)]
    fn get_violations(&self, session_id: &str) -> Result<Vec<String>, String>;
}

pub struct InMemoryViolationStore {
    violations: RwLock<HashMap<String, Vec<String>>>,
}

impl InMemoryViolationStore {
    pub fn new() -> Self {
        InMemoryViolationStore {
            violations: RwLock::new(HashMap::new()),
        }
    }
}

impl SandboxViolationStore for InMemoryViolationStore {
    fn log_violation(&self, session_id: &str, capability: &str, tool_name: &str) -> Result<(), String> {
        let mut violations = self.violations.write().unwrap();
        let entry = format!(
            "[{}] Denied capability '{}' for tool '{}'",
            Utc::now().to_rfc3339(),
            capability,
            tool_name
        );
        violations.entry(session_id.to_string()).or_default().push(entry);
        
        let capability = capability.to_string();
        crate::record_telemetry(move || {
             tracing::warn!("Telemetry: Sandbox violation - capability_denied, capability={}", capability);
        });
        
        Ok(())
    }

    fn get_violations(&self, session_id: &str) -> Result<Vec<String>, String> {
        let violations = self.violations.read().unwrap();
        Ok(violations.get(session_id).cloned().unwrap_or_default())
    }
}

pub struct CapabilityAuthorizer {
    profiles: RwLock<HashMap<String, CapabilityProfile>>,
    violation_store: Box<dyn SandboxViolationStore>,
}

impl CapabilityAuthorizer {
    pub fn new(store: Option<Box<dyn SandboxViolationStore>>) -> Self {
        let store = store.unwrap_or_else(|| Box::new(InMemoryViolationStore::new()));
        CapabilityAuthorizer {
            profiles: RwLock::new(HashMap::new()),
            violation_store: store,
        }
    }

    pub fn set_profile(&self, session_id: String, profile: CapabilityProfile) {
        let mut profiles = self.profiles.write().unwrap();
        profiles.insert(session_id, profile);
    }

    pub fn authorize(&self, session_id: &str, capability: &str, tool_name: &str) -> Result<(), String> {
        let profiles = self.profiles.read().unwrap();
        let profile = profiles.get(session_id);

        if profile.is_none() {
            self.violation_store.log_violation(session_id, capability, tool_name)?;
            return Err(format!("capability {} denied: no profile for session {}", capability, session_id));
        }
        let profile = profile.unwrap();

        // Check explicit denies first
        for denied in &profile.denied_capabilities {
            if denied == capability || denied == "*" {
                self.violation_store.log_violation(session_id, capability, tool_name)?;
                return Err(format!("capability {} denied explicitly for session {}", capability, session_id));
            }
        }

        // Check allows
        for allowed in &profile.allowed_capabilities {
            if allowed == capability || allowed == "*" {
                return Ok(());
            }
        }

        // Implicit deny
        self.violation_store.log_violation(session_id, capability, tool_name)?;
        Err(format!("capability {} denied implicitly for session {}", capability, session_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_capability_authorizer() {
        let store = Box::new(InMemoryViolationStore::new());
        let authorizer = CapabilityAuthorizer::new(Some(store));

        let session_id = "test-session";
        let profile = CapabilityProfile {
            allowed_capabilities: vec!["read".to_string(), "write".to_string()],
            denied_capabilities: vec!["delete".to_string()],
        };

        authorizer.set_profile(session_id.to_string(), profile);

        assert!(authorizer.authorize(session_id, "read", "test_tool").is_ok());
        assert!(authorizer.authorize(session_id, "write", "test_tool").is_ok());
        
        let err = authorizer.authorize(session_id, "delete", "test_tool").unwrap_err();
        assert!(err.contains("denied explicitly"));

        let err = authorizer.authorize(session_id, "execute", "test_tool").unwrap_err();
        assert!(err.contains("denied implicitly"));

        let err = authorizer.authorize("non-existent", "read", "test_tool").unwrap_err();
        assert!(err.contains("no profile for session"));
    }
}
