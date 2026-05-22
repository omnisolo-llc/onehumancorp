use std::thread;

pub struct PreflightResult {
    pub os: String,
    pub arch: String,
    pub num_cpus: usize,
    pub passed: bool,
    pub message: String,
}

pub fn run_preflight_check(is_cloud: bool) -> PreflightResult {
    let os = std::env::consts::OS.to_string();
    let arch = std::env::consts::ARCH.to_string();
    let num_cpus = thread::available_parallelism().map(|n| n.get()).unwrap_or(1);

    let mut res = PreflightResult {
        os,
        arch,
        num_cpus,
        passed: true,
        message: "System meets minimum requirements.".to_string(),
    };

    if is_cloud && res.num_cpus < 2 {
        res.passed = false;
        res.message = "Cloud-native mode requires at least 2 CPUs.".to_string();
    }
    res
}

pub fn generate_preflight_report(res: &PreflightResult) -> String {
    let status = if res.passed { "PASSED" } else { "FAILED" };

    format!(
        "<div style=\"backdrop-filter: blur(30px) saturate(210%); background: rgba(255, 255, 255, 0.65); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px;\">\n\
          <h2>Day One Preflight Checker</h2>\n\
          <p><strong>OS:</strong> {}</p>\n\
          <p><strong>Arch:</strong> {}</p>\n\
          <p><strong>CPUs:</strong> {}</p>\n\
          <p><strong>Status:</strong> {}</p>\n\
          <p><strong>Message:</strong> {}</p>\n\
        </div>",
        res.os, res.arch, res.num_cpus, status, res.message
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_preflight_check() {
        let res = run_preflight_check(false);
        assert!(res.passed); // Standalone should pass

        let res_cloud = run_preflight_check(true);
        if res_cloud.num_cpus < 2 {
            assert!(!res_cloud.passed);
        }
    }

    #[test]
    fn test_generate_preflight_report() {
        let res = PreflightResult {
            os: "linux".to_string(),
            arch: "amd64".to_string(),
            num_cpus: 4,
            passed: true,
            message: "System meets minimum requirements.".to_string(),
        };

        let report = generate_preflight_report(&res);

        assert!(report.contains("Day One Preflight Checker"));
        assert!(report.contains("blur(20px)"));
        assert!(report.contains("PASSED"));
        assert!(report.contains("linux"));
    }
}
