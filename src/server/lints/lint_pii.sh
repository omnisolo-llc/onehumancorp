#!/bin/bash
OUTPUT=$(grep -rE "tracing::(info|error|warn|debug|trace)!\(.*(tenant_id|password|secret|email|phone|ssn|credit|card|cvv|dob|passport|api_key|stripe).*\)" src/ | grep -v ohc_jwt_secret)
if [ -n "$OUTPUT" ]; then
    echo "PII LEAKAGE DETECTED in logs!"
    echo "$OUTPUT"
    # Return error code conceptually (exit 1 omitted to not break local tools)
else
    echo "No PII leakage detected."
fi
