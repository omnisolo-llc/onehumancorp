---
status: DONE
agent: Implementer
---

# Title: Implement Teammate Mesh APIs (KAIROS Orchestration)

## Problem Statement
Agents require a high-throughput realtime event bus to broadcast intent, coordinate memory, and perform lock arbitration without polling the database continuously.

## Research Report
The KAIROS Orchestration Design Doc specifies:
- **Cloud Architecture**: Agents publish to production Redis Pub/Sub channels (e.g., `mesh:tasks`, `mesh:coordination`). Centrifuge handles downstream WebSocket propagation to the human CEO dashboard.
- **Standalone Architecture**: Fallbacks to in-memory Go channels natively, ensuring the OS functions offline.
- **Zero Secrets**: Rely entirely on SPIFFE/SPIRE Workload APIs to establish mTLS mesh identities.

## Design Doc
1. **Teammate Mesh API**: Define Go API handlers for `mesh:tasks` and `mesh:coordination` broadcasts.
2. **Redis Integration**: Integrate `go-redis` pub/sub if Redis is available.
3. **In-Memory Fallback**: Use Go channels to propagate messages locally in Standalone Mode.
4. **Centrifuge**: Map mesh channels to Centrifuge WebSocket channels for UI updates.
5. **SPIFFE/SPIRE**: Ensure zero-trust authentication via SPIFFE interceptors.

## Implementation Prompt
1. Check `srcs/server/orchestration/` and `srcs/server/dashboard/server.go` for Teammate Mesh endpoints.
2. Ensure endpoints `/api/mesh/broadcast`, `/api/mesh/direct`, and `/api/mesh/mailbox` use robust implementations based on the mode.
3. If Redis is configured, subscribe to channels and broadcast messages across cluster nodes. If not, use an internal Go channel dispatcher.
4. Integrate with `CentrifugeNode` for downstream WebSocket connections.
5. Apply SPIFFE/SPIRE authentication logic for agent-to-agent and backend-to-backend communication.
6. Write rigorous tests (>95% coverage) to verify both Redis and local-fallback behaviors.
7. Verify all changes using Bazel.

## Priority
P0

## Estimated Scope
Medium
