use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditStatus {
    Pass,
    Fail,
    Warning,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditResult {
    pub rule_id: String,
    pub status: AuditStatus,
    pub details: String,
}

pub trait ComplianceRule {
    fn id(&self) -> &str;
    fn description(&self) -> &str;
    fn evaluate(&self, context: &AuditContext) -> AuditResult;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditContext {
    pub is_cloud: bool,
    pub tenant_id: Option<String>,
    pub telemetry_enabled: bool,
    pub config_dump: HashMap<String, String>,
}

pub struct PrivacyAuditor {
    rules: Vec<Box<dyn ComplianceRule + Send + Sync>>,
}

impl PrivacyAuditor {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: Box<dyn ComplianceRule + Send + Sync>) {
        self.rules.push(rule);
    }

    pub fn run_audit(&self, context: &AuditContext) -> Vec<AuditResult> {
        self.rules.iter().map(|rule| rule.evaluate(context)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_auditor() {
        let auditor = PrivacyAuditor::new();
        let ctx = AuditContext {
            is_cloud: true,
            tenant_id: Some("t1".to_string()),
            telemetry_enabled: false,
            config_dump: HashMap::new(),
        };
        let res = auditor.run_audit(&ctx);
        assert!(res.is_empty());
    }
}
