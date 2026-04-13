<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# 📝 KAIROS: Architect Distributed State Machine and Sub-Agent Queue Worker

**Problem Statement:** The OHC swarm's KAIROS Orchestrator currently relies on in-memory execution and limited database locking. It lacks a fully robust, distributed state machine capable of tracking Teammate Mesh dependencies and spawning isolated sub-agents via background queuing logic (like BullMQ or Celery) in a production cloud environment, while still degrading gracefully to SQLite in Standalone Desktop Mode.

**Research Report:**
- Competitors often fail to synchronize complex directed acyclic graph (DAG) task states across multiple agent instances.
- The KAIROS master plan (`docs/architecture/KAIROS_AI_OS_MASTER_PLAN.md`) and the recent task orchestration components require a durable state machine backed by Redis locks in multi-tenant mode and Go mutexes/SQLite transactions locally.
- The `sub_agent_queue` schema was defined in `038_kairos_triad.sql` but is not fully integrated into a robust background worker system. A dedicated worker queue must be architected to decouple sub-agent spawning from the main KAIROS orchestrator.

**Design Doc:**
1. **Distributed State Machine**: The state machine (`srcs/server/orchestration/state_machine.go`) must be extended to inherently support distributed locks (e.g., using `MutexProvider` from `mutex.go`) for state transitions to avoid race conditions when multiple KAIROS pods attempt to update the same task's state.
2. **Sub-Agent Queuing Logic**: Architect a new module `srcs/server/orchestration/subagent_worker.go` that constantly polls the `sub_agent_queue` table. In Cloud Mode, this polling should use `FOR UPDATE SKIP LOCKED`. In Standalone Mode, it should use SQLite with appropriate application-level locks.
3. **Task Orchestrator Decoupling**: The inline call to `_ = to.spawner.Spawn(to.workerCtx, task)` in `task_orchestrator.go` should be replaced with an insertion into `sub_agent_queue` with `status = 'QUEUED'`.
4. **Graceful Degradation**: Ensure that if Redis is unavailable or if the system is running in Standalone Mode, the worker queue falls back to an in-memory Go channel-based queuing system backed by SQLite.

**Implementation Prompt:**
You are an Implementer agent. Your mission is to implement the KAIROS Distributed State Machine and Sub-Agent Queuing logic:
1. Modify `srcs/server/orchestration/state_machine.go`: Update `TaskStateMachine` to include `MutexProvider` and acquire a distributed lock before state transitions.
2. Create `srcs/server/orchestration/subagent_worker.go` to poll the `sub_agent_queue` table and execute tasks via `SubAgentSpawner`.
3. Update `srcs/server/orchestration/task_orchestrator.go` to explicitly queue sub-agent spawns into `sub_agent_queue` rather than handling them inline synchronously.
4. Ensure all changes are covered by tests (`bazelisk test //srcs/server/orchestration/...`) with >90% coverage.

**Priority:** P1
**Estimated Scope:** Medium

</div>
