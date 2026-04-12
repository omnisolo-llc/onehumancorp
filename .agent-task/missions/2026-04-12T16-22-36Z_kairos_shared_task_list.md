---
status: PENDING
agent: Implementer
---

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Title: Implement Shared Task List Schema

## Problem Statement
The KAIROS orchestrator needs to decompose complex feature requests into a shared task list for the agent team. Currently, there is no robust backend database design and sequence diagram defining the Shared Task List.

## Research Report
The OHC Hybrid Agentic OS requires agents to execute autonomously but coordinate via a central state. The `ohc_tasks` state machine ensures exactly-once execution.

## Design Doc
We need to track task creation, status, assigned agent, and dependencies.
- `id`: UUID Primary Key
- `title`: String
- `description`: Text
- `status`: String (PENDING, RUNNING, COMPLETED, FAILED)
- `agent_id`: UUID
- `dependencies`: JSONB array of UUIDs
- `created_at`: Timestamp
- `updated_at`: Timestamp

```mermaid
sequenceDiagram
    participant Orchestrator
    participant TaskDB
    participant SubAgent
    Orchestrator->>TaskDB: Decompose & Insert Task (PENDING)
    SubAgent->>TaskDB: Claim Task (UPDATE ... FOR UPDATE SKIP LOCKED)
    TaskDB-->>SubAgent: Return Task
    SubAgent->>TaskDB: Complete Task (UPDATE status=COMPLETED)
```

## Implementation Prompt
1. Create database migration `srcs/server/db/migrations/035_kairos_shared_tasks.sql` defining `ohc_tasks`. Ensure SQLite compatibility (`id TEXT PRIMARY KEY`).
2. Add migration to `srcs/server/db/BUILD.bazel`.

## Priority
P0

## Estimated Scope
Medium
</div>
