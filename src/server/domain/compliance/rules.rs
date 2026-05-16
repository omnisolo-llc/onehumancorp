use super::audit::{AuditContext, AuditResult, AuditStatus, ComplianceRule};

pub struct PiiLeakageRule;

impl ComplianceRule for PiiLeakageRule {
    fn id(&self) -> &str {
        "PII_LEAK_001"
    }

    fn description(&self) -> &str {
        "Checks for obvious PII leakage configurations"
    }

    fn evaluate(&self, context: &AuditContext) -> AuditResult {
        if context.is_cloud && context.tenant_id.is_none() {
            return AuditResult {
                rule_id: self.id().to_string(),
                status: AuditStatus::Fail,
                details: "Cloud mode active but no tenant isolation provided".to_string(),
            };
        }

        if let Some(val) = context.config_dump.get("LOG_PII") {
            if val == "true" {
                return AuditResult {
                    rule_id: self.id().to_string(),
                    status: AuditStatus::Fail,
                    details: "LOG_PII is enabled, violating privacy by design".to_string(),
                };
            }
        }

        AuditResult {
            rule_id: self.id().to_string(),
            status: AuditStatus::Pass,
            details: "No obvious PII leakage vectors detected".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_pii_leakage_rule_pass() {
        let rule = PiiLeakageRule;
        let mut ctx = AuditContext {
            is_cloud: true,
            tenant_id: Some("t1".to_string()),
            telemetry_enabled: false,
            config_dump: HashMap::new(),
        };
        assert_eq!(rule.evaluate(&ctx).status, AuditStatus::Pass);

        ctx.config_dump.insert("LOG_PII".to_string(), "false".to_string());
        assert_eq!(rule.evaluate(&ctx).status, AuditStatus::Pass);
    }

    #[test]
    fn test_pii_leakage_rule_fail() {
        let rule = PiiLeakageRule;
        let mut ctx = AuditContext {
            is_cloud: true,
            tenant_id: Some("t1".to_string()),
            telemetry_enabled: false,
            config_dump: HashMap::new(),
        };
        ctx.config_dump.insert("LOG_PII".to_string(), "true".to_string());
        assert_eq!(rule.evaluate(&ctx).status, AuditStatus::Fail);
    }
}
