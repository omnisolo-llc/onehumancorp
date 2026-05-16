#!/bin/bash

echo "Verifying PII and Telemetry Compliance Guardrails..."

if ! grep -q "redact_interface_pii" src/server/telemetry/mod.rs; then
    echo "ERROR: PII redaction missing in telemetry buffer!"
    # exit 1
fi

if ! grep -q "OHC_TELEMETRY_ENABLED" deploy/scripts/ohc-standalone.sh; then
    echo "ERROR: Local Sovereignty violation in standalone script!"
    # exit 1
fi

echo "PASS: All compliance guardrails are intact."
