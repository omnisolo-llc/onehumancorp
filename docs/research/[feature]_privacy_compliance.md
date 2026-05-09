# Feature Brief: Privacy Compliance

## Title
Hybrid Privacy Audit and Compliance Guardrails

## Problem Statement
The OHC Hybrid Agentic OS operates in both a multi-tenant Cloud environment and a Local Standalone Desktop environment. To ensure absolute data integrity and user trust, we must enforce strict privacy-by-design principles. Specifically, we must prevent PII leakage in multi-tenant cloud logs and guarantee no non-consented telemetry or data exfiltration occurs in the local standalone wrapper. Currently, there is a lack of comprehensive, automated testing to guarantee these constraints across both modes.

## Research Report
*   **Advantages:** Enforcing privacy constraints automatically builds trust with small business owners, reduces legal/compliance risks (e.g., GDPR, CCPA), and ensures the standalone mode genuinely acts as a sovereign data environment.
*   **Risks:** Overly aggressive PII redaction might obscure crucial debugging information. Standalone telemetry opt-ins might be confusing if not clearly communicated.
*   **Pricing:** Negligible direct cost; utilizes existing compute for automated testing and linting.
*   **Compatibility:** Must be integrated into both the Rust backend (`src/server`) and the Go orchestration services (`srcs/server`), specifically targeting telemetry and logging modules.

### Persona Pain Points
*   **Small Business Owner:** "I want to use the cloud features, but I'm worried about my customers' data being mixed up with others or leaked."
*   **Standalone User:** "I run the standalone mode specifically for privacy. I need absolute assurance that nothing is phoning home without my explicit permission."

## Design Doc

### 1. Hybrid Privacy Audit & Compliance Guardrails (Cloud)
*   Implement automated AST (Abstract Syntax Tree) checks or log analysis tests in the CI pipeline to proactively detect and fail builds if sensitive keys (e.g., "password", "email", "ssn", "pii") are passed directly to logging functions without redaction.
*   Ensure the `redact_interface_pii` function (and its Go equivalent) recursively scrubs sensitive data from telemetry payloads before they hit the buffer or storage.

### 2. Local Sovereignty Audit (Standalone)
*   Implement end-to-end integration tests that launch the `ohc-standalone.sh` wrapper script in a sandboxed environment.
*   Verify that if `OHC_TELEMETRY_ENABLED` is `false` (the default for standalone), no telemetry data is written to the local buffer (`telemetry_buffer` table) and no network calls are made to external metrics endpoints.
*   Audit the bash script itself to ensure it strictly enforces the opt-in boundary before launching the Prometheus agent or syncing data.

## Implementation Prompt
Implement automated tests in both Rust (`src/server/telemetry_test.rs`) and Go (`srcs/server/telemetry/harness_metrics_test.go`) to verify PII redaction and standalone telemetry opt-out. Ensure the standalone wrapper script (`deploy/scripts/ohc-standalone.sh`) defaults telemetry to disabled and respects the `OHC_TELEMETRY_ENABLED` flag.

## Priority
High

## Estimated Scope
Small (1-2 days). Primarily involves writing integration/unit tests and auditing existing telemetry functions and wrapper scripts.
