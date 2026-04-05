---
status: DONE
agent: Implementer
agent: Implementer
---

# Title: KAIROS Orchestration: Implement Scalable Sub-Agent Background Queuing Logic

## Problem Statement
The OHC Hybrid Architecture currently supports basic Shared Task Lists, Teammate Mesh APIs, and AutoDream pipelines. However, to achieve true "Task Decomposition (KAIROS Mode)", the swarm requires robust, scalable background queuing logic to spawn isolated, transient sub-agents in a production environment (similar to BullMQ/Celery patterns) safely bounded by distributed state machines. Without this, complex architectural missions cannot be decomposed into parallelized, isolated execution contexts efficiently across Kubernetes pods.

## Research Report
1. **Concurrency and State Management**: Relying strictly on simple loop iterations for task processing limits vertical scalability. We must implement a "Sub-Agent Queuing Service" leveraging `swarm_tasks` and `shared_tasks` backed by PostgreSQL `FOR UPDATE SKIP LOCKED` (Cloud) and explicitly managed transaction locks (SQLite).
2. **Sub-Agent Isolation**: A Sub-Agent should be spawned dynamically. The queuing system must maintain a parent-child state machine tracking to avoid orphaned tasks.
3. **Queue Technology**: Instead of bringing in heavy external dependencies like BullMQ or Celery, we will use our native Go `srcs/server/orchestration/tasks.go` and `task_orchestrator.go` mechanisms. We will introduce a new `SubAgentSpawner` worker that polls the KAIROS queue for `DELEGATED` task types.
4. **Teammate Mesh Broadcast**: Sub-Agent spawns and lifecycle events (e.g., `SUB_AGENT_STARTED`, `SUB_AGENT_COMPLETED`) must broadcast via the existing Redis Pub/Sub `mesh:tasks` channels for the CEO Dashboard updates.
5. **Aesthetic Excellence**: All UI telemetry emitted by this system to the CEO dashboard must adhere to the OHC Premium Feel (`backdrop-filter: blur(20px) saturate(200%)`, `background: rgba(255, 255, 255, 0.03)`, `font-family: 'Outfit', 'Inter', sans-serif`).

## Design Doc
1. **Schema Updates (Implicit via Payload)**: We will utilize the existing `payload JSONB` field in `shared_tasks` to include sub-agent configurations: `{"sub_agent_type": "IMPLEMENTER", "isolated_context": true, "parent_task_id": "uuid"}`.
2. **Go Models & Spawner**:
   Create a new interface `SubAgentSpawner` in `srcs/server/orchestration/sub_agent.go`:
   ```go
   type SubAgentSpawner interface {
       Spawn(ctx context.Context, task *SharedTask) error
       Monitor(ctx context.Context) error
   }
   ```
3. **Task Orchestrator Integration**: Update `DefaultTaskOrchestrator` to recognize tasks with `DELEGATED` priority or specific payload markers, and route them to `SubAgentSpawner`.
4. **Resilience**: The spawner must implement exponential backoff retries and heartbeats (`.agent-task/status/`) mapped back into the SIPDB.
5. **Fallback**: In Standalone mode (SQLite), `SubAgentSpawner` spawns local goroutines instead of requesting new K8s pods or distributed workers. Ensure `pool.IsSQLite()` logic bounds the concurrency to avoid overwhelming the local machine.

## Implementation Prompt
Hello Implementer agent! Please execute the Sub-Agent Background Queuing Logic:
1. Create `srcs/server/orchestration/sub_agent.go` defining the `SubAgentSpawner` logic.
2. Integrate `SubAgentSpawner` into `srcs/server/orchestration/task_orchestrator.go`'s `StartBackgroundWorker` loop. It should poll for tasks specifically designated for sub-agent delegation.
3. Ensure the Spawner utilizes the `Teammate Mesh` (`tm.hub.PublishTaskBroadcast`) to emit lifecycle events (`SUB_AGENT_SPAWNED`, `SUB_AGENT_COMPLETED`).
4. In Cloud Mode, rely on PostgreSQL `FOR UPDATE SKIP LOCKED` via the existing `PollTasks` method.
5. In Standalone Mode, enforce concurrency limits using a buffered Go channel or semaphore to prevent CPU exhaustion on the local machine. Check `to.db.IsSQLite()` to apply the throttle.
6. Write rigorous unit tests in `srcs/server/orchestration/sub_agent_test.go` ensuring >90% coverage. Use `sqlite://file::memory:?cache=shared` for the database provider.
7. Verify your work with `bazelisk test //srcs/server/orchestration/... --test_output=errors`. Do not leave temporary scripts behind.

## Priority
P1

## Estimated Scope
Medium
