# [backend] Architect UltraPlan Deliberation & State Machine Tracking

## Problem Statement
The KAIROS orchestrator requires a highly robust method to manage deep-deliberation cycles for complex architectural changes (e.g., Database migrations, Auth overhaul) and a resilient state machine to track the Directed Acyclic Graph (DAG) dependencies.

## Research Report
Currently, KAIROS relies on an ad-hoc polling mechanism, which is prone to race conditions and lock contention. Deep deliberation cycles for high-complexity UltraPlans (e.g., epic-level features) require a specialized state machine that securely tracks node statuses (PENDING -> DELIBERATING -> CLAIMED -> DONE -> FAILED) within a distributed hybrid environment. A robust state machine (backed by PostgreSQL distributed locks or SQLite mutexes) is essential for correct dependency tracking.

## Design Doc
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

**UltraPlan Deliberation State Machine**

A centralized, distributed state tracking mechanism prevents cyclic dependencies and ensures agents do not duplicate work on complex UltraPlans.

**Database Schema (`ultraplan_state`)**
```sql
CREATE TABLE IF NOT EXISTS ultraplan_state (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    epic_id UUID REFERENCES shared_tasks(id),
    deliberation_status VARCHAR NOT NULL DEFAULT 'PENDING',
    deliberation_history JSONB NOT NULL DEFAULT '[]',
    locked_by VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

**Deliberation Workflow**
1. **Initialize**: KAIROS identifies a complex epic and inserts an entry into `ultraplan_state`.
2. **Locking**: An agent claims the deliberation node via `FOR UPDATE SKIP LOCKED`.
3. **Execution**: The agent performs a deep deliberation cycle (e.g., planning schema migrations).
4. **Finalization**: The agent updates `deliberation_status` to 'DONE' and propagates the sub-tasks to the `shared_tasks` table.

</div>

## Implementation Prompt
Implement the UltraPlan Deliberation State Machine.
1. Create the `ultraplan_state` schema in PostgreSQL with a fallback for local SQLite usage.
2. Implement the state transitions (PENDING -> DELIBERATING -> DONE) and DAG dependency validation logic in `src/server/orchestration/ultraplan.go`.
3. Integrate with the Teammate Mesh to broadcast deliberation milestones to the swarm.
4. Provide comprehensive unit tests verifying lock contention guarantees and state transition invariants.

## Priority
P0

## Estimated Scope
Large
