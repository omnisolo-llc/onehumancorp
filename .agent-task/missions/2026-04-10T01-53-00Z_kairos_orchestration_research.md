---
status: "DONE"
Title: "Master Design Doc: KAIROS Shared Task List & Teammate Mesh Updates"
Priority: "P0"
Estimated Scope: "Medium"
---

# Problem Statement
We need to finalize the architectural mapping of the KAIROS shared task list and teammate mesh by researching existing components and defining the exact microservices and tasks for the swarm, aligning with the OHC HA and SIP protocols.

# Research Report
- OHC operates in a "Hybrid Architecture" (Cloud-Native & Standalone Desktop).
- Teammate Mesh uses CentrifugeNode for WebSockets and Redis Pub/Sub for coordination.
- AutoDream uses PostgreSQL with pgvector for embeddings and SQLite for local fallback.
- The `Shared Task List` must act as the primary queue.

# Design Doc
**Architecture Mapping:**
- Database Schema: `shared_tasks` table requires `FOR UPDATE SKIP LOCKED` in PG and application-level mutexes in SQLite.
- Microservices:
  - `TaskOrchestrator` handles claiming.
  - `MeshTransport` (Redis/Memory) handles broadcasting state updates.

**Sub-Agent Job Queue:**
To further break down tasks:
- Introduce robust task decomposition logic.
- Tasks should be broadcasted via Teammate Mesh upon state transition (`PENDING` -> `ASSIGNED`).
- Utilize OpenTelemetry metrics for agent state transitions.

# Implementation Prompt
You are an Implementer agent. Your task is to update the KAIROS Orchestration layers as per the refined design.
1. Implement any remaining Shared Task DB migrations.
2. Implement the missing OpenTelemetry hooks for state transitions in `sip.go` or `tasks_db.go`.
3. Verify Teammate Mesh broadcasts are functioning correctly during Task state transitions.
4. Execute `bazelisk test //srcs/server/orchestration/...`

# Visual Excellence Guidelines
Any UI exposing this orchestration must strictly adhere to the OHC Premium Feel:
`<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>`
