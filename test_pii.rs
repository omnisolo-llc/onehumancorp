use walkdir::WalkDir;
use std::fs;

fn main() {
    let mut failures = Vec::new();
    for entry in WalkDir::new("src") {
        let entry = entry.unwrap();
        if entry.path().is_file() && entry.path().extension().map_or(false, |ext| ext == "rs") {
            let content = fs::read_to_string(entry.path()).unwrap_or_default();
            for (line_idx, line) in content.lines().enumerate() {
                if line.contains("tracing::") {
                    let lower_line = line.to_lowercase();
                    if lower_line.contains("password") || lower_line.contains("ssn") || lower_line.contains("credit_card") || lower_line.contains("pii") {
                        failures.push(format!("{}:{}: {}", entry.path().display(), line_idx + 1, line.trim()));
                    }
                }
            }
        }
    }
    if !failures.is_empty() {
        println!("PII leakage detected in logs:\n{}", failures.join("\n"));
    } else {
        println!("No PII leakage detected.");
    }
}
