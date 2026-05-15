use walkdir::WalkDir;
use std::fs;

#[test]
fn test_no_pii_leakage() {
    let mut failures = Vec::new();
    for entry in WalkDir::new("src/server") {
        let entry = entry.unwrap();
        if entry.path().is_file() && entry.path().extension().map_or(false, |ext| ext == "rs") {
            let content = fs::read_to_string(entry.path()).unwrap_or_default();
            for (line_idx, line) in content.lines().enumerate() {
                if (line.contains("tracing::info!") || line.contains("tracing::debug!") || line.contains("tracing::warn!") || line.contains("tracing::error!") || line.contains("tracing::trace!")) {
                    if line.contains("password") || line.contains("ssn") || line.contains("credit_card") {
                        failures.push(format!("{}:{}: {}", entry.path().display(), line_idx + 1, line.trim()));
                    }
                }
            }
        }
    }
    if !failures.is_empty() {
        panic!("PII leakage detected in logs:\n{}", failures.join("\n"));
    }
}
