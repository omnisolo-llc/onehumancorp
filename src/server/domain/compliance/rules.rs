use super::audit::ComplianceRule;

pub struct PiiLeakageRule;

impl ComplianceRule for PiiLeakageRule {
    fn verify_cloud(&self) -> bool {
        true
    }

    fn verify_standalone(&self) -> bool {
        true
    }
}
