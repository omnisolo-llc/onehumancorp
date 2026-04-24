# [observability]_mcp_sync_worker_conflict_resolution.md

## Title
Fix Missing X-OHC-Conflict-Resolution Header in MCP Sync Worker

## Problem Statement
The OHC Hybrid Architecture relies on synchronization workers to push buffered local telemetry metrics from the Standalone (SQLite-backed) environment to the Cloud (PostgreSQL-backed) environment via the `X-OHC-Conflict-Resolution` header. However, telemetry synchronization from the Standalone MCP environment to the Cloud API Gateway using the `McpSyncWorker` currently does not include the required `X-OHC-Conflict-Resolution: force-local` header. When the syncing worker pushes these locally generated logs, the OHC-SIP Cloud ingestion endpoint may reject them, or handle conflicts improperly. This causes data loss in multi-tenant cloud ingestion systems if the Standalone agent metrics don't explicitly override existing or conflicting placeholder states. From the perspective of a Swarm Operator, Standalone metrics either disappear or are not reliably prioritized upon reconnection.

## Research Report
- **Goal:** Ensure the standalone telemetry sync worker accurately routes locally-buffered metrics to the centralized cloud system without conflict-related ingestion errors.
- **Findings:**
  - `McpSyncWorker` (located in `src/server/telemetry/mcp_sync_worker.go`) currently fakes/stubs the API call (`// Simulate SPIFFE mTLS API Gateway Call`) and is lacking the HTTP network logic completely.
  - The architectural documentation (`docs/technical/research/[observability]_hybrid_swarm_mcp_telemetry_mesh.md`) calls for periodic flushes of the SQLite buffer to the Cloud API Gateway.
  - All telemetry sync workers pushing buffered local metrics to the OHC-SIP Cloud ingestion endpoint *must* include the `X-OHC-Conflict-Resolution: force-local` HTTP header, per system invariants.
  - The cloud ingestion endpoint expects this header to correctly merge Standalone telemetry without conflict rejections.

## Design Doc
1.  **McpSyncWorker Implementation**:
    - Update `src/server/telemetry/mcp_sync_worker.go`.
    - Introduce an HTTP client configured for mTLS (or simulating it, depending on the actual API implementation strategy) to make POST requests to the Cloud Gateway.
    - Explicitly add the `X-OHC-Conflict-Resolution: force-local` header to all HTTP requests pushing telemetry data.
2.  **API Integration**:
    - Define the Cloud MCP Gateway endpoint URL (e.g., passed via environment variables or a configuration struct).
    - Serialize the `pendingIDs` and their associated metric payloads into a JSON array for batch submission.

## Implementation Prompt
**Objective:** Update the `McpSyncWorker` to execute actual HTTP POST requests to the Cloud MCP Gateway when flushing local metrics, and explicitly include the `X-OHC-Conflict-Resolution: force-local` header to satisfy the ingestion invariants.

1.  **Configuration**: Modify the `McpSyncWorker` initialization (`NewMcpSyncWorker`) to accept an `endpointURL` string and an `httpClient` interface.
2.  **HTTP Request**: Replace the simulated logging statement (`// Simulate SPIFFE mTLS API Gateway Call`) in `syncOnce` with a real `http.NewRequest("POST", endpointURL, payloadBytes)`.
3.  **Header Injection**: Ensure `req.Header.Set("X-OHC-Conflict-Resolution", "force-local")` is called before executing the request.
4.  **Error Handling**: Only mark the metrics as `synced` in the local SQLite buffer if the HTTP request returns a `2xx` success status code.
5.  **Testing**: Update `mcp_sync_worker_test.go` to mock the HTTP client, asserting that the payload is sent correctly and that the `X-OHC-Conflict-Resolution` header is strictly equal to `force-local`.

## Priority
P0

## Estimated Scope
Small
