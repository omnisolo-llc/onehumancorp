#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn test_no_pii_logging_statements() {
        use walkdir::WalkDir;
        use std::fs;
        use std::env;
        use std::path::PathBuf;

        let mut violations = Vec::new();

        let mut search_dirs = vec![PathBuf::from(".")];
        // Try multiple possible source locations
        let possible_src_roots = vec![
            PathBuf::from("src"),
            PathBuf::from("src/server"),
        ];
        if let Ok(runfiles_dir) = env::var("RUNFILES_DIR") {
            let runfiles = PathBuf::from(&runfiles_dir);
            // In bazel runfiles, the manifest is at RUNFILES_DIR/MANIFEST.txt
            // The actual source files are symlinked in the runfiles directory
            // We need to find where the src directory actually is
            for entry in std::fs::read_dir(&runfiles).into_iter().flatten().flatten() {
                let path = entry.path();
                if path.is_dir() && path.file_name().map_or(false, |n| n == "src") {
                    search_dirs.push(path);
                }
            }
            // Also try workspace name prefix (common pattern)
            if let Ok(workspace) = env::var("TEST_WORKSPACE") {
                let prefixed = runfiles.join(&workspace).join("src");
                if prefixed.exists() {
                    search_dirs.push(prefixed);
                }
            }
        }
        for src_root in possible_src_roots {
            if src_root.exists() {
                search_dirs.push(src_root);
            }
        }

        let mut checked_files = 0;

        for dir in &search_dirs {
            if dir.exists() {
                let walker = WalkDir::new(&dir).into_iter().filter_entry(|e| {
                    e.path().components().all(|c| c.as_os_str() != "external")
                });

                for entry in walker
                    .filter_map(Result::ok)
                    .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs" || ext == "go" || ext == "ts"))
                {
                    let path_str = entry.path().to_string_lossy();
                    if path_str.contains("telemetry_test.rs") || path_str.contains("compliance_guardrails_test.rs") {
                        continue;
                    }
                    checked_files += 1;
                    let content = fs::read_to_string(entry.path()).unwrap_or_default();
                    let mut in_log_block = false;
                    let mut current_log_block = String::new();
                    let mut block_start_line = 0;
                    let mut paren_count = 0;

                    for (i, line) in content.lines().enumerate() {
                        let lower_line = line.to_lowercase();

                        if !in_log_block {
                            if lower_line.contains("tracing::info!") ||
                               lower_line.contains("etracing::info!") ||
                               lower_line.contains("info!") ||
                               lower_line.contains("error!") ||
                               lower_line.contains("warn!") ||
                               lower_line.contains("debug!") ||
                               lower_line.contains("tracing::") ||
                               lower_line.contains("println!") ||
                               lower_line.contains("log.print") ||
                               lower_line.contains("fmt.errorf") || lower_line.contains("fmt.error") || lower_line.contains("log.printf") || lower_line.contains("fmt.print") ||
                               lower_line.contains("console.log") || lower_line.contains("console.error") || lower_line.contains("console.warn") || lower_line.contains("console.info") || lower_line.contains("console.debug") ||
                               lower_line.contains("eprintln!")
                            {
                                in_log_block = true;
                                block_start_line = i + 1;
                                current_log_block.clear();
                                current_log_block.push_str(&lower_line);
                                paren_count = 0;

                                paren_count += lower_line.chars().filter(|c| *c == '(' || *c == '{').count() as i32;
                                paren_count -= lower_line.chars().filter(|c| *c == ')' || *c == '}').count() as i32;

                                // In case the statement is entirely on one line with no parens or perfectly balanced
                                if paren_count <= 0 && (lower_line.contains(")") || lower_line.contains("}") || lower_line.ends_with(";")) {
                                    in_log_block = false;
                                }
                            }
                        } else {
                            current_log_block.push_str(" ");
                            current_log_block.push_str(&lower_line);

                            paren_count += lower_line.chars().filter(|c| *c == '(' || *c == '{').count() as i32;
                            paren_count -= lower_line.chars().filter(|c| *c == ')' || *c == '}').count() as i32;

                            if paren_count <= 0 || lower_line.ends_with(");") || lower_line.ends_with("};") {
                                in_log_block = false;
                            }
                        }

                        // Process the complete block once it's closed, OR if it was a single line
                        if !in_log_block && !current_log_block.is_empty() {
                            if current_log_block.contains("tenant_id") ||
                               current_log_block.contains("organization_id") ||
                               current_log_block.contains("org_id") ||
                               current_log_block.contains("session_data") ||
                               current_log_block.contains("session_id") ||
                               current_log_block.contains("payload") ||
                               current_log_block.contains("email") ||
                               current_log_block.contains("password") ||
                               current_log_block.contains("pii") ||
                               current_log_block.contains("api_key") ||
                               current_log_block.contains("secret_key") ||
                               current_log_block.contains("credit") ||
                               current_log_block.contains("card") ||
                               current_log_block.contains("cvv") ||
                               current_log_block.contains("dob") ||
                               current_log_block.contains("birth") ||
                               current_log_block.contains("passport") ||
                               current_log_block.contains("bank") ||
                               current_log_block.contains("account") ||
                               current_log_block.contains("stripe") ||
                               current_log_block.contains("billing") ||
                               current_log_block.contains("ip_address") ||
                               current_log_block.contains("mac_address") ||
                               current_log_block.contains("geolocation") {
                                violations.push(format!("{}:{} (block starting here): {}", entry.path().display(), block_start_line, current_log_block.trim()));
                            }
                            current_log_block.clear();
                        }
                    }
                }
            }
        }

        let search_dirs_for_error = search_dirs.clone();
        if checked_files == 0 {
            // No files found to check - likely running in an environment where source files
            // are not accessible (e.g., some bazel sandboxes). Skip the test gracefully.
            println!("PII test skipped: Could not find any .rs files. Search dirs: {:?}", search_dirs_for_error);
            return;
        }
        assert!(
            violations.is_empty(),
            "Found PII logging violations in the following lines:\n{:#?}",
            violations
        );
    }
}

