[security]
# Title
Hybrid Privacy & Local Sovereignty Audit

# Problem Statement
The OHC "Hybrid Agentic OS" operates in both multi-tenant Cloud environments and local Standalone Desktop modes. As a Principal Ethics & Compliance Officer, it is imperative to ensure absolute data integrity. Specifically, multi-tenant Cloud configurations must isolate tenant PII and prevent data leakage, while Standalone Desktop wrapper distributions must adhere to strict local sovereignty principles (i.e. zero non-consented telemetry or data exfiltration).

# Research Report

During the codebase investigation, several key mechanisms were audited across the core server architecture to contrast data handling and verify privacy-by-design principles:

### Cloud Environment: Multi-tenant Privacy and PII Leakage Protection
- **RLS (Row Level Security):** The Postgres database enforces strict tenant isolation using Row Level Security policies (e.g., in `migrations/067_harden_all_rls_policies.sql`). Every table containing sensitive user/tenant state—such as `telemetry_buffer`, `usage_events`, `swarm_memory_embeddings`, and `capability_plugins`—has an active strict RLS policy requiring the `tenant_id` to match `current_setting('app.current_tenant', true)`.
- **Sandbox Manager Violations:** In `src/server/harness/sandbox/manager.rs`, isolated process execution denies disallowed patterns and strictly logs any security `agent_violations` by emitting redacted metrics and explicitly using `SET LOCAL app.current_tenant` to associate the violation with the respective tenant within a SQL transaction, ensuring no cross-contamination of isolated telemetry.
- **PII Redaction Engine:** Core telemetry pipelines utilize `redact_interface_pii` (`src/server/telemetry.rs`). This recursive engine sanitizes serialized payloads (intercepting passwords, tokens, auth keys, API keys, and sensitive emails) prior to them being inserted into the `telemetry_buffer` or forwarded to external dashboards.
- **Automated Guardrails:** The test suite in `src/server/telemetry_test.rs` includes a continuous `test_no_pii_logging_statements` enforcement mechanism. It recursively scans the codebase to ensure no direct standard output logging functions (`println!`, `info!`, `tracing::`) bypass the redaction engine and dump sensitive `api_key`, `secret_key`, or `password` fields to STDOUT.

### Standalone Desktop Mode: Local Data Sovereignty
- **Data Exfiltration Audit:** In standalone mode, local operations bypass telemetry synchronization entirely.
- **Telemetry Sync Daemon Intercept:** In `src/server/telemetry.rs`, the core `buffer_metric` function verifies the `STANDALONE_MODE` environment variable. If true, the metric collection explicitly returns `Ok(())` without persisting the metrics to the database.
- **Cloud Synchronizer Disable:** In the primary server initialization logic (`src/server/lib.rs`), the `TelemetrySyncDaemon`, `PowerSyncOrchestrator`, and `CloudSynchronizerImpl` are strictly gated behind `std::env::var("STANDALONE_MODE") != "true"`. This guarantees the daemon processes responsible for actively pulling missions or pushing local data buffers to `https://api.onehumancorp.com` are never instantiated.
- **Discovery Service Strict Checking:** The `DiscoveryProxy` integration correctly enforces a `spiffe://local.standalone` tool identifier namespace, rather than bridging local connections via the `spiffe://cloud.ohc` federated domain if `STANDALONE_MODE` is activated.

# Design Doc

The current architecture adequately protects against major vulnerabilities, adhering successfully to the strict privacy constraints requested.
To further fortify the codebase:
- Enforce strict structured logging paradigms (like `slog` or `tracing` structured variants) to entirely eliminate ad-hoc `.to_string()` dumps of unknown serialized objects.
- Maintain and expand the PII scanning logic within the automated Bazel test runner to continuously update the keyword glossary (`credit_card`, `ssn`, etc.).
- Maintain Zero-WIP adherence.

# Implementation Prompt

The implementation to augment the automated logging compliance has been attached alongside this brief. PII checks scanning for `api_key` and `secret_key` have been successfully introduced into `src/server/telemetry_test.rs`.

# Priority
P0

# Estimated Scope
Low (1 hr) - Audit Complete. No major refactoring required, only test enforcement logic implemented.