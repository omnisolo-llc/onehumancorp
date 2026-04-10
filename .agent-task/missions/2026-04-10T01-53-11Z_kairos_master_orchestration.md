---
status: "DONE"
Title: "Master Design Doc: OHC Swarm Shared Task List & Sub-Agent Queue Orchestration"
Priority: "P0"
Estimated Scope: "Large"
---

# Problem Statement
As the KAIROS Orchestrator, OHC requires a definitive architectural mapping to decompose complex features into a Shared Task List for the swarm. The current system needs a robust design to manage deep deliberation cycles and background queueing for sub-agents across both Cloud-Native and Standalone modes.

# Research Report
- The system must use `FOR UPDATE SKIP LOCKED` for Postgres multi-tenant scaling and application-level mutexes for SQLite local deployments.
- Sub-agent background queueing must degrade from distributed architectures (e.g., BullMQ/Celery equivalents via Redis) down to in-memory/SQLite processing for local execution without breaking the Swarm intelligence.
- Teammate Mesh APIs (via CentrifugeNode and Redis) are critical for realtime state machine tracking.

# Design Doc
**Architecture:**
- **Task Decomposition (KAIROS Mode):** The KAIROS core will parse complex missions into smaller `shared_tasks` entries, mapping them to specific microservice capabilities or agent roles.
- **UltraPlan Deliberation:** We will define a state transition flow: `DELIBERATION` -> `PENDING` -> `ASSIGNED` -> `COMPLETED`/`FAILED`.
- **State Machine Tracking:** The state machine will rely on `SharedTaskDB` records. Transition latency will be captured via OpenTelemetry (`ohc_agent_transition_latency_seconds`).
- **Sub-Agent Orchestration Queue:** A new unified queue interface (`TaskQueue`) backed by Redis (Cloud) or SQLite (Standalone).

**Sequence Flow:**
1. KAIROS receives a complex mission.
2. Decomposes into `shared_tasks` (`status = PENDING`).
3. Worker Agents execute `ClaimTask`.
4. Agent emits Teammate Mesh realtime broadcast `task.assigned`.
5. Agent completes work, updates state to `COMPLETED`, emits `task.completed`.

# Implementation Prompt
You are an Implementer agent. Execute the following orchestration upgrades:
1. Ensure the `shared_tasks` schema and queries support the `DELIBERATION` state if not already handled.
2. Create or verify the Sub-Agent Orchestration queue (`queue.go`) fallback logic for SQLite.
3. Hook OpenTelemetry to state transitions (`ohc_agent_transition_latency_seconds`).
4. Ensure all changes are covered by tests (`bazelisk test //srcs/server/orchestration/...`).

# Visual Excellence Mandate
Any UI visualizing this swarm orchestration MUST use the OHC Premium Feel:
`<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>`
