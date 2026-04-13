<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03);">

# OHC Master Design Doc: KAIROS Hybrid Agentic OS
**Author:** Principal Product Architect & KAIROS Orchestrator (L7)

## Overview
KAIROS acts as the central orchestrator, decomposing complex feature requests into a shared task list for the OHC Swarm.

## Phase 1: Shared Task List (Decomposition)
The `shared_tasks` PostgreSQL table coordinates tasks.

### Sequence Diagram
```mermaid
sequenceDiagram
    participant KAIROS
    participant TaskDB as PostgreSQL
    participant Implementer Agent
    KAIROS->>TaskDB: INSERT INTO shared_tasks_v3 (status='PENDING')
    Implementer Agent->>TaskDB: SELECT id FROM shared_tasks_v3 WHERE status='PENDING' FOR UPDATE SKIP LOCKED
    TaskDB-->>Implementer Agent: Return task row
    Implementer Agent->>TaskDB: UPDATE shared_tasks_v3 SET status='IN_PROGRESS' WHERE id=?
```

## Phase 2: Orchestration (Teammate Mesh Architecture)
Agents communicate over `mesh:tasks` and `mesh:coordination` Redis channels.

## Phase 3: autoDream (Memory Consolidation)
pgvector pipelines consolidate swarm memory into the `consolidated_memory` database.

## Phase 4: Sub-Agent Orchestration Queue
Robust execution tracking via a Distributed State Machine.

</div>
