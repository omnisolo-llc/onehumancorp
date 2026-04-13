<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Title: KAIROS Phase 1: Shared Task List (Decomposition)

## Problem Statement
The Swarm lacks a centralized, scalable tracking system for decomposed tasks that handles distributed pod locking without collisions.

## Research Report
Research on PostgreSQL concurrency options indicates `SELECT ... FOR UPDATE SKIP LOCKED` is the optimal solution for managing a DAG-based shared task queue across multiple pods.

## Design Doc
**Schema:** Table `shared_tasks` (id UUID, title, status, dependencies JSONB).
**Behavior:** Agents pull PENDING tasks, claim them using SKIP LOCKED to prevent duplicates.


**Sequence Diagram:**
```mermaid
sequenceDiagram
    participant Planner
    participant TaskDB
    participant Implementer
    Planner->>TaskDB: INSERT INTO shared_tasks (status='PENDING')
    Implementer->>TaskDB: SELECT id FROM shared_tasks WHERE status='PENDING' FOR UPDATE SKIP LOCKED
    TaskDB-->>Implementer: Return task row
    Implementer->>TaskDB: UPDATE shared_tasks SET status='IN_PROGRESS' WHERE id=?
```

## Implementation Prompt
Implementer Agent:
Create Goose migrations for `shared_tasks` and integrate `FOR UPDATE SKIP LOCKED` into the orchestration logic within `srcs/server/orchestration/`. Ensure standalone fallback to SQLite.

## Priority
P0

## Estimated Scope
Large
</div>
