use std::path::Path;

pub struct DiagnosticsResult {
    pub passed: bool,
    pub details: Vec<String>,
}

pub fn run_diagnostics() -> DiagnosticsResult {
    let mut result = DiagnosticsResult {
        passed: true,
        details: Vec::new(),
    };

    let runtime_dir = std::env::var("OHC_RUNTIME_DIR")
        .unwrap_or_else(|_| ".ohc/runtime".to_string());
        
    let memory_dir = std::env::var("OHC_MEMORY_DIR")
        .unwrap_or_else(|_| format!("{}/memory", runtime_dir));
        
    let status_dir = std::env::var("OHC_STATUS_DIR")
        .unwrap_or_else(|_| format!("{}/status", runtime_dir));

    let required_paths = vec![runtime_dir, memory_dir, status_dir];

    for path in required_paths {
        if !Path::new(&path).exists() {
            result.passed = false;
            result.details.push(format!("Missing required path: {}", path));
        } else {
            result.details.push(format!("Found required path: {}", path));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_run_diagnostics() {
        let temp_dir = ".ohc-test-diagnostics";
        let _ = fs::remove_dir_all(temp_dir);

        let runtime_dir = format!("{}/.ohc/runtime", temp_dir);
        let memory_dir = format!("{}/.ohc/runtime/memory", temp_dir);
        let status_dir = format!("{}/.ohc/runtime/status", temp_dir);

        temp_env::with_vars([
            ("OHC_RUNTIME_DIR", Some(runtime_dir.as_str())),
            ("OHC_MEMORY_DIR", Some(memory_dir.as_str())),
            ("OHC_STATUS_DIR", Some(status_dir.as_str()))
        ], || {
            // Scenario 1: All paths are missing
            let res = run_diagnostics();
            assert!(!res.passed);
            assert_eq!(res.details.len(), 3);
            for detail in &res.details {
                assert!(detail.contains("Missing required path"));
            }

            // Create required paths
            fs::create_dir_all(&runtime_dir).unwrap();
            fs::create_dir_all(&memory_dir).unwrap();
            fs::create_dir_all(&status_dir).unwrap();

            // Scenario 2: All paths exist
            let res = run_diagnostics();
            assert!(res.passed);
            assert_eq!(res.details.len(), 3);
            for detail in &res.details {
                assert!(detail.contains("Found required path"));
            }

            fs::remove_dir_all(temp_dir).unwrap();
        });
    }
}
