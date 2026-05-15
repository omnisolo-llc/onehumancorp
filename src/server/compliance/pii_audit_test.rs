use walkdir::WalkDir;
use std::fs;

#[test]
fn audit_tracing_for_pii_leakage() {
    let mut failures = Vec::new();
    let restricted_terms = vec!["password", "ssn", "credit_card", "pii"];

    for entry in WalkDir::new("src") {
        let entry = entry.expect("Failed to read directory entry");
        if entry.path().is_file() && entry.path().extension().map_or(false, |ext| ext == "rs") {
            let content = fs::read_to_string(entry.path()).unwrap_or_default();
            for (line_idx, line) in content.lines().enumerate() {
                if line.contains("tracing::") {
                    let lower_line = line.to_lowercase();
                    for term in &restricted_terms {
                        if lower_line.contains(term) {
                            failures.push(format!("{}:{}: contains restricted term '{}': {}", entry.path().display(), line_idx + 1, term, line.trim()));
                        }
                    }
                }
            }
        }
    }

    if !failures.is_empty() {
        panic!("PII leakage or restricted terms detected in tracing statements:\n{}", failures.join("\n"));
    }
}
