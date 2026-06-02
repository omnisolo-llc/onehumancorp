# [observability] Fix PII redaction leaks in telemetry and event logging

## Title
Fix PII redaction leaks before JSON marshaling in telemetry and event logging.

## Problem Statement
The OHC codebase enforces strict PII redaction across the multi-tenant architecture. Specifically, "In telemetry or logging code, always apply `redact_interface_pii` (or an equivalent redaction function) to payload maps before calling `serde_json::to_string` or passing values along to prevent PII leakage in multi-tenant environments."
However, historically there were multiple code paths violating this rule in the legacy codebase.

Currently, the Rust implementation ensures robust PII redaction natively in the core telemetry and orchestration pipelines. Specifically:
1. `src/server/telemetry/mod.rs`: The `record_queue_length` and other telemetry functions safely construct a `serde_json::Value` payload and pass it to `buffer_metric`. The `buffer_metric` function internally calls `redact_interface_pii(labels)` *before* converting the payload to a JSON string via `serde_json::to_string`, thus guaranteeing no PII leaks.
2. `src/server/hub.rs`: The `sanitize_hub_event` function receives a `serde_json::Value` and invokes `redact_interface_pii` directly on it *before* returning the stringified `HubEvent` struct, ensuring orchestrator logs do not leak PII to persistent storage.

## Research Report
A deep code audit verified all `buffer_metric` calls and `serde_json::to_string` usages in the modern Rust backend.
- In `telemetry/mod.rs`, the redaction mechanism acts as a strict guardrail. By moving `redact_interface_pii` directly into the `buffer_metric` function body, the codebase enforces structural consistency and prevents developers from bypassing redaction when buffering custom payloads.
- In `hub.rs`, `sanitize_hub_event` redacts the incoming `raw` event before serialization.
- Exhaustive testing covering multi-tenant PII guardrails, mixed arrays, varied and nested JSON objects ensures edge cases cannot bypass the PII filter rules.

## Design Doc
1. **telemetry_test.rs**: Enhance existing PII unit tests by adding comprehensive checks for complex nested structures, mixed arrays, and varied casing logic within `test_redact_interface_pii_edge_cases` and `test_redact_interface_pii_highly_nested`.
2. **hub.rs**: Validate that `sanitize_hub_event` continues to redact the `raw` input BEFORE performing the initial stringification. The internal `test_sanitize_hub_event_redaction` verifies this component.

## Priority
P0

## Estimated Scope
Completed in Rust backend.
