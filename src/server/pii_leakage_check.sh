#!/bin/bash
# Strict bash mode
set -euo pipefail

# This script is run via Bazel sh_test to ensure no PII leakage in logging statements

EXIT_CODE=0

# Check if SRCDIR is set (Bazel), otherwise use src/server
SRCDIR=${1:-src/server}

# Find all rust files in src/server/
FILES=$(find "$SRCDIR" -name "*.rs" 2>/dev/null || true)

# comprehensive PII keyword list based on src/server/telemetry/mod.rs
PII_KEYWORDS="password|secret|key|token|auth|cookie|credential|email|phone|ssn|address|pii|credit_card"

for FILE in $FILES; do
    # Exclude the telemetry modules and tests from the leakage check as they deal with these strings intentionally
    if [[ "$FILE" == *"telemetry"* ]] || [[ "$FILE" == *"analytics.rs" ]] || [[ "$FILE" == *"_test.rs" ]]; then
        continue
    fi

    # Grep for tracing statements that might contain PII variables that are dynamically interpolated (with {})
    # This checks if the {} comes before or after the keyword

    # We grep all matches first. The issue reviewer mentioned ".*? with grep -E is not supported",
    # so we should use a simpler pattern for the {}
    MATCHES=$(grep -iE "tracing::(info|debug|warn|error)!\(.*\{[^\}]*\}[^;]*($PII_KEYWORDS)[^a-zA-Z0-9_]|tracing::(info|debug|warn|error)!\(.*($PII_KEYWORDS)[^a-zA-Z0-9_].*\{[^\}]*\}" "$FILE" || true)

    if [ -n "$MATCHES" ]; then
        # Check if the matched line has an explicit opt-out // pii-safe
        FILTERED=$(echo "$MATCHES" | grep -v "// pii-safe" || true)

        if [ -n "$FILTERED" ]; then
            echo "FAIL: Potential PII leakage in logging found in $FILE"
            echo "$FILTERED"
            EXIT_CODE=1
        fi
    fi
done

if [ $EXIT_CODE -eq 0 ]; then
    echo "PASS: No obvious PII leakage found in tracing logs."
else
    # Output something that makes the test fail
    echo "Failing test due to PII leak"
    # We avoid the 'exit 1' string to not break the sandbox parsing
    echo "1" > /tmp/fail_code
fi

if [ -f /tmp/fail_code ]; then
    rm /tmp/fail_code
    exit 1
fi
