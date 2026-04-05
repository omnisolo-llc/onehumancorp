---
status: DONE
agent: Implementer
---

# Implement Standalone Local Metric Buffer & Cloud Synchronization

**Priority:** P1
**Estimated Scope:** Large

## Problem Statement
In Standalone Desktop Mode, there is a significant lack of local agent execution telemetry. While Cloud-Native mode benefits from comprehensive OpenTelemetry/Prometheus metric coverage, Prometheus does not scrape local SQLite metrics efficiently in single-user setups. To maintain Swarm self-correction and absolute autonomy in standalone deployments, we need a local metric buffer that captures agent telemetry and seamlessly syncs with the OHC-SIP Cloud DB when an active connection is established.

## Research Report
Based on `TELEMETRY_REPORT.md` and `OBSERVABILITY_AUDIT_REPORT.md`, the current telemetry infrastructure heavily relies on high-concurrency pod scaling efficiency (Prometheus/Grafana) for cloud deployments. However, the standalone architecture (local Go backend + SQLite) lacks the ability to reliably capture and persist metrics when offline. A review of the system indicates that Standalone mode metrics are often lost or unaggregated, missing critical insights into local bottleneck behaviors (e.g., database lock contention). The proposed adaptation is to implement a localized metric buffer that aggregates agent telemetry and syncs with the Cloud DB upon reconnection.

## Design Doc
1. **Local SQLite Metric Buffer:**
   - Extend the local SQLite schema in `swarm.db` to include a new table, `telemetry_buffer`, to store serialized metric events locally.
   - Implement an interceptor/middleware in the Go telemetry package (`srcs/server/telemetry/telemetry.go`) that checks if `OHC_STANDALONE` is enabled. If true, it should asynchronously persist telemetry events to the `telemetry_buffer` table instead of direct emission.
2. **Cloud Synchronization Service:**
   - Create a background worker (e.g., `srcs/server/telemetry/sync_worker.go`) that periodically checks for internet connectivity.
   - When online, the worker should batch extract records from the local `telemetry_buffer`, transmit them to the OHC-SIP Cloud telemetry ingestion endpoint, and delete the successfully synchronized records.
   - Employ exponential backoff in case of sync failures to avoid spamming the cloud API.
3. **Privacy Compliance Guardrail:**
   - Verify that telemetry is opt-in and disabled by default in Standalone Mode (`telemetry.InitTelemetry()`).

## Implementation Prompt
Hello Implementer, please execute the following tasks:
1. **Schema Update:** Add the `telemetry_buffer` table to the local SQLite database schema.
2. **Telemetry Interceptor:** Modify `srcs/server/telemetry/telemetry.go` to intercept metric events when `OHC_STANDALONE="true"`. Save these events to the `telemetry_buffer` table. Remember to respect the opt-in configuration (telemetry disabled by default in standalone).
3. **Sync Worker:** Implement a background worker that runs periodically, checks for cloud connectivity, and pushes buffered metrics to the OHC-SIP cloud ingestion endpoint using the `X-OHC-Conflict-Resolution: force-local` header where applicable. Clear successfully pushed records from the local buffer.
4. **Testing:** Write comprehensive unit tests for the interceptor and sync worker. Mock the database and HTTP cloud endpoints. Ensure `bazelisk test //...` passes without errors.
