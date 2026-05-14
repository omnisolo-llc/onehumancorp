# OHC Hybrid Privacy Audit Report

## 1. Executive Summary
This report contrasts the data handling and privacy protections in One Human Corp's Hybrid Agentic OS across its two primary modes: **Cloud-Native** and **Local Standalone**. OHC adheres to "Privacy-by-Design" principles, ensuring data isolation in multi-tenant environments and absolute sovereignty in local environments.

## 2. Cloud-Native Privacy (Multi-Tenancy)
In Cloud mode, OHC leverages a centralized PostgreSQL database with strict isolation mechanisms.

### Isolation Strategy: Row-Level Security (RLS)
- **Mechanism**: Every table containing tenant data (e.g., `agents`, `tasks`, `orders`, `memories`) has PostgreSQL Row-Level Security enabled.
- **Enforcement**: Policies are tied to the `app.current_tenant` session variable.
- **Guardrail**: The `set_org_context` utility ensures that even system-level queries must explicitly declare their scope, preventing accidental data leakage across organizations.

### Data at Rest & In Transit
- All data in transit is encrypted via TLS/gRPC.
- Sensitive fields are redacted before being buffered for telemetry.

## 3. Standalone Sovereignty (Local Footprint)
In Standalone mode, the system runs locally on the user's hardware, providing absolute data sovereignty.

### Local Encryption: SQLCipher
- **Mechanism**: The local SQLite database is encrypted using **SQLCipher** (AES-256).
- **Mandatory Key**: `OHC_SQLITE_KEY` must be set; the system will panic rather than fall back to cleartext.
- **Strict Permissions**: The runtime script (`ohc-standalone.sh`) enforces `0600` (file) and `0700` (directory) permissions on all data artifacts.

### Telemetry & Exfiltration Guardrails
- **Strict Opt-In**: Telemetry is disabled by default (`OHC_TELEMETRY_ENABLED=false`).
- **Local Persistence**: If enabled, telemetry is buffered locally and uses the same PII redaction logic as the Cloud mode.

## 4. Unified PII Redaction Logic
Both modes share a common telemetry sanitization layer in `src/server/telemetry/mod.rs`.

- **Recursive Sanitization**: JSON payloads are recursively scanned for sensitive keys.
- **Sensitive Keys**: Includes passwords, secrets, tokens, credit cards, IBAN, SWIFT, Tax IDs, and Social Security numbers.
- **Infrastructure Preservation**: `tenant_id` and `organization_id` are preserved to allow for multi-tenant billing attribution without exposing user PII.

## 5. Compliance Guardrails (Automated)
- **`compliance_test.rs`**: Automated tests verify that PII redaction logic remains effective and that standalone sovereignty settings (like default-off telemetry) are strictly enforced.
- **Logging Audit**: Codebase is periodically audited to ensure no sensitive variables are leaked via `tracing` or `console.log`.

---
**Status**: COMPLIANT
**Audit Date**: 2024-05-22
