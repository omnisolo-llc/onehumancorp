<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# KAIROS Orchestration Core V6

## 1. Executive Summary
The OHC KAIROS Orchestrator provides the centralized intelligence and shared task distribution for the OHC Swarm. This document defines the Phase 1, Phase 2, and Phase 3 architectures needed to power the "Shared Task List", Realtime Teammate Mesh, and AutoDream consolidation.

## 2. Phase 1: Shared Task List Decomposition (UltraPlan)
### Database Architecture (Cloud-Native & Standalone)
- **Cloud-Native Mode (PostgreSQL):** Uses `FOR UPDATE SKIP LOCKED` on the `swarm_tasks` table to allow highly concurrent, distributed worker agents to pull pending tasks without deadlocks.
- **Standalone Mode (SQLite):** Replaces connection-level locking with application-level Go Mutexes over simple transactions to maintain data integrity locally without heavy database features.

### Schema Design
Referencing `srcs/server/db/migrations/20260415120000_shared_tasks.sql`, the KAIROS orchestrator relies on three primary tables:
1. `swarm_tasks`: Central ledger for actionable work (`status`, `priority`, `agent_id`).
2. `task_dependencies`: Directed Acyclic Graph (DAG) edges enabling task chaining.
3. `state_machine_transitions`: Audit log tracking state changes (`PROPOSE` -> `CRITIQUE` -> `APPROVED` -> `EXECUTE`).

## 3. Phase 2: Realtime Teammate Mesh Architecture
### Pub/Sub Coordination
Agents require low-latency coordination to process KAIROS commands synchronously.
- **Message Broker:** Redis Pub/Sub channels (e.g., `ohc-swarm-events-<org_id>`).
- **Distributed Locks:** Redis Redlock ensures exclusive file modifications (Git-Lock Coordination) across the Teammate Mesh.
- **Fallback (Standalone):** Go channels and local event buses proxy the Redis interface when running in local-only environments.

## 4. Phase 3: AutoDream Memory Consolidation Pipeline
### Vector Search
Referencing `srcs/server/db/migrations/20260415123000_autodream_memories_schema.sql`, the system stores high-dimensional context.
- **pgvector (Postgres):** KAIROS orchestrates background sub-agents that embed recent activity logs into `vector(1536)` columns on the `autodream_memories` table.
- **Pinecone (Optional):** Supported as an external drop-in replacement for highly scaled enterprise deployments.

</div>
