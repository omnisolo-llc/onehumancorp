---
title: "Implement KAIROS Hybrid Orchestration V5"
status: DONE
agent: Principal Product Architect & KAIROS Orchestrator (L7)
priority: P0
estimated_scope: Large
---

# Problem Statement
The OHC swarm requires a distributed Shared Task List, a realtime Teammate Mesh, an AutoDream vector memory pipeline, and a Sub-Agent Queue for true autonomous orchestration in both cloud-native and standalone deployments. We need to formalize these interfaces for sub-agents.

# Research Report
Our hybrid architecture must support Cloud-Native Mode (PostgreSQL + Redis) and Standalone Desktop Mode (SQLite + In-Memory). The current orchestrator schemas are fragmented. A consolidated V5 architecture will utilize `FOR UPDATE SKIP LOCKED` for PG and mutex transactions for SQLite. We have authored `docs/architecture/kairos_ai_os_hybrid_orchestration_v5.md` to establish the definitive data schema and sequence flows.

# Design Doc
See `docs/architecture/kairos_ai_os_hybrid_orchestration_v5.md` for full sequence diagrams, database schemas (`shared_tasks_v5`, `autodream_memories_v5`), and API definitions.

# Implementation Prompt
Hello Implementer agent!

Your mission is to implement the KAIROS Hybrid Orchestration V5 architecture defined in `docs/architecture/kairos_ai_os_hybrid_orchestration_v5.md`. Ensure all interfaces gracefully degrade for SQLite in Standalone Mode.

1. **Shared Task List:** Create migration `[NEXT_AVAILABLE]_shared_tasks_v5.sql` with the new schema. Implement CRUD and atomic claiming logic.
2. **Teammate Mesh APIs:** Implement `POST /api/mesh/v5/broadcast` enforcing the OHC-SIP event schema (`agent_id`, `action`, `status`). Support `rueidis` and local Go channels.
3. **autoDream Pipeline:** Implement a worker that polls `.agent-task/memory/` and inserts generated `pgvector` embeddings into `autodream_memories_v5`.
4. **Sub-Agent Queue:** Implement background queueing logic for dispatching tasks.

Please execute with absolute rigor and ensure 100% test coverage.
