use crate::services::onboarding::provisioner;
use crate::services::onboarding::wizard::InteractiveWizard;
use crate::services::onboarding::validation::ValidationEndpoint;
use crate::services::onboarding::audit;

pub fn run_day_one_setup(is_cloud: bool) -> Result<String, String> {
    // 1. Provision environment
    provisioner::provision_environment(is_cloud)?;

    // 2. Interactive setup
    let wizard = InteractiveWizard::new();
    let config = wizard.run_interactive_setup(is_cloud)?;

    // 3. Validate config
    let validator = ValidationEndpoint;
    validator.validate_config(&config)?;

    // 4. Generate audit report
    let report = audit::generate_audit_report(is_cloud);

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_run_day_one_setup_standalone() {
        let _ = fs::remove_dir_all(".ohc-local-data");

        let report = run_day_one_setup(false).unwrap();
        assert!(report.contains("PASSED"));
        assert!(report.contains("Standalone"));

        fs::remove_dir_all(".ohc-local-data").unwrap();
    }

    #[test]
    fn test_run_day_one_setup_cloud() {
        let _ = fs::remove_dir_all(".ohc-cloud-data");

        let num_cpus = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);

        let res = run_day_one_setup(true);
        if num_cpus < 2 {
            assert!(res.is_err());
            assert!(res.unwrap_err().contains("preflight check failed"));
        } else {
            let report = res.unwrap();
            assert!(report.contains("PASSED"));
            assert!(report.contains("Cloud-native"));
        }

        let _ = fs::remove_dir_all(".ohc-cloud-data");
    }
}
