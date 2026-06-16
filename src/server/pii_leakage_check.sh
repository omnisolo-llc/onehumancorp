#!/bin/bash
# Strict bash mode
set -euo pipefail

# This script is run via Bazel sh_test to ensure no PII leakage in logging statements

EXIT_CODE=0

# Check if SRCDIR is set (Bazel), otherwise use src/server
SRCDIR=${1:-src/server}

# Find all rust files in src/server/
FILES=$(find "$SRCDIR" -name "*.rs" 2>/dev/null || true)

for FILE in $FILES; do
    # Exclude the telemetry modules and tests from the leakage check as they deal with these strings intentionally
    if [[ "$FILE" == *"telemetry"* ]] || [[ "$FILE" == *"analytics.rs" ]] || [[ "$FILE" == *"_test.rs" ]]; then
        continue
    fi

    # Grep for tracing statements that might contain PII variables that are dynamically interpolated (with {})
    # This avoids false positives on static strings that just contain the word "password" or "secret"
    if grep -iE 'tracing::(info|debug|warn|error)!\(.*\{.*?\}[^;]*(password|secret|credit_card|ssn|email|phone)[^a-zA-Z0-9_]' "$FILE" > /dev/null; then
        echo "FAIL: Potential PII leakage in logging found in $FILE"
        grep -inE 'tracing::(info|debug|warn|error)!\(.*\{.*?\}[^;]*(password|secret|credit_card|ssn|email|phone)[^a-zA-Z0-9_]' "$FILE"
        EXIT_CODE=1
    fi
done

if [ $EXIT_CODE -eq 0 ]; then
    echo "PASS: No obvious PII leakage found in tracing logs."
else
    # Output something that makes the test fail
    echo "Failing test due to PII leak"
    # We avoid the 'exit 1' string to not break the sandbox parsing
    exit 1
fi
