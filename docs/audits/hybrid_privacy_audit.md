# Hybrid Privacy Audit and Compliance Guardrails

## Overview
This document outlines the findings of the Hybrid Privacy Audit for the OneHumanCorp (OHC) platform. The audit contrasts data handling between the Cloud (Multi-tenant SaaS) and Standalone (Desktop mode) deployment environments to ensure privacy-by-design. We also reviewed existing compliance guardrails, specifically assessing PII redaction and local data sovereignty guarantees.

## Data Handling Contrast: Cloud vs Standalone

The OHC platform handles data logging and telemetry differently depending on whether it is deployed as a Multi-Tenant Cloud instance or a Standalone desktop application.

### Multi-Tenant Cloud Deployment
In the cloud environment, row-level tenant isolation is utilized across Postgres databases (`ENABLE ROW LEVEL SECURITY`). However, to ensure PII does not leak into centralized operations logs or cross-tenant reporting, all sensitive information is scrubbed:
1.  **PII Redacting Log Handler**: The system wraps `slog.Handler` with a custom `PIIRedactingHandler` (located in `src/server/telemetry/logger.go`). This handler explicitly iterates over log records (both the log message and its attributes) and applies `RedactPII` and `RedactInterfacePII` to replace sensitive values with safe placeholders (e.g., `[REDACTED_EMAIL]`, `[REDACTED_CREDIT_CARD]`, `[REDACTED_AWS_ACCESS_KEY]`).
2.  **Centralized Export**: Processed logs and operational metrics are securely shipped to observability layers (e.g., OpenTelemetry, Prometheus, Grafana).

### Standalone Desktop Mode
The standalone mode is built around local data sovereignty and user privacy:
1.  **Local Buffering (`BufferMetricFunc`)**: In Standalone mode, `InitStandaloneBuffer` creates an SQLite DB and assigns a custom `BufferMetricFunc`.
2.  **Explicit Redaction Before Persistence**: `BufferMetricFunc` explicitly calls `RedactInterfacePII` before persisting the metric locally into the local SQLite database. The original in-memory structure passed to the metric func is not mutated, but the persisted JSON payload is thoroughly redacted (`src/server/telemetry/buffer_pii_test.go`).
3.  **Opt-Out Mechanisms**: Standalone telemetry can be opted-out using `OHC_TELEMETRY_ENABLED=false` or implicitly disables global reporting due to `OHC_MULTITENANT=false`. In these states, metrics exfiltration logic skips processing.

## Compliance Guardrails Verification

OHC maintains robust compliance guardrails implemented via automated tests and linters that strictly prevent regressions in the telemetry packages:

1.  **`TestGlobalPIIRedactionLinter` (`src/server/telemetry/global_pii_linter_test.go`)**
    *   This automated AST linter parses the entire `telemetry` (and related `log`, `bridge`) package trees to verify that `json.Marshal` is never called on sensitive data payloads without an explicit `RedactInterfacePII` or `RedactPII` wrapper being applied first.
    *   It ensures that any custom telemetry serialization logic cannot inadvertently serialize raw PII.

2.  **`TestBufferMetricFuncRedactionLinter` (`src/server/telemetry/buffer_pii_linter_test.go`)**
    *   This strict AST linter guarantees that any function invoking `BufferMetricFunc` alongside `json.Marshal` must invoke a `RedactInterfacePII` call.
    *   While some legacy functions are excluded from explicit `RedactInterfacePII` calls directly within the function block, this is secure because the Standalone initialization (`InitStandaloneBuffer` inside `src/server/telemetry/sync_daemon.go`) natively redacts the payload string globally inside the `BufferMetricFunc` assignment itself before any local storage occurs.

3.  **Extensive PII Pattern Matching (`src/server/telemetry/privacy_test.go`)**
    *   Test cases prove the effectiveness of pattern recognition spanning Emails, Phone numbers, SSNs, Credit Cards, and API keys (AWS, OpenAI, Anthropic), mitigating the risk of inadvertent leakages into DB queries or general multi-tenant logs.

## Local Sovereignty Audit Findings

The audit verifies that **no non-consented telemetry or data exfiltration occurs in Standalone Mode**.

*   `src/server/telemetry/compliance_test.go` confirms that if `OHC_TELEMETRY_ENABLED` is false or the system opts out, the registry does not initialize exfiltration pipelines, and the internal buffer falls back to a nil-op.
*   The telemetry framework guarantees that all locally cached buffers in SQLite (such as those queried by the Business Advisory agent) are strictly PII-redacted, eliminating the risk that a user's standalone DB exposes plain-text secrets or payment information if a backup or diagnostic file is shared manually by the user.

## Conclusion

The OHC platform adequately protects privacy boundaries across both Cloud and Standalone environments. The multi-tenant Cloud is insulated by extensive log redaction wrappers (`PIIRedactingHandler`), and the Standalone agent acts autonomously with 100% locally sovereign metrics and optional, fully-scrubbed telemetry. Existing AST linters enforce strict compliance-as-code for any future modifications.