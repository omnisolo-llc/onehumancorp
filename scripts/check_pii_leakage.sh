#!/bin/bash
echo "Running PII Leakage Guardrail Check..."
LEAKS=$(find src -type f -name "*.rs" -exec grep -HnE "info\!|error\!|warn\!|debug\!" {} \; | grep -iE "password|api_key|secret_key|credit_card|cvv|dob|ssn|mac_address" | grep -v "\[REDACTED\]" || true)

if [ ! -z "$LEAKS" ]; then
    echo "ERROR: Potential PII leakage found in log statements without [REDACTED]!"
    echo "$LEAKS"
    exit 1
else
    echo "✓ No PII leakage found in log statements."
fi
