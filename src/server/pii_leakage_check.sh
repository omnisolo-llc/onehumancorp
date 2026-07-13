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
    pii = r'\b(password|secret|key|token|auth|cookie|credential|email|phone|ssn|address|name|pii|jwt|bearer|sessionid|payload|credit|card|cvv|dob|birth|passport|bank|account|iban|stripe|billing|ipaddress|macaddress|geolocation|medical|health|salary|tax|socialsecurity|creditcard|deviceid|gps|latitude|longitude|tenant)\b'

    with open(filepath, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    matches = []
    for i, line in enumerate(lines):
        if re.search(r'(tracing::(info|debug|warn|error)!|println!|print!|eprintln!|dbg!|log::(info|debug|warn|error)!)\(', line):
            if re.search(r'redacted_', line, re.IGNORECASE):
                continue

            has_pii = False
            if re.search(r'\{.*?\}', line) and re.search(pii, line, re.IGNORECASE):
                has_pii = True
            elif re.search(pii + r'\s*=', line, re.IGNORECASE):
                has_pii = True

            if has_pii and '// pii-safe' not in line:
                matches.append((i+1, line.strip()))

    return matches

def main():
    src_dir = sys.argv[1]

    failed = False
    for root, _, files in os.walk(src_dir):
        for file in files:
            if file.endswith('.rs'):
                filepath = os.path.join(root, file)
                if '_test.rs' in filepath:
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
