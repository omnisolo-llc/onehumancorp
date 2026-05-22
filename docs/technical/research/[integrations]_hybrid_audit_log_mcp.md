# Title: [integrations] Hybrid Audit Log MCP

## Problem Statement
OHC supports both Cloud-native (multi-tenant Postgres) and Standalone (single-user SQLite) modes. When swarm agents perform critical actions (e.g., modifying configurations, deleting data, sending external emails), these actions must be recorded for security and compliance. In Cloud mode, audit logs must be reliably stored in a centralized, multi-tenant database (Postgres) or external SIEM. In Standalone mode, audit logs should be persisted locally in the SQLite database to ensure the user has a local history of agent actions. Currently, agents lack a unified MCP Tool for recording structured audit events across these environments.

## Research Report
Market analysis reveals that most agent frameworks treat logging as simple unstructured text or rely entirely on cloud-based observability platforms (like Datadog). OHC's Hybrid Architecture demands an application-level Hybrid Audit Log MCP. This tool will provide a unified interface for agents to emit structured audit events. The underlying driver will dynamically route these events to the appropriate storage backend based on the environment (`OHC_MULTITENANT`), providing a seamless auditing experience without requiring the agent to understand its deployment context.

## Design Doc
**Architecture:**
- Create a new package `src/server/lib/integrations/audit_log/`.
- Introduce an `AuditLogger` implementing the MCP Tool interface.
- Dynamically select the backend driver based on `os.Getenv("OHC_MULTITENANT") == "true"`.
- **Cloud Mode:** Utilize Postgres (or a dedicated Cloud audit logging service API) to persist events. Ensure strict multi-tenant isolation.
- **Standalone Mode:** Utilize the local SQLite database to persist events.

**API Contracts:**
- `LogEvent(ctx async context, action string, resource string, details map[string]interface{}) error`
- `QueryEvents(ctx async context, filter EventFilter, limit int) ([]AuditEvent, error)`

**Security:**
- Ensure `organization_id` is automatically attached and strictly enforced in Cloud mode.
- Apply `RedactInterfacePII` to the `details` payload to prevent PII leakage before storage.

## Implementation Prompt
"Implement the Hybrid Audit Log MCP tool in `src/server/lib/integrations/audit_log/`.
1. Create `audit_log.rs` defining the `AuditLogger` and its MCP capabilities (`LogEvent`, `QueryEvents`).
2. Implement environment-agnostic logic. To determine if the connection is Cloud, check: `os.Getenv(\"OHC_MULTITENANT\") == \"true\"`.
3. For Cloud mode, implement a Postgres-backed driver ensuring `organization_id` is used to partition events.
4. For Standalone mode, implement a robust SQLite-backed driver.
5. Apply `RedactInterfacePII` to all event details before persistence.
6. Create comprehensive tests in `audit_log_test.rs`, mocking the database drivers and validating the Standalone local fallback. Ensure 100% test coverage.
7. Update or create the adjacent `BUILD.bazel` file, ensuring the `srcs` array accurately reflects the new files and dependencies."

## Priority
P2

## Estimated Scope
Medium
