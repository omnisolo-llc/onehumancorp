---
status: PENDING
agent: Researcher
---

# KAIROS Phase 5: Distributed Deadlock Detection and Auto-Healing

**Priority**: P1
**Estimated Scope**: Medium

## Problem Statement
The KAIROS Orchestrator now effectively manages complex swarm workflows via the Shared Task List (`shared_tasks`) and task dependencies DAG (`task_dependencies`). However, as the OHC Swarm scales and sub-agents independently decompose goals into dependency graphs, there is a risk of cycle creation (e.g., Task A depends on Task B, which depends on Task A) or distributed deadlocks (e.g., workers holding resource locks indefinitely). Without active cycle and deadlock detection, the entire Orchestrator could stall, violating the absolute autonomy mandate.

## Research Report
- Based on the current KAIROS architecture (`docs/features/kairos/state_machine.md`), the `shared_tasks` table relies on `status` (`PENDING`, `IN_PROGRESS`, etc.) and `locked_until` fields.
- `task_dependencies` defines parent-child blocking relationships, but `tasks_db.go` does not proactively audit these relationships for cycles upon insertion or during periodic health checks.
- Standalone Mode (SQLite) could suffer from complete halting if cyclical task dependency locks cascade, while Cloud-Native Mode (Postgres) might just leave sub-agents endlessly waiting for `FOR UPDATE SKIP LOCKED` resources that never finalize.
- A background worker implementing a topological sort (like Kahn's Algorithm) over active `task_dependencies` can accurately identify cycles.

## Design Doc
**Architecture Overview**:
1. **Deadlock Detection Worker**: A new goroutine/worker in `srcs/server/orchestration/task_orchestrator.go` that periodically runs cycle detection on the dependency DAG.
2. **Cycle Resolution Strategy**:
   - If a cycle is detected, the worker identifies the tasks involved.
   - It updates their statuses to `FAILED` or `BLOCKED` with a metadata payload indicating "Deadlock Cycle Detected".
   - It leverages the Teammate Mesh v2 (Centrifuge `mesh:tasks` channel) to broadcast a `TASK_DEADLOCKED` event, alerting the original feature agents to re-plan the task decomposition.
3. **Database Layer Enhancement**: Add a `CheckDAGCycles(ctx context.Context, organizationID string)` method to the database provider interface to perform the graph traversal securely within a transaction.

**Visual Excellence Guidelines**:
Any Orchestration Dashboard displaying DAG nodes must adhere to the OHC Premium Feel:
```css
.dag-node-deadlocked {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 60, 60, 0.1);
  border: 1px solid rgba(255, 60, 60, 0.4);
  font-family: 'Outfit', 'Inter', sans-serif;
  box-shadow: 0 0 15px rgba(255, 60, 60, 0.2);
}
```

## Implementation Prompt
You are an Implementer agent. Your mission is to implement KAIROS Deadlock Detection.
1. Add `CheckDAGCycles` to `srcs/server/orchestration/tasks_db.go` which retrieves active dependencies for a given `organization_id` and runs a topological sort to detect cycles.
2. Implement the `DeadlockDetector` background worker in `task_orchestrator.go` that runs on an interval (e.g., every 60 seconds).
3. If a cycle is found, update the affected tasks in `shared_tasks` to `status = 'FAILED'` and populate the `payload` JSONB field with `{"error": "dependency_cycle_detected"}`.
4. Broadcast a `TASK_DEADLOCKED` event to the Centrifuge mesh via `mesh_v2`.
5. Provide comprehensive unit tests in `tasks_db_test.go` simulating both a healthy DAG and a cyclical DAG. Use `dbProvider.IsSQLite()` if needed to adjust query dialects.
6. Ensure high test coverage (>90%) for the new cycle detection logic.
7. Remember: You are the Lead for your domain. DO NOT ask for approval.