#[test]
fn test_standalone_wrapper_audit() {
    let mut script_path = std::path::PathBuf::from("deploy/scripts/ohc-standalone.sh");
    if let Ok(workspace_dir) = std::env::var("BUILD_WORKSPACE_DIRECTORY") {
        script_path = std::path::PathBuf::from(workspace_dir).join("deploy/scripts/ohc-standalone.sh");
    } else if let Ok(runfiles_dir) = std::env::var("RUNFILES_DIR") {
        script_path = std::path::PathBuf::from(runfiles_dir).join("ohc/deploy/scripts/ohc-standalone.sh");
    }
    if !script_path.exists() {
        script_path = std::path::PathBuf::from("deploy/scripts/ohc-standalone.sh");
    }
    let content = std::fs::read_to_string(script_path).expect("Failed to read ohc-standalone.sh script");

    let expected_telemetry_check = r#"if [ "$OHC_TELEMETRY_ENABLED" != "true" ]; then
  export OHC_TELEMETRY_ENABLED=false
fi"#;

    assert!(
        content.contains(expected_telemetry_check),
        "Local Sovereignty violation: ohc-standalone.sh does not properly strictly enforce OHC_TELEMETRY_ENABLED opt-in boundary."
    );
}

#[test]
fn test_redact_interface_pii_malicious_payloads() {
    let payload = serde_json::json!({
        "payload": {
            "credit_card": "4111-1111-1111-1111",
            "cvv": "123",
            "dob": "1990-01-01",
            "passport_number": "A1234567",
            "bank_account": "123456789",
            "stripe_token": "tok_123456789",
            "billing_address": "123 Main St, Anytown USA",
            "ssn": "123-45-6789",
            "phone_number": "555-123-4567",
            "email_address": "malicious@example.com",
            "tenant_id": "tenant-123",
            "organization_id": "org-456",
            "session_id": "session-789",
            "ip_address": "192.168.1.1",
            "mac_address": "00:1B:44:11:3A:B7",
            "geolocation": "37.7749,-122.4194",
        },
        "nested": {
            "deep": {
                "secret_key": "sk-1234567890",
                "api_key": "ak-0987654321",
                "auth_token": "Bearer token",
                "password_hash": "hash",
                "cookie_session": "cookie",
                "credential_id": "cred-1",
            }
        },
        "array_of_evil": [
            { "name": "John Doe", "email": "john@doe.com" },
            { "address": "456 Elm St", "phone": "555-987-6543" }
        ],
        "safe_field": "This should not be redacted",
        "another_safe": 123
    });

    let redacted = crate::telemetry::redact_interface_pii(payload);

    // Verify root level safe fields
    assert_eq!(redacted["safe_field"], "This should not be redacted");
    assert_eq!(redacted["another_safe"], 123);

    // Because the key is "payload", the entire object gets redacted to "[REDACTED]"
    assert_eq!(redacted["payload"], "[REDACTED]");
    // Added explicitly nested checks are hidden by payload redaction, but if we moved them, they would be redacted.

    // Verify deeply nested secret redactions
    assert_eq!(redacted["nested"]["deep"]["secret_key"], "[REDACTED]");
    assert_eq!(redacted["nested"]["deep"]["api_key"], "[REDACTED]");
    assert_eq!(redacted["nested"]["deep"]["auth_token"], "[REDACTED]");
    assert_eq!(redacted["nested"]["deep"]["password_hash"], "[REDACTED]");
    assert_eq!(redacted["nested"]["deep"]["cookie_session"], "[REDACTED]");
    assert_eq!(redacted["nested"]["deep"]["credential_id"], "[REDACTED]");

    // Verify array redactions
    assert_eq!(redacted["array_of_evil"][0]["name"], "[REDACTED]");
    assert_eq!(redacted["array_of_evil"][0]["email"], "[REDACTED]");
    assert_eq!(redacted["array_of_evil"][1]["address"], "[REDACTED]");
    assert_eq!(redacted["array_of_evil"][1]["phone"], "[REDACTED]");
}
