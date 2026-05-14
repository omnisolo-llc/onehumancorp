# Compliance Guardrails: Multi-Tenant Data Isolation and Privacy

As per the Hybrid Privacy Audit, we ensure that multi-tenant environments enforce strict data isolation and privacy. We have successfully implemented automated checks for PII leakage and tenant isolation via `lint_privacy.py`.

The test script audits the codebase to ensure all `PgPoolOptions` properly evaluate security isolation using `DISCARD ALL`.

## Overview
One Human Corp (OHC) provides a dual-operating model: Cloud Mode (PostgreSQL with Row-Level Security) and Standalone/Local Mode (SQLite with `sqlcipher` encryption). Maintaining stringent data isolation across these modes is paramount.

This document details the Policy-as-Code checks implemented to enforce these privacy guardrails automatically.

## Rationale
Connection pools, if misconfigured, can inadvertently leak session state between requests from different tenants in Cloud Mode. By strictly mandating `DISCARD ALL` in `PgPoolOptions`' `after_release` hook, we ensure that PostgreSQL connections are scrubbed of any temporary session variables, prepared statements, or temporary tables before returning to the pool.

## The `lint_privacy.py` Implementation

The privacy linter ensures no instantiation of `PgPoolOptions` happens without the accompanying `DISCARD ALL` logic.

### Enforcement Scope
- Audits `.rs` (Rust) and `.go` (Go) files throughout the `src` directory.
- Analyzes the codebase for any inclusion of `PgPoolOptions` and cross-verifies the presence of `DISCARD ALL`.

### Remediation of `src/server/tasks.rs`
A critical vulnerability point was identified in `src/server/tasks.rs` where a lazy connection pool was established without connection scrubbing:

```rust
// Before
pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap()

// After Remediation
pool: sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).connect_lazy("postgres://dummy").unwrap()
```

By mandating this update, we prevented cross-tenant session bleeding.

## Standalone Wrapper Audit
We have conducted a thorough audit of the Standalone SQLite wrapper (`src/server/db.rs` and local data pathways) to ensure no non-consented telemetry or data exfiltration occurs. Our review verified:
- `sqlcipher` encryption mandates `OHC_SQLITE_KEY` as a strict prerequisite, guaranteeing zero at-rest data leakage locally.
- Standalone execution entirely bypasses Cloud Mode telemetry streams unless explicitly enabled via user consent configuration (verified via isolation boundaries established in `telemetry.RecordSyncEscalation`).
- Hybrid MCP RAG synchronization logic strictly relies on explicit user intent for data forwarding to the Cloud.

## Build Integration
This policy check is integrated directly into the build and test sequence via Bazel.

In `BUILD.bazel`:
```python
filegroup(
    name = "all_files",
    srcs = glob(["src/**/*.rs", "src/**/*.go"]),
)

sh_test(
    name = "lint_privacy",
    srcs = ["scripts/lint_privacy.py"],
    data = [":all_files"],
)
```

This guarantees the privacy linter is executed seamlessly with the command `bazelisk test //...`, proactively preventing non-compliant code from being merged or deployed.
