---
title: "KAIROS: Teammate Mesh API Refinement & OHC-SIP Compliance"
status: PENDING
priority: P0
scope: Medium
agent: Implementer
---

# Title: KAIROS: Teammate Mesh API Refinement & OHC-SIP Compliance

## Problem Statement
The OHC Swarm Intelligence Protocol (OHC-SIP) mandates that all cross-agent communication contains `agent_id`, `action`, and `status` at the JSON root. While `srcs/server/orchestration/mesh.go` implements basic pub/sub, it needs to be strictly enforced across all transport layers (Redis, Memory, WebSocket) to ensure seamless interoperability between Specialized Agents (L5-L7).

## Research Report
- Current `MeshMessage` struct is mostly compliant but some legacy handlers in `api/mesh/mesh_handler.go` might still use non-standard payloads.
- Cloud-Native mode needs better error handling for Redis connection drops (rueidis automatic recovery is good, but application-level retries for 'Publish' are missing).
- Standalone Mode needs a "Local Broadcast Hub" that mimics the Centrifuge/Redis behavior exactly.

## Design Doc
1. **Core Interface Enforcement**:
   - Ensure `TeammateMesh` interface in `srcs/server/orchestration/mesh.go` is the absolute authority for all communication.
2. **OHC-SIP Validator Middleware**:
   - Implement a validator that rejects any outgoing/incoming message missing the mandatory SIP fields.
3. **Hybrid Mesh Hub**:
   - Integrate `CentrifugeHub` more deeply into the `TeammateMesh` so the same "Room/Channel" logic works across WS and Redis.
4. **Observability**:
   - Log mesh latency metrics: `ohc_mesh_delivery_latency_seconds`.

## Implementation Prompt
Refine the `Teammate Mesh` in `srcs/server/orchestration/mesh.go` and `api/mesh/mesh_handler.go`.
1. Update the `MeshMessage` struct to ensure it matches the OHC-SIP specification exactly.
2. Implement a `ValidateSIP()` method on `MeshMessage`.
3. Wrap all `Publish` calls in a retry logic with exponential backoff (already partially present in `meshWithRetry`).
4. Ensure `LocalTeammateMesh` (Standalone) correctly sharts messages to reduce lock contention as implemented in `NewLocalTeammateMesh`.
5. Integrate the `MeshTransport` interface with the `api/mesh` REST endpoints so external thin clients can also participate in the mesh.
6. Write tests in `srcs/server/orchestration/mesh_sip_test.go` verifying that malformed (non-SIP) messages are rejected.
7. Achieve >90% coverage for mesh routing logic.

## Priority
P0

## Estimated Scope
Medium
