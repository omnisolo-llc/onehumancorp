---
title: "Implement Shared Task List, Teammate Mesh, Sub-Agent Queue, and AutoDream Architectures"
status: IN_PROGRESS
agent: jules
priority: P0
scope: Large
---

# Problem Statement
OHC requires an autonomous, resilient backbone to seamlessly decompose massive human goals into isolated, parallel agentic workflows. We need to implement the core KAIROS Orchestration structures defined in the architecture designs: Shared Task Lists, Realtime Teammate Mesh APIs, Sub-Agent Queues, Distributed State Machines, and AutoDream pipelines.

# Research Report
Based on `docs/features/hybrid_agentic_os_features.md`, `docs/KAIROS_ORCHESTRATOR_DESIGN.md`, and specific feature documents in `docs/features/kairos/`, the orchestration engine must function in both Cloud-Native (PostgreSQL + Redis) and Standalone (SQLite) modes. It must enforce strict state transitions and provide scalable coordination.

# Design Doc
Refer to `docs/KAIROS_ORCHESTRATOR_DESIGN.md` for full details.
- **Database schemas** needed: `swarm_tasks`, `state_machine_transitions`, `sub_agent_jobs` (for standalone queues), and `consolidated_memory` (`pgvector`).
- **Teammate Mesh**: Extend generic MeshTransport interfaces to handle agent capability discovery.
- **Queues**: Implement robust CELERY-style background queue interfaces.
- **AutoDream**: Create periodic pipeline to convert `.agent-task/memory/*.yml` to vector embeddings.

# Implementation Prompt
Implementer agents must:
1.  **State Machine & DAG**: Create database migrations for `swarm_tasks` and `state_machine_transitions` schemas. Implement the Distributed Lock mechanism using `rueidis` (Redis) and DB locks (SQLite).
2.  **Teammate Mesh**: Implement new gRPC APIs (`AdvertiseCapabilities`, `DiscoverAgents`, `StreamMeshEvents`) in `srcs/server/orchestration/hub.go`.
3.  **Sub-Agent Queue**: Implement a `TaskQueue` interface in Go with Redis and SQLite variants.
4.  **AutoDream Pipeline**: Implement the Go daemon `AutoDreamPipeline` to process `.agent-task/memory/` files into the `consolidated_memory` vector store.
5.  All Go code must live in `srcs/server/orchestration/` and adhere to `Hybrid-Aware` testing rules. Update frontend API clients/dashboards applying "Premium Feel" aesthetic mandates.
