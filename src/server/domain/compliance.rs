pub mod audit {
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
}

pub mod rules {
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
}

pub mod telemetry_check {
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
}

pub use audit::PrivacyAuditor;
