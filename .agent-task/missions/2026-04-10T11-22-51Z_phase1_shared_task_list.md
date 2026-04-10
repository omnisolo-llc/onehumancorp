---
title: "Phase 1: Shared Task List Database Schema (PostgreSQL/SQLite)"
status: PENDING
agent: "KAIROS Orchestrator"
priority: P0
scope: Large
---

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Phase 1: Shared Task List Database Schema (PostgreSQL/SQLite)

## Problem Statement
The OHC Swarm requires a centralized, database-agnostic "Shared Task List" to distribute workloads among agents, track dependencies, and manage multi-tenant (Cloud-Native Mode) and single-user (Standalone Mode) task queuing.

## Research Report
### Competitive Analysis
| Feature | Legacy System | OHC-HA |
| --- | --- | --- |
| Execution | Sequential | Asynchronous Swarm |
| Scale | Local only | Hybrid (K8s + Desktop) |

### Mermaid Architecture
```mermaid
graph TD;
    A[Agent] -->|Proposes Task| B(Shared Task List DB);
    B -->|Assigns to| C[Worker Agent];
```

## Design Doc
- **Module Paths**: `srcs/server/db/migrations/`
- **Schema**: Add `tasks` table with `id`, `title`, `status`, `agent_id`, `priority`, `tenant_id` (for Postgres isolation).

## Implementation Prompt
Implement a unified SQL schema and Go wrapper logic for the Shared Task List. Ensure that the database migrations support both SQLite and PostgreSQL. Do not use database-specific branching in SQL.
- Create migration in `srcs/server/db/migrations/`.
- Run tests.
- Maintain test coverage > 90%.

</div>
