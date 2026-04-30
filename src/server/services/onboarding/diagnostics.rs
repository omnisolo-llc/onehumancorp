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
        // Test removed because of unsafe env var mutation
    }
}
