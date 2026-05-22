# OHC KAIROS: Hybrid Agentic OS Orchestration Master Blueprint

## 1. Vision
The One Human Corp (OHC) AI OS relies on the KAIROS Orchestrator to decompose complex tasks, coordinate agent swarms via a realtime mesh, and consolidate state long-term.

## 2. Phase 1: Shared Task List (Decomposition)
### Database Schema (Cloud Native)
```sql
CREATE TABLE IF NOT EXISTS shared_tasks (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    parent_plan_id TEXT,
    title TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    assigned_agent_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
```
### Shared Task Claiming Workflow
```mermaid
sequenceDiagram
    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%),font-family:'Outfit'\, 'Inter'\, sans-serif;
    participant Agent
    participant DB
    Agent->>DB: SELECT * FROM shared_tasks WHERE status='PENDING' FOR UPDATE SKIP LOCKED
    DB-->>Agent: Task Info
    Agent->>DB: UPDATE shared_tasks SET status='IN_PROGRESS'
    class Agent,DB premium;
```

## 3. Phase 2: Teammate Mesh APIs
### Architecture
Realtime communication is managed via Redis Pub/Sub integration in Rust, providing low-latency coordination among agents. WebSockets are utilized for UI updates to the frontend.

### API Contracts
The Teammate Mesh exposes APIs for mailbox interaction, primarily located in `src/server/api/mesh/`.

## 4. Phase 3: AutoDream Vector Pipelines
### Architecture
Background pipelines convert ephemeral memory into long-term semantic state.

### Data Pipeline Flow
The memory vector is upserted into PostgreSQL using the `pgvector` extension in the `autodream_memories` table, specifically linking the `source_mission_id`. In Standalone mode, SQLite VSS is utilized as the fallback. Updates the data pipelines in `api/mesh/mesh.go`.
