---
status: "DONE"
Title: "Master Design Doc: KAIROS Sub-Agent Orchestration & Realtime Mesh Architecture"
Priority: "P0"
Estimated Scope: "Large"
---

# Problem Statement
To finalize the KAIROS Triad (Shared Task List, Teammate Mesh, AutoDream) and grant the swarm absolute autonomy, we must build scalable background queuing logic (e.g., BullMQ/Celery style) to spawn isolated sub-agents in production. Furthermore, the realtime Teammate Mesh architecture must be tightly integrated to track state machine changes across these isolated workers reliably in both Cloud-Native and Standalone modes.

# Research Report
- OHC currently utilizes `srcs/server/orchestration/tasks_db.go` for the Shared Task List, using `FOR UPDATE SKIP LOCKED` for PG.
- `srcs/server/orchestration/hub.go` and `mesh.go` represent the Realtime Teammate Mesh APIs leveraging `CentrifugeNode` and Redis Pub/Sub (`rueidis`).
- We need an orchestration layer that ties the Shared Tasks DB queuing mechanics directly into background sub-agent processes (e.g., executing isolated LLM context sweeps without blocking the main event loops).

# Design Doc
**Architecture:**
1. **Sub-Agent Orchestration Queue:**
   - Create a scalable background queuing interface `SubAgentQueue` within the KAIROS Orchestrator.
   - Cloud-Native mode uses a distributed task queue built on Redis Streams or list primitives.
   - Standalone Desktop mode gracefully degrades to an in-memory or SQLite-backed local queue.
   - Use `ohc_agent_transition_latency_seconds` histogram metric to observe the sub-agent state transitions (PENDING -> RUNNING -> COMPLETED).

2. **State Machine Tracking via Teammate Mesh:**
   - Any state mutation of a task within the `SubAgentQueue` MUST broadcast a corresponding realtime event via the Teammate Mesh APIs.
   - Payload schema mapping: `MeshEvent { topic: "task.subagent.state", payload: { taskId, previousState, nextState, transitionLatencyMs } }`.

# Implementation Prompt
You are an Implementer agent. Your task is to implement the sub-agent orchestration loop and realtime mesh integrations.
1. In `srcs/server/orchestration/sub_agent.go`, implement the background queueing logic (`SubAgentQueue`).
2. Implement gracefully degrading queue implementations (Redis vs SQLite).
3. Connect the state transition events to the existing Teammate Mesh APIs (`MeshTransport`).
4. Emit `ohc_agent_transition_latency_seconds` for state changes.
5. Provide high-coverage (>90%) tests for the sub-agent execution loop.
6. Verify your implementation by running `bazelisk test //srcs/server/orchestration/...`.

# Visual Excellence Guidelines
Any UI exposing this sub-agent orchestration tracking must strictly adhere to the OHC Premium Feel:
`<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>`
