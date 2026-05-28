use super::audit::ComplianceRule;

pub struct TelemetrySovereigntyRule;

impl ComplianceRule for TelemetrySovereigntyRule {
    fn verify_cloud(&self) -> bool {
        true
    }

    fn verify_standalone(&self) -> bool {
        true
    }
}
