#!/bin/bash
set -euo pipefail

# This script is run via Bazel sh_test to ensure no PII leakage in logging statements

EXIT_CODE=0

# Check if SRCDIR is set (Bazel), otherwise use src/server
SRCDIR=${1:-src/server}

python3 -c "
import os
import re
import sys

def check_file(filepath):
    # Added organization and org to PII keywords
    pii = r'\b(password|secret|key|token|auth|cookie|credential|email|phone|ssn|address|name|pii|jwt|bearer|sessionid|payload|credit|card|cvv|dob|birth|passport|bank|account|stripe|billing|ipaddress|macaddress|geolocation|medical|health|salary|tax|socialsecurity|creditcard|deviceid|gps|latitude|longitude|tenant|organization|org)\b'

    # Read the file
    with open(filepath, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    matches = []

    is_rust = filepath.endswith('.rs')
    is_ts_js = filepath.endswith(('.ts', '.tsx', '.js', '.jsx'))

    for i, line in enumerate(lines):
        # We want to check for logging patterns depending on file type
        has_log = False
        if is_rust and re.search(r'(tracing::(info|debug|warn|error)!|println!|print!|eprintln!|dbg!|log::(info|debug|warn|error)!)\(', line):
            has_log = True
        elif is_ts_js and re.search(r'(console\.(log|warn|error|info|debug)|logger\.(info|debug|warn|error|log))\s*\(', line):
            has_log = True

        if has_log:
            if re.search(r'redacted_', line, re.IGNORECASE):
                continue

            # If line has '// pii-safe', it's safe.
            if '// pii-safe' in line:
                continue

            # Remove safe IDs
            line_without_safe_ids = re.sub(r'\b(organization_id|tenant_id|org_id|org context)\b', '', line, flags=re.IGNORECASE)

            has_pii_strict = False
            if re.search(r'\{.*?\}', line_without_safe_ids) and re.search(pii, line_without_safe_ids, re.IGNORECASE):
                has_pii_strict = True
            elif is_ts_js and re.search(r'\$\{.*?\}', line_without_safe_ids) and re.search(pii, line_without_safe_ids, re.IGNORECASE):
                has_pii_strict = True
            elif re.search(pii + r'\s*=', line_without_safe_ids, re.IGNORECASE):
                has_pii_strict = True
            elif re.search(pii + r'\s*:', line_without_safe_ids, re.IGNORECASE):
                has_pii_strict = True

            if has_pii_strict:
                matches.append((i+1, line.strip()))

    return matches

def main():
    src_dir = sys.argv[1]

    failed = False
    for root, _, files in os.walk(src_dir):
        for file in files:
            if file.endswith(('.rs', '.ts', '.tsx', '.js', '.jsx')):
                filepath = os.path.join(root, file)
                if '_test.rs' in filepath or '.test.' in filepath or '.spec.' in filepath:
                    continue
                # Also ignore build output directories like node_modules, dist, etc.
                if 'node_modules' in filepath or 'next_out' in filepath or 'out' in filepath or 'dist' in filepath or '.next' in filepath or 'coverage' in filepath:
                    continue

                matches = check_file(filepath)
                if matches:
                    print(f'FAIL: Potential PII leakage in logging found in {filepath}')
                    for line_num, line in matches:
                        print(f'{line_num}: {line}')
                    failed = True

    if failed:
        print('Failing test due to PII leak')
        sys.exit(1)
    else:
        print('PASS: No obvious PII leakage found in tracing logs.')
        sys.exit(0)

if __name__ == '__main__':
    main()
" "$SRCDIR"
