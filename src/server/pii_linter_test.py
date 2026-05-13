import os
import sys
import re

def check_file(filepath):
    violations = 0
    with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
        content = f.read()

    # Find all logging blocks
    # Added (?i) for case insensitivity and improved regex to catch JS/Go logs that might lack semicolons
    log_pattern = re.compile(r'(?i)(tracing::|info!|error!|warn!|debug!|println!|log\.print|fmt\.error|fmt\.print|console\.log|console\.error|console\.warn|console\.info|console\.debug).*?(?:;|\n)')
    matches = log_pattern.finditer(content)

    pii_keywords = ["tenant_id", "organization_id", "org_id", "session_data", "session_id", "payload", "email", "password", "pii", "api_key", "secret_key", "credit", "card", "cvv", "dob", "birth", "passport", "bank", "account", "stripe", "billing", "ip_address", "mac_address", "geolocation"]

    for match in matches:
        log_block = match.group(0).lower()
        for kw in pii_keywords:
            if kw in log_block:
                print(f"Violation in {filepath}: {log_block.strip()}")
                violations += 1
                break

    return violations

def main():
    if not os.path.exists("src/server"):
        # When running in bazel, the path might be different, try to look for the current file
        pass

    search_dir = "."
    if os.path.exists("src/server"):
        search_dir = "src/server"

    total_violations = 0
    checked_files = 0
    for root, _, files in os.walk(search_dir):
        for file in files:
            if file.endswith((".rs", ".go", ".ts")) and file != "telemetry_test.rs" and file != "pii_linter_test.py":
                filepath = os.path.join(root, file)
                total_violations += check_file(filepath)
                checked_files += 1

    print(f"Checked {checked_files} files.")
    if total_violations > 0:
        print(f"Found {total_violations} PII logging violations.")
        sys.exit(1)
    else:
        print("No PII logging violations found.")
        sys.exit(0)

if __name__ == '__main__':
    main()
