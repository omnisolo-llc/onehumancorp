use super::rules::PiiLeakageRule;
use super::telemetry_check::TelemetrySovereigntyRule;

pub trait ComplianceRule {
    fn verify_cloud(&self) -> bool;
    fn verify_standalone(&self) -> bool;
}

pub struct PrivacyAuditor {
    rules: Vec<Box<dyn ComplianceRule>>,
}

impl PrivacyAuditor {
    pub fn new() -> Self {
        Self {
            rules: vec![
                Box::new(PiiLeakageRule {}),
                Box::new(TelemetrySovereigntyRule {}),
            ],
        }
    }

    pub fn audit_cloud(&self) -> bool {
        self.rules.iter().all(|r| r.verify_cloud())
    }

    pub fn audit_standalone(&self) -> bool {
        self.rules.iter().all(|r| r.verify_standalone())
    }
}
