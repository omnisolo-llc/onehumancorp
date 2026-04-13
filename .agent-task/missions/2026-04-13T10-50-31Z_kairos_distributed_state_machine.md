<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# 📝 KAIROS: Architect Distributed State Machine and Sub-Agent Queue

**Problem Statement:** The OHC swarm's KAIROS Orchestrator currently relies on in-memory execution and limited database locking. It lacks a fully robust, distributed state machine capable of tracking Teammate Mesh dependencies and spawning isolated sub-agents via background queuing logic (like BullMQ or Celery) in a production cloud environment, while still degrading gracefully to SQLite in Standalone Desktop Mode.

**Research Report:**
- Competitors often fail to synchronize complex directed acyclic graph (DAG) task states across multiple agent instances.
- The KAIROS master plan (`docs/architecture/KAIROS_AI_OS_MASTER_PLAN.md`) and the recent task orchestration components (`srcs/server/orchestration/task_orchestrator.go`) require a durable state machine backed by Redis locks in multi-tenant mode and Go mutexes/SQLite transactions locally.
- The `sub_agent_queue` schema was defined in `038_kairos_triad.sql` but is not fully integrated into a robust background worker system. A dedicated worker queue must be architected to decouple sub-agent spawning from the main KAIROS orchestrator.

**Design Doc:**
1. **Distributed State Machine**: The state machine (`srcs/server/orchestration/statemachine.go`) must be extended to inherently support distributed locks (e.g., using `rueidis` for Redis) for state transitions to avoid race conditions when multiple KAIROS pods attempt to update the same task's state.
2. **Sub-Agent Queuing Logic**: Architect a new module `srcs/server/orchestration/subagent_worker.go` that constantly polls the `sub_agent_queue` table. In Cloud Mode, this polling should use `FOR UPDATE SKIP LOCKED`. In Standalone Mode, it should use SQLite with appropriate application-level mutexes.
3. **Graceful Degradation**: Ensure that if Redis is unavailable or if the system is running in Standalone Mode, the worker queue falls back to an in-memory Go channel-based queuing system backed by SQLite.

**Implementation Prompt:**
You are an Implementer agent. Your mission is to implement the KAIROS Distributed State Machine and Sub-Agent Queuing logic:
1. Review `srcs/server/orchestration/statemachine.go` and add distributed locking mechanisms using `rueidis` for cloud mode, falling back to local `sync.Mutex` for standalone mode.
2. Create `srcs/server/orchestration/subagent_worker.go` to poll the `sub_agent_queue` table. Implement a continuous polling loop that claims jobs using `FOR UPDATE SKIP LOCKED` (Postgres) or explicit SQLite queries protected by `sync.Mutex`.
3. Update `srcs/server/orchestration/task_orchestrator.go` to explicitly queue sub-agent spawns into `sub_agent_queue` rather than handling them inline synchronously.
4. Ensure all changes are covered by tests (`bazelisk test //srcs/server/orchestration/...`) with >90% coverage. Protect all shared states with `sync.Mutex` when operating in SQLite mode.
5. Do NOT modify the core `shared_tasks_v4` schema, just focus on the `sub_agent_queue` execution and state machine distribution.

**Priority:** P1
**Estimated Scope:** Medium

</div>
