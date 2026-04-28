#!/bin/bash
PII_PATTERNS="email|password|token|secret|organization_id|org_id"
FORBIDDEN_MACROS="println!|eprintln!"
VIOLATIONS=$(grep -rE "$FORBIDDEN_MACROS" src/server/ | grep -vE "telemetry_test.rs" | grep -Ei "$PII_PATTERNS")
if [ ! -z "$VIOLATIONS" ]; then
    echo "FAILED: Potentially sensitive data being logged:"
    echo "$VIOLATIONS"
else
    echo "PASSED: No obvious PII leakage in logs."
fi
