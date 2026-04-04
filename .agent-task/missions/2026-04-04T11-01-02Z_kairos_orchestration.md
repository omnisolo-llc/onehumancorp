---
status: DONE
agent: Jules
---

# Title
Implement KAIROS Orchestration: Shared Task List, Teammate Mesh APIs, and AutoDream Vector Consolidation

# Problem Statement
The One Human Corp (OHC) Swarm requires a robust distributed system to decompose feature requests, share tasks, communicate in real time, and persist architectural insights. Currently, the agents operate in isolation. We need the KAIROS Orchestrator components (Shared Task List, Teammate Mesh, AutoDream) mapped directly into the hybrid (Cloud/Standalone) database layers to enable Swarm Intelligence.

# Research Report
Based on OHC Hybrid Architecture (OHC-HA) principles, the components must support:
1. **Shared Task List**: Distributed state machines preventing race conditions during task claiming. Must use PostgreSQL `FOR UPDATE SKIP LOCKED` in cloud-native and fallback to SQLite transactions in standalone desktop mode.
2. **Teammate Mesh APIs**: High-throughput real-time event bus (Redis Pub/Sub -> Centrifuge -> WebSockets in cloud; in-memory Go channels in standalone).
3. **AutoDream Vector Data Pipelines**: Consolidation of ephemeral session contexts into durable vectorized truth using `pgvector` for exact Nearest Neighbor search, falling back to recency-based text extraction for SQLite.

# Design Doc
- **Database Schema**: Implement `swarm_tasks` (task tracking), `shared_tasks`, `task_dependencies`, and `autodream_memories` with PostgreSQL and SQLite cross-compatibility.
- **Teammate Mesh**: Define API contracts (e.g., `POST /api/mesh/broadcast`) and Centrifuge channels (`mesh:tasks`, `mesh:coordination`).
- **AutoDream Worker**: Background daemon sweeping completed `shared_tasks` and generating vectors via LLM endpoints, then storing them in `autodream_memories`.
- **Aesthetic Core**: Any UI representations (Grafana, Dashboards) must enforce the "Premium Feel": `backdrop-filter: blur(20px) saturate(200%)`, `background: rgba(255, 255, 255, 0.03)`, `font-family: 'Outfit', 'Inter', sans-serif`.

# Implementation Prompt
Implement the backend components for KAIROS Orchestration.
1. Add/modify DB migration scripts in `srcs/server/db/migrations/` to support `swarm_tasks`, `autodream_memories` schemas. Make sure to translate pgvector to SQLite blobs/text where appropriate. Add any new `.sql` migrations to `embedsrcs` in `srcs/server/db/BUILD.bazel`.
2. Implement the Teammate Mesh API endpoint in Go. Support broadcasting events with `agent_id`, `action`, and `status` at the root JSON level to Centrifuge channels (`mesh:tasks`, `mesh:coordination`).
3. Add an AutoDream Worker skeleton in Go to process `COMPLETED` tasks into `autodream_memories`. Ensure concurrency uses `pool.IsSQLite()` logic to disable PostgreSQL-specific locks if in Standalone mode.
4. Ensure code achieves >95% unit test coverage.
5. All DB operations should return `(int64, error)` and should not call `.RowsAffected()`.

# Priority
P0

# Estimated Scope
Large
