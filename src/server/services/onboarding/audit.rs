use crate::services::onboarding::provisioner;

pub fn generate_audit_report(is_cloud: bool) -> String {
    let err = provisioner::check_environment(is_cloud);
    let status = if err.is_ok() { "PASSED" } else { "FAILED" };
    let details = match err {
        Ok(_) => "All required directories are present.".to_string(),
        Err(e) => e,
    };

    let mode = if is_cloud { "Cloud-native" } else { "Standalone" };

    format!(
        "<div style=\"backdrop-filter: blur(30px) saturate(210%); background: rgba(255, 255, 255, 0.65); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px;\">\n\
          <h2>Day One Audit Report ({})</h2>\n\
          <p><strong>Status:</strong> {}</p>\n\
          <p><strong>Details:</strong> {}</p>\n\
        </div>",
        mode, status, details
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::onboarding::provisioner;
    use std::fs;

    #[test]
    fn test_generate_audit_report_passed() {
        let _ = fs::remove_dir_all(".ohc-local-data");
        provisioner::provision_environment(false).unwrap();

        let report = generate_audit_report(false);
        assert!(report.contains("PASSED"));
        assert!(report.contains("backdrop-filter: blur(30px) saturate(210%)"));

        fs::remove_dir_all(".ohc-local-data").unwrap();
    }

    #[test]
    fn test_generate_audit_report_failed() {
        let _ = fs::remove_dir_all(".ohc-cloud-data");

        let report = generate_audit_report(true);
        assert!(report.contains("FAILED"));
        assert!(report.contains("does not exist"));
    }
}
