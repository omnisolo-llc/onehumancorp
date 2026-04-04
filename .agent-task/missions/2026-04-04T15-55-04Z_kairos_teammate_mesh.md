---
title: "Implement Teammate Mesh APIs (KAIROS Orchestration)"
problem_statement: "Agents require a high-throughput realtime event bus to broadcast intent, coordinate memory, and perform lock arbitration without polling the database continuously."
priority: "P0"
estimated_scope: "Medium"
---

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03);">

# Title: Implement Teammate Mesh APIs (KAIROS Orchestration)

## Problem Statement
Agents require a high-throughput realtime event bus to broadcast intent, coordinate memory, and perform lock arbitration without polling the database continuously. The current infrastructure operates in isolation without shared communication channels.

## Research Report
The KAIROS Orchestration Design Doc (`docs/kairos_orchestration_design.md`) specifies:
- **Cloud Architecture**: Agents publish to production Redis Pub/Sub channels (e.g., `mesh:tasks`, `mesh:sync`). Centrifuge handles downstream WebSocket propagation to the human CEO dashboard.
- **Standalone Architecture**: Must fall back to in-memory Go channels natively, ensuring the OS functions offline perfectly when heavy dependencies (Redis) are absent.
- **Zero Secrets**: Rely entirely on SPIFFE/SPIRE Workload APIs to establish mTLS mesh identities.

## Design Doc
1. **Teammate Mesh API**: Define Go API handlers and underlying `TeammateMesh` interfaces for `mesh:tasks` broadcasts.
2. **Redis Integration**: Integrate `rueidis` pub/sub if Redis is available (Cloud Mode).
3. **In-Memory Fallback**: Use Go channels and thread-safe mutex sharding to propagate messages locally in Standalone Mode.
4. **Centrifuge**: Map mesh channels to `CentrifugeNode` WebSocket channels (`PublishTaskBroadcast`) for UI real-time updates.

## Implementation Prompt
1. Check `srcs/server/orchestration/mesh.go` and `srcs/server/dashboard/server.go` for Teammate Mesh endpoints.
2. Ensure endpoints `/api/mesh/broadcast`, `/api/mesh/direct`, and `/api/mesh/mailbox` use robust implementations based on the mode.
3. If Redis is configured, subscribe to channels and broadcast messages across cluster nodes via `rueidis`. If not, use an internal Go channel dispatcher (`LocalTeammateMesh`).
4. Integrate with `CentrifugeNode` for downstream WebSocket connections (`PublishTaskBroadcast`), ensuring payloads have `agent_id`, `action`, `status`, and `task_id` at the root JSON level.
5. Apply SPIFFE/SPIRE authentication logic for agent-to-agent and backend-to-backend communication.
6. Write rigorous tests (>95% coverage) to verify both Redis and local-fallback behaviors without crashing (`ML-Resilience` pattern).
7. Verify all changes using `bazelisk test //srcs/server/orchestration:orchestration_test`.

## Priority
P0

## Estimated Scope
Medium

</div>
