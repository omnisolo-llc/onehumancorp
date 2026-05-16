use super::audit::{AuditContext, AuditResult, AuditStatus, ComplianceRule};

pub struct TelemetrySovereigntyRule;

impl ComplianceRule for TelemetrySovereigntyRule {
    fn id(&self) -> &str {
        "TEL_SOV_001"
    }

    fn description(&self) -> &str {
        "Ensures standalone wrapper has no non-consented telemetry"
    }

    fn evaluate(&self, context: &AuditContext) -> AuditResult {
        if !context.is_cloud && context.telemetry_enabled {
            return AuditResult {
                rule_id: self.id().to_string(),
                status: AuditStatus::Fail,
                details: "Standalone mode telemetry is enabled without explicit consent bypass".to_string(),
            };
        }

        AuditResult {
            rule_id: self.id().to_string(),
            status: AuditStatus::Pass,
            details: "Sovereignty maintained".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_telemetry_sovereignty_rule() {
        let rule = TelemetrySovereigntyRule;
        let ctx_pass = AuditContext {
            is_cloud: false,
            tenant_id: None,
            telemetry_enabled: false,
            config_dump: HashMap::new(),
        };
        assert_eq!(rule.evaluate(&ctx_pass).status, AuditStatus::Pass);

        let ctx_fail = AuditContext {
            is_cloud: false,
            tenant_id: None,
            telemetry_enabled: true,
            config_dump: HashMap::new(),
        };
        assert_eq!(rule.evaluate(&ctx_fail).status, AuditStatus::Fail);
    }
}
